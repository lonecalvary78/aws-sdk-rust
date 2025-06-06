// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_volume_status_event(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::VolumeStatusEvent, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::VolumeStatusEvent::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("description") /* Description com.amazonaws.ec2#VolumeStatusEvent$Description */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_1);
            }
            ,
            s if s.matches("eventId") /* EventId com.amazonaws.ec2#VolumeStatusEvent$EventId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_event_id(var_2);
            }
            ,
            s if s.matches("eventType") /* EventType com.amazonaws.ec2#VolumeStatusEvent$EventType */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_event_type(var_3);
            }
            ,
            s if s.matches("notAfter") /* NotAfter com.amazonaws.ec2#VolumeStatusEvent$NotAfter */ =>  {
                let var_4 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_not_after(var_4);
            }
            ,
            s if s.matches("notBefore") /* NotBefore com.amazonaws.ec2#VolumeStatusEvent$NotBefore */ =>  {
                let var_5 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_not_before(var_5);
            }
            ,
            s if s.matches("instanceId") /* InstanceId com.amazonaws.ec2#VolumeStatusEvent$InstanceId */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_id(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
