use dioxus::prelude::*;

#[component]
pub fn ComponentVariantList(
    title: &'static str,
    variants: Vec<(&'static str, Option<(Element, String)>)>,
) -> Element {
    rsx! {
        div { class: "flex flex-col items-center px-6 pb-16 h-full overflow-y-auto",
            div { class: "w-full max-w-4xl",
                div { class: "grid grid-cols-1 gap-8",
                    h1 { class: "text-4xl mx-2 font-bold", "{title}" }
                    for (title , item) in variants {
                        ComponentPreview { title, item }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ComponentPreview(title: String, item: Option<(Element, String)>) -> Element {
    use convert_case::{Case, Casing};
    let id = title.to_case(Case::Kebab);

    let (preview, rsx, html) = if let Some((preview, rsx)) = item {
        let mut html = dioxus_ssr::render_element(preview.clone());
        // Small hack to introduce new lines in the HTML output
        // TODO: This does not indent the HTML properly
        html = html.replace("><", ">\n<");
        (preview, rsx, html)
    } else {
        let placeholder = "🚧 Coming soon...".to_string();
        let preview = rsx! {
            div { role: "alert", class: "alert alert-warning alert-soft",
                span { "{placeholder}" }
            }
        };
        (preview, placeholder.clone(), placeholder)
    };

    rsx! {
        div { class: "w-full max-w-4xl",
            div { id: "{id}", class: "flex flex-row items-center my-2",
                Link {
                    class: "btn btn-sm mr-2 btn-outline btn-square border-base-200",
                    to: "#{id}",
                    span { class: "font-light", "#" }
                }
                h2 { class: "font-bold text-lg", {title} }
            }
            div { role: "tablist", class: "tabs tabs-lift",
                input {
                    role: "tab",
                    r#type: "radio",
                    name: "preview-tabs-{id}",
                    class: "tab",
                    aria_label: "Preview",
                    checked: true,
                }
                div { class: "tab-content border-base-300 border-2 bg-diagonal-stripes",
                    div {
                        class: "min-h-[6rem] min-w-[18rem] p-10",
                        class: "flex flex-wrap items-center justify-center gap-2",
                        class: "p-4 xl:py-10",
                        {preview}
                    }
                }
                input {
                    role: "tab",
                    r#type: "radio",
                    name: "preview-tabs-{id}",
                    class: "tab",
                    aria_label: "RSX",
                }
                div { class: "tab-content border-base-300 border-2 bg-base-300",
                    pre {
                        class: "textarea bg-transparent",
                        class: "!w-full min-h-[6rem] min-w-[18rem]",
                        class: "border-0 focus:outline-none",
                        overflow: "auto",
                        {rsx}
                    }
                }
                input {
                    role: "tab",
                    r#type: "radio",
                    name: "preview-tabs-{id}",
                    class: "tab",
                    aria_label: "HTML",
                }
                div { class: "tab-content border-base-300 border-2 bg-base-300",
                    pre {
                        class: "textarea bg-transparent",
                        class: "!w-full min-h-[6rem] min-w-[18rem]",
                        class: "border-0 focus:outline-none",
                        overflow: "auto",
                        {html}
                    }
                }
            }
        }
    }
}
