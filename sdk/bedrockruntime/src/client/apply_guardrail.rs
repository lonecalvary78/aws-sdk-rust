// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ApplyGuardrail`](crate::operation::apply_guardrail::builders::ApplyGuardrailFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`guardrail_identifier(impl Into<String>)`](crate::operation::apply_guardrail::builders::ApplyGuardrailFluentBuilder::guardrail_identifier) / [`set_guardrail_identifier(Option<String>)`](crate::operation::apply_guardrail::builders::ApplyGuardrailFluentBuilder::set_guardrail_identifier):<br>required: **true**<br><p>The guardrail identifier used in the request to apply the guardrail.</p><br>
    ///   - [`guardrail_version(impl Into<String>)`](crate::operation::apply_guardrail::builders::ApplyGuardrailFluentBuilder::guardrail_version) / [`set_guardrail_version(Option<String>)`](crate::operation::apply_guardrail::builders::ApplyGuardrailFluentBuilder::set_guardrail_version):<br>required: **true**<br><p>The guardrail version used in the request to apply the guardrail.</p><br>
    ///   - [`source(GuardrailContentSource)`](crate::operation::apply_guardrail::builders::ApplyGuardrailFluentBuilder::source) / [`set_source(Option<GuardrailContentSource>)`](crate::operation::apply_guardrail::builders::ApplyGuardrailFluentBuilder::set_source):<br>required: **true**<br><p>The source of data used in the request to apply the guardrail.</p><br>
    ///   - [`content(GuardrailContentBlock)`](crate::operation::apply_guardrail::builders::ApplyGuardrailFluentBuilder::content) / [`set_content(Option<Vec::<GuardrailContentBlock>>)`](crate::operation::apply_guardrail::builders::ApplyGuardrailFluentBuilder::set_content):<br>required: **true**<br><p>The content details used in the request to apply the guardrail.</p><br>
    /// - On success, responds with [`ApplyGuardrailOutput`](crate::operation::apply_guardrail::ApplyGuardrailOutput) with field(s):
    ///   - [`usage(Option<GuardrailUsage>)`](crate::operation::apply_guardrail::ApplyGuardrailOutput::usage): <p>The usage details in the response from the guardrail.</p>
    ///   - [`action(GuardrailAction)`](crate::operation::apply_guardrail::ApplyGuardrailOutput::action): <p>The action taken in the response from the guardrail.</p>
    ///   - [`outputs(Vec::<GuardrailOutputContent>)`](crate::operation::apply_guardrail::ApplyGuardrailOutput::outputs): <p>The output details in the response from the guardrail.</p>
    ///   - [`assessments(Vec::<GuardrailAssessment>)`](crate::operation::apply_guardrail::ApplyGuardrailOutput::assessments): <p>The assessment details in the response from the guardrail.</p>
    ///   - [`guardrail_coverage(Option<GuardrailCoverage>)`](crate::operation::apply_guardrail::ApplyGuardrailOutput::guardrail_coverage): <p>The guardrail coverage details in the apply guardrail response.</p>
    /// - On failure, responds with [`SdkError<ApplyGuardrailError>`](crate::operation::apply_guardrail::ApplyGuardrailError)
    pub fn apply_guardrail(&self) -> crate::operation::apply_guardrail::builders::ApplyGuardrailFluentBuilder {
        crate::operation::apply_guardrail::builders::ApplyGuardrailFluentBuilder::new(self.handle.clone())
    }
}
