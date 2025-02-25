// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_hook_results_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::list_hook_results::ListHookResultsOutput, crate::operation::list_hook_results::ListHookResultsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_hook_results::ListHookResultsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::list_hook_results::ListHookResultsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "HookResultNotFound" => crate::operation::list_hook_results::ListHookResultsError::HookResultNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::HookResultNotFoundExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_hook_result_not_found_exception::de_hook_result_not_found_exception_xml_err(_response_body, output)
                        .map_err(crate::operation::list_hook_results::ListHookResultsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::list_hook_results::ListHookResultsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_hook_results_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::list_hook_results::ListHookResultsOutput, crate::operation::list_hook_results::ListHookResultsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_hook_results::builders::ListHookResultsOutputBuilder::default();
        output = crate::protocol_serde::shape_list_hook_results::de_list_hook_results(_response_body, output)
            .map_err(crate::operation::list_hook_results::ListHookResultsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_list_hook_results(
    inp: &[u8],
    mut builder: crate::operation::list_hook_results::builders::ListHookResultsOutputBuilder,
) -> std::result::Result<crate::operation::list_hook_results::builders::ListHookResultsOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ListHookResultsResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ListHookResultsResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ListHookResultsResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ListHookResultsResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("TargetType") /* TargetType com.amazonaws.cloudformation.synthetic#ListHookResultsOutput$TargetType */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::ListHookResultsTargetType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ListHookResultsTargetType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_target_type(var_1);
            }
            ,
            s if s.matches("TargetId") /* TargetId com.amazonaws.cloudformation.synthetic#ListHookResultsOutput$TargetId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_target_id(var_2);
            }
            ,
            s if s.matches("HookResults") /* HookResults com.amazonaws.cloudformation.synthetic#ListHookResultsOutput$HookResults */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_hook_result_summaries::de_hook_result_summaries(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_hook_results(var_3);
            }
            ,
            s if s.matches("NextToken") /* NextToken com.amazonaws.cloudformation.synthetic#ListHookResultsOutput$NextToken */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_4);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected ListHookResultsResult tag"));
    };
    Ok(builder)
}
