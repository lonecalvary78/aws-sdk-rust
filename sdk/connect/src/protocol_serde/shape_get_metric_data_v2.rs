// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_metric_data_v2_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_metric_data_v2::GetMetricDataV2Output, crate::operation::get_metric_data_v2::GetMetricDataV2Error> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_metric_data_v2::GetMetricDataV2Error::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_metric_data_v2::GetMetricDataV2Error::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::operation::get_metric_data_v2::GetMetricDataV2Error::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_metric_data_v2::GetMetricDataV2Error::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterException" => crate::operation::get_metric_data_v2::GetMetricDataV2Error::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_metric_data_v2::GetMetricDataV2Error::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidRequestException" => crate::operation::get_metric_data_v2::GetMetricDataV2Error::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_metric_data_v2::GetMetricDataV2Error::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::get_metric_data_v2::GetMetricDataV2Error::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_metric_data_v2::GetMetricDataV2Error::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ThrottlingException" => crate::operation::get_metric_data_v2::GetMetricDataV2Error::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_metric_data_v2::GetMetricDataV2Error::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_metric_data_v2::GetMetricDataV2Error::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_metric_data_v2_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_metric_data_v2::GetMetricDataV2Output, crate::operation::get_metric_data_v2::GetMetricDataV2Error> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_metric_data_v2::builders::GetMetricDataV2OutputBuilder::default();
        output = crate::protocol_serde::shape_get_metric_data_v2::de_get_metric_data_v2(_response_body, output)
            .map_err(crate::operation::get_metric_data_v2::GetMetricDataV2Error::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_get_metric_data_v2_input(
    input: &crate::operation::get_metric_data_v2::GetMetricDataV2Input,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_metric_data_v2_input::ser_get_metric_data_v2_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_metric_data_v2(
    value: &[u8],
    mut builder: crate::operation::get_metric_data_v2::builders::GetMetricDataV2OutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_metric_data_v2::builders::GetMetricDataV2OutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "MetricResults" => {
                    builder = builder.set_metric_results(crate::protocol_serde::shape_metric_results_v2::de_metric_results_v2(tokens)?);
                }
                "NextToken" => {
                    builder = builder.set_next_token(
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
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
