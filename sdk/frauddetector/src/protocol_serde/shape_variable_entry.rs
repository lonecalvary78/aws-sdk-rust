// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_variable_entry(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::VariableEntry,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.data_type {
        object.key("dataType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.data_source {
        object.key("dataSource").string(var_3.as_str());
    }
    if let Some(var_4) = &input.default_value {
        object.key("defaultValue").string(var_4.as_str());
    }
    if let Some(var_5) = &input.description {
        object.key("description").string(var_5.as_str());
    }
    if let Some(var_6) = &input.variable_type {
        object.key("variableType").string(var_6.as_str());
    }
    Ok(())
}
