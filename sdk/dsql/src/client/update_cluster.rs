// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateCluster`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifier(impl Into<String>)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::set_identifier):<br>required: **true**<br><p>The ID of the cluster you want to update.</p><br>
    ///   - [`deletion_protection_enabled(bool)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::deletion_protection_enabled) / [`set_deletion_protection_enabled(Option<bool>)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::set_deletion_protection_enabled):<br>required: **false**<br><p>Specifies whether to enable deletion protection in your cluster.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Idempotency ensures that an API request completes only once. With an idempotent request, if the original request completes successfully. The subsequent retries with the same client token return the result from the original successful request and they have no additional effect.</p> <p>If you don't specify a client token, the Amazon Web Services SDK automatically generates one.</p><br>
    /// - On success, responds with [`UpdateClusterOutput`](crate::operation::update_cluster::UpdateClusterOutput) with field(s):
    ///   - [`identifier(String)`](crate::operation::update_cluster::UpdateClusterOutput::identifier): <p>The ID of the cluster to update.</p>
    ///   - [`arn(String)`](crate::operation::update_cluster::UpdateClusterOutput::arn): <p>The ARN of the updated cluster.</p>
    ///   - [`status(ClusterStatus)`](crate::operation::update_cluster::UpdateClusterOutput::status): <p>The status of the updated cluster.</p>
    ///   - [`creation_time(DateTime)`](crate::operation::update_cluster::UpdateClusterOutput::creation_time): <p>The time of when the cluster was created.</p>
    ///   - [`deletion_protection_enabled(bool)`](crate::operation::update_cluster::UpdateClusterOutput::deletion_protection_enabled): <p>Whether deletion protection is enabled for the updated cluster.</p>
    ///   - [`witness_region(Option<String>)`](crate::operation::update_cluster::UpdateClusterOutput::witness_region): <p>The Region that receives all data you write to linked clusters.</p>
    ///   - [`linked_cluster_arns(Option<Vec::<String>>)`](crate::operation::update_cluster::UpdateClusterOutput::linked_cluster_arns): <p>The ARNs of the clusters linked to the updated cluster. Applicable only for multi-Region clusters.</p>
    /// - On failure, responds with [`SdkError<UpdateClusterError>`](crate::operation::update_cluster::UpdateClusterError)
    pub fn update_cluster(&self) -> crate::operation::update_cluster::builders::UpdateClusterFluentBuilder {
        crate::operation::update_cluster::builders::UpdateClusterFluentBuilder::new(self.handle.clone())
    }
}
