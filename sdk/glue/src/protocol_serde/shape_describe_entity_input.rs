// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_entity_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_entity::DescribeEntityInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.connection_name {
        object.key("ConnectionName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.catalog_id {
        object.key("CatalogId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.entity_name {
        object.key("EntityName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.data_store_api_version {
        object.key("DataStoreApiVersion").string(var_5.as_str());
    }
    Ok(())
}
