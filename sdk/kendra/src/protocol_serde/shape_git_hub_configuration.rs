// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_git_hub_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GitHubConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.saa_s_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("SaaSConfiguration").start_object();
        crate::protocol_serde::shape_saa_s_configuration::ser_saa_s_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.on_premise_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("OnPremiseConfiguration").start_object();
        crate::protocol_serde::shape_on_premise_configuration::ser_on_premise_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.r#type {
        object.key("Type").string(var_5.as_str());
    }
    {
        object.key("SecretArn").string(input.secret_arn.as_str());
    }
    if input.use_change_log {
        object.key("UseChangeLog").boolean(input.use_change_log);
    }
    if let Some(var_6) = &input.git_hub_document_crawl_properties {
        #[allow(unused_mut)]
        let mut object_7 = object.key("GitHubDocumentCrawlProperties").start_object();
        crate::protocol_serde::shape_git_hub_document_crawl_properties::ser_git_hub_document_crawl_properties(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.repository_filter {
        let mut array_9 = object.key("RepositoryFilter").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.inclusion_folder_name_patterns {
        let mut array_12 = object.key("InclusionFolderNamePatterns").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    if let Some(var_14) = &input.inclusion_file_type_patterns {
        let mut array_15 = object.key("InclusionFileTypePatterns").start_array();
        for item_16 in var_14 {
            {
                array_15.value().string(item_16.as_str());
            }
        }
        array_15.finish();
    }
    if let Some(var_17) = &input.inclusion_file_name_patterns {
        let mut array_18 = object.key("InclusionFileNamePatterns").start_array();
        for item_19 in var_17 {
            {
                array_18.value().string(item_19.as_str());
            }
        }
        array_18.finish();
    }
    if let Some(var_20) = &input.exclusion_folder_name_patterns {
        let mut array_21 = object.key("ExclusionFolderNamePatterns").start_array();
        for item_22 in var_20 {
            {
                array_21.value().string(item_22.as_str());
            }
        }
        array_21.finish();
    }
    if let Some(var_23) = &input.exclusion_file_type_patterns {
        let mut array_24 = object.key("ExclusionFileTypePatterns").start_array();
        for item_25 in var_23 {
            {
                array_24.value().string(item_25.as_str());
            }
        }
        array_24.finish();
    }
    if let Some(var_26) = &input.exclusion_file_name_patterns {
        let mut array_27 = object.key("ExclusionFileNamePatterns").start_array();
        for item_28 in var_26 {
            {
                array_27.value().string(item_28.as_str());
            }
        }
        array_27.finish();
    }
    if let Some(var_29) = &input.vpc_configuration {
        #[allow(unused_mut)]
        let mut object_30 = object.key("VpcConfiguration").start_object();
        crate::protocol_serde::shape_data_source_vpc_configuration::ser_data_source_vpc_configuration(&mut object_30, var_29)?;
        object_30.finish();
    }
    if let Some(var_31) = &input.git_hub_repository_configuration_field_mappings {
        let mut array_32 = object.key("GitHubRepositoryConfigurationFieldMappings").start_array();
        for item_33 in var_31 {
            {
                #[allow(unused_mut)]
                let mut object_34 = array_32.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_34, item_33)?;
                object_34.finish();
            }
        }
        array_32.finish();
    }
    if let Some(var_35) = &input.git_hub_commit_configuration_field_mappings {
        let mut array_36 = object.key("GitHubCommitConfigurationFieldMappings").start_array();
        for item_37 in var_35 {
            {
                #[allow(unused_mut)]
                let mut object_38 = array_36.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_38, item_37)?;
                object_38.finish();
            }
        }
        array_36.finish();
    }
    if let Some(var_39) = &input.git_hub_issue_document_configuration_field_mappings {
        let mut array_40 = object.key("GitHubIssueDocumentConfigurationFieldMappings").start_array();
        for item_41 in var_39 {
            {
                #[allow(unused_mut)]
                let mut object_42 = array_40.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_42, item_41)?;
                object_42.finish();
            }
        }
        array_40.finish();
    }
    if let Some(var_43) = &input.git_hub_issue_comment_configuration_field_mappings {
        let mut array_44 = object.key("GitHubIssueCommentConfigurationFieldMappings").start_array();
        for item_45 in var_43 {
            {
                #[allow(unused_mut)]
                let mut object_46 = array_44.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_46, item_45)?;
                object_46.finish();
            }
        }
        array_44.finish();
    }
    if let Some(var_47) = &input.git_hub_issue_attachment_configuration_field_mappings {
        let mut array_48 = object.key("GitHubIssueAttachmentConfigurationFieldMappings").start_array();
        for item_49 in var_47 {
            {
                #[allow(unused_mut)]
                let mut object_50 = array_48.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_50, item_49)?;
                object_50.finish();
            }
        }
        array_48.finish();
    }
    if let Some(var_51) = &input.git_hub_pull_request_comment_configuration_field_mappings {
        let mut array_52 = object.key("GitHubPullRequestCommentConfigurationFieldMappings").start_array();
        for item_53 in var_51 {
            {
                #[allow(unused_mut)]
                let mut object_54 = array_52.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_54, item_53)?;
                object_54.finish();
            }
        }
        array_52.finish();
    }
    if let Some(var_55) = &input.git_hub_pull_request_document_configuration_field_mappings {
        let mut array_56 = object.key("GitHubPullRequestDocumentConfigurationFieldMappings").start_array();
        for item_57 in var_55 {
            {
                #[allow(unused_mut)]
                let mut object_58 = array_56.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_58, item_57)?;
                object_58.finish();
            }
        }
        array_56.finish();
    }
    if let Some(var_59) = &input.git_hub_pull_request_document_attachment_configuration_field_mappings {
        let mut array_60 = object.key("GitHubPullRequestDocumentAttachmentConfigurationFieldMappings").start_array();
        for item_61 in var_59 {
            {
                #[allow(unused_mut)]
                let mut object_62 = array_60.value().start_object();
                crate::protocol_serde::shape_data_source_to_index_field_mapping::ser_data_source_to_index_field_mapping(&mut object_62, item_61)?;
                object_62.finish();
            }
        }
        array_60.finish();
    }
    Ok(())
}

