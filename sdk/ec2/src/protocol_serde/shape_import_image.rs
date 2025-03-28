// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_import_image_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::import_image::ImportImageOutput, crate::operation::import_image::ImportImageError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::import_image::ImportImageError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::import_image::ImportImageError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_import_image_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::import_image::ImportImageOutput, crate::operation::import_image::ImportImageError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::import_image::builders::ImportImageOutputBuilder::default();
        output = crate::protocol_serde::shape_import_image::de_import_image(_response_body, output)
            .map_err(crate::operation::import_image::ImportImageError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_import_image(
    inp: &[u8],
    mut builder: crate::operation::import_image::builders::ImportImageOutputBuilder,
) -> std::result::Result<crate::operation::import_image::builders::ImportImageOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ImportImageResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ImportImageResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("architecture") /* Architecture com.amazonaws.ec2.synthetic#ImportImageOutput$Architecture */ =>  {
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
            s if s.matches("description") /* Description com.amazonaws.ec2.synthetic#ImportImageOutput$Description */ =>  {
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
            s if s.matches("encrypted") /* Encrypted com.amazonaws.ec2.synthetic#ImportImageOutput$Encrypted */ =>  {
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
            s if s.matches("hypervisor") /* Hypervisor com.amazonaws.ec2.synthetic#ImportImageOutput$Hypervisor */ =>  {
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
            s if s.matches("imageId") /* ImageId com.amazonaws.ec2.synthetic#ImportImageOutput$ImageId */ =>  {
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
            s if s.matches("importTaskId") /* ImportTaskId com.amazonaws.ec2.synthetic#ImportImageOutput$ImportTaskId */ =>  {
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
            s if s.matches("kmsKeyId") /* KmsKeyId com.amazonaws.ec2.synthetic#ImportImageOutput$KmsKeyId */ =>  {
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
            s if s.matches("licenseType") /* LicenseType com.amazonaws.ec2.synthetic#ImportImageOutput$LicenseType */ =>  {
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
            s if s.matches("platform") /* Platform com.amazonaws.ec2.synthetic#ImportImageOutput$Platform */ =>  {
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
            s if s.matches("progress") /* Progress com.amazonaws.ec2.synthetic#ImportImageOutput$Progress */ =>  {
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
            s if s.matches("snapshotDetailSet") /* SnapshotDetails com.amazonaws.ec2.synthetic#ImportImageOutput$SnapshotDetails */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_snapshot_detail_list::de_snapshot_detail_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_snapshot_details(var_11);
            }
            ,
            s if s.matches("status") /* Status com.amazonaws.ec2.synthetic#ImportImageOutput$Status */ =>  {
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
            s if s.matches("statusMessage") /* StatusMessage com.amazonaws.ec2.synthetic#ImportImageOutput$StatusMessage */ =>  {
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
            s if s.matches("licenseSpecifications") /* LicenseSpecifications com.amazonaws.ec2.synthetic#ImportImageOutput$LicenseSpecifications */ =>  {
                let var_14 =
                    Some(
                        crate::protocol_serde::shape_import_image_license_specification_list_response::de_import_image_license_specification_list_response(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_license_specifications(var_14);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2.synthetic#ImportImageOutput$Tags */ =>  {
                let var_15 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_15);
            }
            ,
            s if s.matches("usageOperation") /* UsageOperation com.amazonaws.ec2.synthetic#ImportImageOutput$UsageOperation */ =>  {
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
            _ => {}
        }
    }
    Ok(builder)
}
