//! Error
//!
//!
//! ## Common ErrorKinds
//! // //
//! // // `custom` errors
//! // #[from(ignore)]
//! // #[display("Error extracting lines from input: {}", source_input)]
//! // InputNoLines { source_input: String },
//! // #[from(ignore)]
//! // #[display("error parsing char: {}", uninterpretable_char)]
//! // ParseChar { uninterpretable_char: char },
//! // #[from(ignore)]
//! // #[display("parse error: {}", source)]
//! // ParseInt { source: num::ParseIntError },
//! // #[display("Unparsable character: {}", source_char)]
//! // ParseOther { source_char: char },
//! // //
//! // // `packed` errors
//! // #[display("CLI parsing library error: {}", source)]
//! // Clap { source: clap::Error },
//! // #[display("Error with tracing_subscriber::EnvFilter parsing env directive: {}", source)]
//! // EnvError { source: tracing_subscriber::filter::FromEnvError },
//! // #[display("eframe (egui) error: {}", source)]
//! // EFrame { source: eframe::Error },
//! // #[display("io error: {}", source)]
//! // Io { source: io::Error },
//! // #[display("Error setting tracing subscriber default: {}", source)]
//! // TracingSubscriber { source: SetGlobalDefaultError },
//! // //
//! // // `other` errors
//! // #[from(ignore)] // use `make_dyn_error` instead; would conflict with auto-derives
//! // #[display("Uncategorized Error (dyn error object): {}", source)]
//! // OtherDynError { source: Box<dyn std::error::Error + Send + Sync> },
//! // #[display(r#"Uncategorized string err: "{}""#, source_string)]
//! // OtherStringError { source_string: String },
//!
//! ## Utility reference
//! For adding backtrace to errors:
//! `#![feature(error_generic_member_access)]`
//! `use std::backtrace;`

use std::io;

use derive_more::{Display, Error, From};
use tracing::{instrument, subscriber::SetGlobalDefaultError};
use tracing_error::SpanTrace;

// use derive_more::{Display, Error, derive::From};
#[derive(Debug, Display, From, Error)]
pub enum ErrKind {
        //
        // `custom` errors
        #[from(ignore)]
        #[display("Unparsable character: {}", source_char)]
        ParseOther { source_char: char },
        #[from(ignore)]
        #[display("Error extracting lines from input: {}", source_input)]
        NoInputLines { source_input: String },
        //
        // `packed` errors
        #[display("Error with tracing_subscriber::EnvFilter parsing env directive: {}", source)]
        EnvError { source: tracing_subscriber::filter::FromEnvError },
        #[display("eframe (egui) error: {}", source)]
        EFrame { source: eframe::Error },
        #[display("io error: {}", source)]
        Io { source: io::Error },
        #[display("Error setting tracing subscriber default: {}", source)]
        TracingSubscriber { source: SetGlobalDefaultError },
        //
        // `other` errors
        #[from(ignore)] // use `make_dyn_error` instead; would conflict with auto-derives
        #[display("Uncategorized Error (dyn error object): {}", source)]
        OtherDynError { source: Box<dyn std::error::Error + Send + Sync> },
        #[display(r#"Uncategorized string err: "{}""#, source_string)]
        OtherStringError { source_string: String },
}
impl ErrKind {
        #[instrument(skip_all)]
        pub fn make_dyn_error<E>(error: E) -> Self
        where
                E: Into<Box<dyn std::error::Error + Send + Sync>>,
        {
                Self::OtherDynError { source: error.into() }
        }
}

#[derive(Display, Error, From)]
#[display(
        "error: {:#}\n\n\nspantrace capture: {:?}\n\n\nspantrace: {:#}",
        source,
        spantrace.status(),
        spantrace,
)]
pub struct ErrorWrapper {
        pub source:    ErrKind,
        pub spantrace: tracing_error::SpanTrace,
        // backtrace: backtrace::Backtrace,
}
// Using custom display as debug so we can get SpanTrace auto printed.
impl std::fmt::Debug for ErrorWrapper {
        #[instrument(skip_all)]
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
        }
}
impl<T> From<T> for ErrorWrapper
where
        T: Into<ErrKind>,
{
        #[instrument(skip_all)]
        fn from(error: T) -> Self {
                Self {
                        source:    error.into(),
                        spantrace: SpanTrace::capture(),
                        // backtrace: backtrace::Backtrace::capture(),
                }
        }
}

pub trait ToOther {
        fn to_other(self) -> ErrorWrapper;
}
impl<E> ToOther for E
where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
        #[instrument(skip_all)]
        fn to_other(self) -> ErrorWrapper {
                ErrKind::OtherDynError { source: self.into() }.into()
        }
}
