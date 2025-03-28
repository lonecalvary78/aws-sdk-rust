// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_ipam_pool_source_resource(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::IpamPoolSourceResource, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::IpamPoolSourceResource::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("resourceId") /* ResourceId com.amazonaws.ec2#IpamPoolSourceResource$ResourceId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_id(var_1);
            }
            ,
            s if s.matches("resourceType") /* ResourceType com.amazonaws.ec2#IpamPoolSourceResource$ResourceType */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::IpamPoolSourceResourceType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::IpamPoolSourceResourceType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_type(var_2);
            }
            ,
            s if s.matches("resourceRegion") /* ResourceRegion com.amazonaws.ec2#IpamPoolSourceResource$ResourceRegion */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_region(var_3);
            }
            ,
            s if s.matches("resourceOwner") /* ResourceOwner com.amazonaws.ec2#IpamPoolSourceResource$ResourceOwner */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_owner(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
