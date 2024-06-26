// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDataDeletionJob`](crate::operation::describe_data_deletion_job::builders::DescribeDataDeletionJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`data_deletion_job_arn(impl Into<String>)`](crate::operation::describe_data_deletion_job::builders::DescribeDataDeletionJobFluentBuilder::data_deletion_job_arn) / [`set_data_deletion_job_arn(Option<String>)`](crate::operation::describe_data_deletion_job::builders::DescribeDataDeletionJobFluentBuilder::set_data_deletion_job_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the data deletion job.</p><br>
    /// - On success, responds with [`DescribeDataDeletionJobOutput`](crate::operation::describe_data_deletion_job::DescribeDataDeletionJobOutput) with field(s):
    ///   - [`data_deletion_job(Option<DataDeletionJob>)`](crate::operation::describe_data_deletion_job::DescribeDataDeletionJobOutput::data_deletion_job): <p>Information about the data deletion job, including the status.</p> <p>The status is one of the following values:</p> <ul>  <li>   <p>PENDING</p></li>  <li>   <p>IN_PROGRESS</p></li>  <li>   <p>COMPLETED</p></li>  <li>   <p>FAILED</p></li> </ul>
    /// - On failure, responds with [`SdkError<DescribeDataDeletionJobError>`](crate::operation::describe_data_deletion_job::DescribeDataDeletionJobError)
    pub fn describe_data_deletion_job(&self) -> crate::operation::describe_data_deletion_job::builders::DescribeDataDeletionJobFluentBuilder {
        crate::operation::describe_data_deletion_job::builders::DescribeDataDeletionJobFluentBuilder::new(self.handle.clone())
    }
}
