// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_data_spec(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::S3DataSpec,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("DataLocationS3").string(input.data_location_s3.as_str());
    }
    if let Some(var_1) = &input.data_rearrangement {
        object.key("DataRearrangement").string(var_1.as_str());
    }
    if let Some(var_2) = &input.data_schema {
        object.key("DataSchema").string(var_2.as_str());
    }
    if let Some(var_3) = &input.data_schema_location_s3 {
        object.key("DataSchemaLocationS3").string(var_3.as_str());
    }
    Ok(())
}
