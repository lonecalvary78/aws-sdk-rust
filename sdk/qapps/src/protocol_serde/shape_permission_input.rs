// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_permission_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PermissionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("action").string(input.action.as_str());
    }
    {
        object.key("principal").string(input.principal.as_str());
    }
    Ok(())
}
