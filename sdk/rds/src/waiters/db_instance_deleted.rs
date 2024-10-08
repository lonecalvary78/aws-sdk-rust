// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `db_instance_deleted` waiter.
///
/// This builder is intended to be used similar to the other fluent builders for
/// normal operations on the client. However, instead of a `send` method, it has
/// a `wait` method that takes a maximum amount of time to wait.
///
/// Construct this fluent builder using the client by importing the
/// [`Waiters`](crate::client::Waiters) trait and calling the methods
/// prefixed with `wait_until`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DbInstanceDeletedFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_db_instances::builders::DescribeDbInstancesInputBuilder,
}
impl DbInstanceDeletedFluentBuilder {
    /// Creates a new `DbInstanceDeletedFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the DescribeDBInstances as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_db_instances::builders::DescribeDbInstancesInputBuilder {
        &self.inner
    }
    /// Wait for `db_instance_deleted`
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<
        crate::waiters::db_instance_deleted::DbInstanceDeletedFinalPoll,
        crate::waiters::db_instance_deleted::WaitUntilDbInstanceDeletedError,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_db_instances::DescribeDBInstances::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            ::std::option::Option::None,
        )
        .with_operation_plugin(crate::sdk_feature_tracker::waiter::WaiterFeatureTrackerRuntimePlugin::new());
        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
        let runtime_components_builder = runtime_plugins
            .apply_client_configuration(&mut cfg)
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let time_components = runtime_components_builder.into_time_components();
        let sleep_impl = time_components.sleep_impl().expect("a sleep impl is required by waiters");
        let time_source = time_components.time_source().expect("a time source is required by waiters");

        let acceptor = move |result: ::std::result::Result<
            &crate::operation::describe_db_instances::DescribeDbInstancesOutput,
            &crate::operation::describe_db_instances::DescribeDBInstancesError,
        >| {
            // Matches: {"output":{"path":"length(DBInstances) == `0`","expected":"true","comparator":"booleanEquals"}}
            if crate::waiters::matchers::match_describe_db_instances_6e146202dc53a1aac(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            // Matches: {"errorType":"DBInstanceNotFound"}
            if crate::waiters::matchers::match_describe_db_instances_44c2cb846aa391790(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            // Matches: {"output":{"path":"DBInstances[].DBInstanceStatus","expected":"creating","comparator":"anyStringEquals"}}
            if crate::waiters::matchers::match_describe_db_instances_bbcaccab59728b5f5(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"output":{"path":"DBInstances[].DBInstanceStatus","expected":"modifying","comparator":"anyStringEquals"}}
            if crate::waiters::matchers::match_describe_db_instances_866e8bcc02dcb5d94(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"output":{"path":"DBInstances[].DBInstanceStatus","expected":"rebooting","comparator":"anyStringEquals"}}
            if crate::waiters::matchers::match_describe_db_instances_ceb87a540084b9d41(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"output":{"path":"DBInstances[].DBInstanceStatus","expected":"resetting-master-credentials","comparator":"anyStringEquals"}}
            if crate::waiters::matchers::match_describe_db_instances_b34d711b975263077(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
        };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::describe_db_instances::DescribeDBInstances::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(30))
            .max_delay(::std::time::Duration::from_secs(120))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
    }
    /// <p>The user-supplied instance identifier or the Amazon Resource Name (ARN) of the DB instance. If this parameter is specified, information from only the specific DB instance is returned. This parameter isn't case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>If supplied, must match the identifier of an existing DB instance.</p></li>
    /// </ul>
    pub fn db_instance_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_instance_identifier(input.into());
        self
    }
    /// <p>The user-supplied instance identifier or the Amazon Resource Name (ARN) of the DB instance. If this parameter is specified, information from only the specific DB instance is returned. This parameter isn't case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>If supplied, must match the identifier of an existing DB instance.</p></li>
    /// </ul>
    pub fn set_db_instance_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_instance_identifier(input);
        self
    }
    /// <p>The user-supplied instance identifier or the Amazon Resource Name (ARN) of the DB instance. If this parameter is specified, information from only the specific DB instance is returned. This parameter isn't case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>If supplied, must match the identifier of an existing DB instance.</p></li>
    /// </ul>
    pub fn get_db_instance_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_db_instance_identifier()
    }
    ///
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>A filter that specifies one or more DB instances to describe.</p>
    /// <p>Supported Filters:</p>
    /// <ul>
    /// <li>
    /// <p><code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list only includes information about the DB instances associated with the DB clusters identified by these ARNs.</p></li>
    /// <li>
    /// <p><code>db-instance-id</code> - Accepts DB instance identifiers and DB instance Amazon Resource Names (ARNs). The results list only includes information about the DB instances identified by these ARNs.</p></li>
    /// <li>
    /// <p><code>dbi-resource-id</code> - Accepts DB instance resource identifiers. The results list only includes information about the DB instances identified by these DB instance resource identifiers.</p></li>
    /// <li>
    /// <p><code>domain</code> - Accepts Active Directory directory IDs. The results list only includes information about the DB instances associated with these domains.</p></li>
    /// <li>
    /// <p><code>engine</code> - Accepts engine names. The results list only includes information about the DB instances for these engines.</p></li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>A filter that specifies one or more DB instances to describe.</p>
    /// <p>Supported Filters:</p>
    /// <ul>
    /// <li>
    /// <p><code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list only includes information about the DB instances associated with the DB clusters identified by these ARNs.</p></li>
    /// <li>
    /// <p><code>db-instance-id</code> - Accepts DB instance identifiers and DB instance Amazon Resource Names (ARNs). The results list only includes information about the DB instances identified by these ARNs.</p></li>
    /// <li>
    /// <p><code>dbi-resource-id</code> - Accepts DB instance resource identifiers. The results list only includes information about the DB instances identified by these DB instance resource identifiers.</p></li>
    /// <li>
    /// <p><code>domain</code> - Accepts Active Directory directory IDs. The results list only includes information about the DB instances associated with these domains.</p></li>
    /// <li>
    /// <p><code>engine</code> - Accepts engine names. The results list only includes information about the DB instances for these engines.</p></li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>A filter that specifies one or more DB instances to describe.</p>
    /// <p>Supported Filters:</p>
    /// <ul>
    /// <li>
    /// <p><code>db-cluster-id</code> - Accepts DB cluster identifiers and DB cluster Amazon Resource Names (ARNs). The results list only includes information about the DB instances associated with the DB clusters identified by these ARNs.</p></li>
    /// <li>
    /// <p><code>db-instance-id</code> - Accepts DB instance identifiers and DB instance Amazon Resource Names (ARNs). The results list only includes information about the DB instances identified by these ARNs.</p></li>
    /// <li>
    /// <p><code>dbi-resource-id</code> - Accepts DB instance resource identifiers. The results list only includes information about the DB instances identified by these DB instance resource identifiers.</p></li>
    /// <li>
    /// <p><code>domain</code> - Accepts Active Directory directory IDs. The results list only includes information about the DB instances associated with these domains.</p></li>
    /// <li>
    /// <p><code>engine</code> - Accepts engine names. The results list only includes information about the DB instances for these engines.</p></li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that you can retrieve the remaining results.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that you can retrieve the remaining results.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that you can retrieve the remaining results.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn get_max_records(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_records()
    }
    /// <p>An optional pagination token provided by a previous <code>DescribeDBInstances</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>An optional pagination token provided by a previous <code>DescribeDBInstances</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>An optional pagination token provided by a previous <code>DescribeDBInstances</code> request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
}

/// Successful return type for the `db_instance_deleted` waiter.
pub type DbInstanceDeletedFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::describe_db_instances::DescribeDbInstancesOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::describe_db_instances::DescribeDBInstancesError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `db_instance_deleted` waiter.
pub type WaitUntilDbInstanceDeletedError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::describe_db_instances::DescribeDbInstancesOutput,
    crate::operation::describe_db_instances::DescribeDBInstancesError,
>;
