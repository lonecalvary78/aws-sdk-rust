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
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_xml::decode::XmlDecodeError> {
    crate::rest_xml_wrapped_errors::parse_error_metadata(response_body)
}

pub(crate) mod shape_accept_reserved_node_exchange;

pub(crate) mod shape_accept_reserved_node_exchange_input;

pub(crate) mod shape_add_partner;

pub(crate) mod shape_add_partner_input;

pub(crate) mod shape_associate_data_share_consumer;

pub(crate) mod shape_associate_data_share_consumer_input;

pub(crate) mod shape_authorize_cluster_security_group_ingress;

pub(crate) mod shape_authorize_cluster_security_group_ingress_input;

pub(crate) mod shape_authorize_data_share;

pub(crate) mod shape_authorize_data_share_input;

pub(crate) mod shape_authorize_endpoint_access;

pub(crate) mod shape_authorize_endpoint_access_input;

pub(crate) mod shape_authorize_snapshot_access;

pub(crate) mod shape_authorize_snapshot_access_input;

pub(crate) mod shape_batch_delete_cluster_snapshots;

pub(crate) mod shape_batch_delete_cluster_snapshots_input;

pub(crate) mod shape_batch_modify_cluster_snapshots;

pub(crate) mod shape_batch_modify_cluster_snapshots_input;

pub(crate) mod shape_cancel_resize;

pub(crate) mod shape_cancel_resize_input;

pub(crate) mod shape_copy_cluster_snapshot;

pub(crate) mod shape_copy_cluster_snapshot_input;

pub(crate) mod shape_create_authentication_profile;

pub(crate) mod shape_create_authentication_profile_input;

pub(crate) mod shape_create_cluster;

pub(crate) mod shape_create_cluster_input;

pub(crate) mod shape_create_cluster_parameter_group;

pub(crate) mod shape_create_cluster_parameter_group_input;

pub(crate) mod shape_create_cluster_security_group;

pub(crate) mod shape_create_cluster_security_group_input;

pub(crate) mod shape_create_cluster_snapshot;

pub(crate) mod shape_create_cluster_snapshot_input;

pub(crate) mod shape_create_cluster_subnet_group;

pub(crate) mod shape_create_cluster_subnet_group_input;

pub(crate) mod shape_create_custom_domain_association;

pub(crate) mod shape_create_custom_domain_association_input;

pub(crate) mod shape_create_endpoint_access;

pub(crate) mod shape_create_endpoint_access_input;

pub(crate) mod shape_create_event_subscription;

pub(crate) mod shape_create_event_subscription_input;

pub(crate) mod shape_create_hsm_client_certificate;

pub(crate) mod shape_create_hsm_client_certificate_input;

pub(crate) mod shape_create_hsm_configuration;

pub(crate) mod shape_create_hsm_configuration_input;

pub(crate) mod shape_create_integration;

pub(crate) mod shape_create_integration_input;

pub(crate) mod shape_create_redshift_idc_application;

pub(crate) mod shape_create_redshift_idc_application_input;

pub(crate) mod shape_create_scheduled_action;

pub(crate) mod shape_create_scheduled_action_input;

pub(crate) mod shape_create_snapshot_copy_grant;

pub(crate) mod shape_create_snapshot_copy_grant_input;

pub(crate) mod shape_create_snapshot_schedule;

pub(crate) mod shape_create_snapshot_schedule_input;

pub(crate) mod shape_create_tags;

pub(crate) mod shape_create_tags_input;

pub(crate) mod shape_create_usage_limit;

pub(crate) mod shape_create_usage_limit_input;

pub(crate) mod shape_deauthorize_data_share;

pub(crate) mod shape_deauthorize_data_share_input;

pub(crate) mod shape_delete_authentication_profile;

pub(crate) mod shape_delete_authentication_profile_input;

pub(crate) mod shape_delete_cluster;

pub(crate) mod shape_delete_cluster_input;

pub(crate) mod shape_delete_cluster_parameter_group;

pub(crate) mod shape_delete_cluster_parameter_group_input;

pub(crate) mod shape_delete_cluster_security_group;

pub(crate) mod shape_delete_cluster_security_group_input;

pub(crate) mod shape_delete_cluster_snapshot;

pub(crate) mod shape_delete_cluster_snapshot_input;

pub(crate) mod shape_delete_cluster_subnet_group;

pub(crate) mod shape_delete_cluster_subnet_group_input;

pub(crate) mod shape_delete_custom_domain_association;

