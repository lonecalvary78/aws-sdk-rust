// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreatePromptVersion`](crate::operation::create_prompt_version::builders::CreatePromptVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`prompt_identifier(impl Into<String>)`](crate::operation::create_prompt_version::builders::CreatePromptVersionFluentBuilder::prompt_identifier) / [`set_prompt_identifier(Option<String>)`](crate::operation::create_prompt_version::builders::CreatePromptVersionFluentBuilder::set_prompt_identifier):<br>required: **true**<br><p>The unique identifier of the prompt that you want to create a version of.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_prompt_version::builders::CreatePromptVersionFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_prompt_version::builders::CreatePromptVersionFluentBuilder::set_description):<br>required: **false**<br><p>A description for the version of the prompt.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_prompt_version::builders::CreatePromptVersionFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_prompt_version::builders::CreatePromptVersionFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive identifier to ensure that the API request completes no more than one time. If this token matches a previous request, Amazon Bedrock ignores the request, but does not return an error. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_prompt_version::builders::CreatePromptVersionFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_prompt_version::builders::CreatePromptVersionFluentBuilder::set_tags):<br>required: **false**<br><p>Any tags that you want to attach to the version of the prompt. For more information, see <a href="https://docs.aws.amazon.com/bedrock/latest/userguide/tagging.html">Tagging resources in Amazon Bedrock</a>.</p><br>
    /// - On success, responds with [`CreatePromptVersionOutput`](crate::operation::create_prompt_version::CreatePromptVersionOutput) with field(s):
    ///   - [`name(String)`](crate::operation::create_prompt_version::CreatePromptVersionOutput::name): <p>The name of the prompt.</p>
    ///   - [`description(Option<String>)`](crate::operation::create_prompt_version::CreatePromptVersionOutput::description): <p>A description for the version.</p>
    ///   - [`customer_encryption_key_arn(Option<String>)`](crate::operation::create_prompt_version::CreatePromptVersionOutput::customer_encryption_key_arn): <p>The Amazon Resource Name (ARN) of the KMS key to encrypt the version of the prompt.</p>
    ///   - [`default_variant(Option<String>)`](crate::operation::create_prompt_version::CreatePromptVersionOutput::default_variant): <p>The name of the default variant for the prompt. This value must match the <code>name</code> field in the relevant <a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent_PromptVariant.html">PromptVariant</a> object.</p>
    ///   - [`variants(Option<Vec::<PromptVariant>>)`](crate::operation::create_prompt_version::CreatePromptVersionOutput::variants): <p>A list of objects, each containing details about a variant of the prompt.</p>
    ///   - [`id(String)`](crate::operation::create_prompt_version::CreatePromptVersionOutput::id): <p>The unique identifier of the prompt.</p>
    ///   - [`arn(String)`](crate::operation::create_prompt_version::CreatePromptVersionOutput::arn): <p>The Amazon Resource Name (ARN) of the version of the prompt.</p>
    ///   - [`version(String)`](crate::operation::create_prompt_version::CreatePromptVersionOutput::version): <p>The version of the prompt that was created. Versions are numbered incrementally, starting from 1.</p>
    ///   - [`created_at(DateTime)`](crate::operation::create_prompt_version::CreatePromptVersionOutput::created_at): <p>The time at which the prompt was created.</p>
    ///   - [`updated_at(DateTime)`](crate::operation::create_prompt_version::CreatePromptVersionOutput::updated_at): <p>The time at which the prompt was last updated.</p>
    /// - On failure, responds with [`SdkError<CreatePromptVersionError>`](crate::operation::create_prompt_version::CreatePromptVersionError)
    pub fn create_prompt_version(&self) -> crate::operation::create_prompt_version::builders::CreatePromptVersionFluentBuilder {
        crate::operation::create_prompt_version::builders::CreatePromptVersionFluentBuilder::new(self.handle.clone())
    }
}
