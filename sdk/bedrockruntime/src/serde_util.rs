// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn apply_guardrail_output_output_correct_errors(
    mut builder: crate::operation::apply_guardrail::builders::ApplyGuardrailOutputBuilder,
) -> crate::operation::apply_guardrail::builders::ApplyGuardrailOutputBuilder {
    if builder.usage.is_none() {
        builder.usage = {
            let builder = crate::types::builders::GuardrailUsageBuilder::default();
            crate::serde_util::guardrail_usage_correct_errors(builder).build().ok()
        }
    }
    if builder.action.is_none() {
        builder.action = "no value was set".parse::<crate::types::GuardrailAction>().ok()
    }
    if builder.outputs.is_none() {
        builder.outputs = Some(Default::default())
    }
    if builder.assessments.is_none() {
        builder.assessments = Some(Default::default())
    }
    builder
}

pub(crate) fn converse_output_output_correct_errors(
    mut builder: crate::operation::converse::builders::ConverseOutputBuilder,
) -> crate::operation::converse::builders::ConverseOutputBuilder {
    if builder.output.is_none() {
        builder.output = Some(crate::types::ConverseOutput::Unknown)
    }
    if builder.stop_reason.is_none() {
        builder.stop_reason = "no value was set".parse::<crate::types::StopReason>().ok()
    }
    if builder.usage.is_none() {
        builder.usage = {
            let builder = crate::types::builders::TokenUsageBuilder::default();
            crate::serde_util::token_usage_correct_errors(builder).build().ok()
        }
    }
    if builder.metrics.is_none() {
        builder.metrics = {
            let builder = crate::types::builders::ConverseMetricsBuilder::default();
            crate::serde_util::converse_metrics_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn get_async_invoke_output_output_correct_errors(
    mut builder: crate::operation::get_async_invoke::builders::GetAsyncInvokeOutputBuilder,
) -> crate::operation::get_async_invoke::builders::GetAsyncInvokeOutputBuilder {
    if builder.invocation_arn.is_none() {
        builder.invocation_arn = Some(Default::default())
    }
    if builder.model_arn.is_none() {
        builder.model_arn = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::AsyncInvokeStatus>().ok()
    }
    if builder.submit_time.is_none() {
        builder.submit_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.output_data_config.is_none() {
        builder.output_data_config = Some(crate::types::AsyncInvokeOutputDataConfig::Unknown)
    }
    builder
}

pub(crate) fn invoke_model_output_output_correct_errors(
    mut builder: crate::operation::invoke_model::builders::InvokeModelOutputBuilder,
) -> crate::operation::invoke_model::builders::InvokeModelOutputBuilder {
    if builder.body.is_none() {
        builder.body = Some(::aws_smithy_types::Blob::new(""))
    }
    if builder.content_type.is_none() {
        builder.content_type = Some(Default::default())
    }
    builder
}

pub(crate) fn invoke_model_with_response_stream_output_output_correct_errors(
    mut builder: crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamOutputBuilder,
) -> crate::operation::invoke_model_with_response_stream::builders::InvokeModelWithResponseStreamOutputBuilder {
    if builder.content_type.is_none() {
        builder.content_type = Some(Default::default())
    }
    builder
}

pub(crate) fn start_async_invoke_output_output_correct_errors(
    mut builder: crate::operation::start_async_invoke::builders::StartAsyncInvokeOutputBuilder,
) -> crate::operation::start_async_invoke::builders::StartAsyncInvokeOutputBuilder {
    if builder.invocation_arn.is_none() {
        builder.invocation_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn guardrail_usage_correct_errors(
    mut builder: crate::types::builders::GuardrailUsageBuilder,
) -> crate::types::builders::GuardrailUsageBuilder {
    if builder.topic_policy_units.is_none() {
        builder.topic_policy_units = Some(Default::default())
    }
    if builder.content_policy_units.is_none() {
        builder.content_policy_units = Some(Default::default())
    }
    if builder.word_policy_units.is_none() {
        builder.word_policy_units = Some(Default::default())
    }
    if builder.sensitive_information_policy_units.is_none() {
        builder.sensitive_information_policy_units = Some(Default::default())
    }
    if builder.sensitive_information_policy_free_units.is_none() {
        builder.sensitive_information_policy_free_units = Some(Default::default())
    }
    if builder.contextual_grounding_policy_units.is_none() {
        builder.contextual_grounding_policy_units = Some(Default::default())
    }
    builder
}

pub(crate) fn token_usage_correct_errors(mut builder: crate::types::builders::TokenUsageBuilder) -> crate::types::builders::TokenUsageBuilder {
    if builder.input_tokens.is_none() {
        builder.input_tokens = Some(Default::default())
    }
    if builder.output_tokens.is_none() {
        builder.output_tokens = Some(Default::default())
    }
    if builder.total_tokens.is_none() {
        builder.total_tokens = Some(Default::default())
    }
    builder
}

pub(crate) fn converse_metrics_correct_errors(
    mut builder: crate::types::builders::ConverseMetricsBuilder,
) -> crate::types::builders::ConverseMetricsBuilder {
    if builder.latency_ms.is_none() {
        builder.latency_ms = Some(Default::default())
    }
    builder
}

pub(crate) fn async_invoke_s3_output_data_config_correct_errors(
    mut builder: crate::types::builders::AsyncInvokeS3OutputDataConfigBuilder,
) -> crate::types::builders::AsyncInvokeS3OutputDataConfigBuilder {
    if builder.s3_uri.is_none() {
        builder.s3_uri = Some(Default::default())
    }
    builder
}

pub(crate) fn async_invoke_summary_correct_errors(
    mut builder: crate::types::builders::AsyncInvokeSummaryBuilder,
) -> crate::types::builders::AsyncInvokeSummaryBuilder {
    if builder.invocation_arn.is_none() {
        builder.invocation_arn = Some(Default::default())
    }
    if builder.model_arn.is_none() {
        builder.model_arn = Some(Default::default())
    }
    if builder.submit_time.is_none() {
        builder.submit_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.output_data_config.is_none() {
        builder.output_data_config = Some(crate::types::AsyncInvokeOutputDataConfig::Unknown)
    }
    builder
}

pub(crate) fn message_correct_errors(mut builder: crate::types::builders::MessageBuilder) -> crate::types::builders::MessageBuilder {
    if builder.role.is_none() {
        builder.role = "no value was set".parse::<crate::types::ConversationRole>().ok()
    }
    if builder.content.is_none() {
        builder.content = Some(Default::default())
    }
    builder
}

pub(crate) fn content_block_delta_event_correct_errors(
    mut builder: crate::types::builders::ContentBlockDeltaEventBuilder,
) -> crate::types::builders::ContentBlockDeltaEventBuilder {
    if builder.delta.is_none() {
        builder.delta = Some(crate::types::ContentBlockDelta::Unknown)
    }
    if builder.content_block_index.is_none() {
        builder.content_block_index = Some(Default::default())
    }
    builder
}

pub(crate) fn content_block_start_event_correct_errors(
    mut builder: crate::types::builders::ContentBlockStartEventBuilder,
) -> crate::types::builders::ContentBlockStartEventBuilder {
    if builder.start.is_none() {
        builder.start = Some(crate::types::ContentBlockStart::Unknown)
    }
    if builder.content_block_index.is_none() {
        builder.content_block_index = Some(Default::default())
    }
    builder
}

pub(crate) fn content_block_stop_event_correct_errors(
    mut builder: crate::types::builders::ContentBlockStopEventBuilder,
) -> crate::types::builders::ContentBlockStopEventBuilder {
    if builder.content_block_index.is_none() {
        builder.content_block_index = Some(Default::default())
    }
    builder
}

pub(crate) fn converse_stream_metadata_event_correct_errors(
    mut builder: crate::types::builders::ConverseStreamMetadataEventBuilder,
) -> crate::types::builders::ConverseStreamMetadataEventBuilder {
    if builder.usage.is_none() {
        builder.usage = {
            let builder = crate::types::builders::TokenUsageBuilder::default();
            crate::serde_util::token_usage_correct_errors(builder).build().ok()
        }
    }
    if builder.metrics.is_none() {
        builder.metrics = {
            let builder = crate::types::builders::ConverseStreamMetricsBuilder::default();
            crate::serde_util::converse_stream_metrics_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn guardrail_content_policy_assessment_correct_errors(
    mut builder: crate::types::builders::GuardrailContentPolicyAssessmentBuilder,
) -> crate::types::builders::GuardrailContentPolicyAssessmentBuilder {
    if builder.filters.is_none() {
        builder.filters = Some(Default::default())
    }
    builder
}

pub(crate) fn guardrail_sensitive_information_policy_assessment_correct_errors(
    mut builder: crate::types::builders::GuardrailSensitiveInformationPolicyAssessmentBuilder,
) -> crate::types::builders::GuardrailSensitiveInformationPolicyAssessmentBuilder {
    if builder.pii_entities.is_none() {
        builder.pii_entities = Some(Default::default())
    }
    if builder.regexes.is_none() {
        builder.regexes = Some(Default::default())
    }
    builder
}

pub(crate) fn guardrail_topic_policy_assessment_correct_errors(
    mut builder: crate::types::builders::GuardrailTopicPolicyAssessmentBuilder,
) -> crate::types::builders::GuardrailTopicPolicyAssessmentBuilder {
    if builder.topics.is_none() {
        builder.topics = Some(Default::default())
    }
    builder
}

pub(crate) fn guardrail_word_policy_assessment_correct_errors(
    mut builder: crate::types::builders::GuardrailWordPolicyAssessmentBuilder,
) -> crate::types::builders::GuardrailWordPolicyAssessmentBuilder {
    if builder.custom_words.is_none() {
        builder.custom_words = Some(Default::default())
    }
    if builder.managed_word_lists.is_none() {
        builder.managed_word_lists = Some(Default::default())
    }
    builder
}

pub(crate) fn message_start_event_correct_errors(
    mut builder: crate::types::builders::MessageStartEventBuilder,
) -> crate::types::builders::MessageStartEventBuilder {
    if builder.role.is_none() {
        builder.role = "no value was set".parse::<crate::types::ConversationRole>().ok()
    }
    builder
}

pub(crate) fn message_stop_event_correct_errors(
    mut builder: crate::types::builders::MessageStopEventBuilder,
) -> crate::types::builders::MessageStopEventBuilder {
    if builder.stop_reason.is_none() {
        builder.stop_reason = "no value was set".parse::<crate::types::StopReason>().ok()
    }
    builder
}

pub(crate) fn converse_stream_metrics_correct_errors(
    mut builder: crate::types::builders::ConverseStreamMetricsBuilder,
) -> crate::types::builders::ConverseStreamMetricsBuilder {
    if builder.latency_ms.is_none() {
        builder.latency_ms = Some(Default::default())
    }
    builder
}

pub(crate) fn cache_point_block_correct_errors(
    mut builder: crate::types::builders::CachePointBlockBuilder,
) -> crate::types::builders::CachePointBlockBuilder {
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::CachePointType>().ok()
    }
    builder
}

pub(crate) fn document_block_correct_errors(
    mut builder: crate::types::builders::DocumentBlockBuilder,
) -> crate::types::builders::DocumentBlockBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.source.is_none() {
        builder.source = Some(crate::types::DocumentSource::Unknown)
    }
    builder
}

pub(crate) fn guardrail_content_filter_correct_errors(
    mut builder: crate::types::builders::GuardrailContentFilterBuilder,
) -> crate::types::builders::GuardrailContentFilterBuilder {
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::GuardrailContentFilterType>().ok()
    }
    if builder.confidence.is_none() {
        builder.confidence = "no value was set".parse::<crate::types::GuardrailContentFilterConfidence>().ok()
    }
    if builder.action.is_none() {
        builder.action = "no value was set".parse::<crate::types::GuardrailContentPolicyAction>().ok()
    }
    builder
}

pub(crate) fn guardrail_contextual_grounding_filter_correct_errors(
    mut builder: crate::types::builders::GuardrailContextualGroundingFilterBuilder,
) -> crate::types::builders::GuardrailContextualGroundingFilterBuilder {
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::GuardrailContextualGroundingFilterType>().ok()
    }
    if builder.threshold.is_none() {
        builder.threshold = Some(Default::default())
    }
    if builder.score.is_none() {
        builder.score = Some(Default::default())
    }
    if builder.action.is_none() {
        builder.action = "no value was set".parse::<crate::types::GuardrailContextualGroundingPolicyAction>().ok()
    }
    builder
}

pub(crate) fn guardrail_custom_word_correct_errors(
    mut builder: crate::types::builders::GuardrailCustomWordBuilder,
) -> crate::types::builders::GuardrailCustomWordBuilder {
    if builder.r#match.is_none() {
        builder.r#match = Some(Default::default())
    }
    if builder.action.is_none() {
        builder.action = "no value was set".parse::<crate::types::GuardrailWordPolicyAction>().ok()
    }
    builder
}

pub(crate) fn guardrail_managed_word_correct_errors(
    mut builder: crate::types::builders::GuardrailManagedWordBuilder,
) -> crate::types::builders::GuardrailManagedWordBuilder {
    if builder.r#match.is_none() {
        builder.r#match = Some(Default::default())
    }
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::GuardrailManagedWordType>().ok()
    }
    if builder.action.is_none() {
        builder.action = "no value was set".parse::<crate::types::GuardrailWordPolicyAction>().ok()
    }
    builder
}

pub(crate) fn guardrail_pii_entity_filter_correct_errors(
    mut builder: crate::types::builders::GuardrailPiiEntityFilterBuilder,
) -> crate::types::builders::GuardrailPiiEntityFilterBuilder {
    if builder.r#match.is_none() {
        builder.r#match = Some(Default::default())
    }
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::GuardrailPiiEntityType>().ok()
    }
    if builder.action.is_none() {
        builder.action = "no value was set".parse::<crate::types::GuardrailSensitiveInformationPolicyAction>().ok()
    }
    builder
}

pub(crate) fn guardrail_regex_filter_correct_errors(
    mut builder: crate::types::builders::GuardrailRegexFilterBuilder,
) -> crate::types::builders::GuardrailRegexFilterBuilder {
    if builder.action.is_none() {
        builder.action = "no value was set".parse::<crate::types::GuardrailSensitiveInformationPolicyAction>().ok()
    }
    builder
}

pub(crate) fn guardrail_topic_correct_errors(
    mut builder: crate::types::builders::GuardrailTopicBuilder,
) -> crate::types::builders::GuardrailTopicBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::GuardrailTopicType>().ok()
    }
    if builder.action.is_none() {
        builder.action = "no value was set".parse::<crate::types::GuardrailTopicPolicyAction>().ok()
    }
    builder
}

