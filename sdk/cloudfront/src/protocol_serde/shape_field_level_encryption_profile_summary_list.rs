// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_field_level_encryption_profile_summary_list(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<::std::vec::Vec<crate::types::FieldLevelEncryptionProfileSummary>, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("FieldLevelEncryptionProfileSummary") /* member com.amazonaws.cloudfront#FieldLevelEncryptionProfileSummaryList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_field_level_encryption_profile_summary::de_field_level_encryption_profile_summary(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
