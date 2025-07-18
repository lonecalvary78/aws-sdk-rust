// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListGiVersions`](crate::operation::list_gi_versions::builders::ListGiVersionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_gi_versions::builders::ListGiVersionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_gi_versions::builders::ListGiVersionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_gi_versions::builders::ListGiVersionsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output.</p> <p>Default: <code>10</code></p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_gi_versions::builders::ListGiVersionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_gi_versions::builders::ListGiVersionsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p><br>
    ///   - [`shape(impl Into<String>)`](crate::operation::list_gi_versions::builders::ListGiVersionsFluentBuilder::shape) / [`set_shape(Option<String>)`](crate::operation::list_gi_versions::builders::ListGiVersionsFluentBuilder::set_shape):<br>required: **false**<br><p>The shape to return GI versions for. For a list of valid shapes, use the <code>ListDbSystemShapes</code> operation..</p><br>
    /// - On success, responds with [`ListGiVersionsOutput`](crate::operation::list_gi_versions::ListGiVersionsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_gi_versions::ListGiVersionsOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    ///   - [`gi_versions(Vec::<GiVersionSummary>)`](crate::operation::list_gi_versions::ListGiVersionsOutput::gi_versions): <p>The list of GI versions and their properties.</p>
    /// - On failure, responds with [`SdkError<ListGiVersionsError>`](crate::operation::list_gi_versions::ListGiVersionsError)
    pub fn list_gi_versions(&self) -> crate::operation::list_gi_versions::builders::ListGiVersionsFluentBuilder {
        crate::operation::list_gi_versions::builders::ListGiVersionsFluentBuilder::new(self.handle.clone())
    }
}
