// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAccessGrantsInstanceResourcePolicy`](crate::operation::get_access_grants_instance_resource_policy::builders::GetAccessGrantsInstanceResourcePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::get_access_grants_instance_resource_policy::builders::GetAccessGrantsInstanceResourcePolicyFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::get_access_grants_instance_resource_policy::builders::GetAccessGrantsInstanceResourcePolicyFluentBuilder::set_account_id):<br>required: **true**<br><p>The Amazon Web Services account ID of the S3 Access Grants instance.</p><br>
    /// - On success, responds with [`GetAccessGrantsInstanceResourcePolicyOutput`](crate::operation::get_access_grants_instance_resource_policy::GetAccessGrantsInstanceResourcePolicyOutput) with field(s):
    ///   - [`policy(Option<String>)`](crate::operation::get_access_grants_instance_resource_policy::GetAccessGrantsInstanceResourcePolicyOutput::policy): <p>The resource policy of the S3 Access Grants instance.</p>
    ///   - [`organization(Option<String>)`](crate::operation::get_access_grants_instance_resource_policy::GetAccessGrantsInstanceResourcePolicyOutput::organization): <p>The Organization of the resource policy of the S3 Access Grants instance.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::get_access_grants_instance_resource_policy::GetAccessGrantsInstanceResourcePolicyOutput::created_at): <p>The date and time when you created the S3 Access Grants instance resource policy.</p>
    /// - On failure, responds with [`SdkError<GetAccessGrantsInstanceResourcePolicyError>`](crate::operation::get_access_grants_instance_resource_policy::GetAccessGrantsInstanceResourcePolicyError)
    pub fn get_access_grants_instance_resource_policy(
        &self,
    ) -> crate::operation::get_access_grants_instance_resource_policy::builders::GetAccessGrantsInstanceResourcePolicyFluentBuilder {
        crate::operation::get_access_grants_instance_resource_policy::builders::GetAccessGrantsInstanceResourcePolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
