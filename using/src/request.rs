//! A `Request` for a spec within a document.
//!
//! `Request`s are derived from `Directive`s during schema bootstrapping.
use graphql_parser::{Pos, schema::{Directive, Value}};

use crate::spec::{
    Spec,
    SpecParseError,
};

/// Requests contain a `spec`, the `prefix` requested for that spec (which
/// will be the spec's [default prefix](Spec.html#default_prefix) if none was
/// explicitly specified, and the position of the directive making the request
/// (for validation error reporting).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub spec: Spec,
    pub prefix: String,
    pub position: Pos,
}

impl Request {
    /// Extract a `Request` from a directive.
    ///
    /// This returns an `Option<Result<_, _>>`, which is admittedly odd! The reason
    /// it does so is to represent two classes of extraction failures:
    ///   - If the directive *does not contain* a `spec` argument with a string value,
    ///     this method returns `None`.
    ///   - If the directive *contains* a string `spec` argument, but that argument fails
    ///     to parse as a [`Spec`](Spec.html), this method returns `Some(Err(SpecParseError))`.
    ///   - If the directive contains a string `spec` argument which parses as a [`Spec`](Spec.html),
    ///     this method returns `Some(Ok(Request))`
    ///
    /// This keeps `SpecParseError` from having to represent the "no spec argument at all" case,
    /// which is impossible to reach from [`Spec::parse`](Spec.html#parse). It also simplifies
    /// the bootstrapping code, which can simply use `filter_map` to collect `Result`s. (We track
    /// `Result<Request, SpecParseError>` during bootstrapping to assist error reporting.)
    pub(crate) fn from_directive(dir: &Directive) -> Option<Result<Request, SpecParseError>> {
        let mut spec: Option<Result<Spec, SpecParseError>> = None;
        let mut prefix: Option<String> = None;
        for (arg, val) in &dir.arguments {
            if *arg == "spec" {
                if let Value::String(url) = val {
                    spec = Some(Spec::parse(url));
                }
            }
            if *arg == "prefix" {
                if let Value::String(prefix_str) = val {
                    prefix = Some(prefix_str.to_owned())
                }
            }
        }

        spec.map(|result|
            result.map(|spec|
                Request {
                    prefix: prefix.unwrap_or_else(|| spec.default_prefix.clone()),
                    spec,
                    position: dir.position,
                }
            )
        )
    }
}