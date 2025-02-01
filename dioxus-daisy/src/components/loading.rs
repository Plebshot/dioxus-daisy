use dioxus::prelude::*;

use crate::{
    attributes::{Color, Size},
    css_class, merge_class,
};

#[css_class(prefix = "loading-")]
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
    #[props(extends = GlobalAttributes, extends = button)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let class = merge_class!("loading", size => "loading-{}", color => "text-{}", style);
    rsx! {
        span { class, ..attributes, {children} }
    }
}
