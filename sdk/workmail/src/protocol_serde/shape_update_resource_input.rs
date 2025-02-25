// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_resource_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_resource::UpdateResourceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.organization_id {
        object.key("OrganizationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_id {
        object.key("ResourceId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.booking_options {
        #[allow(unused_mut)]
        let mut object_5 = object.key("BookingOptions").start_object();
        crate::protocol_serde::shape_booking_options::ser_booking_options(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.description {
        object.key("Description").string(var_6.as_str());
    }
    if let Some(var_7) = &input.r#type {
        object.key("Type").string(var_7.as_str());
    }
    if let Some(var_8) = &input.hidden_from_global_address_list {
        object.key("HiddenFromGlobalAddressList").boolean(*var_8);
    }
    Ok(())
}
