// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_account_limits_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_account_limits::DescribeAccountLimitsOutput,
    crate::operation::describe_account_limits::DescribeAccountLimitsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_account_limits::DescribeAccountLimitsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::describe_account_limits::DescribeAccountLimitsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_account_limits_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_account_limits::DescribeAccountLimitsOutput,
    crate::operation::describe_account_limits::DescribeAccountLimitsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_account_limits::builders::DescribeAccountLimitsOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_account_limits::de_describe_account_limits(_response_body, output)
            .map_err(crate::operation::describe_account_limits::DescribeAccountLimitsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_account_limits(
    inp: &[u8],
    mut builder: crate::operation::describe_account_limits::builders::DescribeAccountLimitsOutputBuilder,
) -> std::result::Result<
    crate::operation::describe_account_limits::builders::DescribeAccountLimitsOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeAccountLimitsResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeAccountLimitsResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DescribeAccountLimitsResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DescribeAccountLimitsResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Limits") /* Limits com.amazonaws.elasticloadbalancingv2.synthetic#DescribeAccountLimitsOutput$Limits */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_limits::de_limits(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_limits(var_1);
            }
            ,
            s if s.matches("NextMarker") /* NextMarker com.amazonaws.elasticloadbalancingv2.synthetic#DescribeAccountLimitsOutput$NextMarker */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_marker(var_2);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected DescribeAccountLimitsResult tag",
        ));
    };
    Ok(builder)
}
