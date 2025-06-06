// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_managed_agent_state_change(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ManagedAgentStateChange,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("containerName").string(input.container_name.as_str());
    }
    {
        object.key("managedAgentName").string(input.managed_agent_name.as_str());
    }
    {
        object.key("status").string(input.status.as_str());
    }
    if let Some(var_1) = &input.reason {
        object.key("reason").string(var_1.as_str());
    }
    Ok(())
}
