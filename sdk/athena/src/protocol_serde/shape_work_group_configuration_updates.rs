// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_work_group_configuration_updates(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::WorkGroupConfigurationUpdates,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.enforce_work_group_configuration {
        object.key("EnforceWorkGroupConfiguration").boolean(*var_1);
    }
    if let Some(var_2) = &input.result_configuration_updates {
        #[allow(unused_mut)]
        let mut object_3 = object.key("ResultConfigurationUpdates").start_object();
        crate::protocol_serde::shape_result_configuration_updates::ser_result_configuration_updates(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.managed_query_results_configuration_updates {
        #[allow(unused_mut)]
        let mut object_5 = object.key("ManagedQueryResultsConfigurationUpdates").start_object();
        crate::protocol_serde::shape_managed_query_results_configuration_updates::ser_managed_query_results_configuration_updates(
            &mut object_5,
            var_4,
        )?;
        object_5.finish();
    }
    if let Some(var_6) = &input.publish_cloud_watch_metrics_enabled {
        object.key("PublishCloudWatchMetricsEnabled").boolean(*var_6);
    }
    if let Some(var_7) = &input.bytes_scanned_cutoff_per_query {
        object.key("BytesScannedCutoffPerQuery").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.remove_bytes_scanned_cutoff_per_query {
        object.key("RemoveBytesScannedCutoffPerQuery").boolean(*var_8);
    }
    if let Some(var_9) = &input.requester_pays_enabled {
        object.key("RequesterPaysEnabled").boolean(*var_9);
    }
    if let Some(var_10) = &input.engine_version {
        #[allow(unused_mut)]
        let mut object_11 = object.key("EngineVersion").start_object();
        crate::protocol_serde::shape_engine_version::ser_engine_version(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.remove_customer_content_encryption_configuration {
        object.key("RemoveCustomerContentEncryptionConfiguration").boolean(*var_12);
    }
    if let Some(var_13) = &input.additional_configuration {
        object.key("AdditionalConfiguration").string(var_13.as_str());
    }
    if let Some(var_14) = &input.execution_role {
        object.key("ExecutionRole").string(var_14.as_str());
    }
    if let Some(var_15) = &input.customer_content_encryption_configuration {
        #[allow(unused_mut)]
        let mut object_16 = object.key("CustomerContentEncryptionConfiguration").start_object();
        crate::protocol_serde::shape_customer_content_encryption_configuration::ser_customer_content_encryption_configuration(
            &mut object_16,
            var_15,
        )?;
        object_16.finish();
    }
    if let Some(var_17) = &input.enable_minimum_encryption_configuration {
        object.key("EnableMinimumEncryptionConfiguration").boolean(*var_17);
    }
    if let Some(var_18) = &input.query_results_s3_access_grants_configuration {
        #[allow(unused_mut)]
        let mut object_19 = object.key("QueryResultsS3AccessGrantsConfiguration").start_object();
        crate::protocol_serde::shape_query_results_s3_access_grants_configuration::ser_query_results_s3_access_grants_configuration(
            &mut object_19,
            var_18,
        )?;
        object_19.finish();
    }
    Ok(())
}
