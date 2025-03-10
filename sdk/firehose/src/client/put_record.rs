// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutRecord`](crate::operation::put_record::builders::PutRecordFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`delivery_stream_name(impl Into<String>)`](crate::operation::put_record::builders::PutRecordFluentBuilder::delivery_stream_name) / [`set_delivery_stream_name(Option<String>)`](crate::operation::put_record::builders::PutRecordFluentBuilder::set_delivery_stream_name):<br>required: **true**<br><p>The name of the Firehose stream.</p><br>
    ///   - [`record(Record)`](crate::operation::put_record::builders::PutRecordFluentBuilder::record) / [`set_record(Option<Record>)`](crate::operation::put_record::builders::PutRecordFluentBuilder::set_record):<br>required: **true**<br><p>The record.</p><br>
    /// - On success, responds with [`PutRecordOutput`](crate::operation::put_record::PutRecordOutput) with field(s):
    ///   - [`record_id(String)`](crate::operation::put_record::PutRecordOutput::record_id): <p>The ID of the record.</p>
    ///   - [`encrypted(Option<bool>)`](crate::operation::put_record::PutRecordOutput::encrypted): <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    /// - On failure, responds with [`SdkError<PutRecordError>`](crate::operation::put_record::PutRecordError)
    pub fn put_record(&self) -> crate::operation::put_record::builders::PutRecordFluentBuilder {
        crate::operation::put_record::builders::PutRecordFluentBuilder::new(self.handle.clone())
    }
}
