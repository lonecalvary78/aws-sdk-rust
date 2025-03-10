// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_subnet(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::Subnet, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Subnet::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("SubnetIdentifier") /* SubnetIdentifier com.amazonaws.rds#Subnet$SubnetIdentifier */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_identifier(var_1);
            }
            ,
            s if s.matches("SubnetAvailabilityZone") /* SubnetAvailabilityZone com.amazonaws.rds#Subnet$SubnetAvailabilityZone */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_availability_zone::de_availability_zone(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_subnet_availability_zone(var_2);
            }
            ,
            s if s.matches("SubnetOutpost") /* SubnetOutpost com.amazonaws.rds#Subnet$SubnetOutpost */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_outpost::de_outpost(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_subnet_outpost(var_3);
            }
            ,
            s if s.matches("SubnetStatus") /* SubnetStatus com.amazonaws.rds#Subnet$SubnetStatus */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_status(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
