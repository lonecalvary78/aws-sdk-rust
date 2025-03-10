// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_replication_group_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_replication_group::CreateReplicationGroupOutput,
    crate::operation::create_replication_group::CreateReplicationGroupError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CacheClusterNotFound" => crate::operation::create_replication_group::CreateReplicationGroupError::CacheClusterNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CacheClusterNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_cache_cluster_not_found_fault::de_cache_cluster_not_found_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "CacheParameterGroupNotFound" => crate::operation::create_replication_group::CreateReplicationGroupError::CacheParameterGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CacheParameterGroupNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_cache_parameter_group_not_found_fault::de_cache_parameter_group_not_found_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "CacheSecurityGroupNotFound" => crate::operation::create_replication_group::CreateReplicationGroupError::CacheSecurityGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CacheSecurityGroupNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_cache_security_group_not_found_fault::de_cache_security_group_not_found_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "CacheSubnetGroupNotFoundFault" => crate::operation::create_replication_group::CreateReplicationGroupError::CacheSubnetGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CacheSubnetGroupNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_cache_subnet_group_not_found_fault::de_cache_subnet_group_not_found_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ClusterQuotaForCustomerExceeded" => {
            crate::operation::create_replication_group::CreateReplicationGroupError::ClusterQuotaForCustomerExceededFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ClusterQuotaForCustomerExceededFaultBuilder::default();
                    output =
                        crate::protocol_serde::shape_cluster_quota_for_customer_exceeded_fault::de_cluster_quota_for_customer_exceeded_fault_xml_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "GlobalReplicationGroupNotFoundFault" => {
            crate::operation::create_replication_group::CreateReplicationGroupError::GlobalReplicationGroupNotFoundFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::GlobalReplicationGroupNotFoundFaultBuilder::default();
                    output =
                        crate::protocol_serde::shape_global_replication_group_not_found_fault::de_global_replication_group_not_found_fault_xml_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InsufficientCacheClusterCapacity" => {
            crate::operation::create_replication_group::CreateReplicationGroupError::InsufficientCacheClusterCapacityFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InsufficientCacheClusterCapacityFaultBuilder::default();
                    output =
                        crate::protocol_serde::shape_insufficient_cache_cluster_capacity_fault::de_insufficient_cache_cluster_capacity_fault_xml_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidCacheClusterState" => crate::operation::create_replication_group::CreateReplicationGroupError::InvalidCacheClusterStateFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidCacheClusterStateFaultBuilder::default();
                output = crate::protocol_serde::shape_invalid_cache_cluster_state_fault::de_invalid_cache_cluster_state_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidGlobalReplicationGroupState" => {
            crate::operation::create_replication_group::CreateReplicationGroupError::InvalidGlobalReplicationGroupStateFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidGlobalReplicationGroupStateFaultBuilder::default();
                    output = crate::protocol_serde::shape_invalid_global_replication_group_state_fault::de_invalid_global_replication_group_state_fault_xml_err(_response_body, output).map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterCombination" => {
            crate::operation::create_replication_group::CreateReplicationGroupError::InvalidParameterCombinationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterCombinationExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_invalid_parameter_combination_exception::de_invalid_parameter_combination_exception_xml_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterValue" => crate::operation::create_replication_group::CreateReplicationGroupError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidUserGroupState" => crate::operation::create_replication_group::CreateReplicationGroupError::InvalidUserGroupStateFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidUserGroupStateFaultBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_user_group_state_fault::de_invalid_user_group_state_fault_xml_err(_response_body, output)
                        .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidVPCNetworkStateFault" => crate::operation::create_replication_group::CreateReplicationGroupError::InvalidVpcNetworkStateFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidVpcNetworkStateFaultBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_vpc_network_state_fault::de_invalid_vpc_network_state_fault_xml_err(_response_body, output)
                        .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NodeGroupsPerReplicationGroupQuotaExceeded" => {
            crate::operation::create_replication_group::CreateReplicationGroupError::NodeGroupsPerReplicationGroupQuotaExceededFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NodeGroupsPerReplicationGroupQuotaExceededFaultBuilder::default();
                    output = crate::protocol_serde::shape_node_groups_per_replication_group_quota_exceeded_fault::de_node_groups_per_replication_group_quota_exceeded_fault_xml_err(_response_body, output).map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NodeQuotaForClusterExceeded" => crate::operation::create_replication_group::CreateReplicationGroupError::NodeQuotaForClusterExceededFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NodeQuotaForClusterExceededFaultBuilder::default();
                output = crate::protocol_serde::shape_node_quota_for_cluster_exceeded_fault::de_node_quota_for_cluster_exceeded_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NodeQuotaForCustomerExceeded" => {
            crate::operation::create_replication_group::CreateReplicationGroupError::NodeQuotaForCustomerExceededFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NodeQuotaForCustomerExceededFaultBuilder::default();
                    output = crate::protocol_serde::shape_node_quota_for_customer_exceeded_fault::de_node_quota_for_customer_exceeded_fault_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ReplicationGroupAlreadyExists" => {
            crate::operation::create_replication_group::CreateReplicationGroupError::ReplicationGroupAlreadyExistsFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ReplicationGroupAlreadyExistsFaultBuilder::default();
                    output = crate::protocol_serde::shape_replication_group_already_exists_fault::de_replication_group_already_exists_fault_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TagQuotaPerResourceExceeded" => crate::operation::create_replication_group::CreateReplicationGroupError::TagQuotaPerResourceExceeded({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TagQuotaPerResourceExceededBuilder::default();
                output =
                    crate::protocol_serde::shape_tag_quota_per_resource_exceeded::de_tag_quota_per_resource_exceeded_xml_err(_response_body, output)
                        .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UserGroupNotFound" => crate::operation::create_replication_group::CreateReplicationGroupError::UserGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UserGroupNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_user_group_not_found_fault::de_user_group_not_found_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::create_replication_group::CreateReplicationGroupError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_replication_group_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_replication_group::CreateReplicationGroupOutput,
    crate::operation::create_replication_group::CreateReplicationGroupError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_replication_group::builders::CreateReplicationGroupOutputBuilder::default();
        output = crate::protocol_serde::shape_create_replication_group::de_create_replication_group(_response_body, output)
            .map_err(crate::operation::create_replication_group::CreateReplicationGroupError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_replication_group(
    inp: &[u8],
    mut builder: crate::operation::create_replication_group::builders::CreateReplicationGroupOutputBuilder,
) -> std::result::Result<
    crate::operation::create_replication_group::builders::CreateReplicationGroupOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateReplicationGroupResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateReplicationGroupResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("CreateReplicationGroupResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected CreateReplicationGroupResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("ReplicationGroup") /* ReplicationGroup com.amazonaws.elasticache.synthetic#CreateReplicationGroupOutput$ReplicationGroup */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_replication_group::de_replication_group(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_replication_group(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected CreateReplicationGroupResult tag",
        ));
    };
    Ok(builder)
}
