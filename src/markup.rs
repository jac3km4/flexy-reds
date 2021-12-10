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
    let dom = tl::parse(str, tl::ParserOptions::default());
    let parser = dom.parser();

    let root = dom.children().first().unwrap().get(parser).unwrap();
    Ok(parse_elem(root, parser)?.unwrap())
}

fn parse_elem(node: &tl::Node, parser: &tl::Parser) -> Result<Option<Elem>> {
    match node {
        tl::Node::Tag(tag) => match tag.name().raw() {
            b"box" => {
                let attrs = tag.attributes();
                let children = tag
                    .children()
                    .filter_map(|handle| handle.get(parser))
                    .filter_map(|child| parse_elem(child, parser).transpose())
                    .collect::<Result<Vec<_>>>()?;
                let color = attrs
                    .get_attribute("background-color")
                    .flatten()
                    .and_then(|bytes| parse_color(&bytes.as_utf8_str()).ok());

                let elem = Elem::new_box(children, color).with_layout(parse_layout(attrs)?);

                Ok(Some(elem))
            }
            b"img" => {
                let attrs = tag.attributes();
                let atlas = attrs
                    .get_attribute("atlas")
                    .flatten()
                    .map(|bytes| bytes.as_utf8_str());

                if let Some(atlas) = atlas {
                    let part = attrs
                        .get_attribute("part")
                        .flatten()
                        .map(|bytes| bytes.as_utf8_str());
                    let color = attrs
                        .get_attribute("tint")
                        .flatten()
                        .and_then(|bytes| parse_color(&bytes.as_utf8_str()).ok());
                    let nine_slice = attrs
                        .get_attribute("nine-slice")
                        .flatten()
                        .and_then(|bytes| bytes.as_utf8_str().parse().ok())
                        .unwrap_or(false);

                    let elem = Elem::new_image(&atlas, part.as_deref(), color, nine_slice)
                        .with_layout(parse_layout(attrs)?);

                    Ok(Some(elem))
                } else {
                    Ok(None)
                }
            }
            _ => Err(anyhow!("Unexpected tag")),
        },
        tl::Node::Raw(bytes) => {
            let elem = Elem::new_text(&bytes.as_utf8_str(), None);
            Ok(Some(elem))
        }
        tl::Node::Comment(_) => Ok(None),
    }
}

fn parse_color(str: &str) -> Result<Color> {
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

fn parse_layout(attrs: &tl::Attributes) -> Result<Layout> {
    fn parse_position_type(bytes: &[u8]) -> Result<PositionType> {
        match bytes {
            b"relative" => Ok(PositionType::Relative),
            b"absolute" => Ok(PositionType::Absolute),
            _ => return Err(anyhow!("Invalid FlexWrap")),
        }
    }

    fn parse_flex_wrap(bytes: &[u8]) -> Result<FlexWrap> {
        match bytes {
            b"no-wrap" => Ok(FlexWrap::NoWrap),
            b"wrap" => Ok(FlexWrap::Wrap),
            b"wrap-reverse" => Ok(FlexWrap::WrapReverse),
            _ => return Err(anyhow!("Invalid FlexWrap")),
        }
    }

    fn parse_flex_dir(bytes: &[u8]) -> Result<FlexDirection> {
        match bytes {
            b"row" => Ok(FlexDirection::Row),
            b"column" => Ok(FlexDirection::Column),
            b"row-reverse" => Ok(FlexDirection::RowReverse),
            b"column-reverse" => Ok(FlexDirection::ColumnReverse),
            _ => return Err(anyhow!("Invalid FlexDirection")),
        }
    }

    fn parse_flex_align(bytes: &[u8]) -> Result<FlexAlign> {
        match bytes {
            b"inherit" => Ok(FlexAlign::Inherit),
            b"stretch" => Ok(FlexAlign::Stretch),
            b"start" => Ok(FlexAlign::Start),
            b"center" => Ok(FlexAlign::Center),
            b"end" => Ok(FlexAlign::End),
            b"space-between" => Ok(FlexAlign::SpaceBetween),
            b"space-around" => Ok(FlexAlign::SpaceAround),
            b"baseline" => Ok(FlexAlign::Baseline),
            _ => return Err(anyhow!("Invalid FlexAlign")),
        }
    }

    let mut layout = Layout::new();
    if let Some(bytes) = attrs.get_attribute("position").flatten() {
        layout.with_position_type(parse_position_type(bytes.raw())?);
    }
    if let Some(bytes) = attrs.get_attribute("flex-wrap").flatten() {
        layout.with_flex_wrap(parse_flex_wrap(bytes.raw())?);
    }
    if let Some(bytes) = attrs.get_attribute("flex-direction").flatten() {
        layout.with_flex_direction(parse_flex_dir(bytes.raw())?);
    }
    if let Some(bytes) = attrs.get_attribute("align-items").flatten() {
        layout.with_align_items(parse_flex_align(bytes.raw())?);
    }
    if let Some(bytes) = attrs.get_attribute("align-content").flatten() {
        layout.with_align_content(parse_flex_align(bytes.raw())?);
    }
    if let Some(bytes) = attrs.get_attribute("justify-content").flatten() {
        layout.with_justify_content(parse_flex_align(bytes.raw())?);
    }
    if let Some(bytes) = attrs.get_attribute("width").flatten() {
        layout.with_width(&bytes.as_utf8_str());
    }
    if let Some(bytes) = attrs.get_attribute("height").flatten() {
        layout.with_height(&bytes.as_utf8_str());
    }
    if let Some(bytes) = attrs.get_attribute("padding").flatten() {
        let val = bytes.as_utf8_str().parse()?;
        layout.with_padding(val);
    }
    if let Some(bytes) = attrs.get_attribute("margin").flatten() {
        let val = bytes.as_utf8_str().parse()?;
        layout.with_margin(val);
    }
    if let Some(bytes) = attrs.get_attribute("flex-grow").flatten() {
        let val = bytes.as_utf8_str().parse()?;
        layout.with_flex_grow(val);
    }

    Ok(layout)
}

mod test {
    #[test]
    fn parse_simple_box() {
        let dom = tl::parse("<box flex-grow='1'>as</box>", tl::ParserOptions::default());
        let parser = dom.parser();
        let node = dom.children().first().unwrap().get(&parser).unwrap();
        let tag: &str = &node.as_tag().unwrap().name().as_utf8_str();
        assert_eq!(tag, "box");
    }
}
