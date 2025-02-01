use dioxus::prelude::*;

use crate::attributes::{Color, Size};

#[derive(Clone, PartialEq)]
pub enum ButtonStyle {
    Outline,
    Dash,
    Soft,
    Ghost,
    Link,
    Active,
    Disabled,
}

#[derive(Clone, PartialEq)]
pub enum ButtonModifier {
    Wide,
    Block,
    Square,
    Circle,
}

#[component]
pub fn Button(
    color: Option<Color>,
    size: Option<Size>,
    style: Option<ButtonStyle>,
    modifier: Option<ButtonModifier>,
    // NOTE: We currently have no way to extend all event handlers, so
    // we just pass the onclick handler manually for now.
    // https://github.com/DioxusLabs/dioxus/issues/2467
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: "btn",
            class: if let Some(ButtonStyle::Outline) = style {"btn-outline"},
            class: if let Some(ButtonStyle::Dash) = style {"btn-dash"},
            class: if let Some(ButtonStyle::Soft) = style {"btn-soft"},
            class: if let Some(ButtonStyle::Ghost) = style {"btn-ghost"},
            class: if let Some(ButtonStyle::Link) = style {"btn-link"},
            class: if let Some(ButtonStyle::Active) = style {"btn-active"},
            class: if let Some(ButtonStyle::Disabled) = style {"btn-disabled"},
            class: if let Some(ButtonModifier::Wide) = modifier {"btn-wide"},
            class: if let Some(ButtonModifier::Block) = modifier {"btn-block"},
            class: if let Some(ButtonModifier::Square) = modifier {"btn-square"},
            class: if let Some(ButtonModifier::Circle) = modifier {"btn-circle"},
            class: if let Some(Size::Xs) = size {"btn-xs"},
            class: if let Some(Size::Sm) = size {"btn-sm"},
            class: if let Some(Size::Md) = size {"btn-md"},
            class: if let Some(Size::Lg) = size {"btn-lg"},
            class: if let Some(Size::Xl) = size {"btn-xl"},
            class: if let Some(Color::Primary) = color {"btn-primary"},
            class: if let Some(Color::Secondary) = color {"btn-secondary"},
            class: if let Some(Color::Success) = color {"btn-success"},
            class: if let Some(Color::Accent) = color {"btn-accent"},
            class: if let Some(Color::Warning) = color {"btn-warning"},
            class: if let Some(Color::Info) = color {"btn-info"},
            class: if let Some(Color::Error) = color {"btn-error"},
            onclick: move |event| {
                if let Some(handler) = onclick {
                    handler(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}
