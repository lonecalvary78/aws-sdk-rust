// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateClusterSchedulerConfig`](crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder::set_name):<br>required: **true**<br><p>Name for the cluster policy.</p><br>
    ///   - [`cluster_arn(impl Into<String>)`](crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder::cluster_arn) / [`set_cluster_arn(Option<String>)`](crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder::set_cluster_arn):<br>required: **true**<br><p>ARN of the cluster.</p><br>
    ///   - [`scheduler_config(SchedulerConfig)`](crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder::scheduler_config) / [`set_scheduler_config(Option<SchedulerConfig>)`](crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder::set_scheduler_config):<br>required: **true**<br><p>Configuration about the monitoring schedule.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder::set_description):<br>required: **false**<br><p>Description of the cluster policy.</p><br>
    ///   - [`tags(Tag)`](crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder::set_tags):<br>required: **false**<br><p>Tags of the cluster policy.</p><br>
    /// - On success, responds with [`CreateClusterSchedulerConfigOutput`](crate::operation::create_cluster_scheduler_config::CreateClusterSchedulerConfigOutput) with field(s):
    ///   - [`cluster_scheduler_config_arn(Option<String>)`](crate::operation::create_cluster_scheduler_config::CreateClusterSchedulerConfigOutput::cluster_scheduler_config_arn): <p>ARN of the cluster policy.</p>
    ///   - [`cluster_scheduler_config_id(Option<String>)`](crate::operation::create_cluster_scheduler_config::CreateClusterSchedulerConfigOutput::cluster_scheduler_config_id): <p>ID of the cluster policy.</p>
    /// - On failure, responds with [`SdkError<CreateClusterSchedulerConfigError>`](crate::operation::create_cluster_scheduler_config::CreateClusterSchedulerConfigError)
    pub fn create_cluster_scheduler_config(
        &self,
    ) -> crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder {
        crate::operation::create_cluster_scheduler_config::builders::CreateClusterSchedulerConfigFluentBuilder::new(self.handle.clone())
    }
}
