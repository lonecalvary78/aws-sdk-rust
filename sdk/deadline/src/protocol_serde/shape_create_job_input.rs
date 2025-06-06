// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_job_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_job::CreateJobInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.attachments {
        #[allow(unused_mut)]
        let mut object_2 = object.key("attachments").start_object();
        crate::protocol_serde::shape_attachments::ser_attachments(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.max_failed_tasks_count {
        object.key("maxFailedTasksCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.max_retries_per_task {
        object.key("maxRetriesPerTask").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.max_worker_count {
        object.key("maxWorkerCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_7 = object.key("parameters").start_object();
        for (key_8, value_9) in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_10 = object_7.key(key_8.as_str()).start_object();
                crate::protocol_serde::shape_job_parameter::ser_job_parameter(&mut object_10, value_9)?;
                object_10.finish();
            }
        }
        object_7.finish();
    }
    if let Some(var_11) = &input.priority {
        object.key("priority").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    if let Some(var_12) = &input.source_job_id {
        object.key("sourceJobId").string(var_12.as_str());
    }
    if let Some(var_13) = &input.storage_profile_id {
        object.key("storageProfileId").string(var_13.as_str());
    }
    if let Some(var_14) = &input.target_task_run_status {
        object.key("targetTaskRunStatus").string(var_14.as_str());
    }
    if let Some(var_15) = &input.template {
        object.key("template").string(var_15.as_str());
    }
    if let Some(var_16) = &input.template_type {
        object.key("templateType").string(var_16.as_str());
    }
    Ok(())
}
