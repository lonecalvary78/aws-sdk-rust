// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_register_target_with_maintenance_window_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::register_target_with_maintenance_window::RegisterTargetWithMaintenanceWindowInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.window_id {
        object.key("WindowId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_type {
        object.key("ResourceType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.targets {
        let mut array_4 = object.key("Targets").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_target::ser_target(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.owner_information {
        object.key("OwnerInformation").string(var_7.as_str());
    }
    if let Some(var_8) = &input.name {
        object.key("Name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.description {
        object.key("Description").string(var_9.as_str());
    }
    if let Some(var_10) = &input.client_token {
        object.key("ClientToken").string(var_10.as_str());
    }
    Ok(())
}
