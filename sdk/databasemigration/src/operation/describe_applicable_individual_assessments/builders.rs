// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_applicable_individual_assessments::_describe_applicable_individual_assessments_output::DescribeApplicableIndividualAssessmentsOutputBuilder;

pub use crate::operation::describe_applicable_individual_assessments::_describe_applicable_individual_assessments_input::DescribeApplicableIndividualAssessmentsInputBuilder;

impl crate::operation::describe_applicable_individual_assessments::builders::DescribeApplicableIndividualAssessmentsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_applicable_individual_assessments::DescribeApplicableIndividualAssessmentsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_applicable_individual_assessments::DescribeApplicableIndividualAssessmentsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_applicable_individual_assessments();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeApplicableIndividualAssessments`.
///
/// <p>Provides a list of individual assessments that you can specify for a new premigration assessment run, given one or more parameters.</p>
/// <p>If you specify an existing migration task, this operation provides the default individual assessments you can specify for that task. Otherwise, the specified parameters model elements of a possible migration task on which to base a premigration assessment run.</p>
/// <p>To use these migration task modeling parameters, you must specify an existing replication instance, a source database engine, a target database engine, and a migration type. This combination of parameters potentially limits the default individual assessments available for an assessment run created for a corresponding migration task.</p>
/// <p>If you specify no parameters, this operation provides a list of all possible individual assessments that you can specify for an assessment run. If you specify any one of the task modeling parameters, you must specify all of them or the operation cannot provide a list of individual assessments. The only parameter that you can specify alone is for an existing migration task. The specified task definition then determines the default list of individual assessments that you can specify in an assessment run for the task.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeApplicableIndividualAssessmentsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_applicable_individual_assessments::builders::DescribeApplicableIndividualAssessmentsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_applicable_individual_assessments::DescribeApplicableIndividualAssessmentsOutput,
        crate::operation::describe_applicable_individual_assessments::DescribeApplicableIndividualAssessmentsError,
    > for DescribeApplicableIndividualAssessmentsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_applicable_individual_assessments::DescribeApplicableIndividualAssessmentsOutput,
            crate::operation::describe_applicable_individual_assessments::DescribeApplicableIndividualAssessmentsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeApplicableIndividualAssessmentsFluentBuilder {
    /// Creates a new `DescribeApplicableIndividualAssessmentsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeApplicableIndividualAssessments as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_applicable_individual_assessments::builders::DescribeApplicableIndividualAssessmentsInputBuilder {
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
        crate::operation::describe_applicable_individual_assessments::DescribeApplicableIndividualAssessmentsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_applicable_individual_assessments::DescribeApplicableIndividualAssessmentsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_applicable_individual_assessments::DescribeApplicableIndividualAssessments::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_applicable_individual_assessments::DescribeApplicableIndividualAssessments::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_applicable_individual_assessments::DescribeApplicableIndividualAssessmentsOutput,
        crate::operation::describe_applicable_individual_assessments::DescribeApplicableIndividualAssessmentsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_applicable_individual_assessments::paginator::DescribeApplicableIndividualAssessmentsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_applicable_individual_assessments::paginator::DescribeApplicableIndividualAssessmentsPaginator {
        crate::operation::describe_applicable_individual_assessments::paginator::DescribeApplicableIndividualAssessmentsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>Amazon Resource Name (ARN) of a migration task on which you want to base the default list of individual assessments.</p>
    pub fn replication_task_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.replication_task_arn(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of a migration task on which you want to base the default list of individual assessments.</p>
    pub fn set_replication_task_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_replication_task_arn(input);
        self
    }
    /// <p>Amazon Resource Name (ARN) of a migration task on which you want to base the default list of individual assessments.</p>
    pub fn get_replication_task_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_replication_task_arn()
    }
    /// <p>ARN of a replication instance on which you want to base the default list of individual assessments.</p>
    pub fn replication_instance_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.replication_instance_arn(input.into());
        self
    }
    /// <p>ARN of a replication instance on which you want to base the default list of individual assessments.</p>
    pub fn set_replication_instance_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_replication_instance_arn(input);
        self
    }
    /// <p>ARN of a replication instance on which you want to base the default list of individual assessments.</p>
    pub fn get_replication_instance_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_replication_instance_arn()
    }
    /// <p>Amazon Resource Name (ARN) of a serverless replication on which you want to base the default list of individual assessments.</p>
    pub fn replication_config_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.replication_config_arn(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of a serverless replication on which you want to base the default list of individual assessments.</p>
    pub fn set_replication_config_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_replication_config_arn(input);
        self
    }
    /// <p>Amazon Resource Name (ARN) of a serverless replication on which you want to base the default list of individual assessments.</p>
    pub fn get_replication_config_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_replication_config_arn()
    }
    /// <p>Name of a database engine that the specified replication instance supports as a source.</p>
    pub fn source_engine_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_engine_name(input.into());
        self
    }
    /// <p>Name of a database engine that the specified replication instance supports as a source.</p>
    pub fn set_source_engine_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_engine_name(input);
        self
    }
    /// <p>Name of a database engine that the specified replication instance supports as a source.</p>
    pub fn get_source_engine_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_engine_name()
    }
    /// <p>Name of a database engine that the specified replication instance supports as a target.</p>
    pub fn target_engine_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_engine_name(input.into());
        self
    }
    /// <p>Name of a database engine that the specified replication instance supports as a target.</p>
    pub fn set_target_engine_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_engine_name(input);
        self
    }
    /// <p>Name of a database engine that the specified replication instance supports as a target.</p>
    pub fn get_target_engine_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_engine_name()
    }
    /// <p>Name of the migration type that each provided individual assessment must support.</p>
    pub fn migration_type(mut self, input: crate::types::MigrationTypeValue) -> Self {
        self.inner = self.inner.migration_type(input);
        self
    }
    /// <p>Name of the migration type that each provided individual assessment must support.</p>
    pub fn set_migration_type(mut self, input: ::std::option::Option<crate::types::MigrationTypeValue>) -> Self {
        self.inner = self.inner.set_migration_type(input);
        self
    }
    /// <p>Name of the migration type that each provided individual assessment must support.</p>
    pub fn get_migration_type(&self) -> &::std::option::Option<crate::types::MigrationTypeValue> {
        self.inner.get_migration_type()
    }
    /// <p>Maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>Maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>Maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>
    pub fn get_max_records(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_records()
    }
    /// <p>Optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>Optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>Optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
}
