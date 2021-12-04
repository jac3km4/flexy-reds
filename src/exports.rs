use flexlayout_rs::Dimension;
use red4ext_rs::prelude::*;

use crate::layout;
use crate::redscript::{self, Vector2};

#[redscript_export]
pub fn render_elem(root: redscript::Elem, size: Vector2) -> redscript::Widget {
    let node = layout::build(root);

    let width = if size.x > 0f32 { Some(size.x) } else { None };
    let height = if size.y > 0f32 { Some(size.y) } else { None };

    let layout = node.layout(width, height);
    layout::render(layout)
}

#[redscript_export]
pub fn parse_dimension(str: String) -> Ref<RED4ext::IScriptable> {
    let (val, unit) = match layout::parse_dimension(&str).unwrap() {
        Dimension::Auto => (0f32, redscript::DimensionUnit::Auto),
        Dimension::Point(val) => (val, redscript::DimensionUnit::Point),
        Dimension::Percent(val) => (val, redscript::DimensionUnit::Percent),
    };

    call!("Flexy.Layout.NewDim;FloatUnit" (val, unit) -> Ref<RED4ext::IScriptable>)
}

pub extern "C" fn register() {}

pub extern "C" fn post_register() {
    rtti::register_function("Flexy.UI.RenderElem", render_elem);
    rtti::register_function("Flexy.Layout.ParseDim", parse_dimension);
}
