#[derive(Clone, PartialEq)]
pub enum Size {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[derive(Clone, PartialEq)]
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
