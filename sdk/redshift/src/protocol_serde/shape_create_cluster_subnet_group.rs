// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_cluster_subnet_group_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupOutput,
    crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ClusterSubnetGroupAlreadyExists" => {
            crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::ClusterSubnetGroupAlreadyExistsFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ClusterSubnetGroupAlreadyExistsFaultBuilder::default();
                    output =
                        crate::protocol_serde::shape_cluster_subnet_group_already_exists_fault::de_cluster_subnet_group_already_exists_fault_xml_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ClusterSubnetGroupQuotaExceeded" => {
            crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::ClusterSubnetGroupQuotaExceededFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ClusterSubnetGroupQuotaExceededFaultBuilder::default();
                    output =
                        crate::protocol_serde::shape_cluster_subnet_group_quota_exceeded_fault::de_cluster_subnet_group_quota_exceeded_fault_xml_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ClusterSubnetQuotaExceededFault" => {
            crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::ClusterSubnetQuotaExceededFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ClusterSubnetQuotaExceededFaultBuilder::default();
                    output = crate::protocol_serde::shape_cluster_subnet_quota_exceeded_fault::de_cluster_subnet_quota_exceeded_fault_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "DependentServiceRequestThrottlingFault" => {
            crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::DependentServiceRequestThrottlingFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DependentServiceRequestThrottlingFaultBuilder::default();
                    output = crate::protocol_serde::shape_dependent_service_request_throttling_fault::de_dependent_service_request_throttling_fault_xml_err(_response_body, output).map_err(crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidSubnet" => crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::InvalidSubnet({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidSubnetBuilder::default();
                output = crate::protocol_serde::shape_invalid_subnet::de_invalid_subnet_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidTagFault" => crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::InvalidTagFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidTagFaultBuilder::default();
                output = crate::protocol_serde::shape_invalid_tag_fault::de_invalid_tag_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TagLimitExceededFault" => crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::TagLimitExceededFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TagLimitExceededFaultBuilder::default();
                output = crate::protocol_serde::shape_tag_limit_exceeded_fault::de_tag_limit_exceeded_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnauthorizedOperation" => crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::UnauthorizedOperation({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnauthorizedOperationBuilder::default();
                output = crate::protocol_serde::shape_unauthorized_operation::de_unauthorized_operation_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_cluster_subnet_group_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupOutput,
    crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_cluster_subnet_group::builders::CreateClusterSubnetGroupOutputBuilder::default();
        output = crate::protocol_serde::shape_create_cluster_subnet_group::de_create_cluster_subnet_group(_response_body, output)
            .map_err(crate::operation::create_cluster_subnet_group::CreateClusterSubnetGroupError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_cluster_subnet_group(
    inp: &[u8],
    mut builder: crate::operation::create_cluster_subnet_group::builders::CreateClusterSubnetGroupOutputBuilder,
) -> std::result::Result<
    crate::operation::create_cluster_subnet_group::builders::CreateClusterSubnetGroupOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateClusterSubnetGroupResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateClusterSubnetGroupResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("CreateClusterSubnetGroupResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected CreateClusterSubnetGroupResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("ClusterSubnetGroup") /* ClusterSubnetGroup com.amazonaws.redshift.synthetic#CreateClusterSubnetGroupOutput$ClusterSubnetGroup */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_cluster_subnet_group::de_cluster_subnet_group(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cluster_subnet_group(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected CreateClusterSubnetGroupResult tag",
        ));
    };
    Ok(builder)
}
