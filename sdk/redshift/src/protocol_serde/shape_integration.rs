// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_integration(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::Integration, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Integration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("IntegrationArn") /* IntegrationArn com.amazonaws.redshift#Integration$IntegrationArn */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_integration_arn(var_1);
            }
            ,
            s if s.matches("IntegrationName") /* IntegrationName com.amazonaws.redshift#Integration$IntegrationName */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_integration_name(var_2);
            }
            ,
            s if s.matches("SourceArn") /* SourceArn com.amazonaws.redshift#Integration$SourceArn */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_source_arn(var_3);
            }
            ,
            s if s.matches("TargetArn") /* TargetArn com.amazonaws.redshift#Integration$TargetArn */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_target_arn(var_4);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.redshift#Integration$Status */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::ZeroEtlIntegrationStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ZeroEtlIntegrationStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_5);
            }
            ,
            s if s.matches("Errors") /* Errors com.amazonaws.redshift#Integration$Errors */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_integration_error_list::de_integration_error_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_errors(var_6);
            }
            ,
            s if s.matches("CreateTime") /* CreateTime com.amazonaws.redshift#Integration$CreateTime */ =>  {
                let var_7 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.redshift#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_create_time(var_7);
            }
            ,
            s if s.matches("Description") /* Description com.amazonaws.redshift#Integration$Description */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_8);
            }
            ,
            s if s.matches("KMSKeyId") /* KMSKeyId com.amazonaws.redshift#Integration$KMSKeyId */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_kms_key_id(var_9);
            }
            ,
            s if s.matches("AdditionalEncryptionContext") /* AdditionalEncryptionContext com.amazonaws.redshift#Integration$AdditionalEncryptionContext */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_encryption_context_map::de_encryption_context_map(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_additional_encryption_context(var_10);
            }
            ,
            s if s.matches("Tags") /* Tags com.amazonaws.redshift#Integration$Tags */ =>  {
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
