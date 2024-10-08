// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::search_users::_search_users_output::SearchUsersOutputBuilder;

pub use crate::operation::search_users::_search_users_input::SearchUsersInputBuilder;

impl crate::operation::search_users::builders::SearchUsersInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::search_users::SearchUsersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::search_users::SearchUsersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.search_users();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SearchUsers`.
///
/// <p>Searches the specified directory for a user. You can find users that match the <code>SearchString</code> parameter with the value of their attributes included in the <code>SearchString</code> parameter.</p>
/// <p>This operation supports pagination with the use of the <code>NextToken</code> request and response parameters. If more results are available, the <code>SearchUsers.NextToken</code> member contains a token that you pass in the next call to <code>SearchUsers</code>. This retrieves the next set of items.</p>
/// <p>You can also specify a maximum number of return results with the <code>MaxResults</code> parameter.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SearchUsersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::search_users::builders::SearchUsersInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::search_users::SearchUsersOutput,
        crate::operation::search_users::SearchUsersError,
    > for SearchUsersFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::search_users::SearchUsersOutput,
            crate::operation::search_users::SearchUsersError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SearchUsersFluentBuilder {
    /// Creates a new `SearchUsersFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SearchUsers as a reference.
    pub fn as_input(&self) -> &crate::operation::search_users::builders::SearchUsersInputBuilder {
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
        crate::operation::search_users::SearchUsersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::search_users::SearchUsersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::search_users::SearchUsers::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::search_users::SearchUsers::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::search_users::SearchUsersOutput,
        crate::operation::search_users::SearchUsersError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::search_users::paginator::SearchUsersPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::search_users::paginator::SearchUsersPaginator {
        crate::operation::search_users::paginator::SearchUsersPaginator::new(self.handle, self.inner)
    }
    /// <p>The identifier (ID) of the directory that's associated with the user.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The identifier (ID) of the directory that's associated with the user.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>The identifier (ID) of the directory that's associated with the user.</p>
    pub fn get_directory_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_id()
    }
    /// <p>The domain name that's associated with the user.</p><note>
    /// <p>This parameter is optional, so you can return users outside of your Managed Microsoft AD domain. When no value is defined, only your Managed Microsoft AD users are returned.</p>
    /// <p>This value is case insensitive.</p>
    /// </note>
    pub fn realm(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.realm(input.into());
        self
    }
    /// <p>The domain name that's associated with the user.</p><note>
    /// <p>This parameter is optional, so you can return users outside of your Managed Microsoft AD domain. When no value is defined, only your Managed Microsoft AD users are returned.</p>
    /// <p>This value is case insensitive.</p>
    /// </note>
    pub fn set_realm(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_realm(input);
        self
    }
    /// <p>The domain name that's associated with the user.</p><note>
    /// <p>This parameter is optional, so you can return users outside of your Managed Microsoft AD domain. When no value is defined, only your Managed Microsoft AD users are returned.</p>
    /// <p>This value is case insensitive.</p>
    /// </note>
    pub fn get_realm(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_realm()
    }
    /// <p>The attribute value that you want to search for.</p><note>
    /// <p>Wildcard <code>(*)</code> searches aren't supported. For a list of supported attributes, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/ad_data_attributes.html">Directory Service Data Attributes</a>.</p>
    /// </note>
    pub fn search_string(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.search_string(input.into());
        self
    }
    /// <p>The attribute value that you want to search for.</p><note>
    /// <p>Wildcard <code>(*)</code> searches aren't supported. For a list of supported attributes, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/ad_data_attributes.html">Directory Service Data Attributes</a>.</p>
    /// </note>
    pub fn set_search_string(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_search_string(input);
        self
    }
    /// <p>The attribute value that you want to search for.</p><note>
    /// <p>Wildcard <code>(*)</code> searches aren't supported. For a list of supported attributes, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/ad_data_attributes.html">Directory Service Data Attributes</a>.</p>
    /// </note>
    pub fn get_search_string(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_search_string()
    }
    ///
    /// Appends an item to `SearchAttributes`.
    ///
    /// To override the contents of this collection use [`set_search_attributes`](Self::set_search_attributes).
    ///
    /// <p>One or more data attributes that are used to search for a user. For a list of supported attributes, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/ad_data_attributes.html">Directory Service Data Attributes</a>.</p>
    pub fn search_attributes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.search_attributes(input.into());
        self
    }
    /// <p>One or more data attributes that are used to search for a user. For a list of supported attributes, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/ad_data_attributes.html">Directory Service Data Attributes</a>.</p>
    pub fn set_search_attributes(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_search_attributes(input);
        self
    }
    /// <p>One or more data attributes that are used to search for a user. For a list of supported attributes, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/ad_data_attributes.html">Directory Service Data Attributes</a>.</p>
    pub fn get_search_attributes(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_search_attributes()
    }
    /// <p>An encoded paging token for paginated calls that can be passed back to retrieve the next page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An encoded paging token for paginated calls that can be passed back to retrieve the next page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>An encoded paging token for paginated calls that can be passed back to retrieve the next page.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to be returned per request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to be returned per request.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to be returned per request.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
