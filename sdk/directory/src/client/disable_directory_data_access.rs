// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisableDirectoryDataAccess`](crate::operation::disable_directory_data_access::builders::DisableDirectoryDataAccessFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::operation::disable_directory_data_access::builders::DisableDirectoryDataAccessFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::disable_directory_data_access::builders::DisableDirectoryDataAccessFluentBuilder::set_directory_id):<br>required: **true**<br><p>The directory identifier.</p><br>
    /// - On success, responds with [`DisableDirectoryDataAccessOutput`](crate::operation::disable_directory_data_access::DisableDirectoryDataAccessOutput)
    /// - On failure, responds with [`SdkError<DisableDirectoryDataAccessError>`](crate::operation::disable_directory_data_access::DisableDirectoryDataAccessError)
    pub fn disable_directory_data_access(
        &self,
    ) -> crate::operation::disable_directory_data_access::builders::DisableDirectoryDataAccessFluentBuilder {
        crate::operation::disable_directory_data_access::builders::DisableDirectoryDataAccessFluentBuilder::new(self.handle.clone())
    }
}
