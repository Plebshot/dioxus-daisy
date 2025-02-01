use crate::css_class;

#[css_class]
pub enum Size {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[css_class]
pub enum Color {
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Warning,
    Error,
}
