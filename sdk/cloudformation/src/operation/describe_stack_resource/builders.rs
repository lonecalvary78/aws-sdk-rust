// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_stack_resource::_describe_stack_resource_output::DescribeStackResourceOutputBuilder;

pub use crate::operation::describe_stack_resource::_describe_stack_resource_input::DescribeStackResourceInputBuilder;

impl crate::operation::describe_stack_resource::builders::DescribeStackResourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_stack_resource::DescribeStackResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_stack_resource::DescribeStackResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_stack_resource();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeStackResource`.
///
/// <p>Returns a description of the specified resource in the specified stack.</p>
/// <p>For deleted stacks, DescribeStackResource returns resource information for up to 90 days after the stack has been deleted.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeStackResourceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_stack_resource::builders::DescribeStackResourceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_stack_resource::DescribeStackResourceOutput,
        crate::operation::describe_stack_resource::DescribeStackResourceError,
    > for DescribeStackResourceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_stack_resource::DescribeStackResourceOutput,
            crate::operation::describe_stack_resource::DescribeStackResourceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeStackResourceFluentBuilder {
    /// Creates a new `DescribeStackResourceFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeStackResource as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_stack_resource::builders::DescribeStackResourceInputBuilder {
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
        crate::operation::describe_stack_resource::DescribeStackResourceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_stack_resource::DescribeStackResourceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_stack_resource::DescribeStackResource::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_stack_resource::DescribeStackResource::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_stack_resource::DescribeStackResourceOutput,
        crate::operation::describe_stack_resource::DescribeStackResourceError,
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
    /// <p>The name or the unique stack ID that's associated with the stack, which aren't always interchangeable:</p>
    /// <ul>
    /// <li>
    /// <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p></li>
    /// <li>
    /// <p>Deleted stacks: You must specify the unique stack ID.</p></li>
    /// </ul>
    pub fn stack_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stack_name(input.into());
        self
    }
    /// <p>The name or the unique stack ID that's associated with the stack, which aren't always interchangeable:</p>
    /// <ul>
    /// <li>
    /// <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p></li>
    /// <li>
    /// <p>Deleted stacks: You must specify the unique stack ID.</p></li>
    /// </ul>
    pub fn set_stack_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stack_name(input);
        self
    }
    /// <p>The name or the unique stack ID that's associated with the stack, which aren't always interchangeable:</p>
    /// <ul>
    /// <li>
    /// <p>Running stacks: You can specify either the stack's name or its unique stack ID.</p></li>
    /// <li>
    /// <p>Deleted stacks: You must specify the unique stack ID.</p></li>
    /// </ul>
    pub fn get_stack_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stack_name()
    }
    /// <p>The logical name of the resource as specified in the template.</p>
    pub fn logical_resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.logical_resource_id(input.into());
        self
    }
    /// <p>The logical name of the resource as specified in the template.</p>
    pub fn set_logical_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_logical_resource_id(input);
        self
    }
    /// <p>The logical name of the resource as specified in the template.</p>
    pub fn get_logical_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_logical_resource_id()
    }
}
