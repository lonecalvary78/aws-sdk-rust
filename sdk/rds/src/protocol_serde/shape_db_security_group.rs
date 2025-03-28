// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_db_security_group(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::DbSecurityGroup, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::DbSecurityGroup::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("OwnerId") /* OwnerId com.amazonaws.rds#DBSecurityGroup$OwnerId */ =>  {
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
            s if s.matches("DBSecurityGroupName") /* DBSecurityGroupName com.amazonaws.rds#DBSecurityGroup$DBSecurityGroupName */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_security_group_name(var_2);
            }
            ,
            s if s.matches("DBSecurityGroupDescription") /* DBSecurityGroupDescription com.amazonaws.rds#DBSecurityGroup$DBSecurityGroupDescription */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_security_group_description(var_3);
            }
            ,
            s if s.matches("VpcId") /* VpcId com.amazonaws.rds#DBSecurityGroup$VpcId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_4);
            }
            ,
            s if s.matches("EC2SecurityGroups") /* EC2SecurityGroups com.amazonaws.rds#DBSecurityGroup$EC2SecurityGroups */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_ec2_security_group_list::de_ec2_security_group_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ec2_security_groups(var_5);
            }
            ,
            s if s.matches("IPRanges") /* IPRanges com.amazonaws.rds#DBSecurityGroup$IPRanges */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_ip_range_list::de_ip_range_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ip_ranges(var_6);
            }
            ,
            s if s.matches("DBSecurityGroupArn") /* DBSecurityGroupArn com.amazonaws.rds#DBSecurityGroup$DBSecurityGroupArn */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_security_group_arn(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
