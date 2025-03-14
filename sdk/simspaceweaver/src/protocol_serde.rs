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

pub(crate) mod shape_create_snapshot;

pub(crate) mod shape_delete_app;

pub(crate) mod shape_delete_simulation;

pub(crate) mod shape_describe_app;

pub(crate) mod shape_describe_simulation;

pub(crate) mod shape_list_apps;

pub(crate) mod shape_list_simulations;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_start_app;

pub(crate) mod shape_start_clock;

pub(crate) mod shape_start_simulation;

pub(crate) mod shape_stop_app;

pub(crate) mod shape_stop_clock;

pub(crate) mod shape_stop_simulation;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_snapshot_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_app_input;

pub(crate) mod shape_start_clock_input;

pub(crate) mod shape_start_simulation_input;

pub(crate) mod shape_stop_app_input;

pub(crate) mod shape_stop_clock_input;

pub(crate) mod shape_stop_simulation_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_too_many_tags_exception;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_launch_overrides;

pub(crate) mod shape_live_simulation_state;

pub(crate) mod shape_logging_configuration;

pub(crate) mod shape_s3_destination;

pub(crate) mod shape_s3_location;

pub(crate) mod shape_simulation_app_endpoint_info;

pub(crate) mod shape_simulation_app_list;

pub(crate) mod shape_simulation_list;

pub(crate) mod shape_tag_map;

pub(crate) mod shape_app_port_mappings;

pub(crate) mod shape_domain_list;

pub(crate) mod shape_launch_command_list;

pub(crate) mod shape_log_destinations;

pub(crate) mod shape_simulation_app_metadata;

pub(crate) mod shape_simulation_clock_list;

pub(crate) mod shape_simulation_metadata;

pub(crate) mod shape_domain;

pub(crate) mod shape_log_destination;

pub(crate) mod shape_simulation_app_port_mapping;

pub(crate) mod shape_simulation_clock;

pub(crate) mod shape_cloud_watch_logs_log_group;
