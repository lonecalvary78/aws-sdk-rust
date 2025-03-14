// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_datapoint(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::Datapoint, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Datapoint::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Timestamp") /* Timestamp com.amazonaws.cloudwatch#Datapoint$Timestamp */ =>  {
                let var_1 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.cloudwatch#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_timestamp(var_1);
            }
            ,
            s if s.matches("SampleCount") /* SampleCount com.amazonaws.cloudwatch#Datapoint$SampleCount */ =>  {
                let var_2 =
                    Some(
                         {
                            <f64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.cloudwatch#DatapointValue`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_sample_count(var_2);
            }
            ,
            s if s.matches("Average") /* Average com.amazonaws.cloudwatch#Datapoint$Average */ =>  {
                let var_3 =
                    Some(
                         {
                            <f64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.cloudwatch#DatapointValue`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_average(var_3);
            }
            ,
            s if s.matches("Sum") /* Sum com.amazonaws.cloudwatch#Datapoint$Sum */ =>  {
                let var_4 =
                    Some(
                         {
                            <f64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.cloudwatch#DatapointValue`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_sum(var_4);
            }
            ,
            s if s.matches("Minimum") /* Minimum com.amazonaws.cloudwatch#Datapoint$Minimum */ =>  {
                let var_5 =
                    Some(
                         {
                            <f64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.cloudwatch#DatapointValue`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_minimum(var_5);
            }
            ,
            s if s.matches("Maximum") /* Maximum com.amazonaws.cloudwatch#Datapoint$Maximum */ =>  {
                let var_6 =
                    Some(
                         {
                            <f64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.cloudwatch#DatapointValue`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_maximum(var_6);
            }
            ,
            s if s.matches("Unit") /* Unit com.amazonaws.cloudwatch#Datapoint$Unit */ =>  {
                let var_7 =
                    Some(
                        Result::<crate::types::StandardUnit, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::StandardUnit::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_unit(var_7);
            }
            ,
            s if s.matches("ExtendedStatistics") /* ExtendedStatistics com.amazonaws.cloudwatch#Datapoint$ExtendedStatistics */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_datapoint_value_map::de_datapoint_value_map(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_extended_statistics(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
