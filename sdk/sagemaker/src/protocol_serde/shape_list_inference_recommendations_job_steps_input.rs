// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_inference_recommendations_job_steps_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_inference_recommendations_job_steps::ListInferenceRecommendationsJobStepsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.job_name {
        object.key("JobName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.status {
        object.key("Status").string(var_2.as_str());
    }
    if let Some(var_3) = &input.step_type {
        object.key("StepType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.next_token {
        object.key("NextToken").string(var_5.as_str());
    }
    Ok(())
}
