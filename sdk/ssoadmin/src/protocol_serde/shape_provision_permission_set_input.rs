// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_provision_permission_set_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::provision_permission_set::ProvisionPermissionSetInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.instance_arn {
        object.key("InstanceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.permission_set_arn {
        object.key("PermissionSetArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.target_id {
        object.key("TargetId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.target_type {
        object.key("TargetType").string(var_4.as_str());
    }
    Ok(())
}
