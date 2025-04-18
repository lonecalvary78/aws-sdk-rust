// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetPolicyStore`](crate::operation::get_policy_store::builders::GetPolicyStoreFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`policy_store_id(impl Into<String>)`](crate::operation::get_policy_store::builders::GetPolicyStoreFluentBuilder::policy_store_id) / [`set_policy_store_id(Option<String>)`](crate::operation::get_policy_store::builders::GetPolicyStoreFluentBuilder::set_policy_store_id):<br>required: **true**<br><p>Specifies the ID of the policy store that you want information about.</p><br>
    /// - On success, responds with [`GetPolicyStoreOutput`](crate::operation::get_policy_store::GetPolicyStoreOutput) with field(s):
    ///   - [`policy_store_id(String)`](crate::operation::get_policy_store::GetPolicyStoreOutput::policy_store_id): <p>The ID of the policy store;</p>
    ///   - [`arn(String)`](crate::operation::get_policy_store::GetPolicyStoreOutput::arn): <p>The Amazon Resource Name (ARN) of the policy store.</p>
    ///   - [`validation_settings(Option<ValidationSettings>)`](crate::operation::get_policy_store::GetPolicyStoreOutput::validation_settings): <p>The current validation settings for the policy store.</p>
    ///   - [`created_date(DateTime)`](crate::operation::get_policy_store::GetPolicyStoreOutput::created_date): <p>The date and time that the policy store was originally created.</p>
    ///   - [`last_updated_date(DateTime)`](crate::operation::get_policy_store::GetPolicyStoreOutput::last_updated_date): <p>The date and time that the policy store was last updated.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_policy_store::GetPolicyStoreOutput::description): <p>Descriptive text that you can provide to help with identification of the current policy store.</p>
    ///   - [`deletion_protection(Option<DeletionProtection>)`](crate::operation::get_policy_store::GetPolicyStoreOutput::deletion_protection): <p>Specifies whether the policy store can be deleted. If enabled, the policy store can't be deleted.</p> <p>The default state is <code>DISABLED</code>.</p>
    /// - On failure, responds with [`SdkError<GetPolicyStoreError>`](crate::operation::get_policy_store::GetPolicyStoreError)
    pub fn get_policy_store(&self) -> crate::operation::get_policy_store::builders::GetPolicyStoreFluentBuilder {
        crate::operation::get_policy_store::builders::GetPolicyStoreFluentBuilder::new(self.handle.clone())
    }
}
