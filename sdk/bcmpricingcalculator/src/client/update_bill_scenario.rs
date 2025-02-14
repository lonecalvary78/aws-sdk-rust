// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateBillScenario`](crate::operation::update_bill_scenario::builders::UpdateBillScenarioFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifier(impl Into<String>)`](crate::operation::update_bill_scenario::builders::UpdateBillScenarioFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::update_bill_scenario::builders::UpdateBillScenarioFluentBuilder::set_identifier):<br>required: **true**<br><p>The unique identifier of the bill scenario to update.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::update_bill_scenario::builders::UpdateBillScenarioFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_bill_scenario::builders::UpdateBillScenarioFluentBuilder::set_name):<br>required: **false**<br><p>The new name for the bill scenario.</p><br>
    ///   - [`expires_at(DateTime)`](crate::operation::update_bill_scenario::builders::UpdateBillScenarioFluentBuilder::expires_at) / [`set_expires_at(Option<DateTime>)`](crate::operation::update_bill_scenario::builders::UpdateBillScenarioFluentBuilder::set_expires_at):<br>required: **false**<br><p>The new expiration date for the bill scenario.</p><br>
    /// - On success, responds with [`UpdateBillScenarioOutput`](crate::operation::update_bill_scenario::UpdateBillScenarioOutput) with field(s):
    ///   - [`id(String)`](crate::operation::update_bill_scenario::UpdateBillScenarioOutput::id): <p>The unique identifier of the updated bill scenario.</p>
    ///   - [`name(Option<String>)`](crate::operation::update_bill_scenario::UpdateBillScenarioOutput::name): <p>The updated name of the bill scenario.</p>
    ///   - [`bill_interval(Option<BillInterval>)`](crate::operation::update_bill_scenario::UpdateBillScenarioOutput::bill_interval): <p>The time period covered by the updated bill scenario.</p>
    ///   - [`status(Option<BillScenarioStatus>)`](crate::operation::update_bill_scenario::UpdateBillScenarioOutput::status): <p>The current status of the updated bill scenario.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::update_bill_scenario::UpdateBillScenarioOutput::created_at): <p>The timestamp when the bill scenario was originally created.</p>
    ///   - [`expires_at(Option<DateTime>)`](crate::operation::update_bill_scenario::UpdateBillScenarioOutput::expires_at): <p>The updated expiration timestamp for the bill scenario.</p>
    ///   - [`failure_message(Option<String>)`](crate::operation::update_bill_scenario::UpdateBillScenarioOutput::failure_message): <p>An error message if the bill scenario update failed.</p>
    /// - On failure, responds with [`SdkError<UpdateBillScenarioError>`](crate::operation::update_bill_scenario::UpdateBillScenarioError)
    pub fn update_bill_scenario(&self) -> crate::operation::update_bill_scenario::builders::UpdateBillScenarioFluentBuilder {
        crate::operation::update_bill_scenario::builders::UpdateBillScenarioFluentBuilder::new(self.handle.clone())
    }
}
