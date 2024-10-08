// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListFleets`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`farm_id(impl Into<String>)`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::farm_id) / [`set_farm_id(Option<String>)`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::set_farm_id):<br>required: **true**<br><p>The farm ID of the fleets.</p><br>
    ///   - [`principal_id(impl Into<String>)`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::principal_id) / [`set_principal_id(Option<String>)`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::set_principal_id):<br>required: **false**<br><p>The principal ID of the members to include in the fleet.</p><br>
    ///   - [`display_name(impl Into<String>)`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::display_name) / [`set_display_name(Option<String>)`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::set_display_name):<br>required: **false**<br><p>The display names of a list of fleets.</p><important>  <p>This field can store any content. Escape or encode this content before displaying it on a webpage or any other system that might interpret the content of this field.</p> </important><br>
    ///   - [`status(FleetStatus)`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::status) / [`set_status(Option<FleetStatus>)`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::set_status):<br>required: **false**<br><p>The status of the fleet.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of results, or <code>null</code> to start from the beginning.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_fleets::builders::ListFleetsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p><br>
    /// - On success, responds with [`ListFleetsOutput`](crate::operation::list_fleets::ListFleetsOutput) with field(s):
    ///   - [`fleets(Vec::<FleetSummary>)`](crate::operation::list_fleets::ListFleetsOutput::fleets): <p>The fleets on the list.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_fleets::ListFleetsOutput::next_token): <p>If Deadline Cloud returns <code>nextToken</code>, then there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. To retrieve the next page, call the operation again using the returned token. Keep all other arguments unchanged. If no results remain, then <code>nextToken</code> is set to <code>null</code>. Each pagination token expires after 24 hours. If you provide a token that isn't valid, then you receive an HTTP 400 <code>ValidationException</code> error.</p>
    /// - On failure, responds with [`SdkError<ListFleetsError>`](crate::operation::list_fleets::ListFleetsError)
    pub fn list_fleets(&self) -> crate::operation::list_fleets::builders::ListFleetsFluentBuilder {
        crate::operation::list_fleets::builders::ListFleetsFluentBuilder::new(self.handle.clone())
    }
}
