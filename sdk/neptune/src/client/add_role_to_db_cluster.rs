// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddRoleToDBCluster`](crate::operation::add_role_to_db_cluster::builders::AddRoleToDBClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`db_cluster_identifier(impl Into<String>)`](crate::operation::add_role_to_db_cluster::builders::AddRoleToDBClusterFluentBuilder::db_cluster_identifier) / [`set_db_cluster_identifier(Option<String>)`](crate::operation::add_role_to_db_cluster::builders::AddRoleToDBClusterFluentBuilder::set_db_cluster_identifier):<br>required: **true**<br><p>The name of the DB cluster to associate the IAM role with.</p><br>
    ///   - [`role_arn(impl Into<String>)`](crate::operation::add_role_to_db_cluster::builders::AddRoleToDBClusterFluentBuilder::role_arn) / [`set_role_arn(Option<String>)`](crate::operation::add_role_to_db_cluster::builders::AddRoleToDBClusterFluentBuilder::set_role_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the IAM role to associate with the Neptune DB cluster, for example <code>arn:aws:iam::123456789012:role/NeptuneAccessRole</code>.</p><br>
    ///   - [`feature_name(impl Into<String>)`](crate::operation::add_role_to_db_cluster::builders::AddRoleToDBClusterFluentBuilder::feature_name) / [`set_feature_name(Option<String>)`](crate::operation::add_role_to_db_cluster::builders::AddRoleToDBClusterFluentBuilder::set_feature_name):<br>required: **false**<br><p>The name of the feature for the Neptune DB cluster that the IAM role is to be associated with. For the list of supported feature names, see <code>DBEngineVersion</code>.</p><br>
    /// - On success, responds with [`AddRoleToDbClusterOutput`](crate::operation::add_role_to_db_cluster::AddRoleToDbClusterOutput)
    /// - On failure, responds with [`SdkError<AddRoleToDBClusterError>`](crate::operation::add_role_to_db_cluster::AddRoleToDBClusterError)
    pub fn add_role_to_db_cluster(&self) -> crate::operation::add_role_to_db_cluster::builders::AddRoleToDBClusterFluentBuilder {
        crate::operation::add_role_to_db_cluster::builders::AddRoleToDBClusterFluentBuilder::new(self.handle.clone())
    }
}
