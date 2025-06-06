// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_click_feedback(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ClickFeedback,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("ResultId").string(input.result_id.as_str());
    }
    {
        object
            .key("ClickTime")
            .date_time(&input.click_time, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}
