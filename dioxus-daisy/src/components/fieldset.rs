use dioxus::prelude::*;

#[component]
pub fn Fieldset(
    legend: Option<String>,
    #[props(default)] border: bool,
    #[props(extends = GlobalAttributes, extends = fieldset)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        fieldset {
            class: "fieldset",
            class: if border {"w-xs bg-base-200 border border-base-300 p-4 rounded-box"},
            ..attributes,
            if let Some(legend) = legend {
                FieldLegend {
                    {legend}
                }
            },
            {children}
        }
    }
}

#[component]
pub fn FieldLegend(
    #[props(extends = GlobalAttributes, extends = legend)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        legend {
            class: "fieldset-legend",
            ..attributes, {children}
        }
    }
}

#[component]
pub fn FieldLabel(
    #[props(extends = GlobalAttributes, extends = p)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        p {
            class: "fieldset-label",
            ..attributes, {children}
        }
    }
}
