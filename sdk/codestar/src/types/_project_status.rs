// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An indication of whether a project creation or deletion is failed or successful.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProjectStatus {
    /// <p>The phase of completion for a project creation or deletion.</p>
    pub state: ::std::string::String,
    /// <p>In the case of a project creation or deletion failure, a reason for the failure.</p>
    pub reason: ::std::option::Option<::std::string::String>,
}
impl ProjectStatus {
    /// <p>The phase of completion for a project creation or deletion.</p>
    pub fn state(&self) -> &str {
        use std::ops::Deref;
        self.state.deref()
    }
    /// <p>In the case of a project creation or deletion failure, a reason for the failure.</p>
    pub fn reason(&self) -> ::std::option::Option<&str> {
        self.reason.as_deref()
    }
}
impl ProjectStatus {
    /// Creates a new builder-style object to manufacture [`ProjectStatus`](crate::types::ProjectStatus).
    pub fn builder() -> crate::types::builders::ProjectStatusBuilder {
        crate::types::builders::ProjectStatusBuilder::default()
    }
}

/// A builder for [`ProjectStatus`](crate::types::ProjectStatus).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ProjectStatusBuilder {
    pub(crate) state: ::std::option::Option<::std::string::String>,
    pub(crate) reason: ::std::option::Option<::std::string::String>,
}
impl ProjectStatusBuilder {
    /// <p>The phase of completion for a project creation or deletion.</p>
    /// This field is required.
    pub fn state(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.state = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The phase of completion for a project creation or deletion.</p>
    pub fn set_state(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.state = input;
        self
    }
    /// <p>The phase of completion for a project creation or deletion.</p>
    pub fn get_state(&self) -> &::std::option::Option<::std::string::String> {
        &self.state
    }
    /// <p>In the case of a project creation or deletion failure, a reason for the failure.</p>
    pub fn reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>In the case of a project creation or deletion failure, a reason for the failure.</p>
    pub fn set_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reason = input;
        self
    }
    /// <p>In the case of a project creation or deletion failure, a reason for the failure.</p>
    pub fn get_reason(&self) -> &::std::option::Option<::std::string::String> {
        &self.reason
    }
    /// Consumes the builder and constructs a [`ProjectStatus`](crate::types::ProjectStatus).
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](crate::types::builders::ProjectStatusBuilder::state)
    pub fn build(self) -> ::std::result::Result<crate::types::ProjectStatus, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ProjectStatus {
            state: self.state.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "state",
                    "state was not specified but it is required when building ProjectStatus",
                )
            })?,
            reason: self.reason,
        })
    }
}
