use bevy_asset::{Asset, Handle};
use bevy_color::Color;
use bevy_reflect::TypePath;
use bevy_text::Font;
use bevy_ui::prelude::*;

#[derive(Asset, TypePath)]
pub struct UIX {
    pub root: Node,
}

pub enum Content {
    Text(Text),
}

pub struct Node {
    pub display: Display,

    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Val,

    pub width: Val,
    pub height: Val,
    pub min_width: Val,
    pub min_height: Val,
    pub max_width: Val,
    pub max_height: Val,
    pub top: Val,
    pub bottom: Val,
    pub left: Val,
    pub right: Val,
    pub margin: UiRect,
    pub padding: UiRect,

    pub background_color: Option<Color>,

    pub content: Option<Content>,

    pub children: Vec<Node>,
}

impl Default for Node {
    fn default() -> Self {
        let ui_node = bevy_ui::Node::default();

        Self {
            display: Display::Flex,

            flex_direction: ui_node.flex_direction,
            flex_wrap: ui_node.flex_wrap,
            flex_grow: ui_node.flex_grow,
            flex_shrink: ui_node.flex_shrink,
            flex_basis: ui_node.flex_basis,

            width: ui_node.width,
            height: ui_node.height,
            min_width: ui_node.min_width,
            min_height: ui_node.min_height,
            max_width: ui_node.max_width,
            max_height: ui_node.max_height,
            top: ui_node.top,
            bottom: ui_node.bottom,
            left: ui_node.left,
            right: ui_node.right,
            margin: ui_node.margin,
            padding: ui_node.padding,

            background_color: None,

            content: None,

            children: Vec::new(),
        }
    }
}

pub struct Text {
    pub text: Vec<TextSpan>,
}

pub struct TextSpan {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub color: Color,
}

impl Default for TextSpan {
    fn default() -> Self {
        Self {
            text: String::new(),
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            color: Color::BLACK,
        }
    }
}