pub(crate) mod shape_delete_custom_domain_association_input;

pub(crate) mod shape_delete_endpoint_access;

pub(crate) mod shape_delete_endpoint_access_input;

pub(crate) mod shape_delete_event_subscription;

pub(crate) mod shape_delete_event_subscription_input;

pub(crate) mod shape_delete_hsm_client_certificate;

pub(crate) mod shape_delete_hsm_client_certificate_input;

pub(crate) mod shape_delete_hsm_configuration;

pub(crate) mod shape_delete_hsm_configuration_input;

pub(crate) mod shape_delete_integration;

pub(crate) mod shape_delete_integration_input;

pub(crate) mod shape_delete_partner;

pub(crate) mod shape_delete_partner_input;

pub(crate) mod shape_delete_redshift_idc_application;

pub(crate) mod shape_delete_redshift_idc_application_input;

pub(crate) mod shape_delete_resource_policy;

pub(crate) mod shape_delete_resource_policy_input;

pub(crate) mod shape_delete_scheduled_action;

pub(crate) mod shape_delete_scheduled_action_input;

pub(crate) mod shape_delete_snapshot_copy_grant;

pub(crate) mod shape_delete_snapshot_copy_grant_input;

pub(crate) mod shape_delete_snapshot_schedule;

pub(crate) mod shape_delete_snapshot_schedule_input;

pub(crate) mod shape_delete_tags;

pub(crate) mod shape_delete_tags_input;

pub(crate) mod shape_delete_usage_limit;

pub(crate) mod shape_delete_usage_limit_input;

pub(crate) mod shape_deregister_namespace;

pub(crate) mod shape_deregister_namespace_input;

pub(crate) mod shape_describe_account_attributes;

pub(crate) mod shape_describe_account_attributes_input;

pub(crate) mod shape_describe_authentication_profiles;

pub(crate) mod shape_describe_authentication_profiles_input;

pub(crate) mod shape_describe_cluster_db_revisions;

pub(crate) mod shape_describe_cluster_db_revisions_input;

pub(crate) mod shape_describe_cluster_parameter_groups;

pub(crate) mod shape_describe_cluster_parameter_groups_input;

pub(crate) mod shape_describe_cluster_parameters;

pub(crate) mod shape_describe_cluster_parameters_input;

pub(crate) mod shape_describe_cluster_security_groups;

pub(crate) mod shape_describe_cluster_security_groups_input;

pub(crate) mod shape_describe_cluster_snapshots;

pub(crate) mod shape_describe_cluster_snapshots_input;

pub(crate) mod shape_describe_cluster_subnet_groups;

pub(crate) mod shape_describe_cluster_subnet_groups_input;

pub(crate) mod shape_describe_cluster_tracks;

pub(crate) mod shape_describe_cluster_tracks_input;

pub(crate) mod shape_describe_cluster_versions;

pub(crate) mod shape_describe_cluster_versions_input;

pub(crate) mod shape_describe_clusters;

pub(crate) mod shape_describe_clusters_input;

pub(crate) mod shape_describe_custom_domain_associations;

pub(crate) mod shape_describe_custom_domain_associations_input;

pub(crate) mod shape_describe_data_shares;

pub(crate) mod shape_describe_data_shares_for_consumer;

pub(crate) mod shape_describe_data_shares_for_consumer_input;

pub(crate) mod shape_describe_data_shares_for_producer;

pub(crate) mod shape_describe_data_shares_for_producer_input;

pub(crate) mod shape_describe_data_shares_input;

pub(crate) mod shape_describe_default_cluster_parameters;

pub(crate) mod shape_describe_default_cluster_parameters_input;

pub(crate) mod shape_describe_endpoint_access;

pub(crate) mod shape_describe_endpoint_access_input;

pub(crate) mod shape_describe_endpoint_authorization;

pub(crate) mod shape_describe_endpoint_authorization_input;

pub(crate) mod shape_describe_event_categories;

pub(crate) mod shape_describe_event_categories_input;

pub(crate) mod shape_describe_event_subscriptions;

pub(crate) mod shape_describe_event_subscriptions_input;

pub(crate) mod shape_describe_events;

pub(crate) mod shape_describe_events_input;

pub(crate) mod shape_describe_hsm_client_certificates;

pub(crate) mod shape_describe_hsm_client_certificates_input;

pub(crate) mod shape_describe_hsm_configurations;

pub(crate) mod shape_describe_hsm_configurations_input;

