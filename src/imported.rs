use std::collections::HashMap;

use red4ext_rs::interop::{FromRED, IntoRED};
use red4ext_rs::prelude::*;
use stretch::geometry::{Rect, Size};
use stretch::node::Node;
use stretch::number::Number;
use stretch::style::{AlignContent, Dimension, FlexDirection, FlexWrap, JustifyContent, PositionType, Style};
use stretch::Stretch;

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl IntoRED for Vector2 {
    type Repr = Self;

    fn type_name() -> &'static str {
        "Vector2"
    }

    fn into_repr(self) -> Self::Repr {
        self
    }
}

impl FromRED for Vector2 {
    type Repr = Self;

    fn from_repr(repr: Self::Repr) -> Self {
        repr
    }
}

#[repr(u64)]
pub enum DimUnit {
    Auto = 0,
    Point = 1,
    Percent = 2,
}

impl FromRED for DimUnit {
    type Repr = u64;

    fn from_repr(repr: Self::Repr) -> Self {
        unsafe { std::mem::transmute(repr) }
    }
}

pub struct VDom {
    stretch: Stretch,
    nodes: HashMap<Node, ExternNode>,
}

impl VDom {
    pub fn new() -> Self {
        Self {
            stretch: Stretch::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn build(root: ExternNode, size: Size<Number>, container: ExternWidget) {
        let mut dom = VDom::new();
        let root_ref = dom.add(root);
        dom.stretch.compute_layout(root_ref, size).unwrap();
        dom.render(root_ref, container);
    }

    fn add(&mut self, node: ExternNode) -> Node {
        let style = node.style();
        // let preferred_size = node.preferred_size();
        // let width = match style.width() {
        //     Dimension::Auto if preferred_size.x > 0f32 => Dimension::Points(preferred_size.x),
        //     other => other,
        // };
        // let height = match style.height() {
        //     Dimension::Auto if preferred_size.y > 0f32 => Dimension::Points(preferred_size.y),
        //     other => other,
        // };

        let flex = Style {
            position_type: style.position_type(),
            flex_direction: style.flex_direction(),
            flex_wrap: style.flex_wrap(),
            align_content: style.align_content(),
            justify_content: style.justify_content(),
            size: Size {
                width: style.width(),
                height: style.height(),
            },
            position: Rect {
                start: style.left(),
                end: style.right(),
                top: style.top(),
                bottom: style.bottom(),
            },
            margin: Rect {
                start: style.margin_left(),
                end: style.margin_right(),
                top: style.margin_top(),
                bottom: style.margin_bottom(),
            },
            padding: Rect {
                start: style.padding_left(),
                end: style.padding_right(),
                top: style.padding_top(),
                bottom: style.padding_bottom(),
            },
            flex_grow: style.flex_grow(),
            ..Style::default()
        };

        let children = node.children().iter().map(|n| self.add(n.clone())).collect();
        let node_ref = self.stretch.new_node(flex, children).unwrap();
        self.nodes.insert(node_ref, node);
        node_ref
    }

    fn render(&mut self, node_ref: Node, container: ExternWidget) {
        // self.stretch.compute_layout(node_ref, Size::undefined()).unwrap();

        let node = self.nodes.get(&node_ref).unwrap();
        let layout = self.stretch.layout(node_ref).unwrap();
        let pos = Vector2::new(layout.location.x, layout.location.y);
        let size = Vector2::new(layout.size.width, layout.size.height);

        let widget = call!(node.0.clone(), "Render" (pos, size) -> ExternWidget);
        call!(container.0.clone(), "AddChildWidget" (widget) -> ());

        for child in self.stretch.children(node_ref).unwrap() {
            self.render(child, container.clone());
        }
    }
}

#[derive(Clone)]
pub struct ExternWidget(pub Ref<RED4ext::IScriptable>);

impl FromRED for ExternWidget {
    type Repr = Ref<RED4ext::IScriptable>;

    fn from_repr(repr: Self::Repr) -> Self {
        ExternWidget(repr)
    }
}

impl IntoRED for ExternWidget {
    type Repr = Ref<RED4ext::IScriptable>;

    fn type_name() -> &'static str {
        "ref<inkWidget>"
    }

    fn into_repr(self) -> Self::Repr {
        self.0
    }
}

#[derive(Clone)]
pub struct ExternNode(Ref<RED4ext::IScriptable>);

impl FromRED for ExternNode {
    type Repr = Ref<RED4ext::IScriptable>;

