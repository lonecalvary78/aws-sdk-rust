// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ApplyGuardrailInput {
    /// <p>The guardrail identifier used in the request to apply the guardrail.</p>
    pub guardrail_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The guardrail version used in the request to apply the guardrail.</p>
    pub guardrail_version: ::std::option::Option<::std::string::String>,
    /// <p>The source of data used in the request to apply the guardrail.</p>
    pub source: ::std::option::Option<crate::types::GuardrailContentSource>,
    /// <p>The content details used in the request to apply the guardrail.</p>
    pub content: ::std::option::Option<::std::vec::Vec<crate::types::GuardrailContentBlock>>,
}
impl ApplyGuardrailInput {
    /// <p>The guardrail identifier used in the request to apply the guardrail.</p>
    pub fn guardrail_identifier(&self) -> ::std::option::Option<&str> {
        self.guardrail_identifier.as_deref()
    }
    /// <p>The guardrail version used in the request to apply the guardrail.</p>
    pub fn guardrail_version(&self) -> ::std::option::Option<&str> {
        self.guardrail_version.as_deref()
    }
    /// <p>The source of data used in the request to apply the guardrail.</p>
    pub fn source(&self) -> ::std::option::Option<&crate::types::GuardrailContentSource> {
        self.source.as_ref()
    }
    /// <p>The content details used in the request to apply the guardrail.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.content.is_none()`.
    pub fn content(&self) -> &[crate::types::GuardrailContentBlock] {
        self.content.as_deref().unwrap_or_default()
    }
}
impl ApplyGuardrailInput {
    /// Creates a new builder-style object to manufacture [`ApplyGuardrailInput`](crate::operation::apply_guardrail::ApplyGuardrailInput).
    pub fn builder() -> crate::operation::apply_guardrail::builders::ApplyGuardrailInputBuilder {
        crate::operation::apply_guardrail::builders::ApplyGuardrailInputBuilder::default()
    }
}

/// A builder for [`ApplyGuardrailInput`](crate::operation::apply_guardrail::ApplyGuardrailInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ApplyGuardrailInputBuilder {
    pub(crate) guardrail_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) guardrail_version: ::std::option::Option<::std::string::String>,
    pub(crate) source: ::std::option::Option<crate::types::GuardrailContentSource>,
    pub(crate) content: ::std::option::Option<::std::vec::Vec<crate::types::GuardrailContentBlock>>,
}
impl ApplyGuardrailInputBuilder {
    /// <p>The guardrail identifier used in the request to apply the guardrail.</p>
    /// This field is required.
    pub fn guardrail_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.guardrail_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The guardrail identifier used in the request to apply the guardrail.</p>
    pub fn set_guardrail_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.guardrail_identifier = input;
        self
    }
    /// <p>The guardrail identifier used in the request to apply the guardrail.</p>
    pub fn get_guardrail_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.guardrail_identifier
    }
    /// <p>The guardrail version used in the request to apply the guardrail.</p>
    /// This field is required.
    pub fn guardrail_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.guardrail_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The guardrail version used in the request to apply the guardrail.</p>
    pub fn set_guardrail_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.guardrail_version = input;
        self
    }
    /// <p>The guardrail version used in the request to apply the guardrail.</p>
    pub fn get_guardrail_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.guardrail_version
    }
    /// <p>The source of data used in the request to apply the guardrail.</p>
    /// This field is required.
    pub fn source(mut self, input: crate::types::GuardrailContentSource) -> Self {
        self.source = ::std::option::Option::Some(input);
        self
    }
    /// <p>The source of data used in the request to apply the guardrail.</p>
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::GuardrailContentSource>) -> Self {
        self.source = input;
        self
    }
    /// <p>The source of data used in the request to apply the guardrail.</p>
    pub fn get_source(&self) -> &::std::option::Option<crate::types::GuardrailContentSource> {
        &self.source
    }
    /// Appends an item to `content`.
    ///
    /// To override the contents of this collection use [`set_content`](Self::set_content).
    ///
    /// <p>The content details used in the request to apply the guardrail.</p>
    pub fn content(mut self, input: crate::types::GuardrailContentBlock) -> Self {
        let mut v = self.content.unwrap_or_default();
        v.push(input);
        self.content = ::std::option::Option::Some(v);
        self
    }
    /// <p>The content details used in the request to apply the guardrail.</p>
    pub fn set_content(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::GuardrailContentBlock>>) -> Self {
        self.content = input;
        self
    }
    /// <p>The content details used in the request to apply the guardrail.</p>
    pub fn get_content(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::GuardrailContentBlock>> {
        &self.content
    }
    /// Consumes the builder and constructs a [`ApplyGuardrailInput`](crate::operation::apply_guardrail::ApplyGuardrailInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::apply_guardrail::ApplyGuardrailInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::apply_guardrail::ApplyGuardrailInput {
            guardrail_identifier: self.guardrail_identifier,
            guardrail_version: self.guardrail_version,
            source: self.source,
            content: self.content,
        })
    }
}
