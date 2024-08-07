// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>S3 Location of the training data.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TrainingDataConfig {
    /// <p>The S3 URI where the training data is stored.</p>
    pub s3_uri: ::std::string::String,
}
impl TrainingDataConfig {
    /// <p>The S3 URI where the training data is stored.</p>
    pub fn s3_uri(&self) -> &str {
        use std::ops::Deref;
        self.s3_uri.deref()
    }
}
impl TrainingDataConfig {
    /// Creates a new builder-style object to manufacture [`TrainingDataConfig`](crate::types::TrainingDataConfig).
    pub fn builder() -> crate::types::builders::TrainingDataConfigBuilder {
        crate::types::builders::TrainingDataConfigBuilder::default()
    }
}

/// A builder for [`TrainingDataConfig`](crate::types::TrainingDataConfig).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct TrainingDataConfigBuilder {
    pub(crate) s3_uri: ::std::option::Option<::std::string::String>,
}
impl TrainingDataConfigBuilder {
    /// <p>The S3 URI where the training data is stored.</p>
    /// This field is required.
    pub fn s3_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The S3 URI where the training data is stored.</p>
    pub fn set_s3_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_uri = input;
        self
    }
    /// <p>The S3 URI where the training data is stored.</p>
    pub fn get_s3_uri(&self) -> &::std::option::Option<::std::string::String> {
        &self.s3_uri
    }
    /// Consumes the builder and constructs a [`TrainingDataConfig`](crate::types::TrainingDataConfig).
    /// This method will fail if any of the following fields are not set:
    /// - [`s3_uri`](crate::types::builders::TrainingDataConfigBuilder::s3_uri)
    pub fn build(self) -> ::std::result::Result<crate::types::TrainingDataConfig, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::TrainingDataConfig {
            s3_uri: self.s3_uri.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "s3_uri",
                    "s3_uri was not specified but it is required when building TrainingDataConfig",
                )
            })?,
        })
    }
}
