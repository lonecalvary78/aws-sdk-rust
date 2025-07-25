// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetTableBucket`](crate::operation::get_table_bucket::builders::GetTableBucketFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_bucket_arn(impl Into<String>)`](crate::operation::get_table_bucket::builders::GetTableBucketFluentBuilder::table_bucket_arn) / [`set_table_bucket_arn(Option<String>)`](crate::operation::get_table_bucket::builders::GetTableBucketFluentBuilder::set_table_bucket_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the table bucket.</p><br>
    /// - On success, responds with [`GetTableBucketOutput`](crate::operation::get_table_bucket::GetTableBucketOutput) with field(s):
    ///   - [`arn(String)`](crate::operation::get_table_bucket::GetTableBucketOutput::arn): <p>The Amazon Resource Name (ARN) of the table bucket.</p>
    ///   - [`name(String)`](crate::operation::get_table_bucket::GetTableBucketOutput::name): <p>The name of the table bucket</p>
    ///   - [`owner_account_id(String)`](crate::operation::get_table_bucket::GetTableBucketOutput::owner_account_id): <p>The ID of the account that owns the table bucket.</p>
    ///   - [`created_at(DateTime)`](crate::operation::get_table_bucket::GetTableBucketOutput::created_at): <p>The date and time the table bucket was created.</p>
    ///   - [`table_bucket_id(Option<String>)`](crate::operation::get_table_bucket::GetTableBucketOutput::table_bucket_id): <p>The unique identifier of the table bucket.</p>
    ///   - [`r#type(Option<TableBucketType>)`](crate::operation::get_table_bucket::GetTableBucketOutput::type): <p>The type of the table bucket.</p>
    /// - On failure, responds with [`SdkError<GetTableBucketError>`](crate::operation::get_table_bucket::GetTableBucketError)
    pub fn get_table_bucket(&self) -> crate::operation::get_table_bucket::builders::GetTableBucketFluentBuilder {
        crate::operation::get_table_bucket::builders::GetTableBucketFluentBuilder::new(self.handle.clone())
    }
}
