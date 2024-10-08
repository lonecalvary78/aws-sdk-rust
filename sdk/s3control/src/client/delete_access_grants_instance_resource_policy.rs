// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAccessGrantsInstanceResourcePolicy`](crate::operation::delete_access_grants_instance_resource_policy::builders::DeleteAccessGrantsInstanceResourcePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::delete_access_grants_instance_resource_policy::builders::DeleteAccessGrantsInstanceResourcePolicyFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::delete_access_grants_instance_resource_policy::builders::DeleteAccessGrantsInstanceResourcePolicyFluentBuilder::set_account_id):<br>required: **true**<br><p>The Amazon Web Services account ID of the S3 Access Grants instance.</p><br>
    /// - On success, responds with [`DeleteAccessGrantsInstanceResourcePolicyOutput`](crate::operation::delete_access_grants_instance_resource_policy::DeleteAccessGrantsInstanceResourcePolicyOutput)
    /// - On failure, responds with [`SdkError<DeleteAccessGrantsInstanceResourcePolicyError>`](crate::operation::delete_access_grants_instance_resource_policy::DeleteAccessGrantsInstanceResourcePolicyError)
    pub fn delete_access_grants_instance_resource_policy(
        &self,
    ) -> crate::operation::delete_access_grants_instance_resource_policy::builders::DeleteAccessGrantsInstanceResourcePolicyFluentBuilder {
        crate::operation::delete_access_grants_instance_resource_policy::builders::DeleteAccessGrantsInstanceResourcePolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
