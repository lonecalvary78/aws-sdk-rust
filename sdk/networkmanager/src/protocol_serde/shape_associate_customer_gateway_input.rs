// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_customer_gateway_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::associate_customer_gateway::AssociateCustomerGatewayInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.customer_gateway_arn {
        object.key("CustomerGatewayArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.device_id {
        object.key("DeviceId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.link_id {
        object.key("LinkId").string(var_3.as_str());
    }
    Ok(())
}
