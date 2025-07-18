// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::upgrade_account_plan::_upgrade_account_plan_output::UpgradeAccountPlanOutputBuilder;

pub use crate::operation::upgrade_account_plan::_upgrade_account_plan_input::UpgradeAccountPlanInputBuilder;

impl crate::operation::upgrade_account_plan::builders::UpgradeAccountPlanInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::upgrade_account_plan::UpgradeAccountPlanOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::upgrade_account_plan::UpgradeAccountPlanError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.upgrade_account_plan();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpgradeAccountPlan`.
///
/// <p>The account plan type for the Amazon Web Services account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpgradeAccountPlanFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::upgrade_account_plan::builders::UpgradeAccountPlanInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::upgrade_account_plan::UpgradeAccountPlanOutput,
        crate::operation::upgrade_account_plan::UpgradeAccountPlanError,
    > for UpgradeAccountPlanFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::upgrade_account_plan::UpgradeAccountPlanOutput,
            crate::operation::upgrade_account_plan::UpgradeAccountPlanError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpgradeAccountPlanFluentBuilder {
    /// Creates a new `UpgradeAccountPlanFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpgradeAccountPlan as a reference.
    pub fn as_input(&self) -> &crate::operation::upgrade_account_plan::builders::UpgradeAccountPlanInputBuilder {
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
        crate::operation::upgrade_account_plan::UpgradeAccountPlanOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::upgrade_account_plan::UpgradeAccountPlanError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::upgrade_account_plan::UpgradeAccountPlan::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::upgrade_account_plan::UpgradeAccountPlan::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::upgrade_account_plan::UpgradeAccountPlanOutput,
        crate::operation::upgrade_account_plan::UpgradeAccountPlanError,
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
    /// <p>The target account plan type. This makes it explicit about the change and latest value of the <code>accountPlanType</code>.</p>
    pub fn account_plan_type(mut self, input: crate::types::AccountPlanType) -> Self {
        self.inner = self.inner.account_plan_type(input);
        self
    }
    /// <p>The target account plan type. This makes it explicit about the change and latest value of the <code>accountPlanType</code>.</p>
    pub fn set_account_plan_type(mut self, input: ::std::option::Option<crate::types::AccountPlanType>) -> Self {
        self.inner = self.inner.set_account_plan_type(input);
        self
    }
    /// <p>The target account plan type. This makes it explicit about the change and latest value of the <code>accountPlanType</code>.</p>
    pub fn get_account_plan_type(&self) -> &::std::option::Option<crate::types::AccountPlanType> {
        self.inner.get_account_plan_type()
    }
}
