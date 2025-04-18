// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_rule_group::_update_rule_group_output::UpdateRuleGroupOutputBuilder;

pub use crate::operation::update_rule_group::_update_rule_group_input::UpdateRuleGroupInputBuilder;

impl crate::operation::update_rule_group::builders::UpdateRuleGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_rule_group::UpdateRuleGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_rule_group::UpdateRuleGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_rule_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateRuleGroup`.
///
/// <p>Updates the specified <code>RuleGroup</code>.</p><note>
/// <p>This operation completely replaces the mutable specifications that you already have for the rule group with the ones that you provide to this call.</p>
/// <p>To modify a rule group, do the following:</p>
/// <ol>
/// <li>
/// <p>Retrieve it by calling <code>GetRuleGroup</code></p></li>
/// <li>
/// <p>Update its settings as needed</p></li>
/// <li>
/// <p>Provide the complete rule group specification to this call</p></li>
/// </ol>
/// </note>
/// <p>A rule group defines a collection of rules to inspect and control web requests that you can use in a <code>WebACL</code>. When you create a rule group, you define an immutable capacity limit. If you update a rule group, you must stay within the capacity. This allows others to reuse the rule group with confidence in its capacity requirements.</p>
/// <p><b>Temporary inconsistencies during updates</b></p>
/// <p>When you create or change a web ACL or other WAF resources, the changes take a small amount of time to propagate to all areas where the resources are stored. The propagation time can be from a few seconds to a number of minutes.</p>
/// <p>The following are examples of the temporary inconsistencies that you might notice during change propagation:</p>
/// <ul>
/// <li>
/// <p>After you create a web ACL, if you try to associate it with a resource, you might get an exception indicating that the web ACL is unavailable.</p></li>
/// <li>
/// <p>After you add a rule group to a web ACL, the new rule group rules might be in effect in one area where the web ACL is used and not in another.</p></li>
/// <li>
/// <p>After you change a rule action setting, you might see the old action in some places and the new action in others.</p></li>
/// <li>
/// <p>After you add an IP address to an IP set that is in use in a blocking rule, the new address might be blocked in one area while still allowed in another.</p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateRuleGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_rule_group::builders::UpdateRuleGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_rule_group::UpdateRuleGroupOutput,
        crate::operation::update_rule_group::UpdateRuleGroupError,
    > for UpdateRuleGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_rule_group::UpdateRuleGroupOutput,
            crate::operation::update_rule_group::UpdateRuleGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateRuleGroupFluentBuilder {
    /// Creates a new `UpdateRuleGroupFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateRuleGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::update_rule_group::builders::UpdateRuleGroupInputBuilder {
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
        crate::operation::update_rule_group::UpdateRuleGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_rule_group::UpdateRuleGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_rule_group::UpdateRuleGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_rule_group::UpdateRuleGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_rule_group::UpdateRuleGroupOutput,
        crate::operation::update_rule_group::UpdateRuleGroupError,
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
    /// <p>The name of the rule group. You cannot change the name of a rule group after you create it.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the rule group. You cannot change the name of a rule group after you create it.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the rule group. You cannot change the name of a rule group after you create it.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
    /// <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows:</p>
    /// <ul>
    /// <li>
    /// <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>.</p></li>
    /// <li>
    /// <p>API and SDKs - For all calls, use the Region endpoint us-east-1.</p></li>
    /// </ul>
    pub fn scope(mut self, input: crate::types::Scope) -> Self {
        self.inner = self.inner.scope(input);
        self
    }
    /// <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
    /// <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows:</p>
    /// <ul>
    /// <li>
    /// <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>.</p></li>
    /// <li>
    /// <p>API and SDKs - For all calls, use the Region endpoint us-east-1.</p></li>
    /// </ul>
    pub fn set_scope(mut self, input: ::std::option::Option<crate::types::Scope>) -> Self {
        self.inner = self.inner.set_scope(input);
        self
    }
    /// <p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p>
    /// <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows:</p>
    /// <ul>
    /// <li>
    /// <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>.</p></li>
    /// <li>
    /// <p>API and SDKs - For all calls, use the Region endpoint us-east-1.</p></li>
    /// </ul>
    pub fn get_scope(&self) -> &::std::option::Option<crate::types::Scope> {
        self.inner.get_scope()
    }
    /// <p>A unique identifier for the rule group. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>A unique identifier for the rule group. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>A unique identifier for the rule group. This ID is returned in the responses to create and list commands. You provide it to operations like update and delete.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>A description of the rule group that helps with identification.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the rule group that helps with identification.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the rule group that helps with identification.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    ///
    /// Appends an item to `Rules`.
    ///
    /// To override the contents of this collection use [`set_rules`](Self::set_rules).
    ///
    /// <p>The <code>Rule</code> statements used to identify the web requests that you want to manage. Each rule includes one top-level statement that WAF uses to identify matching web requests, and parameters that govern how WAF handles them.</p>
    pub fn rules(mut self, input: crate::types::Rule) -> Self {
        self.inner = self.inner.rules(input);
        self
    }
    /// <p>The <code>Rule</code> statements used to identify the web requests that you want to manage. Each rule includes one top-level statement that WAF uses to identify matching web requests, and parameters that govern how WAF handles them.</p>
    pub fn set_rules(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Rule>>) -> Self {
        self.inner = self.inner.set_rules(input);
        self
    }
    /// <p>The <code>Rule</code> statements used to identify the web requests that you want to manage. Each rule includes one top-level statement that WAF uses to identify matching web requests, and parameters that govern how WAF handles them.</p>
    pub fn get_rules(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Rule>> {
        self.inner.get_rules()
    }
    /// <p>Defines and enables Amazon CloudWatch metrics and web request sample collection.</p>
    pub fn visibility_config(mut self, input: crate::types::VisibilityConfig) -> Self {
        self.inner = self.inner.visibility_config(input);
        self
    }
    /// <p>Defines and enables Amazon CloudWatch metrics and web request sample collection.</p>
    pub fn set_visibility_config(mut self, input: ::std::option::Option<crate::types::VisibilityConfig>) -> Self {
        self.inner = self.inner.set_visibility_config(input);
        self
    }
    /// <p>Defines and enables Amazon CloudWatch metrics and web request sample collection.</p>
    pub fn get_visibility_config(&self) -> &::std::option::Option<crate::types::VisibilityConfig> {
        self.inner.get_visibility_config()
    }
    /// <p>A token used for optimistic locking. WAF returns a token to your <code>get</code> and <code>list</code> requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like <code>update</code> and <code>delete</code>. WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another <code>get</code>, and use the new token returned by that operation.</p>
    pub fn lock_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.lock_token(input.into());
        self
    }
    /// <p>A token used for optimistic locking. WAF returns a token to your <code>get</code> and <code>list</code> requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like <code>update</code> and <code>delete</code>. WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another <code>get</code>, and use the new token returned by that operation.</p>
    pub fn set_lock_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_lock_token(input);
        self
    }
    /// <p>A token used for optimistic locking. WAF returns a token to your <code>get</code> and <code>list</code> requests, to mark the state of the entity at the time of the request. To make changes to the entity associated with the token, you provide the token to operations like <code>update</code> and <code>delete</code>. WAF uses the token to ensure that no changes have been made to the entity since you last retrieved it. If a change has been made, the update fails with a <code>WAFOptimisticLockException</code>. If this happens, perform another <code>get</code>, and use the new token returned by that operation.</p>
    pub fn get_lock_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_lock_token()
    }
    ///
    /// Adds a key-value pair to `CustomResponseBodies`.
    ///
    /// To override the contents of this collection use [`set_custom_response_bodies`](Self::set_custom_response_bodies).
    ///
    /// <p>A map of custom response keys and content bodies. When you create a rule with a block action, you can send a custom response to the web request. You define these for the rule group, and then use them in the rules that you define in the rule group.</p>
    /// <p>For information about customizing web requests and responses, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-custom-request-response.html">Customizing web requests and responses in WAF</a> in the <i>WAF Developer Guide</i>.</p>
    /// <p>For information about the limits on count and size for custom request and response settings, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">WAF quotas</a> in the <i>WAF Developer Guide</i>.</p>
    pub fn custom_response_bodies(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::CustomResponseBody) -> Self {
        self.inner = self.inner.custom_response_bodies(k.into(), v);
        self
    }
    /// <p>A map of custom response keys and content bodies. When you create a rule with a block action, you can send a custom response to the web request. You define these for the rule group, and then use them in the rules that you define in the rule group.</p>
    /// <p>For information about customizing web requests and responses, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-custom-request-response.html">Customizing web requests and responses in WAF</a> in the <i>WAF Developer Guide</i>.</p>
    /// <p>For information about the limits on count and size for custom request and response settings, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">WAF quotas</a> in the <i>WAF Developer Guide</i>.</p>
    pub fn set_custom_response_bodies(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::CustomResponseBody>>,
    ) -> Self {
        self.inner = self.inner.set_custom_response_bodies(input);
        self
    }
    /// <p>A map of custom response keys and content bodies. When you create a rule with a block action, you can send a custom response to the web request. You define these for the rule group, and then use them in the rules that you define in the rule group.</p>
    /// <p>For information about customizing web requests and responses, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-custom-request-response.html">Customizing web requests and responses in WAF</a> in the <i>WAF Developer Guide</i>.</p>
    /// <p>For information about the limits on count and size for custom request and response settings, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/limits.html">WAF quotas</a> in the <i>WAF Developer Guide</i>.</p>
    pub fn get_custom_response_bodies(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::CustomResponseBody>> {
        self.inner.get_custom_response_bodies()
    }
}
