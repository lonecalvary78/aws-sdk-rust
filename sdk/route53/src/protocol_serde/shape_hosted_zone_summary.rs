// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_hosted_zone_summary(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::HostedZoneSummary, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::HostedZoneSummary::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("HostedZoneId") /* HostedZoneId com.amazonaws.route53#HostedZoneSummary$HostedZoneId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_hosted_zone_id(var_1);
            }
            ,
            s if s.matches("Name") /* Name com.amazonaws.route53#HostedZoneSummary$Name */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_2);
            }
            ,
            s if s.matches("Owner") /* Owner com.amazonaws.route53#HostedZoneSummary$Owner */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_hosted_zone_owner::de_hosted_zone_owner(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_owner(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::hosted_zone_summary_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}
