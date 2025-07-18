// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetOutpostBillingInformation`](crate::operation::get_outpost_billing_information::builders::GetOutpostBillingInformationFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_outpost_billing_information::builders::GetOutpostBillingInformationFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_outpost_billing_information::builders::GetOutpostBillingInformationFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_outpost_billing_information::builders::GetOutpostBillingInformationFluentBuilder::set_next_token):<br>required: **false**<br><p>The pagination token.</p><br>
    ///   - [`max_results(i32)`](crate::operation::get_outpost_billing_information::builders::GetOutpostBillingInformationFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_outpost_billing_information::builders::GetOutpostBillingInformationFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum page size.</p><br>
    ///   - [`outpost_identifier(impl Into<String>)`](crate::operation::get_outpost_billing_information::builders::GetOutpostBillingInformationFluentBuilder::outpost_identifier) / [`set_outpost_identifier(Option<String>)`](crate::operation::get_outpost_billing_information::builders::GetOutpostBillingInformationFluentBuilder::set_outpost_identifier):<br>required: **true**<br><p>The ID or ARN of the Outpost.</p><br>
    /// - On success, responds with [`GetOutpostBillingInformationOutput`](crate::operation::get_outpost_billing_information::GetOutpostBillingInformationOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::get_outpost_billing_information::GetOutpostBillingInformationOutput::next_token): <p>The pagination token.</p>
    ///   - [`subscriptions(Option<Vec::<Subscription>>)`](crate::operation::get_outpost_billing_information::GetOutpostBillingInformationOutput::subscriptions): <p>The subscription details for the specified Outpost.</p>
    ///   - [`contract_end_date(Option<String>)`](crate::operation::get_outpost_billing_information::GetOutpostBillingInformationOutput::contract_end_date): <p>The date the current contract term ends for the specified Outpost. You must start the renewal or decommission process at least 5 business days before the current term for your Amazon Web Services Outposts ends. Failing to complete these steps at least 5 business days before the current term ends might result in unanticipated charges.</p>
    /// - On failure, responds with [`SdkError<GetOutpostBillingInformationError>`](crate::operation::get_outpost_billing_information::GetOutpostBillingInformationError)
    pub fn get_outpost_billing_information(
        &self,
    ) -> crate::operation::get_outpost_billing_information::builders::GetOutpostBillingInformationFluentBuilder {
        crate::operation::get_outpost_billing_information::builders::GetOutpostBillingInformationFluentBuilder::new(self.handle.clone())
    }
}
