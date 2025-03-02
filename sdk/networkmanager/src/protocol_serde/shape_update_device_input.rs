// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_device_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_device::UpdateDeviceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.aws_location {
        #[allow(unused_mut)]
        let mut object_2 = object.key("AWSLocation").start_object();
        crate::protocol_serde::shape_aws_location::ser_aws_location(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.location {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Location").start_object();
        crate::protocol_serde::shape_location::ser_location(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.model {
        object.key("Model").string(var_6.as_str());
    }
    if let Some(var_7) = &input.serial_number {
        object.key("SerialNumber").string(var_7.as_str());
    }
    if let Some(var_8) = &input.site_id {
        object.key("SiteId").string(var_8.as_str());
    }
    if let Some(var_9) = &input.r#type {
        object.key("Type").string(var_9.as_str());
    }
    if let Some(var_10) = &input.vendor {
        object.key("Vendor").string(var_10.as_str());
    }
    Ok(())
}
