// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_db_proxy_target_group(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::DbProxyTargetGroup, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::DbProxyTargetGroup::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DBProxyName") /* DBProxyName com.amazonaws.rds#DBProxyTargetGroup$DBProxyName */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_proxy_name(var_1);
            }
            ,
            s if s.matches("TargetGroupName") /* TargetGroupName com.amazonaws.rds#DBProxyTargetGroup$TargetGroupName */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_target_group_name(var_2);
            }
            ,
            s if s.matches("TargetGroupArn") /* TargetGroupArn com.amazonaws.rds#DBProxyTargetGroup$TargetGroupArn */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_target_group_arn(var_3);
            }
            ,
            s if s.matches("IsDefault") /* IsDefault com.amazonaws.rds#DBProxyTargetGroup$IsDefault */ =>  {
                let var_4 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.rds#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_default(var_4);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.rds#DBProxyTargetGroup$Status */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_5);
            }
            ,
            s if s.matches("ConnectionPoolConfig") /* ConnectionPoolConfig com.amazonaws.rds#DBProxyTargetGroup$ConnectionPoolConfig */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_connection_pool_configuration_info::de_connection_pool_configuration_info(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_connection_pool_config(var_6);
            }
            ,
            s if s.matches("CreatedDate") /* CreatedDate com.amazonaws.rds#DBProxyTargetGroup$CreatedDate */ =>  {
                let var_7 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.rds#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_created_date(var_7);
            }
            ,
            s if s.matches("UpdatedDate") /* UpdatedDate com.amazonaws.rds#DBProxyTargetGroup$UpdatedDate */ =>  {
                let var_8 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.rds#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_updated_date(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
