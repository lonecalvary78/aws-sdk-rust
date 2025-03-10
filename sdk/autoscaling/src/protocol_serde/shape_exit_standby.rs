// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_exit_standby_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::exit_standby::ExitStandbyOutput, crate::operation::exit_standby::ExitStandbyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::exit_standby::ExitStandbyError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::exit_standby::ExitStandbyError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceContention" => crate::operation::exit_standby::ExitStandbyError::ResourceContentionFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceContentionFaultBuilder::default();
                output = crate::protocol_serde::shape_resource_contention_fault::de_resource_contention_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::exit_standby::ExitStandbyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::exit_standby::ExitStandbyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_exit_standby_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::exit_standby::ExitStandbyOutput, crate::operation::exit_standby::ExitStandbyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::exit_standby::builders::ExitStandbyOutputBuilder::default();
        output = crate::protocol_serde::shape_exit_standby::de_exit_standby(_response_body, output)
            .map_err(crate::operation::exit_standby::ExitStandbyError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_exit_standby(
    inp: &[u8],
    mut builder: crate::operation::exit_standby::builders::ExitStandbyOutputBuilder,
) -> std::result::Result<crate::operation::exit_standby::builders::ExitStandbyOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ExitStandbyResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ExitStandbyResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ExitStandbyResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ExitStandbyResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Activities") /* Activities com.amazonaws.autoscaling.synthetic#ExitStandbyOutput$Activities */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_activities::de_activities(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_activities(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected ExitStandbyResult tag"));
    };
    Ok(builder)
}
