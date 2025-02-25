// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_audience_export_job_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_audience_export_job::StartAudienceExportJobInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.audience_generation_job_arn {
        object.key("audienceGenerationJobArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.audience_size {
        #[allow(unused_mut)]
        let mut object_3 = object.key("audienceSize").start_object();
        crate::protocol_serde::shape_audience_size::ser_audience_size(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.description {
        object.key("description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.name {
        object.key("name").string(var_5.as_str());
    }
    Ok(())
}
