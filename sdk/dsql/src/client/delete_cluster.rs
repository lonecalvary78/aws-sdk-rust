// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCluster`](crate::operation::delete_cluster::builders::DeleteClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifier(impl Into<String>)`](crate::operation::delete_cluster::builders::DeleteClusterFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::delete_cluster::builders::DeleteClusterFluentBuilder::set_identifier):<br>required: **true**<br><p>The ID of the cluster to delete.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::delete_cluster::builders::DeleteClusterFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::delete_cluster::builders::DeleteClusterFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Idempotency ensures that an API request completes only once. With an idempotent request, if the original request completes successfully. The subsequent retries with the same client token return the result from the original successful request and they have no additional effect.</p> <p>If you don't specify a client token, the Amazon Web Services SDK automatically generates one.</p><br>
    /// - On success, responds with [`DeleteClusterOutput`](crate::operation::delete_cluster::DeleteClusterOutput) with field(s):
    ///   - [`identifier(String)`](crate::operation::delete_cluster::DeleteClusterOutput::identifier): <p>The ID of the deleted cluster.</p>
    ///   - [`arn(String)`](crate::operation::delete_cluster::DeleteClusterOutput::arn): <p>The ARN of the deleted cluster.</p>
    ///   - [`status(ClusterStatus)`](crate::operation::delete_cluster::DeleteClusterOutput::status): <p>The status of the cluster.</p>
    ///   - [`creation_time(DateTime)`](crate::operation::delete_cluster::DeleteClusterOutput::creation_time): <p>The time of when the cluster was created.</p>
    /// - On failure, responds with [`SdkError<DeleteClusterError>`](crate::operation::delete_cluster::DeleteClusterError)
    pub fn delete_cluster(&self) -> crate::operation::delete_cluster::builders::DeleteClusterFluentBuilder {
        crate::operation::delete_cluster::builders::DeleteClusterFluentBuilder::new(self.handle.clone())
    }
}
