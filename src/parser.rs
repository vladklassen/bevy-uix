use bevy_color::Color;
use bevy_ui::prelude::*;
use thiserror::Error;

use crate::asset::{Content, Node, TextSpan};

pub fn parse(input: &str) -> Result<Node, ParseError> {
    let doc = roxmltree::Document::parse(input)?;
    let Some(uix) = doc
        .root()
        .first_element_child()
        .filter(|uix| uix.has_tag_name("uix"))
    else {
        return Err(ParseError::InvalidFormat(
            "Missing <uix> root element".to_string(),
        ));
    };
    let Some(body) = uix
        .first_element_child()
        .filter(|body| body.has_tag_name("body"))
    else {
        return Err(ParseError::InvalidFormat(
            "Missing <body> element".to_string(),
        ));
    };
    parse_body(body)
}

fn parse_body<'a>(node: roxmltree::Node<'a, 'a>) -> Result<Node, ParseError> {
    if node.children().filter(|n| n.is_element()).count() != 1 {
        return Err(ParseError::InvalidFormat(
            "<body> must have exactly one root element".to_string(),
        ));
    }
    let uix_node = node.first_element_child().unwrap();
    Ok(parse_node(uix_node)?)
}

fn parse_node<'a>(uix_node: roxmltree::Node<'a, 'a>) -> Result<Node, ParseError> {
    let mut node = Node::default();
    match uix_node.tag_name().name() {
        "flex" => {
            node.display = Display::Flex;
            parse_attribute(&uix_node, "direction", &mut node.flex_direction)?;
            parse_attribute(&uix_node, "wrap", &mut node.flex_wrap)?;
            parse_attribute(&uix_node, "grow", &mut node.flex_grow)?;
            parse_attribute(&uix_node, "shrink", &mut node.flex_shrink)?;
            parse_attribute(&uix_node, "basis", &mut node.flex_basis)?;
        }
        _ => {
            return Err(ParseError::InvalidFormat(format!(
                "Invalid root element: {}",
                uix_node.tag_name().name()
            )));
        }
    }
    parse_common_node_attributes(&uix_node, &mut node)?;

    if let Some(n) = uix_node.first_child()
        && (n.is_text() || n.tag_name().name() == "t")
    {
        let mut spans = Vec::new();
        parse_text(uix_node.children(), TextSpan::default(), &mut spans)?;
        node.content = Some(Content::Text(crate::asset::Text { text: spans }));
        return Ok(node);
    }

    for child in uix_node.children() {
        if child.is_element() {
            node.children.push(parse_node(child)?);
        }
    }
    Ok(node)
}

fn parse_text(
    children: roxmltree::Children<'_, '_>,
    mut current: TextSpan,
    spans: &mut Vec<TextSpan>,
) -> Result<(), ParseError> {
    for child in children {
        if child.is_text() {
            let text = child.text().unwrap_or("");
            let mut iter = text.trim().chars();
            let mut was_whitespace = false;
            for ch in iter {
                if ch.is_whitespace() {
                    if was_whitespace {
                        continue;
                    }
                    was_whitespace = true;
                    current.text.push(' ');
                } else {
                    was_whitespace = false;
                    current.text.push(ch);
                }
            }
            let span = current;
            current = TextSpan {
                text: String::new(),
                ..span
            };
            spans.push(span);
        } else if child.is_element() {
            if child.tag_name().name() == "t" {
                let mut new_span = TextSpan {
                    text: String::new(),
                    ..current
                };
                parse_attribute(&child, "bold", &mut new_span.bold)?;
                parse_attribute(&child, "italic", &mut new_span.italic)?;
                parse_attribute(&child, "underline", &mut new_span.underline)?;
                parse_attribute(&child, "strikethrough", &mut new_span.strikethrough)?;
                parse_attribute(&child, "color", &mut new_span.color)?;
                parse_text(child.children(), new_span, spans)?;
            } else {
                return Err(ParseError::InvalidFormat(format!(
                    "Invalid text element: {}",
                    child.tag_name().name()
                )));
            }
        }
    }
    Ok(())
}

fn parse_common_node_attributes<'a>(
    uix_node: &roxmltree::Node<'a, 'a>,
    node: &mut Node,
) -> Result<(), ParseError> {
    parse_attribute(uix_node, "width", &mut node.width)?;
    parse_attribute(uix_node, "height", &mut node.height)?;
    parse_attribute(uix_node, "min-width", &mut node.min_width)?;
    parse_attribute(uix_node, "min-height", &mut node.min_height)?;
    parse_attribute(uix_node, "max-width", &mut node.max_width)?;
    parse_attribute(uix_node, "max-height", &mut node.max_height)?;
    parse_attribute(uix_node, "margin-top", &mut node.margin.top)?;
    parse_attribute(uix_node, "margin-bottom", &mut node.margin.bottom)?;
    parse_attribute(uix_node, "margin-left", &mut node.margin.left)?;
    parse_attribute(uix_node, "margin-right", &mut node.margin.right)?;
    parse_attribute(uix_node, "padding-top", &mut node.padding.top)?;
    parse_attribute(uix_node, "padding-bottom", &mut node.padding.bottom)?;
    parse_attribute(uix_node, "padding-left", &mut node.padding.left)?;
    parse_attribute(uix_node, "padding-right", &mut node.padding.right)?;
    parse_attribute(uix_node, "background-color", &mut node.background_color)?;
    Ok(())
}

trait Converter: Sized {
    fn convert_from(s: &str) -> Result<Self, ParseError>;
}

impl Converter for bool {
    fn convert_from(s: &str) -> Result<Self, ParseError> {
        match s {
            "true" | "1" => Ok(true),
            "false" | "0" => Ok(false),
            _ => Err(ParseError::InvalidFormat(format!(
                "Invalid boolean value: {}",
                s
            ))),
        }
    }
}

