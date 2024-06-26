// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateWorkspace`](crate::operation::create_workspace::builders::CreateWorkspaceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`alias(impl Into<String>)`](crate::operation::create_workspace::builders::CreateWorkspaceFluentBuilder::alias) / [`set_alias(Option<String>)`](crate::operation::create_workspace::builders::CreateWorkspaceFluentBuilder::set_alias):<br>required: **false**<br><p>An alias that you assign to this workspace to help you identify it. It does not need to be unique.</p> <p>Blank spaces at the beginning or end of the alias that you specify will be trimmed from the value used.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_workspace::builders::CreateWorkspaceFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_workspace::builders::CreateWorkspaceFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique identifier that you can provide to ensure the idempotency of the request. Case-sensitive.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_workspace::builders::CreateWorkspaceFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_workspace::builders::CreateWorkspaceFluentBuilder::set_tags):<br>required: **false**<br><p>The list of tag keys and values to associate with the workspace.</p><br>
    ///   - [`kms_key_arn(impl Into<String>)`](crate::operation::create_workspace::builders::CreateWorkspaceFluentBuilder::kms_key_arn) / [`set_kms_key_arn(Option<String>)`](crate::operation::create_workspace::builders::CreateWorkspaceFluentBuilder::set_kms_key_arn):<br>required: **false**<br><p>(optional) The ARN for a customer managed KMS key to use for encrypting data within your workspace. For more information about using your own key in your workspace, see <a href="https://docs.aws.amazon.com/prometheus/latest/userguide/encryption-at-rest-Amazon-Service-Prometheus.html">Encryption at rest</a> in the <i>Amazon Managed Service for Prometheus User Guide</i>.</p><br>
    /// - On success, responds with [`CreateWorkspaceOutput`](crate::operation::create_workspace::CreateWorkspaceOutput) with field(s):
    ///   - [`workspace_id(String)`](crate::operation::create_workspace::CreateWorkspaceOutput::workspace_id): <p>The unique ID for the new workspace.</p>
    ///   - [`arn(String)`](crate::operation::create_workspace::CreateWorkspaceOutput::arn): <p>The ARN for the new workspace.</p>
    ///   - [`status(Option<WorkspaceStatus>)`](crate::operation::create_workspace::CreateWorkspaceOutput::status): <p>The current status of the new workspace. Immediately after you create the workspace, the status is usually <code>CREATING</code>.</p>
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::create_workspace::CreateWorkspaceOutput::tags): <p>The list of tag keys and values that are associated with the workspace.</p>
    ///   - [`kms_key_arn(Option<String>)`](crate::operation::create_workspace::CreateWorkspaceOutput::kms_key_arn): <p>(optional) If the workspace was created with a customer managed KMS key, the ARN for the key used.</p>
    /// - On failure, responds with [`SdkError<CreateWorkspaceError>`](crate::operation::create_workspace::CreateWorkspaceError)
    pub fn create_workspace(&self) -> crate::operation::create_workspace::builders::CreateWorkspaceFluentBuilder {
        crate::operation::create_workspace::builders::CreateWorkspaceFluentBuilder::new(self.handle.clone())
    }
}
