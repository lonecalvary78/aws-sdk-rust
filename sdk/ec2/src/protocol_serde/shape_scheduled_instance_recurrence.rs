// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_scheduled_instance_recurrence(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::ScheduledInstanceRecurrence, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ScheduledInstanceRecurrence::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("frequency") /* Frequency com.amazonaws.ec2#ScheduledInstanceRecurrence$Frequency */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_frequency(var_1);
            }
            ,
            s if s.matches("interval") /* Interval com.amazonaws.ec2#ScheduledInstanceRecurrence$Interval */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_interval(var_2);
            }
            ,
            s if s.matches("occurrenceDaySet") /* OccurrenceDaySet com.amazonaws.ec2#ScheduledInstanceRecurrence$OccurrenceDaySet */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_occurrence_day_set::de_occurrence_day_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_occurrence_day_set(var_3);
            }
            ,
            s if s.matches("occurrenceRelativeToEnd") /* OccurrenceRelativeToEnd com.amazonaws.ec2#ScheduledInstanceRecurrence$OccurrenceRelativeToEnd */ =>  {
                let var_4 =
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
                builder = builder.set_occurrence_relative_to_end(var_4);
            }
            ,
            s if s.matches("occurrenceUnit") /* OccurrenceUnit com.amazonaws.ec2#ScheduledInstanceRecurrence$OccurrenceUnit */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_occurrence_unit(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
