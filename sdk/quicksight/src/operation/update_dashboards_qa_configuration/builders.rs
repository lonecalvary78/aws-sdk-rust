// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_dashboards_qa_configuration::_update_dashboards_qa_configuration_output::UpdateDashboardsQaConfigurationOutputBuilder;

pub use crate::operation::update_dashboards_qa_configuration::_update_dashboards_qa_configuration_input::UpdateDashboardsQaConfigurationInputBuilder;

impl crate::operation::update_dashboards_qa_configuration::builders::UpdateDashboardsQaConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_dashboards_qa_configuration::UpdateDashboardsQaConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_dashboards_qa_configuration::UpdateDashboardsQAConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_dashboards_qa_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateDashboardsQAConfiguration`.
///
/// <p>Updates a Dashboard QA configuration.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDashboardsQAConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_dashboards_qa_configuration::builders::UpdateDashboardsQaConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_dashboards_qa_configuration::UpdateDashboardsQaConfigurationOutput,
        crate::operation::update_dashboards_qa_configuration::UpdateDashboardsQAConfigurationError,
    > for UpdateDashboardsQAConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_dashboards_qa_configuration::UpdateDashboardsQaConfigurationOutput,
            crate::operation::update_dashboards_qa_configuration::UpdateDashboardsQAConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateDashboardsQAConfigurationFluentBuilder {
    /// Creates a new `UpdateDashboardsQAConfigurationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateDashboardsQAConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::update_dashboards_qa_configuration::builders::UpdateDashboardsQaConfigurationInputBuilder {
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
        crate::operation::update_dashboards_qa_configuration::UpdateDashboardsQaConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_dashboards_qa_configuration::UpdateDashboardsQAConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_dashboards_qa_configuration::UpdateDashboardsQAConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_dashboards_qa_configuration::UpdateDashboardsQAConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_dashboards_qa_configuration::UpdateDashboardsQaConfigurationOutput,
        crate::operation::update_dashboards_qa_configuration::UpdateDashboardsQAConfigurationError,
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
    /// <p>The ID of the Amazon Web Services account that contains the dashboard QA configuration that you want to update.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the dashboard QA configuration that you want to update.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the dashboard QA configuration that you want to update.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>The status of dashboards QA configuration that you want to update.</p>
    pub fn dashboards_qa_status(mut self, input: crate::types::DashboardsQaStatus) -> Self {
        self.inner = self.inner.dashboards_qa_status(input);
        self
    }
    /// <p>The status of dashboards QA configuration that you want to update.</p>
    pub fn set_dashboards_qa_status(mut self, input: ::std::option::Option<crate::types::DashboardsQaStatus>) -> Self {
        self.inner = self.inner.set_dashboards_qa_status(input);
        self
    }
    /// <p>The status of dashboards QA configuration that you want to update.</p>
    pub fn get_dashboards_qa_status(&self) -> &::std::option::Option<crate::types::DashboardsQaStatus> {
        self.inner.get_dashboards_qa_status()
    }
}