pub(crate) mod shape_describe_inbound_integrations;

pub(crate) mod shape_describe_inbound_integrations_input;

pub(crate) mod shape_describe_integrations;

pub(crate) mod shape_describe_integrations_input;

pub(crate) mod shape_describe_logging_status;

pub(crate) mod shape_describe_logging_status_input;

pub(crate) mod shape_describe_node_configuration_options;

pub(crate) mod shape_describe_node_configuration_options_input;

pub(crate) mod shape_describe_orderable_cluster_options;

pub(crate) mod shape_describe_orderable_cluster_options_input;

pub(crate) mod shape_describe_partners;

pub(crate) mod shape_describe_partners_input;

pub(crate) mod shape_describe_redshift_idc_applications;

pub(crate) mod shape_describe_redshift_idc_applications_input;

pub(crate) mod shape_describe_reserved_node_exchange_status;

pub(crate) mod shape_describe_reserved_node_exchange_status_input;

pub(crate) mod shape_describe_reserved_node_offerings;

pub(crate) mod shape_describe_reserved_node_offerings_input;

pub(crate) mod shape_describe_reserved_nodes;

pub(crate) mod shape_describe_reserved_nodes_input;

pub(crate) mod shape_describe_resize;

pub(crate) mod shape_describe_resize_input;

pub(crate) mod shape_describe_scheduled_actions;

pub(crate) mod shape_describe_scheduled_actions_input;

pub(crate) mod shape_describe_snapshot_copy_grants;

pub(crate) mod shape_describe_snapshot_copy_grants_input;

pub(crate) mod shape_describe_snapshot_schedules;

pub(crate) mod shape_describe_snapshot_schedules_input;

pub(crate) mod shape_describe_storage;

pub(crate) mod shape_describe_storage_input;

pub(crate) mod shape_describe_table_restore_status;

pub(crate) mod shape_describe_table_restore_status_input;

pub(crate) mod shape_describe_tags;

pub(crate) mod shape_describe_tags_input;

pub(crate) mod shape_describe_usage_limits;

pub(crate) mod shape_describe_usage_limits_input;

pub(crate) mod shape_disable_logging;

pub(crate) mod shape_disable_logging_input;

pub(crate) mod shape_disable_snapshot_copy;

pub(crate) mod shape_disable_snapshot_copy_input;

pub(crate) mod shape_disassociate_data_share_consumer;

pub(crate) mod shape_disassociate_data_share_consumer_input;

pub(crate) mod shape_enable_logging;

pub(crate) mod shape_enable_logging_input;

pub(crate) mod shape_enable_snapshot_copy;

pub(crate) mod shape_enable_snapshot_copy_input;

pub(crate) mod shape_failover_primary_compute;

pub(crate) mod shape_failover_primary_compute_input;

pub(crate) mod shape_get_cluster_credentials;

pub(crate) mod shape_get_cluster_credentials_input;

pub(crate) mod shape_get_cluster_credentials_with_iam;

pub(crate) mod shape_get_cluster_credentials_with_iam_input;

pub(crate) mod shape_get_reserved_node_exchange_configuration_options;

pub(crate) mod shape_get_reserved_node_exchange_configuration_options_input;

pub(crate) mod shape_get_reserved_node_exchange_offerings;

pub(crate) mod shape_get_reserved_node_exchange_offerings_input;

pub(crate) mod shape_get_resource_policy;

pub(crate) mod shape_get_resource_policy_input;

pub(crate) mod shape_list_recommendations;

pub(crate) mod shape_list_recommendations_input;

pub(crate) mod shape_modify_aqua_configuration;

pub(crate) mod shape_modify_aqua_configuration_input;

pub(crate) mod shape_modify_authentication_profile;

pub(crate) mod shape_modify_authentication_profile_input;

pub(crate) mod shape_modify_cluster;

pub(crate) mod shape_modify_cluster_db_revision;

pub(crate) mod shape_modify_cluster_db_revision_input;

pub(crate) mod shape_modify_cluster_iam_roles;

pub(crate) mod shape_modify_cluster_iam_roles_input;

pub(crate) mod shape_modify_cluster_input;

pub(crate) mod shape_modify_cluster_maintenance;

pub(crate) mod shape_modify_cluster_maintenance_input;

pub(crate) mod shape_modify_cluster_parameter_group;

pub(crate) mod shape_modify_cluster_parameter_group_input;

pub(crate) mod shape_modify_cluster_snapshot;

