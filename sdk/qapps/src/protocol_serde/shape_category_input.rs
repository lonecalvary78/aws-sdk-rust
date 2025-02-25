// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_category_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CategoryInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("id").string(input.id.as_str());
    }
    {
        object.key("title").string(input.title.as_str());
    }
    if let Some(var_1) = &input.color {
        object.key("color").string(var_1.as_str());
    }
    Ok(())
}
