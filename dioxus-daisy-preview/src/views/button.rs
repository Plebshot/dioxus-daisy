use dioxus::prelude::*;

use crate::components::*;
use dioxus_daisy::{attributes::*, components::*};

use crate::with_source;

#[component]
pub fn ButtonPreview() -> Element {
    let variants = vec![
        (
            "Button",
            Some(with_source! {
                Button { "Default" }
            }),
        ),
        (
            "Button sizes",
            Some(with_source! {
                Button { size: Size::Xs, "Xsmall" }
                Button { size: Size::Sm, "Small" }
                Button { size: Size::Md, "Medium" }
                Button { size: Size::Lg, "Large" }
                Button { size: Size::Xl, "Xlarge" }
            }),
        ),
        (
            "Button colors",
            Some(with_source! {
                Button { color: Color::Neutral, "Neutral" }
                Button { color: Color::Primary, "Primary" }
                Button { color: Color::Secondary, "Secondary" }
                Button { color: Color::Accent, "Accent" }
                Button { color: Color::Info, "Info" }
                Button { color: Color::Success, "Success" }
                Button { color: Color::Warning, "Warning" }
                Button { color: Color::Error, "Error" }
            }),
        ),
        (
            "Soft buttons",
            Some(with_source! {
                Button { style: ButtonStyle::Soft, "Default" }
                Button { style: ButtonStyle::Soft, color: Color::Primary, "Primary" }
                Button { style: ButtonStyle::Soft, color: Color::Secondary, "Secondary" }
                Button { style: ButtonStyle::Soft, color: Color::Accent, "Accent" }
                Button { style: ButtonStyle::Soft, color: Color::Info, "Info" }
                Button { style: ButtonStyle::Soft, color: Color::Success, "Success" }
                Button { style: ButtonStyle::Soft, color: Color::Warning, "Warning" }
                Button { style: ButtonStyle::Soft, color: Color::Error, "Error" }
            }),
        ),
        ("Responsive button", None),
        (
            "Outline buttons",
            Some(with_source! {
                Button { style: ButtonStyle::Outline, "Default" }
                Button { style: ButtonStyle::Outline, color: Color::Primary, "Primary" }
                Button { style: ButtonStyle::Outline, color: Color::Secondary, "Secondary" }
                Button { style: ButtonStyle::Outline, color: Color::Accent, "Accent" }
                Button { style: ButtonStyle::Outline, color: Color::Info, "Info" }
                Button { style: ButtonStyle::Outline, color: Color::Success, "Success" }
                Button { style: ButtonStyle::Outline, color: Color::Warning, "Warning" }
                Button { style: ButtonStyle::Outline, color: Color::Error, "Error" }
            }),
        ),
        (
            "Dash buttons",
            Some(with_source! {
                Button { style: ButtonStyle::Dash, "Default" }
                Button { style: ButtonStyle::Dash, color: Color::Primary, "Primary" }
                Button { style: ButtonStyle::Dash, color: Color::Secondary, "Secondary" }
                Button { style: ButtonStyle::Dash, color: Color::Accent, "Accent" }
                Button { style: ButtonStyle::Dash, color: Color::Info, "Info" }
                Button { style: ButtonStyle::Dash, color: Color::Success, "Success" }
                Button { style: ButtonStyle::Dash, color: Color::Warning, "Warning" }
                Button { style: ButtonStyle::Dash, color: Color::Error, "Error" }
            }),
        ),
        (
            "Active buttons",
            Some(with_source! {
                Button { style: ButtonStyle::Active, "Default" }
                Button { style: ButtonStyle::Active, color: Color::Primary, "Primary" }
                Button { style: ButtonStyle::Active, color: Color::Secondary, "Secondary" }
                Button { style: ButtonStyle::Active, color: Color::Accent, "Accent" }
                Button { style: ButtonStyle::Active, color: Color::Info, "Info" }
                Button { style: ButtonStyle::Active, color: Color::Success, "Success" }
                Button { style: ButtonStyle::Active, color: Color::Warning, "Warning" }
                Button { style: ButtonStyle::Active, color: Color::Error, "Error" }
            }),
        ),
        (
            "Button ghost and button link",
            Some(with_source! {
                Button { style: ButtonStyle::Ghost, "Ghost" }
                Button { style: ButtonStyle::Link, "Link" }
            }),
        ),
        (
            "Wide button",
            Some(with_source! {
                Button { modifier: ButtonModifier::Wide, "Wide" }
            }),
        ),
        ("Buttons with any HTML tags", None),
        (
            "Disabled buttons",
            Some(with_source! {
                Button { disabled: true, "Disabled using attribute" }
            }),
        ),
        (
            "Circle button and square button",
            Some(with_source! {
                Button { modifier: ButtonModifier::Circle, svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "2.5",
                    stroke: "currentColor",
                    class:"size-[1.2em]",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"
                    }
                }}
                Button { modifier: ButtonModifier::Square, svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    stroke_width: "2.5",
                    stroke: "currentColor",
                    class:"size-[1.2em]",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        d: "M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"
                    }
                }}
            }),
        ),
        (
            "Button with Icon",
            Some(with_source! {
                Button {
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "2.5",
                        stroke: "currentColor",
                        class:"size-[1.2em]",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"
                        }
                    }
                    "Like"
                }
                Button {
                    "Like"
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 24 24",
                        stroke_width: "2.5",
                        stroke: "currentColor",
                        class:"size-[1.2em]",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M21 8.25c0-2.485-2.099-4.5-4.688-4.5-1.935 0-3.597 1.126-4.312 2.733-.715-1.607-2.377-2.733-4.313-2.733C5.1 3.75 3 5.765 3 8.25c0 7.22 9 12 9 12s9-4.78 9-12Z"
                        }
                    }
                }
            }),
        ),
        (
            "Button block",
            Some(with_source! {
                Button { modifier: ButtonModifier::Block, "Block" }
            }),
        ),
        (
            "Button with loading spinner",
            Some(with_source! {
                Button {
                    modifier: ButtonModifier::Square,
                    Loading { style: LoadingStyle::Spinner }
                }
                Button {
                    "Loading"
                    Loading { style: LoadingStyle::Spinner }
                }

            }),
        ),
        ("Login buttons", None),
    ];

    rsx! {
        ComponentVariantList { title: "Button", variants }
    }
}
