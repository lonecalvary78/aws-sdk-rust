// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_configuration_recorder_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.configuration_recorder_name {
        object.key("ConfigurationRecorderName").string(var_1.as_str());
    }
    Ok(())
}
