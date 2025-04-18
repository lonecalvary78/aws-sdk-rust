// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_resource_record_sets::_list_resource_record_sets_output::ListResourceRecordSetsOutputBuilder;

pub use crate::operation::list_resource_record_sets::_list_resource_record_sets_input::ListResourceRecordSetsInputBuilder;

impl crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_resource_record_sets::ListResourceRecordSetsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_resource_record_sets::ListResourceRecordSetsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_resource_record_sets();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListResourceRecordSets`.
///
/// <p>Lists the resource record sets in a specified hosted zone.</p>
/// <p><code>ListResourceRecordSets</code> returns up to 300 resource record sets at a time in ASCII order, beginning at a position specified by the <code>name</code> and <code>type</code> elements.</p>
/// <p><b>Sort order</b></p>
/// <p><code>ListResourceRecordSets</code> sorts results first by DNS name with the labels reversed, for example:</p>
/// <p><code>com.example.www.</code></p>
/// <p>Note the trailing dot, which can change the sort order when the record name contains characters that appear before <code>.</code> (decimal 46) in the ASCII table. These characters include the following: <code>! " # $ % &amp; ' ( ) * + , -</code></p>
/// <p>When multiple records have the same DNS name, <code>ListResourceRecordSets</code> sorts results by the record type.</p>
/// <p><b>Specifying where to start listing records</b></p>
/// <p>You can use the name and type elements to specify the resource record set that the list begins with:</p>
/// <dl>
/// <dt>
/// If you do not specify Name or Type
/// </dt>
/// <dd>
/// <p>The results begin with the first resource record set that the hosted zone contains.</p>
/// </dd>
/// <dt>
/// If you specify Name but not Type
/// </dt>
/// <dd>
/// <p>The results begin with the first resource record set in the list whose name is greater than or equal to <code>Name</code>.</p>
/// </dd>
/// <dt>
/// If you specify Type but not Name
/// </dt>
/// <dd>
/// <p>Amazon Route 53 returns the <code>InvalidInput</code> error.</p>
/// </dd>
/// <dt>
/// If you specify both Name and Type
/// </dt>
/// <dd>
/// <p>The results begin with the first resource record set in the list whose name is greater than or equal to <code>Name</code>, and whose type is greater than or equal to <code>Type</code>.</p><note>
/// <p>Type is only used to sort between records with the same record Name.</p>
/// </note>
/// </dd>
/// </dl>
/// <p><b>Resource record sets that are PENDING</b></p>
/// <p>This action returns the most current version of the records. This includes records that are <code>PENDING</code>, and that are not yet available on all Route 53 DNS servers.</p>
/// <p><b>Changing resource record sets</b></p>
/// <p>To ensure that you get an accurate listing of the resource record sets for a hosted zone at a point in time, do not submit a <code>ChangeResourceRecordSets</code> request while you're paging through the results of a <code>ListResourceRecordSets</code> request. If you do, some pages may display results without the latest changes while other pages display results with the latest changes.</p>
/// <p><b>Displaying the next page of results</b></p>
/// <p>If a <code>ListResourceRecordSets</code> command returns more than one page of results, the value of <code>IsTruncated</code> is <code>true</code>. To display the next page of results, get the values of <code>NextRecordName</code>, <code>NextRecordType</code>, and <code>NextRecordIdentifier</code> (if any) from the response. Then submit another <code>ListResourceRecordSets</code> request, and specify those values for <code>StartRecordName</code>, <code>StartRecordType</code>, and <code>StartRecordIdentifier</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListResourceRecordSetsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_resource_record_sets::ListResourceRecordSetsOutput,
        crate::operation::list_resource_record_sets::ListResourceRecordSetsError,
    > for ListResourceRecordSetsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_resource_record_sets::ListResourceRecordSetsOutput,
            crate::operation::list_resource_record_sets::ListResourceRecordSetsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListResourceRecordSetsFluentBuilder {
    /// Creates a new `ListResourceRecordSetsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListResourceRecordSets as a reference.
    pub fn as_input(&self) -> &crate::operation::list_resource_record_sets::builders::ListResourceRecordSetsInputBuilder {
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
        crate::operation::list_resource_record_sets::ListResourceRecordSetsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_resource_record_sets::ListResourceRecordSetsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_resource_record_sets::ListResourceRecordSets::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_resource_record_sets::ListResourceRecordSets::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_resource_record_sets::ListResourceRecordSetsOutput,
        crate::operation::list_resource_record_sets::ListResourceRecordSetsError,
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
    /// <p>The ID of the hosted zone that contains the resource record sets that you want to list.</p>
    pub fn hosted_zone_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.hosted_zone_id(input.into());
        self
    }
    /// <p>The ID of the hosted zone that contains the resource record sets that you want to list.</p>
    pub fn set_hosted_zone_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_hosted_zone_id(input);
        self
    }
    /// <p>The ID of the hosted zone that contains the resource record sets that you want to list.</p>
    pub fn get_hosted_zone_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_hosted_zone_id()
    }
    /// <p>The first name in the lexicographic ordering of resource record sets that you want to list. If the specified record name doesn't exist, the results begin with the first resource record set that has a name greater than the value of <code>name</code>.</p>
    pub fn start_record_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.start_record_name(input.into());
        self
    }
    /// <p>The first name in the lexicographic ordering of resource record sets that you want to list. If the specified record name doesn't exist, the results begin with the first resource record set that has a name greater than the value of <code>name</code>.</p>
    pub fn set_start_record_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_start_record_name(input);
        self
    }
    /// <p>The first name in the lexicographic ordering of resource record sets that you want to list. If the specified record name doesn't exist, the results begin with the first resource record set that has a name greater than the value of <code>name</code>.</p>
    pub fn get_start_record_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_start_record_name()
    }
    /// <p>The type of resource record set to begin the record listing from.</p>
    /// <p>Valid values for basic resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>NS</code> | <code>PTR</code> | <code>SOA</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code></p>
    /// <p>Values for weighted, latency, geolocation, and failover resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>PTR</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code></p>
    /// <p>Values for alias resource record sets:</p>
    /// <ul>
    /// <li>
    /// <p><b>API Gateway custom regional API or edge-optimized API</b>: A</p></li>
    /// <li>
    /// <p><b>CloudFront distribution</b>: A or AAAA</p></li>
    /// <li>
    /// <p><b>Elastic Beanstalk environment that has a regionalized subdomain</b>: A</p></li>
    /// <li>
    /// <p><b>Elastic Load Balancing load balancer</b>: A | AAAA</p></li>
    /// <li>
    /// <p><b>S3 bucket</b>: A</p></li>
    /// <li>
    /// <p><b>VPC interface VPC endpoint</b>: A</p></li>
    /// <li>
    /// <p><b>Another resource record set in this hosted zone:</b> The type of the resource record set that the alias references.</p></li>
    /// </ul>
    /// <p>Constraint: Specifying <code>type</code> without specifying <code>name</code> returns an <code>InvalidInput</code> error.</p>
    pub fn start_record_type(mut self, input: crate::types::RrType) -> Self {
        self.inner = self.inner.start_record_type(input);
        self
    }
    /// <p>The type of resource record set to begin the record listing from.</p>
    /// <p>Valid values for basic resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>NS</code> | <code>PTR</code> | <code>SOA</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code></p>
    /// <p>Values for weighted, latency, geolocation, and failover resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>PTR</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code></p>
    /// <p>Values for alias resource record sets:</p>
    /// <ul>
    /// <li>
    /// <p><b>API Gateway custom regional API or edge-optimized API</b>: A</p></li>
    /// <li>
    /// <p><b>CloudFront distribution</b>: A or AAAA</p></li>
    /// <li>
    /// <p><b>Elastic Beanstalk environment that has a regionalized subdomain</b>: A</p></li>
    /// <li>
    /// <p><b>Elastic Load Balancing load balancer</b>: A | AAAA</p></li>
    /// <li>
    /// <p><b>S3 bucket</b>: A</p></li>
    /// <li>
    /// <p><b>VPC interface VPC endpoint</b>: A</p></li>
    /// <li>
    /// <p><b>Another resource record set in this hosted zone:</b> The type of the resource record set that the alias references.</p></li>
    /// </ul>
    /// <p>Constraint: Specifying <code>type</code> without specifying <code>name</code> returns an <code>InvalidInput</code> error.</p>
    pub fn set_start_record_type(mut self, input: ::std::option::Option<crate::types::RrType>) -> Self {
        self.inner = self.inner.set_start_record_type(input);
        self
    }
    /// <p>The type of resource record set to begin the record listing from.</p>
    /// <p>Valid values for basic resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>NS</code> | <code>PTR</code> | <code>SOA</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code></p>
    /// <p>Values for weighted, latency, geolocation, and failover resource record sets: <code>A</code> | <code>AAAA</code> | <code>CAA</code> | <code>CNAME</code> | <code>MX</code> | <code>NAPTR</code> | <code>PTR</code> | <code>SPF</code> | <code>SRV</code> | <code>TXT</code></p>
    /// <p>Values for alias resource record sets:</p>
    /// <ul>
    /// <li>
    /// <p><b>API Gateway custom regional API or edge-optimized API</b>: A</p></li>
    /// <li>
    /// <p><b>CloudFront distribution</b>: A or AAAA</p></li>
    /// <li>
    /// <p><b>Elastic Beanstalk environment that has a regionalized subdomain</b>: A</p></li>
    /// <li>
    /// <p><b>Elastic Load Balancing load balancer</b>: A | AAAA</p></li>
    /// <li>
    /// <p><b>S3 bucket</b>: A</p></li>
    /// <li>
    /// <p><b>VPC interface VPC endpoint</b>: A</p></li>
    /// <li>
    /// <p><b>Another resource record set in this hosted zone:</b> The type of the resource record set that the alias references.</p></li>
    /// </ul>
    /// <p>Constraint: Specifying <code>type</code> without specifying <code>name</code> returns an <code>InvalidInput</code> error.</p>
    pub fn get_start_record_type(&self) -> &::std::option::Option<crate::types::RrType> {
        self.inner.get_start_record_type()
    }
    /// <p><i>Resource record sets that have a routing policy other than simple:</i> If results were truncated for a given DNS name and type, specify the value of <code>NextRecordIdentifier</code> from the previous response to get the next resource record set that has the current DNS name and type.</p>
    pub fn start_record_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.start_record_identifier(input.into());
        self
    }
    /// <p><i>Resource record sets that have a routing policy other than simple:</i> If results were truncated for a given DNS name and type, specify the value of <code>NextRecordIdentifier</code> from the previous response to get the next resource record set that has the current DNS name and type.</p>
    pub fn set_start_record_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_start_record_identifier(input);
        self
    }
    /// <p><i>Resource record sets that have a routing policy other than simple:</i> If results were truncated for a given DNS name and type, specify the value of <code>NextRecordIdentifier</code> from the previous response to get the next resource record set that has the current DNS name and type.</p>
    pub fn get_start_record_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_start_record_identifier()
    }
    /// <p>(Optional) The maximum number of resource records sets to include in the response body for this request. If the response includes more than <code>maxitems</code> resource record sets, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of the <code>NextRecordName</code> and <code>NextRecordType</code> elements in the response identify the first resource record set in the next group of <code>maxitems</code> resource record sets.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>(Optional) The maximum number of resource records sets to include in the response body for this request. If the response includes more than <code>maxitems</code> resource record sets, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of the <code>NextRecordName</code> and <code>NextRecordType</code> elements in the response identify the first resource record set in the next group of <code>maxitems</code> resource record sets.</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
    /// <p>(Optional) The maximum number of resource records sets to include in the response body for this request. If the response includes more than <code>maxitems</code> resource record sets, the value of the <code>IsTruncated</code> element in the response is <code>true</code>, and the values of the <code>NextRecordName</code> and <code>NextRecordType</code> elements in the response identify the first resource record set in the next group of <code>maxitems</code> resource record sets.</p>
    pub fn get_max_items(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_items()
    }
}
