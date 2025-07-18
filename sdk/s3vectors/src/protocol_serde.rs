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

pub(crate) mod shape_create_index;

pub(crate) mod shape_create_vector_bucket;

pub(crate) mod shape_delete_index;

pub(crate) mod shape_delete_vector_bucket;

pub(crate) mod shape_delete_vector_bucket_policy;

pub(crate) mod shape_delete_vectors;

pub(crate) mod shape_get_index;

pub(crate) mod shape_get_vector_bucket;

pub(crate) mod shape_get_vector_bucket_policy;

pub(crate) mod shape_get_vectors;

pub(crate) mod shape_list_indexes;

pub(crate) mod shape_list_vector_buckets;

pub(crate) mod shape_list_vectors;

pub(crate) mod shape_put_vector_bucket_policy;

pub(crate) mod shape_put_vectors;

pub(crate) mod shape_query_vectors;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_index_input;

pub(crate) mod shape_create_vector_bucket_input;

pub(crate) mod shape_delete_index_input;

pub(crate) mod shape_delete_vector_bucket_input;

pub(crate) mod shape_delete_vector_bucket_policy_input;

pub(crate) mod shape_delete_vectors_input;

pub(crate) mod shape_get_index_input;

pub(crate) mod shape_get_vector_bucket_input;

pub(crate) mod shape_get_vector_bucket_policy_input;

pub(crate) mod shape_get_vectors_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_kms_disabled_exception;

pub(crate) mod shape_kms_invalid_key_usage_exception;

pub(crate) mod shape_kms_invalid_state_exception;

pub(crate) mod shape_kms_not_found_exception;

pub(crate) mod shape_list_indexes_input;

pub(crate) mod shape_list_vector_buckets_input;

pub(crate) mod shape_list_vectors_input;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_put_vector_bucket_policy_input;

pub(crate) mod shape_put_vectors_input;

pub(crate) mod shape_query_vectors_input;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_service_unavailable_exception;

pub(crate) mod shape_too_many_requests_exception;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_encryption_configuration;

pub(crate) mod shape_get_vectors_output_list;

pub(crate) mod shape_index;

pub(crate) mod shape_list_indexes_output_list;

pub(crate) mod shape_list_vector_buckets_output_list;

pub(crate) mod shape_list_vectors_output_list;

pub(crate) mod shape_metadata_configuration;

pub(crate) mod shape_put_input_vector;

pub(crate) mod shape_query_vectors_output_list;

pub(crate) mod shape_validation_exception_field_list;

pub(crate) mod shape_vector_bucket;

pub(crate) mod shape_vector_data;

pub(crate) mod shape_get_output_vector;

pub(crate) mod shape_index_summary;

pub(crate) mod shape_list_output_vector;

pub(crate) mod shape_query_output_vector;

pub(crate) mod shape_validation_exception_field;

pub(crate) mod shape_vector_bucket_summary;

pub(crate) mod shape_non_filterable_metadata_keys;

pub(crate) mod shape_float32_vector_data;
