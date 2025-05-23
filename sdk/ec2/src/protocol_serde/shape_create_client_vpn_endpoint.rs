// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_client_vpn_endpoint_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_client_vpn_endpoint::CreateClientVpnEndpointOutput,
    crate::operation::create_client_vpn_endpoint::CreateClientVpnEndpointError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_client_vpn_endpoint::CreateClientVpnEndpointError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::create_client_vpn_endpoint::CreateClientVpnEndpointError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_client_vpn_endpoint_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_client_vpn_endpoint::CreateClientVpnEndpointOutput,
    crate::operation::create_client_vpn_endpoint::CreateClientVpnEndpointError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_client_vpn_endpoint::builders::CreateClientVpnEndpointOutputBuilder::default();
        output = crate::protocol_serde::shape_create_client_vpn_endpoint::de_create_client_vpn_endpoint(_response_body, output)
            .map_err(crate::operation::create_client_vpn_endpoint::CreateClientVpnEndpointError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_client_vpn_endpoint(
    inp: &[u8],
    mut builder: crate::operation::create_client_vpn_endpoint::builders::CreateClientVpnEndpointOutputBuilder,
) -> std::result::Result<
    crate::operation::create_client_vpn_endpoint::builders::CreateClientVpnEndpointOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateClientVpnEndpointResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateClientVpnEndpointResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("clientVpnEndpointId") /* ClientVpnEndpointId com.amazonaws.ec2.synthetic#CreateClientVpnEndpointOutput$ClientVpnEndpointId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_client_vpn_endpoint_id(var_1);
            }
            ,
            s if s.matches("status") /* Status com.amazonaws.ec2.synthetic#CreateClientVpnEndpointOutput$Status */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_client_vpn_endpoint_status::de_client_vpn_endpoint_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_status(var_2);
            }
            ,
            s if s.matches("dnsName") /* DnsName com.amazonaws.ec2.synthetic#CreateClientVpnEndpointOutput$DnsName */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_dns_name(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
