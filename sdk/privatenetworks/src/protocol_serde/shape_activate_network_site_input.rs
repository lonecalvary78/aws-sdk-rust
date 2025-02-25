// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_activate_network_site_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::activate_network_site::ActivateNetworkSiteInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.commitment_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("commitmentConfiguration").start_object();
        crate::protocol_serde::shape_commitment_configuration::ser_commitment_configuration(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.network_site_arn {
        object.key("networkSiteArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.shipping_address {
        #[allow(unused_mut)]
        let mut object_6 = object.key("shippingAddress").start_object();
        crate::protocol_serde::shape_address::ser_address(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
