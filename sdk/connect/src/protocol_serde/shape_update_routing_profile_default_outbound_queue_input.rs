// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_routing_profile_default_outbound_queue_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_routing_profile_default_outbound_queue::UpdateRoutingProfileDefaultOutboundQueueInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.default_outbound_queue_id {
        object.key("DefaultOutboundQueueId").string(var_1.as_str());
    }
    Ok(())
}
