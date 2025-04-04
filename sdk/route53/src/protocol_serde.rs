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

pub(crate) mod shape_activate_key_signing_key;

pub(crate) mod shape_associate_vpc_with_hosted_zone;

pub(crate) mod shape_change_cidr_collection;

pub(crate) mod shape_change_resource_record_sets;

pub(crate) mod shape_change_tags_for_resource;

pub(crate) mod shape_create_cidr_collection;

pub(crate) mod shape_create_health_check;

pub(crate) mod shape_create_hosted_zone;

pub(crate) mod shape_create_key_signing_key;

pub(crate) mod shape_create_query_logging_config;

pub(crate) mod shape_create_reusable_delegation_set;

pub(crate) mod shape_create_traffic_policy;

pub(crate) mod shape_create_traffic_policy_instance;

pub(crate) mod shape_create_traffic_policy_version;

pub(crate) mod shape_create_vpc_association_authorization;

pub(crate) mod shape_deactivate_key_signing_key;

pub(crate) mod shape_delete_cidr_collection;

pub(crate) mod shape_delete_health_check;

pub(crate) mod shape_delete_hosted_zone;

pub(crate) mod shape_delete_key_signing_key;

pub(crate) mod shape_delete_query_logging_config;

pub(crate) mod shape_delete_reusable_delegation_set;

pub(crate) mod shape_delete_traffic_policy;

pub(crate) mod shape_delete_traffic_policy_instance;

pub(crate) mod shape_delete_vpc_association_authorization;

pub(crate) mod shape_disable_hosted_zone_dnssec;

pub(crate) mod shape_disassociate_vpc_from_hosted_zone;

pub(crate) mod shape_enable_hosted_zone_dnssec;

pub(crate) mod shape_get_account_limit;

pub(crate) mod shape_get_change;

pub(crate) mod shape_get_checker_ip_ranges;

pub(crate) mod shape_get_dnssec;

pub(crate) mod shape_get_geo_location;

pub(crate) mod shape_get_health_check;

pub(crate) mod shape_get_health_check_count;

pub(crate) mod shape_get_health_check_last_failure_reason;

pub(crate) mod shape_get_health_check_status;

pub(crate) mod shape_get_hosted_zone;

pub(crate) mod shape_get_hosted_zone_count;

pub(crate) mod shape_get_hosted_zone_limit;

pub(crate) mod shape_get_query_logging_config;

pub(crate) mod shape_get_reusable_delegation_set;

pub(crate) mod shape_get_reusable_delegation_set_limit;

pub(crate) mod shape_get_traffic_policy;

pub(crate) mod shape_get_traffic_policy_instance;

pub(crate) mod shape_get_traffic_policy_instance_count;

pub(crate) mod shape_list_cidr_blocks;

pub(crate) mod shape_list_cidr_collections;

pub(crate) mod shape_list_cidr_locations;

pub(crate) mod shape_list_geo_locations;

pub(crate) mod shape_list_health_checks;

pub(crate) mod shape_list_hosted_zones;

pub(crate) mod shape_list_hosted_zones_by_name;

pub(crate) mod shape_list_hosted_zones_by_vpc;

pub(crate) mod shape_list_query_logging_configs;

pub(crate) mod shape_list_resource_record_sets;

pub(crate) mod shape_list_reusable_delegation_sets;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_tags_for_resources;

pub(crate) mod shape_list_traffic_policies;

pub(crate) mod shape_list_traffic_policy_instances;

pub(crate) mod shape_list_traffic_policy_instances_by_hosted_zone;

pub(crate) mod shape_list_traffic_policy_instances_by_policy;

pub(crate) mod shape_list_traffic_policy_versions;

pub(crate) mod shape_list_vpc_association_authorizations;

pub(crate) mod shape_test_dns_answer;

pub(crate) mod shape_update_health_check;

pub(crate) mod shape_update_hosted_zone_comment;

pub(crate) mod shape_update_traffic_policy_comment;

pub(crate) mod shape_update_traffic_policy_instance;

pub(crate) mod shape_associate_vpc_with_hosted_zone_input;

pub(crate) mod shape_change_cidr_collection_input;

pub(crate) mod shape_change_resource_record_sets_input;

pub(crate) mod shape_change_tags_for_resource_input;

pub(crate) mod shape_cidr_block_in_use_exception;

