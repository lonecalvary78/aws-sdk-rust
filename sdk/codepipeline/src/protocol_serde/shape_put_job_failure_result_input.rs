// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_job_failure_result_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_job_failure_result::PutJobFailureResultInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.job_id {
        object.key("jobId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.failure_details {
        #[allow(unused_mut)]
        let mut object_3 = object.key("failureDetails").start_object();
        crate::protocol_serde::shape_failure_details::ser_failure_details(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}
