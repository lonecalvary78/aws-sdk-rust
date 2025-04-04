// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_list_policy_granting_service_access_response_list_type(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<::std::vec::Vec<crate::types::ListPoliciesGrantingServiceAccessEntry>, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("member") /* member com.amazonaws.iam#listPolicyGrantingServiceAccessResponseListType$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_list_policies_granting_service_access_entry::de_list_policies_granting_service_access_entry(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
