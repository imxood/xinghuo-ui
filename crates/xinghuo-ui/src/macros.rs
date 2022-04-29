#[macro_export]
macro_rules! ui_style {
    () => {
        width(f32)
        height(f32)
        margin($crate::Value)
        padding($crate::Value)
        border($crate::Value)
        border_color(u32)
        background_color(u32)
    };
}

#[macro_export]
macro_rules! attr_method {
    (
        $(#[$outer:meta])*
        $publicity:vis $attr:ident($attr_ty:ty)
    ) => {
        $(#[$outer])*
        $publicity fn $attr(mut self, $attr: $attr_ty) -> Self {
            use crate::DomNode;
            self.dom_mut().style.$attr = $attr;
            self
            // self.attribute(attr_name!($attr), to_set.to_string())
        }
    };
    (
        $(#[$outer:meta])*
        $publicity:vis $attr:ident
    ) => {
        attr_method! {
            $(#[$outer])*
            $publicity $attr(bool)
        }
    };
}

#[macro_export]
macro_rules! ui_element {
    (
        $(#[$outer:meta])*
        <$name:ident>

        $(children {
            $(categories {
                $($child_category:ident),+
            })?
        })?

        $(attributes {
            $(
                $(#[$attr_meta:meta])*
                $attr:ident ( $attr_ty:ty )
                // $($publicity:vis)?
            )*
        })?
    ) => {
        paste::item! {
            $(impl [< $name:camel Builder >] {
                $($crate::attr_method! {
                    $(#[$attr_meta])*
                    pub $attr($attr_ty)
                })*
            })?

            pub struct [<$name:camel>] {
                inner: rctree::Node<$crate::DomElement>,
            }

            impl [<$name:camel>] {
                pub fn new(data: rctree::Node<$crate::DomElement>) -> Self {
                    Self { inner: data }
                }
            }

            impl $crate::DomNode for [<$name:camel>] {
                fn dom_ref(&self) -> std::cell::Ref<$crate::DomElement> {
                    self.inner.borrow()
                }
                fn dom_mut(&mut self) -> std::cell::RefMut<$crate::DomElement> {
                    self.inner.borrow_mut()
                }
                fn node_ref(&self) -> & rctree::Node<$crate::DomElement> {
                    &self.inner
                }
                fn node_mut(&mut self) -> &mut rctree::Node<$crate::DomElement> {
                    &mut self.inner
                }
            }

            ///
            /// A type for initializing the element's attributes before calling `build`.
            #[must_use = "needs to be built"]
            pub struct [<$name:camel Builder>] {
                $(
                    $(
                        $(#[$attr_meta:meta])*
                        $attr: $attr_ty,
                    )*
                )?
                inner: rctree::Node<$crate::DomElement>,
            }

            impl [<$name:camel Builder>] {
                pub fn build(self) -> [<$name:camel>] {
                    [<$name:camel>]::new(self.inner)
                }
            }

            impl Default for [<$name:camel Builder>] {
                fn default() -> Self {
                    Self {
                        inner: rctree::Node::new($crate::DomElement::new(stringify!($name))),
                        $(
                            $(
                                $(#[$attr_meta:meta])*
                                $attr: $attr_ty::default(),
                            )*
                        )?
                    }
                }
            }

            impl $crate::event::GlobalEventHandler for [<$name:camel Builder>] {}

            impl $crate::ParentNode for [<$name:camel Builder>] {
                fn child<T: $crate::NodeBuilder>(mut self, node: T) -> Self {
                    self.inner.append(node.build());
                    self
                }

                // fn child(self, node: rctree::Node<$crate::DomElement>) -> Self {
                // }
            }

            impl $crate::DomNode for [<$name:camel Builder>] {
                fn dom_ref(&self) -> std::cell::Ref<$crate::DomElement> {
                    self.inner.borrow()
                }
                fn dom_mut(&mut self) -> std::cell::RefMut<$crate::DomElement> {
                    self.inner.borrow_mut()
                }
                fn node_ref(&self) -> & rctree::Node<$crate::DomElement> {
                    &self.inner
                }
                fn node_mut(&mut self) -> &mut rctree::Node<$crate::DomElement> {
                    &mut self.inner
                }
            }

            impl $crate::NodeBuilder for [<$name:camel Builder>] {
                fn build(self) -> rctree::Node<$crate::DomElement> {
                    self.inner
                }
            }

            // impl DomElementBuilder for [<$name:camel Builder>] {
            //     fn id(self, id: &'static str) -> Self {
            //         self.id = id;
            //         self
            //     }
            // }

            pub fn $name() -> [<$name:camel Builder>] {
                [<$name:camel Builder>]::default()
            }
        }
    };
}
