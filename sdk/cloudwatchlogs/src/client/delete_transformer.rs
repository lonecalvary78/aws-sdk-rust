// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteTransformer`](crate::operation::delete_transformer::builders::DeleteTransformerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`log_group_identifier(impl Into<String>)`](crate::operation::delete_transformer::builders::DeleteTransformerFluentBuilder::log_group_identifier) / [`set_log_group_identifier(Option<String>)`](crate::operation::delete_transformer::builders::DeleteTransformerFluentBuilder::set_log_group_identifier):<br>required: **true**<br><p>Specify either the name or ARN of the log group to delete the transformer for. If the log group is in a source account and you are using a monitoring account, you must use the log group ARN.</p><br>
    /// - On success, responds with [`DeleteTransformerOutput`](crate::operation::delete_transformer::DeleteTransformerOutput)
    /// - On failure, responds with [`SdkError<DeleteTransformerError>`](crate::operation::delete_transformer::DeleteTransformerError)
    pub fn delete_transformer(&self) -> crate::operation::delete_transformer::builders::DeleteTransformerFluentBuilder {
        crate::operation::delete_transformer::builders::DeleteTransformerFluentBuilder::new(self.handle.clone())
    }
}
