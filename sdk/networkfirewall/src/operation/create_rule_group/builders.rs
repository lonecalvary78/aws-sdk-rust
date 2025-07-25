// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_rule_group::_create_rule_group_output::CreateRuleGroupOutputBuilder;

pub use crate::operation::create_rule_group::_create_rule_group_input::CreateRuleGroupInputBuilder;

impl crate::operation::create_rule_group::builders::CreateRuleGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_rule_group::CreateRuleGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_rule_group::CreateRuleGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_rule_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateRuleGroup`.
///
/// <p>Creates the specified stateless or stateful rule group, which includes the rules for network traffic inspection, a capacity setting, and tags.</p>
/// <p>You provide your rule group specification in your request using either <code>RuleGroup</code> or <code>Rules</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRuleGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_rule_group::builders::CreateRuleGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_rule_group::CreateRuleGroupOutput,
        crate::operation::create_rule_group::CreateRuleGroupError,
    > for CreateRuleGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_rule_group::CreateRuleGroupOutput,
            crate::operation::create_rule_group::CreateRuleGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateRuleGroupFluentBuilder {
    /// Creates a new `CreateRuleGroupFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateRuleGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::create_rule_group::builders::CreateRuleGroupInputBuilder {
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
        crate::operation::create_rule_group::CreateRuleGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_rule_group::CreateRuleGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_rule_group::CreateRuleGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_rule_group::CreateRuleGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_rule_group::CreateRuleGroupOutput,
        crate::operation::create_rule_group::CreateRuleGroupError,
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
    /// <p>The descriptive name of the rule group. You can't change the name of a rule group after you create it.</p>
    pub fn rule_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rule_group_name(input.into());
        self
    }
    /// <p>The descriptive name of the rule group. You can't change the name of a rule group after you create it.</p>
    pub fn set_rule_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rule_group_name(input);
        self
    }
    /// <p>The descriptive name of the rule group. You can't change the name of a rule group after you create it.</p>
    pub fn get_rule_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rule_group_name()
    }
    /// <p>An object that defines the rule group rules.</p><note>
    /// <p>You must provide either this rule group setting or a <code>Rules</code> setting, but not both.</p>
    /// </note>
    pub fn rule_group(mut self, input: crate::types::RuleGroup) -> Self {
        self.inner = self.inner.rule_group(input);
        self
    }
    /// <p>An object that defines the rule group rules.</p><note>
    /// <p>You must provide either this rule group setting or a <code>Rules</code> setting, but not both.</p>
    /// </note>
    pub fn set_rule_group(mut self, input: ::std::option::Option<crate::types::RuleGroup>) -> Self {
        self.inner = self.inner.set_rule_group(input);
        self
    }
    /// <p>An object that defines the rule group rules.</p><note>
    /// <p>You must provide either this rule group setting or a <code>Rules</code> setting, but not both.</p>
    /// </note>
    pub fn get_rule_group(&self) -> &::std::option::Option<crate::types::RuleGroup> {
        self.inner.get_rule_group()
    }
    /// <p>A string containing stateful rule group rules specifications in Suricata flat format, with one rule per line. Use this to import your existing Suricata compatible rule groups.</p><note>
    /// <p>You must provide either this rules setting or a populated <code>RuleGroup</code> setting, but not both.</p>
    /// </note>
    /// <p>You can provide your rule group specification in Suricata flat format through this setting when you create or update your rule group. The call response returns a <code>RuleGroup</code> object that Network Firewall has populated from your string.</p>
    pub fn rules(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rules(input.into());
        self
    }
    /// <p>A string containing stateful rule group rules specifications in Suricata flat format, with one rule per line. Use this to import your existing Suricata compatible rule groups.</p><note>
    /// <p>You must provide either this rules setting or a populated <code>RuleGroup</code> setting, but not both.</p>
    /// </note>
    /// <p>You can provide your rule group specification in Suricata flat format through this setting when you create or update your rule group. The call response returns a <code>RuleGroup</code> object that Network Firewall has populated from your string.</p>
    pub fn set_rules(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rules(input);
        self
    }
    /// <p>A string containing stateful rule group rules specifications in Suricata flat format, with one rule per line. Use this to import your existing Suricata compatible rule groups.</p><note>
    /// <p>You must provide either this rules setting or a populated <code>RuleGroup</code> setting, but not both.</p>
    /// </note>
    /// <p>You can provide your rule group specification in Suricata flat format through this setting when you create or update your rule group. The call response returns a <code>RuleGroup</code> object that Network Firewall has populated from your string.</p>
    pub fn get_rules(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rules()
    }
    /// <p>Indicates whether the rule group is stateless or stateful. If the rule group is stateless, it contains stateless rules. If it is stateful, it contains stateful rules.</p>
    pub fn r#type(mut self, input: crate::types::RuleGroupType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>Indicates whether the rule group is stateless or stateful. If the rule group is stateless, it contains stateless rules. If it is stateful, it contains stateful rules.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::RuleGroupType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>Indicates whether the rule group is stateless or stateful. If the rule group is stateless, it contains stateless rules. If it is stateful, it contains stateful rules.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::RuleGroupType> {
        self.inner.get_type()
    }
    /// <p>A description of the rule group.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the rule group.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the rule group.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The maximum operating resources that this rule group can use. Rule group capacity is fixed at creation. When you update a rule group, you are limited to this capacity. When you reference a rule group from a firewall policy, Network Firewall reserves this capacity for the rule group.</p>
    /// <p>You can retrieve the capacity that would be required for a rule group before you create the rule group by calling <code>CreateRuleGroup</code> with <code>DryRun</code> set to <code>TRUE</code>.</p><note>
    /// <p>You can't change or exceed this capacity when you update the rule group, so leave room for your rule group to grow.</p>
    /// </note>
    /// <p><b>Capacity for a stateless rule group</b></p>
    /// <p>For a stateless rule group, the capacity required is the sum of the capacity requirements of the individual rules that you expect to have in the rule group.</p>
    /// <p>To calculate the capacity requirement of a single rule, multiply the capacity requirement values of each of the rule's match settings:</p>
    /// <ul>
    /// <li>
    /// <p>A match setting with no criteria specified has a value of 1.</p></li>
    /// <li>
    /// <p>A match setting with <code>Any</code> specified has a value of 1.</p></li>
    /// <li>
    /// <p>All other match settings have a value equal to the number of elements provided in the setting. For example, a protocol setting \["UDP"\] and a source setting \["10.0.0.0/24"\] each have a value of 1. A protocol setting \["UDP","TCP"\] has a value of 2. A source setting \["10.0.0.0/24","10.0.0.1/24","10.0.0.2/24"\] has a value of 3.</p></li>
    /// </ul>
    /// <p>A rule with no criteria specified in any of its match settings has a capacity requirement of 1. A rule with protocol setting \["UDP","TCP"\], source setting \["10.0.0.0/24","10.0.0.1/24","10.0.0.2/24"\], and a single specification or no specification for each of the other match settings has a capacity requirement of 6.</p>
    /// <p><b>Capacity for a stateful rule group</b></p>
    /// <p>For a stateful rule group, the minimum capacity required is the number of individual rules that you expect to have in the rule group.</p>
    pub fn capacity(mut self, input: i32) -> Self {
        self.inner = self.inner.capacity(input);
        self
    }
    /// <p>The maximum operating resources that this rule group can use. Rule group capacity is fixed at creation. When you update a rule group, you are limited to this capacity. When you reference a rule group from a firewall policy, Network Firewall reserves this capacity for the rule group.</p>
    /// <p>You can retrieve the capacity that would be required for a rule group before you create the rule group by calling <code>CreateRuleGroup</code> with <code>DryRun</code> set to <code>TRUE</code>.</p><note>
    /// <p>You can't change or exceed this capacity when you update the rule group, so leave room for your rule group to grow.</p>
    /// </note>
    /// <p><b>Capacity for a stateless rule group</b></p>
    /// <p>For a stateless rule group, the capacity required is the sum of the capacity requirements of the individual rules that you expect to have in the rule group.</p>
    /// <p>To calculate the capacity requirement of a single rule, multiply the capacity requirement values of each of the rule's match settings:</p>
    /// <ul>
    /// <li>
    /// <p>A match setting with no criteria specified has a value of 1.</p></li>
    /// <li>
    /// <p>A match setting with <code>Any</code> specified has a value of 1.</p></li>
    /// <li>
    /// <p>All other match settings have a value equal to the number of elements provided in the setting. For example, a protocol setting \["UDP"\] and a source setting \["10.0.0.0/24"\] each have a value of 1. A protocol setting \["UDP","TCP"\] has a value of 2. A source setting \["10.0.0.0/24","10.0.0.1/24","10.0.0.2/24"\] has a value of 3.</p></li>
    /// </ul>
    /// <p>A rule with no criteria specified in any of its match settings has a capacity requirement of 1. A rule with protocol setting \["UDP","TCP"\], source setting \["10.0.0.0/24","10.0.0.1/24","10.0.0.2/24"\], and a single specification or no specification for each of the other match settings has a capacity requirement of 6.</p>
    /// <p><b>Capacity for a stateful rule group</b></p>
    /// <p>For a stateful rule group, the minimum capacity required is the number of individual rules that you expect to have in the rule group.</p>
    pub fn set_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_capacity(input);
        self
    }
    /// <p>The maximum operating resources that this rule group can use. Rule group capacity is fixed at creation. When you update a rule group, you are limited to this capacity. When you reference a rule group from a firewall policy, Network Firewall reserves this capacity for the rule group.</p>
    /// <p>You can retrieve the capacity that would be required for a rule group before you create the rule group by calling <code>CreateRuleGroup</code> with <code>DryRun</code> set to <code>TRUE</code>.</p><note>
    /// <p>You can't change or exceed this capacity when you update the rule group, so leave room for your rule group to grow.</p>
    /// </note>
    /// <p><b>Capacity for a stateless rule group</b></p>
    /// <p>For a stateless rule group, the capacity required is the sum of the capacity requirements of the individual rules that you expect to have in the rule group.</p>
    /// <p>To calculate the capacity requirement of a single rule, multiply the capacity requirement values of each of the rule's match settings:</p>
    /// <ul>
    /// <li>
    /// <p>A match setting with no criteria specified has a value of 1.</p></li>
    /// <li>
    /// <p>A match setting with <code>Any</code> specified has a value of 1.</p></li>
    /// <li>
    /// <p>All other match settings have a value equal to the number of elements provided in the setting. For example, a protocol setting \["UDP"\] and a source setting \["10.0.0.0/24"\] each have a value of 1. A protocol setting \["UDP","TCP"\] has a value of 2. A source setting \["10.0.0.0/24","10.0.0.1/24","10.0.0.2/24"\] has a value of 3.</p></li>
    /// </ul>
    /// <p>A rule with no criteria specified in any of its match settings has a capacity requirement of 1. A rule with protocol setting \["UDP","TCP"\], source setting \["10.0.0.0/24","10.0.0.1/24","10.0.0.2/24"\], and a single specification or no specification for each of the other match settings has a capacity requirement of 6.</p>
    /// <p><b>Capacity for a stateful rule group</b></p>
    /// <p>For a stateful rule group, the minimum capacity required is the number of individual rules that you expect to have in the rule group.</p>
    pub fn get_capacity(&self) -> &::std::option::Option<i32> {
        self.inner.get_capacity()
    }
    ///
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The key:value pairs to associate with the resource.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The key:value pairs to associate with the resource.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The key:value pairs to associate with the resource.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>Indicates whether you want Network Firewall to just check the validity of the request, rather than run the request.</p>
    /// <p>If set to <code>TRUE</code>, Network Firewall checks whether the request can run successfully, but doesn't actually make the requested changes. The call returns the value that the request would return if you ran it with dry run set to <code>FALSE</code>, but doesn't make additions or changes to your resources. This option allows you to make sure that you have the required permissions to run the request and that your request parameters are valid.</p>
    /// <p>If set to <code>FALSE</code>, Network Firewall makes the requested changes to your resources.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Indicates whether you want Network Firewall to just check the validity of the request, rather than run the request.</p>
    /// <p>If set to <code>TRUE</code>, Network Firewall checks whether the request can run successfully, but doesn't actually make the requested changes. The call returns the value that the request would return if you ran it with dry run set to <code>FALSE</code>, but doesn't make additions or changes to your resources. This option allows you to make sure that you have the required permissions to run the request and that your request parameters are valid.</p>
    /// <p>If set to <code>FALSE</code>, Network Firewall makes the requested changes to your resources.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Indicates whether you want Network Firewall to just check the validity of the request, rather than run the request.</p>
    /// <p>If set to <code>TRUE</code>, Network Firewall checks whether the request can run successfully, but doesn't actually make the requested changes. The call returns the value that the request would return if you ran it with dry run set to <code>FALSE</code>, but doesn't make additions or changes to your resources. This option allows you to make sure that you have the required permissions to run the request and that your request parameters are valid.</p>
    /// <p>If set to <code>FALSE</code>, Network Firewall makes the requested changes to your resources.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>A complex type that contains settings for encryption of your rule group resources.</p>
    pub fn encryption_configuration(mut self, input: crate::types::EncryptionConfiguration) -> Self {
        self.inner = self.inner.encryption_configuration(input);
        self
    }
    /// <p>A complex type that contains settings for encryption of your rule group resources.</p>
    pub fn set_encryption_configuration(mut self, input: ::std::option::Option<crate::types::EncryptionConfiguration>) -> Self {
        self.inner = self.inner.set_encryption_configuration(input);
        self
    }
    /// <p>A complex type that contains settings for encryption of your rule group resources.</p>
    pub fn get_encryption_configuration(&self) -> &::std::option::Option<crate::types::EncryptionConfiguration> {
        self.inner.get_encryption_configuration()
    }
    /// <p>A complex type that contains metadata about the rule group that your own rule group is copied from. You can use the metadata to keep track of updates made to the originating rule group.</p>
    pub fn source_metadata(mut self, input: crate::types::SourceMetadata) -> Self {
        self.inner = self.inner.source_metadata(input);
        self
    }
    /// <p>A complex type that contains metadata about the rule group that your own rule group is copied from. You can use the metadata to keep track of updates made to the originating rule group.</p>
    pub fn set_source_metadata(mut self, input: ::std::option::Option<crate::types::SourceMetadata>) -> Self {
        self.inner = self.inner.set_source_metadata(input);
        self
    }
    /// <p>A complex type that contains metadata about the rule group that your own rule group is copied from. You can use the metadata to keep track of updates made to the originating rule group.</p>
    pub fn get_source_metadata(&self) -> &::std::option::Option<crate::types::SourceMetadata> {
        self.inner.get_source_metadata()
    }
    /// <p>Indicates whether you want Network Firewall to analyze the stateless rules in the rule group for rule behavior such as asymmetric routing. If set to <code>TRUE</code>, Network Firewall runs the analysis and then creates the rule group for you. To run the stateless rule group analyzer without creating the rule group, set <code>DryRun</code> to <code>TRUE</code>.</p>
    pub fn analyze_rule_group(mut self, input: bool) -> Self {
        self.inner = self.inner.analyze_rule_group(input);
        self
    }
    /// <p>Indicates whether you want Network Firewall to analyze the stateless rules in the rule group for rule behavior such as asymmetric routing. If set to <code>TRUE</code>, Network Firewall runs the analysis and then creates the rule group for you. To run the stateless rule group analyzer without creating the rule group, set <code>DryRun</code> to <code>TRUE</code>.</p>
    pub fn set_analyze_rule_group(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_analyze_rule_group(input);
        self
    }
    /// <p>Indicates whether you want Network Firewall to analyze the stateless rules in the rule group for rule behavior such as asymmetric routing. If set to <code>TRUE</code>, Network Firewall runs the analysis and then creates the rule group for you. To run the stateless rule group analyzer without creating the rule group, set <code>DryRun</code> to <code>TRUE</code>.</p>
    pub fn get_analyze_rule_group(&self) -> &::std::option::Option<bool> {
        self.inner.get_analyze_rule_group()
    }
    /// <p>An object that contains a <code>RuleOptions</code> array of strings. You use <code>RuleOptions</code> to determine which of the following <code>RuleSummary</code> values are returned in response to <code>DescribeRuleGroupSummary</code>.</p>
    /// <ul>
    /// <li>
    /// <p><code>Metadata</code> - returns</p></li>
    /// <li>
    /// <p><code>Msg</code></p></li>
    /// <li>
    /// <p><code>SID</code></p></li>
    /// </ul>
    pub fn summary_configuration(mut self, input: crate::types::SummaryConfiguration) -> Self {
        self.inner = self.inner.summary_configuration(input);
        self
    }
    /// <p>An object that contains a <code>RuleOptions</code> array of strings. You use <code>RuleOptions</code> to determine which of the following <code>RuleSummary</code> values are returned in response to <code>DescribeRuleGroupSummary</code>.</p>
    /// <ul>
    /// <li>
    /// <p><code>Metadata</code> - returns</p></li>
    /// <li>
    /// <p><code>Msg</code></p></li>
    /// <li>
    /// <p><code>SID</code></p></li>
    /// </ul>
    pub fn set_summary_configuration(mut self, input: ::std::option::Option<crate::types::SummaryConfiguration>) -> Self {
        self.inner = self.inner.set_summary_configuration(input);
        self
    }
    /// <p>An object that contains a <code>RuleOptions</code> array of strings. You use <code>RuleOptions</code> to determine which of the following <code>RuleSummary</code> values are returned in response to <code>DescribeRuleGroupSummary</code>.</p>
    /// <ul>
    /// <li>
    /// <p><code>Metadata</code> - returns</p></li>
    /// <li>
    /// <p><code>Msg</code></p></li>
    /// <li>
    /// <p><code>SID</code></p></li>
    /// </ul>
    pub fn get_summary_configuration(&self) -> &::std::option::Option<crate::types::SummaryConfiguration> {
        self.inner.get_summary_configuration()
    }
}
