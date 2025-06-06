// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_replication_instance_task_logs_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_replication_instance_task_logs::DescribeReplicationInstanceTaskLogsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.replication_instance_arn {
        object.key("ReplicationInstanceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_records {
        object.key("MaxRecords").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.marker {
        object.key("Marker").string(var_3.as_str());
    }
    Ok(())
}
