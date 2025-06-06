// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_resource_details<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ResourceDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {
            match tokens.next().transpose()? {
                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
                        tokens.peek()
                    {
                        let _ = tokens.next().expect("peek returned a token")?;
                        continue;
                    }
                    let key = key.to_unescaped()?;
                    if key == "__type" {
                        ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                        continue;
                    }
                    if variant.is_some() {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            "encountered mixed variants in union",
                        ));
                    }
                    variant = match key.as_ref() {
                        "lambdaFunction" => Some(crate::types::ResourceDetails::LambdaFunction(
                            crate::protocol_serde::shape_lambda_function::de_lambda_function(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'lambdaFunction' cannot be null")
                            })?,
                        )),
                        "ecsService" => Some(crate::types::ResourceDetails::EcsService(
                            crate::protocol_serde::shape_ecs_service::de_ecs_service(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'ecsService' cannot be null")
                            })?,
                        )),
                        "ec2Instance" => Some(crate::types::ResourceDetails::Ec2Instance(
                            crate::protocol_serde::shape_ec2_instance::de_ec2_instance(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'ec2Instance' cannot be null")
                            })?,
                        )),
                        "ebsVolume" => Some(crate::types::ResourceDetails::EbsVolume(
                            crate::protocol_serde::shape_ebs_volume::de_ebs_volume(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'ebsVolume' cannot be null")
                            })?,
                        )),
                        "ec2AutoScalingGroup" => Some(crate::types::ResourceDetails::Ec2AutoScalingGroup(
                            crate::protocol_serde::shape_ec2_auto_scaling_group::de_ec2_auto_scaling_group(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'ec2AutoScalingGroup' cannot be null")
                            })?,
                        )),
                        "ec2ReservedInstances" => Some(crate::types::ResourceDetails::Ec2ReservedInstances(
                            crate::protocol_serde::shape_ec2_reserved_instances::de_ec2_reserved_instances(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'ec2ReservedInstances' cannot be null")
                            })?,
                        )),
                        "rdsReservedInstances" => Some(crate::types::ResourceDetails::RdsReservedInstances(
                            crate::protocol_serde::shape_rds_reserved_instances::de_rds_reserved_instances(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'rdsReservedInstances' cannot be null")
                            })?,
                        )),
                        "elastiCacheReservedInstances" => Some(crate::types::ResourceDetails::ElastiCacheReservedInstances(
                            crate::protocol_serde::shape_elasti_cache_reserved_instances::de_elasti_cache_reserved_instances(tokens)?.ok_or_else(
                                || {
                                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                        "value for 'elastiCacheReservedInstances' cannot be null",
                                    )
                                },
                            )?,
                        )),
                        "openSearchReservedInstances" => Some(crate::types::ResourceDetails::OpenSearchReservedInstances(
                            crate::protocol_serde::shape_open_search_reserved_instances::de_open_search_reserved_instances(tokens)?.ok_or_else(
                                || {
                                    ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                        "value for 'openSearchReservedInstances' cannot be null",
                                    )
                                },
                            )?,
                        )),
                        "redshiftReservedInstances" => Some(crate::types::ResourceDetails::RedshiftReservedInstances(
                            crate::protocol_serde::shape_redshift_reserved_instances::de_redshift_reserved_instances(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                    "value for 'redshiftReservedInstances' cannot be null",
                                )
                            })?,
                        )),
                        "ec2InstanceSavingsPlans" => Some(crate::types::ResourceDetails::Ec2InstanceSavingsPlans(
                            crate::protocol_serde::shape_ec2_instance_savings_plans::de_ec2_instance_savings_plans(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'ec2InstanceSavingsPlans' cannot be null")
                            })?,
                        )),
                        "computeSavingsPlans" => Some(crate::types::ResourceDetails::ComputeSavingsPlans(
                            crate::protocol_serde::shape_compute_savings_plans::de_compute_savings_plans(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'computeSavingsPlans' cannot be null")
                            })?,
                        )),
                        "sageMakerSavingsPlans" => Some(crate::types::ResourceDetails::SageMakerSavingsPlans(
                            crate::protocol_serde::shape_sage_maker_savings_plans::de_sage_maker_savings_plans(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'sageMakerSavingsPlans' cannot be null")
                            })?,
                        )),
                        "rdsDbInstance" => Some(crate::types::ResourceDetails::RdsDbInstance(
                            crate::protocol_serde::shape_rds_db_instance::de_rds_db_instance(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'rdsDbInstance' cannot be null")
                            })?,
                        )),
                        "rdsDbInstanceStorage" => Some(crate::types::ResourceDetails::RdsDbInstanceStorage(
                            crate::protocol_serde::shape_rds_db_instance_storage::de_rds_db_instance_storage(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'rdsDbInstanceStorage' cannot be null")
                            })?,
                        )),
                        "auroraDbClusterStorage" => Some(crate::types::ResourceDetails::AuroraDbClusterStorage(
                            crate::protocol_serde::shape_aurora_db_cluster_storage::de_aurora_db_cluster_storage(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'auroraDbClusterStorage' cannot be null")
                            })?,
                        )),
                        "dynamoDbReservedCapacity" => Some(crate::types::ResourceDetails::DynamoDbReservedCapacity(
                            crate::protocol_serde::shape_dynamo_db_reserved_capacity::de_dynamo_db_reserved_capacity(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'dynamoDbReservedCapacity' cannot be null")
                            })?,
                        )),
                        "memoryDbReservedInstances" => Some(crate::types::ResourceDetails::MemoryDbReservedInstances(
                            crate::protocol_serde::shape_memory_db_reserved_instances::de_memory_db_reserved_instances(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom(
                                    "value for 'memoryDbReservedInstances' cannot be null",
                                )
                            })?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::ResourceDetails::Unknown)
                        }
                    };
                }
                other => {
                    return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )))
                }
            }
        },
        _ => {
            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ))
        }
    }
    if variant.is_none() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "Union did not contain a valid variant.",
        ));
    }
    Ok(variant)
}
