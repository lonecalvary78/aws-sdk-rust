// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_link_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_link::UpdateLinkInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.identifier {
        object.key("Identifier").string(var_1.as_str());
    }
    if let Some(var_2) = &input.link_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("LinkConfiguration").start_object();
        crate::protocol_serde::shape_link_configuration::ser_link_configuration(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.resource_types {
        let mut array_5 = object.key("ResourceTypes").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    Ok(())
}
