// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_snapshot_schedule_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleOutput,
    crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidSchedule" => crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleError::InvalidScheduleFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidScheduleFaultBuilder::default();
                output = crate::protocol_serde::shape_invalid_schedule_fault::de_invalid_schedule_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "SnapshotScheduleNotFound" => crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleError::SnapshotScheduleNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::SnapshotScheduleNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_snapshot_schedule_not_found_fault::de_snapshot_schedule_not_found_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "SnapshotScheduleUpdateInProgress" => {
            crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleError::SnapshotScheduleUpdateInProgressFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::SnapshotScheduleUpdateInProgressFaultBuilder::default();
                    output = crate::protocol_serde::shape_snapshot_schedule_update_in_progress_fault::de_snapshot_schedule_update_in_progress_fault_xml_err(_response_body, output).map_err(crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_snapshot_schedule_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleOutput,
    crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_snapshot_schedule::builders::ModifySnapshotScheduleOutputBuilder::default();
        output = crate::protocol_serde::shape_modify_snapshot_schedule::de_modify_snapshot_schedule(_response_body, output)
            .map_err(crate::operation::modify_snapshot_schedule::ModifySnapshotScheduleError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_snapshot_schedule(
    inp: &[u8],
    mut builder: crate::operation::modify_snapshot_schedule::builders::ModifySnapshotScheduleOutputBuilder,
) -> std::result::Result<
    crate::operation::modify_snapshot_schedule::builders::ModifySnapshotScheduleOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifySnapshotScheduleResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifySnapshotScheduleResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ModifySnapshotScheduleResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ModifySnapshotScheduleResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("ScheduleDefinitions") /* ScheduleDefinitions com.amazonaws.redshift.synthetic#ModifySnapshotScheduleOutput$ScheduleDefinitions */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_schedule_definition_list::de_schedule_definition_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_schedule_definitions(var_1);
            }
            ,
            s if s.matches("ScheduleIdentifier") /* ScheduleIdentifier com.amazonaws.redshift.synthetic#ModifySnapshotScheduleOutput$ScheduleIdentifier */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_schedule_identifier(var_2);
            }
            ,
            s if s.matches("ScheduleDescription") /* ScheduleDescription com.amazonaws.redshift.synthetic#ModifySnapshotScheduleOutput$ScheduleDescription */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_schedule_description(var_3);
            }
            ,
            s if s.matches("Tags") /* Tags com.amazonaws.redshift.synthetic#ModifySnapshotScheduleOutput$Tags */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_4);
            }
            ,
            s if s.matches("NextInvocations") /* NextInvocations com.amazonaws.redshift.synthetic#ModifySnapshotScheduleOutput$NextInvocations */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_scheduled_snapshot_time_list::de_scheduled_snapshot_time_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_next_invocations(var_5);
            }
            ,
            s if s.matches("AssociatedClusterCount") /* AssociatedClusterCount com.amazonaws.redshift.synthetic#ModifySnapshotScheduleOutput$AssociatedClusterCount */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.redshift#IntegerOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_associated_cluster_count(var_6);
            }
            ,
            s if s.matches("AssociatedClusters") /* AssociatedClusters com.amazonaws.redshift.synthetic#ModifySnapshotScheduleOutput$AssociatedClusters */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_associated_cluster_list::de_associated_cluster_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_associated_clusters(var_7);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected ModifySnapshotScheduleResult tag",
        ));
    };
    Ok(builder)
}
