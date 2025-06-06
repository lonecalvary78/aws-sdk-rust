// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_vpce_configuration_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_vpce_configuration::UpdateVpceConfigurationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.arn {
        object.key("arn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.vpce_configuration_name {
        object.key("vpceConfigurationName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.vpce_service_name {
        object.key("vpceServiceName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.service_dns_name {
        object.key("serviceDnsName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.vpce_configuration_description {
        object.key("vpceConfigurationDescription").string(var_5.as_str());
    }
    Ok(())
}
