// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_provision_public_ipv4_pool_cidr_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrOutput,
    crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_provision_public_ipv4_pool_cidr_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrOutput,
    crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrOutputBuilder::default();
        output = crate::protocol_serde::shape_provision_public_ipv4_pool_cidr::de_provision_public_ipv4_pool_cidr(_response_body, output)
            .map_err(crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_provision_public_ipv4_pool_cidr(
    inp: &[u8],
    mut builder: crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrOutputBuilder,
) -> std::result::Result<
    crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ProvisionPublicIpv4PoolCidrResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ProvisionPublicIpv4PoolCidrResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("poolId") /* PoolId com.amazonaws.ec2.synthetic#ProvisionPublicIpv4PoolCidrOutput$PoolId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_pool_id(var_1);
            }
            ,
            s if s.matches("poolAddressRange") /* PoolAddressRange com.amazonaws.ec2.synthetic#ProvisionPublicIpv4PoolCidrOutput$PoolAddressRange */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_public_ipv4_pool_range::de_public_ipv4_pool_range(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_pool_address_range(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
