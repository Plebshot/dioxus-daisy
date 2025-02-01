use dioxus::prelude::*;

use crate::{Route, ACTIVE_THEME};

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { class: "bg-base-100 drawer max-w-[100rem] mx-auto lg:drawer-open",
            input {
                r#type: "checkbox",
                id: "nav-drawer",
                class: "drawer-toggle",
            }
            div { class: "drawer-content",
                div { class: "bg-base-100/90 print:hidden text-base-content sticky top-0 z-30 flex h-16 w-full justify-center backdrop-blur transition-shadow duration-100 [transform:translate3d(0,0,0)]",
                    nav { class: "navbar w-full",
                        div { class: "flex flex-1 md-gap-1 lg:gap-2 items-center",
                            label {
                                class: "lg:hidden btn btn-square btn-ghost drawer-button",
                                r#for: "nav-drawer",
                                aria_label: "Open menu",
                                svg {
                                    class: "lg:hidden inline-block h-5 w-5 stroke-current md:h-6 md-w-6",
                                    width: "20",
                                    height: "20",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_width: "2",
                                        d: "M4 6h16M4 12h16M4 18h16",
                                    }
                                }
                            }
                            span { class: "lg:hidden text-lg font-bold", "🌼 Dioxus DaisyUI Preview" }
                        }
                        div { class: "flex", Themes {} }
                    }
                }
                Outlet::<Route> {}
            }
            div { class: "drawer-side z-40", scroll_behavior: "smooth",
                label {
                    class: "drawer-overlay",
                    r#for: "nav-drawer",
                    aria_label: "Close menu",
                }
                aside { class: "bg-base-100 min-h-screen w-80",
                    div { class: "sm:max-lg:hidden bg-base-100/90 sticky top-0 z-20 navbar items-center gap-2 px-4 py-2 backdrop-blur lg:flex shadow-xs",
                        span { class: "sm:max-lg:hidden text-lg font-bold",
                            "🌼 Dioxus DaisyUI Preview"
                        }
                    }
                    ul { class: "menu w-full px-4 py-0",
                        li {
                            h2 { class: "menu-title flex items-center gap-4 px-1.5",
                                "Actions"
                            }
                            ul {
                                li {
                                    Link {
                                        class: "group",
                                        to: Route::ButtonPreview,
                                        "Button"
                                    }
                                }
                            }
                        }
                        li {
                            h2 { class: "menu-title flex items-center gap-4 px-1.5",
                                "Feedback"
                            }
                            ul {
                                li {
                                    Link {
                                        class: "group",
                                        to: Route::LoadingPreview,
                                        "Loading"
                                    }
                                }
                            }
                        }
                    }
                    div { class: "bg-base-100 pointer-events-none sticky bottom-0 flex h-40 [mask-image:linear-gradient(transparent,#000000)]" }
                }
            }
        }
    }
}

#[component]
fn Themes() -> Element {
    let themes = [
        "light",
        "dark",
        "cupcake",
        "bumblebee",
        "emerald",
        "corporate",
        "synthwave",
        "retro",
        "cyberpunk",
        "valentine",
        "halloween",
        "garden",
        "forest",
        "aqua",
        "lofi",
        "pastel",
        "fantasy",
        "wireframe",
        "black",
        "luxury",
        "dracula",
        "cmyk",
        "autumn",
        "business",
        "acid",
        "lemonade",
        "night",
        "coffee",
        "winter",
        "dim",
        "nord",
        "sunset",
    ];

    rsx! {
        div { class: "dropdown",
            button { tabindex: "0", class: "btn btn-ghost",
                "Theme"
                svg {
                    width: "12",
                    height: "12",
                    class: "hidden h-2 w-2 fill-current opacity-60 sm:inline-block",
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 2048 2048",
                    path { d: "M1799 349l242 241-1017 1017L7 590l242-241 775 775 775-775z" }
                }
            }
            ul {
                tabindex: "0",
                class: "dropdown-content bg-base-100 rounded-box w-lg p-2 shadow-2xl grid grid-cols-3 gap-1 right-0 left-auto",
                for value in themes {
                    li {
                        input {
                            r#type: "radio",
                            name: "theme-buttons",
                            class: "btn btn-ghost w-full",
                            aria_label: value[0..1].to_uppercase() + &value[1..],
                            value,
                            onclick: move |_| {
                                *ACTIVE_THEME.write() = value.to_string();
                            },
                        }
                    }
                }
            }
        }
    }
}
