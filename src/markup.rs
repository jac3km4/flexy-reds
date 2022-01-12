use std::borrow::Cow;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use flexlayout_rs::{FlexAlign, FlexDirection, FlexWrap};
use red4ext_rs::interop::Color;

use crate::redscript::{Elem, Layout, PositionType};

pub fn load(name: &str) -> Result<Elem> {
    let path = PathBuf::from("r6")
        .join("ui")
        .join("templates")
        .join(name)
        .with_extension("html");

    parse(&std::fs::read_to_string(path)?)
}

pub fn parse(str: &str) -> Result<Elem> {
    let dom = tl::parse(str, tl::ParserOptions::default())?;
    let parser = dom.parser();

    let root = dom.children().first().unwrap().get(parser).unwrap();
    Ok(parse_elem(root, parser)?.unwrap())
}

fn parse_elem(node: &tl::Node, parser: &tl::Parser) -> Result<Option<Elem>> {
    match node {
        tl::Node::Tag(tag) => {
            let attrs = tag.attributes();
            match tag.name().as_bytes() {
                b"box" => {
                    let children = tag
                        .children()
                        .filter_map(|handle| handle.get(parser))
                        .filter_map(|child| parse_elem(child, parser).transpose())
                        .collect::<Result<Vec<_>>>()?;
                    let color: Option<Color> = attrs.read_attr("background-color").ok();

                    let elem = Elem::new_box(children, color).with_layout(parse_layout(attrs)?);
                    Ok(Some(elem))
                }
                b"img" => {
                    let atlas: Option<Cow<str>> = attrs.read_attr("atlas").ok();

                    if let Some(atlas) = atlas {
                        let part: Option<Cow<str>> = attrs.read_attr("part").ok();
                        let color: Option<Color> = attrs.read_attr("tint").ok();
                        let nine_slice = attrs.read_attr("nine-slice").unwrap_or(false);

                        let elem = Elem::new_image(&atlas, part.as_deref(), color, nine_slice)
                            .with_layout(parse_layout(attrs)?);
                        Ok(Some(elem))
                    } else {
                        Ok(None)
                    }
                }
                b"text" => {
                    let attrs = tag.attributes();
                    let text = tag.inner_text(parser);
                    let font_size: Option<i32> = attrs.read_attr("font-size").ok();
                    let color: Option<Color> = attrs.read_attr("color").ok();

                    let elem = Elem::new_text(&text, font_size, color).with_layout(parse_layout(attrs)?);
                    Ok(Some(elem))
                }
                _ => Err(anyhow!("Unexpected tag")),
            }
        }
        tl::Node::Raw(bytes) => {
            let elem = Elem::new_text(&bytes.as_utf8_str(), None, None);
            Ok(Some(elem))
        }
        tl::Node::Comment(_) => Ok(None),
    }
}

pub trait AttrRead<'a>: Sized {
    fn read(attrs: &'a tl::Attributes, name: &'a str) -> Result<Self>;
}

impl<'a> AttrRead<'a> for Cow<'a, str> {
    fn read(attrs: &'a tl::Attributes, name: &'a str) -> Result<Self> {
        let bytes = attrs
            .get(name)
            .flatten()
            .ok_or_else(|| anyhow!("Attribute {} not found", name))?;
        let str = bytes
            .as_bytes_borrowed()
            .and_then(|buf| std::str::from_utf8(buf).ok())
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(bytes.as_utf8_str().into_owned()));
        Ok(str)
    }
}

impl<'a> AttrRead<'a> for bool {
    #[inline]
    fn read(attrs: &'a tl::Attributes, name: &'a str) -> Result<Self> {
        Ok(attrs.read_attr::<Cow<'a, str>>(name)?.parse()?)
    }
}

impl<'a> AttrRead<'a> for i32 {
    #[inline]
    fn read(attrs: &'a tl::Attributes, name: &'a str) -> Result<Self> {
        Ok(attrs.read_attr::<Cow<'a, str>>(name)?.parse()?)
    }
}

impl<'a> AttrRead<'a> for Color {
    fn read(attrs: &'a tl::Attributes, name: &'a str) -> Result<Self> {
        let str = attrs.read_attr::<Cow<'a, str>>(name)?;
        let str = str
            .strip_prefix('#')
            .ok_or_else(|| anyhow!("Invalid color literal"))?;
        if str.len() != 6 {
            return Err(anyhow!("Only full hex color literals allowed"));
        }
        let red = u8::from_str_radix(&str[0..2], 16)?;
        let green = u8::from_str_radix(&str[2..4], 16)?;
        let blue = u8::from_str_radix(&str[4..6], 16)?;
        Ok(Color::new(red, green, blue, 255))
    }
}

pub trait AttrReadSynax<'a> {
    fn read_attr<A: AttrRead<'a>>(&'a self, name: &'a str) -> Result<A>;
}

impl<'a> AttrReadSynax<'a> for tl::Attributes<'a> {
    #[inline]
    fn read_attr<A: AttrRead<'a>>(&'a self, name: &'a str) -> Result<A> {
        A::read(self, name)
    }
}

