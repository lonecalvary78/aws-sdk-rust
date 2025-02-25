// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_group_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_group::CreateGroupInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.configuration {
        let mut array_2 = object.key("Configuration").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_group_configuration_item::ser_group_configuration_item(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.criticality {
        object.key("Criticality").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.description {
        object.key("Description").string(var_6.as_str());
    }
    if let Some(var_7) = &input.display_name {
        object.key("DisplayName").string(var_7.as_str());
    }
    if let Some(var_8) = &input.name {
        object.key("Name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.owner {
        object.key("Owner").string(var_9.as_str());
    }
    if let Some(var_10) = &input.resource_query {
        #[allow(unused_mut)]
        let mut object_11 = object.key("ResourceQuery").start_object();
        crate::protocol_serde::shape_resource_query::ser_resource_query(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.tags {
        #[allow(unused_mut)]
        let mut object_13 = object.key("Tags").start_object();
        for (key_14, value_15) in var_12 {
            {
                object_13.key(key_14.as_str()).string(value_15.as_str());
            }
        }
        object_13.finish();
    }
    Ok(())
}
