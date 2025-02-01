use dioxus::prelude::*;

use crate::{
    attributes::{Color, Size},
    css_class, merge_class,
};

#[css_class(prefix = "btn-")]
pub enum ButtonStyle {
    Outline,
    Dash,
    Soft,
    Ghost,
    Link,
    Active,
    Disabled,
}

#[css_class(prefix = "btn-")]
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
    let class = merge_class!("btn", color => "btn-{}", size => "btn-{}", style, modifier);
    rsx! {
        button {
            class,
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
