// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_thing_registration_task_reports_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsOutput,
    crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalFailureException" => {
            crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError::InternalFailureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalFailureExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(_response_body, output)
                        .map_err(crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidRequestException" => {
            crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError::InvalidRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                        .map_err(crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ThrottlingException" => {
            crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                        .map_err(crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UnauthorizedException" => {
            crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError::UnauthorizedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
                        .map_err(crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_thing_registration_task_reports_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsOutput,
    crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_thing_registration_task_reports::builders::ListThingRegistrationTaskReportsOutputBuilder::default();
        output = crate::protocol_serde::shape_list_thing_registration_task_reports::de_list_thing_registration_task_reports(_response_body, output)
            .map_err(crate::operation::list_thing_registration_task_reports::ListThingRegistrationTaskReportsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_thing_registration_task_reports(
    value: &[u8],
    mut builder: crate::operation::list_thing_registration_task_reports::builders::ListThingRegistrationTaskReportsOutputBuilder,
) -> ::std::result::Result<
    crate::operation::list_thing_registration_task_reports::builders::ListThingRegistrationTaskReportsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "nextToken" => {
                    builder = builder.set_next_token(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "reportType" => {
                    builder = builder.set_report_type(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::ReportType::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "resourceLinks" => {
                    builder = builder.set_resource_links(crate::protocol_serde::shape_s3_file_url_list::de_s3_file_url_list(tokens)?);
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
