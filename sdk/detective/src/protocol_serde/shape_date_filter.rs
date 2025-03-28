// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_date_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DateFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object
            .key("StartInclusive")
            .date_time(&input.start_inclusive, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    {
        object
            .key("EndInclusive")
            .date_time(&input.end_inclusive, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    Ok(())
}
