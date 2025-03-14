// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_media_capture_pipeline_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_media_capture_pipeline::CreateMediaCapturePipelineInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.chime_sdk_meeting_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ChimeSdkMeetingConfiguration").start_object();
        crate::protocol_serde::shape_chime_sdk_meeting_configuration::ser_chime_sdk_meeting_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.sink_arn {
        object.key("SinkArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.sink_iam_role_arn {
        object.key("SinkIamRoleArn").string(var_5.as_str());
    }
    if let Some(var_6) = &input.sink_type {
        object.key("SinkType").string(var_6.as_str());
    }
    if let Some(var_7) = &input.source_arn {
        object.key("SourceArn").string(var_7.as_str());
    }
    if let Some(var_8) = &input.source_type {
        object.key("SourceType").string(var_8.as_str());
    }
    if let Some(var_9) = &input.sse_aws_key_management_params {
        #[allow(unused_mut)]
        let mut object_10 = object.key("SseAwsKeyManagementParams").start_object();
        crate::protocol_serde::shape_sse_aws_key_management_params::ser_sse_aws_key_management_params(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.tags {
        let mut array_12 = object.key("Tags").start_array();
        for item_13 in var_11 {
            {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    Ok(())
}
