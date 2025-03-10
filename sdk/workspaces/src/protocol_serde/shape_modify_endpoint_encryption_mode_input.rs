// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_endpoint_encryption_mode_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionModeInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.directory_id {
        object.key("DirectoryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.endpoint_encryption_mode {
        object.key("EndpointEncryptionMode").string(var_2.as_str());
    }
    Ok(())
}
