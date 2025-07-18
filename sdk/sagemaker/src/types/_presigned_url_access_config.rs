// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configuration for accessing hub content through presigned URLs, including license agreement acceptance and URL validation settings.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PresignedUrlAccessConfig {
    /// <p>Indicates acceptance of the End User License Agreement (EULA) for gated models. Set to true to acknowledge acceptance of the license terms required for accessing gated content.</p>
    pub accept_eula: ::std::option::Option<bool>,
    /// <p>The expected S3 URL prefix for validation purposes. This parameter helps ensure consistency between the resolved S3 URIs and the deployment configuration, reducing potential compatibility issues.</p>
    pub expected_s3_url: ::std::option::Option<::std::string::String>,
}
impl PresignedUrlAccessConfig {
    /// <p>Indicates acceptance of the End User License Agreement (EULA) for gated models. Set to true to acknowledge acceptance of the license terms required for accessing gated content.</p>
    pub fn accept_eula(&self) -> ::std::option::Option<bool> {
        self.accept_eula
    }
    /// <p>The expected S3 URL prefix for validation purposes. This parameter helps ensure consistency between the resolved S3 URIs and the deployment configuration, reducing potential compatibility issues.</p>
    pub fn expected_s3_url(&self) -> ::std::option::Option<&str> {
        self.expected_s3_url.as_deref()
    }
}
impl PresignedUrlAccessConfig {
    /// Creates a new builder-style object to manufacture [`PresignedUrlAccessConfig`](crate::types::PresignedUrlAccessConfig).
    pub fn builder() -> crate::types::builders::PresignedUrlAccessConfigBuilder {
        crate::types::builders::PresignedUrlAccessConfigBuilder::default()
    }
}

/// A builder for [`PresignedUrlAccessConfig`](crate::types::PresignedUrlAccessConfig).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct PresignedUrlAccessConfigBuilder {
    pub(crate) accept_eula: ::std::option::Option<bool>,
    pub(crate) expected_s3_url: ::std::option::Option<::std::string::String>,
}
impl PresignedUrlAccessConfigBuilder {
    /// <p>Indicates acceptance of the End User License Agreement (EULA) for gated models. Set to true to acknowledge acceptance of the license terms required for accessing gated content.</p>
    pub fn accept_eula(mut self, input: bool) -> Self {
        self.accept_eula = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates acceptance of the End User License Agreement (EULA) for gated models. Set to true to acknowledge acceptance of the license terms required for accessing gated content.</p>
    pub fn set_accept_eula(mut self, input: ::std::option::Option<bool>) -> Self {
        self.accept_eula = input;
        self
    }
    /// <p>Indicates acceptance of the End User License Agreement (EULA) for gated models. Set to true to acknowledge acceptance of the license terms required for accessing gated content.</p>
    pub fn get_accept_eula(&self) -> &::std::option::Option<bool> {
        &self.accept_eula
    }
    /// <p>The expected S3 URL prefix for validation purposes. This parameter helps ensure consistency between the resolved S3 URIs and the deployment configuration, reducing potential compatibility issues.</p>
    pub fn expected_s3_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expected_s3_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The expected S3 URL prefix for validation purposes. This parameter helps ensure consistency between the resolved S3 URIs and the deployment configuration, reducing potential compatibility issues.</p>
    pub fn set_expected_s3_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expected_s3_url = input;
        self
    }
    /// <p>The expected S3 URL prefix for validation purposes. This parameter helps ensure consistency between the resolved S3 URIs and the deployment configuration, reducing potential compatibility issues.</p>
    pub fn get_expected_s3_url(&self) -> &::std::option::Option<::std::string::String> {
        &self.expected_s3_url
    }
    /// Consumes the builder and constructs a [`PresignedUrlAccessConfig`](crate::types::PresignedUrlAccessConfig).
    pub fn build(self) -> crate::types::PresignedUrlAccessConfig {
        crate::types::PresignedUrlAccessConfig {
            accept_eula: self.accept_eula,
            expected_s3_url: self.expected_s3_url,
        }
    }
}
