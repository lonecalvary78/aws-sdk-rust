// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_field_level_encryption_list_payload(
    body: &[u8],
) -> std::result::Result<
    ::std::option::Option<crate::types::FieldLevelEncryptionList>,
    crate::operation::list_field_level_encryption_configs::ListFieldLevelEncryptionConfigsError,
> {
    (!body.is_empty())
        .then(|| {
            crate::protocol_serde::shape_list_field_level_encryption_configs_output::de_field_level_encryption_list(body)
                .map_err(crate::operation::list_field_level_encryption_configs::ListFieldLevelEncryptionConfigsError::unhandled)
        })
        .transpose()
}

pub fn de_field_level_encryption_list(
    inp: &[u8],
) -> std::result::Result<crate::types::FieldLevelEncryptionList, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    let start_el = decoder.start_el();
    if !(start_el.matches("FieldLevelEncryptionList")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected FieldLevelEncryptionList got {:?}",
            start_el
        )));
    }
    crate::protocol_serde::shape_field_level_encryption_list::de_field_level_encryption_list(&mut decoder)
}
