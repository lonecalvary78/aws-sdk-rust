// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_assessment_report_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_assessment_report::GetAssessmentReportInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.assessment_run_arn {
        object.key("assessmentRunArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.report_file_format {
        object.key("reportFileFormat").string(var_2.as_str());
    }
    if let Some(var_3) = &input.report_type {
        object.key("reportType").string(var_3.as_str());
    }
    Ok(())
}
