// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTaxExemptions`](crate::operation::list_tax_exemptions::builders::ListTaxExemptionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_tax_exemptions::builders::ListTaxExemptionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_tax_exemptions::builders::ListTaxExemptionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_tax_exemptions::builders::ListTaxExemptionsFluentBuilder::set_max_results):<br>required: **false**<br><p>The number of results you want in one response.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_tax_exemptions::builders::ListTaxExemptionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_tax_exemptions::builders::ListTaxExemptionsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token to retrieve the next set of results.</p><br>
    /// - On success, responds with [`ListTaxExemptionsOutput`](crate::operation::list_tax_exemptions::ListTaxExemptionsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_tax_exemptions::ListTaxExemptionsOutput::next_token): <p>The token to retrieve the next set of results.</p>
    ///   - [`tax_exemption_details_map(Option<HashMap::<String, TaxExemptionDetails>>)`](crate::operation::list_tax_exemptions::ListTaxExemptionsOutput::tax_exemption_details_map): <p>The tax exemption details map of <code>accountId</code> and tax exemption details.</p>
    /// - On failure, responds with [`SdkError<ListTaxExemptionsError>`](crate::operation::list_tax_exemptions::ListTaxExemptionsError)
    pub fn list_tax_exemptions(&self) -> crate::operation::list_tax_exemptions::builders::ListTaxExemptionsFluentBuilder {
        crate::operation::list_tax_exemptions::builders::ListTaxExemptionsFluentBuilder::new(self.handle.clone())
    }
}
