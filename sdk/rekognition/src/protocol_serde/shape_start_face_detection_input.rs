// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_face_detection_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_face_detection::StartFaceDetectionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.video {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Video").start_object();
        crate::protocol_serde::shape_video::ser_video(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.notification_channel {
        #[allow(unused_mut)]
        let mut object_5 = object.key("NotificationChannel").start_object();
        crate::protocol_serde::shape_notification_channel::ser_notification_channel(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.face_attributes {
        object.key("FaceAttributes").string(var_6.as_str());
    }
    if let Some(var_7) = &input.job_tag {
        object.key("JobTag").string(var_7.as_str());
    }
    Ok(())
}
