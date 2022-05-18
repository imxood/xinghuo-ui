use std::{
    any::Any,
    collections::{
        hash_map::{DefaultHasher, Entry},
        HashMap,
    },
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Id(u64);

impl Id {
    /// Short and readable summary
    pub fn short_debug_format(&self) -> String {
        format!("{:04X}", self.0 as u16)
    }

    #[inline(always)]
    pub(crate) fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct TypeId(u64);

impl TypeId {
    #[inline]
    pub fn of<T: Any + 'static>() -> Self {
        std::any::TypeId::of::<T>().into()
    }

    #[inline(always)]
    pub(crate) fn value(&self) -> u64 {
        self.0
    }
}

impl From<std::any::TypeId> for TypeId {
    #[inline]
    fn from(v: std::any::TypeId) -> Self {
        let mut hasher = DefaultHasher::new();
        v.hash(&mut hasher);
        Self(hasher.finish())
    }
}

#[inline(always)]
fn hash(type_id: TypeId, id: Id) -> u64 {
    type_id.value() ^ id.value()
}

#[derive(Default)]
pub struct Memory {
    pub data: IdTypeMap,
}

struct Element {
    value: Box<dyn Any + 'static + Send + Sync>,
}

impl Element {
    fn new<T: Any + Clone + 'static + Send + Sync>(value: T) -> Self {
        Self {
            value: Box::new(value),
        }
    }

    #[inline]
    fn get_mut<T: 'static + Any + Clone + Send + Sync>(&mut self) -> Option<&mut T> {
        self.value.downcast_mut()
    }

    pub fn get_mut_or_insert_with<T: 'static + Any + Clone + Send + Sync>(
        &mut self,
        insert_with: T,
    ) -> &mut T {
        if !self.value.is::<T>() {
            *self = Self::new(insert_with);
        }
        self.get_mut().unwrap()
    }
}

#[derive(Default)]
pub struct IdTypeMap {
    /// 保存内存数据
    memory: HashMap<u64, Element>,
}

impl IdTypeMap {
    #[inline]
    pub fn insert_memory<T: 'static + Any + Clone + Send + Sync>(&mut self, id: Id, value: T) {
        let hash = hash(TypeId::of::<T>(), id);
        self.memory.insert(hash, Element::new(value));
    }

    #[inline]
    pub fn get_memory<T: 'static + Any + Clone + Send + Sync>(&mut self, id: Id) -> Option<T> {
        let hash = hash(TypeId::of::<T>(), id);
        self.memory
            .get_mut(&hash)
            .and_then(|x| x.get_mut())
            .cloned()
    }

    #[inline]
    pub fn get_memory_mut_or<T: 'static + Any + Clone + Send + Sync>(
        &mut self,
        id: Id,
        or_insert: T,
    ) -> &mut T {
        let hash = hash(TypeId::of::<T>(), id);
        match self.memory.entry(hash) {
            Entry::Vacant(vacant) => vacant.insert(Element::new(or_insert)).get_mut().unwrap(),
            Entry::Occupied(occupied) => occupied.into_mut().get_mut_or_insert_with(or_insert),
        }
        // self.memory
        //     .get_mut(&hash)
        //     .get_mut_or_insert_with(|x| x.get_mut());
        // self.get_mut_or_insert_with(id, || or_insert)
    }
}

#[test]
fn test_cache() {
    let mut data = IdTypeMap::default();

    data.insert_memory(Id(0), "hello");
    let s = data.get_memory::<&str>(Id(0));
    println!("s: {:?}", &s);

    data.insert_memory(Id(1), Arc::new(Mutex::new(5)));
    let s = data.get_memory::<Arc<Mutex<i32>>>(Id(1));
    println!("s: {:?}", &s);
}
