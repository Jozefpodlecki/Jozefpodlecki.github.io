use std::{error::Error, fmt};
use thirtyfour::error::WebDriverError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssertionKind {
    Condition,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssertionFailure {
    pub kind: AssertionKind,
    pub expression: &'static str,
    pub actual: String,
    pub expected: Option<String>,
    pub context: Option<String>,
}

impl AssertionFailure {
    pub fn condition(expression: &'static str, context: Option<String>) -> Self {
        Self {
            kind: AssertionKind::Condition,
            expression,
            actual: "false".to_owned(),
            expected: Some("true".to_owned()),
            context,
        }
    }

    pub fn comparison(
        kind: AssertionKind,
        expression: &'static str,
        actual: String,
        expected: String,
        context: Option<String>,
    ) -> Self {
        Self {
            kind,
            expression,
            actual,
            expected: Some(expected),
            context,
        }
    }
}

impl fmt::Display for AssertionFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let comparison = match self.kind {
            AssertionKind::Condition => "condition was false",
            AssertionKind::Equal => "values differed",
            AssertionKind::NotEqual => "values were equal",
        };
        write!(formatter, "E2E assertion failed: {} ({comparison}), actual: {}", self.expression, self.actual)?;
        if let Some(expected) = &self.expected {
            write!(formatter, ", expected: {expected}")?;
        }
        if let Some(context) = &self.context {
            write!(formatter, ", context: {context}")?;
        }
        Ok(())
    }
}

impl Error for AssertionFailure {}

impl From<AssertionFailure> for WebDriverError {
    fn from(failure: AssertionFailure) -> Self {
        Self::RequestFailed(failure.to_string())
    }
}

#[macro_export]
macro_rules! ensure {
    ($condition:expr $(, $($context:tt)+)?) => {
        if !($condition) {
            return Err($crate::assertions::AssertionFailure::condition(
                stringify!($condition),
                $crate::assertion_context!($($($context)+)?),
            ).into());
        }
    };
}

#[macro_export]
macro_rules! ensure_eq {
    ($actual:expr, $expected:expr $(, $($context:tt)+)?) => {
        {
            match (&($actual), &($expected)) {
                (actual, expected) if actual != expected => {
                    return Err($crate::assertions::AssertionFailure::comparison(
                        $crate::assertions::AssertionKind::Equal,
                        concat!(stringify!($actual), " == ", stringify!($expected)),
                        format!("{actual:?}"),
                        format!("{expected:?}"),
                        $crate::assertion_context!($($($context)+)?),
                    ).into());
                }
                _ => {}
            }
        }
    };
}

#[macro_export]
macro_rules! ensure_ne {
    ($actual:expr, $expected:expr $(, $($context:tt)+)?) => {
        {
            match (&($actual), &($expected)) {
                (actual, expected) if actual == expected => {
                    return Err($crate::assertions::AssertionFailure::comparison(
                        $crate::assertions::AssertionKind::NotEqual,
                        concat!(stringify!($actual), " != ", stringify!($expected)),
                        format!("{actual:?}"),
                        format!("{expected:?}"),
                        $crate::assertion_context!($($($context)+)?),
                    ).into());
                }
                _ => {}
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! assertion_context {
    () => { None };
    ($($context:tt)+) => { Some(format!($($context)+)) };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assertion_failure_preserves_comparison_and_context() {
        let failure = AssertionFailure::comparison(
            AssertionKind::Equal,
            "actual == expected",
            "1".to_owned(),
            "2".to_owned(),
            Some("on About page".to_owned()),
        );
        let message = failure.to_string();
        assert!(message.contains("actual == expected"));
        assert!(message.contains("on About page"));
        assert!(message.contains("expected: 2"));
    }

    #[test]
    fn comparison_macros_return_errors_without_panicking() {
        fn check() -> thirtyfour::error::WebDriverResult<()> {
            crate::ensure_eq!(1, 2, "values should match");
            Ok(())
        }
        let error = check().expect_err("mismatched values should return an error");
        assert!(error.to_string().contains("values should match"));
    }

    #[test]
    fn comparison_macros_accept_indexed_non_copy_values() {
        fn check() -> thirtyfour::error::WebDriverResult<()> {
            let state = serde_json::json!({
                "actual": { "colour": "teal" },
                "expected": { "colour": "teal" },
                "different": { "colour": "blue" }
            });
            crate::ensure_eq!(state["actual"], state["expected"]);
            crate::ensure_ne!(state["actual"], state["different"]);
            Ok(())
        }
        check().expect("comparison should borrow indexed values without moving them");
    }

    #[test]
    fn comparison_macros_accept_borrowed_temporaries() {
        fn check() -> thirtyfour::error::WebDriverResult<()> {
            crate::ensure_eq!(Some("page".to_owned()).as_deref(), Some("page"));
            crate::ensure_ne!(Some("page".to_owned()).as_deref(), Some("other"));
            Ok(())
        }
        check().expect("temporary values should live through the comparison");
    }

    #[test]
    fn condition_macro_evaluates_combined_expression() {
        fn check() -> thirtyfour::error::WebDriverResult<()> {
            crate::ensure!(true && false, "combined expression failed");
            Ok(())
        }
        let error = check().expect_err("false condition should return an error");
        assert!(error.to_string().contains("combined expression failed"));
    }

    #[test]
    fn browser_assertion_error_conversion_retains_details() {
        let failure = AssertionFailure::condition("element.is_displayed()", None);
        let error: WebDriverError = failure.into();
        assert!(error.to_string().contains("element.is_displayed()"));
    }
}
