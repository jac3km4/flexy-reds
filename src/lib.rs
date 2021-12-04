use imported::{ExternNode, ExternWidget, VDom, Vector2};
use red4ext_rs::prelude::*;
use stretch::geometry::Size;
use stretch::number::Number;

mod imported;

#[redscript_export]
fn render_dom(root: ExternNode, container: ExternWidget) {
    let vec = call!(container.0.clone(), "GetSize" () -> Vector2);
    let size = Size {
        width: if vec.x > 0f32 {
            Number::Defined(vec.x)
        } else {
            Number::Undefined
        },
        height: if vec.y > 0f32 {
            Number::Defined(vec.y)
        } else {
            Number::Undefined
        },
    };
    VDom::build(root, size, container)
}

#[ctor::ctor]
fn init() {
    rtti::on_register(register, post_register);
}

extern "C" fn register() {}

extern "C" fn post_register() {
    rtti::register_function("Flexy.UI.RenderDOM", render_dom);
}
