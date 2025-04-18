// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_dns_entry_set(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<::std::vec::Vec<crate::types::DnsEntry>, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#DnsEntrySet$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_dns_entry::de_dns_entry(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
