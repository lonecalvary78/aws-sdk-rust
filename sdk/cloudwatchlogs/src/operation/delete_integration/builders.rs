// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_integration::_delete_integration_output::DeleteIntegrationOutputBuilder;

pub use crate::operation::delete_integration::_delete_integration_input::DeleteIntegrationInputBuilder;

impl crate::operation::delete_integration::builders::DeleteIntegrationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_integration::DeleteIntegrationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_integration::DeleteIntegrationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_integration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteIntegration`.
///
/// <p>Deletes the integration between CloudWatch Logs and OpenSearch Service. If your integration has active vended logs dashboards, you must specify <code>true</code> for the <code>force</code> parameter, otherwise the operation will fail. If you delete the integration by setting <code>force</code> to <code>true</code>, all your vended logs dashboards powered by OpenSearch Service will be deleted and the data that was on them will no longer be accessible.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteIntegrationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_integration::builders::DeleteIntegrationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_integration::DeleteIntegrationOutput,
        crate::operation::delete_integration::DeleteIntegrationError,
    > for DeleteIntegrationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_integration::DeleteIntegrationOutput,
            crate::operation::delete_integration::DeleteIntegrationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteIntegrationFluentBuilder {
    /// Creates a new `DeleteIntegrationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteIntegration as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_integration::builders::DeleteIntegrationInputBuilder {
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
        crate::operation::delete_integration::DeleteIntegrationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_integration::DeleteIntegrationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_integration::DeleteIntegration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_integration::DeleteIntegration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_integration::DeleteIntegrationOutput,
        crate::operation::delete_integration::DeleteIntegrationError,
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
    /// <p>The name of the integration to delete. To find the name of your integration, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListIntegrations.html">ListIntegrations</a>.</p>
    pub fn integration_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.integration_name(input.into());
        self
    }
    /// <p>The name of the integration to delete. To find the name of your integration, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListIntegrations.html">ListIntegrations</a>.</p>
    pub fn set_integration_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_integration_name(input);
        self
    }
    /// <p>The name of the integration to delete. To find the name of your integration, use <a href="https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_ListIntegrations.html">ListIntegrations</a>.</p>
    pub fn get_integration_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_integration_name()
    }
    /// <p>Specify <code>true</code> to force the deletion of the integration even if vended logs dashboards currently exist.</p>
    /// <p>The default is <code>false</code>.</p>
    pub fn force(mut self, input: bool) -> Self {
        self.inner = self.inner.force(input);
        self
    }
    /// <p>Specify <code>true</code> to force the deletion of the integration even if vended logs dashboards currently exist.</p>
    /// <p>The default is <code>false</code>.</p>
    pub fn set_force(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force(input);
        self
    }
    /// <p>Specify <code>true</code> to force the deletion of the integration even if vended logs dashboards currently exist.</p>
    /// <p>The default is <code>false</code>.</p>
    pub fn get_force(&self) -> &::std::option::Option<bool> {
        self.inner.get_force()
    }
}