pub(crate) fn image_block_correct_errors(mut builder: crate::types::builders::ImageBlockBuilder) -> crate::types::builders::ImageBlockBuilder {
    if builder.format.is_none() {
        builder.format = "no value was set".parse::<crate::types::ImageFormat>().ok()
    }
    if builder.source.is_none() {
        builder.source = Some(crate::types::ImageSource::Unknown)
    }
    builder
}

pub(crate) fn tool_result_block_correct_errors(
    mut builder: crate::types::builders::ToolResultBlockBuilder,
) -> crate::types::builders::ToolResultBlockBuilder {
    if builder.tool_use_id.is_none() {
        builder.tool_use_id = Some(Default::default())
    }
    if builder.content.is_none() {
        builder.content = Some(Default::default())
    }
    builder
}

pub(crate) fn tool_use_block_correct_errors(mut builder: crate::types::builders::ToolUseBlockBuilder) -> crate::types::builders::ToolUseBlockBuilder {
    if builder.tool_use_id.is_none() {
        builder.tool_use_id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.input.is_none() {
        builder.input = Some(Default::default())
    }
    builder
}

pub(crate) fn tool_use_block_delta_correct_errors(
    mut builder: crate::types::builders::ToolUseBlockDeltaBuilder,
) -> crate::types::builders::ToolUseBlockDeltaBuilder {
    if builder.input.is_none() {
        builder.input = Some(Default::default())
    }
    builder
}

pub(crate) fn tool_use_block_start_correct_errors(
    mut builder: crate::types::builders::ToolUseBlockStartBuilder,
) -> crate::types::builders::ToolUseBlockStartBuilder {
    if builder.tool_use_id.is_none() {
        builder.tool_use_id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    builder
}

pub(crate) fn video_block_correct_errors(mut builder: crate::types::builders::VideoBlockBuilder) -> crate::types::builders::VideoBlockBuilder {
    if builder.format.is_none() {
        builder.format = "no value was set".parse::<crate::types::VideoFormat>().ok()
    }
    if builder.source.is_none() {
        builder.source = Some(crate::types::VideoSource::Unknown)
    }
    builder
}

pub(crate) fn citations_config_correct_errors(
    mut builder: crate::types::builders::CitationsConfigBuilder,
) -> crate::types::builders::CitationsConfigBuilder {
    if builder.enabled.is_none() {
        builder.enabled = Some(Default::default())
    }
    builder
}

pub(crate) fn guardrail_converse_image_block_correct_errors(
    mut builder: crate::types::builders::GuardrailConverseImageBlockBuilder,
) -> crate::types::builders::GuardrailConverseImageBlockBuilder {
    if builder.format.is_none() {
        builder.format = "no value was set".parse::<crate::types::GuardrailConverseImageFormat>().ok()
    }
    if builder.source.is_none() {
        builder.source = Some(crate::types::GuardrailConverseImageSource::Unknown)
    }
    builder
}

pub(crate) fn guardrail_converse_text_block_correct_errors(
    mut builder: crate::types::builders::GuardrailConverseTextBlockBuilder,
) -> crate::types::builders::GuardrailConverseTextBlockBuilder {
    if builder.text.is_none() {
        builder.text = Some(Default::default())
    }
    builder
}

pub(crate) fn reasoning_text_block_correct_errors(
    mut builder: crate::types::builders::ReasoningTextBlockBuilder,
) -> crate::types::builders::ReasoningTextBlockBuilder {
    if builder.text.is_none() {
        builder.text = Some(Default::default())
    }
    builder
}

pub(crate) fn s3_location_correct_errors(mut builder: crate::types::builders::S3LocationBuilder) -> crate::types::builders::S3LocationBuilder {
    if builder.uri.is_none() {
        builder.uri = Some(Default::default())
    }
    builder
}
