use dioxus::prelude::*;

use crate::attributes::{Color, Size};

#[derive(Clone, PartialEq)]
pub enum LoadingStyle {
    Spinner,
    Dots,
    Ring,
    Ball,
    Bars,
    Infinity,
}

#[component]
pub fn Loading(
    size: Option<Size>,
    style: Option<LoadingStyle>,
    color: Option<Color>,
    #[props(extends = GlobalAttributes, extends = span)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        span {
            class: "loading",
            class: if let Some(LoadingStyle::Spinner) = style {"loading-spinner"},
            class: if let Some(LoadingStyle::Dots) = style {"loading-dots"},
            class: if let Some(LoadingStyle::Ring) = style {"loading-ring"},
            class: if let Some(LoadingStyle::Ball) = style {"loading-ball"},
            class: if let Some(LoadingStyle::Bars) = style {"loading-bars"},
            class: if let Some(LoadingStyle::Infinity) = style {"loading-infinity"},
            class: if let Some(Size::Xs) = size {"btn-xs"},
            class: if let Some(Size::Sm) = size {"btn-sm"},
            class: if let Some(Size::Md) = size {"btn-md"},
            class: if let Some(Size::Lg) = size {"btn-lg"},
            class: if let Some(Size::Xl) = size {"btn-xl"},
            class: if let Some(Color::Primary) = color {"text-primary"},
            class: if let Some(Color::Secondary) = color {"text-secondary"},
            class: if let Some(Color::Success) = color {"text-success"},
            class: if let Some(Color::Accent) = color {"text-danger"},
            class: if let Some(Color::Warning) = color {"text-warning"},
            class: if let Some(Color::Info) = color {"text-info"},
            ..attributes, {children}
        }
    }
}
