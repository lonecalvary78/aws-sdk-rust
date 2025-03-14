// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_function_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::create_function::CreateFunctionOutput, crate::operation::create_function::CreateFunctionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_function::CreateFunctionError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::create_function::CreateFunctionError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "FunctionAlreadyExists" => crate::operation::create_function::CreateFunctionError::FunctionAlreadyExists({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::FunctionAlreadyExistsBuilder::default();
                output = crate::protocol_serde::shape_function_already_exists::de_function_already_exists_xml_err(_response_body, output)
                    .map_err(crate::operation::create_function::CreateFunctionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "FunctionSizeLimitExceeded" => crate::operation::create_function::CreateFunctionError::FunctionSizeLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::FunctionSizeLimitExceededBuilder::default();
                output = crate::protocol_serde::shape_function_size_limit_exceeded::de_function_size_limit_exceeded_xml_err(_response_body, output)
                    .map_err(crate::operation::create_function::CreateFunctionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidArgument" => crate::operation::create_function::CreateFunctionError::InvalidArgument({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArgumentBuilder::default();
                output = crate::protocol_serde::shape_invalid_argument::de_invalid_argument_xml_err(_response_body, output)
                    .map_err(crate::operation::create_function::CreateFunctionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TooManyFunctions" => crate::operation::create_function::CreateFunctionError::TooManyFunctions({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TooManyFunctionsBuilder::default();
                output = crate::protocol_serde::shape_too_many_functions::de_too_many_functions_xml_err(_response_body, output)
                    .map_err(crate::operation::create_function::CreateFunctionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnsupportedOperation" => crate::operation::create_function::CreateFunctionError::UnsupportedOperation({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnsupportedOperationBuilder::default();
                output = crate::protocol_serde::shape_unsupported_operation::de_unsupported_operation_xml_err(_response_body, output)
                    .map_err(crate::operation::create_function::CreateFunctionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::create_function::CreateFunctionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_function_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::create_function::CreateFunctionOutput, crate::operation::create_function::CreateFunctionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_function::builders::CreateFunctionOutputBuilder::default();
        output = output.set_e_tag(
            crate::protocol_serde::shape_create_function_output::de_e_tag_header(_response_headers)
                .map_err(|_| crate::operation::create_function::CreateFunctionError::unhandled("Failed to parse ETag from header `ETag"))?,
        );
        output = output.set_function_summary(crate::protocol_serde::shape_create_function_output::de_function_summary_payload(
            _response_body,
        )?);
        output = output.set_location(
            crate::protocol_serde::shape_create_function_output::de_location_header(_response_headers)
                .map_err(|_| crate::operation::create_function::CreateFunctionError::unhandled("Failed to parse Location from header `Location"))?,
        );
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_create_function_op_input(
    input: &crate::operation::create_function::CreateFunctionInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = ::aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("CreateFunctionRequest")
            .write_ns("http://cloudfront.amazonaws.com/doc/2020-05-31/", None);
        crate::protocol_serde::shape_create_function_input::ser_create_function_input_input_input(input, root)?
    }
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
