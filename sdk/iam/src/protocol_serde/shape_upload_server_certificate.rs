// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_upload_server_certificate_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::upload_server_certificate::UploadServerCertificateOutput,
    crate::operation::upload_server_certificate::UploadServerCertificateError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::upload_server_certificate::UploadServerCertificateError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::upload_server_certificate::UploadServerCertificateError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentModification" => crate::operation::upload_server_certificate::UploadServerCertificateError::ConcurrentModificationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConcurrentModificationExceptionBuilder::default();
                output = crate::protocol_serde::shape_concurrent_modification_exception::de_concurrent_modification_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::upload_server_certificate::UploadServerCertificateError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EntityAlreadyExists" => crate::operation::upload_server_certificate::UploadServerCertificateError::EntityAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EntityAlreadyExistsExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_entity_already_exists_exception::de_entity_already_exists_exception_xml_err(_response_body, output)
                        .map_err(crate::operation::upload_server_certificate::UploadServerCertificateError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidInput" => crate::operation::upload_server_certificate::UploadServerCertificateError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::upload_server_certificate::UploadServerCertificateError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "KeyPairMismatch" => crate::operation::upload_server_certificate::UploadServerCertificateError::KeyPairMismatchException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::KeyPairMismatchExceptionBuilder::default();
                output = crate::protocol_serde::shape_key_pair_mismatch_exception::de_key_pair_mismatch_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::upload_server_certificate::UploadServerCertificateError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "LimitExceeded" => crate::operation::upload_server_certificate::UploadServerCertificateError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::upload_server_certificate::UploadServerCertificateError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "MalformedCertificate" => crate::operation::upload_server_certificate::UploadServerCertificateError::MalformedCertificateException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::MalformedCertificateExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_malformed_certificate_exception::de_malformed_certificate_exception_xml_err(_response_body, output)
                        .map_err(crate::operation::upload_server_certificate::UploadServerCertificateError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceFailure" => crate::operation::upload_server_certificate::UploadServerCertificateError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::upload_server_certificate::UploadServerCertificateError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::upload_server_certificate::UploadServerCertificateError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_upload_server_certificate_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::upload_server_certificate::UploadServerCertificateOutput,
    crate::operation::upload_server_certificate::UploadServerCertificateError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::upload_server_certificate::builders::UploadServerCertificateOutputBuilder::default();
        output = crate::protocol_serde::shape_upload_server_certificate::de_upload_server_certificate(_response_body, output)
            .map_err(crate::operation::upload_server_certificate::UploadServerCertificateError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_upload_server_certificate(
    inp: &[u8],
    mut builder: crate::operation::upload_server_certificate::builders::UploadServerCertificateOutputBuilder,
) -> std::result::Result<
    crate::operation::upload_server_certificate::builders::UploadServerCertificateOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("UploadServerCertificateResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected UploadServerCertificateResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("UploadServerCertificateResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected UploadServerCertificateResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("ServerCertificateMetadata") /* ServerCertificateMetadata com.amazonaws.iam.synthetic#UploadServerCertificateOutput$ServerCertificateMetadata */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_server_certificate_metadata::de_server_certificate_metadata(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_server_certificate_metadata(var_1);
            }
            ,
            s if s.matches("Tags") /* Tags com.amazonaws.iam.synthetic#UploadServerCertificateOutput$Tags */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_tag_list_type::de_tag_list_type(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_2);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected UploadServerCertificateResult tag",
        ));
    };
    Ok(builder)
}
