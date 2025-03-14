// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeletePartnerApp`](crate::operation::delete_partner_app::builders::DeletePartnerAppFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::delete_partner_app::builders::DeletePartnerAppFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::delete_partner_app::builders::DeletePartnerAppFluentBuilder::set_arn):<br>required: **true**<br><p>The ARN of the SageMaker Partner AI App to delete.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::delete_partner_app::builders::DeletePartnerAppFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::delete_partner_app::builders::DeletePartnerAppFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique token that guarantees that the call to this API is idempotent.</p><br>
    /// - On success, responds with [`DeletePartnerAppOutput`](crate::operation::delete_partner_app::DeletePartnerAppOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::delete_partner_app::DeletePartnerAppOutput::arn): <p>The ARN of the SageMaker Partner AI App that was deleted.</p>
    /// - On failure, responds with [`SdkError<DeletePartnerAppError>`](crate::operation::delete_partner_app::DeletePartnerAppError)
    pub fn delete_partner_app(&self) -> crate::operation::delete_partner_app::builders::DeletePartnerAppFluentBuilder {
        crate::operation::delete_partner_app::builders::DeletePartnerAppFluentBuilder::new(self.handle.clone())
    }
}
