// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_run_pipeline_activity_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::run_pipeline_activity::RunPipelineActivityInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.payloads {
        let mut array_2 = object.key("payloads").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string_unchecked(&::aws_smithy_types::base64::encode(item_3));
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.pipeline_activity {
        #[allow(unused_mut)]
        let mut object_5 = object.key("pipelineActivity").start_object();
        crate::protocol_serde::shape_pipeline_activity::ser_pipeline_activity(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}