    fn from_repr(repr: Self::Repr) -> Self {
        ExternNode(repr)
    }
}

impl ExternNode {
    pub fn style(&self) -> ExternStyle {
        call!(self.0.clone(), "GetStyle" () -> ExternStyle)
    }

    pub fn children(&self) -> Vec<ExternNode> {
        call!(self.0.clone(), "GetChildren" () -> Vec<ExternNode>)
    }
}

#[derive(Clone)]
pub struct ExternStyle(Ref<RED4ext::IScriptable>);

impl FromRED for ExternStyle {
    type Repr = Ref<RED4ext::IScriptable>;

    fn from_repr(repr: Self::Repr) -> Self {
        ExternStyle(repr)
    }
}

impl ExternStyle {
    pub fn position_type(&self) -> PositionType {
        match call!(self.0.clone(), "GetPositionType" () -> u64) {
            0 => PositionType::Relative,
            1 => PositionType::Absolute,
            _ => panic!(),
        }
    }

    pub fn flex_direction(&self) -> FlexDirection {
        match call!(self.0.clone(), "GetFlexDirection" () -> u64) {
            0 => FlexDirection::Row,
            1 => FlexDirection::Column,
            2 => FlexDirection::RowReverse,
            3 => FlexDirection::ColumnReverse,
            _ => panic!(),
        }
    }

    pub fn flex_wrap(&self) -> FlexWrap {
        match call!(self.0.clone(), "GetFlexWrap" () -> u64) {
            0 => FlexWrap::NoWrap,
            1 => FlexWrap::Wrap,
            2 => FlexWrap::WrapReverse,
            _ => panic!(),
        }
    }

    pub fn align_content(&self) -> AlignContent {
        match call!(self.0.clone(), "GetAlignContent" () -> u64) {
            0 => AlignContent::Stretch,
            1 => AlignContent::FlexStart,
            2 => AlignContent::FlexEnd,
            3 => AlignContent::Center,
            4 => AlignContent::SpaceBetween,
            5 => AlignContent::SpaceAround,
            _ => panic!(),
        }
    }

    pub fn justify_content(&self) -> JustifyContent {
        match call!(self.0.clone(), "GetJustifyContent" () -> u64) {
            0 => JustifyContent::FlexStart,
            1 => JustifyContent::FlexEnd,
            2 => JustifyContent::Center,
            3 => JustifyContent::SpaceBetween,
            4 => JustifyContent::SpaceAround,
            5 => JustifyContent::SpaceEvenly,
            _ => panic!(),
        }
    }

    pub fn height(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetHeight" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn width(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetWidth" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn left(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetLeft" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn right(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetRight" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn top(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetTop" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn bottom(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetBottom" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn margin_left(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetMarginLeft" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn margin_right(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetMarginRight" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn margin_top(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetMarginTop" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn margin_bottom(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetMarginBottom" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn padding_left(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetPaddingLeft" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn padding_right(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetPaddingRight" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn padding_top(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetPaddingTop" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn padding_bottom(&self) -> Dimension {
        Self::create_dim(call!(self.0.clone(), "GetPaddingBottom" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn flex_grow(&self) -> f32 {
        call!(self.0.clone(), "GetFlexGrow" () -> f32)
    }

    fn create_dim(dim: Ref<RED4ext::IScriptable>) -> Dimension {
        if dim.instance.is_null() {
            Dimension::Auto
        } else {
            let val = call!(dim.clone(), "GetValue" () -> f32);
            match call!(dim, "GetUnit" () -> DimUnit) {
                DimUnit::Auto => Dimension::Auto,
                DimUnit::Point => Dimension::Points(val),
                DimUnit::Percent => Dimension::Percent(val),
            }
        }
    }
}