pub(crate) mod shape_modify_cluster_snapshot_input;

pub(crate) mod shape_modify_cluster_snapshot_schedule;

pub(crate) mod shape_modify_cluster_snapshot_schedule_input;

pub(crate) mod shape_modify_cluster_subnet_group;

pub(crate) mod shape_modify_cluster_subnet_group_input;

pub(crate) mod shape_modify_custom_domain_association;

pub(crate) mod shape_modify_custom_domain_association_input;

pub(crate) mod shape_modify_endpoint_access;

pub(crate) mod shape_modify_endpoint_access_input;

pub(crate) mod shape_modify_event_subscription;

pub(crate) mod shape_modify_event_subscription_input;

pub(crate) mod shape_modify_integration;

pub(crate) mod shape_modify_integration_input;

pub(crate) mod shape_modify_redshift_idc_application;

pub(crate) mod shape_modify_redshift_idc_application_input;

pub(crate) mod shape_modify_scheduled_action;

pub(crate) mod shape_modify_scheduled_action_input;

pub(crate) mod shape_modify_snapshot_copy_retention_period;

pub(crate) mod shape_modify_snapshot_copy_retention_period_input;

pub(crate) mod shape_modify_snapshot_schedule;

pub(crate) mod shape_modify_snapshot_schedule_input;

pub(crate) mod shape_modify_usage_limit;

pub(crate) mod shape_modify_usage_limit_input;

pub(crate) mod shape_pause_cluster;

pub(crate) mod shape_pause_cluster_input;

pub(crate) mod shape_purchase_reserved_node_offering;

pub(crate) mod shape_purchase_reserved_node_offering_input;

pub(crate) mod shape_put_resource_policy;

pub(crate) mod shape_put_resource_policy_input;

pub(crate) mod shape_reboot_cluster;

pub(crate) mod shape_reboot_cluster_input;

pub(crate) mod shape_register_namespace;

pub(crate) mod shape_register_namespace_input;

pub(crate) mod shape_reject_data_share;

pub(crate) mod shape_reject_data_share_input;

pub(crate) mod shape_reset_cluster_parameter_group;

pub(crate) mod shape_reset_cluster_parameter_group_input;

pub(crate) mod shape_resize_cluster;

pub(crate) mod shape_resize_cluster_input;

pub(crate) mod shape_restore_from_cluster_snapshot;

pub(crate) mod shape_restore_from_cluster_snapshot_input;

pub(crate) mod shape_restore_table_from_cluster_snapshot;

pub(crate) mod shape_restore_table_from_cluster_snapshot_input;

pub(crate) mod shape_resume_cluster;

pub(crate) mod shape_resume_cluster_input;

pub(crate) mod shape_revoke_cluster_security_group_ingress;

pub(crate) mod shape_revoke_cluster_security_group_ingress_input;

pub(crate) mod shape_revoke_endpoint_access;

pub(crate) mod shape_revoke_endpoint_access_input;

pub(crate) mod shape_revoke_snapshot_access;

pub(crate) mod shape_revoke_snapshot_access_input;

pub(crate) mod shape_rotate_encryption_key;

pub(crate) mod shape_rotate_encryption_key_input;

pub(crate) mod shape_update_partner_status;

pub(crate) mod shape_update_partner_status_input;

pub(crate) mod shape_access_to_cluster_denied_fault;

pub(crate) mod shape_access_to_snapshot_denied_fault;

pub(crate) mod shape_authentication_profile_already_exists_fault;

pub(crate) mod shape_authentication_profile_not_found_fault;

pub(crate) mod shape_authentication_profile_quota_exceeded_fault;

pub(crate) mod shape_authorization_already_exists_fault;

pub(crate) mod shape_authorization_not_found_fault;

pub(crate) mod shape_authorization_quota_exceeded_fault;

pub(crate) mod shape_authorized_token_issuer;

pub(crate) mod shape_batch_delete_request_size_exceeded_fault;

pub(crate) mod shape_batch_modify_cluster_snapshots_limit_exceeded_fault;

pub(crate) mod shape_bucket_not_found_fault;

pub(crate) mod shape_cluster_already_exists_fault;

pub(crate) mod shape_cluster_not_found_fault;

pub(crate) mod shape_cluster_on_latest_revision_fault;

pub(crate) mod shape_cluster_parameter_group_already_exists_fault;

pub(crate) mod shape_cluster_parameter_group_not_found_fault;

pub(crate) mod shape_cluster_parameter_group_quota_exceeded_fault;

