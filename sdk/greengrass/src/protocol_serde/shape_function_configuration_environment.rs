// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_function_configuration_environment(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::FunctionConfigurationEnvironment,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.access_sysfs {
        object.key("AccessSysfs").boolean(*var_1);
    }
    if let Some(var_2) = &input.execution {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Execution").start_object();
        crate::protocol_serde::shape_function_execution_config::ser_function_execution_config(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.resource_access_policies {
        let mut array_5 = object.key("ResourceAccessPolicies").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_resource_access_policy::ser_resource_access_policy(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.variables {
        #[allow(unused_mut)]
        let mut object_9 = object.key("Variables").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    Ok(())
}

pub(crate) fn de_function_configuration_environment<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::FunctionConfigurationEnvironment>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::FunctionConfigurationEnvironmentBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "AccessSysfs" => {
                            builder = builder.set_access_sysfs(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "Execution" => {
                            builder = builder.set_execution(crate::protocol_serde::shape_function_execution_config::de_function_execution_config(
                                tokens,
                            )?);
                        }
                        "ResourceAccessPolicies" => {
                            builder = builder.set_resource_access_policies(
                                crate::protocol_serde::shape_list_of_resource_access_policy::de_list_of_resource_access_policy(tokens)?,
                            );
                        }
                        "Variables" => {
                            builder = builder.set_variables(crate::protocol_serde::shape_map_of_string::de_map_of_string(tokens)?);
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
