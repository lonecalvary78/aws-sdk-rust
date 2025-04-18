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

pub(crate) mod shape_associate_web_acl;

pub(crate) mod shape_create_byte_match_set;

pub(crate) mod shape_create_geo_match_set;

pub(crate) mod shape_create_ip_set;

pub(crate) mod shape_create_rate_based_rule;

pub(crate) mod shape_create_regex_match_set;

pub(crate) mod shape_create_regex_pattern_set;

pub(crate) mod shape_create_rule;

pub(crate) mod shape_create_rule_group;

pub(crate) mod shape_create_size_constraint_set;

pub(crate) mod shape_create_sql_injection_match_set;

pub(crate) mod shape_create_web_acl;

pub(crate) mod shape_create_web_acl_migration_stack;

pub(crate) mod shape_create_xss_match_set;

pub(crate) mod shape_delete_byte_match_set;

pub(crate) mod shape_delete_geo_match_set;

pub(crate) mod shape_delete_ip_set;

pub(crate) mod shape_delete_logging_configuration;

pub(crate) mod shape_delete_permission_policy;

pub(crate) mod shape_delete_rate_based_rule;

pub(crate) mod shape_delete_regex_match_set;

pub(crate) mod shape_delete_regex_pattern_set;

pub(crate) mod shape_delete_rule;

pub(crate) mod shape_delete_rule_group;

pub(crate) mod shape_delete_size_constraint_set;

pub(crate) mod shape_delete_sql_injection_match_set;

pub(crate) mod shape_delete_web_acl;

pub(crate) mod shape_delete_xss_match_set;

pub(crate) mod shape_disassociate_web_acl;

pub(crate) mod shape_get_byte_match_set;

pub(crate) mod shape_get_change_token;

pub(crate) mod shape_get_change_token_status;

pub(crate) mod shape_get_geo_match_set;

pub(crate) mod shape_get_ip_set;

pub(crate) mod shape_get_logging_configuration;

pub(crate) mod shape_get_permission_policy;

pub(crate) mod shape_get_rate_based_rule;

pub(crate) mod shape_get_rate_based_rule_managed_keys;

pub(crate) mod shape_get_regex_match_set;

pub(crate) mod shape_get_regex_pattern_set;

pub(crate) mod shape_get_rule;

pub(crate) mod shape_get_rule_group;

pub(crate) mod shape_get_sampled_requests;

pub(crate) mod shape_get_size_constraint_set;

pub(crate) mod shape_get_sql_injection_match_set;

pub(crate) mod shape_get_web_acl;

pub(crate) mod shape_get_web_acl_for_resource;

pub(crate) mod shape_get_xss_match_set;

pub(crate) mod shape_list_activated_rules_in_rule_group;

pub(crate) mod shape_list_byte_match_sets;

pub(crate) mod shape_list_geo_match_sets;

pub(crate) mod shape_list_ip_sets;

pub(crate) mod shape_list_logging_configurations;

pub(crate) mod shape_list_rate_based_rules;

pub(crate) mod shape_list_regex_match_sets;

pub(crate) mod shape_list_regex_pattern_sets;

pub(crate) mod shape_list_resources_for_web_acl;

pub(crate) mod shape_list_rule_groups;

pub(crate) mod shape_list_rules;

pub(crate) mod shape_list_size_constraint_sets;

pub(crate) mod shape_list_sql_injection_match_sets;

pub(crate) mod shape_list_subscribed_rule_groups;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_web_acls;

pub(crate) mod shape_list_xss_match_sets;

pub(crate) mod shape_put_logging_configuration;

pub(crate) mod shape_put_permission_policy;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_byte_match_set;

pub(crate) mod shape_update_geo_match_set;

pub(crate) mod shape_update_ip_set;

pub(crate) mod shape_update_rate_based_rule;

pub(crate) mod shape_update_regex_match_set;

pub(crate) mod shape_update_regex_pattern_set;

pub(crate) mod shape_update_rule;

pub(crate) mod shape_update_rule_group;

pub(crate) mod shape_update_size_constraint_set;

pub(crate) mod shape_update_sql_injection_match_set;

pub(crate) mod shape_update_web_acl;

pub(crate) mod shape_update_xss_match_set;

pub(crate) mod shape_associate_web_acl_input;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_create_byte_match_set_input;

pub(crate) mod shape_create_geo_match_set_input;

pub(crate) mod shape_create_ip_set_input;

pub(crate) mod shape_create_rate_based_rule_input;

pub(crate) mod shape_create_regex_match_set_input;

pub(crate) mod shape_create_regex_pattern_set_input;

pub(crate) mod shape_create_rule_group_input;

pub(crate) mod shape_create_rule_input;

pub(crate) mod shape_create_size_constraint_set_input;

