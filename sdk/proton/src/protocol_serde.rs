// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn type_erase_result<O, E>(
    result: ::std::result::Result<O, E>,
) -> ::std::result::Result<
    ::aws_smithy_runtime_api::client::interceptors::context::Output,
    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,
>
where
    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
{
    result
        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
        .map_err(::std::convert::Into::into)
}

pub fn parse_http_error_metadata(
    _response_status: u16,
    response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_accept_environment_account_connection;

pub(crate) mod shape_cancel_component_deployment;

pub(crate) mod shape_cancel_environment_deployment;

pub(crate) mod shape_cancel_service_instance_deployment;

pub(crate) mod shape_cancel_service_pipeline_deployment;

pub(crate) mod shape_create_component;

pub(crate) mod shape_create_environment;

pub(crate) mod shape_create_environment_account_connection;

pub(crate) mod shape_create_environment_template;

pub(crate) mod shape_create_environment_template_version;

pub(crate) mod shape_create_repository;

pub(crate) mod shape_create_service;

pub(crate) mod shape_create_service_instance;

pub(crate) mod shape_create_service_sync_config;

pub(crate) mod shape_create_service_template;

pub(crate) mod shape_create_service_template_version;

pub(crate) mod shape_create_template_sync_config;

pub(crate) mod shape_delete_component;

pub(crate) mod shape_delete_deployment;

pub(crate) mod shape_delete_environment;

pub(crate) mod shape_delete_environment_account_connection;

pub(crate) mod shape_delete_environment_template;

pub(crate) mod shape_delete_environment_template_version;

pub(crate) mod shape_delete_repository;

pub(crate) mod shape_delete_service;

pub(crate) mod shape_delete_service_sync_config;

pub(crate) mod shape_delete_service_template;

pub(crate) mod shape_delete_service_template_version;

pub(crate) mod shape_delete_template_sync_config;

pub(crate) mod shape_get_account_settings;

pub(crate) mod shape_get_component;

pub(crate) mod shape_get_deployment;

pub(crate) mod shape_get_environment;

pub(crate) mod shape_get_environment_account_connection;

pub(crate) mod shape_get_environment_template;

pub(crate) mod shape_get_environment_template_version;

pub(crate) mod shape_get_repository;

pub(crate) mod shape_get_repository_sync_status;

pub(crate) mod shape_get_resources_summary;

pub(crate) mod shape_get_service;

pub(crate) mod shape_get_service_instance;

pub(crate) mod shape_get_service_instance_sync_status;

pub(crate) mod shape_get_service_sync_blocker_summary;

pub(crate) mod shape_get_service_sync_config;

pub(crate) mod shape_get_service_template;

pub(crate) mod shape_get_service_template_version;

pub(crate) mod shape_get_template_sync_config;

pub(crate) mod shape_get_template_sync_status;

pub(crate) mod shape_list_component_outputs;

pub(crate) mod shape_list_component_provisioned_resources;

pub(crate) mod shape_list_components;

pub(crate) mod shape_list_deployments;

pub(crate) mod shape_list_environment_account_connections;

pub(crate) mod shape_list_environment_outputs;

pub(crate) mod shape_list_environment_provisioned_resources;

pub(crate) mod shape_list_environment_template_versions;

pub(crate) mod shape_list_environment_templates;

pub(crate) mod shape_list_environments;

pub(crate) mod shape_list_repositories;

pub(crate) mod shape_list_repository_sync_definitions;

pub(crate) mod shape_list_service_instance_outputs;

pub(crate) mod shape_list_service_instance_provisioned_resources;

pub(crate) mod shape_list_service_instances;

pub(crate) mod shape_list_service_pipeline_outputs;

pub(crate) mod shape_list_service_pipeline_provisioned_resources;

pub(crate) mod shape_list_service_template_versions;

pub(crate) mod shape_list_service_templates;

pub(crate) mod shape_list_services;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_notify_resource_deployment_status_change;

pub(crate) mod shape_reject_environment_account_connection;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_account_settings;

pub(crate) mod shape_update_component;

pub(crate) mod shape_update_environment;

pub(crate) mod shape_update_environment_account_connection;

pub(crate) mod shape_update_environment_template;

pub(crate) mod shape_update_environment_template_version;

pub(crate) mod shape_update_service;

pub(crate) mod shape_update_service_instance;

pub(crate) mod shape_update_service_pipeline;

pub(crate) mod shape_update_service_sync_blocker;

pub(crate) mod shape_update_service_sync_config;

pub(crate) mod shape_update_service_template;

pub(crate) mod shape_update_service_template_version;

pub(crate) mod shape_update_template_sync_config;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_accept_environment_account_connection_input;

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_cancel_component_deployment_input;

pub(crate) mod shape_cancel_environment_deployment_input;

pub(crate) mod shape_cancel_service_instance_deployment_input;

pub(crate) mod shape_cancel_service_pipeline_deployment_input;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_component_input;

pub(crate) mod shape_create_environment_account_connection_input;

pub(crate) mod shape_create_environment_input;

pub(crate) mod shape_create_environment_template_input;

pub(crate) mod shape_create_environment_template_version_input;

pub(crate) mod shape_create_repository_input;

pub(crate) mod shape_create_service_input;

pub(crate) mod shape_create_service_instance_input;

pub(crate) mod shape_create_service_sync_config_input;

pub(crate) mod shape_create_service_template_input;

pub(crate) mod shape_create_service_template_version_input;

pub(crate) mod shape_create_template_sync_config_input;

pub(crate) mod shape_delete_component_input;

pub(crate) mod shape_delete_deployment_input;

pub(crate) mod shape_delete_environment_account_connection_input;

pub(crate) mod shape_delete_environment_input;

pub(crate) mod shape_delete_environment_template_input;

pub(crate) mod shape_delete_environment_template_version_input;

pub(crate) mod shape_delete_repository_input;

pub(crate) mod shape_delete_service_input;

pub(crate) mod shape_delete_service_sync_config_input;

pub(crate) mod shape_delete_service_template_input;

pub(crate) mod shape_delete_service_template_version_input;

pub(crate) mod shape_delete_template_sync_config_input;

pub(crate) mod shape_get_component_input;

pub(crate) mod shape_get_deployment_input;

pub(crate) mod shape_get_environment_account_connection_input;

pub(crate) mod shape_get_environment_input;

pub(crate) mod shape_get_environment_template_input;

pub(crate) mod shape_get_environment_template_version_input;

pub(crate) mod shape_get_repository_input;

pub(crate) mod shape_get_repository_sync_status_input;

pub(crate) mod shape_get_service_input;

pub(crate) mod shape_get_service_instance_input;

pub(crate) mod shape_get_service_instance_sync_status_input;

pub(crate) mod shape_get_service_sync_blocker_summary_input;

pub(crate) mod shape_get_service_sync_config_input;

pub(crate) mod shape_get_service_template_input;

pub(crate) mod shape_get_service_template_version_input;

pub(crate) mod shape_get_template_sync_config_input;

pub(crate) mod shape_get_template_sync_status_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_list_component_outputs_input;

pub(crate) mod shape_list_component_provisioned_resources_input;

pub(crate) mod shape_list_components_input;

pub(crate) mod shape_list_deployments_input;

pub(crate) mod shape_list_environment_account_connections_input;

pub(crate) mod shape_list_environment_outputs_input;

pub(crate) mod shape_list_environment_provisioned_resources_input;

pub(crate) mod shape_list_environment_template_versions_input;

pub(crate) mod shape_list_environment_templates_input;

pub(crate) mod shape_list_environments_input;

pub(crate) mod shape_list_repositories_input;

pub(crate) mod shape_list_repository_sync_definitions_input;

pub(crate) mod shape_list_service_instance_outputs_input;

pub(crate) mod shape_list_service_instance_provisioned_resources_input;

pub(crate) mod shape_list_service_instances_input;

pub(crate) mod shape_list_service_pipeline_outputs_input;

pub(crate) mod shape_list_service_pipeline_provisioned_resources_input;

pub(crate) mod shape_list_service_template_versions_input;

pub(crate) mod shape_list_service_templates_input;

pub(crate) mod shape_list_services_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_notify_resource_deployment_status_change_input;

pub(crate) mod shape_reject_environment_account_connection_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_account_settings_input;

pub(crate) mod shape_update_component_input;

pub(crate) mod shape_update_environment_account_connection_input;

pub(crate) mod shape_update_environment_input;

pub(crate) mod shape_update_environment_template_input;

pub(crate) mod shape_update_environment_template_version_input;

pub(crate) mod shape_update_service_input;

pub(crate) mod shape_update_service_instance_input;

pub(crate) mod shape_update_service_pipeline_input;

pub(crate) mod shape_update_service_sync_blocker_input;

pub(crate) mod shape_update_service_sync_config_input;

pub(crate) mod shape_update_service_template_input;

pub(crate) mod shape_update_service_template_version_input;

pub(crate) mod shape_update_template_sync_config_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_account_settings;

pub(crate) mod shape_compatible_environment_template_input;

pub(crate) mod shape_component;

pub(crate) mod shape_component_summary_list;

pub(crate) mod shape_counts_summary;

pub(crate) mod shape_deployment;

pub(crate) mod shape_deployment_summary_list;

pub(crate) mod shape_environment;

pub(crate) mod shape_environment_account_connection;

pub(crate) mod shape_environment_account_connection_summary_list;

pub(crate) mod shape_environment_summary_list;

pub(crate) mod shape_environment_template;

pub(crate) mod shape_environment_template_filter;

pub(crate) mod shape_environment_template_summary_list;

pub(crate) mod shape_environment_template_version;

pub(crate) mod shape_environment_template_version_summary_list;

pub(crate) mod shape_list_service_instances_filter;

pub(crate) mod shape_output;

pub(crate) mod shape_outputs_list;

pub(crate) mod shape_provisioned_resource_list;

pub(crate) mod shape_repository;

pub(crate) mod shape_repository_branch_input;

pub(crate) mod shape_repository_summary_list;

pub(crate) mod shape_repository_sync_attempt;

pub(crate) mod shape_repository_sync_definition_list;

pub(crate) mod shape_resource_sync_attempt;

pub(crate) mod shape_revision;

pub(crate) mod shape_service;

pub(crate) mod shape_service_instance;

pub(crate) mod shape_service_instance_summary_list;

pub(crate) mod shape_service_pipeline;

pub(crate) mod shape_service_summary_list;

pub(crate) mod shape_service_sync_blocker_summary;

pub(crate) mod shape_service_sync_config;

pub(crate) mod shape_service_template;

pub(crate) mod shape_service_template_summary_list;

pub(crate) mod shape_service_template_version;

pub(crate) mod shape_service_template_version_summary_list;

pub(crate) mod shape_sync_blocker;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_template_sync_config;

pub(crate) mod shape_template_version_source_input;

pub(crate) mod shape_compatible_environment_template_list;

pub(crate) mod shape_component_summary;

pub(crate) mod shape_deployment_state;

pub(crate) mod shape_deployment_summary;

pub(crate) mod shape_environment_account_connection_summary;

pub(crate) mod shape_environment_summary;

pub(crate) mod shape_environment_template_summary;

pub(crate) mod shape_environment_template_version_summary;

pub(crate) mod shape_latest_sync_blockers;

pub(crate) mod shape_provisioned_resource;

pub(crate) mod shape_repository_branch;

pub(crate) mod shape_repository_summary;

pub(crate) mod shape_repository_sync_definition;

pub(crate) mod shape_repository_sync_events;

pub(crate) mod shape_resource_counts_summary;

pub(crate) mod shape_resource_sync_events;

pub(crate) mod shape_s3_object_source;

pub(crate) mod shape_service_instance_summary;

pub(crate) mod shape_service_summary;

pub(crate) mod shape_service_template_summary;

pub(crate) mod shape_service_template_supported_component_source_input_list;

pub(crate) mod shape_service_template_version_summary;

pub(crate) mod shape_sync_blocker_contexts;

pub(crate) mod shape_compatible_environment_template;

pub(crate) mod shape_component_state;

pub(crate) mod shape_environment_state;

pub(crate) mod shape_repository_sync_event;

pub(crate) mod shape_resource_sync_event;

pub(crate) mod shape_service_instance_state;

pub(crate) mod shape_service_pipeline_state;

pub(crate) mod shape_sync_blocker_context;

pub(crate) mod shape_component_deployment_id_list;
