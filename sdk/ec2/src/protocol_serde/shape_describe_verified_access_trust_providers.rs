// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_verified_access_trust_providers_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_verified_access_trust_providers::DescribeVerifiedAccessTrustProvidersOutput,
    crate::operation::describe_verified_access_trust_providers::DescribeVerifiedAccessTrustProvidersError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_verified_access_trust_providers::DescribeVerifiedAccessTrustProvidersError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::describe_verified_access_trust_providers::DescribeVerifiedAccessTrustProvidersError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_verified_access_trust_providers_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_verified_access_trust_providers::DescribeVerifiedAccessTrustProvidersOutput,
    crate::operation::describe_verified_access_trust_providers::DescribeVerifiedAccessTrustProvidersError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_verified_access_trust_providers::de_describe_verified_access_trust_providers(
            _response_body,
            output,
        )
        .map_err(crate::operation::describe_verified_access_trust_providers::DescribeVerifiedAccessTrustProvidersError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_verified_access_trust_providers(
    inp: &[u8],
    mut builder: crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersOutputBuilder,
) -> std::result::Result<
    crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeVerifiedAccessTrustProvidersResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeVerifiedAccessTrustProvidersResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("verifiedAccessTrustProviderSet") /* VerifiedAccessTrustProviders com.amazonaws.ec2.synthetic#DescribeVerifiedAccessTrustProvidersOutput$VerifiedAccessTrustProviders */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_verified_access_trust_provider_list::de_verified_access_trust_provider_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_verified_access_trust_providers(var_1);
            }
            ,
            s if s.matches("nextToken") /* NextToken com.amazonaws.ec2.synthetic#DescribeVerifiedAccessTrustProvidersOutput$NextToken */ =>  {
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
    Ok(builder)
}
