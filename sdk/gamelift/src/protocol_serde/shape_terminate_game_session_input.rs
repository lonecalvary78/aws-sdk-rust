// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_terminate_game_session_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::terminate_game_session::TerminateGameSessionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.game_session_id {
        object.key("GameSessionId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.termination_mode {
        object.key("TerminationMode").string(var_2.as_str());
    }
    Ok(())
}
