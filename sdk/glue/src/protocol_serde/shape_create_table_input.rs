// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_table_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_table::CreateTableInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.catalog_id {
        object.key("CatalogId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.database_name {
        object.key("DatabaseName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.table_input {
        #[allow(unused_mut)]
        let mut object_4 = object.key("TableInput").start_object();
        crate::protocol_serde::shape_table_input::ser_table_input(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.partition_indexes {
        let mut array_6 = object.key("PartitionIndexes").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_partition_index::ser_partition_index(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.transaction_id {
        object.key("TransactionId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.open_table_format_input {
        #[allow(unused_mut)]
        let mut object_11 = object.key("OpenTableFormatInput").start_object();
        crate::protocol_serde::shape_open_table_format_input::ser_open_table_format_input(&mut object_11, var_10)?;
        object_11.finish();
    }
    Ok(())
}