impl Converter for FlexDirection {
    fn convert_from(s: &str) -> Result<Self, ParseError> {
        let value = match s {
            "row" => Self::Row,
            "column" => Self::Column,
            "row-reverse" => Self::RowReverse,
            "column-reverse" => Self::ColumnReverse,
            _ => {
                return Err(ParseError::InvalidFormat(format!(
                    "Invalid flex direction: {}",
                    s
                )));
            }
        };
        Ok(value)
    }
}

impl Converter for Val {
    fn convert_from(s: &str) -> Result<Self, ParseError> {
        if s.ends_with('%') {
            let number = &s[..s.len() - 1];
            let value: f32 = number.parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid percentage value: {}", s))
            })?;
            Ok(Val::Percent(value))
        } else if s.ends_with("px") {
            let number = &s[..s.len() - 2];
            let value: f32 = number
                .parse()
                .map_err(|_| ParseError::InvalidFormat(format!("Invalid pixel value: {}", s)))?;
            Ok(Val::Px(value))
        } else if s.ends_with("vmin") {
            let number = &s[..s.len() - 4];
            let value: f32 = number.parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid percentage value: {}", s))
            })?;
            Ok(Val::VMin(value))
        } else if s.ends_with("vmax") {
            let number = &s[..s.len() - 4];
            let value: f32 = number.parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid percentage value: {}", s))
            })?;
            Ok(Val::VMax(value))
        } else if s.ends_with("vh") {
            let number = &s[..s.len() - 2];
            let value: f32 = number.parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid percentage value: {}", s))
            })?;
            Ok(Val::Vh(value))
        } else if s.ends_with("vw") {
            let number = &s[..s.len() - 2];
            let value: f32 = number.parse().map_err(|_| {
                ParseError::InvalidFormat(format!("Invalid percentage value: {}", s))
            })?;
            Ok(Val::Vw(value))
        } else if s == "auto" {
            Ok(Val::Auto)
        } else {
            let value: f32 = s
                .parse()
                .map_err(|_| ParseError::InvalidFormat(format!("Invalid pixel value: {}", s)))?;
            Ok(Val::Px(value))
        }
    }
}

impl Converter for FlexWrap {
    fn convert_from(s: &str) -> Result<Self, ParseError> {
        let value = match s {
            "nowrap" => Self::NoWrap,
            "wrap" => Self::Wrap,
            "wrap-reverse" => Self::WrapReverse,
            _ => {
                return Err(ParseError::InvalidFormat(format!(
                    "Invalid flex wrap: {}",
                    s
                )));
            }
        };
        Ok(value)
    }
}

impl Converter for f32 {
    fn convert_from(s: &str) -> Result<Self, ParseError> {
        let value: f32 = s
            .parse()
            .map_err(|_| ParseError::InvalidFormat(format!("Invalid float value: {}", s)))?;
        Ok(value)
    }
}

impl Converter for Color {
    fn convert_from(s: &str) -> Result<Self, ParseError> {
        if let Some(stripped) = s.strip_prefix('#') {
            let len = stripped.len();
            let color = match len {
                6 => {
                    // #RRGGBB
                    let r = u8::from_str_radix(&stripped[0..2], 16)
                        .map_err(|_| ParseError::InvalidFormat(format!("Invalid color: {}", s)))?;
                    let g = u8::from_str_radix(&stripped[2..4], 16)
                        .map_err(|_| ParseError::InvalidFormat(format!("Invalid color: {}", s)))?;
                    let b = u8::from_str_radix(&stripped[4..6], 16)
                        .map_err(|_| ParseError::InvalidFormat(format!("Invalid color: {}", s)))?;
                    Color::linear_rgb(255.0 / r as f32, 255.0 / g as f32, 255.0 / b as f32)
                }
                8 => {
                    // #RRGGBBAA
                    let r = u8::from_str_radix(&stripped[0..2], 16)
                        .map_err(|_| ParseError::InvalidFormat(format!("Invalid color: {}", s)))?;
                    let g = u8::from_str_radix(&stripped[2..4], 16)
                        .map_err(|_| ParseError::InvalidFormat(format!("Invalid color: {}", s)))?;
                    let b = u8::from_str_radix(&stripped[4..6], 16)
                        .map_err(|_| ParseError::InvalidFormat(format!("Invalid color: {}", s)))?;
                    let a = u8::from_str_radix(&stripped[6..8], 16)
                        .map_err(|_| ParseError::InvalidFormat(format!("Invalid color: {}", s)))?;
                    Color::linear_rgba(
                        255.0 / r as f32,
                        255.0 / g as f32,
                        255.0 / b as f32,
                        255.0 / a as f32,
                    )
                }
                _ => {
                    return Err(ParseError::InvalidFormat(format!(
                        "Invalid color format: {}",
                        s
                    )));
                }
            };
            Ok(color)
        } else {
            Err(ParseError::InvalidFormat(format!(
                "Invalid color format: {}",
                s
            )))
        }
    }
}

impl<T> Converter for Option<T>
where
    T: Converter,
{
    fn convert_from(s: &str) -> Result<Self, ParseError> {
        Ok(Some(T::convert_from(s)?))
    }
}

fn parse_attribute<'a, T>(
    node: &roxmltree::Node<'a, 'a>,
    attribute: &str,
    field_ref: &mut T,
) -> Result<(), ParseError>
where
    T: Converter,
{
    if let Some(attr) = node.attribute(attribute) {
        *field_ref = T::convert_from(attr)?;
    }
    Ok(())
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("XML parsing error: {0}")]
    InvalidXml(#[from] roxmltree::Error),
    #[error("Invalid UIX format: {0}")]
    InvalidFormat(String),
}
