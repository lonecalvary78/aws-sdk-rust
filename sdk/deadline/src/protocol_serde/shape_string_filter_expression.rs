// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_string_filter_expression(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::StringFilterExpression,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("name").string(input.name.as_str());
    }
    {
        object.key("operator").string(input.operator.as_str());
    }
    {
        object.key("value").string(input.value.as_str());
    }
    Ok(())
}
