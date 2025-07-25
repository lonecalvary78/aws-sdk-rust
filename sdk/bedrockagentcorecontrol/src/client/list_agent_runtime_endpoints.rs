// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAgentRuntimeEndpoints`](crate::operation::list_agent_runtime_endpoints::builders::ListAgentRuntimeEndpointsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_agent_runtime_endpoints::builders::ListAgentRuntimeEndpointsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`agent_runtime_id(impl Into<String>)`](crate::operation::list_agent_runtime_endpoints::builders::ListAgentRuntimeEndpointsFluentBuilder::agent_runtime_id) / [`set_agent_runtime_id(Option<String>)`](crate::operation::list_agent_runtime_endpoints::builders::ListAgentRuntimeEndpointsFluentBuilder::set_agent_runtime_id):<br>required: **true**<br><p>The unique identifier of the agent runtime to list endpoints for.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_agent_runtime_endpoints::builders::ListAgentRuntimeEndpointsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_agent_runtime_endpoints::builders::ListAgentRuntimeEndpointsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in the response.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_agent_runtime_endpoints::builders::ListAgentRuntimeEndpointsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_agent_runtime_endpoints::builders::ListAgentRuntimeEndpointsFluentBuilder::set_next_token):<br>required: **false**<br><p>A token to retrieve the next page of results.</p><br>
    /// - On success, responds with [`ListAgentRuntimeEndpointsOutput`](crate::operation::list_agent_runtime_endpoints::ListAgentRuntimeEndpointsOutput) with field(s):
    ///   - [`runtime_endpoints(Vec::<AgentEndpoint>)`](crate::operation::list_agent_runtime_endpoints::ListAgentRuntimeEndpointsOutput::runtime_endpoints): <p>The list of agent runtime endpoints.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_agent_runtime_endpoints::ListAgentRuntimeEndpointsOutput::next_token): <p>A token to retrieve the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListAgentRuntimeEndpointsError>`](crate::operation::list_agent_runtime_endpoints::ListAgentRuntimeEndpointsError)
    pub fn list_agent_runtime_endpoints(&self) -> crate::operation::list_agent_runtime_endpoints::builders::ListAgentRuntimeEndpointsFluentBuilder {
        crate::operation::list_agent_runtime_endpoints::builders::ListAgentRuntimeEndpointsFluentBuilder::new(self.handle.clone())
    }
}
