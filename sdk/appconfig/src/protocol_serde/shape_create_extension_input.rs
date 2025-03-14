// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_extension_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_extension::CreateExtensionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.actions {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Actions").start_object();
        for (key_3, value_4) in var_1 {
            {
                let mut array_5 = object_2.key(key_3.as_str()).start_array();
                for item_6 in value_4 {
                    {
                        #[allow(unused_mut)]
                        let mut object_7 = array_5.value().start_object();
                        crate::protocol_serde::shape_action::ser_action(&mut object_7, item_6)?;
                        object_7.finish();
                    }
                }
                array_5.finish();
            }
        }
        object_2.finish();
    }
    if let Some(var_8) = &input.description {
        object.key("Description").string(var_8.as_str());
    }
    if let Some(var_9) = &input.name {
        object.key("Name").string(var_9.as_str());
    }
    if let Some(var_10) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_11 = object.key("Parameters").start_object();
        for (key_12, value_13) in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_14 = object_11.key(key_12.as_str()).start_object();
                crate::protocol_serde::shape_parameter::ser_parameter(&mut object_14, value_13)?;
                object_14.finish();
            }
        }
        object_11.finish();
    }
    if let Some(var_15) = &input.tags {
        #[allow(unused_mut)]
        let mut object_16 = object.key("Tags").start_object();
        for (key_17, value_18) in var_15 {
            {
                object_16.key(key_17.as_str()).string(value_18.as_str());
            }
        }
        object_16.finish();
    }
    Ok(())
}
