// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_application_resource_lifecycle_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_application_resource_lifecycle::UpdateApplicationResourceLifecycleOutput,
    crate::operation::update_application_resource_lifecycle::UpdateApplicationResourceLifecycleError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_application_resource_lifecycle::UpdateApplicationResourceLifecycleError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::update_application_resource_lifecycle::UpdateApplicationResourceLifecycleError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InsufficientPrivilegesException" => {
            crate::operation::update_application_resource_lifecycle::UpdateApplicationResourceLifecycleError::InsufficientPrivilegesException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InsufficientPrivilegesExceptionBuilder::default();
                    output = crate::protocol_serde::shape_insufficient_privileges_exception::de_insufficient_privileges_exception_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::update_application_resource_lifecycle::UpdateApplicationResourceLifecycleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::update_application_resource_lifecycle::UpdateApplicationResourceLifecycleError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_application_resource_lifecycle_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_application_resource_lifecycle::UpdateApplicationResourceLifecycleOutput,
    crate::operation::update_application_resource_lifecycle::UpdateApplicationResourceLifecycleError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::update_application_resource_lifecycle::builders::UpdateApplicationResourceLifecycleOutputBuilder::default();
        output = crate::protocol_serde::shape_update_application_resource_lifecycle::de_update_application_resource_lifecycle(_response_body, output)
            .map_err(crate::operation::update_application_resource_lifecycle::UpdateApplicationResourceLifecycleError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_update_application_resource_lifecycle(
    inp: &[u8],
    mut builder: crate::operation::update_application_resource_lifecycle::builders::UpdateApplicationResourceLifecycleOutputBuilder,
) -> std::result::Result<
    crate::operation::update_application_resource_lifecycle::builders::UpdateApplicationResourceLifecycleOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("UpdateApplicationResourceLifecycleResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected UpdateApplicationResourceLifecycleResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("UpdateApplicationResourceLifecycleResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected UpdateApplicationResourceLifecycleResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("ApplicationName") /* ApplicationName com.amazonaws.elasticbeanstalk.synthetic#UpdateApplicationResourceLifecycleOutput$ApplicationName */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_application_name(var_1);
            }
            ,
            s if s.matches("ResourceLifecycleConfig") /* ResourceLifecycleConfig com.amazonaws.elasticbeanstalk.synthetic#UpdateApplicationResourceLifecycleOutput$ResourceLifecycleConfig */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_application_resource_lifecycle_config::de_application_resource_lifecycle_config(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_resource_lifecycle_config(var_2);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected UpdateApplicationResourceLifecycleResult tag",
        ));
    };
    Ok(builder)
}
