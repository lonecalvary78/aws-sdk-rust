// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_investigation_group_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_investigation_group::UpdateInvestigationGroupInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.chatbot_notification_channel {
        #[allow(unused_mut)]
        let mut object_2 = object.key("chatbotNotificationChannel").start_object();
        for (key_3, value_4) in var_1 {
            {
                let mut array_5 = object_2.key(key_3.as_str()).start_array();
                for item_6 in value_4 {
                    {
                        array_5.value().string(item_6.as_str());
                    }
                }
                array_5.finish();
            }
        }
        object_2.finish();
    }
    if let Some(var_7) = &input.encryption_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("encryptionConfiguration").start_object();
        crate::protocol_serde::shape_encryption_configuration::ser_encryption_configuration(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.is_cloud_trail_event_history_enabled {
        object.key("isCloudTrailEventHistoryEnabled").boolean(*var_9);
    }
    if let Some(var_10) = &input.role_arn {
        object.key("roleArn").string(var_10.as_str());
    }
    if let Some(var_11) = &input.tag_key_boundaries {
        let mut array_12 = object.key("tagKeyBoundaries").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    Ok(())
}
