// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input for <code>ValidateTemplate</code> action.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ValidateTemplateInput {
    /// <p>Structure that contains the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes.</p>
    /// <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub template_body: ::std::option::Option<::std::string::String>,
    /// <p>The URL of a file that contains the template body. The URL must point to a template (max size: 1 MB) that is located in an Amazon S3 bucket or a Systems Manager document. The location for an Amazon S3 bucket must start with <code>https://</code>.</p>
    /// <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub template_url: ::std::option::Option<::std::string::String>,
}
impl ValidateTemplateInput {
    /// <p>Structure that contains the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes.</p>
    /// <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub fn template_body(&self) -> ::std::option::Option<&str> {
        self.template_body.as_deref()
    }
    /// <p>The URL of a file that contains the template body. The URL must point to a template (max size: 1 MB) that is located in an Amazon S3 bucket or a Systems Manager document. The location for an Amazon S3 bucket must start with <code>https://</code>.</p>
    /// <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub fn template_url(&self) -> ::std::option::Option<&str> {
        self.template_url.as_deref()
    }
}
impl ValidateTemplateInput {
    /// Creates a new builder-style object to manufacture [`ValidateTemplateInput`](crate::operation::validate_template::ValidateTemplateInput).
    pub fn builder() -> crate::operation::validate_template::builders::ValidateTemplateInputBuilder {
        crate::operation::validate_template::builders::ValidateTemplateInputBuilder::default()
    }
}

/// A builder for [`ValidateTemplateInput`](crate::operation::validate_template::ValidateTemplateInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ValidateTemplateInputBuilder {
    pub(crate) template_body: ::std::option::Option<::std::string::String>,
    pub(crate) template_url: ::std::option::Option<::std::string::String>,
}
impl ValidateTemplateInputBuilder {
    /// <p>Structure that contains the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes.</p>
    /// <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub fn template_body(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.template_body = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Structure that contains the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes.</p>
    /// <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub fn set_template_body(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.template_body = input;
        self
    }
    /// <p>Structure that contains the template body with a minimum length of 1 byte and a maximum length of 51,200 bytes.</p>
    /// <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub fn get_template_body(&self) -> &::std::option::Option<::std::string::String> {
        &self.template_body
    }
    /// <p>The URL of a file that contains the template body. The URL must point to a template (max size: 1 MB) that is located in an Amazon S3 bucket or a Systems Manager document. The location for an Amazon S3 bucket must start with <code>https://</code>.</p>
    /// <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub fn template_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.template_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URL of a file that contains the template body. The URL must point to a template (max size: 1 MB) that is located in an Amazon S3 bucket or a Systems Manager document. The location for an Amazon S3 bucket must start with <code>https://</code>.</p>
    /// <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub fn set_template_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.template_url = input;
        self
    }
    /// <p>The URL of a file that contains the template body. The URL must point to a template (max size: 1 MB) that is located in an Amazon S3 bucket or a Systems Manager document. The location for an Amazon S3 bucket must start with <code>https://</code>.</p>
    /// <p>Conditional: You must pass <code>TemplateURL</code> or <code>TemplateBody</code>. If both are passed, only <code>TemplateBody</code> is used.</p>
    pub fn get_template_url(&self) -> &::std::option::Option<::std::string::String> {
        &self.template_url
    }
    /// Consumes the builder and constructs a [`ValidateTemplateInput`](crate::operation::validate_template::ValidateTemplateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::validate_template::ValidateTemplateInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::validate_template::ValidateTemplateInput {
            template_body: self.template_body,
            template_url: self.template_url,
        })
    }
}
