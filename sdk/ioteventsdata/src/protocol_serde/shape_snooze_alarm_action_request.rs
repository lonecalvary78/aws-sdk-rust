// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_snooze_alarm_action_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SnoozeAlarmActionRequest,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("requestId").string(input.request_id.as_str());
    }
    {
        object.key("alarmModelName").string(input.alarm_model_name.as_str());
    }
    if let Some(var_1) = &input.key_value {
        object.key("keyValue").string(var_1.as_str());
    }
    if let Some(var_2) = &input.note {
        object.key("note").string(var_2.as_str());
    }
    {
        object.key("snoozeDuration").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.snooze_duration).into()),
        );
    }
    Ok(())
}
