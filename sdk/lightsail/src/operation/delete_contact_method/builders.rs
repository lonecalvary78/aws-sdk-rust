// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_contact_method::_delete_contact_method_output::DeleteContactMethodOutputBuilder;

pub use crate::operation::delete_contact_method::_delete_contact_method_input::DeleteContactMethodInputBuilder;

impl crate::operation::delete_contact_method::builders::DeleteContactMethodInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_contact_method::DeleteContactMethodOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_contact_method::DeleteContactMethodError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_contact_method();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteContactMethod`.
///
/// <p>Deletes a contact method.</p>
/// <p>A contact method is used to send you notifications about your Amazon Lightsail resources. You can add one email address and one mobile phone number contact method in each Amazon Web Services Region. However, SMS text messaging is not supported in some Amazon Web Services Regions, and SMS text messages cannot be sent to some countries/regions. For more information, see <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-notifications">Notifications in Amazon Lightsail</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteContactMethodFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_contact_method::builders::DeleteContactMethodInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_contact_method::DeleteContactMethodOutput,
        crate::operation::delete_contact_method::DeleteContactMethodError,
    > for DeleteContactMethodFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_contact_method::DeleteContactMethodOutput,
            crate::operation::delete_contact_method::DeleteContactMethodError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteContactMethodFluentBuilder {
    /// Creates a new `DeleteContactMethodFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteContactMethod as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_contact_method::builders::DeleteContactMethodInputBuilder {
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
        crate::operation::delete_contact_method::DeleteContactMethodOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_contact_method::DeleteContactMethodError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_contact_method::DeleteContactMethod::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_contact_method::DeleteContactMethod::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_contact_method::DeleteContactMethodOutput,
        crate::operation::delete_contact_method::DeleteContactMethodError,
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
    /// <p>The protocol that will be deleted, such as <code>Email</code> or <code>SMS</code> (text messaging).</p><note>
    /// <p>To delete an <code>Email</code> and an <code>SMS</code> contact method if you added both, you must run separate <code>DeleteContactMethod</code> actions to delete each protocol.</p>
    /// </note>
    pub fn protocol(mut self, input: crate::types::ContactProtocol) -> Self {
        self.inner = self.inner.protocol(input);
        self
    }
    /// <p>The protocol that will be deleted, such as <code>Email</code> or <code>SMS</code> (text messaging).</p><note>
    /// <p>To delete an <code>Email</code> and an <code>SMS</code> contact method if you added both, you must run separate <code>DeleteContactMethod</code> actions to delete each protocol.</p>
    /// </note>
    pub fn set_protocol(mut self, input: ::std::option::Option<crate::types::ContactProtocol>) -> Self {
        self.inner = self.inner.set_protocol(input);
        self
    }
    /// <p>The protocol that will be deleted, such as <code>Email</code> or <code>SMS</code> (text messaging).</p><note>
    /// <p>To delete an <code>Email</code> and an <code>SMS</code> contact method if you added both, you must run separate <code>DeleteContactMethod</code> actions to delete each protocol.</p>
    /// </note>
    pub fn get_protocol(&self) -> &::std::option::Option<crate::types::ContactProtocol> {
        self.inner.get_protocol()
    }
}
