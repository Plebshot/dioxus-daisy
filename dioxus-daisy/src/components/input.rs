use dioxus::prelude::*;

use crate::attributes::{Color, Size};

#[component]
pub fn TextInput(
    size: Option<Size>,
    color: Option<Color>,
    #[props(default)] ghost: bool,
    #[props(extends = GlobalAttributes, extends = input)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        input {
            r#type: "text",
            class: "input",
            class: if ghost {"input-ghost"},
            class: if let Some(Size::Xs) = size {"input-xs"},
            class: if let Some(Size::Sm) = size {"input-sm"},
            class: if let Some(Size::Md) = size {"input-md"},
            class: if let Some(Size::Lg) = size {"input-lg"},
            class: if let Some(Size::Xl) = size {"input-xl"},
            class: if let Some(Color::Primary) = color {"input-primary"},
            class: if let Some(Color::Secondary) = color {"input-secondary"},
            class: if let Some(Color::Success) = color {"input-success"},
            class: if let Some(Color::Accent) = color {"input-accent"},
            class: if let Some(Color::Warning) = color {"input-warning"},
            class: if let Some(Color::Info) = color {"input-info"},
            class: if let Some(Color::Error) = color {"input-error"},
            ..attributes,
            {children}
        }
    }
}
