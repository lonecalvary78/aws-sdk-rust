// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_coip_pool(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::CoipPool, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::CoipPool::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("poolId") /* PoolId com.amazonaws.ec2#CoipPool$PoolId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_pool_id(var_1);
            }
            ,
            s if s.matches("poolCidrSet") /* PoolCidrs com.amazonaws.ec2#CoipPool$PoolCidrs */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_value_string_list::de_value_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_pool_cidrs(var_2);
            }
            ,
            s if s.matches("localGatewayRouteTableId") /* LocalGatewayRouteTableId com.amazonaws.ec2#CoipPool$LocalGatewayRouteTableId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_gateway_route_table_id(var_3);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#CoipPool$Tags */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_4);
            }
            ,
            s if s.matches("poolArn") /* PoolArn com.amazonaws.ec2#CoipPool$PoolArn */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_pool_arn(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
