// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_reference_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ReferenceFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.md5 {
        object.key("md5").string(var_2.as_str());
    }
    if let Some(var_3) = &input.created_after {
        object
            .key("createdAfter")
            .date_time(var_3, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    if let Some(var_4) = &input.created_before {
        object
            .key("createdBefore")
            .date_time(var_4, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    Ok(())
}