pub(crate) mod shape_cidr_collection_already_exists_exception;

pub(crate) mod shape_cidr_collection_in_use_exception;

pub(crate) mod shape_cidr_collection_version_mismatch_exception;

pub(crate) mod shape_concurrent_modification;

pub(crate) mod shape_conflicting_domain_exists;

pub(crate) mod shape_conflicting_types;

pub(crate) mod shape_create_cidr_collection_input;

pub(crate) mod shape_create_cidr_collection_output;

pub(crate) mod shape_create_health_check_input;

pub(crate) mod shape_create_health_check_output;

pub(crate) mod shape_create_hosted_zone_input;

pub(crate) mod shape_create_hosted_zone_output;

pub(crate) mod shape_create_key_signing_key_input;

pub(crate) mod shape_create_key_signing_key_output;

pub(crate) mod shape_create_query_logging_config_input;

pub(crate) mod shape_create_query_logging_config_output;

pub(crate) mod shape_create_reusable_delegation_set_input;

pub(crate) mod shape_create_reusable_delegation_set_output;

pub(crate) mod shape_create_traffic_policy_input;

pub(crate) mod shape_create_traffic_policy_instance_input;

pub(crate) mod shape_create_traffic_policy_instance_output;

pub(crate) mod shape_create_traffic_policy_output;

pub(crate) mod shape_create_traffic_policy_version_input;

pub(crate) mod shape_create_traffic_policy_version_output;

pub(crate) mod shape_create_vpc_association_authorization_input;

pub(crate) mod shape_delegation_set_already_created;

pub(crate) mod shape_delegation_set_already_reusable;

pub(crate) mod shape_delegation_set_in_use;

pub(crate) mod shape_delegation_set_not_available;

pub(crate) mod shape_delegation_set_not_reusable;

pub(crate) mod shape_delete_vpc_association_authorization_input;

pub(crate) mod shape_disassociate_vpc_from_hosted_zone_input;

pub(crate) mod shape_dnssec_not_found;

pub(crate) mod shape_health_check_already_exists;

pub(crate) mod shape_health_check_in_use;

pub(crate) mod shape_health_check_version_mismatch;

pub(crate) mod shape_hosted_zone_already_exists;

pub(crate) mod shape_hosted_zone_not_empty;

pub(crate) mod shape_hosted_zone_not_found;

pub(crate) mod shape_hosted_zone_not_private;

pub(crate) mod shape_hosted_zone_partially_delegated;

pub(crate) mod shape_incompatible_version;

pub(crate) mod shape_insufficient_cloud_watch_logs_resource_policy;

pub(crate) mod shape_invalid_argument;

pub(crate) mod shape_invalid_change_batch;

pub(crate) mod shape_invalid_domain_name;

pub(crate) mod shape_invalid_input;

pub(crate) mod shape_invalid_key_signing_key_name;

pub(crate) mod shape_invalid_key_signing_key_status;

pub(crate) mod shape_invalid_kms_arn;

pub(crate) mod shape_invalid_pagination_token;

pub(crate) mod shape_invalid_signing_status;

pub(crate) mod shape_invalid_traffic_policy_document;

pub(crate) mod shape_invalid_vpc_id;

pub(crate) mod shape_key_signing_key_already_exists;

pub(crate) mod shape_key_signing_key_in_parent_ds_record;

pub(crate) mod shape_key_signing_key_in_use;

pub(crate) mod shape_key_signing_key_with_active_status_not_found;

pub(crate) mod shape_last_vpc_association;

pub(crate) mod shape_limits_exceeded;

pub(crate) mod shape_list_tags_for_resources_input;

pub(crate) mod shape_no_such_change;

pub(crate) mod shape_no_such_cidr_collection_exception;

pub(crate) mod shape_no_such_cidr_location_exception;

pub(crate) mod shape_no_such_cloud_watch_logs_log_group;

pub(crate) mod shape_no_such_delegation_set;

pub(crate) mod shape_no_such_geo_location;

pub(crate) mod shape_no_such_health_check;

pub(crate) mod shape_no_such_hosted_zone;

pub(crate) mod shape_no_such_key_signing_key;

pub(crate) mod shape_no_such_query_logging_config;

pub(crate) mod shape_no_such_traffic_policy;

pub(crate) mod shape_no_such_traffic_policy_instance;

pub(crate) mod shape_not_authorized_exception;

