// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelTagSyncTask`](crate::operation::cancel_tag_sync_task::builders::CancelTagSyncTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`task_arn(impl Into<String>)`](crate::operation::cancel_tag_sync_task::builders::CancelTagSyncTaskFluentBuilder::task_arn) / [`set_task_arn(Option<String>)`](crate::operation::cancel_tag_sync_task::builders::CancelTagSyncTaskFluentBuilder::set_task_arn):<br>required: **true**<br><p>The Amazon resource name (ARN) of the tag-sync task.</p><br>
    /// - On success, responds with [`CancelTagSyncTaskOutput`](crate::operation::cancel_tag_sync_task::CancelTagSyncTaskOutput)
    /// - On failure, responds with [`SdkError<CancelTagSyncTaskError>`](crate::operation::cancel_tag_sync_task::CancelTagSyncTaskError)
    pub fn cancel_tag_sync_task(&self) -> crate::operation::cancel_tag_sync_task::builders::CancelTagSyncTaskFluentBuilder {
        crate::operation::cancel_tag_sync_task::builders::CancelTagSyncTaskFluentBuilder::new(self.handle.clone())
    }
}
