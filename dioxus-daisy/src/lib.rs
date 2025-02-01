pub mod attributes;
pub mod components;
pub mod merge_class;

/// A trait for types that can be converted to a CSS class.
///
/// The returned value is always an option, to make working with optional values easier.
///
/// For the derive version of this trait, see the `css_class` macro.
pub trait CssClass {
    fn class(&self) -> Option<&str>;
}

impl CssClass for &str {
    fn class(&self) -> Option<&str> {
        Some(*self)
    }
}

impl CssClass for String {
    fn class(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl<T> CssClass for Option<T>
where
    T: CssClass,
{
    fn class(&self) -> Option<&str> {
        self.as_ref().and_then(|v| v.class())
    }
}

/// A macro that implements the `CssClass` trait for an enum.
pub use dioxus_daisy_macros::css_class;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CssClass;

    #[test]
    fn test_derive_basic() {
        #[css_class]
        enum Test {
            A,
        }
        assert_eq!(Test::A.class(), Some("a"));
        assert_eq!(Test::A.to_string(), "a");
        assert_eq!(Test::A.as_ref(), "a");
    }

    #[test]
    fn test_derive_default() {
        #[css_class]
        enum Test {
            A,
            #[default]
            B,
        }
        assert_eq!(Test::A.class(), Some("a"));
        assert_eq!(Test::B.class(), Some("b"));
    }

    #[test]
    fn test_derive_prefix() {
        #[css_class(prefix = "test-")]
        enum Test {
            A,
        }
        let expected = "test-a";
        assert_eq!(Test::A.class(), Some(expected));
        assert_eq!(Test::A.to_string(), expected);
        assert_eq!(Test::A.as_ref(), expected);
    }

    #[test]
    fn test_derive_suffix() {
        #[css_class(suffix = "-test")]
        enum Test {
            A,
        }
        let expected = "a-test";
        assert_eq!(Test::A.class(), Some(expected));
        assert_eq!(Test::A.to_string(), expected);
        assert_eq!(Test::A.as_ref(), expected);
    }

    #[test]
    fn test_derive_prefix_suffix() {
        #[css_class(prefix = "test-", suffix = "-test")]
        enum Test {
            A,
        }
        let expected = "test-a-test";
        assert_eq!(Test::A.class(), Some(expected));
        assert_eq!(Test::A.to_string(), expected);
        assert_eq!(Test::A.as_ref(), expected);
    }
}
