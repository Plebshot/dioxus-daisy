use dioxus::prelude::*;

mod components;
mod routes;
mod views;

use routes::Route;

static ACTIVE_THEME: GlobalSignal<String> = GlobalSignal::new(|| "dark".to_string());

fn main() {
    #[cfg(feature = "desktop")]
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::default()
                .with_menu(None)
                .with_window(dioxus::desktop::WindowBuilder::default().with_always_on_top(false)),
        )
        .launch(App);

    #[cfg(not(feature = "desktop"))]
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/daisyui.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/themes.css") }
        body { "data-theme": ACTIVE_THEME.resolve(), Router::<Route> {} }
    }
}

#[macro_export]
macro_rules! with_source {
    ($($rsx_tokens:tt)*) => {{
        let rendered = rsx! { $($rsx_tokens)* };
        let source = stringify!($($rsx_tokens)*);
        let options = dioxus_autofmt::IndentOptions::default();
        let formatted = dioxus_autofmt::fmt_block(source, 0, options).unwrap();

        // TODO: The formatting here is somewhat inefficient, but the fmt_block
        //       adds an empty newline and at least one level of indentation for
        //       some reason. This is a temporary workaround.
        let trimmed = formatted
            // Remove empty lines at the start
            .trim_start_matches('\n')
            .lines()
            // Remove 4-space indentation if present
            .map(|l| l.strip_prefix("    ").unwrap_or(l))
            .collect::<Vec<_>>()
            .join("\n");

        (rendered, trimmed)
    }};
}
