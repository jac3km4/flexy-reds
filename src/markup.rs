use anyhow::{anyhow, Result};
use flexlayout_rs::{FlexAlign, FlexDirection, FlexWrap};

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
                let attrs = tag.attributes();
                let children = tag
                    .children()
                    .filter_map(|handle| handle.get(parser))
                    .filter_map(|child| parse_elem(child, parser).transpose())
                    .collect::<Result<Vec<_>>>()?;
                let color = attrs
                    .get_attribute("background-color")
                    .flatten()
                    .and_then(|bytes| Color::from_hex(&bytes.as_utf8_str()).ok());

                let elem = Elem::new_box(children, color).with_layout(parse_layout(attrs)?);

                Ok(Some(elem))
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

fn parse_layout(attrs: &tl::Attributes) -> Result<Layout> {
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
