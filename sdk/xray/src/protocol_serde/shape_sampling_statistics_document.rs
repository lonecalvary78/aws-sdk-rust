// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_sampling_statistics_document(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SamplingStatisticsDocument,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("RuleName").string(input.rule_name.as_str());
    }
    {
        object.key("ClientID").string(input.client_id.as_str());
    }
    {
        object
            .key("Timestamp")
            .date_time(&input.timestamp, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    {
        object.key("RequestCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.request_count).into()),
        );
    }
    {
        object.key("SampledCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.sampled_count).into()),
        );
    }
    if input.borrow_count != 0 {
        object.key("BorrowCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.borrow_count).into()),
        );
    }
    Ok(())
}
