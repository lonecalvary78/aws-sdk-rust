// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_engine_transcribe_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EngineTranscribeSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.language_code {
        object.key("LanguageCode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.vocabulary_filter_method {
        object.key("VocabularyFilterMethod").string(var_2.as_str());
    }
    if let Some(var_3) = &input.vocabulary_filter_name {
        object.key("VocabularyFilterName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.vocabulary_name {
        object.key("VocabularyName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.region {
        object.key("Region").string(var_5.as_str());
    }
    if input.enable_partial_results_stabilization {
        object
            .key("EnablePartialResultsStabilization")
            .boolean(input.enable_partial_results_stabilization);
    }
    if let Some(var_6) = &input.partial_results_stability {
        object.key("PartialResultsStability").string(var_6.as_str());
    }
    if let Some(var_7) = &input.content_identification_type {
        object.key("ContentIdentificationType").string(var_7.as_str());
    }
    if let Some(var_8) = &input.content_redaction_type {
        object.key("ContentRedactionType").string(var_8.as_str());
    }
    if let Some(var_9) = &input.pii_entity_types {
        object.key("PiiEntityTypes").string(var_9.as_str());
    }
    if let Some(var_10) = &input.language_model_name {
        object.key("LanguageModelName").string(var_10.as_str());
    }
    if input.identify_language {
        object.key("IdentifyLanguage").boolean(input.identify_language);
    }
    if let Some(var_11) = &input.language_options {
        object.key("LanguageOptions").string(var_11.as_str());
    }
    if let Some(var_12) = &input.preferred_language {
        object.key("PreferredLanguage").string(var_12.as_str());
    }
    if let Some(var_13) = &input.vocabulary_names {
        object.key("VocabularyNames").string(var_13.as_str());
    }
    if let Some(var_14) = &input.vocabulary_filter_names {
        object.key("VocabularyFilterNames").string(var_14.as_str());
    }
    Ok(())
}
