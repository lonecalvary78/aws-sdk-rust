// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CompleteAttachedFileUpload`](crate::operation::complete_attached_file_upload::builders::CompleteAttachedFileUploadFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::operation::complete_attached_file_upload::builders::CompleteAttachedFileUploadFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::complete_attached_file_upload::builders::CompleteAttachedFileUploadFluentBuilder::set_instance_id):<br>required: **true**<br><p>The unique identifier of the Amazon Connect instance.</p><br>
    ///   - [`file_id(impl Into<String>)`](crate::operation::complete_attached_file_upload::builders::CompleteAttachedFileUploadFluentBuilder::file_id) / [`set_file_id(Option<String>)`](crate::operation::complete_attached_file_upload::builders::CompleteAttachedFileUploadFluentBuilder::set_file_id):<br>required: **true**<br><p>The unique identifier of the attached file resource.</p><br>
    ///   - [`associated_resource_arn(impl Into<String>)`](crate::operation::complete_attached_file_upload::builders::CompleteAttachedFileUploadFluentBuilder::associated_resource_arn) / [`set_associated_resource_arn(Option<String>)`](crate::operation::complete_attached_file_upload::builders::CompleteAttachedFileUploadFluentBuilder::set_associated_resource_arn):<br>required: **true**<br><p>The resource to which the attached file is (being) uploaded to. The supported resources are <a href="https://docs.aws.amazon.com/connect/latest/adminguide/cases.html">Cases</a> and <a href="https://docs.aws.amazon.com/connect/latest/adminguide/setup-email-channel.html">Email</a>.</p><note>  <p>This value must be a valid ARN.</p> </note><br>
    /// - On success, responds with [`CompleteAttachedFileUploadOutput`](crate::operation::complete_attached_file_upload::CompleteAttachedFileUploadOutput)
    /// - On failure, responds with [`SdkError<CompleteAttachedFileUploadError>`](crate::operation::complete_attached_file_upload::CompleteAttachedFileUploadError)
    pub fn complete_attached_file_upload(
        &self,
    ) -> crate::operation::complete_attached_file_upload::builders::CompleteAttachedFileUploadFluentBuilder {
        crate::operation::complete_attached_file_upload::builders::CompleteAttachedFileUploadFluentBuilder::new(self.handle.clone())
    }
}
