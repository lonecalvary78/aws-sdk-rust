// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListEnvironmentHosts`](crate::operation::list_environment_hosts::builders::ListEnvironmentHostsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_environment_hosts::builders::ListEnvironmentHostsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_environment_hosts::builders::ListEnvironmentHostsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_environment_hosts::builders::ListEnvironmentHostsFluentBuilder::set_next_token):<br>required: **false**<br><p>A unique pagination token for each page. If <code>nextToken</code> is returned, there are more results available. Make the call again using the returned token with all other arguments unchanged to retrieve the next page. Each pagination token expires after 24 hours. Using an expired pagination token will return an <i>HTTP 400 InvalidToken</i> error.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_environment_hosts::builders::ListEnvironmentHostsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_environment_hosts::builders::ListEnvironmentHostsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return. If you specify <code>MaxResults</code> in the request, the response includes information up to the limit specified.</p><br>
    ///   - [`environment_id(impl Into<String>)`](crate::operation::list_environment_hosts::builders::ListEnvironmentHostsFluentBuilder::environment_id) / [`set_environment_id(Option<String>)`](crate::operation::list_environment_hosts::builders::ListEnvironmentHostsFluentBuilder::set_environment_id):<br>required: **true**<br><p>A unique ID for the environment.</p><br>
    /// - On success, responds with [`ListEnvironmentHostsOutput`](crate::operation::list_environment_hosts::ListEnvironmentHostsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_environment_hosts::ListEnvironmentHostsOutput::next_token): <p>A unique pagination token for next page results. Make the call again using this token to retrieve the next page.</p>
    ///   - [`environment_hosts(Option<Vec::<Host>>)`](crate::operation::list_environment_hosts::ListEnvironmentHostsOutput::environment_hosts): <p>A list of hosts in the environment.</p>
    /// - On failure, responds with [`SdkError<ListEnvironmentHostsError>`](crate::operation::list_environment_hosts::ListEnvironmentHostsError)
    pub fn list_environment_hosts(&self) -> crate::operation::list_environment_hosts::builders::ListEnvironmentHostsFluentBuilder {
        crate::operation::list_environment_hosts::builders::ListEnvironmentHostsFluentBuilder::new(self.handle.clone())
    }
}
