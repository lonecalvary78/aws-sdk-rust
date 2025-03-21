// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeQueryDefinitions`](crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`query_language(QueryLanguage)`](crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsFluentBuilder::query_language) / [`set_query_language(Option<QueryLanguage>)`](crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsFluentBuilder::set_query_language):<br>required: **false**<br><p>The query language used for this query. For more information about the query languages that CloudWatch Logs supports, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/CWL_AnalyzeLogData_Languages.html">Supported query languages</a>.</p><br>
    ///   - [`query_definition_name_prefix(impl Into<String>)`](crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsFluentBuilder::query_definition_name_prefix) / [`set_query_definition_name_prefix(Option<String>)`](crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsFluentBuilder::set_query_definition_name_prefix):<br>required: **false**<br><p>Use this parameter to filter your results to only the query definitions that have names that start with the prefix you specify.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsFluentBuilder::set_max_results):<br>required: **false**<br><p>Limits the number of returned query definitions to the specified number.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of items to return. The token expires after 24 hours.</p><br>
    /// - On success, responds with [`DescribeQueryDefinitionsOutput`](crate::operation::describe_query_definitions::DescribeQueryDefinitionsOutput) with field(s):
    ///   - [`query_definitions(Option<Vec::<QueryDefinition>>)`](crate::operation::describe_query_definitions::DescribeQueryDefinitionsOutput::query_definitions): <p>The list of query definitions that match your request.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_query_definitions::DescribeQueryDefinitionsOutput::next_token): <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    /// - On failure, responds with [`SdkError<DescribeQueryDefinitionsError>`](crate::operation::describe_query_definitions::DescribeQueryDefinitionsError)
    pub fn describe_query_definitions(&self) -> crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsFluentBuilder {
        crate::operation::describe_query_definitions::builders::DescribeQueryDefinitionsFluentBuilder::new(self.handle.clone())
    }
}
