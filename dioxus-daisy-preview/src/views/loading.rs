use dioxus::prelude::*;
use dioxus_daisy::{attributes::*, components::*};

use crate::components::*;

use crate::with_source;

#[component]
pub fn LoadingPreview() -> Element {
    let variants = vec![
        (
            "Loading spinner",
            Some(with_source!(
                Loading { style: LoadingStyle::Spinner, size: Size::Xs }
                Loading { style: LoadingStyle::Spinner, size: Size::Md }
                Loading { style: LoadingStyle::Spinner, size: Size::Lg }
                Loading { style: LoadingStyle::Spinner, size: Size::Xl }
            )),
        ),
        (
            "Loading dots",
            Some(with_source!(
                Loading { style: LoadingStyle::Dots, size: Size::Xs }
                Loading { style: LoadingStyle::Dots, size: Size::Md }
                Loading { style: LoadingStyle::Dots, size: Size::Lg }
                Loading { style: LoadingStyle::Dots, size: Size::Xl }
            )),
        ),
        (
            "Loading ring",
            Some(with_source!(
                Loading { style: LoadingStyle::Ring, size: Size::Xs }
                Loading { style: LoadingStyle::Ring, size: Size::Md }
                Loading { style: LoadingStyle::Ring, size: Size::Lg }
                Loading { style: LoadingStyle::Ring, size: Size::Xl }
            )),
        ),
        (
            "Loading ball",
            Some(with_source!(
                Loading { style: LoadingStyle::Ball, size: Size::Xs }
                Loading { style: LoadingStyle::Ball, size: Size::Md }
                Loading { style: LoadingStyle::Ball, size: Size::Lg }
                Loading { style: LoadingStyle::Ball, size: Size::Xl }
            )),
        ),
        (
            "Loading bars",
            Some(with_source!(
                Loading { style: LoadingStyle::Bars, size: Size::Xs }
                Loading { style: LoadingStyle::Bars, size: Size::Md }
                Loading { style: LoadingStyle::Bars, size: Size::Lg }
                Loading { style: LoadingStyle::Bars, size: Size::Xl }
            )),
        ),
        (
            "Loading infinity",
            Some(with_source!(
                Loading { style: LoadingStyle::Infinity, size: Size::Xs }
                Loading { style: LoadingStyle::Infinity, size: Size::Md }
                Loading { style: LoadingStyle::Infinity, size: Size::Lg }
                Loading { style: LoadingStyle::Infinity, size: Size::Xl }
            )),
        ),
        (
            "Loading with colors",
            Some(with_source!(
                Loading { color: Color::Primary }
                Loading { color: Color::Secondary }
                Loading { color: Color::Accent }
                Loading { color: Color::Neutral }
                Loading { color: Color::Info }
                Loading { color: Color::Success }
                Loading { color: Color::Warning }
                Loading { color: Color::Error }
            )),
        ),
    ];

    rsx! {
        ComponentVariantList { title: "Loading", variants }
    }
}
