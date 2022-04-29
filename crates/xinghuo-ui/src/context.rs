use std::sync::Arc;

use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};

use crate::{memory::Memory, layer::GraphicLayers};

#[derive(Clone)]
pub struct Context(Arc<RwLock<ContextImpl>>);

impl Default for Context {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(ContextImpl::default())))
    }
}

impl Context {
    /// 保存 Dom 状态信息
    #[inline]
    pub fn memory(&self) -> MappedRwLockWriteGuard<'_, Memory> {
        MappedRwLockWriteGuard::map(self.write(), |c| &mut c.memory)
    }

    fn read(&self) -> MappedRwLockReadGuard<'_, ContextImpl> {
        RwLockReadGuard::map(self.0.read(), |c| c)
    }

    fn write(&self) -> MappedRwLockWriteGuard<'_, ContextImpl> {
        RwLockWriteGuard::map(self.0.write(), |c| c)
    }
}

#[derive(Default)]
struct ContextImpl {
    memory: Memory,
    graphics: GraphicLayers,
}
