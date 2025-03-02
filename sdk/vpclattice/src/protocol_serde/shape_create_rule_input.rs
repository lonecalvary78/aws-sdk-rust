// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_rule_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_rule::CreateRuleInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.action {
        #[allow(unused_mut)]
        let mut object_2 = object.key("action").start_object();
        crate::protocol_serde::shape_rule_action::ser_rule_action(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.client_token {
        object.key("clientToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.r#match {
        #[allow(unused_mut)]
        let mut object_5 = object.key("match").start_object();
        crate::protocol_serde::shape_rule_match::ser_rule_match(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.name {
        object.key("name").string(var_6.as_str());
    }
    if let Some(var_7) = &input.priority {
        object.key("priority").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.tags {
        #[allow(unused_mut)]
        let mut object_9 = object.key("tags").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    Ok(())
}
