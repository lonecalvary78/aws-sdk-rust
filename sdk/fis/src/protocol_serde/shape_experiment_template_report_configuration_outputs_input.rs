// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_experiment_template_report_configuration_outputs_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ExperimentTemplateReportConfigurationOutputsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.s3_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("s3Configuration").start_object();
        crate::protocol_serde::shape_report_configuration_s3_output_input::ser_report_configuration_s3_output_input(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
