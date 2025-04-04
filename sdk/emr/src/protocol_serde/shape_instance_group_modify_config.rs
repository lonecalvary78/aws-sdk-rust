// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_instance_group_modify_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::InstanceGroupModifyConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.instance_group_id {
        object.key("InstanceGroupId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.instance_count {
        object.key("InstanceCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.ec2_instance_ids_to_terminate {
        let mut array_4 = object.key("EC2InstanceIdsToTerminate").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.shrink_policy {
        #[allow(unused_mut)]
        let mut object_7 = object.key("ShrinkPolicy").start_object();
        crate::protocol_serde::shape_shrink_policy::ser_shrink_policy(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.reconfiguration_type {
        object.key("ReconfigurationType").string(var_8.as_str());
    }
    if let Some(var_9) = &input.configurations {
        let mut array_10 = object.key("Configurations").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_configuration::ser_configuration(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    Ok(())
}