pub(crate) mod shape_create_sql_injection_match_set_input;

pub(crate) mod shape_create_web_acl_input;

pub(crate) mod shape_create_web_acl_migration_stack_input;

pub(crate) mod shape_create_xss_match_set_input;

pub(crate) mod shape_delete_byte_match_set_input;

pub(crate) mod shape_delete_geo_match_set_input;

pub(crate) mod shape_delete_ip_set_input;

pub(crate) mod shape_delete_logging_configuration_input;

pub(crate) mod shape_delete_permission_policy_input;

pub(crate) mod shape_delete_rate_based_rule_input;

pub(crate) mod shape_delete_regex_match_set_input;

pub(crate) mod shape_delete_regex_pattern_set_input;

pub(crate) mod shape_delete_rule_group_input;

pub(crate) mod shape_delete_rule_input;

pub(crate) mod shape_delete_size_constraint_set_input;

pub(crate) mod shape_delete_sql_injection_match_set_input;

pub(crate) mod shape_delete_web_acl_input;

pub(crate) mod shape_delete_xss_match_set_input;

pub(crate) mod shape_disassociate_web_acl_input;

pub(crate) mod shape_get_byte_match_set_input;

pub(crate) mod shape_get_change_token_status_input;

pub(crate) mod shape_get_geo_match_set_input;

pub(crate) mod shape_get_ip_set_input;

pub(crate) mod shape_get_logging_configuration_input;

pub(crate) mod shape_get_permission_policy_input;

pub(crate) mod shape_get_rate_based_rule_input;

pub(crate) mod shape_get_rate_based_rule_managed_keys_input;

pub(crate) mod shape_get_regex_match_set_input;

pub(crate) mod shape_get_regex_pattern_set_input;

pub(crate) mod shape_get_rule_group_input;

pub(crate) mod shape_get_rule_input;

pub(crate) mod shape_get_sampled_requests_input;

pub(crate) mod shape_get_size_constraint_set_input;

pub(crate) mod shape_get_sql_injection_match_set_input;

pub(crate) mod shape_get_web_acl_for_resource_input;

pub(crate) mod shape_get_web_acl_input;

pub(crate) mod shape_get_xss_match_set_input;

pub(crate) mod shape_list_activated_rules_in_rule_group_input;

pub(crate) mod shape_list_byte_match_sets_input;

pub(crate) mod shape_list_geo_match_sets_input;

pub(crate) mod shape_list_ip_sets_input;

pub(crate) mod shape_list_logging_configurations_input;

pub(crate) mod shape_list_rate_based_rules_input;

pub(crate) mod shape_list_regex_match_sets_input;

pub(crate) mod shape_list_regex_pattern_sets_input;

pub(crate) mod shape_list_resources_for_web_acl_input;

pub(crate) mod shape_list_rule_groups_input;

pub(crate) mod shape_list_rules_input;

pub(crate) mod shape_list_size_constraint_sets_input;

pub(crate) mod shape_list_sql_injection_match_sets_input;

pub(crate) mod shape_list_subscribed_rule_groups_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_list_web_acls_input;

pub(crate) mod shape_list_xss_match_sets_input;

pub(crate) mod shape_put_logging_configuration_input;

pub(crate) mod shape_put_permission_policy_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_byte_match_set_input;

pub(crate) mod shape_update_geo_match_set_input;

pub(crate) mod shape_update_ip_set_input;

pub(crate) mod shape_update_rate_based_rule_input;

pub(crate) mod shape_update_regex_match_set_input;

pub(crate) mod shape_update_regex_pattern_set_input;

pub(crate) mod shape_update_rule_group_input;

pub(crate) mod shape_update_rule_input;

pub(crate) mod shape_update_size_constraint_set_input;

pub(crate) mod shape_update_sql_injection_match_set_input;

pub(crate) mod shape_update_web_acl_input;

pub(crate) mod shape_update_xss_match_set_input;

pub(crate) mod shape_waf_bad_request_exception;

pub(crate) mod shape_waf_disallowed_name_exception;

pub(crate) mod shape_waf_entity_migration_exception;

pub(crate) mod shape_waf_internal_error_exception;

pub(crate) mod shape_waf_invalid_account_exception;

pub(crate) mod shape_waf_invalid_operation_exception;

pub(crate) mod shape_waf_invalid_parameter_exception;

pub(crate) mod shape_waf_invalid_permission_policy_exception;

pub(crate) mod shape_waf_invalid_regex_pattern_exception;

pub(crate) mod shape_waf_limits_exceeded_exception;

pub(crate) mod shape_waf_non_empty_entity_exception;

pub(crate) mod shape_waf_nonexistent_container_exception;