pub(crate) mod shape_cluster_quota_exceeded_fault;

pub(crate) mod shape_cluster_security_group_already_exists_fault;

pub(crate) mod shape_cluster_security_group_not_found_fault;

pub(crate) mod shape_cluster_security_group_quota_exceeded_fault;

pub(crate) mod shape_cluster_snapshot_already_exists_fault;

pub(crate) mod shape_cluster_snapshot_not_found_fault;

pub(crate) mod shape_cluster_snapshot_quota_exceeded_fault;

pub(crate) mod shape_cluster_subnet_group_already_exists_fault;

pub(crate) mod shape_cluster_subnet_group_not_found_fault;

pub(crate) mod shape_cluster_subnet_group_quota_exceeded_fault;

pub(crate) mod shape_cluster_subnet_quota_exceeded_fault;

pub(crate) mod shape_conflict_policy_update_fault;

pub(crate) mod shape_copy_to_region_disabled_fault;

pub(crate) mod shape_custom_cname_association_fault;

pub(crate) mod shape_custom_domain_association_not_found_fault;

pub(crate) mod shape_delete_cluster_snapshot_message;

pub(crate) mod shape_dependent_service_access_denied_fault;

pub(crate) mod shape_dependent_service_request_throttling_fault;

pub(crate) mod shape_dependent_service_unavailable_fault;

pub(crate) mod shape_describe_integrations_filter;

pub(crate) mod shape_endpoint_already_exists_fault;

pub(crate) mod shape_endpoint_authorization_already_exists_fault;

pub(crate) mod shape_endpoint_authorization_not_found_fault;

pub(crate) mod shape_endpoint_authorizations_per_cluster_limit_exceeded_fault;

pub(crate) mod shape_endpoint_not_found_fault;

pub(crate) mod shape_endpoints_per_authorization_limit_exceeded_fault;

pub(crate) mod shape_endpoints_per_cluster_limit_exceeded_fault;

pub(crate) mod shape_event_subscription_quota_exceeded_fault;

pub(crate) mod shape_hsm_client_certificate_already_exists_fault;

pub(crate) mod shape_hsm_client_certificate_not_found_fault;

pub(crate) mod shape_hsm_client_certificate_quota_exceeded_fault;

pub(crate) mod shape_hsm_configuration_already_exists_fault;

pub(crate) mod shape_hsm_configuration_not_found_fault;

pub(crate) mod shape_hsm_configuration_quota_exceeded_fault;

pub(crate) mod shape_in_progress_table_restore_quota_exceeded_fault;

pub(crate) mod shape_incompatible_orderable_options;

pub(crate) mod shape_insufficient_cluster_capacity_fault;

pub(crate) mod shape_insufficient_s3_bucket_policy_fault;

pub(crate) mod shape_integration_already_exists_fault;

pub(crate) mod shape_integration_conflict_operation_fault;

pub(crate) mod shape_integration_conflict_state_fault;

pub(crate) mod shape_integration_not_found_fault;

pub(crate) mod shape_integration_quota_exceeded_fault;

pub(crate) mod shape_integration_source_not_found_fault;

pub(crate) mod shape_integration_target_not_found_fault;

pub(crate) mod shape_invalid_authentication_profile_request_fault;

pub(crate) mod shape_invalid_authorization_state_fault;

pub(crate) mod shape_invalid_cluster_parameter_group_state_fault;

pub(crate) mod shape_invalid_cluster_security_group_state_fault;

pub(crate) mod shape_invalid_cluster_snapshot_schedule_state_fault;

pub(crate) mod shape_invalid_cluster_snapshot_state_fault;

pub(crate) mod shape_invalid_cluster_state_fault;

pub(crate) mod shape_invalid_cluster_subnet_group_state_fault;

pub(crate) mod shape_invalid_cluster_subnet_state_fault;

pub(crate) mod shape_invalid_cluster_track_fault;

pub(crate) mod shape_invalid_data_share_fault;

pub(crate) mod shape_invalid_elastic_ip_fault;

pub(crate) mod shape_invalid_endpoint_state_fault;

pub(crate) mod shape_invalid_hsm_client_certificate_state_fault;

pub(crate) mod shape_invalid_hsm_configuration_state_fault;

pub(crate) mod shape_invalid_namespace_fault;

pub(crate) mod shape_invalid_policy_fault;

pub(crate) mod shape_invalid_reserved_node_state_fault;

pub(crate) mod shape_invalid_restore_fault;

