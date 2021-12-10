use flexlayout_rs::{Dimension, FlexAlign, FlexDirection, FlexProperty, FlexWrap};
use red4ext_rs::interop::{Color, IsoRED};
use red4ext_rs::prelude::*;

#[derive(Clone, Default)]
#[repr(C)]
pub struct Elem {
    pub(crate) repr: Ref<RED4ext::IScriptable>,
}

impl IsoRED for Elem {
    #[inline]
    fn type_name() -> &'static str {
        "ref<Elem>"
    }
}

impl Elem {
    pub fn new_box(children: impl IntoIterator<Item = Elem>, color: Option<Color>) -> Self {
        let elem = call!("Flexy.UI.Box::New;" () -> Elem);
        for child in children {
            call!(elem.repr.clone(), "Child" (child) -> Elem);
        }
        if let Some(color) = color {
            call!(elem.repr.clone(), "BackgroundColor" (color) -> Elem);
        }
        elem
    }

    pub fn new_text(str: &str, font_size: Option<i32>) -> Self {
        let elem = call!("Flexy.UI.Text::New;String" (str) -> Elem);
        if let Some(font_size) = font_size {
            call!(elem.repr.clone(), "FontSize" (font_size) -> Elem);
        }
        elem
    }

    pub fn new_image(atlas: &str, part: Option<&str>, color: Option<Color>, nine_slice: bool) -> Self {
        let elem = call!("Flexy.UI.Image::New;String" (atlas) -> Elem);
        if let Some(part) = part {
            call!(elem.repr.clone(), "TexturePart" (part) -> Elem);
        }
        if let Some(color) = color {
            call!(elem.repr.clone(), "Tint" (color) -> Elem);
        }
        call!(elem.repr.clone(), "NineSliceScale" (nine_slice) -> Elem);
        elem
    }

    pub fn layout(&self) -> Layout {
        call!(self.repr.clone(), "GetLayout" () -> Layout)
    }

    pub fn children(&self) -> Vec<Elem> {
        call!(self.repr.clone(), "GetChildren" () -> Vec<Elem>)
    }

    pub fn with_layout(&mut self, layout: Layout) -> Self {
        call!(self.repr.clone(), "Layout" (layout) -> Elem)
    }
}

#[derive(Clone, Default)]
#[repr(C)]
pub struct Layout {
    pub(crate) repr: Ref<RED4ext::IScriptable>,
}

impl IsoRED for Layout {
    #[inline]
    fn type_name() -> &'static str {
        "ref<Layout>"
    }
}

impl Layout {
    pub fn new() -> Self {
        call!("Flexy.Layout.Layout::New;" () -> Layout)
    }

    pub fn position_type(&self) -> PositionType {
        match call!(self.repr.clone(), "GetPositionType" () -> u64) {
            0 => PositionType::Relative,
            1 => PositionType::Absolute,
            _ => panic!(),
        }
    }

    pub fn with_position_type(&mut self, typ: PositionType) -> Self {
        call!(self.repr.clone(), "PositionType" (typ as u64) -> Self)
    }

    pub fn flex_direction(&self) -> FlexDirection {
        match call!(self.repr.clone(), "GetFlexDirection" () -> u64) {
            0 => FlexDirection::Row,
            1 => FlexDirection::Column,
            2 => FlexDirection::RowReverse,
            3 => FlexDirection::ColumnReverse,
            _ => panic!(),
        }
    }

    pub fn with_flex_direction(&mut self, dir: FlexDirection) -> Self {
        call!(self.repr.clone(), "FlexDirection" (dir as u64) -> Self)
    }

    pub fn flex_wrap(&self) -> FlexWrap {
        match call!(self.repr.clone(), "GetFlexWrap" () -> u64) {
            0 => FlexWrap::NoWrap,
            1 => FlexWrap::Wrap,
            2 => FlexWrap::WrapReverse,
            _ => panic!(),
        }
    }

    pub fn with_flex_wrap(&mut self, wrap: FlexWrap) -> Self {
        call!(self.repr.clone(), "FlexWrap" (wrap as u64) -> Self)
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

    pub fn with_align_content(&mut self, align: FlexAlign) -> Self {
        call!(self.repr.clone(), "AlignContent" (align as u64) -> Self)
    }

    pub fn align_items(&self) -> FlexAlign {
        match call!(self.repr.clone(), "GetAlignItems" () -> u64) {
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

    pub fn with_align_items(&mut self, align: FlexAlign) -> Self {
        call!(self.repr.clone(), "AlignItems" (align as u64) -> Self)
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

    pub fn with_justify_content(&mut self, align: FlexAlign) -> Self {
        call!(self.repr.clone(), "JustifyContent" (align as u64) -> Self)
    }

    pub fn height(&self) -> Dimension {
        Self::create_dim(call!(self.repr.clone(), "GetHeight" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn with_height(&mut self, str: &str) -> Self {
        call!(self.repr.clone(), "Height" (str) -> Self)
    }

    pub fn width(&self) -> Dimension {
        Self::create_dim(call!(self.repr.clone(), "GetWidth" () -> Ref<RED4ext::IScriptable>))
    }

    pub fn with_width(&mut self, str: &str) -> Self {
        call!(self.repr.clone(), "Width" (str) -> Self)
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

    pub fn with_margin(&mut self, val: f32) -> Self {
        call!(self.repr.clone(), "Margin" (val) -> Self)
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

    pub fn with_padding(&mut self, val: f32) -> Self {
        call!(self.repr.clone(), "Padding" (val) -> Self)
    }

    pub fn flex_grow(&self) -> f32 {
        call!(self.repr.clone(), "GetFlexGrow" () -> f32)
    }

    pub fn with_flex_grow(&mut self, val: f32) -> Self {
        call!(self.repr.clone(), "FlexGrow" (val) -> Self)
    }

    pub fn properties(&self) -> Vec<FlexProperty> {
        vec![
            FlexProperty::Direction(self.flex_direction()),
            FlexProperty::Wrap(self.flex_wrap()),
            FlexProperty::AlignItems(self.align_items()),
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
            FlexProperty::Fixed(self.position_type() == PositionType::Absolute),
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

#[derive(Clone, Default)]
#[repr(C)]
pub struct Widget {
    pub(crate) repr: Ref<RED4ext::IScriptable>,
}

impl IsoRED for Widget {
    #[inline]
    fn type_name() -> &'static str {
        "ref<inkWidget>"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum PositionType {
    Relative = 0,
    Absolute = 1,
}

impl Default for PositionType {
    fn default() -> Self {
        PositionType::Relative
    }
}

impl IsoRED for PositionType {
    #[inline]
    fn type_name() -> &'static str {
        "PositionType"
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum DimensionUnit {
    Auto = 0,
    Point = 1,
    Percent = 2,
}

impl Default for DimensionUnit {
    fn default() -> Self {
        DimensionUnit::Auto
    }
}

impl IsoRED for DimensionUnit {
    #[inline]
    fn type_name() -> &'static str {
        "Unit"
    }
}
