// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetResourcePolicy`](crate::operation::get_resource_policy::builders::GetResourcePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::get_resource_policy::builders::GetResourcePolicyFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::get_resource_policy::builders::GetResourcePolicyFluentBuilder::set_name):<br>required: **true**<br><p>The name of the app monitor that is associated with the resource-based policy that you want to view.</p><br>
    /// - On success, responds with [`GetResourcePolicyOutput`](crate::operation::get_resource_policy::GetResourcePolicyOutput) with field(s):
    ///   - [`policy_document(Option<String>)`](crate::operation::get_resource_policy::GetResourcePolicyOutput::policy_document): <p>The JSON policy document that you requested.</p>
    ///   - [`policy_revision_id(Option<String>)`](crate::operation::get_resource_policy::GetResourcePolicyOutput::policy_revision_id): <p>The revision ID information for this version of the policy document that you requested.</p>
    /// - On failure, responds with [`SdkError<GetResourcePolicyError>`](crate::operation::get_resource_policy::GetResourcePolicyError)
    pub fn get_resource_policy(&self) -> crate::operation::get_resource_policy::builders::GetResourcePolicyFluentBuilder {
        crate::operation::get_resource_policy::builders::GetResourcePolicyFluentBuilder::new(self.handle.clone())
    }
}
