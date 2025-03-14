// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_view_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_view::CreateViewInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.content {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Content").start_object();
        crate::protocol_serde::shape_view_input_content::ser_view_input_content(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.description {
        object.key("Description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.name {
        object.key("Name").string(var_5.as_str());
    }
    if let Some(var_6) = &input.status {
        object.key("Status").string(var_6.as_str());
    }
    if let Some(var_7) = &input.tags {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Tags").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    Ok(())
}
