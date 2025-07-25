// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Exception thrown when decryption of a secret fails.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DecryptionFailure {
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::string::String,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl DecryptionFailure {
    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}
impl ::std::fmt::Display for DecryptionFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "DecryptionFailure")?;
        {
            ::std::write!(f, ": {}", &self.message)?;
        }
        Ok(())
    }
}
impl ::std::error::Error for DecryptionFailure {}
impl ::aws_types::request_id::RequestId for crate::types::error::DecryptionFailure {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for DecryptionFailure {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl DecryptionFailure {
    /// Creates a new builder-style object to manufacture [`DecryptionFailure`](crate::types::error::DecryptionFailure).
    pub fn builder() -> crate::types::error::builders::DecryptionFailureBuilder {
        crate::types::error::builders::DecryptionFailureBuilder::default()
    }
}

/// A builder for [`DecryptionFailure`](crate::types::error::DecryptionFailure).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DecryptionFailureBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl DecryptionFailureBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`DecryptionFailure`](crate::types::error::DecryptionFailure).
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](crate::types::error::builders::DecryptionFailureBuilder::message)
    pub fn build(self) -> ::std::result::Result<crate::types::error::DecryptionFailure, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::error::DecryptionFailure {
            message: self.message.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "message",
                    "message was not specified but it is required when building DecryptionFailure",
                )
            })?,
            meta: self.meta.unwrap_or_default(),
        })
    }
}
