#![feature(prelude_import)]
#![feature(associated_type_bounds)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::time::Duration;
use xinghuo_core::{prelude::*, DomNode, Quaternion, Size};
///
/// A type for initializing the element's attributes before calling `build`.
#[must_use = "needs to be built"]
pub struct DivBuilder {
    border_color: u32,
    background_color: u32,
    inner: rctree::Node<::xinghuo_core::DomElement>,
}
impl DivBuilder {
    pub fn build(self) -> Div {
        Div::new(self.inner)
    }
}
impl Default for DivBuilder {
    fn default() -> Self {
        Self {
            inner: rctree::Node::new(::xinghuo_core::DomElement::new("div")),
            border_color: u32::default(),
            background_color: u32::default(),
        }
    }
}
impl ::xinghuo_core::event::GlobalEventHandler for DivBuilder {}
impl ::xinghuo_core::ParentNode for DivBuilder {
    fn child<T: ::xinghuo_core::NodeBuilder>(mut self, node: T) -> Self {
        self.inner.append(node.build());
        self
    }
}
impl ::xinghuo_core::NodeBuilder for DivBuilder {
    fn build(self) -> rctree::Node<::xinghuo_core::DomElement> {
        self.inner
    }
}
pub fn div() -> DivBuilder {
    DivBuilder::default()
}
impl DivBuilder {
    pub fn border_color(mut self, border_color: u32) -> Self {
        self.inner.borrow_mut().style.border_color = border_color;
        self
    }
    pub fn background_color(mut self, background_color: u32) -> Self {
        self.inner.borrow_mut().style.background_color = background_color;
        self
    }
}
impl DivBuilder {
    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.inner.borrow_mut().style.width = width.into();
        self
    }
    pub fn height(mut self, height: impl Into<Size>) -> Self {
        self.inner.borrow_mut().style.height = height.into();
        self
    }
    pub fn margin(mut self, margin: impl Into<Quaternion>) -> Self {
        self.inner.borrow_mut().style.margin = margin.into();
        self
    }
    pub fn padding(mut self, padding: impl Into<Quaternion>) -> Self {
        self.inner.borrow_mut().style.padding = padding.into();
        self
    }
    pub fn border(mut self, border: impl Into<Quaternion>) -> Self {
        self.inner.borrow_mut().style.border = border.into();
        self
    }
}
pub struct Div {
    inner: rctree::Node<::xinghuo_core::DomElement>,
}
impl Div {
    pub fn new(data: rctree::Node<::xinghuo_core::DomElement>) -> Self {
        Self { inner: data }
    }
}
impl ::xinghuo_core::DomNode for Div {
    fn dom_ref(&self) -> std::cell::Ref<::xinghuo_core::DomElement> {
        self.inner.borrow()
    }
    fn dom_mut(&mut self) -> std::cell::RefMut<::xinghuo_core::DomElement> {
        self.inner.borrow_mut()
    }
    fn node_ref(&self) -> &rctree::Node<::xinghuo_core::DomElement> {
        &self.inner
    }
    fn node_mut(&mut self) -> &mut rctree::Node<::xinghuo_core::DomElement> {
        &mut self.inner
    }
}
fn test() {}
trait DrawIface {
    fn rect(&mut self, rect: Rect<f32>, col: Color);
    fn text(&mut self, text: String, pos: Point2<f32>, size: f32, color: Color);
    fn draw(&mut self);
}
enum Direction {
    MinToMax,
    MaxToMin,
    Center,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Direction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Direction::MinToMax,) => ::core::fmt::Formatter::write_str(f, "MinToMax"),
            (&Direction::MaxToMin,) => ::core::fmt::Formatter::write_str(f, "MaxToMin"),
            (&Direction::Center,) => ::core::fmt::Formatter::write_str(f, "Center"),
        }
    }
}
impl ::core::marker::StructuralPartialEq for Direction {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Direction {
    #[inline]
    fn eq(&self, other: &Direction) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => true,
                }
            } else {
                false
            }
        }
    }
}
struct Layout {
    clip_rect: Rect<f32>,
    cursor: Rect<f32>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Layout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Layout {
                clip_rect: ref __self_0_0,
                cursor: ref __self_0_1,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Layout");
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "clip_rect",
                    &&(*__self_0_0),
                );
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "cursor", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
impl Layout {
    pub fn allocate_block(&mut self, height: f32) {}
    pub fn allocate_inline(&mut self, width: f32) {}
    pub fn allocate_inline_block(&mut self, width: f32, height: f32) {}
}
struct DrawDummy {}
impl DrawIface for DrawDummy {
    fn rect(&mut self, rect: Rect<f32>, col: Color) {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["-- draw rect --> rect: ", ", color: ", "\n"],
                &[
                    ::core::fmt::ArgumentV1::new_debug(&&rect),
                    ::core::fmt::ArgumentV1::new_debug(&&col),
                ],
            ));
        };
    }
    fn text(&mut self, text: String, pos: Point2<f32>, size: f32, color: Color) {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &[
                    "-- draw text --> text: ",
                    " pos: ",
                    " size: ",
                    " color: ",
                    "\n",
                ],
                &[
                    ::core::fmt::ArgumentV1::new_debug(&&text),
                    ::core::fmt::ArgumentV1::new_debug(&&pos),
                    ::core::fmt::ArgumentV1::new_debug(&&size),
                    ::core::fmt::ArgumentV1::new_debug(&&color),
                ],
            ));
        };
    }
    fn draw(&mut self) {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["drawing..\n"], &[]));
        };
    }
}
fn main() {
    fn click(evt: Click) {
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["recv event: ", "\n"],
                &[::core::fmt::ArgumentV1::new_debug(&&evt)],
            ));
        };
    }
    let root = {
        div()
            .width(200.0)
            .height(200.0)
            .onclick(click)
            .border("1.0 1.0 1.0")
            .background_color(1)
            .child(div().margin("1.0 1.0").width(100.0).height(100.0).child(
                div().width(50.0).height(50.0).child(
                    #[allow(unused_braces)]
                    "hello",
                ),
            ))
            .child(div().margin("1.0").width(100.0).height(100.0).child(
                div().width(50.0).height(50.0).child(
                    #[allow(unused_braces)]
                    "hello",
                ),
            ))
            .build()
    };
    let root: Box<dyn DomNode> = Box::new(root);
    let root_node = root.node_ref();
    let mut i = 0;
    for node in root_node.traverse() {
        match node {
            rctree::NodeEdge::Start(node) => {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                        &["", " start node: ", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&i),
                            ::core::fmt::ArgumentV1::new_debug(&&node),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Is(2usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 4u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ));
                };
                i += 1;
            }
            rctree::NodeEdge::End(_) => {}
        }
    }
    std::thread::sleep(Duration::from_millis(100));
}