pub(crate) fn de_git_hub_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::GitHubConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::GitHubConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "SaaSConfiguration" => {
                            builder =
                                builder.set_saa_s_configuration(crate::protocol_serde::shape_saa_s_configuration::de_saa_s_configuration(tokens)?);
                        }
                        "OnPremiseConfiguration" => {
                            builder = builder.set_on_premise_configuration(
                                crate::protocol_serde::shape_on_premise_configuration::de_on_premise_configuration(tokens)?,
                            );
                        }
                        "Type" => {
                            builder = builder.set_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::Type::from(u.as_ref())))
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
                        "UseChangeLog" => {
                            builder = builder.set_use_change_log(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "GitHubDocumentCrawlProperties" => {
                            builder = builder.set_git_hub_document_crawl_properties(
                                crate::protocol_serde::shape_git_hub_document_crawl_properties::de_git_hub_document_crawl_properties(tokens)?,
                            );
                        }
                        "RepositoryFilter" => {
                            builder = builder.set_repository_filter(crate::protocol_serde::shape_repository_names::de_repository_names(tokens)?);
                        }
                        "InclusionFolderNamePatterns" => {
                            builder = builder.set_inclusion_folder_name_patterns(crate::protocol_serde::shape_string_list::de_string_list(tokens)?);
                        }
                        "InclusionFileTypePatterns" => {
                            builder = builder.set_inclusion_file_type_patterns(crate::protocol_serde::shape_string_list::de_string_list(tokens)?);
                        }
                        "InclusionFileNamePatterns" => {
                            builder = builder.set_inclusion_file_name_patterns(crate::protocol_serde::shape_string_list::de_string_list(tokens)?);
                        }
                        "ExclusionFolderNamePatterns" => {
                            builder = builder.set_exclusion_folder_name_patterns(crate::protocol_serde::shape_string_list::de_string_list(tokens)?);
                        }
                        "ExclusionFileTypePatterns" => {
                            builder = builder.set_exclusion_file_type_patterns(crate::protocol_serde::shape_string_list::de_string_list(tokens)?);
                        }
                        "ExclusionFileNamePatterns" => {
                            builder = builder.set_exclusion_file_name_patterns(crate::protocol_serde::shape_string_list::de_string_list(tokens)?);
                        }
                        "VpcConfiguration" => {
                            builder = builder.set_vpc_configuration(
                                crate::protocol_serde::shape_data_source_vpc_configuration::de_data_source_vpc_configuration(tokens)?,
                            );
                        }
                        "GitHubRepositoryConfigurationFieldMappings" => {
                            builder = builder.set_git_hub_repository_configuration_field_mappings(
                                crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(
                                    tokens,
                                )?,
                            );
                        }
                        "GitHubCommitConfigurationFieldMappings" => {
                            builder = builder.set_git_hub_commit_configuration_field_mappings(
                                crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(
                                    tokens,
                                )?,
                            );
                        }
                        "GitHubIssueDocumentConfigurationFieldMappings" => {
                            builder = builder.set_git_hub_issue_document_configuration_field_mappings(
                                crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(
                                    tokens,
                                )?,
                            );
                        }
                        "GitHubIssueCommentConfigurationFieldMappings" => {
                            builder = builder.set_git_hub_issue_comment_configuration_field_mappings(
                                crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(
                                    tokens,
                                )?,
                            );
                        }
                        "GitHubIssueAttachmentConfigurationFieldMappings" => {
                            builder = builder.set_git_hub_issue_attachment_configuration_field_mappings(
                                crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(
                                    tokens,
                                )?,
                            );
                        }
                        "GitHubPullRequestCommentConfigurationFieldMappings" => {
                            builder = builder.set_git_hub_pull_request_comment_configuration_field_mappings(
                                crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(
                                    tokens,
                                )?,
                            );
                        }
                        "GitHubPullRequestDocumentConfigurationFieldMappings" => {
                            builder = builder.set_git_hub_pull_request_document_configuration_field_mappings(
                                crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(
                                    tokens,
                                )?,
                            );
                        }
                        "GitHubPullRequestDocumentAttachmentConfigurationFieldMappings" => {
                            builder = builder.set_git_hub_pull_request_document_attachment_configuration_field_mappings(
                                crate::protocol_serde::shape_data_source_to_index_field_mapping_list::de_data_source_to_index_field_mapping_list(
                                    tokens,
                                )?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(crate::serde_util::git_hub_configuration_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
