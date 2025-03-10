// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListPluginTypeMetadata`](crate::operation::list_plugin_type_metadata::builders::ListPluginTypeMetadataFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_plugin_type_metadata::builders::ListPluginTypeMetadataFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_plugin_type_metadata::builders::ListPluginTypeMetadataFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_plugin_type_metadata::builders::ListPluginTypeMetadataFluentBuilder::set_next_token):<br>required: **false**<br><p>If the metadata returned exceeds <code>maxResults</code>, Amazon Q Business returns a next token as a pagination token to retrieve the next set of metadata.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_plugin_type_metadata::builders::ListPluginTypeMetadataFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_plugin_type_metadata::builders::ListPluginTypeMetadataFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of plugin metadata items to return.</p><br>
    /// - On success, responds with [`ListPluginTypeMetadataOutput`](crate::operation::list_plugin_type_metadata::ListPluginTypeMetadataOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_plugin_type_metadata::ListPluginTypeMetadataOutput::next_token): <p>If the response is truncated, Amazon Q Business returns this token, which you can use in a later request to list the next set of plugin metadata.</p>
    ///   - [`items(Option<Vec::<PluginTypeMetadataSummary>>)`](crate::operation::list_plugin_type_metadata::ListPluginTypeMetadataOutput::items): <p>An array of information on plugin metadata.</p>
    /// - On failure, responds with [`SdkError<ListPluginTypeMetadataError>`](crate::operation::list_plugin_type_metadata::ListPluginTypeMetadataError)
    pub fn list_plugin_type_metadata(&self) -> crate::operation::list_plugin_type_metadata::builders::ListPluginTypeMetadataFluentBuilder {
        crate::operation::list_plugin_type_metadata::builders::ListPluginTypeMetadataFluentBuilder::new(self.handle.clone())
    }
}
