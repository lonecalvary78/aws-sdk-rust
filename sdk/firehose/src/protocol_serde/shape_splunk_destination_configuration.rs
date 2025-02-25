// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_splunk_destination_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SplunkDestinationConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("HECEndpoint").string(input.hec_endpoint.as_str());
    }
    {
        object.key("HECEndpointType").string(input.hec_endpoint_type.as_str());
    }
    if let Some(var_1) = &input.hec_token {
        object.key("HECToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.hec_acknowledgment_timeout_in_seconds {
        object.key("HECAcknowledgmentTimeoutInSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.retry_options {
        #[allow(unused_mut)]
        let mut object_4 = object.key("RetryOptions").start_object();
        crate::protocol_serde::shape_splunk_retry_options::ser_splunk_retry_options(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.s3_backup_mode {
        object.key("S3BackupMode").string(var_5.as_str());
    }
    if let Some(var_6) = &input.s3_configuration {
        #[allow(unused_mut)]
        let mut object_7 = object.key("S3Configuration").start_object();
        crate::protocol_serde::shape_s3_destination_configuration::ser_s3_destination_configuration(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.processing_configuration {
        #[allow(unused_mut)]
        let mut object_9 = object.key("ProcessingConfiguration").start_object();
        crate::protocol_serde::shape_processing_configuration::ser_processing_configuration(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.cloud_watch_logging_options {
        #[allow(unused_mut)]
        let mut object_11 = object.key("CloudWatchLoggingOptions").start_object();
        crate::protocol_serde::shape_cloud_watch_logging_options::ser_cloud_watch_logging_options(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.buffering_hints {
        #[allow(unused_mut)]
        let mut object_13 = object.key("BufferingHints").start_object();
        crate::protocol_serde::shape_splunk_buffering_hints::ser_splunk_buffering_hints(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.secrets_manager_configuration {
        #[allow(unused_mut)]
        let mut object_15 = object.key("SecretsManagerConfiguration").start_object();
        crate::protocol_serde::shape_secrets_manager_configuration::ser_secrets_manager_configuration(&mut object_15, var_14)?;
        object_15.finish();
    }
    Ok(())
}
