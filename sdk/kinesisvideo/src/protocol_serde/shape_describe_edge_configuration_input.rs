// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_edge_configuration_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_edge_configuration::DescribeEdgeConfigurationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.stream_arn {
        object.key("StreamARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.stream_name {
        object.key("StreamName").string(var_2.as_str());
    }
    Ok(())
}
