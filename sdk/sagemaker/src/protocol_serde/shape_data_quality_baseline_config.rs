// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_data_quality_baseline_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DataQualityBaselineConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.baselining_job_name {
        object.key("BaseliningJobName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.constraints_resource {
        #[allow(unused_mut)]
        let mut object_3 = object.key("ConstraintsResource").start_object();
        crate::protocol_serde::shape_monitoring_constraints_resource::ser_monitoring_constraints_resource(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.statistics_resource {
        #[allow(unused_mut)]
        let mut object_5 = object.key("StatisticsResource").start_object();
        crate::protocol_serde::shape_monitoring_statistics_resource::ser_monitoring_statistics_resource(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

pub(crate) fn de_data_quality_baseline_config<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::DataQualityBaselineConfig>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::DataQualityBaselineConfigBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "BaseliningJobName" => {
                            builder = builder.set_baselining_job_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ConstraintsResource" => {
                            builder = builder.set_constraints_resource(
                                crate::protocol_serde::shape_monitoring_constraints_resource::de_monitoring_constraints_resource(tokens)?,
                            );
                        }
                        "StatisticsResource" => {
                            builder = builder.set_statistics_resource(
                                crate::protocol_serde::shape_monitoring_statistics_resource::de_monitoring_statistics_resource(tokens)?,
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
