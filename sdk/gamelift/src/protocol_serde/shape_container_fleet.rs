// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_container_fleet<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ContainerFleet>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ContainerFleetBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "FleetId" => {
                            builder = builder.set_fleet_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "FleetArn" => {
                            builder = builder.set_fleet_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "FleetRoleArn" => {
                            builder = builder.set_fleet_role_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "GameServerContainerGroupDefinitionName" => {
                            builder = builder.set_game_server_container_group_definition_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "GameServerContainerGroupDefinitionArn" => {
                            builder = builder.set_game_server_container_group_definition_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PerInstanceContainerGroupDefinitionName" => {
                            builder = builder.set_per_instance_container_group_definition_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PerInstanceContainerGroupDefinitionArn" => {
                            builder = builder.set_per_instance_container_group_definition_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "InstanceConnectionPortRange" => {
                            builder = builder.set_instance_connection_port_range(
                                crate::protocol_serde::shape_connection_port_range::de_connection_port_range(tokens)?,
                            );
                        }
                        "InstanceInboundPermissions" => {
                            builder = builder
                                .set_instance_inbound_permissions(crate::protocol_serde::shape_ip_permissions_list::de_ip_permissions_list(tokens)?);
                        }
                        "GameServerContainerGroupsPerInstance" => {
                            builder = builder.set_game_server_container_groups_per_instance(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "MaximumGameServerContainerGroupsPerInstance" => {
                            builder = builder.set_maximum_game_server_container_groups_per_instance(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "InstanceType" => {
                            builder = builder.set_instance_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "BillingType" => {
                            builder = builder.set_billing_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ContainerFleetBillingType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "Description" => {
                            builder = builder.set_description(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "CreationTime" => {
                            builder = builder.set_creation_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "MetricGroups" => {
                            builder = builder.set_metric_groups(crate::protocol_serde::shape_metric_group_list::de_metric_group_list(tokens)?);
                        }
                        "NewGameSessionProtectionPolicy" => {
                            builder = builder.set_new_game_session_protection_policy(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ProtectionPolicy::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "GameSessionCreationLimitPolicy" => {
                            builder = builder.set_game_session_creation_limit_policy(
                                crate::protocol_serde::shape_game_session_creation_limit_policy::de_game_session_creation_limit_policy(tokens)?,
                            );
                        }
                        "Status" => {
                            builder = builder.set_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ContainerFleetStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "DeploymentDetails" => {
                            builder = builder.set_deployment_details(crate::protocol_serde::shape_deployment_details::de_deployment_details(tokens)?);
                        }
                        "LogConfiguration" => {
                            builder = builder.set_log_configuration(crate::protocol_serde::shape_log_configuration::de_log_configuration(tokens)?);
                        }
                        "LocationAttributes" => {
                            builder = builder.set_location_attributes(
                                crate::protocol_serde::shape_container_fleet_location_attributes_list::de_container_fleet_location_attributes_list(
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
