// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_ssl_policies_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_ssl_policies::DescribeSslPoliciesOutput,
    crate::operation::describe_ssl_policies::DescribeSSLPoliciesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_ssl_policies::DescribeSSLPoliciesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_ssl_policies::DescribeSSLPoliciesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "SSLPolicyNotFound" => crate::operation::describe_ssl_policies::DescribeSSLPoliciesError::SslPolicyNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::SslPolicyNotFoundExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_ssl_policy_not_found_exception::de_ssl_policy_not_found_exception_xml_err(_response_body, output)
                        .map_err(crate::operation::describe_ssl_policies::DescribeSSLPoliciesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::describe_ssl_policies::DescribeSSLPoliciesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_ssl_policies_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_ssl_policies::DescribeSslPoliciesOutput,
    crate::operation::describe_ssl_policies::DescribeSSLPoliciesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_ssl_policies::builders::DescribeSslPoliciesOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_ssl_policies::de_describe_ssl_policies(_response_body, output)
            .map_err(crate::operation::describe_ssl_policies::DescribeSSLPoliciesError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_ssl_policies(
    inp: &[u8],
    mut builder: crate::operation::describe_ssl_policies::builders::DescribeSslPoliciesOutputBuilder,
) -> std::result::Result<crate::operation::describe_ssl_policies::builders::DescribeSslPoliciesOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
{
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeSSLPoliciesResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeSSLPoliciesResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DescribeSSLPoliciesResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DescribeSSLPoliciesResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("SslPolicies") /* SslPolicies com.amazonaws.elasticloadbalancingv2.synthetic#DescribeSSLPoliciesOutput$SslPolicies */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_ssl_policies::de_ssl_policies(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ssl_policies(var_1);
            }
            ,
            s if s.matches("NextMarker") /* NextMarker com.amazonaws.elasticloadbalancingv2.synthetic#DescribeSSLPoliciesOutput$NextMarker */ =>  {
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
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeSSLPoliciesResult tag"));
    };
    Ok(builder)
}
