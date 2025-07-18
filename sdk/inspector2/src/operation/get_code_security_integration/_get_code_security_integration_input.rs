// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetCodeSecurityIntegrationInput {
    /// <p>The Amazon Resource Name (ARN) of the code security integration to retrieve.</p>
    pub integration_arn: ::std::option::Option<::std::string::String>,
    /// <p>The tags associated with the code security integration.</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl GetCodeSecurityIntegrationInput {
    /// <p>The Amazon Resource Name (ARN) of the code security integration to retrieve.</p>
    pub fn integration_arn(&self) -> ::std::option::Option<&str> {
        self.integration_arn.as_deref()
    }
    /// <p>The tags associated with the code security integration.</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
}
impl GetCodeSecurityIntegrationInput {
    /// Creates a new builder-style object to manufacture [`GetCodeSecurityIntegrationInput`](crate::operation::get_code_security_integration::GetCodeSecurityIntegrationInput).
    pub fn builder() -> crate::operation::get_code_security_integration::builders::GetCodeSecurityIntegrationInputBuilder {
        crate::operation::get_code_security_integration::builders::GetCodeSecurityIntegrationInputBuilder::default()
    }
}

/// A builder for [`GetCodeSecurityIntegrationInput`](crate::operation::get_code_security_integration::GetCodeSecurityIntegrationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetCodeSecurityIntegrationInputBuilder {
    pub(crate) integration_arn: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl GetCodeSecurityIntegrationInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the code security integration to retrieve.</p>
    /// This field is required.
    pub fn integration_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.integration_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the code security integration to retrieve.</p>
    pub fn set_integration_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.integration_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the code security integration to retrieve.</p>
    pub fn get_integration_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.integration_arn
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags associated with the code security integration.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags associated with the code security integration.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags associated with the code security integration.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`GetCodeSecurityIntegrationInput`](crate::operation::get_code_security_integration::GetCodeSecurityIntegrationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_code_security_integration::GetCodeSecurityIntegrationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_code_security_integration::GetCodeSecurityIntegrationInput {
            integration_arn: self.integration_arn,
            tags: self.tags,
        })
    }
}
