use dioxus::prelude::*;
use dioxus_daisy::attributes::Color;
use dioxus_daisy::attributes::Size;
use dioxus_daisy::components::*;

use crate::components::*;

use crate::with_source;

#[component]
pub fn InputPreview() -> Element {
    let variants = vec![
        (
            "Text input",
            Some(with_source!(TextInput {
                placeholder: "Type here",
            })),
        ),
        ("Text input with text label inside", None),
        (
            "Ghost style",
            Some(with_source!(TextInput {
                ghost: true,
                placeholder: "Type here",
            })),
        ),
        (
            "With fieldset and legend",
            Some(with_source!(
                Fieldset {
                    legend: "What is your name?",
                    TextInput {
                        placeholder: "Type here",
                    }
                    FieldLabel { "Optional" }

                }
            )),
        ),
        (
            "Colors",
            Some(with_source!(
                TextInput {
                    placeholder: "Neutral",
                    color: Color::Neutral,
                }
                TextInput {
                    placeholder: "Primary",
                    color: Color::Primary,
                }
                TextInput {
                    placeholder: "Secondary",
                    color: Color::Secondary,
                }
                TextInput {
                    placeholder: "Accent",
                    color: Color::Accent,
                }
                TextInput {
                    placeholder: "Success",
                    color: Color::Success,
                }
                TextInput {
                    placeholder: "Warning",
                    color: Color::Warning,
                }
                TextInput {
                    r#type: "text",
                    placeholder: "Error",
                    color: Color::Error,
                }
            )),
        ),
        (
            "Sizes",
            Some(with_source!(
                TextInput {
                    placeholder: "Xsmall",
                    size: Size::Xs,
                }
                TextInput {
                    placeholder: "Small",
                    size: Size::Sm,
                }
                TextInput {
                    placeholder: "Medium",
                    size: Size::Md,
                }
                TextInput {
                    placeholder: "Large",
                    size: Size::Lg,
                }
                TextInput {
                    placeholder: "Xlarge",
                    size: Size::Xl,
                }
            )),
        ),
        (
            "Disabled",
            Some(with_source!(TextInput {
                r#type: "text",
                placeholder: "Disabled",
                disabled: true,
            })),
        ),
        ("Input types", None),
    ];

    rsx! {
        ComponentVariantList { title: "Input", variants }
    }
}
