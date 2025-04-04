// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_transit_gateway_multicast_group_members_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersOutput,
    crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(
        crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersError::unhandled,
    )?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_transit_gateway_multicast_group_members_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersOutput,
    crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersOutputBuilder::default();
        output =
            crate::protocol_serde::shape_deregister_transit_gateway_multicast_group_members::de_deregister_transit_gateway_multicast_group_members(
                _response_body,
                output,
            )
            .map_err(
                crate::operation::deregister_transit_gateway_multicast_group_members::DeregisterTransitGatewayMulticastGroupMembersError::unhandled,
            )?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_deregister_transit_gateway_multicast_group_members(
    inp: &[u8],
    mut builder: crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersOutputBuilder,
) -> std::result::Result<
    crate::operation::deregister_transit_gateway_multicast_group_members::builders::DeregisterTransitGatewayMulticastGroupMembersOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeregisterTransitGatewayMulticastGroupMembersResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeregisterTransitGatewayMulticastGroupMembersResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("deregisteredMulticastGroupMembers") /* DeregisteredMulticastGroupMembers com.amazonaws.ec2.synthetic#DeregisterTransitGatewayMulticastGroupMembersOutput$DeregisteredMulticastGroupMembers */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_transit_gateway_multicast_deregistered_group_members::de_transit_gateway_multicast_deregistered_group_members(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_deregistered_multicast_group_members(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
