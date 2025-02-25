// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutRecordBatch`](crate::operation::put_record_batch::builders::PutRecordBatchFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`delivery_stream_name(impl Into<String>)`](crate::operation::put_record_batch::builders::PutRecordBatchFluentBuilder::delivery_stream_name) / [`set_delivery_stream_name(Option<String>)`](crate::operation::put_record_batch::builders::PutRecordBatchFluentBuilder::set_delivery_stream_name):<br>required: **true**<br><p>The name of the Firehose stream.</p><br>
    ///   - [`records(Record)`](crate::operation::put_record_batch::builders::PutRecordBatchFluentBuilder::records) / [`set_records(Option<Vec::<Record>>)`](crate::operation::put_record_batch::builders::PutRecordBatchFluentBuilder::set_records):<br>required: **true**<br><p>One or more records.</p><br>
    /// - On success, responds with [`PutRecordBatchOutput`](crate::operation::put_record_batch::PutRecordBatchOutput) with field(s):
    ///   - [`failed_put_count(i32)`](crate::operation::put_record_batch::PutRecordBatchOutput::failed_put_count): <p>The number of records that might have failed processing. This number might be greater than 0 even if the <code>PutRecordBatch</code> call succeeds. Check <code>FailedPutCount</code> to determine whether there are records that you need to resend.</p>
    ///   - [`encrypted(Option<bool>)`](crate::operation::put_record_batch::PutRecordBatchOutput::encrypted): <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    ///   - [`request_responses(Vec::<PutRecordBatchResponseEntry>)`](crate::operation::put_record_batch::PutRecordBatchOutput::request_responses): <p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>
    /// - On failure, responds with [`SdkError<PutRecordBatchError>`](crate::operation::put_record_batch::PutRecordBatchError)
    pub fn put_record_batch(&self) -> crate::operation::put_record_batch::builders::PutRecordBatchFluentBuilder {
        crate::operation::put_record_batch::builders::PutRecordBatchFluentBuilder::new(self.handle.clone())
    }
}
