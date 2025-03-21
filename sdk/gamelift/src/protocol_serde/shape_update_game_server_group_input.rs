// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_game_server_group_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_game_server_group::UpdateGameServerGroupInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.game_server_group_name {
        object.key("GameServerGroupName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.role_arn {
        object.key("RoleArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.instance_definitions {
        let mut array_4 = object.key("InstanceDefinitions").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_instance_definition::ser_instance_definition(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.game_server_protection_policy {
        object.key("GameServerProtectionPolicy").string(var_7.as_str());
    }
    if let Some(var_8) = &input.balancing_strategy {
        object.key("BalancingStrategy").string(var_8.as_str());
    }
    Ok(())
}
