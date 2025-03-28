// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_orderable_db_instance_options_list(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<::std::vec::Vec<crate::types::OrderableDbInstanceOption>, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("OrderableDBInstanceOption") /* member com.amazonaws.rds#OrderableDBInstanceOptionsList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_orderable_db_instance_option::de_orderable_db_instance_option(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