pub(crate) mod shape_invalid_retention_period_fault;

pub(crate) mod shape_invalid_s3_bucket_name_fault;

pub(crate) mod shape_invalid_s3_key_prefix_fault;

pub(crate) mod shape_invalid_schedule_fault;

pub(crate) mod shape_invalid_scheduled_action_fault;

pub(crate) mod shape_invalid_snapshot_copy_grant_state_fault;

pub(crate) mod shape_invalid_subnet;

pub(crate) mod shape_invalid_subscription_state_fault;

pub(crate) mod shape_invalid_table_restore_argument_fault;

pub(crate) mod shape_invalid_tag_fault;

pub(crate) mod shape_invalid_usage_limit_fault;

pub(crate) mod shape_invalid_vpc_network_state_fault;

pub(crate) mod shape_ipv6_cidr_block_not_found_fault;

pub(crate) mod shape_limit_exceeded_fault;

pub(crate) mod shape_namespace_identifier_union;

pub(crate) mod shape_node_configuration_options_filter;

pub(crate) mod shape_number_of_nodes_per_cluster_limit_exceeded_fault;

pub(crate) mod shape_number_of_nodes_quota_exceeded_fault;

pub(crate) mod shape_parameter;

pub(crate) mod shape_partner_not_found_fault;

pub(crate) mod shape_redshift_idc_application_already_exists_fault;

pub(crate) mod shape_redshift_idc_application_not_exists_fault;

pub(crate) mod shape_redshift_idc_application_quota_exceeded_fault;

pub(crate) mod shape_reserved_node_already_exists_fault;

pub(crate) mod shape_reserved_node_already_migrated_fault;

pub(crate) mod shape_reserved_node_exchange_not_found_fault;

pub(crate) mod shape_reserved_node_not_found_fault;

pub(crate) mod shape_reserved_node_offering_not_found_fault;

pub(crate) mod shape_reserved_node_quota_exceeded_fault;

pub(crate) mod shape_resize_not_found_fault;

pub(crate) mod shape_resource_not_found_fault;

pub(crate) mod shape_schedule_definition_type_unsupported_fault;

pub(crate) mod shape_scheduled_action_already_exists_fault;

pub(crate) mod shape_scheduled_action_filter;

pub(crate) mod shape_scheduled_action_not_found_fault;

pub(crate) mod shape_scheduled_action_quota_exceeded_fault;

pub(crate) mod shape_scheduled_action_type;

pub(crate) mod shape_scheduled_action_type_unsupported_fault;

pub(crate) mod shape_service_integrations_union;

pub(crate) mod shape_snapshot_copy_already_disabled_fault;

pub(crate) mod shape_snapshot_copy_already_enabled_fault;

pub(crate) mod shape_snapshot_copy_disabled_fault;

pub(crate) mod shape_snapshot_copy_grant_already_exists_fault;

pub(crate) mod shape_snapshot_copy_grant_not_found_fault;

pub(crate) mod shape_snapshot_copy_grant_quota_exceeded_fault;

pub(crate) mod shape_snapshot_schedule_already_exists_fault;

pub(crate) mod shape_snapshot_schedule_not_found_fault;

pub(crate) mod shape_snapshot_schedule_quota_exceeded_fault;

pub(crate) mod shape_snapshot_schedule_update_in_progress_fault;

pub(crate) mod shape_snapshot_sorting_entity;

pub(crate) mod shape_sns_invalid_topic_fault;

pub(crate) mod shape_sns_no_authorization_fault;

pub(crate) mod shape_sns_topic_arn_not_found_fault;

pub(crate) mod shape_source_not_found_fault;

pub(crate) mod shape_subnet_already_in_use;

pub(crate) mod shape_subscription_already_exist_fault;

pub(crate) mod shape_subscription_category_not_found_fault;

pub(crate) mod shape_subscription_event_id_not_found_fault;

pub(crate) mod shape_subscription_not_found_fault;

pub(crate) mod shape_subscription_severity_not_found_fault;

pub(crate) mod shape_table_limit_exceeded_fault;

pub(crate) mod shape_table_restore_not_found_fault;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_limit_exceeded_fault;

pub(crate) mod shape_unauthorized_operation;

pub(crate) mod shape_unauthorized_partner_integration_fault;

pub(crate) mod shape_unknown_snapshot_copy_region_fault;

pub(crate) mod shape_unsupported_operation_fault;

pub(crate) mod shape_unsupported_option_fault;

