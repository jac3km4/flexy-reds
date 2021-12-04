use flexlayout_rs::{Dimension, FlexAlign, FlexDirection, FlexProperty, FlexWrap};
use red4ext_rs::interop::{FromRED, IntoRED};
use red4ext_rs::prelude::*;

#[derive(Clone)]
pub struct Elem {
    pub(crate) repr: Ref<RED4ext::IScriptable>,
}

impl FromRED for Elem {
    type Repr = Ref<RED4ext::IScriptable>;

    fn from_repr(repr: Self::Repr) -> Self {
        Elem { repr }
    }
}

impl Elem {
    pub fn layout(&self) -> Layout {
        call!(self.repr.clone(), "GetLayout" () -> Layout)
    }

    pub fn children(&self) -> Vec<Elem> {
        call!(self.repr.clone(), "GetChildren" () -> Vec<Elem>)
    }
}

#[derive(Clone)]
pub struct Layout {
    pub(crate) repr: Ref<RED4ext::IScriptable>,
}

impl FromRED for Layout {
    type Repr = Ref<RED4ext::IScriptable>;

    fn from_repr(repr: Self::Repr) -> Self {
        Layout { repr }
    }
}

impl Layout {
    pub fn flex_direction(&self) -> FlexDirection {
        match call!(self.repr.clone(), "GetFlexDirection" () -> u64) {
            0 => FlexDirection::Row,
            1 => FlexDirection::Column,
            2 => FlexDirection::RowReverse,
            3 => FlexDirection::ColumnReverse,
            _ => panic!(),
        }
    }

    pub fn flex_wrap(&self) -> FlexWrap {
        match call!(self.repr.clone(), "GetFlexWrap" () -> u64) {
            0 => FlexWrap::NoWrap,
            1 => FlexWrap::Wrap,
            2 => FlexWrap::WrapReverse,
            _ => panic!(),
        }
    }

    pub fn align_content(&self) -> FlexAlign {
        match call!(self.repr.clone(), "GetAlignContent" () -> u64) {
            0 => FlexAlign::Inherit,
            1 => FlexAlign::Stretch,
            2 => FlexAlign::Start,
            3 => FlexAlign::Center,
            4 => FlexAlign::End,
            5 => FlexAlign::SpaceBetween,
            6 => FlexAlign::SpaceAround,
            7 => FlexAlign::Baseline,
            _ => panic!(),
        }
    }

    pub fn justify_content(&self) -> FlexAlign {
        match call!(self.repr.clone(), "GetJustifyContent" () -> u64) {
            0 => FlexAlign::Inherit,
            1 => FlexAlign::Stretch,
            2 => FlexAlign::Start,
            3 => FlexAlign::Center,
            4 => FlexAlign::End,
            5 => FlexAlign::SpaceBetween,
            6 => FlexAlign::SpaceAround,
            7 => FlexAlign::Baseline,
            _ => panic!(),
        }
    }

    pub fn height(&self) -> Dimension {
        Self::create_dim(call!(self.repr.clone(), "GetHeight" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn width(&self) -> Dimension {
        Self::create_dim(call!(self.repr.clone(), "GetWidth" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn margin_left(&self) -> f32 {
        call!(self.repr.clone(), "GetMarginLeft" () -> f32)
    }

    pub fn margin_right(&self) -> f32 {
        call!(self.repr.clone(), "GetMarginRight" () -> f32)
    }

    pub fn margin_top(&self) -> f32 {
        call!(self.repr.clone(), "GetMarginTop" () -> f32)
    }

    pub fn margin_bottom(&self) -> f32 {
        call!(self.repr.clone(), "GetMarginBottom" () -> f32)
    }

    pub fn padding_left(&self) -> f32 {
        call!(self.repr.clone(), "GetPaddingLeft" () -> f32)
    }

    pub fn padding_right(&self) -> f32 {
        call!(self.repr.clone(), "GetPaddingRight" () -> f32)
    }

    pub fn padding_top(&self) -> f32 {
        call!(self.repr.clone(), "GetPaddingTop" () -> f32)
    }

    pub fn padding_bottom(&self) -> f32 {
        call!(self.repr.clone(), "GetPaddingBottom" () -> f32)
    }

    pub fn flex_grow(&self) -> f32 {
        call!(self.repr.clone(), "GetFlexGrow" () -> f32)
    }

    pub fn properties(&self) -> Vec<FlexProperty> {
        vec![
            FlexProperty::Direction(self.flex_direction()),
            FlexProperty::Wrap(self.flex_wrap()),
            FlexProperty::AlignContent(self.align_content()),
            FlexProperty::JustifyContent(self.justify_content()),
            FlexProperty::Height(self.height()),
            FlexProperty::Width(self.width()),
            FlexProperty::MarginLeft(self.margin_left()),
            FlexProperty::MarginRight(self.margin_right()),
            FlexProperty::MarginTop(self.margin_top()),
            FlexProperty::MarginBottom(self.margin_bottom()),
            FlexProperty::PaddingLeft(self.padding_left()),
            FlexProperty::PaddingRight(self.padding_right()),
            FlexProperty::PaddingTop(self.padding_top()),
            FlexProperty::PaddingBottom(self.padding_bottom()),
            FlexProperty::Grow(self.flex_grow()),
        ]
    }

    fn create_dim(dim: Ref<RED4ext::IScriptable>) -> Dimension {
        if dim.instance.is_null() {
            Dimension::Auto
        } else {
            let val = call!(dim.clone(), "GetValue" () -> f32);
            match call!(dim, "GetUnit" () -> DimensionUnit) {
                DimensionUnit::Auto => Dimension::Auto,
                DimensionUnit::Point => Dimension::Point(val),
                DimensionUnit::Percent => Dimension::Percent(val),
            }
        }
    }
}

#[derive(Clone)]
pub struct Widget {
    pub(crate) repr: Ref<RED4ext::IScriptable>,
}

impl FromRED for Widget {
    type Repr = Ref<RED4ext::IScriptable>;

    fn from_repr(repr: Self::Repr) -> Self {
        Widget { repr }
    }
}

impl IntoRED for Widget {
    type Repr = Ref<RED4ext::IScriptable>;

    fn type_name() -> &'static str {
        "ref<inkWidget>"
    }

    fn into_repr(self) -> Self::Repr {
        self.repr
    }
}

#[allow(unused)]
#[repr(u64)]
pub enum DimensionUnit {
    Auto = 0,
    Point = 1,
    Percent = 2,
}

impl FromRED for DimensionUnit {
    type Repr = u64;

    fn from_repr(repr: Self::Repr) -> Self {
        unsafe { std::mem::transmute(repr) }
    }
}

impl IntoRED for DimensionUnit {
    type Repr = u64;

    fn type_name() -> &'static str {
        "Uint64"
    }

    fn into_repr(self) -> Self::Repr {
        self as u64
    }
}

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
