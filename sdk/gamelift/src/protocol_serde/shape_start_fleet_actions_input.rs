// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_fleet_actions_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_fleet_actions::StartFleetActionsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.fleet_id {
        object.key("FleetId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.actions {
        let mut array_3 = object.key("Actions").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.location {
        object.key("Location").string(var_5.as_str());
    }
    Ok(())
}
