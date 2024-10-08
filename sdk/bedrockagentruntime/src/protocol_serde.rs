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
) -> Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_delete_agent_memory;

pub(crate) mod shape_get_agent_memory;

pub(crate) mod shape_invoke_agent;

pub(crate) mod shape_invoke_flow;

pub(crate) mod shape_retrieve;

pub(crate) mod shape_retrieve_and_generate;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_bad_gateway_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_dependency_failed_exception;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_invoke_agent_input;

pub(crate) mod shape_invoke_agent_output;

pub(crate) mod shape_invoke_flow_input;

pub(crate) mod shape_invoke_flow_output;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_retrieve_and_generate_input;

pub(crate) mod shape_retrieve_input;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_validation_exception;

pub fn parse_event_stream_error_metadata(
    payload: &::bytes::Bytes,
) -> Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(payload, &::aws_smithy_runtime_api::http::Headers::new())
}

pub(crate) mod shape_citations;

pub(crate) mod shape_flow_input;

pub(crate) mod shape_knowledge_base_query;

pub(crate) mod shape_knowledge_base_retrieval_configuration;

pub(crate) mod shape_knowledge_base_retrieval_results;

pub(crate) mod shape_memories;

pub(crate) mod shape_retrieve_and_generate_configuration;

pub(crate) mod shape_retrieve_and_generate_output;

pub(crate) mod shape_retrieve_and_generate_session_configuration;

pub(crate) mod shape_session_state;

pub(crate) mod shape_citation;

pub(crate) mod shape_external_sources_retrieve_and_generate_configuration;

pub(crate) mod shape_file_part;

pub(crate) mod shape_flow_completion_event;

pub(crate) mod shape_flow_input_content;

pub(crate) mod shape_flow_output_event;

pub(crate) mod shape_input_file;

pub(crate) mod shape_invocation_result_member;

pub(crate) mod shape_knowledge_base_configuration;

pub(crate) mod shape_knowledge_base_retrieval_result;

pub(crate) mod shape_knowledge_base_retrieve_and_generate_configuration;

pub(crate) mod shape_knowledge_base_vector_search_configuration;

pub(crate) mod shape_memory;

pub(crate) mod shape_payload_part;

pub(crate) mod shape_return_control_payload;

pub(crate) mod shape_trace_part;

pub(crate) mod shape_api_result;

pub(crate) mod shape_external_source;

pub(crate) mod shape_external_sources_generation_configuration;

pub(crate) mod shape_file_source;

pub(crate) mod shape_function_result;

pub(crate) mod shape_generated_response_part;

pub(crate) mod shape_generation_configuration;

pub(crate) mod shape_memory_session_summary;

pub(crate) mod shape_orchestration_configuration;

pub(crate) mod shape_retrieval_filter;

pub(crate) mod shape_retrieval_result_content;

pub(crate) mod shape_retrieval_result_location;

pub(crate) mod shape_retrieval_result_metadata;

pub(crate) mod shape_retrieved_references;

pub(crate) mod shape_attribution;

pub(crate) mod shape_byte_content_doc;

pub(crate) mod shape_byte_content_file;

pub(crate) mod shape_content_body;

pub(crate) mod shape_filter_attribute;

pub(crate) mod shape_flow_output_content;

pub(crate) mod shape_guardrail_configuration;

pub(crate) mod shape_inference_config;

pub(crate) mod shape_invocation_inputs;

pub(crate) mod shape_output_files;

pub(crate) mod shape_prompt_template;

pub(crate) mod shape_query_transformation_configuration;

pub(crate) mod shape_retrieval_result_confluence_location;

pub(crate) mod shape_retrieval_result_s3_location;

pub(crate) mod shape_retrieval_result_salesforce_location;

pub(crate) mod shape_retrieval_result_share_point_location;

pub(crate) mod shape_retrieval_result_web_location;

pub(crate) mod shape_retrieved_reference;

pub(crate) mod shape_s3_object_doc;

pub(crate) mod shape_s3_object_file;

pub(crate) mod shape_text_response_part;

pub(crate) mod shape_trace;

pub(crate) mod shape_failure_trace;

pub(crate) mod shape_guardrail_trace;

pub(crate) mod shape_invocation_input_member;

pub(crate) mod shape_orchestration_trace;

pub(crate) mod shape_output_file;

pub(crate) mod shape_post_processing_trace;

pub(crate) mod shape_pre_processing_trace;

pub(crate) mod shape_span;

pub(crate) mod shape_text_inference_config;

pub(crate) mod shape_api_invocation_input;

pub(crate) mod shape_function_invocation_input;

pub(crate) mod shape_guardrail_assessment_list;

pub(crate) mod shape_invocation_input;

pub(crate) mod shape_model_invocation_input;

pub(crate) mod shape_observation;

pub(crate) mod shape_orchestration_model_invocation_output;

pub(crate) mod shape_post_processing_model_invocation_output;

pub(crate) mod shape_pre_processing_model_invocation_output;

pub(crate) mod shape_rationale;

pub(crate) mod shape_action_group_invocation_input;

pub(crate) mod shape_action_group_invocation_output;

pub(crate) mod shape_api_parameters;

pub(crate) mod shape_api_request_body;

pub(crate) mod shape_code_interpreter_invocation_input;

pub(crate) mod shape_code_interpreter_invocation_output;

pub(crate) mod shape_final_response;

pub(crate) mod shape_function_parameters;

pub(crate) mod shape_guardrail_assessment;

pub(crate) mod shape_inference_configuration;

pub(crate) mod shape_knowledge_base_lookup_input;

pub(crate) mod shape_knowledge_base_lookup_output;

pub(crate) mod shape_metadata;

pub(crate) mod shape_post_processing_parsed_response;

pub(crate) mod shape_pre_processing_parsed_response;

pub(crate) mod shape_raw_response;

pub(crate) mod shape_reprompt_response;

pub(crate) mod shape_api_content_map;

pub(crate) mod shape_api_parameter;

pub(crate) mod shape_files;

pub(crate) mod shape_function_parameter;

pub(crate) mod shape_guardrail_content_policy_assessment;

pub(crate) mod shape_guardrail_sensitive_information_policy_assessment;

pub(crate) mod shape_guardrail_topic_policy_assessment;

pub(crate) mod shape_guardrail_word_policy_assessment;

pub(crate) mod shape_parameters;

pub(crate) mod shape_request_body;

pub(crate) mod shape_stop_sequences;

pub(crate) mod shape_usage;

pub(crate) mod shape_content_map;

pub(crate) mod shape_guardrail_content_filter_list;

pub(crate) mod shape_guardrail_custom_word_list;

pub(crate) mod shape_guardrail_managed_word_list;

pub(crate) mod shape_guardrail_pii_entity_filter_list;

pub(crate) mod shape_guardrail_regex_filter_list;

pub(crate) mod shape_guardrail_topic_list;

pub(crate) mod shape_parameter;

pub(crate) mod shape_property_parameters;

pub(crate) mod shape_guardrail_content_filter;

pub(crate) mod shape_guardrail_custom_word;

pub(crate) mod shape_guardrail_managed_word;

pub(crate) mod shape_guardrail_pii_entity_filter;

pub(crate) mod shape_guardrail_regex_filter;

pub(crate) mod shape_guardrail_topic;

pub(crate) mod shape_parameter_list;
