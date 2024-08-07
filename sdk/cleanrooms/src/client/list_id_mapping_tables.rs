// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListIdMappingTables`](crate::operation::list_id_mapping_tables::builders::ListIdMappingTablesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_id_mapping_tables::builders::ListIdMappingTablesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`membership_identifier(impl Into<String>)`](crate::operation::list_id_mapping_tables::builders::ListIdMappingTablesFluentBuilder::membership_identifier) / [`set_membership_identifier(Option<String>)`](crate::operation::list_id_mapping_tables::builders::ListIdMappingTablesFluentBuilder::set_membership_identifier):<br>required: **true**<br><p>The unique identifier of the membership that contains the ID mapping tables that you want to view.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_id_mapping_tables::builders::ListIdMappingTablesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_id_mapping_tables::builders::ListIdMappingTablesFluentBuilder::set_next_token):<br>required: **false**<br><p>The pagination token that's used to fetch the next set of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_id_mapping_tables::builders::ListIdMappingTablesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_id_mapping_tables::builders::ListIdMappingTablesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum size of the results that is returned per call. Service chooses a default if it has not been set. Service may return a nextToken even if the maximum results has not been met.</p><br>
    /// - On success, responds with [`ListIdMappingTablesOutput`](crate::operation::list_id_mapping_tables::ListIdMappingTablesOutput) with field(s):
    ///   - [`id_mapping_table_summaries(Vec::<IdMappingTableSummary>)`](crate::operation::list_id_mapping_tables::ListIdMappingTablesOutput::id_mapping_table_summaries): <p>The summary information of the ID mapping tables that you requested.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_id_mapping_tables::ListIdMappingTablesOutput::next_token): <p>The token value provided to access the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListIdMappingTablesError>`](crate::operation::list_id_mapping_tables::ListIdMappingTablesError)
    pub fn list_id_mapping_tables(&self) -> crate::operation::list_id_mapping_tables::builders::ListIdMappingTablesFluentBuilder {
        crate::operation::list_id_mapping_tables::builders::ListIdMappingTablesFluentBuilder::new(self.handle.clone())
    }
}
