// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_forget_device_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::forget_device::ForgetDeviceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.access_token {
        object.key("AccessToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.device_key {
        object.key("DeviceKey").string(var_2.as_str());
    }
    Ok(())
}
