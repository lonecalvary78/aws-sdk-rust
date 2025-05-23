// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_signin_delegate_groups_with_account_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountOutput,
    crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(
                crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::unhandled(generic),
            )
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output).map_err(crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "ForbiddenException" => crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ForbiddenExceptionBuilder::default();
                    output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(_response_body, output).map_err(crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "NotFoundException" => crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output).map_err(crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "ServiceFailureException" => crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_json_err(_response_body, output).map_err(crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "ServiceUnavailableException" => crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(_response_body, output).map_err(crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "ThrottledClientException" => crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::ThrottledClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottledClientExceptionBuilder::default();
                    output = crate::protocol_serde::shape_throttled_client_exception::de_throttled_client_exception_json_err(_response_body, output).map_err(crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "UnauthorizedClientException" => crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::UnauthorizedClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnauthorizedClientExceptionBuilder::default();
                    output = crate::protocol_serde::shape_unauthorized_client_exception::de_unauthorized_client_exception_json_err(_response_body, output).map_err(crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        _ => crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_signin_delegate_groups_with_account_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountOutput,
    crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::associate_signin_delegate_groups_with_account::builders::AssociateSigninDelegateGroupsWithAccountOutputBuilder::default(
            );
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_associate_signin_delegate_groups_with_account_input(
    input: &crate::operation::associate_signin_delegate_groups_with_account::AssociateSigninDelegateGroupsWithAccountInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_associate_signin_delegate_groups_with_account_input::ser_associate_signin_delegate_groups_with_account_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
