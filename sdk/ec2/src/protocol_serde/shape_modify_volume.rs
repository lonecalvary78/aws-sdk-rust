// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_volume_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::modify_volume::ModifyVolumeOutput, crate::operation::modify_volume::ModifyVolumeError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::modify_volume::ModifyVolumeError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::modify_volume::ModifyVolumeError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_volume_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::modify_volume::ModifyVolumeOutput, crate::operation::modify_volume::ModifyVolumeError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_volume::builders::ModifyVolumeOutputBuilder::default();
        output = crate::protocol_serde::shape_modify_volume::de_modify_volume(_response_body, output)
            .map_err(crate::operation::modify_volume::ModifyVolumeError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_volume(
    inp: &[u8],
    mut builder: crate::operation::modify_volume::builders::ModifyVolumeOutputBuilder,
) -> std::result::Result<crate::operation::modify_volume::builders::ModifyVolumeOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyVolumeResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyVolumeResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("volumeModification") /* VolumeModification com.amazonaws.ec2.synthetic#ModifyVolumeOutput$VolumeModification */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_volume_modification::de_volume_modification(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_volume_modification(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
