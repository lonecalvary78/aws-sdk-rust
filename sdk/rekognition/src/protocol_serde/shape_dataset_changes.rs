// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_dataset_changes(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DatasetChanges,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object
            .key("GroundTruth")
            .string_unchecked(&::aws_smithy_types::base64::encode(&input.ground_truth));
    }
    Ok(())
}
