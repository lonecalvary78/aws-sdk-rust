// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_protected_job_result_configuration_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ProtectedJobResultConfigurationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.output_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("outputConfiguration").start_object();
        crate::protocol_serde::shape_protected_job_output_configuration_input::ser_protected_job_output_configuration_input(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
