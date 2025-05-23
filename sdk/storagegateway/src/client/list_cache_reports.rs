// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListCacheReports`](crate::operation::list_cache_reports::builders::ListCacheReportsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_cache_reports::builders::ListCacheReportsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`marker(impl Into<String>)`](crate::operation::list_cache_reports::builders::ListCacheReportsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_cache_reports::builders::ListCacheReportsFluentBuilder::set_marker):<br>required: **false**<br><p>Opaque pagination token returned from a previous <code>ListCacheReports</code> operation. If present, <code>Marker</code> specifies where to continue the list from after a previous call to <code>ListCacheReports</code>. Optional.</p><br>
    /// - On success, responds with [`ListCacheReportsOutput`](crate::operation::list_cache_reports::ListCacheReportsOutput) with field(s):
    ///   - [`cache_report_list(Option<Vec::<CacheReportInfo>>)`](crate::operation::list_cache_reports::ListCacheReportsOutput::cache_report_list): <p>A list of existing cache reports for all file shares associated with your Amazon Web Services account. This list includes all information provided by the <code>DescribeCacheReport</code> action, such as report status, completion progress, start time, end time, filters, and tags.</p>
    ///   - [`marker(Option<String>)`](crate::operation::list_cache_reports::ListCacheReportsOutput::marker): <p>If the request includes <code>Marker</code>, the response returns that value in this field.</p>
    /// - On failure, responds with [`SdkError<ListCacheReportsError>`](crate::operation::list_cache_reports::ListCacheReportsError)
    pub fn list_cache_reports(&self) -> crate::operation::list_cache_reports::builders::ListCacheReportsFluentBuilder {
        crate::operation::list_cache_reports::builders::ListCacheReportsFluentBuilder::new(self.handle.clone())
    }
}
