// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_database_source_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DatabaseSourceConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Type").string(input.r#type.as_str());
    }
    {
        object.key("Endpoint").string(input.endpoint.as_str());
    }
    {
        object.key("Port").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.port).into()),
        );
    }
    if let Some(var_1) = &input.ssl_mode {
        object.key("SSLMode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.databases {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Databases").start_object();
        crate::protocol_serde::shape_database_list::ser_database_list(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.tables {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Tables").start_object();
        crate::protocol_serde::shape_database_table_list::ser_database_table_list(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.columns {
        #[allow(unused_mut)]
        let mut object_7 = object.key("Columns").start_object();
        crate::protocol_serde::shape_database_column_list::ser_database_column_list(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.surrogate_keys {
        let mut array_9 = object.key("SurrogateKeys").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    {
        object.key("SnapshotWatermarkTable").string(input.snapshot_watermark_table.as_str());
    }
    if let Some(var_11) = &input.database_source_authentication_configuration {
        #[allow(unused_mut)]
        let mut object_12 = object.key("DatabaseSourceAuthenticationConfiguration").start_object();
        crate::protocol_serde::shape_database_source_authentication_configuration::ser_database_source_authentication_configuration(
            &mut object_12,
            var_11,
        )?;
        object_12.finish();
    }
    if let Some(var_13) = &input.database_source_vpc_configuration {
        #[allow(unused_mut)]
        let mut object_14 = object.key("DatabaseSourceVPCConfiguration").start_object();
        crate::protocol_serde::shape_database_source_vpc_configuration::ser_database_source_vpc_configuration(&mut object_14, var_13)?;
        object_14.finish();
    }
    Ok(())
}