pub(crate) mod shape_usage_limit_already_exists_fault;

pub(crate) mod shape_usage_limit_not_found_fault;

pub(crate) mod shape_aqua_configuration;

pub(crate) mod shape_associated_cluster_list;

pub(crate) mod shape_association_list;

pub(crate) mod shape_attribute_list;

pub(crate) mod shape_authentication_profile_list;

pub(crate) mod shape_batch_snapshot_operation_error_list;

pub(crate) mod shape_batch_snapshot_operation_errors;

pub(crate) mod shape_cluster;

pub(crate) mod shape_cluster_db_revisions_list;

pub(crate) mod shape_cluster_list;

pub(crate) mod shape_cluster_parameter_group;

pub(crate) mod shape_cluster_security_group;

pub(crate) mod shape_cluster_security_groups;

pub(crate) mod shape_cluster_subnet_group;

pub(crate) mod shape_cluster_subnet_groups;

pub(crate) mod shape_cluster_version_list;

pub(crate) mod shape_data_share_association_list;

pub(crate) mod shape_data_share_list;

pub(crate) mod shape_default_cluster_parameters;

pub(crate) mod shape_encryption_context_map;

pub(crate) mod shape_endpoint_accesses;

pub(crate) mod shape_endpoint_authorizations;

pub(crate) mod shape_event_categories_map_list;

pub(crate) mod shape_event_list;

pub(crate) mod shape_event_subscription;

pub(crate) mod shape_event_subscriptions_list;

pub(crate) mod shape_hsm_client_certificate;

pub(crate) mod shape_hsm_client_certificate_list;

pub(crate) mod shape_hsm_configuration;

pub(crate) mod shape_hsm_configuration_list;

pub(crate) mod shape_import_tables_completed;

pub(crate) mod shape_import_tables_in_progress;

pub(crate) mod shape_import_tables_not_started;

pub(crate) mod shape_inbound_integration_list;

pub(crate) mod shape_integration_error_list;

pub(crate) mod shape_integration_list;

pub(crate) mod shape_lake_formation_scope_union;

pub(crate) mod shape_log_type_list;

pub(crate) mod shape_node_configuration_option_list;

pub(crate) mod shape_orderable_cluster_options_list;

pub(crate) mod shape_parameter_group_list;

pub(crate) mod shape_parameters_list;

pub(crate) mod shape_partner_integration_info_list;

pub(crate) mod shape_pause_cluster_message;

pub(crate) mod shape_provisioned_identifier;

pub(crate) mod shape_recommendation_list;

pub(crate) mod shape_redshift_idc_application;

pub(crate) mod shape_redshift_idc_application_list;

pub(crate) mod shape_reserved_node;

pub(crate) mod shape_reserved_node_configuration_option_list;

pub(crate) mod shape_reserved_node_exchange_status_list;

pub(crate) mod shape_reserved_node_list;

pub(crate) mod shape_reserved_node_offering_list;

pub(crate) mod shape_resize_cluster_message;

pub(crate) mod shape_resource_policy;

pub(crate) mod shape_resume_cluster_message;

pub(crate) mod shape_s3_access_grants_scope_union;

pub(crate) mod shape_schedule_definition_list;

pub(crate) mod shape_scheduled_action_list;

pub(crate) mod shape_scheduled_action_time_list;

pub(crate) mod shape_scheduled_snapshot_time_list;

pub(crate) mod shape_serverless_identifier;

pub(crate) mod shape_snapshot;

pub(crate) mod shape_snapshot_copy_grant;

pub(crate) mod shape_snapshot_copy_grant_list;

pub(crate) mod shape_snapshot_identifier_list;

pub(crate) mod shape_snapshot_list;

pub(crate) mod shape_snapshot_schedule_list;

pub(crate) mod shape_table_restore_status;

pub(crate) mod shape_table_restore_status_list;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_tagged_resource_list;

pub(crate) mod shape_track_list;

pub(crate) mod shape_usage_limits;

pub(crate) mod shape_vpc_endpoint;

pub(crate) mod shape_vpc_identifier_list;

pub(crate) mod shape_vpc_security_group_membership_list;

pub(crate) mod shape_account_attribute;

pub(crate) mod shape_accounts_with_restore_access_list;

pub(crate) mod shape_association;

pub(crate) mod shape_authentication_profile;

pub(crate) mod shape_authorized_token_issuer_list;

pub(crate) mod shape_cluster_associated_to_schedule;

pub(crate) mod shape_cluster_db_revision;

