// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_rule_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_rule::DescribeRuleInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.event_bus_name {
        object.key("EventBusName").string(var_2.as_str());
    }
    Ok(())
}
