// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_s3_action(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::S3Action,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TopicArn");
    if let Some(var_2) = &input.topic_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("BucketName");
    {
        scope_3.string(&input.bucket_name);
    }
    #[allow(unused_mut)]
    let mut scope_4 = writer.prefix("ObjectKeyPrefix");
    if let Some(var_5) = &input.object_key_prefix {
        scope_4.string(var_5);
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("KmsKeyArn");
    if let Some(var_7) = &input.kms_key_arn {
        scope_6.string(var_7);
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("IamRoleArn");
    if let Some(var_9) = &input.iam_role_arn {
        scope_8.string(var_9);
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_s3_action(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::S3Action, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::S3Action::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("TopicArn") /* TopicArn com.amazonaws.ses#S3Action$TopicArn */ =>  {
                let var_10 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_topic_arn(var_10);
            }
            ,
            s if s.matches("BucketName") /* BucketName com.amazonaws.ses#S3Action$BucketName */ =>  {
                let var_11 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_bucket_name(var_11);
            }
            ,
            s if s.matches("ObjectKeyPrefix") /* ObjectKeyPrefix com.amazonaws.ses#S3Action$ObjectKeyPrefix */ =>  {
                let var_12 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_object_key_prefix(var_12);
            }
            ,
            s if s.matches("KmsKeyArn") /* KmsKeyArn com.amazonaws.ses#S3Action$KmsKeyArn */ =>  {
                let var_13 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_kms_key_arn(var_13);
            }
            ,
            s if s.matches("IamRoleArn") /* IamRoleArn com.amazonaws.ses#S3Action$IamRoleArn */ =>  {
                let var_14 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_iam_role_arn(var_14);
            }
            ,
            _ => {}
        }
    }
    Ok(crate::serde_util::s3_action_correct_errors(builder)
        .build()
        .map_err(|_| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing field"))?)
}
