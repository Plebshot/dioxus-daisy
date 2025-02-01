use dioxus::prelude::*;

use crate::{components::NavBar, views::*};

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    // For now, we have no overview page, so we redirect to the button preview
    #[redirect("/", || Route::ButtonPreview)]
    #[layout(NavBar)]
    #[route("/button")]
    ButtonPreview,
    #[route("/loading")]
    LoadingPreview,
    #[route("/fieldset")]
    FieldsetPreview,
    #[route("/input")]
    InputPreview,
}
