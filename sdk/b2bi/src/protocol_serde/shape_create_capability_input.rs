// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_capability_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_capability::CreateCapabilityInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.r#type {
        object.key("type").string(var_2.as_str());
    }
    if let Some(var_3) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("configuration").start_object();
        crate::protocol_serde::shape_capability_configuration::ser_capability_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.instructions_documents {
        let mut array_6 = object.key("instructionsDocuments").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_s3_location::ser_s3_location(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.client_token {
        object.key("clientToken").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tags {
        let mut array_11 = object.key("tags").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}
