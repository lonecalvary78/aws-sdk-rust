// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_notification_configuration(
    input: &crate::types::NotificationConfiguration,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.topic_configurations {
        for list_item_2 in var_1 {
            {
                let inner_writer = scope.start_el("TopicConfiguration");
                crate::protocol_serde::shape_topic_configuration::ser_topic_configuration(list_item_2, inner_writer)?
            }
        }
    }
    if let Some(var_3) = &input.queue_configurations {
        for list_item_4 in var_3 {
            {
                let inner_writer = scope.start_el("QueueConfiguration");
                crate::protocol_serde::shape_queue_configuration::ser_queue_configuration(list_item_4, inner_writer)?
            }
        }
    }
    if let Some(var_5) = &input.lambda_function_configurations {
        for list_item_6 in var_5 {
            {
                let inner_writer = scope.start_el("CloudFunctionConfiguration");
                crate::protocol_serde::shape_lambda_function_configuration::ser_lambda_function_configuration(list_item_6, inner_writer)?
            }
        }
    }
    if let Some(_var_7) = &input.event_bridge_configuration {
        scope.start_el("EventBridgeConfiguration").finish();
    }
    scope.finish();
    Ok(())
}
