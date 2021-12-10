use anyhow::{anyhow, Result};
use flexlayout_rs::{FlexAlign, FlexDirection, FlexWrap};
use red4ext_rs::prelude::*;

use crate::redscript::{Color, Elem, Layout};

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
                let elem = call!("Flexy.UI.Box::New;" () -> Elem);
                let mut layout = call!("Flexy.Layout.Layout::New;" () -> Layout);

                for child in tag.children().filter_map(|handle| handle.get(parser)) {
                    if let Some(child_elem) = parse_elem(child, parser)? {
                        call!(elem.repr.clone(), "Child" (child_elem) -> Elem);
                    }
                }

                let attrs = tag.attributes();
                parse_layout(attrs, &mut layout)?;

                call!(elem.repr.clone(), "Layout" (layout) -> Elem);

                if let Some(background) = attrs.get_attribute("background-color").flatten() {
                    let color = Color::from_hex(&background.as_utf8_str())?;
                    call!(elem.repr.clone(), "BackgroundColor" (color) -> Elem);
                }

                Ok(Some(elem))
            }
            _ => Err(anyhow!("Unexpected tag")),
        },
        tl::Node::Raw(bytes) => {
            let str: &str = &bytes.as_utf8_str();
            let elem = call!("Flexy.UI.Text::New;String" (str) -> Elem);
            Ok(Some(elem))
        }
        tl::Node::Comment(_) => Ok(None),
    }
}

fn parse_layout(attrs: &tl::Attributes, layout: &mut Layout) -> Result<()> {
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

    if let Some(bytes) = attrs.get_attribute("flex-wrap").flatten() {
        layout.with_flex_wrap(parse_flex_wrap(bytes.raw())?);
    }
    if let Some(bytes) = attrs.get_attribute("flex-direction").flatten() {
        layout.with_flex_direction(parse_flex_dir(bytes.raw())?);
    }
    if let Some(bytes) = attrs.get_attribute("justify-content").flatten() {
        layout.with_justify_content(parse_flex_align(bytes.raw())?);
    }
    if let Some(bytes) = attrs.get_attribute("align-content").flatten() {
        layout.with_align_content(parse_flex_align(bytes.raw())?);
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

    Ok(())
}

mod test {
    #[test]
    fn xd() {
        let dom = tl::parse("<box flex-grow=1>as</box>", tl::ParserOptions::default());
        let parser = dom.parser();
        let node = dom.children().first().unwrap().get(&parser).unwrap();
        let tag: &str = &node.as_tag().unwrap().name().as_utf8_str();
        assert_eq!(tag, "box");
    }
}
