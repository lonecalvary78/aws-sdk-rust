// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListInvocations`](crate::operation::list_invocations::builders::ListInvocationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_invocations::builders::ListInvocationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_invocations::builders::ListInvocationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_invocations::builders::ListInvocationsFluentBuilder::set_next_token):<br>required: **false**<br><p>If the total number of results is greater than the <code>maxResults</code> value provided in the request, enter the token returned in the <code>nextToken</code> field in the response in this field to return the next batch of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_invocations::builders::ListInvocationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_invocations::builders::ListInvocationsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in the response. If the total number of results is greater than this value, use the token returned in the response in the <code>nextToken</code> field when making another request to return the next batch of results.</p><br>
    ///   - [`session_identifier(impl Into<String>)`](crate::operation::list_invocations::builders::ListInvocationsFluentBuilder::session_identifier) / [`set_session_identifier(Option<String>)`](crate::operation::list_invocations::builders::ListInvocationsFluentBuilder::set_session_identifier):<br>required: **true**<br><p>The unique identifier for the session to list invocations for. You can specify either the session's <code>sessionId</code> or its Amazon Resource Name (ARN).</p><br>
    /// - On success, responds with [`ListInvocationsOutput`](crate::operation::list_invocations::ListInvocationsOutput) with field(s):
    ///   - [`invocation_summaries(Vec::<InvocationSummary>)`](crate::operation::list_invocations::ListInvocationsOutput::invocation_summaries): <p>A list of invocation summaries associated with the session.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_invocations::ListInvocationsOutput::next_token): <p>If the total number of results is greater than the <code>maxResults</code> value provided in the request, use this token when making another request in the <code>nextToken</code> field to return the next batch of results.</p>
    /// - On failure, responds with [`SdkError<ListInvocationsError>`](crate::operation::list_invocations::ListInvocationsError)
    pub fn list_invocations(&self) -> crate::operation::list_invocations::builders::ListInvocationsFluentBuilder {
        crate::operation::list_invocations::builders::ListInvocationsFluentBuilder::new(self.handle.clone())
    }
}
