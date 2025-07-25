// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_threat_intel_set::_create_threat_intel_set_output::CreateThreatIntelSetOutputBuilder;

pub use crate::operation::create_threat_intel_set::_create_threat_intel_set_input::CreateThreatIntelSetInputBuilder;

impl crate::operation::create_threat_intel_set::builders::CreateThreatIntelSetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_threat_intel_set::CreateThreatIntelSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_threat_intel_set::CreateThreatIntelSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_threat_intel_set();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateThreatIntelSet`.
///
/// <p>Creates a new ThreatIntelSet. ThreatIntelSets consist of known malicious IP addresses. GuardDuty generates findings based on ThreatIntelSets. Only users of the administrator account can use this operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateThreatIntelSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_threat_intel_set::builders::CreateThreatIntelSetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_threat_intel_set::CreateThreatIntelSetOutput,
        crate::operation::create_threat_intel_set::CreateThreatIntelSetError,
    > for CreateThreatIntelSetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_threat_intel_set::CreateThreatIntelSetOutput,
            crate::operation::create_threat_intel_set::CreateThreatIntelSetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateThreatIntelSetFluentBuilder {
    /// Creates a new `CreateThreatIntelSetFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateThreatIntelSet as a reference.
    pub fn as_input(&self) -> &crate::operation::create_threat_intel_set::builders::CreateThreatIntelSetInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_threat_intel_set::CreateThreatIntelSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_threat_intel_set::CreateThreatIntelSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_threat_intel_set::CreateThreatIntelSet::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_threat_intel_set::CreateThreatIntelSet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_threat_intel_set::CreateThreatIntelSetOutput,
        crate::operation::create_threat_intel_set::CreateThreatIntelSetError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The unique ID of the detector of the GuardDuty account for which you want to create a <code>ThreatIntelSet</code>.</p>
    /// <p>To find the <code>detectorId</code> in the current Region, see the Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The unique ID of the detector of the GuardDuty account for which you want to create a <code>ThreatIntelSet</code>.</p>
    /// <p>To find the <code>detectorId</code> in the current Region, see the Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>The unique ID of the detector of the GuardDuty account for which you want to create a <code>ThreatIntelSet</code>.</p>
    /// <p>To find the <code>detectorId</code> in the current Region, see the Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p>
    pub fn get_detector_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_detector_id()
    }
    /// <p>A user-friendly ThreatIntelSet name displayed in all findings that are generated by activity that involves IP addresses included in this ThreatIntelSet.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A user-friendly ThreatIntelSet name displayed in all findings that are generated by activity that involves IP addresses included in this ThreatIntelSet.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A user-friendly ThreatIntelSet name displayed in all findings that are generated by activity that involves IP addresses included in this ThreatIntelSet.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The format of the file that contains the ThreatIntelSet.</p>
    pub fn format(mut self, input: crate::types::ThreatIntelSetFormat) -> Self {
        self.inner = self.inner.format(input);
        self
    }
    /// <p>The format of the file that contains the ThreatIntelSet.</p>
    pub fn set_format(mut self, input: ::std::option::Option<crate::types::ThreatIntelSetFormat>) -> Self {
        self.inner = self.inner.set_format(input);
        self
    }
    /// <p>The format of the file that contains the ThreatIntelSet.</p>
    pub fn get_format(&self) -> &::std::option::Option<crate::types::ThreatIntelSetFormat> {
        self.inner.get_format()
    }
    /// <p>The URI of the file that contains the ThreatIntelSet.</p>
    pub fn location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.location(input.into());
        self
    }
    /// <p>The URI of the file that contains the ThreatIntelSet.</p>
    pub fn set_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_location(input);
        self
    }
    /// <p>The URI of the file that contains the ThreatIntelSet.</p>
    pub fn get_location(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_location()
    }
    /// <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded ThreatIntelSet.</p>
    pub fn activate(mut self, input: bool) -> Self {
        self.inner = self.inner.activate(input);
        self
    }
    /// <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded ThreatIntelSet.</p>
    pub fn set_activate(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_activate(input);
        self
    }
    /// <p>A Boolean value that indicates whether GuardDuty is to start using the uploaded ThreatIntelSet.</p>
    pub fn get_activate(&self) -> &::std::option::Option<bool> {
        self.inner.get_activate()
    }
    /// <p>The idempotency token for the create request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The idempotency token for the create request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The idempotency token for the create request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    ///
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to be added to a new threat list resource.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags to be added to a new threat list resource.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to be added to a new threat list resource.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
    /// <p>The Amazon Web Services account ID that owns the Amazon S3 bucket specified in the <b>location</b> parameter.</p>
    pub fn expected_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.expected_bucket_owner(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID that owns the Amazon S3 bucket specified in the <b>location</b> parameter.</p>
    pub fn set_expected_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_expected_bucket_owner(input);
        self
    }
    /// <p>The Amazon Web Services account ID that owns the Amazon S3 bucket specified in the <b>location</b> parameter.</p>
    pub fn get_expected_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_expected_bucket_owner()
    }
}