pub(crate) mod shape_waf_nonexistent_item_exception;

pub(crate) mod shape_waf_referenced_item_exception;

pub(crate) mod shape_waf_service_linked_role_error_exception;

pub(crate) mod shape_waf_stale_data_exception;

pub(crate) mod shape_waf_subscription_not_found_exception;

pub(crate) mod shape_waf_tag_operation_exception;

pub(crate) mod shape_waf_tag_operation_internal_error_exception;

pub(crate) mod shape_waf_unavailable_entity_exception;

pub(crate) mod shape_activated_rules;

pub(crate) mod shape_byte_match_set;

pub(crate) mod shape_byte_match_set_summaries;

pub(crate) mod shape_byte_match_set_update;

pub(crate) mod shape_geo_match_set;

pub(crate) mod shape_geo_match_set_summaries;

pub(crate) mod shape_geo_match_set_update;

pub(crate) mod shape_ip_set;

pub(crate) mod shape_ip_set_summaries;

pub(crate) mod shape_ip_set_update;

pub(crate) mod shape_logging_configuration;

pub(crate) mod shape_logging_configurations;

pub(crate) mod shape_managed_keys;

pub(crate) mod shape_rate_based_rule;

pub(crate) mod shape_regex_match_set;

pub(crate) mod shape_regex_match_set_summaries;

pub(crate) mod shape_regex_match_set_update;

pub(crate) mod shape_regex_pattern_set;

pub(crate) mod shape_regex_pattern_set_summaries;

pub(crate) mod shape_regex_pattern_set_update;

pub(crate) mod shape_resource_arns;

pub(crate) mod shape_rule;

pub(crate) mod shape_rule_group;

pub(crate) mod shape_rule_group_summaries;

pub(crate) mod shape_rule_group_update;

pub(crate) mod shape_rule_summaries;

pub(crate) mod shape_rule_update;

pub(crate) mod shape_sampled_http_requests;

pub(crate) mod shape_size_constraint_set;

pub(crate) mod shape_size_constraint_set_summaries;

pub(crate) mod shape_size_constraint_set_update;

pub(crate) mod shape_sql_injection_match_set;

pub(crate) mod shape_sql_injection_match_set_summaries;

pub(crate) mod shape_sql_injection_match_set_update;

pub(crate) mod shape_subscribed_rule_group_summaries;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_info_for_resource;

pub(crate) mod shape_time_window;

pub(crate) mod shape_waf_action;

pub(crate) mod shape_web_acl;

pub(crate) mod shape_web_acl_summaries;

pub(crate) mod shape_web_acl_summary;

pub(crate) mod shape_web_acl_update;

pub(crate) mod shape_xss_match_set;

pub(crate) mod shape_xss_match_set_summaries;

pub(crate) mod shape_xss_match_set_update;

pub(crate) mod shape_activated_rule;

pub(crate) mod shape_byte_match_set_summary;

pub(crate) mod shape_byte_match_tuple;

pub(crate) mod shape_byte_match_tuples;

pub(crate) mod shape_field_to_match;

pub(crate) mod shape_geo_match_constraint;

pub(crate) mod shape_geo_match_constraints;

pub(crate) mod shape_geo_match_set_summary;

pub(crate) mod shape_ip_set_descriptor;

pub(crate) mod shape_ip_set_descriptors;

pub(crate) mod shape_ip_set_summary;

pub(crate) mod shape_log_destination_configs;

pub(crate) mod shape_predicate;

pub(crate) mod shape_predicates;

pub(crate) mod shape_redacted_fields;

pub(crate) mod shape_regex_match_set_summary;

pub(crate) mod shape_regex_match_tuple;

pub(crate) mod shape_regex_match_tuples;

pub(crate) mod shape_regex_pattern_set_summary;

pub(crate) mod shape_regex_pattern_strings;

pub(crate) mod shape_rule_group_summary;

pub(crate) mod shape_rule_summary;

pub(crate) mod shape_sampled_http_request;

pub(crate) mod shape_size_constraint;

pub(crate) mod shape_size_constraint_set_summary;

pub(crate) mod shape_size_constraints;

pub(crate) mod shape_sql_injection_match_set_summary;

pub(crate) mod shape_sql_injection_match_tuple;

pub(crate) mod shape_sql_injection_match_tuples;

pub(crate) mod shape_subscribed_rule_group_summary;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_xss_match_set_summary;

pub(crate) mod shape_xss_match_tuple;

pub(crate) mod shape_xss_match_tuples;

pub(crate) mod shape_excluded_rule;

pub(crate) mod shape_excluded_rules;

pub(crate) mod shape_http_request;

pub(crate) mod shape_waf_override_action;

pub(crate) mod shape_http_headers;

pub(crate) mod shape_http_header;
