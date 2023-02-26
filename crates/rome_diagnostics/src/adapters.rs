//! This modules exposes a number of "adapter diagnostics" that wrap error types
//! such as [std::error::Error] or [std::io::Error] in newtypes implementing the
//! [Diagnostic] trait

use std::{io, sync::Arc};

use rome_console::{fmt, markup};

use crate::{category, Category, Diagnostic, DiagnosticTags};

/// Implements [Diagnostic] over types implementing [std::error::Error].
#[derive(Clone, Debug)]
pub struct StdError {
    error: Arc<dyn std::error::Error + Send + Sync>,
}

impl<E: std::error::Error + Send + Sync + 'static> From<E> for StdError {
    fn from(error: E) -> Self {
        Self {
            error: Arc::new(error),
        }
    }
}

impl Diagnostic for StdError {
    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.error)
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        fmt.write_markup(markup!({ AsConsoleDisplay(&self.error) }))
    }

    fn to_owned_diagnostic<'a>(&self) -> Box<dyn Diagnostic + Send + Sync + 'a> {
        Box::new(self.clone())
    }
}

struct AsConsoleDisplay<'a, T>(&'a T);

impl<T: std::fmt::Display> fmt::Display for AsConsoleDisplay<'_, T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        fmt.write_fmt(format_args!("{}", self.0))
    }
}

/// Implements [Diagnostic] over for [io::Error].
#[derive(Clone, Debug)]
pub struct IoError {
    error: Arc<io::Error>,
}

impl From<io::Error> for IoError {
    fn from(error: io::Error) -> Self {
        Self {
            error: Arc::new(error),
        }
    }
}

impl Diagnostic for IoError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/io"))
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.error)
    }

    fn tags(&self) -> DiagnosticTags {
        DiagnosticTags::INTERNAL
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let error = self.error.to_string();
        fmt.write_str(&error)
    }

    fn to_owned_diagnostic<'a>(&self) -> Box<dyn Diagnostic + Send + Sync + 'a> {
        Box::new(self.clone())
    }
}

/// Implements [Diagnostic] over for [pico_args::Error].
#[derive(Clone, Debug)]
pub struct PicoArgsError {
    error: pico_args::Error,
}

impl From<pico_args::Error> for PicoArgsError {
    fn from(error: pico_args::Error) -> Self {
        Self { error }
    }
}

impl Diagnostic for PicoArgsError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("flags/invalid"))
    }

    fn tags(&self) -> DiagnosticTags {
        DiagnosticTags::FIXABLE
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.error)
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let error = self.error.to_string();
        fmt.write_str(&error)
    }

    fn to_owned_diagnostic<'a>(&self) -> Box<dyn Diagnostic + Send + Sync + 'a> {
        Box::new(self.clone())
    }
}
