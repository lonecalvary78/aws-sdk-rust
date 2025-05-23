// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTracks`](crate::operation::list_tracks::builders::ListTracksFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_tracks::builders::ListTracksFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_tracks::builders::ListTracksFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_tracks::builders::ListTracksFluentBuilder::set_next_token):<br>required: **false**<br><p>If your initial <code>ListTracksRequest</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in following <code>ListTracksRequest</code> operations, which returns results in the next page.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_tracks::builders::ListTracksFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_tracks::builders::ListTracksFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of response records to return in each call. If the number of remaining response records exceeds the specified MaxRecords value, a value is returned in a marker field of the response. You can retrieve the next set of records by retrying the command with the returned marker value.</p><br>
    /// - On success, responds with [`ListTracksOutput`](crate::operation::list_tracks::ListTracksOutput) with field(s):
    ///   - [`tracks(Option<Vec::<ServerlessTrack>>)`](crate::operation::list_tracks::ListTracksOutput::tracks): <p>The returned tracks.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_tracks::ListTracksOutput::next_token): <p>When <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    /// - On failure, responds with [`SdkError<ListTracksError>`](crate::operation::list_tracks::ListTracksError)
    pub fn list_tracks(&self) -> crate::operation::list_tracks::builders::ListTracksFluentBuilder {
        crate::operation::list_tracks::builders::ListTracksFluentBuilder::new(self.handle.clone())
    }
}
