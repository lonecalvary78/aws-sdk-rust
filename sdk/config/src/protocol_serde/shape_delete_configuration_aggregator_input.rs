// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_configuration_aggregator_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_configuration_aggregator::DeleteConfigurationAggregatorInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.configuration_aggregator_name {
        object.key("ConfigurationAggregatorName").string(var_1.as_str());
    }
    Ok(())
}
