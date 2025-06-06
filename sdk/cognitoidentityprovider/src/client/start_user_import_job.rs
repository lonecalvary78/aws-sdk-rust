// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartUserImportJob`](crate::operation::start_user_import_job::builders::StartUserImportJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_pool_id(impl Into<String>)`](crate::operation::start_user_import_job::builders::StartUserImportJobFluentBuilder::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::operation::start_user_import_job::builders::StartUserImportJobFluentBuilder::set_user_pool_id):<br>required: **true**<br><p>The ID of the user pool that you want to start importing users into.</p><br>
    ///   - [`job_id(impl Into<String>)`](crate::operation::start_user_import_job::builders::StartUserImportJobFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::start_user_import_job::builders::StartUserImportJobFluentBuilder::set_job_id):<br>required: **true**<br><p>The ID of a user import job that you previously created.</p><br>
    /// - On success, responds with [`StartUserImportJobOutput`](crate::operation::start_user_import_job::StartUserImportJobOutput) with field(s):
    ///   - [`user_import_job(Option<UserImportJobType>)`](crate::operation::start_user_import_job::StartUserImportJobOutput::user_import_job): <p>The details of the user import job. Includes logging destination, status, and the Amazon S3 pre-signed URL for CSV upload.</p>
    /// - On failure, responds with [`SdkError<StartUserImportJobError>`](crate::operation::start_user_import_job::StartUserImportJobError)
    pub fn start_user_import_job(&self) -> crate::operation::start_user_import_job::builders::StartUserImportJobFluentBuilder {
        crate::operation::start_user_import_job::builders::StartUserImportJobFluentBuilder::new(self.handle.clone())
    }
}
