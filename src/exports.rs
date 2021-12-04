use flexlayout_rs::Dimension;
use red4ext_rs::prelude::*;

use crate::{layout, redscript};

#[redscript_export]
pub fn render_dom(root: redscript::Elem, target: redscript::Widget) {
    let node = layout::build(root);

    let size = call!(target.repr.clone(), "GetSize" () -> redscript::Vector2);
    let width = if size.x > 0f32 { Some(size.x) } else { None };
    let height = if size.y > 0f32 { Some(size.y) } else { None };

    let layout = node.layout(width, height);
    layout::render(layout, target);
}

#[redscript_export]
pub fn parse_dimension(str: String) -> redscript::Vector2 {
    let (val, unit) = match layout::parse_dimension(&str).unwrap() {
        Dimension::Auto => (0f32, redscript::DimensionUnit::Auto),
        Dimension::Point(val) => (val, redscript::DimensionUnit::Point),
        Dimension::Percent(val) => (val, redscript::DimensionUnit::Percent),
    };
    // hack due to a bug when passing refs, this should create a ref<Dim>
    redscript::Vector2::new(val, unit as u64 as f32)
}

pub extern "C" fn register() {}

pub extern "C" fn post_register() {
    rtti::register_function("Flexy.UI.RenderElem", render_dom);
    rtti::register_function("Flexy.Layout.ParseDimVec", parse_dimension);
}
