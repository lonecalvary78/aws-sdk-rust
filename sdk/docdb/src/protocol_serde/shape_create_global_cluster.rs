// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_global_cluster_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_global_cluster::CreateGlobalClusterOutput,
    crate::operation::create_global_cluster::CreateGlobalClusterError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_global_cluster::CreateGlobalClusterError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::create_global_cluster::CreateGlobalClusterError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DBClusterNotFoundFault" => crate::operation::create_global_cluster::CreateGlobalClusterError::DbClusterNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DbClusterNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_db_cluster_not_found_fault::de_db_cluster_not_found_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_global_cluster::CreateGlobalClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "GlobalClusterAlreadyExistsFault" => crate::operation::create_global_cluster::CreateGlobalClusterError::GlobalClusterAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::GlobalClusterAlreadyExistsFaultBuilder::default();
                output = crate::protocol_serde::shape_global_cluster_already_exists_fault::de_global_cluster_already_exists_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_global_cluster::CreateGlobalClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "GlobalClusterQuotaExceededFault" => crate::operation::create_global_cluster::CreateGlobalClusterError::GlobalClusterQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::GlobalClusterQuotaExceededFaultBuilder::default();
                output = crate::protocol_serde::shape_global_cluster_quota_exceeded_fault::de_global_cluster_quota_exceeded_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_global_cluster::CreateGlobalClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidDBClusterStateFault" => crate::operation::create_global_cluster::CreateGlobalClusterError::InvalidDbClusterStateFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidDbClusterStateFaultBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_db_cluster_state_fault::de_invalid_db_cluster_state_fault_xml_err(_response_body, output)
                        .map_err(crate::operation::create_global_cluster::CreateGlobalClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::create_global_cluster::CreateGlobalClusterError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_global_cluster_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_global_cluster::CreateGlobalClusterOutput,
    crate::operation::create_global_cluster::CreateGlobalClusterError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_global_cluster::builders::CreateGlobalClusterOutputBuilder::default();
        output = crate::protocol_serde::shape_create_global_cluster::de_create_global_cluster(_response_body, output)
            .map_err(crate::operation::create_global_cluster::CreateGlobalClusterError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_global_cluster(
    inp: &[u8],
    mut builder: crate::operation::create_global_cluster::builders::CreateGlobalClusterOutputBuilder,
) -> std::result::Result<crate::operation::create_global_cluster::builders::CreateGlobalClusterOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
{
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateGlobalClusterResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateGlobalClusterResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("CreateGlobalClusterResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected CreateGlobalClusterResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("GlobalCluster") /* GlobalCluster com.amazonaws.docdb.synthetic#CreateGlobalClusterOutput$GlobalCluster */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_global_cluster::de_global_cluster(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_global_cluster(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected CreateGlobalClusterResult tag"));
    };
    Ok(builder)
}
