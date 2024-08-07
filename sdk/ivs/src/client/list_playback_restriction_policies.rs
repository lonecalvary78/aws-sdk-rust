// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListPlaybackRestrictionPolicies`](crate::operation::list_playback_restriction_policies::builders::ListPlaybackRestrictionPoliciesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_playback_restriction_policies::builders::ListPlaybackRestrictionPoliciesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_playback_restriction_policies::builders::ListPlaybackRestrictionPoliciesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_playback_restriction_policies::builders::ListPlaybackRestrictionPoliciesFluentBuilder::set_next_token):<br>required: **false**<br><p>The first policy to retrieve. This is used for pagination; see the <code>nextToken</code> response field.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_playback_restriction_policies::builders::ListPlaybackRestrictionPoliciesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_playback_restriction_policies::builders::ListPlaybackRestrictionPoliciesFluentBuilder::set_max_results):<br>required: **false**<br><p>Maximum number of policies to return. Default: 1.</p><br>
    /// - On success, responds with [`ListPlaybackRestrictionPoliciesOutput`](crate::operation::list_playback_restriction_policies::ListPlaybackRestrictionPoliciesOutput) with field(s):
    ///   - [`playback_restriction_policies(Vec::<PlaybackRestrictionPolicySummary>)`](crate::operation::list_playback_restriction_policies::ListPlaybackRestrictionPoliciesOutput::playback_restriction_policies): <p>List of the matching policies.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_playback_restriction_policies::ListPlaybackRestrictionPoliciesOutput::next_token): <p>If there are more channels than <code>maxResults</code>, use <code>nextToken</code> in the request to get the next set.</p>
    /// - On failure, responds with [`SdkError<ListPlaybackRestrictionPoliciesError>`](crate::operation::list_playback_restriction_policies::ListPlaybackRestrictionPoliciesError)
    pub fn list_playback_restriction_policies(
        &self,
    ) -> crate::operation::list_playback_restriction_policies::builders::ListPlaybackRestrictionPoliciesFluentBuilder {
        crate::operation::list_playback_restriction_policies::builders::ListPlaybackRestrictionPoliciesFluentBuilder::new(self.handle.clone())
    }
}
