// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_users_in_group_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_users_in_group::ListUsersInGroupInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.user_pool_id {
        object.key("UserPoolId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.group_name {
        object.key("GroupName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.limit {
        object.key("Limit").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    Ok(())
}
