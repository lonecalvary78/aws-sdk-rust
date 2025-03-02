// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_alfresco_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AlfrescoConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("SiteUrl").string(input.site_url.as_str());
    }
    {
        object.key("SiteId").string(input.site_id.as_str());
    }
    {
        object.key("SecretArn").string(input.secret_arn.as_str());
    }
    if let Some(var_1) = &input.ssl_certificate_s3_path {
        #[allow(unused_mut)]
        let mut object_2 = object.key("SslCertificateS3Path").start_object();
        crate::protocol_serde::shape_s3_path::ser_s3_path(&mut object_2, var_1)?;
        object_2.finish();
    }
    if input.crawl_system_folders {
        object.key("CrawlSystemFolders").boolean(input.crawl_system_folders);
    }
    if input.crawl_comments {
        object.key("CrawlComments").boolean(input.crawl_comments);
    }
    if let Some(var_3) = &input.entity_filter {
        let mut array_4 = object.key("EntityFilter").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.document_library_field_mappings {
        let mut array_7 = object.key("DocumentLibraryFieldMappings").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.blog_field_mappings {
        let mut array_11 = object.key("BlogFieldMappings").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.wiki_field_mappings {
        let mut array_15 = object.key("WikiFieldMappings").start_array();
        for item_16 in var_14 {
            {
                #[allow(unused_mut)]
                let mut object_17 = array_15.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_17, item_16)?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    if let Some(var_18) = &input.inclusion_patterns {
        let mut array_19 = object.key("InclusionPatterns").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20.as_str());
            }
        }
        array_19.finish();
    }
    if let Some(var_21) = &input.exclusion_patterns {
        let mut array_22 = object.key("ExclusionPatterns").start_array();
        for item_23 in var_21 {
            {
                array_22.value().string(item_23.as_str());
            }
        }
        array_22.finish();
    }
    if let Some(var_24) = &input.vpc_configuration {
        #[allow(unused_mut)]
        let mut object_25 = object.key("VpcConfiguration").start_object();
        crate::protocol_serde::shape_data_source_vpc_configuration::ser_data_source_vpc_configuration(&mut object_25, var_24)?;
        object_25.finish();
    }
    Ok(())
}

pub(crate) fn de_alfresco_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AlfrescoConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AlfrescoConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "SiteUrl" => {
                                builder = builder.set_site_url(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            }
                            "SiteId" => {
                                builder = builder.set_site_id(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            }
                            "SecretArn" => {
                                builder = builder.set_secret_arn(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                        .transpose()?,
                                );
                            }
                            "SslCertificateS3Path" => {
                                builder = builder.set_ssl_certificate_s3_path(crate::protocol_serde::shape_s3_path::de_s3_path(tokens)?);
                            }
                            "CrawlSystemFolders" => {
                                builder =
                                    builder.set_crawl_system_folders(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                            }
                            "CrawlComments" => {
                                builder = builder.set_crawl_comments(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                            }
                            "EntityFilter" => {
                                builder = builder.set_entity_filter(crate::protocol_serde::shape_entity_filter::de_entity_filter(tokens)?);
                            }
                            "DocumentLibraryFieldMappings" => {
                                builder = builder.set_document_library_field_mappings(
                                    crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(
                                        tokens,
                                    )?,
                                );
                            }
                            "BlogFieldMappings" => {
                                builder = builder.set_blog_field_mappings(
                                    crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(
                                        tokens,
                                    )?,
                                );
                            }
                            "WikiFieldMappings" => {
                                builder = builder.set_wiki_field_mappings(
                                    crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(
                                        tokens,
                                    )?,
                                );
                            }
                            "InclusionPatterns" => {
                                builder = builder.set_inclusion_patterns(
                                    crate::protocol_serde::shape_data_source_inclusions_exclusions_strings::de_data_source_inclusions_exclusions_strings(tokens)?
                                );
                            }
                            "ExclusionPatterns" => {
                                builder = builder.set_exclusion_patterns(
                                    crate::protocol_serde::shape_data_source_inclusions_exclusions_strings::de_data_source_inclusions_exclusions_strings(tokens)?
                                );
                            }
                            "VpcConfiguration" => {
                                builder = builder.set_vpc_configuration(
                                    crate::protocol_serde::shape_data_source_vpc_configuration::de_data_source_vpc_configuration(tokens)?,
                                );
                            }
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(crate::serde_util::alfresco_configuration_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
