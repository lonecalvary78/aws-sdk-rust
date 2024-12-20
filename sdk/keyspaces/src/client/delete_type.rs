// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteType`](crate::operation::delete_type::builders::DeleteTypeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`keyspace_name(impl Into<String>)`](crate::operation::delete_type::builders::DeleteTypeFluentBuilder::keyspace_name) / [`set_keyspace_name(Option<String>)`](crate::operation::delete_type::builders::DeleteTypeFluentBuilder::set_keyspace_name):<br>required: **true**<br><p>The name of the keyspace of the to be deleted type.</p><br>
    ///   - [`type_name(impl Into<String>)`](crate::operation::delete_type::builders::DeleteTypeFluentBuilder::type_name) / [`set_type_name(Option<String>)`](crate::operation::delete_type::builders::DeleteTypeFluentBuilder::set_type_name):<br>required: **true**<br><p>The name of the type to be deleted.</p><br>
    /// - On success, responds with [`DeleteTypeOutput`](crate::operation::delete_type::DeleteTypeOutput) with field(s):
    ///   - [`keyspace_arn(String)`](crate::operation::delete_type::DeleteTypeOutput::keyspace_arn): <p>The unique identifier of the keyspace from which the type was deleted in the format of an Amazon Resource Name (ARN).</p>
    ///   - [`type_name(String)`](crate::operation::delete_type::DeleteTypeOutput::type_name): <p>The name of the type that was deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteTypeError>`](crate::operation::delete_type::DeleteTypeError)
    pub fn delete_type(&self) -> crate::operation::delete_type::builders::DeleteTypeFluentBuilder {
        crate::operation::delete_type::builders::DeleteTypeFluentBuilder::new(self.handle.clone())
    }
}
