// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_api_keys_output_output_next_token(
    input: &crate::operation::list_api_keys::ListApiKeysOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_apis_output_output_next_token(
    input: &crate::operation::list_apis::ListApisOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_channel_namespaces_output_output_next_token(
    input: &crate::operation::list_channel_namespaces::ListChannelNamespacesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_data_sources_output_output_next_token(
    input: &crate::operation::list_data_sources::ListDataSourcesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_domain_names_output_output_next_token(
    input: &crate::operation::list_domain_names::ListDomainNamesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_functions_output_output_next_token(
    input: &crate::operation::list_functions::ListFunctionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_graphql_apis_output_output_next_token(
    input: &crate::operation::list_graphql_apis::ListGraphqlApisOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_resolvers_output_output_next_token(
    input: &crate::operation::list_resolvers::ListResolversOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_resolvers_by_function_output_output_next_token(
    input: &crate::operation::list_resolvers_by_function::ListResolversByFunctionOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_source_api_associations_output_output_next_token(
    input: &crate::operation::list_source_api_associations::ListSourceApiAssociationsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_types_output_output_next_token(
    input: &crate::operation::list_types::ListTypesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_types_by_association_output_output_next_token(
    input: &crate::operation::list_types_by_association::ListTypesByAssociationOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_api_keys_output_output_api_keys(
    input: crate::operation::list_api_keys::ListApiKeysOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ApiKey>> {
    let input = input.api_keys?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_apis_output_output_apis(
    input: crate::operation::list_apis::ListApisOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Api>> {
    let input = input.apis?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_channel_namespaces_output_output_channel_namespaces(
    input: crate::operation::list_channel_namespaces::ListChannelNamespacesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::ChannelNamespace>> {
    let input = input.channel_namespaces?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_data_sources_output_output_data_sources(
    input: crate::operation::list_data_sources::ListDataSourcesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::DataSource>> {
    let input = input.data_sources?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_domain_names_output_output_domain_name_configs(
    input: crate::operation::list_domain_names::ListDomainNamesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::DomainNameConfig>> {
    let input = input.domain_name_configs?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_functions_output_output_functions(
    input: crate::operation::list_functions::ListFunctionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::FunctionConfiguration>> {
    let input = input.functions?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_graphql_apis_output_output_graphql_apis(
    input: crate::operation::list_graphql_apis::ListGraphqlApisOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::GraphqlApi>> {
    let input = input.graphql_apis?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_resolvers_output_output_resolvers(
    input: crate::operation::list_resolvers::ListResolversOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Resolver>> {
    let input = input.resolvers?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_resolvers_by_function_output_output_resolvers(
    input: crate::operation::list_resolvers_by_function::ListResolversByFunctionOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Resolver>> {
    let input = input.resolvers?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_source_api_associations_output_output_source_api_association_summaries(
    input: crate::operation::list_source_api_associations::ListSourceApiAssociationsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::SourceApiAssociationSummary>> {
    let input = input.source_api_association_summaries?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_types_output_output_types(
    input: crate::operation::list_types::ListTypesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Type>> {
    let input = input.types?;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_types_by_association_output_output_types(
    input: crate::operation::list_types_by_association::ListTypesByAssociationOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Type>> {
    let input = input.types?;
    ::std::option::Option::Some(input)
}
