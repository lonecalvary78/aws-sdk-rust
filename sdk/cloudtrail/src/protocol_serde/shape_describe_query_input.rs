// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_query_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_query::DescribeQueryInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.event_data_store {
        object.key("EventDataStore").string(var_1.as_str());
    }
    if let Some(var_2) = &input.query_id {
        object.key("QueryId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.query_alias {
        object.key("QueryAlias").string(var_3.as_str());
    }
    if let Some(var_4) = &input.refresh_id {
        object.key("RefreshId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.event_data_store_owner_account_id {
        object.key("EventDataStoreOwnerAccountId").string(var_5.as_str());
    }
    Ok(())
}
