// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_resource_scan_resources_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_resource_scan_resources::ListResourceScanResourcesOutput,
    crate::operation::list_resource_scan_resources::ListResourceScanResourcesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_resource_scan_resources::ListResourceScanResourcesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::list_resource_scan_resources::ListResourceScanResourcesError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceScanInProgress" => {
            crate::operation::list_resource_scan_resources::ListResourceScanResourcesError::ResourceScanInProgressException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceScanInProgressExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_scan_in_progress_exception::de_resource_scan_in_progress_exception_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::list_resource_scan_resources::ListResourceScanResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceScanNotFound" => crate::operation::list_resource_scan_resources::ListResourceScanResourcesError::ResourceScanNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceScanNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_scan_not_found_exception::de_resource_scan_not_found_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::list_resource_scan_resources::ListResourceScanResourcesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::list_resource_scan_resources::ListResourceScanResourcesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_resource_scan_resources_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_resource_scan_resources::ListResourceScanResourcesOutput,
    crate::operation::list_resource_scan_resources::ListResourceScanResourcesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_resource_scan_resources::builders::ListResourceScanResourcesOutputBuilder::default();
        output = crate::protocol_serde::shape_list_resource_scan_resources::de_list_resource_scan_resources(_response_body, output)
            .map_err(crate::operation::list_resource_scan_resources::ListResourceScanResourcesError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_list_resource_scan_resources(
    inp: &[u8],
    mut builder: crate::operation::list_resource_scan_resources::builders::ListResourceScanResourcesOutputBuilder,
) -> std::result::Result<
    crate::operation::list_resource_scan_resources::builders::ListResourceScanResourcesOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ListResourceScanResourcesResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ListResourceScanResourcesResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ListResourceScanResourcesResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ListResourceScanResourcesResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Resources") /* Resources com.amazonaws.cloudformation.synthetic#ListResourceScanResourcesOutput$Resources */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_scanned_resources::de_scanned_resources(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_resources(var_1);
            }
            ,
            s if s.matches("NextToken") /* NextToken com.amazonaws.cloudformation.synthetic#ListResourceScanResourcesOutput$NextToken */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_2);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected ListResourceScanResourcesResult tag",
        ));
    };
    Ok(builder)
}
