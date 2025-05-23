// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_update_standards_control_associations::_batch_update_standards_control_associations_output::BatchUpdateStandardsControlAssociationsOutputBuilder;

pub use crate::operation::batch_update_standards_control_associations::_batch_update_standards_control_associations_input::BatchUpdateStandardsControlAssociationsInputBuilder;

impl crate::operation::batch_update_standards_control_associations::builders::BatchUpdateStandardsControlAssociationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_update_standards_control_associations::BatchUpdateStandardsControlAssociationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_update_standards_control_associations::BatchUpdateStandardsControlAssociationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_update_standards_control_associations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchUpdateStandardsControlAssociations`.
///
/// <p>For a batch of security controls and standards, this operation updates the enablement status of a control in a standard.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchUpdateStandardsControlAssociationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_update_standards_control_associations::builders::BatchUpdateStandardsControlAssociationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::batch_update_standards_control_associations::BatchUpdateStandardsControlAssociationsOutput,
        crate::operation::batch_update_standards_control_associations::BatchUpdateStandardsControlAssociationsError,
    > for BatchUpdateStandardsControlAssociationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::batch_update_standards_control_associations::BatchUpdateStandardsControlAssociationsOutput,
            crate::operation::batch_update_standards_control_associations::BatchUpdateStandardsControlAssociationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BatchUpdateStandardsControlAssociationsFluentBuilder {
    /// Creates a new `BatchUpdateStandardsControlAssociationsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchUpdateStandardsControlAssociations as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::batch_update_standards_control_associations::builders::BatchUpdateStandardsControlAssociationsInputBuilder {
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
        crate::operation::batch_update_standards_control_associations::BatchUpdateStandardsControlAssociationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_update_standards_control_associations::BatchUpdateStandardsControlAssociationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::batch_update_standards_control_associations::BatchUpdateStandardsControlAssociations::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::batch_update_standards_control_associations::BatchUpdateStandardsControlAssociations::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::batch_update_standards_control_associations::BatchUpdateStandardsControlAssociationsOutput,
        crate::operation::batch_update_standards_control_associations::BatchUpdateStandardsControlAssociationsError,
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
    ///
    /// Appends an item to `StandardsControlAssociationUpdates`.
    ///
    /// To override the contents of this collection use [`set_standards_control_association_updates`](Self::set_standards_control_association_updates).
    ///
    /// <p>Updates the enablement status of a security control in a specified standard.</p>
    /// <p>Calls to this operation return a <code>RESOURCE_NOT_FOUND_EXCEPTION</code> error when the standard subscription for the control has <code>StandardsControlsUpdatable</code> value <code>NOT_READY_FOR_UPDATES</code>.</p>
    pub fn standards_control_association_updates(mut self, input: crate::types::StandardsControlAssociationUpdate) -> Self {
        self.inner = self.inner.standards_control_association_updates(input);
        self
    }
    /// <p>Updates the enablement status of a security control in a specified standard.</p>
    /// <p>Calls to this operation return a <code>RESOURCE_NOT_FOUND_EXCEPTION</code> error when the standard subscription for the control has <code>StandardsControlsUpdatable</code> value <code>NOT_READY_FOR_UPDATES</code>.</p>
    pub fn set_standards_control_association_updates(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::StandardsControlAssociationUpdate>>,
    ) -> Self {
        self.inner = self.inner.set_standards_control_association_updates(input);
        self
    }
    /// <p>Updates the enablement status of a security control in a specified standard.</p>
    /// <p>Calls to this operation return a <code>RESOURCE_NOT_FOUND_EXCEPTION</code> error when the standard subscription for the control has <code>StandardsControlsUpdatable</code> value <code>NOT_READY_FOR_UPDATES</code>.</p>
    pub fn get_standards_control_association_updates(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::StandardsControlAssociationUpdate>> {
        self.inner.get_standards_control_association_updates()
    }
}
