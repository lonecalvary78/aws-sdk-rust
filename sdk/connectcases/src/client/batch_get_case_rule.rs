// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetCaseRule`](crate::operation::batch_get_case_rule::builders::BatchGetCaseRuleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_id(impl Into<String>)`](crate::operation::batch_get_case_rule::builders::BatchGetCaseRuleFluentBuilder::domain_id) / [`set_domain_id(Option<String>)`](crate::operation::batch_get_case_rule::builders::BatchGetCaseRuleFluentBuilder::set_domain_id):<br>required: **true**<br><p>Unique identifier of a Cases domain.</p><br>
    ///   - [`case_rules(CaseRuleIdentifier)`](crate::operation::batch_get_case_rule::builders::BatchGetCaseRuleFluentBuilder::case_rules) / [`set_case_rules(Option<Vec::<CaseRuleIdentifier>>)`](crate::operation::batch_get_case_rule::builders::BatchGetCaseRuleFluentBuilder::set_case_rules):<br>required: **true**<br><p>List of case rule identifiers.</p><br>
    /// - On success, responds with [`BatchGetCaseRuleOutput`](crate::operation::batch_get_case_rule::BatchGetCaseRuleOutput) with field(s):
    ///   - [`case_rules(Vec::<GetCaseRuleResponse>)`](crate::operation::batch_get_case_rule::BatchGetCaseRuleOutput::case_rules): <p>List of detailed case rule information.</p>
    ///   - [`errors(Vec::<CaseRuleError>)`](crate::operation::batch_get_case_rule::BatchGetCaseRuleOutput::errors): <p>List of case rule errors.</p>
    /// - On failure, responds with [`SdkError<BatchGetCaseRuleError>`](crate::operation::batch_get_case_rule::BatchGetCaseRuleError)
    pub fn batch_get_case_rule(&self) -> crate::operation::batch_get_case_rule::builders::BatchGetCaseRuleFluentBuilder {
        crate::operation::batch_get_case_rule::builders::BatchGetCaseRuleFluentBuilder::new(self.handle.clone())
    }
}
