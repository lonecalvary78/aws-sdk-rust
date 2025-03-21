// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_schema_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_schema::UpdateSchemaInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token_id {
        object.key("ClientTokenId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.content {
        object.key("Content").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.r#type {
        object.key("Type").string(var_4.as_str());
    }
    Ok(())
}
