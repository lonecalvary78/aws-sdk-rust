// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_origin_access_control_list_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::OriginAccessControlList>,
    crate::operation::list_origin_access_controls::ListOriginAccessControlsError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_list_origin_access_controls_output::de_origin_access_control_list(body)
                .map_err(crate::operation::list_origin_access_controls::ListOriginAccessControlsError::unhandled)
        })
        .transpose()
}

pub fn de_origin_access_control_list(
    inp: &[u8],
) -> std::result::Result<crate::types::OriginAccessControlList, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    let start_el = decoder.start_el();
    if !(start_el.matches("OriginAccessControlList")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected OriginAccessControlList got {:?}",
            start_el
        )));
    }
    crate::protocol_serde::shape_origin_access_control_list::de_origin_access_control_list(&mut decoder)
}
