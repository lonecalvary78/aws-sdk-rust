// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdatePrompt`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::set_name):<br>required: **true**<br><p>A name for the prompt.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::set_description):<br>required: **false**<br><p>A description for the prompt.</p><br>
    ///   - [`customer_encryption_key_arn(impl Into<String>)`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::customer_encryption_key_arn) / [`set_customer_encryption_key_arn(Option<String>)`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::set_customer_encryption_key_arn):<br>required: **false**<br><p>The Amazon Resource Name (ARN) of the KMS key to encrypt the prompt.</p><br>
    ///   - [`default_variant(impl Into<String>)`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::default_variant) / [`set_default_variant(Option<String>)`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::set_default_variant):<br>required: **false**<br><p>The name of the default variant for the prompt. This value must match the <code>name</code> field in the relevant <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent_PromptVariant.html">PromptVariant</a> object.</p><br>
    ///   - [`variants(PromptVariant)`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::variants) / [`set_variants(Option<Vec::<PromptVariant>>)`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::set_variants):<br>required: **false**<br><p>A list of objects, each containing details about a variant of the prompt.</p><br>
    ///   - [`prompt_identifier(impl Into<String>)`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::prompt_identifier) / [`set_prompt_identifier(Option<String>)`](crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::set_prompt_identifier):<br>required: **true**<br><p>The unique identifier of the prompt.</p><br>
    /// - On success, responds with [`UpdatePromptOutput`](crate::operation::update_prompt::UpdatePromptOutput) with field(s):
    ///   - [`name(String)`](crate::operation::update_prompt::UpdatePromptOutput::name): <p>The name of the prompt.</p>
    ///   - [`description(Option<String>)`](crate::operation::update_prompt::UpdatePromptOutput::description): <p>The description of the prompt.</p>
    ///   - [`customer_encryption_key_arn(Option<String>)`](crate::operation::update_prompt::UpdatePromptOutput::customer_encryption_key_arn): <p>The Amazon Resource Name (ARN) of the KMS key to encrypt the prompt.</p>
    ///   - [`default_variant(Option<String>)`](crate::operation::update_prompt::UpdatePromptOutput::default_variant): <p>The name of the default variant for the prompt. This value must match the <code>name</code> field in the relevant <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent_PromptVariant.html">PromptVariant</a> object.</p>
    ///   - [`variants(Option<Vec::<PromptVariant>>)`](crate::operation::update_prompt::UpdatePromptOutput::variants): <p>A list of objects, each containing details about a variant of the prompt.</p>
    ///   - [`id(String)`](crate::operation::update_prompt::UpdatePromptOutput::id): <p>The unique identifier of the prompt.</p>
    ///   - [`arn(String)`](crate::operation::update_prompt::UpdatePromptOutput::arn): <p>The Amazon Resource Name (ARN) of the prompt.</p>
    ///   - [`version(String)`](crate::operation::update_prompt::UpdatePromptOutput::version): <p>The version of the prompt. When you update a prompt, the version updated is the <code>DRAFT</code> version.</p>
    ///   - [`created_at(DateTime)`](crate::operation::update_prompt::UpdatePromptOutput::created_at): <p>The time at which the prompt was created.</p>
    ///   - [`updated_at(DateTime)`](crate::operation::update_prompt::UpdatePromptOutput::updated_at): <p>The time at which the prompt was last updated.</p>
    /// - On failure, responds with [`SdkError<UpdatePromptError>`](crate::operation::update_prompt::UpdatePromptError)
    pub fn update_prompt(&self) -> crate::operation::update_prompt::builders::UpdatePromptFluentBuilder {
        crate::operation::update_prompt::builders::UpdatePromptFluentBuilder::new(self.handle.clone())
    }
}
