// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeClusterNode`](crate::operation::describe_cluster_node::builders::DescribeClusterNodeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_name(impl Into<String>)`](crate::operation::describe_cluster_node::builders::DescribeClusterNodeFluentBuilder::cluster_name) / [`set_cluster_name(Option<String>)`](crate::operation::describe_cluster_node::builders::DescribeClusterNodeFluentBuilder::set_cluster_name):<br>required: **true**<br><p>The string name or the Amazon Resource Name (ARN) of the SageMaker HyperPod cluster in which the node is.</p><br>
    ///   - [`node_id(impl Into<String>)`](crate::operation::describe_cluster_node::builders::DescribeClusterNodeFluentBuilder::node_id) / [`set_node_id(Option<String>)`](crate::operation::describe_cluster_node::builders::DescribeClusterNodeFluentBuilder::set_node_id):<br>required: **false**<br><p>The ID of the SageMaker HyperPod cluster node.</p><br>
    /// - On success, responds with [`DescribeClusterNodeOutput`](crate::operation::describe_cluster_node::DescribeClusterNodeOutput) with field(s):
    ///   - [`node_details(Option<ClusterNodeDetails>)`](crate::operation::describe_cluster_node::DescribeClusterNodeOutput::node_details): <p>The details of the SageMaker HyperPod cluster node.</p>
    /// - On failure, responds with [`SdkError<DescribeClusterNodeError>`](crate::operation::describe_cluster_node::DescribeClusterNodeError)
    pub fn describe_cluster_node(&self) -> crate::operation::describe_cluster_node::builders::DescribeClusterNodeFluentBuilder {
        crate::operation::describe_cluster_node::builders::DescribeClusterNodeFluentBuilder::new(self.handle.clone())
    }
}
