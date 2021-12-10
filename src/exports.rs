use flexlayout_rs::Dimension;
use red4ext_rs::prelude::*;

use crate::layout;
use crate::redscript::{DimensionUnit, Elem, Vector2, Widget};

pub fn render_elem(root: Elem, size: Vector2) -> Widget {
    let node = layout::build(root);

    let width = if size.x > 0f32 { Some(size.x) } else { None };
    let height = if size.y > 0f32 { Some(size.y) } else { None };

    let layout = node.layout(width, height);
    layout::render(layout)
}

pub fn parse_dimension(str: String) -> Ref<RED4ext::IScriptable> {
    let (val, unit) = match layout::parse_dimension(&str).unwrap() {
        Dimension::Auto => (0f32, DimensionUnit::Auto),
        Dimension::Point(val) => (val, DimensionUnit::Point),
        Dimension::Percent(val) => (val, DimensionUnit::Percent),
    };

    call!("Flexy.Layout.Dim::New;FloatUnit" (val, unit) -> Ref<RED4ext::IScriptable>)
}
