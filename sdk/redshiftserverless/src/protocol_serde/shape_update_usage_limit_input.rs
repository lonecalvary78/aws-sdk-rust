// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_usage_limit_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_usage_limit::UpdateUsageLimitInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.usage_limit_id {
        object.key("usageLimitId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.amount {
        object.key("amount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.breach_action {
        object.key("breachAction").string(var_3.as_str());
    }
    Ok(())
}
