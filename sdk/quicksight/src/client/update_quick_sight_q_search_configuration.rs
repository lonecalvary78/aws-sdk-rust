// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateQuickSightQSearchConfiguration`](crate::operation::update_quick_sight_q_search_configuration::builders::UpdateQuickSightQSearchConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::operation::update_quick_sight_q_search_configuration::builders::UpdateQuickSightQSearchConfigurationFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::update_quick_sight_q_search_configuration::builders::UpdateQuickSightQSearchConfigurationFluentBuilder::set_aws_account_id):<br>required: **true**<br><p>The ID of the Amazon Web Services account that contains the Amazon QuickSight Q Search configuration that you want to update.</p><br>
    ///   - [`q_search_status(QSearchStatus)`](crate::operation::update_quick_sight_q_search_configuration::builders::UpdateQuickSightQSearchConfigurationFluentBuilder::q_search_status) / [`set_q_search_status(Option<QSearchStatus>)`](crate::operation::update_quick_sight_q_search_configuration::builders::UpdateQuickSightQSearchConfigurationFluentBuilder::set_q_search_status):<br>required: **true**<br><p>The status of the Amazon QuickSight Q Search configuration that the user wants to update.</p><br>
    /// - On success, responds with [`UpdateQuickSightQSearchConfigurationOutput`](crate::operation::update_quick_sight_q_search_configuration::UpdateQuickSightQSearchConfigurationOutput) with field(s):
    ///   - [`q_search_status(Option<QSearchStatus>)`](crate::operation::update_quick_sight_q_search_configuration::UpdateQuickSightQSearchConfigurationOutput::q_search_status): <p>The status of the Amazon QuickSight Q Search configuration.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::update_quick_sight_q_search_configuration::UpdateQuickSightQSearchConfigurationOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::update_quick_sight_q_search_configuration::UpdateQuickSightQSearchConfigurationOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<UpdateQuickSightQSearchConfigurationError>`](crate::operation::update_quick_sight_q_search_configuration::UpdateQuickSightQSearchConfigurationError)
    pub fn update_quick_sight_q_search_configuration(
        &self,
    ) -> crate::operation::update_quick_sight_q_search_configuration::builders::UpdateQuickSightQSearchConfigurationFluentBuilder {
        crate::operation::update_quick_sight_q_search_configuration::builders::UpdateQuickSightQSearchConfigurationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
