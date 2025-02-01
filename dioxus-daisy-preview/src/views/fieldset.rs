use dioxus::prelude::*;
use dioxus_daisy::components::*;

use crate::components::*;

use crate::with_source;

#[component]
pub fn FieldsetPreview() -> Element {
    let variants = vec![
        (
            "Fieldset with legend and label",
            Some(with_source!(
                Fieldset {
                    FieldLegend { "Page title" }
                    TextInput {
                        placeholder: "Enter page title",

                    }
                    FieldLabel { "You can edit page title later on from settings" }
                }
            )),
        ),
        (
            "Fieldset with background and border",
            Some(with_source!(
                Fieldset {
                    border: true,
                    legend: "Page title",
                    TextInput {
                        placeholder: "Enter page title",
                    }
                    FieldLabel { "You can edit page title later on from settings" }
                }
            )),
        ),
        (
            "Fieldset with multiple inputs",
            Some(with_source!(
                Fieldset {
                    border: true,
                    legend: "Page details",

                    FieldLabel { "Title" }
                    TextInput {
                        placeholder: "My awesome page",
                    }

                    FieldLabel { "Slug" }
                    TextInput {
                        placeholder: "my-awesome-page",
                    }

                    FieldLabel { "Author" }
                    TextInput {
                        placeholder: "Name"
                    }
                }
            )),
        ),
    ];

    rsx! {
        ComponentVariantList { title: "Fieldset", variants }
    }
}
