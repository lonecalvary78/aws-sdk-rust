// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_distributions_by_anycast_ip_list_id::_list_distributions_by_anycast_ip_list_id_output::ListDistributionsByAnycastIpListIdOutputBuilder;

pub use crate::operation::list_distributions_by_anycast_ip_list_id::_list_distributions_by_anycast_ip_list_id_input::ListDistributionsByAnycastIpListIdInputBuilder;

impl crate::operation::list_distributions_by_anycast_ip_list_id::builders::ListDistributionsByAnycastIpListIdInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_distributions_by_anycast_ip_list_id::ListDistributionsByAnycastIpListIdOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_distributions_by_anycast_ip_list_id::ListDistributionsByAnycastIpListIdError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_distributions_by_anycast_ip_list_id();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListDistributionsByAnycastIpListId`.
///
/// <p>Lists the distributions in your account that are associated with the specified <code>AnycastIpListId</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListDistributionsByAnycastIpListIdFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_distributions_by_anycast_ip_list_id::builders::ListDistributionsByAnycastIpListIdInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_distributions_by_anycast_ip_list_id::ListDistributionsByAnycastIpListIdOutput,
        crate::operation::list_distributions_by_anycast_ip_list_id::ListDistributionsByAnycastIpListIdError,
    > for ListDistributionsByAnycastIpListIdFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_distributions_by_anycast_ip_list_id::ListDistributionsByAnycastIpListIdOutput,
            crate::operation::list_distributions_by_anycast_ip_list_id::ListDistributionsByAnycastIpListIdError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListDistributionsByAnycastIpListIdFluentBuilder {
    /// Creates a new `ListDistributionsByAnycastIpListIdFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListDistributionsByAnycastIpListId as a reference.
    pub fn as_input(&self) -> &crate::operation::list_distributions_by_anycast_ip_list_id::builders::ListDistributionsByAnycastIpListIdInputBuilder {
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
        crate::operation::list_distributions_by_anycast_ip_list_id::ListDistributionsByAnycastIpListIdOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_distributions_by_anycast_ip_list_id::ListDistributionsByAnycastIpListIdError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::list_distributions_by_anycast_ip_list_id::ListDistributionsByAnycastIpListId::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::list_distributions_by_anycast_ip_list_id::ListDistributionsByAnycastIpListId::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_distributions_by_anycast_ip_list_id::ListDistributionsByAnycastIpListIdOutput,
        crate::operation::list_distributions_by_anycast_ip_list_id::ListDistributionsByAnycastIpListIdError,
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
    /// <p>Use this field when paginating results to indicate where to begin in your list. The response includes items in the list that occur after the marker. To get the next page of the list, set this field's value to the value of <code>NextMarker</code> from the current page's response.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>Use this field when paginating results to indicate where to begin in your list. The response includes items in the list that occur after the marker. To get the next page of the list, set this field's value to the value of <code>NextMarker</code> from the current page's response.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>Use this field when paginating results to indicate where to begin in your list. The response includes items in the list that occur after the marker. To get the next page of the list, set this field's value to the value of <code>NextMarker</code> from the current page's response.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
    /// <p>The maximum number of distributions that you want returned in the response.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>The maximum number of distributions that you want returned in the response.</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
    /// <p>The maximum number of distributions that you want returned in the response.</p>
    pub fn get_max_items(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_items()
    }
    /// <p>The ID of the Anycast static IP list.</p>
    pub fn anycast_ip_list_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.anycast_ip_list_id(input.into());
        self
    }
    /// <p>The ID of the Anycast static IP list.</p>
    pub fn set_anycast_ip_list_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_anycast_ip_list_id(input);
        self
    }
    /// <p>The ID of the Anycast static IP list.</p>
    pub fn get_anycast_ip_list_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_anycast_ip_list_id()
    }
}