pub(crate) mod shape_prior_request_not_complete;

pub(crate) mod shape_public_zone_vpc_association;

pub(crate) mod shape_query_logging_config_already_exists;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_too_many_health_checks;

pub(crate) mod shape_too_many_hosted_zones;

pub(crate) mod shape_too_many_key_signing_keys;

pub(crate) mod shape_too_many_traffic_policies;

pub(crate) mod shape_too_many_traffic_policy_instances;

pub(crate) mod shape_too_many_traffic_policy_versions_for_current_policy;

pub(crate) mod shape_too_many_vpc_association_authorizations;

pub(crate) mod shape_traffic_policy_already_exists;

pub(crate) mod shape_traffic_policy_in_use;

pub(crate) mod shape_traffic_policy_instance_already_exists;

pub(crate) mod shape_update_health_check_input;

pub(crate) mod shape_update_hosted_zone_comment_input;

pub(crate) mod shape_update_traffic_policy_comment_input;

pub(crate) mod shape_update_traffic_policy_instance_input;

pub(crate) mod shape_vpc_association_authorization_not_found;

pub(crate) mod shape_vpc_association_not_found;

pub(crate) mod shape_account_limit;

pub(crate) mod shape_alarm_identifier;

pub(crate) mod shape_change_batch;

pub(crate) mod shape_change_info;

pub(crate) mod shape_checker_ip_ranges;

pub(crate) mod shape_cidr_block_summaries;

pub(crate) mod shape_cidr_collection;

pub(crate) mod shape_cidr_collection_change;

pub(crate) mod shape_collection_summaries;

pub(crate) mod shape_delegation_set;

pub(crate) mod shape_delegation_sets;

pub(crate) mod shape_dnssec_status;

pub(crate) mod shape_error_messages;

pub(crate) mod shape_geo_location_details;

pub(crate) mod shape_geo_location_details_list;

pub(crate) mod shape_health_check;

pub(crate) mod shape_health_check_config;

pub(crate) mod shape_health_check_observations;

pub(crate) mod shape_health_checks;

pub(crate) mod shape_hosted_zone;

pub(crate) mod shape_hosted_zone_config;

pub(crate) mod shape_hosted_zone_limit;

pub(crate) mod shape_hosted_zone_summaries;

pub(crate) mod shape_hosted_zones;

pub(crate) mod shape_key_signing_key;

pub(crate) mod shape_key_signing_keys;

pub(crate) mod shape_location_summaries;

pub(crate) mod shape_query_logging_config;

pub(crate) mod shape_query_logging_configs;

pub(crate) mod shape_record_data;

pub(crate) mod shape_resource_record_sets;

pub(crate) mod shape_resource_tag_set;

pub(crate) mod shape_resource_tag_set_list;

pub(crate) mod shape_reusable_delegation_set_limit;

pub(crate) mod shape_tag;

pub(crate) mod shape_traffic_policies;

pub(crate) mod shape_traffic_policy;

pub(crate) mod shape_traffic_policy_instance;

pub(crate) mod shape_traffic_policy_instances;

pub(crate) mod shape_traffic_policy_summaries;

pub(crate) mod shape_vpc;

pub(crate) mod shape_vpcs;

pub(crate) mod shape_change;

pub(crate) mod shape_cidr_block_summary;

pub(crate) mod shape_cloud_watch_alarm_configuration;

pub(crate) mod shape_collection_summary;

pub(crate) mod shape_delegation_set_name_servers;

pub(crate) mod shape_health_check_observation;

pub(crate) mod shape_hosted_zone_summary;

pub(crate) mod shape_linked_service;

pub(crate) mod shape_location_summary;

pub(crate) mod shape_resource_record_set;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_traffic_policy_summary;

pub(crate) mod shape_alias_target;

pub(crate) mod shape_child_health_check_list;

pub(crate) mod shape_cidr_routing_config;

pub(crate) mod shape_dimension_list;

pub(crate) mod shape_geo_location;

pub(crate) mod shape_geo_proximity_location;

pub(crate) mod shape_health_check_region_list;

pub(crate) mod shape_hosted_zone_owner;

pub(crate) mod shape_resource_records;

pub(crate) mod shape_status_report;

pub(crate) mod shape_coordinates;

pub(crate) mod shape_dimension;

pub(crate) mod shape_resource_record;
