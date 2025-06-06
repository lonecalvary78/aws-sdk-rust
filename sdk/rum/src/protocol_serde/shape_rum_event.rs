// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rum_event(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RumEvent,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("id").string(input.id.as_str());
    }
    {
        object
            .key("timestamp")
            .date_time(&input.timestamp, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    {
        object.key("type").string(input.r#type.as_str());
    }
    if let Some(var_1) = &input.metadata {
        object.key("metadata").string(var_1.as_str());
    }
    {
        object.key("details").string(input.details.as_str());
    }
    Ok(())
}
