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

pub(crate) mod shape_associate_delegate_to_resource;

pub(crate) mod shape_associate_member_to_group;

pub(crate) mod shape_assume_impersonation_role;

pub(crate) mod shape_cancel_mailbox_export_job;

pub(crate) mod shape_create_alias;

pub(crate) mod shape_create_availability_configuration;

pub(crate) mod shape_create_group;

pub(crate) mod shape_create_identity_center_application;

pub(crate) mod shape_create_impersonation_role;

pub(crate) mod shape_create_mobile_device_access_rule;

pub(crate) mod shape_create_organization;

pub(crate) mod shape_create_resource;

pub(crate) mod shape_create_user;

pub(crate) mod shape_delete_access_control_rule;

pub(crate) mod shape_delete_alias;

pub(crate) mod shape_delete_availability_configuration;

pub(crate) mod shape_delete_email_monitoring_configuration;

pub(crate) mod shape_delete_group;

pub(crate) mod shape_delete_identity_center_application;

pub(crate) mod shape_delete_identity_provider_configuration;

pub(crate) mod shape_delete_impersonation_role;

pub(crate) mod shape_delete_mailbox_permissions;

pub(crate) mod shape_delete_mobile_device_access_override;

pub(crate) mod shape_delete_mobile_device_access_rule;

pub(crate) mod shape_delete_organization;

pub(crate) mod shape_delete_personal_access_token;

pub(crate) mod shape_delete_resource;

pub(crate) mod shape_delete_retention_policy;

pub(crate) mod shape_delete_user;

pub(crate) mod shape_deregister_from_work_mail;

pub(crate) mod shape_deregister_mail_domain;

pub(crate) mod shape_describe_email_monitoring_configuration;

pub(crate) mod shape_describe_entity;

pub(crate) mod shape_describe_group;

pub(crate) mod shape_describe_identity_provider_configuration;

pub(crate) mod shape_describe_inbound_dmarc_settings;

pub(crate) mod shape_describe_mailbox_export_job;

pub(crate) mod shape_describe_organization;

pub(crate) mod shape_describe_resource;

pub(crate) mod shape_describe_user;

pub(crate) mod shape_disassociate_delegate_from_resource;

pub(crate) mod shape_disassociate_member_from_group;

pub(crate) mod shape_get_access_control_effect;

pub(crate) mod shape_get_default_retention_policy;

pub(crate) mod shape_get_impersonation_role;

pub(crate) mod shape_get_impersonation_role_effect;

pub(crate) mod shape_get_mail_domain;

pub(crate) mod shape_get_mailbox_details;

pub(crate) mod shape_get_mobile_device_access_effect;

pub(crate) mod shape_get_mobile_device_access_override;

pub(crate) mod shape_get_personal_access_token_metadata;

pub(crate) mod shape_list_access_control_rules;

pub(crate) mod shape_list_aliases;

pub(crate) mod shape_list_availability_configurations;

pub(crate) mod shape_list_group_members;

pub(crate) mod shape_list_groups;

pub(crate) mod shape_list_groups_for_entity;

pub(crate) mod shape_list_impersonation_roles;

pub(crate) mod shape_list_mail_domains;

pub(crate) mod shape_list_mailbox_export_jobs;

pub(crate) mod shape_list_mailbox_permissions;

pub(crate) mod shape_list_mobile_device_access_overrides;

pub(crate) mod shape_list_mobile_device_access_rules;

pub(crate) mod shape_list_organizations;

pub(crate) mod shape_list_personal_access_tokens;

pub(crate) mod shape_list_resource_delegates;

pub(crate) mod shape_list_resources;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_users;

pub(crate) mod shape_put_access_control_rule;

pub(crate) mod shape_put_email_monitoring_configuration;

pub(crate) mod shape_put_identity_provider_configuration;

pub(crate) mod shape_put_inbound_dmarc_settings;

pub(crate) mod shape_put_mailbox_permissions;

pub(crate) mod shape_put_mobile_device_access_override;

pub(crate) mod shape_put_retention_policy;

pub(crate) mod shape_register_mail_domain;

pub(crate) mod shape_register_to_work_mail;

pub(crate) mod shape_reset_password;

pub(crate) mod shape_start_mailbox_export_job;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_test_availability_configuration;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_availability_configuration;

pub(crate) mod shape_update_default_mail_domain;

pub(crate) mod shape_update_group;

pub(crate) mod shape_update_impersonation_role;

pub(crate) mod shape_update_mailbox_quota;

pub(crate) mod shape_update_mobile_device_access_rule;

pub(crate) mod shape_update_primary_email_address;

pub(crate) mod shape_update_resource;

pub(crate) mod shape_update_user;

pub(crate) mod shape_associate_delegate_to_resource_input;

