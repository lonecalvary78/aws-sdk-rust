// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_launch_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::LaunchConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.package_name {
        object.key("packageName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.launch_file {
        object.key("launchFile").string(var_2.as_str());
    }
    if let Some(var_3) = &input.environment_variables {
        #[allow(unused_mut)]
        let mut object_4 = object.key("environmentVariables").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    if let Some(var_7) = &input.port_forwarding_config {
        #[allow(unused_mut)]
        let mut object_8 = object.key("portForwardingConfig").start_object();
        crate::protocol_serde::shape_port_forwarding_config::ser_port_forwarding_config(&mut object_8, var_7)?;
        object_8.finish();
    }
    if input.stream_ui {
        object.key("streamUI").boolean(input.stream_ui);
    }
    if let Some(var_9) = &input.command {
        let mut array_10 = object.key("command").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    Ok(())
}

pub(crate) fn de_launch_config<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::LaunchConfig>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::LaunchConfigBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "packageName" => {
                            builder = builder.set_package_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "launchFile" => {
                            builder = builder.set_launch_file(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "environmentVariables" => {
                            builder = builder.set_environment_variables(
                                crate::protocol_serde::shape_environment_variable_map::de_environment_variable_map(tokens)?,
                            );
                        }
                        "portForwardingConfig" => {
                            builder = builder
                                .set_port_forwarding_config(crate::protocol_serde::shape_port_forwarding_config::de_port_forwarding_config(tokens)?);
                        }
                        "streamUI" => {
                            builder = builder.set_stream_ui(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "command" => {
                            builder = builder.set_command(crate::protocol_serde::shape_command_list::de_command_list(tokens)?);
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
