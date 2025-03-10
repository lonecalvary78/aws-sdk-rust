// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_project<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::Project>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ProjectBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "arn" => {
                            builder = builder.set_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "description" => {
                            builder = builder.set_description(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "source" => {
                            builder = builder.set_source(crate::protocol_serde::shape_project_source::de_project_source(tokens)?);
                        }
                        "secondarySources" => {
                            builder = builder.set_secondary_sources(crate::protocol_serde::shape_project_sources::de_project_sources(tokens)?);
                        }
                        "sourceVersion" => {
                            builder = builder.set_source_version(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "secondarySourceVersions" => {
                            builder = builder.set_secondary_source_versions(
                                crate::protocol_serde::shape_project_secondary_source_versions::de_project_secondary_source_versions(tokens)?,
                            );
                        }
                        "artifacts" => {
                            builder = builder.set_artifacts(crate::protocol_serde::shape_project_artifacts::de_project_artifacts(tokens)?);
                        }
                        "secondaryArtifacts" => {
                            builder = builder
                                .set_secondary_artifacts(crate::protocol_serde::shape_project_artifacts_list::de_project_artifacts_list(tokens)?);
                        }
                        "cache" => {
                            builder = builder.set_cache(crate::protocol_serde::shape_project_cache::de_project_cache(tokens)?);
                        }
                        "environment" => {
                            builder = builder.set_environment(crate::protocol_serde::shape_project_environment::de_project_environment(tokens)?);
                        }
                        "serviceRole" => {
                            builder = builder.set_service_role(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "timeoutInMinutes" => {
                            builder = builder.set_timeout_in_minutes(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "queuedTimeoutInMinutes" => {
                            builder = builder.set_queued_timeout_in_minutes(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "encryptionKey" => {
                            builder = builder.set_encryption_key(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "tags" => {
                            builder = builder.set_tags(crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?);
                        }
                        "created" => {
                            builder = builder.set_created(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "lastModified" => {
                            builder = builder.set_last_modified(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "webhook" => {
                            builder = builder.set_webhook(crate::protocol_serde::shape_webhook::de_webhook(tokens)?);
                        }
                        "vpcConfig" => {
                            builder = builder.set_vpc_config(crate::protocol_serde::shape_vpc_config::de_vpc_config(tokens)?);
                        }
                        "badge" => {
                            builder = builder.set_badge(crate::protocol_serde::shape_project_badge::de_project_badge(tokens)?);
                        }
                        "logsConfig" => {
                            builder = builder.set_logs_config(crate::protocol_serde::shape_logs_config::de_logs_config(tokens)?);
                        }
                        "fileSystemLocations" => {
                            builder = builder.set_file_system_locations(
                                crate::protocol_serde::shape_project_file_system_locations::de_project_file_system_locations(tokens)?,
                            );
                        }
                        "buildBatchConfig" => {
                            builder = builder.set_build_batch_config(
                                crate::protocol_serde::shape_project_build_batch_config::de_project_build_batch_config(tokens)?,
                            );
                        }
                        "concurrentBuildLimit" => {
                            builder = builder.set_concurrent_build_limit(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "projectVisibility" => {
                            builder = builder.set_project_visibility(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ProjectVisibilityType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "publicProjectAlias" => {
                            builder = builder.set_public_project_alias(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "resourceAccessRole" => {
                            builder = builder.set_resource_access_role(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "autoRetryLimit" => {
                            builder = builder.set_auto_retry_limit(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