pub(crate) mod shape_associate_member_to_group_input;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_assume_impersonation_role_input;

pub(crate) mod shape_cancel_mailbox_export_job_input;

pub(crate) mod shape_create_alias_input;

pub(crate) mod shape_create_availability_configuration_input;

pub(crate) mod shape_create_group_input;

pub(crate) mod shape_create_identity_center_application_input;

pub(crate) mod shape_create_impersonation_role_input;

pub(crate) mod shape_create_mobile_device_access_rule_input;

pub(crate) mod shape_create_organization_input;

pub(crate) mod shape_create_resource_input;

pub(crate) mod shape_create_user_input;

pub(crate) mod shape_delete_access_control_rule_input;

pub(crate) mod shape_delete_alias_input;

pub(crate) mod shape_delete_availability_configuration_input;

pub(crate) mod shape_delete_email_monitoring_configuration_input;

pub(crate) mod shape_delete_group_input;

pub(crate) mod shape_delete_identity_center_application_input;

pub(crate) mod shape_delete_identity_provider_configuration_input;

pub(crate) mod shape_delete_impersonation_role_input;

pub(crate) mod shape_delete_mailbox_permissions_input;

pub(crate) mod shape_delete_mobile_device_access_override_input;

pub(crate) mod shape_delete_mobile_device_access_rule_input;

pub(crate) mod shape_delete_organization_input;

pub(crate) mod shape_delete_personal_access_token_input;

pub(crate) mod shape_delete_resource_input;

pub(crate) mod shape_delete_retention_policy_input;

pub(crate) mod shape_delete_user_input;

pub(crate) mod shape_deregister_from_work_mail_input;

pub(crate) mod shape_deregister_mail_domain_input;

pub(crate) mod shape_describe_email_monitoring_configuration_input;

pub(crate) mod shape_describe_entity_input;

pub(crate) mod shape_describe_group_input;

pub(crate) mod shape_describe_identity_provider_configuration_input;

pub(crate) mod shape_describe_inbound_dmarc_settings_input;

pub(crate) mod shape_describe_mailbox_export_job_input;

pub(crate) mod shape_describe_organization_input;

pub(crate) mod shape_describe_resource_input;

pub(crate) mod shape_describe_user_input;

pub(crate) mod shape_directory_in_use_exception;

pub(crate) mod shape_directory_service_authentication_failed_exception;

pub(crate) mod shape_directory_unavailable_exception;

pub(crate) mod shape_disassociate_delegate_from_resource_input;

pub(crate) mod shape_disassociate_member_from_group_input;

pub(crate) mod shape_email_address_in_use_exception;

pub(crate) mod shape_entity_already_registered_exception;

pub(crate) mod shape_entity_not_found_exception;

pub(crate) mod shape_entity_state_exception;

pub(crate) mod shape_get_access_control_effect_input;

pub(crate) mod shape_get_default_retention_policy_input;

pub(crate) mod shape_get_impersonation_role_effect_input;

pub(crate) mod shape_get_impersonation_role_input;

pub(crate) mod shape_get_mail_domain_input;

pub(crate) mod shape_get_mailbox_details_input;

pub(crate) mod shape_get_mobile_device_access_effect_input;

pub(crate) mod shape_get_mobile_device_access_override_input;

pub(crate) mod shape_get_personal_access_token_metadata_input;

pub(crate) mod shape_invalid_configuration_exception;

pub(crate) mod shape_invalid_custom_ses_configuration_exception;

pub(crate) mod shape_invalid_parameter_exception;

pub(crate) mod shape_invalid_password_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_list_access_control_rules_input;

pub(crate) mod shape_list_aliases_input;

pub(crate) mod shape_list_availability_configurations_input;

pub(crate) mod shape_list_group_members_input;

pub(crate) mod shape_list_groups_for_entity_input;

pub(crate) mod shape_list_groups_input;

pub(crate) mod shape_list_impersonation_roles_input;

pub(crate) mod shape_list_mail_domains_input;

pub(crate) mod shape_list_mailbox_export_jobs_input;

pub(crate) mod shape_list_mailbox_permissions_input;

pub(crate) mod shape_list_mobile_device_access_overrides_input;

pub(crate) mod shape_list_mobile_device_access_rules_input;

pub(crate) mod shape_list_organizations_input;

pub(crate) mod shape_list_personal_access_tokens_input;

pub(crate) mod shape_list_resource_delegates_input;

pub(crate) mod shape_list_resources_input;

pub(crate) mod shape_list_tags_for_resource_input;

pub(crate) mod shape_list_users_input;

pub(crate) mod shape_mail_domain_in_use_exception;

pub(crate) mod shape_mail_domain_not_found_exception;

pub(crate) mod shape_mail_domain_state_exception;

