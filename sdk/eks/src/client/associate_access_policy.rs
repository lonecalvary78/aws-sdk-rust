// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateAccessPolicy`](crate::operation::associate_access_policy::builders::AssociateAccessPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_name(impl Into<String>)`](crate::operation::associate_access_policy::builders::AssociateAccessPolicyFluentBuilder::cluster_name) / [`set_cluster_name(Option<String>)`](crate::operation::associate_access_policy::builders::AssociateAccessPolicyFluentBuilder::set_cluster_name):<br>required: **true**<br><p>The name of your cluster.</p><br>
    ///   - [`principal_arn(impl Into<String>)`](crate::operation::associate_access_policy::builders::AssociateAccessPolicyFluentBuilder::principal_arn) / [`set_principal_arn(Option<String>)`](crate::operation::associate_access_policy::builders::AssociateAccessPolicyFluentBuilder::set_principal_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the IAM user or role for the <code>AccessEntry</code> that you're associating the access policy to.</p><br>
    ///   - [`policy_arn(impl Into<String>)`](crate::operation::associate_access_policy::builders::AssociateAccessPolicyFluentBuilder::policy_arn) / [`set_policy_arn(Option<String>)`](crate::operation::associate_access_policy::builders::AssociateAccessPolicyFluentBuilder::set_policy_arn):<br>required: **true**<br><p>The ARN of the <code>AccessPolicy</code> that you're associating. For a list of ARNs, use <code>ListAccessPolicies</code>.</p><br>
    ///   - [`access_scope(AccessScope)`](crate::operation::associate_access_policy::builders::AssociateAccessPolicyFluentBuilder::access_scope) / [`set_access_scope(Option<AccessScope>)`](crate::operation::associate_access_policy::builders::AssociateAccessPolicyFluentBuilder::set_access_scope):<br>required: **true**<br><p>The scope for the <code>AccessPolicy</code>. You can scope access policies to an entire cluster or to specific Kubernetes namespaces.</p><br>
    /// - On success, responds with [`AssociateAccessPolicyOutput`](crate::operation::associate_access_policy::AssociateAccessPolicyOutput) with field(s):
    ///   - [`cluster_name(Option<String>)`](crate::operation::associate_access_policy::AssociateAccessPolicyOutput::cluster_name): <p>The name of your cluster.</p>
    ///   - [`principal_arn(Option<String>)`](crate::operation::associate_access_policy::AssociateAccessPolicyOutput::principal_arn): <p>The ARN of the IAM principal for the <code>AccessEntry</code>.</p>
    ///   - [`associated_access_policy(Option<AssociatedAccessPolicy>)`](crate::operation::associate_access_policy::AssociateAccessPolicyOutput::associated_access_policy): <p>The <code>AccessPolicy</code> and scope associated to the <code>AccessEntry</code>.</p>
    /// - On failure, responds with [`SdkError<AssociateAccessPolicyError>`](crate::operation::associate_access_policy::AssociateAccessPolicyError)
    pub fn associate_access_policy(&self) -> crate::operation::associate_access_policy::builders::AssociateAccessPolicyFluentBuilder {
        crate::operation::associate_access_policy::builders::AssociateAccessPolicyFluentBuilder::new(self.handle.clone())
    }
}
