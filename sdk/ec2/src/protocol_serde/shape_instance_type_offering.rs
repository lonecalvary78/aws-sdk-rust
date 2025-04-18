// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_instance_type_offering(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::InstanceTypeOffering, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InstanceTypeOffering::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceType") /* InstanceType com.amazonaws.ec2#InstanceTypeOffering$InstanceType */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::InstanceType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InstanceType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_type(var_1);
            }
            ,
            s if s.matches("locationType") /* LocationType com.amazonaws.ec2#InstanceTypeOffering$LocationType */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::LocationType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::LocationType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_location_type(var_2);
            }
            ,
            s if s.matches("location") /* Location com.amazonaws.ec2#InstanceTypeOffering$Location */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_location(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
