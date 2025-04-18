// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_service_deployments_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_service_deployments::DescribeServiceDeploymentsOutput,
    crate::operation::describe_service_deployments::DescribeServiceDeploymentsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ClientException" => crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::ClientException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClientExceptionBuilder::default();
                output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ClusterNotFoundException" => crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::ClusterNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClusterNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_cluster_not_found_exception::de_cluster_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterException" => crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServerException" => crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::ServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_server_exception::de_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceNotFoundException" => crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::ServiceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_not_found_exception::de_service_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnsupportedFeatureException" => {
            crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::UnsupportedFeatureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedFeatureExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_unsupported_feature_exception::de_unsupported_feature_exception_json_err(_response_body, output)
                            .map_err(crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_service_deployments_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_service_deployments::DescribeServiceDeploymentsOutput,
    crate::operation::describe_service_deployments::DescribeServiceDeploymentsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_service_deployments::builders::DescribeServiceDeploymentsOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_service_deployments::de_describe_service_deployments(_response_body, output)
            .map_err(crate::operation::describe_service_deployments::DescribeServiceDeploymentsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_describe_service_deployments_input(
    input: &crate::operation::describe_service_deployments::DescribeServiceDeploymentsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_service_deployments_input::ser_describe_service_deployments_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_describe_service_deployments(
    value: &[u8],
    mut builder: crate::operation::describe_service_deployments::builders::DescribeServiceDeploymentsOutputBuilder,
) -> ::std::result::Result<
    crate::operation::describe_service_deployments::builders::DescribeServiceDeploymentsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "serviceDeployments" => {
                    builder = builder.set_service_deployments(crate::protocol_serde::shape_service_deployments::de_service_deployments(tokens)?);
                }
                "failures" => {
                    builder = builder.set_failures(crate::protocol_serde::shape_failures::de_failures(tokens)?);
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
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