fn parse_layout(attrs: &tl::Attributes) -> Result<Layout> {
    fn parse_position_type(bytes: &str) -> Result<PositionType> {
        match bytes {
            "relative" => Ok(PositionType::Relative),
            "absolute" => Ok(PositionType::Absolute),
            _ => return Err(anyhow!("Invalid FlexWrap")),
        }
    }

    fn parse_flex_wrap(bytes: &str) -> Result<FlexWrap> {
        match bytes {
            "no-wrap" => Ok(FlexWrap::NoWrap),
            "wrap" => Ok(FlexWrap::Wrap),
            "wrap-reverse" => Ok(FlexWrap::WrapReverse),
            _ => return Err(anyhow!("Invalid FlexWrap")),
        }
    }

    fn parse_flex_dir(bytes: &str) -> Result<FlexDirection> {
        match bytes {
            "row" => Ok(FlexDirection::Row),
            "column" => Ok(FlexDirection::Column),
            "row-reverse" => Ok(FlexDirection::RowReverse),
            "column-reverse" => Ok(FlexDirection::ColumnReverse),
            _ => return Err(anyhow!("Invalid FlexDirection")),
        }
    }

    fn parse_flex_align(bytes: &str) -> Result<FlexAlign> {
        match bytes {
            "inherit" => Ok(FlexAlign::Inherit),
            "stretch" => Ok(FlexAlign::Stretch),
            "start" => Ok(FlexAlign::Start),
            "center" => Ok(FlexAlign::Center),
            "end" => Ok(FlexAlign::End),
            "space-between" => Ok(FlexAlign::SpaceBetween),
            "space-around" => Ok(FlexAlign::SpaceAround),
            "baseline" => Ok(FlexAlign::Baseline),
            _ => return Err(anyhow!("Invalid FlexAlign")),
        }
    }

    let mut layout = Layout::new();
    if let Some(bytes) = attrs.get("position").flatten() {
        layout.with_position_type(parse_position_type(&bytes.as_utf8_str())?);
    }
    if let Some(bytes) = attrs.get("flex-wrap").flatten() {
        layout.with_flex_wrap(parse_flex_wrap(&bytes.as_utf8_str())?);
    }
    if let Some(bytes) = attrs.get("flex-direction").flatten() {
        layout.with_flex_direction(parse_flex_dir(&bytes.as_utf8_str())?);
    }
    if let Some(bytes) = attrs.get("align-items").flatten() {
        layout.with_align_items(parse_flex_align(&bytes.as_utf8_str())?);
    }
    if let Some(bytes) = attrs.get("align-content").flatten() {
        layout.with_align_content(parse_flex_align(&bytes.as_utf8_str())?);
    }
    if let Some(bytes) = attrs.get("justify-content").flatten() {
        layout.with_justify_content(parse_flex_align(&bytes.as_utf8_str())?);
    }
    if let Some(bytes) = attrs.get("width").flatten() {
        layout.with_width(&bytes.as_utf8_str());
    }
    if let Some(bytes) = attrs.get("height").flatten() {
        layout.with_height(&bytes.as_utf8_str());
    }
    if let Some(bytes) = attrs.get("padding").flatten() {
        let val = bytes.as_utf8_str().parse()?;
        layout.with_padding(val);
    }
    if let Some(bytes) = attrs.get("margin").flatten() {
        let val = bytes.as_utf8_str().parse()?;
        layout.with_margin(val);
    }
    if let Some(bytes) = attrs.get("flex-grow").flatten() {
        let val = bytes.as_utf8_str().parse()?;
        layout.with_flex_grow(val);
    }

    Ok(layout)
}

mod test {
    #[test]
    fn parse_simple_box() -> anyhow::Result<()> {
        let dom = tl::parse("<box flex-grow='1'>as</box>", tl::ParserOptions::default())?;
        let parser = dom.parser();
        let node = dom.children().first().unwrap().get(&parser).unwrap();
        let tag = node.as_tag().unwrap().name();
        assert_eq!(tag.as_utf8_str(), "box");
        Ok(())
    }
}
