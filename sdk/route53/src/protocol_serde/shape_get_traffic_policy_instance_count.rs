// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_traffic_policy_instance_count_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_traffic_policy_instance_count::GetTrafficPolicyInstanceCountOutput,
    crate::operation::get_traffic_policy_instance_count::GetTrafficPolicyInstanceCountError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_traffic_policy_instance_count::GetTrafficPolicyInstanceCountError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::get_traffic_policy_instance_count::GetTrafficPolicyInstanceCountError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_traffic_policy_instance_count_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_traffic_policy_instance_count::GetTrafficPolicyInstanceCountOutput,
    crate::operation::get_traffic_policy_instance_count::GetTrafficPolicyInstanceCountError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_traffic_policy_instance_count::builders::GetTrafficPolicyInstanceCountOutputBuilder::default();
        output = crate::protocol_serde::shape_get_traffic_policy_instance_count::de_get_traffic_policy_instance_count(_response_body, output)
            .map_err(crate::operation::get_traffic_policy_instance_count::GetTrafficPolicyInstanceCountError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::get_traffic_policy_instance_count_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::get_traffic_policy_instance_count::GetTrafficPolicyInstanceCountError::unhandled)?
    })
}

#[allow(unused_mut)]
pub fn de_get_traffic_policy_instance_count(
    inp: &[u8],
    mut builder: crate::operation::get_traffic_policy_instance_count::builders::GetTrafficPolicyInstanceCountOutputBuilder,
) -> std::result::Result<
    crate::operation::get_traffic_policy_instance_count::builders::GetTrafficPolicyInstanceCountOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("GetTrafficPolicyInstanceCountResponse") {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "encountered invalid XML root: expected GetTrafficPolicyInstanceCountResponse but got {:?}. This is likely a bug in the SDK.",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("TrafficPolicyInstanceCount") /* TrafficPolicyInstanceCount com.amazonaws.route53.synthetic#GetTrafficPolicyInstanceCountOutput$TrafficPolicyInstanceCount */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.route53#TrafficPolicyInstanceCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_traffic_policy_instance_count(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
