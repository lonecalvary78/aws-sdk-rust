// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The current status and value of a card in an active Amazon Q App session.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CardStatus {
    /// <p>The current state of the card.</p>
    pub current_state: crate::types::ExecutionStatus,
    /// <p>The current value or result associated with the card.</p>
    pub current_value: ::std::string::String,
}
impl CardStatus {
    /// <p>The current state of the card.</p>
    pub fn current_state(&self) -> &crate::types::ExecutionStatus {
        &self.current_state
    }
    /// <p>The current value or result associated with the card.</p>
    pub fn current_value(&self) -> &str {
        use std::ops::Deref;
        self.current_value.deref()
    }
}
impl CardStatus {
    /// Creates a new builder-style object to manufacture [`CardStatus`](crate::types::CardStatus).
    pub fn builder() -> crate::types::builders::CardStatusBuilder {
        crate::types::builders::CardStatusBuilder::default()
    }
}

/// A builder for [`CardStatus`](crate::types::CardStatus).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CardStatusBuilder {
    pub(crate) current_state: ::std::option::Option<crate::types::ExecutionStatus>,
    pub(crate) current_value: ::std::option::Option<::std::string::String>,
}
impl CardStatusBuilder {
    /// <p>The current state of the card.</p>
    /// This field is required.
    pub fn current_state(mut self, input: crate::types::ExecutionStatus) -> Self {
        self.current_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current state of the card.</p>
    pub fn set_current_state(mut self, input: ::std::option::Option<crate::types::ExecutionStatus>) -> Self {
        self.current_state = input;
        self
    }
    /// <p>The current state of the card.</p>
    pub fn get_current_state(&self) -> &::std::option::Option<crate::types::ExecutionStatus> {
        &self.current_state
    }
    /// <p>The current value or result associated with the card.</p>
    /// This field is required.
    pub fn current_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.current_value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The current value or result associated with the card.</p>
    pub fn set_current_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.current_value = input;
        self
    }
    /// <p>The current value or result associated with the card.</p>
    pub fn get_current_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.current_value
    }
    /// Consumes the builder and constructs a [`CardStatus`](crate::types::CardStatus).
    /// This method will fail if any of the following fields are not set:
    /// - [`current_state`](crate::types::builders::CardStatusBuilder::current_state)
    /// - [`current_value`](crate::types::builders::CardStatusBuilder::current_value)
    pub fn build(self) -> ::std::result::Result<crate::types::CardStatus, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::CardStatus {
            current_state: self.current_state.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "current_state",
                    "current_state was not specified but it is required when building CardStatus",
                )
            })?,
            current_value: self.current_value.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "current_value",
                    "current_value was not specified but it is required when building CardStatus",
                )
            })?,
        })
    }
}
