// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListCommands`](crate::operation::list_commands::builders::ListCommandsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_commands::builders::ListCommandsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_commands::builders::ListCommandsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_commands::builders::ListCommandsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in this operation. By default, the API returns up to a maximum of 25 results. You can override this default value to return up to a maximum of 100 results for this operation.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_commands::builders::ListCommandsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_commands::builders::ListCommandsFluentBuilder::set_next_token):<br>required: **false**<br><p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <code>null</code> to receive the first set of results.</p><br>
    ///   - [`namespace(CommandNamespace)`](crate::operation::list_commands::builders::ListCommandsFluentBuilder::namespace) / [`set_namespace(Option<CommandNamespace>)`](crate::operation::list_commands::builders::ListCommandsFluentBuilder::set_namespace):<br>required: **false**<br><p>The namespace of the command. By default, the API returns all commands that have been created for both <code>AWS-IoT</code> and <code>AWS-IoT-FleetWise</code> namespaces. You can override this default value if you want to return all commands that have been created only for a specific namespace.</p><br>
    ///   - [`command_parameter_name(impl Into<String>)`](crate::operation::list_commands::builders::ListCommandsFluentBuilder::command_parameter_name) / [`set_command_parameter_name(Option<String>)`](crate::operation::list_commands::builders::ListCommandsFluentBuilder::set_command_parameter_name):<br>required: **false**<br><p>A filter that can be used to display the list of commands that have a specific command parameter name.</p><br>
    ///   - [`sort_order(SortOrder)`](crate::operation::list_commands::builders::ListCommandsFluentBuilder::sort_order) / [`set_sort_order(Option<SortOrder>)`](crate::operation::list_commands::builders::ListCommandsFluentBuilder::set_sort_order):<br>required: **false**<br><p>Specify whether to list the commands that you have created in the ascending or descending order. By default, the API returns all commands in the descending order based on the time that they were created.</p><br>
    /// - On success, responds with [`ListCommandsOutput`](crate::operation::list_commands::ListCommandsOutput) with field(s):
    ///   - [`commands(Option<Vec::<CommandSummary>>)`](crate::operation::list_commands::ListCommandsOutput::commands): <p>The list of commands.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_commands::ListCommandsOutput::next_token): <p>The token to use to get the next set of results, or <code>null</code> if there are no additional results.</p>
    /// - On failure, responds with [`SdkError<ListCommandsError>`](crate::operation::list_commands::ListCommandsError)
    pub fn list_commands(&self) -> crate::operation::list_commands::builders::ListCommandsFluentBuilder {
        crate::operation::list_commands::builders::ListCommandsFluentBuilder::new(self.handle.clone())
    }
}
