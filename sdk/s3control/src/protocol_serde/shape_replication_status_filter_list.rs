// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_replication_status_filter_list(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<::std::vec::Vec<crate::types::ReplicationStatus>, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("member") /* member com.amazonaws.s3control#ReplicationStatusFilterList$member */ =>  {
                out.push(
                    Result::<crate::types::ReplicationStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                        crate::types::ReplicationStatus::from(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                        )
                    )
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
