// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_query_definitions_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_query_definitions::DescribeQueryDefinitionsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.query_language {
        object.key("queryLanguage").string(var_1.as_str());
    }
    if let Some(var_2) = &input.query_definition_name_prefix {
        object.key("queryDefinitionNamePrefix").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.next_token {
        object.key("nextToken").string(var_4.as_str());
    }
    Ok(())
}