pub(crate) mod shape_cluster_iam_role_list;

pub(crate) mod shape_cluster_nodes_list;

pub(crate) mod shape_cluster_parameter_group_status_list;

pub(crate) mod shape_cluster_security_group_membership_list;

pub(crate) mod shape_cluster_snapshot_copy_status;

pub(crate) mod shape_cluster_version;

pub(crate) mod shape_data_share;

pub(crate) mod shape_data_share_association;

pub(crate) mod shape_data_transfer_progress;

pub(crate) mod shape_deferred_maintenance_windows_list;

pub(crate) mod shape_ec2_security_group_list;

pub(crate) mod shape_elastic_ip_status;

pub(crate) mod shape_endpoint;

pub(crate) mod shape_endpoint_access;

pub(crate) mod shape_endpoint_authorization;

pub(crate) mod shape_event;

pub(crate) mod shape_event_categories_list;

pub(crate) mod shape_event_categories_map;

pub(crate) mod shape_hsm_status;

pub(crate) mod shape_inbound_integration;

pub(crate) mod shape_integration;

pub(crate) mod shape_integration_error;

pub(crate) mod shape_ip_range_list;

pub(crate) mod shape_lake_formation_query;

pub(crate) mod shape_maintenance_track;

pub(crate) mod shape_network_interface_list;

pub(crate) mod shape_node_configuration_option;

pub(crate) mod shape_orderable_cluster_option;

pub(crate) mod shape_partner_integration_info;

pub(crate) mod shape_pending_actions_list;

pub(crate) mod shape_pending_modified_values;

pub(crate) mod shape_read_write_access;

pub(crate) mod shape_recommendation;

pub(crate) mod shape_recurring_charge_list;

pub(crate) mod shape_reserved_node_configuration_option;

pub(crate) mod shape_reserved_node_exchange_status;

pub(crate) mod shape_reserved_node_offering;

pub(crate) mod shape_resize_info;

pub(crate) mod shape_restorable_node_type_list;

pub(crate) mod shape_restore_status;

pub(crate) mod shape_scheduled_action;

pub(crate) mod shape_secondary_cluster_info;

pub(crate) mod shape_service_integration_list;

pub(crate) mod shape_snapshot_error_message;

pub(crate) mod shape_snapshot_schedule;

pub(crate) mod shape_source_ids_list;

pub(crate) mod shape_subnet_list;

pub(crate) mod shape_tagged_resource;

pub(crate) mod shape_usage_limit;

pub(crate) mod shape_value_string_list;

pub(crate) mod shape_vpc_security_group_membership;

pub(crate) mod shape_account_with_restore_access;

pub(crate) mod shape_attribute_value_list;

pub(crate) mod shape_availability_zone_list;

pub(crate) mod shape_certificate_association_list;

pub(crate) mod shape_cluster_iam_role;

pub(crate) mod shape_cluster_node;

pub(crate) mod shape_cluster_parameter_group_status;

pub(crate) mod shape_cluster_security_group_membership;

pub(crate) mod shape_deferred_maintenance_window;

pub(crate) mod shape_ec2_security_group;

pub(crate) mod shape_eligible_tracks_to_update_list;

pub(crate) mod shape_event_info_map_list;

pub(crate) mod shape_ip_range;

pub(crate) mod shape_network_interface;

pub(crate) mod shape_recommended_action_list;

pub(crate) mod shape_recurring_charge;

pub(crate) mod shape_reference_link_list;

pub(crate) mod shape_revision_targets_list;

pub(crate) mod shape_subnet;

pub(crate) mod shape_vpc_endpoints_list;

pub(crate) mod shape_attribute_value_target;

pub(crate) mod shape_authorized_audience_list;

pub(crate) mod shape_availability_zone;

pub(crate) mod shape_certificate_association;

pub(crate) mod shape_cluster_parameter_status_list;

pub(crate) mod shape_event_info_map;

pub(crate) mod shape_lake_formation_service_integrations;

pub(crate) mod shape_recommended_action;

pub(crate) mod shape_reference_link;

pub(crate) mod shape_revision_target;

pub(crate) mod shape_s3_access_grants_service_integrations;

pub(crate) mod shape_update_target;

pub(crate) mod shape_cluster_parameter_status;

pub(crate) mod shape_supported_operation_list;

pub(crate) mod shape_supported_platforms_list;

pub(crate) mod shape_supported_operation;

pub(crate) mod shape_supported_platform;
