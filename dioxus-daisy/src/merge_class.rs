/// A macro to combine multiple optional CSS classes into a single class string.
///
/// This is mostly a convenience macro to avoid writing a lot of `if let Some(class) = ...` code.
/// The macro supports inline formatting for the class string, which is useful for adding prefixes.
/// Any value that implements the `CssClass` trait can be used as a class value.
///
/// # Example
///
/// ```
/// # use dioxus_daisy::merge_class;
/// let class = merge_class!(
///     // Direct value
///     "btn",
///     // Direct with prefix
///     "xl" => "btn-{}",
///     // Optional value
///     None::<&'static str>,
///     // Optional value
///     Some("btn-ghost".to_string()),
///     // Optional value with prefix
///     Some("primary") => "btn-{}",
/// );
/// assert_eq!(class, "btn btn-xl btn-ghost btn-primary");
/// ```
#[macro_export]
macro_rules! merge_class {
    ($($tail:tt)*) => {{
        let mut result = String::new();
        let mut first = true;
        $crate::merge_class_inner!(&mut result, &mut first, $($tail)*);
        result
    }};
}

#[macro_export]
macro_rules! merge_class_inner {
    ($result:expr, $first:expr) => {};
    ($result:expr, $first:expr, $val:expr $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::CssClass;
        if let Some(class) = $val.class() {
            if *$first {
                *$first = false;
            } else {
                $result.push(' ');
            }
            $result.push_str(class);
        }
    }};
    ($result:expr, $first:expr, $val:expr => $fmt:expr $(,)?) => {{
        use std::fmt::Write;
        #[allow(unused_imports)]
        use $crate::CssClass;
        if let Some(class) = $val.class() {
            if *$first {
                *$first = false;
            } else {
                write!($result, " ").unwrap();
            }
            write!($result, $fmt, class).unwrap();
        }
    }};
    ($result:expr, $first:expr, $val:expr, $($tail:tt)*) => {
        $crate::merge_class_inner!($result, $first, $val);
        $crate::merge_class_inner!($result, $first, $($tail)*);
    };
    ($result:expr, $first:expr, $val:expr => $fmt:expr, $($tail:tt)*) => {
        $crate::merge_class_inner!($result, $first, $val => $fmt);
        $crate::merge_class_inner!($result, $first, $($tail)*);
    };
}

#[cfg(test)]
mod tests {
    use crate::CssClass;

    #[test]
    fn test_merge_class_var() {
        pub enum Color {
            Primary,
            Secondary,
        }

        impl CssClass for Color {
            fn class(&self) -> Option<&'static str> {
                Some(match self {
                    Color::Primary => "primary",
                    Color::Secondary => "secondary",
                })
            }
        }

        let class = merge_class!(
            // Direct value
            "btn",
            // Direct with prefix
            Color::Secondary => "btn-{}",
            // Optional value
            None::<Color>,
            // Optional value
            Some("btn-ghost"),
            // Optional value with prefix
            Some(Color::Primary) => "btn-{}",
        );
        assert_eq!(class, "btn btn-secondary btn-ghost btn-primary");
    }

    #[test]
    fn test_merge_class_str() {
        let class = merge_class!(
            // Direct value
            "btn",
            // Direct with prefix
            "xl" => "btn-{}",
            // Optional value
            None::<&'static str>,
            // Optional value
            Some("btn-ghost".to_string()),
            // Optional value with prefix
            Some("primary") => "btn-{}",
        );
        assert_eq!(class, "btn btn-xl btn-ghost btn-primary");
    }
}
