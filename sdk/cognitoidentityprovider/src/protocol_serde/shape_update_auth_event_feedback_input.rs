// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_auth_event_feedback_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_auth_event_feedback::UpdateAuthEventFeedbackInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.user_pool_id {
        object.key("UserPoolId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.username {
        object.key("Username").string(var_2.as_str());
    }
    if let Some(var_3) = &input.event_id {
        object.key("EventId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.feedback_token {
        object.key("FeedbackToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.feedback_value {
        object.key("FeedbackValue").string(var_5.as_str());
    }
    Ok(())
}
