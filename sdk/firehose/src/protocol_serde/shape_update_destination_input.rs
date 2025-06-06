// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_destination_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_destination::UpdateDestinationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.delivery_stream_name {
        object.key("DeliveryStreamName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.current_delivery_stream_version_id {
        object.key("CurrentDeliveryStreamVersionId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.destination_id {
        object.key("DestinationId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.s3_destination_update {
        #[allow(unused_mut)]
        let mut object_5 = object.key("S3DestinationUpdate").start_object();
        crate::protocol_serde::shape_s3_destination_update::ser_s3_destination_update(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.extended_s3_destination_update {
        #[allow(unused_mut)]
        let mut object_7 = object.key("ExtendedS3DestinationUpdate").start_object();
        crate::protocol_serde::shape_extended_s3_destination_update::ser_extended_s3_destination_update(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.redshift_destination_update {
        #[allow(unused_mut)]
        let mut object_9 = object.key("RedshiftDestinationUpdate").start_object();
        crate::protocol_serde::shape_redshift_destination_update::ser_redshift_destination_update(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.elasticsearch_destination_update {
        #[allow(unused_mut)]
        let mut object_11 = object.key("ElasticsearchDestinationUpdate").start_object();
        crate::protocol_serde::shape_elasticsearch_destination_update::ser_elasticsearch_destination_update(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.amazonopensearchservice_destination_update {
        #[allow(unused_mut)]
        let mut object_13 = object.key("AmazonopensearchserviceDestinationUpdate").start_object();
        crate::protocol_serde::shape_amazonopensearchservice_destination_update::ser_amazonopensearchservice_destination_update(
            &mut object_13,
            var_12,
        )?;
        object_13.finish();
    }
    if let Some(var_14) = &input.splunk_destination_update {
        #[allow(unused_mut)]
        let mut object_15 = object.key("SplunkDestinationUpdate").start_object();
        crate::protocol_serde::shape_splunk_destination_update::ser_splunk_destination_update(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.http_endpoint_destination_update {
        #[allow(unused_mut)]
        let mut object_17 = object.key("HttpEndpointDestinationUpdate").start_object();
        crate::protocol_serde::shape_http_endpoint_destination_update::ser_http_endpoint_destination_update(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.amazon_open_search_serverless_destination_update {
        #[allow(unused_mut)]
        let mut object_19 = object.key("AmazonOpenSearchServerlessDestinationUpdate").start_object();
        crate::protocol_serde::shape_amazon_open_search_serverless_destination_update::ser_amazon_open_search_serverless_destination_update(
            &mut object_19,
            var_18,
        )?;
        object_19.finish();
    }
    if let Some(var_20) = &input.snowflake_destination_update {
        #[allow(unused_mut)]
        let mut object_21 = object.key("SnowflakeDestinationUpdate").start_object();
        crate::protocol_serde::shape_snowflake_destination_update::ser_snowflake_destination_update(&mut object_21, var_20)?;
        object_21.finish();
    }
    if let Some(var_22) = &input.iceberg_destination_update {
        #[allow(unused_mut)]
        let mut object_23 = object.key("IcebergDestinationUpdate").start_object();
        crate::protocol_serde::shape_iceberg_destination_update::ser_iceberg_destination_update(&mut object_23, var_22)?;
        object_23.finish();
    }
    Ok(())
}
