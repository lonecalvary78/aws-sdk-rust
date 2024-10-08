// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCluster`](crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_id(impl Into<String>)`](crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder::cluster_id) / [`set_cluster_id(Option<String>)`](crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder::set_cluster_id):<br>required: **true**<br>The ID of the cluster.<br>
    /// - On success, responds with [`DescribeClusterOutput`](crate::operation::describe_cluster::DescribeClusterOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::describe_cluster::DescribeClusterOutput::arn): The ARN of this Cluster. It is automatically assigned when the Cluster is created.
    ///   - [`channel_ids(Option<Vec::<String>>)`](crate::operation::describe_cluster::DescribeClusterOutput::channel_ids): Placeholder documentation for __listOf__string
    ///   - [`cluster_type(Option<ClusterType>)`](crate::operation::describe_cluster::DescribeClusterOutput::cluster_type): The hardware type for the Cluster
    ///   - [`id(Option<String>)`](crate::operation::describe_cluster::DescribeClusterOutput::id): The ID of the Cluster. Unique in the AWS account. The ID is the resource-id portion of the ARN.
    ///   - [`instance_role_arn(Option<String>)`](crate::operation::describe_cluster::DescribeClusterOutput::instance_role_arn): The ARN of the IAM role for the Node in this Cluster. Any Nodes that are associated with this Cluster assume this role. The role gives permissions to the operations that you expect these Node to perform.
    ///   - [`name(Option<String>)`](crate::operation::describe_cluster::DescribeClusterOutput::name): The name that you specified for the Cluster.
    ///   - [`network_settings(Option<ClusterNetworkSettings>)`](crate::operation::describe_cluster::DescribeClusterOutput::network_settings): Network settings that connect the Nodes in the Cluster to one or more of the Networks that the Cluster is associated with.
    ///   - [`state(Option<ClusterState>)`](crate::operation::describe_cluster::DescribeClusterOutput::state): The current state of the Cluster.
    /// - On failure, responds with [`SdkError<DescribeClusterError>`](crate::operation::describe_cluster::DescribeClusterError)
    pub fn describe_cluster(&self) -> crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder {
        crate::operation::describe_cluster::builders::DescribeClusterFluentBuilder::new(self.handle.clone())
    }
}
