// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_import_image_task(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::ImportImageTask, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ImportImageTask::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("architecture") /* Architecture com.amazonaws.ec2#ImportImageTask$Architecture */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_architecture(var_1);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#ImportImageTask$Description */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_2);
            }
            ,
            s if s.matches("encrypted") /* Encrypted com.amazonaws.ec2#ImportImageTask$Encrypted */ =>  {
                let var_3 =
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
                builder = builder.set_encrypted(var_3);
            }
            ,
            s if s.matches("hypervisor") /* Hypervisor com.amazonaws.ec2#ImportImageTask$Hypervisor */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_hypervisor(var_4);
            }
            ,
            s if s.matches("imageId") /* ImageId com.amazonaws.ec2#ImportImageTask$ImageId */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_image_id(var_5);
            }
            ,
            s if s.matches("importTaskId") /* ImportTaskId com.amazonaws.ec2#ImportImageTask$ImportTaskId */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_import_task_id(var_6);
            }
            ,
            s if s.matches("kmsKeyId") /* KmsKeyId com.amazonaws.ec2#ImportImageTask$KmsKeyId */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_kms_key_id(var_7);
            }
            ,
            s if s.matches("licenseType") /* LicenseType com.amazonaws.ec2#ImportImageTask$LicenseType */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_license_type(var_8);
            }
            ,
            s if s.matches("platform") /* Platform com.amazonaws.ec2#ImportImageTask$Platform */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_platform(var_9);
            }
            ,
            s if s.matches("progress") /* Progress com.amazonaws.ec2#ImportImageTask$Progress */ =>  {
                let var_10 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_progress(var_10);
            }
            ,
            s if s.matches("snapshotDetailSet") /* SnapshotDetails com.amazonaws.ec2#ImportImageTask$SnapshotDetails */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_snapshot_detail_list::de_snapshot_detail_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_snapshot_details(var_11);
            }
            ,
            s if s.matches("status") /* Status com.amazonaws.ec2#ImportImageTask$Status */ =>  {
                let var_12 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_12);
            }
            ,
            s if s.matches("statusMessage") /* StatusMessage com.amazonaws.ec2#ImportImageTask$StatusMessage */ =>  {
                let var_13 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status_message(var_13);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#ImportImageTask$Tags */ =>  {
                let var_14 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_14);
            }
            ,
            s if s.matches("licenseSpecifications") /* LicenseSpecifications com.amazonaws.ec2#ImportImageTask$LicenseSpecifications */ =>  {
                let var_15 =
                    Some(
                        crate::protocol_serde::shape_import_image_license_specification_list_response::de_import_image_license_specification_list_response(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_license_specifications(var_15);
            }
            ,
            s if s.matches("usageOperation") /* UsageOperation com.amazonaws.ec2#ImportImageTask$UsageOperation */ =>  {
                let var_16 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_usage_operation(var_16);
            }
            ,
            s if s.matches("bootMode") /* BootMode com.amazonaws.ec2#ImportImageTask$BootMode */ =>  {
                let var_17 =
                    Some(
                        Result::<crate::types::BootModeValues, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::BootModeValues::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_boot_mode(var_17);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