pub(crate) mod shape_name_availability_exception;

pub(crate) mod shape_organization_not_found_exception;

pub(crate) mod shape_organization_state_exception;

pub(crate) mod shape_put_access_control_rule_input;

pub(crate) mod shape_put_email_monitoring_configuration_input;

pub(crate) mod shape_put_identity_provider_configuration_input;

pub(crate) mod shape_put_inbound_dmarc_settings_input;

pub(crate) mod shape_put_mailbox_permissions_input;

pub(crate) mod shape_put_mobile_device_access_override_input;

pub(crate) mod shape_put_retention_policy_input;

pub(crate) mod shape_register_mail_domain_input;

pub(crate) mod shape_register_to_work_mail_input;

pub(crate) mod shape_reserved_name_exception;

pub(crate) mod shape_reset_password_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_start_mailbox_export_job_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_test_availability_configuration_input;

pub(crate) mod shape_too_many_tags_exception;

pub(crate) mod shape_unsupported_operation_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_availability_configuration_input;

pub(crate) mod shape_update_default_mail_domain_input;

pub(crate) mod shape_update_group_input;

pub(crate) mod shape_update_impersonation_role_input;

pub(crate) mod shape_update_mailbox_quota_input;

pub(crate) mod shape_update_mobile_device_access_rule_input;

pub(crate) mod shape_update_primary_email_address_input;

pub(crate) mod shape_update_resource_input;

pub(crate) mod shape_update_user_input;

pub(crate) mod shape_access_control_rule_name_list;

pub(crate) mod shape_access_control_rules_list;

pub(crate) mod shape_aliases;

pub(crate) mod shape_availability_configuration_list;

pub(crate) mod shape_booking_options;

pub(crate) mod shape_dns_records;

pub(crate) mod shape_domain;

pub(crate) mod shape_ews_availability_provider;

pub(crate) mod shape_folder_configuration;

pub(crate) mod shape_folder_configurations;

pub(crate) mod shape_group_identifiers;

pub(crate) mod shape_groups;

pub(crate) mod shape_identity_center_configuration;

pub(crate) mod shape_impersonation_matched_rule_list;

pub(crate) mod shape_impersonation_role_list;

pub(crate) mod shape_impersonation_rule;

pub(crate) mod shape_impersonation_rule_list;

pub(crate) mod shape_jobs;

pub(crate) mod shape_lambda_availability_provider;

pub(crate) mod shape_list_groups_filters;

pub(crate) mod shape_list_groups_for_entity_filters;

pub(crate) mod shape_list_resources_filters;

pub(crate) mod shape_list_users_filters;

pub(crate) mod shape_mail_domains;

pub(crate) mod shape_members;

pub(crate) mod shape_mobile_device_access_matched_rule_list;

pub(crate) mod shape_mobile_device_access_overrides_list;

pub(crate) mod shape_mobile_device_access_rules_list;

pub(crate) mod shape_organization_summaries;

pub(crate) mod shape_permissions;

pub(crate) mod shape_personal_access_token_configuration;

pub(crate) mod shape_personal_access_token_scope_list;

pub(crate) mod shape_personal_access_token_summary_list;

pub(crate) mod shape_resource_delegates;

pub(crate) mod shape_resources;

pub(crate) mod shape_tag;

pub(crate) mod shape_tag_list;

pub(crate) mod shape_users;

pub(crate) mod shape_access_control_rule;

pub(crate) mod shape_availability_configuration;

pub(crate) mod shape_delegate;

pub(crate) mod shape_dns_record;

pub(crate) mod shape_group;

pub(crate) mod shape_group_identifier;

pub(crate) mod shape_impersonation_matched_rule;

pub(crate) mod shape_impersonation_role;

pub(crate) mod shape_mail_domain_summary;

pub(crate) mod shape_mailbox_export_job;

pub(crate) mod shape_member;

pub(crate) mod shape_mobile_device_access_matched_rule;

pub(crate) mod shape_mobile_device_access_override;

pub(crate) mod shape_mobile_device_access_rule;

pub(crate) mod shape_organization_summary;

pub(crate) mod shape_permission;

pub(crate) mod shape_personal_access_token_summary;

pub(crate) mod shape_resource;

pub(crate) mod shape_user;

pub(crate) mod shape_actions_list;

pub(crate) mod shape_device_model_list;

pub(crate) mod shape_device_operating_system_list;

pub(crate) mod shape_device_type_list;

pub(crate) mod shape_device_user_agent_list;

pub(crate) mod shape_impersonation_role_id_list;

pub(crate) mod shape_ip_range_list;

pub(crate) mod shape_permission_values;

pub(crate) mod shape_redacted_ews_availability_provider;

pub(crate) mod shape_target_users;

pub(crate) mod shape_user_id_list;
