// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_data_accessor::_update_data_accessor_output::UpdateDataAccessorOutputBuilder;

pub use crate::operation::update_data_accessor::_update_data_accessor_input::UpdateDataAccessorInputBuilder;

impl crate::operation::update_data_accessor::builders::UpdateDataAccessorInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_data_accessor::UpdateDataAccessorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_data_accessor::UpdateDataAccessorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_data_accessor();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateDataAccessor`.
///
/// <p>Updates an existing data accessor. This operation allows modifying the action configurations (the allowed actions and associated filters) and the display name of the data accessor. It does not allow changing the IAM role associated with the data accessor or other core properties of the data accessor.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDataAccessorFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_data_accessor::builders::UpdateDataAccessorInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_data_accessor::UpdateDataAccessorOutput,
        crate::operation::update_data_accessor::UpdateDataAccessorError,
    > for UpdateDataAccessorFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_data_accessor::UpdateDataAccessorOutput,
            crate::operation::update_data_accessor::UpdateDataAccessorError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateDataAccessorFluentBuilder {
    /// Creates a new `UpdateDataAccessorFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateDataAccessor as a reference.
    pub fn as_input(&self) -> &crate::operation::update_data_accessor::builders::UpdateDataAccessorInputBuilder {
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
        crate::operation::update_data_accessor::UpdateDataAccessorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_data_accessor::UpdateDataAccessorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_data_accessor::UpdateDataAccessor::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_data_accessor::UpdateDataAccessor::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_data_accessor::UpdateDataAccessorOutput,
        crate::operation::update_data_accessor::UpdateDataAccessorError,
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
    /// <p>The unique identifier of the Amazon Q Business application.</p>
    pub fn application_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The unique identifier of the Amazon Q Business application.</p>
    pub fn set_application_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The unique identifier of the Amazon Q Business application.</p>
    pub fn get_application_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_id()
    }
    /// <p>The unique identifier of the data accessor to update.</p>
    pub fn data_accessor_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.data_accessor_id(input.into());
        self
    }
    /// <p>The unique identifier of the data accessor to update.</p>
    pub fn set_data_accessor_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_data_accessor_id(input);
        self
    }
    /// <p>The unique identifier of the data accessor to update.</p>
    pub fn get_data_accessor_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_data_accessor_id()
    }
    ///
    /// Appends an item to `actionConfigurations`.
    ///
    /// To override the contents of this collection use [`set_action_configurations`](Self::set_action_configurations).
    ///
    /// <p>The updated list of action configurations specifying the allowed actions and any associated filters.</p>
    pub fn action_configurations(mut self, input: crate::types::ActionConfiguration) -> Self {
        self.inner = self.inner.action_configurations(input);
        self
    }
    /// <p>The updated list of action configurations specifying the allowed actions and any associated filters.</p>
    pub fn set_action_configurations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ActionConfiguration>>) -> Self {
        self.inner = self.inner.set_action_configurations(input);
        self
    }
    /// <p>The updated list of action configurations specifying the allowed actions and any associated filters.</p>
    pub fn get_action_configurations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ActionConfiguration>> {
        self.inner.get_action_configurations()
    }
    /// <p>The updated authentication configuration details for the data accessor. This specifies how the ISV will authenticate when accessing data through this data accessor.</p>
    pub fn authentication_detail(mut self, input: crate::types::DataAccessorAuthenticationDetail) -> Self {
        self.inner = self.inner.authentication_detail(input);
        self
    }
    /// <p>The updated authentication configuration details for the data accessor. This specifies how the ISV will authenticate when accessing data through this data accessor.</p>
    pub fn set_authentication_detail(mut self, input: ::std::option::Option<crate::types::DataAccessorAuthenticationDetail>) -> Self {
        self.inner = self.inner.set_authentication_detail(input);
        self
    }
    /// <p>The updated authentication configuration details for the data accessor. This specifies how the ISV will authenticate when accessing data through this data accessor.</p>
    pub fn get_authentication_detail(&self) -> &::std::option::Option<crate::types::DataAccessorAuthenticationDetail> {
        self.inner.get_authentication_detail()
    }
    /// <p>The updated friendly name for the data accessor.</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.display_name(input.into());
        self
    }
    /// <p>The updated friendly name for the data accessor.</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_display_name(input);
        self
    }
    /// <p>The updated friendly name for the data accessor.</p>
    pub fn get_display_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_display_name()
    }
}
