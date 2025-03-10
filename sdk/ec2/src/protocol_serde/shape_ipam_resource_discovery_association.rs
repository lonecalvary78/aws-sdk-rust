// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_ipam_resource_discovery_association(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::IpamResourceDiscoveryAssociation, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::IpamResourceDiscoveryAssociation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2#IpamResourceDiscoveryAssociation$OwnerId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_1);
            }
            ,
            s if s.matches("ipamResourceDiscoveryAssociationId") /* IpamResourceDiscoveryAssociationId com.amazonaws.ec2#IpamResourceDiscoveryAssociation$IpamResourceDiscoveryAssociationId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_resource_discovery_association_id(var_2);
            }
            ,
            s if s.matches("ipamResourceDiscoveryAssociationArn") /* IpamResourceDiscoveryAssociationArn com.amazonaws.ec2#IpamResourceDiscoveryAssociation$IpamResourceDiscoveryAssociationArn */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_resource_discovery_association_arn(var_3);
            }
            ,
            s if s.matches("ipamResourceDiscoveryId") /* IpamResourceDiscoveryId com.amazonaws.ec2#IpamResourceDiscoveryAssociation$IpamResourceDiscoveryId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_resource_discovery_id(var_4);
            }
            ,
            s if s.matches("ipamId") /* IpamId com.amazonaws.ec2#IpamResourceDiscoveryAssociation$IpamId */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_id(var_5);
            }
            ,
            s if s.matches("ipamArn") /* IpamArn com.amazonaws.ec2#IpamResourceDiscoveryAssociation$IpamArn */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_arn(var_6);
            }
            ,
            s if s.matches("ipamRegion") /* IpamRegion com.amazonaws.ec2#IpamResourceDiscoveryAssociation$IpamRegion */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_region(var_7);
            }
            ,
            s if s.matches("isDefault") /* IsDefault com.amazonaws.ec2#IpamResourceDiscoveryAssociation$IsDefault */ =>  {
                let var_8 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_default(var_8);
            }
            ,
            s if s.matches("resourceDiscoveryStatus") /* ResourceDiscoveryStatus com.amazonaws.ec2#IpamResourceDiscoveryAssociation$ResourceDiscoveryStatus */ =>  {
                let var_9 =
                    Some(
                        Result::<crate::types::IpamAssociatedResourceDiscoveryStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::IpamAssociatedResourceDiscoveryStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_discovery_status(var_9);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#IpamResourceDiscoveryAssociation$State */ =>  {
                let var_10 =
                    Some(
                        Result::<crate::types::IpamResourceDiscoveryAssociationState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::IpamResourceDiscoveryAssociationState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_10);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#IpamResourceDiscoveryAssociation$Tags */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_11);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
