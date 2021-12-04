use anyhow::{anyhow, Result};
use flexlayout_rs::{Dimension, Node, NodeWithLayout};
use red4ext_rs::prelude::*;

use crate::redscript;

pub fn build(elem: redscript::Elem) -> Node<redscript::Elem> {
    let children = elem.children().into_iter().map(build).collect();
    let props = elem.layout().properties();
    Node::new(children, props, elem)
}

pub fn render(elem: NodeWithLayout<redscript::Elem>) -> redscript::Widget {
    let pos = redscript::Vector2::new(elem.left(), elem.top());
    let size = redscript::Vector2::new(elem.width(), elem.height());

    let widget = call!(elem.inner().context().repr.clone(), "Render" (pos, size) -> redscript::Widget);

    for child in elem.children() {
        let child_widget = render(child);
        call!(widget.repr.clone(), "AddChildWidget" (child_widget) -> ());
    }

    widget
}

pub fn parse_dimension(str: &str) -> Result<Dimension> {
    match str {
        "auto" => Ok(Dimension::Auto),
        other => {
            let idx = other
                .replace(" ", "")
                .find(|c: char| !c.is_digit(10) && c != '.')
                .ok_or_else(|| anyhow!("Missing unit"))?;
            match other.split_at(idx) {
                (num, "pt") => Ok(Dimension::Point(num.parse()?)),
                (num, "%") => Ok(Dimension::Percent(num.parse()?)),
                _ => Err(anyhow!("Invalid dimension literal")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use flexlayout_rs::Dimension;

    use super::parse_dimension;

    #[test]
    fn it_parses_percentages() {
        assert!(matches!(parse_dimension("100%"), Ok(Dimension::Percent(v)) if v == 100.0));
    }

    #[test]
    fn it_parses_points() {
        assert!(matches!(parse_dimension("320pt"), Ok(Dimension::Point(v)) if v == 320.0));
    }
}
