// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteFlow`](crate::operation::delete_flow::builders::DeleteFlowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`flow_arn(impl Into<String>)`](crate::operation::delete_flow::builders::DeleteFlowFluentBuilder::flow_arn) / [`set_flow_arn(Option<String>)`](crate::operation::delete_flow::builders::DeleteFlowFluentBuilder::set_flow_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the flow that you want to delete.</p><br>
    /// - On success, responds with [`DeleteFlowOutput`](crate::operation::delete_flow::DeleteFlowOutput) with field(s):
    ///   - [`flow_arn(Option<String>)`](crate::operation::delete_flow::DeleteFlowOutput::flow_arn): <p>The ARN of the flow that was deleted.</p>
    ///   - [`status(Option<Status>)`](crate::operation::delete_flow::DeleteFlowOutput::status): <p>The status of the flow when the <code>DeleteFlow</code> process begins.</p>
    /// - On failure, responds with [`SdkError<DeleteFlowError>`](crate::operation::delete_flow::DeleteFlowError)
    pub fn delete_flow(&self) -> crate::operation::delete_flow::builders::DeleteFlowFluentBuilder {
        crate::operation::delete_flow::builders::DeleteFlowFluentBuilder::new(self.handle.clone())
    }
}
