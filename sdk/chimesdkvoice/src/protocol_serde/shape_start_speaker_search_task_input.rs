// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_speaker_search_task_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_speaker_search_task::StartSpeakerSearchTaskInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.call_leg {
        object.key("CallLeg").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.transaction_id {
        object.key("TransactionId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.voice_profile_domain_id {
        object.key("VoiceProfileDomainId").string(var_4.as_str());
    }
    Ok(())
}
