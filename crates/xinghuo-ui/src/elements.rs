use crate::{ui_element, Value};

ui_element! {
    <div>

    attributes {
        width(f32)
        height(f32)
        margin(Value)
        padding(Value)
        border(Value)
        border_color(u32)
        background_color(u32)
    }
}

ui_element! {
    <p>

    attributes {
        width(f32)
        height(f32)
        margin(Value)
        padding(Value)
        border(Value)
        border_color(u32)
        background_color(u32)
    }
}
