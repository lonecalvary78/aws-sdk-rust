// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_job_metadata<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::JobMetadata>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::JobMetadataBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "JobId" => {
                            builder = builder.set_job_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "JobState" => {
                            builder = builder.set_job_state(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::JobState::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "JobType" => {
                            builder = builder.set_job_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::JobType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "SnowballType" => {
                            builder = builder.set_snowball_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::SnowballType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "CreationDate" => {
                            builder = builder.set_creation_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "Resources" => {
                            builder = builder.set_resources(crate::protocol_serde::shape_job_resource::de_job_resource(tokens)?);
                        }
                        "Description" => {
                            builder = builder.set_description(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "KmsKeyARN" => {
                            builder = builder.set_kms_key_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "RoleARN" => {
                            builder = builder.set_role_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "AddressId" => {
                            builder = builder.set_address_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ShippingDetails" => {
                            builder = builder.set_shipping_details(crate::protocol_serde::shape_shipping_details::de_shipping_details(tokens)?);
                        }
                        "SnowballCapacityPreference" => {
                            builder = builder.set_snowball_capacity_preference(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::SnowballCapacity::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "Notification" => {
                            builder = builder.set_notification(crate::protocol_serde::shape_notification::de_notification(tokens)?);
                        }
                        "DataTransferProgress" => {
                            builder = builder.set_data_transfer_progress(crate::protocol_serde::shape_data_transfer::de_data_transfer(tokens)?);
                        }
                        "JobLogInfo" => {
                            builder = builder.set_job_log_info(crate::protocol_serde::shape_job_logs::de_job_logs(tokens)?);
                        }
                        "ClusterId" => {
                            builder = builder.set_cluster_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ForwardingAddressId" => {
                            builder = builder.set_forwarding_address_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TaxDocuments" => {
                            builder = builder.set_tax_documents(crate::protocol_serde::shape_tax_documents::de_tax_documents(tokens)?);
                        }
                        "DeviceConfiguration" => {
                            builder =
                                builder.set_device_configuration(crate::protocol_serde::shape_device_configuration::de_device_configuration(tokens)?);
                        }
                        "RemoteManagement" => {
                            builder = builder.set_remote_management(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::RemoteManagement::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "LongTermPricingId" => {
                            builder = builder.set_long_term_pricing_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "OnDeviceServiceConfiguration" => {
                            builder = builder.set_on_device_service_configuration(
                                crate::protocol_serde::shape_on_device_service_configuration::de_on_device_service_configuration(tokens)?,
                            );
                        }
                        "ImpactLevel" => {
                            builder = builder.set_impact_level(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ImpactLevel::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "PickupDetails" => {
                            builder = builder.set_pickup_details(crate::protocol_serde::shape_pickup_details::de_pickup_details(tokens)?);
                        }
                        "SnowballId" => {
                            builder = builder.set_snowball_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
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
