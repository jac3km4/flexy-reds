use flexlayout_rs::Dimension;
use red4ext_rs::interop::Vector2;
use red4ext_rs::prelude::*;

use crate::redscript::{DimensionUnit, Elem, Widget};
use crate::{layout, markup};

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

pub fn parse_markup(input: String) -> Elem {
    markup::parse(&input).unwrap()
}

pub fn load_markup(name: String) -> Elem {
    markup::load(&name).unwrap()
}
