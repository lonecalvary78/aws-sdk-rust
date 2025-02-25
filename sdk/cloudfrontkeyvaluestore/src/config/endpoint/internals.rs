// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(
    clippy::collapsible_if,
    clippy::bool_comparison,
    clippy::nonminimal_bool,
    clippy::comparison_to_empty,
    clippy::redundant_pattern_matching,
    clippy::useless_asref
)]
pub(super) fn resolve_endpoint(
    _params: &crate::config::endpoint::Params,
    _diagnostic_collector: &mut crate::endpoint_lib::diagnostic::DiagnosticCollector,
    partition_resolver: &crate::endpoint_lib::partition::PartitionResolver,
) -> ::aws_smithy_http::endpoint::Result {
    #[allow(unused_variables)]
    let kvs_arn = &_params.kvs_arn;
    #[allow(unused_variables)]
    let region = &_params.region;
    #[allow(unused_variables)]
    let use_fips = &_params.use_fips;
    #[allow(unused_variables)]
    let endpoint = &_params.endpoint;
    if (*use_fips) == (false) {
        #[allow(unused_variables)]
        if let Some(kvs_arn) = kvs_arn {
            #[allow(unused_variables)]
            if let Some(parsed_arn) = crate::endpoint_lib::arn::parse_arn(kvs_arn.as_ref() as &str, _diagnostic_collector) {
                if (parsed_arn.service()) == ("cloudfront") {
                    if (parsed_arn.region()) == ("") {
                        #[allow(unused_variables)]
                        if let Some(arn_type) = parsed_arn.resource_id().first().cloned() {
                            if !((arn_type.as_ref() as &str) == ("")) {
                                if (arn_type.as_ref() as &str) == ("key-value-store") {
                                    if (parsed_arn.partition()) == ("aws") {
                                        #[allow(unused_variables)]
                                        if let Some(region) = region {
                                            #[allow(unused_variables)]
                                            if let Some(partition_result) =
                                                partition_resolver.resolve_partition(region.as_ref() as &str, _diagnostic_collector)
                                            {
                                                if (partition_result.name()) == (parsed_arn.partition()) {
                                                    #[allow(unused_variables)]
                                                    if let Some(endpoint) = endpoint {
                                                        #[allow(unused_variables)]
                                                        if let Some(url) = crate::endpoint_lib::parse_url::parse_url(
                                                            endpoint.as_ref() as &str,
                                                            _diagnostic_collector,
                                                        ) {
                                                            return Ok(::aws_smithy_types::endpoint::Endpoint::builder()
                                                                .url({
                                                                    let mut out = String::new();
                                                                    #[allow(clippy::needless_borrow)]
                                                                    out.push_str(&url.scheme());
                                                                    out.push_str("://");
                                                                    #[allow(clippy::needless_borrow)]
                                                                    out.push_str(&parsed_arn.account_id());
                                                                    out.push('.');
                                                                    #[allow(clippy::needless_borrow)]
                                                                    out.push_str(&url.authority());
                                                                    #[allow(clippy::needless_borrow)]
                                                                    out.push_str(&url.path());
                                                                    out
                                                                })
                                                                .property(
                                                                    "authSchemes",
                                                                    vec![::aws_smithy_types::Document::from({
                                                                        let mut out =
                                                                            ::std::collections::HashMap::<String, ::aws_smithy_types::Document>::new(
                                                                            );
                                                                        out.insert("name".to_string(), "sigv4a".to_string().into());
                                                                        out.insert(
                                                                            "signingName".to_string(),
                                                                            "cloudfront-keyvaluestore".to_string().into(),
                                                                        );
                                                                        out.insert(
                                                                            "signingRegionSet".to_string(),
                                                                            vec![::aws_smithy_types::Document::from("*".to_string())].into(),
                                                                        );
                                                                        out
                                                                    })],
                                                                )
                                                                .build());
                                                        }
                                                        return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message(
                                                            "Provided endpoint is not a valid URL".to_string(),
                                                        ));
                                                    }
                                                    return Ok(::aws_smithy_types::endpoint::Endpoint::builder()
                                                        .url({
                                                            let mut out = String::new();
                                                            out.push_str("https://");
                                                            #[allow(clippy::needless_borrow)]
                                                            out.push_str(&parsed_arn.account_id());
                                                            out.push_str(".cloudfront-kvs.global.api.aws");
                                                            out
                                                        })
                                                        .property(
                                                            "authSchemes",
                                                            vec![::aws_smithy_types::Document::from({
                                                                let mut out =
                                                                    ::std::collections::HashMap::<String, ::aws_smithy_types::Document>::new();
                                                                out.insert("name".to_string(), "sigv4a".to_string().into());
                                                                out.insert("signingName".to_string(), "cloudfront-keyvaluestore".to_string().into());
                                                                out.insert(
                                                                    "signingRegionSet".to_string(),
                                                                    vec![::aws_smithy_types::Document::from("*".to_string())].into(),
                                                                );
                                                                out
                                                            })],
                                                        )
                                                        .build());
                                                }
                                                return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message({
                                                    let mut out = String::new();
                                                    out.push_str("Client was configured for partition `");
                                                    #[allow(clippy::needless_borrow)]
                                                    out.push_str(&partition_result.name());
                                                    out.push_str("` but Kvs ARN has `");
                                                    #[allow(clippy::needless_borrow)]
                                                    out.push_str(&parsed_arn.partition());
                                                    out.push('`');
                                                    out
                                                }));
                                            }
                                            #[allow(unreachable_code)]
                                            return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message(format!(
                                                "No rules matched these parameters. This is a bug. {:?}",
                                                _params
                                            )));
                                        }
                                        #[allow(unused_variables)]
                                        if let Some(endpoint) = endpoint {
                                            #[allow(unused_variables)]
                                            if let Some(url) =
                                                crate::endpoint_lib::parse_url::parse_url(endpoint.as_ref() as &str, _diagnostic_collector)
                                            {
                                                return Ok(::aws_smithy_types::endpoint::Endpoint::builder()
                                                    .url({
                                                        let mut out = String::new();
                                                        #[allow(clippy::needless_borrow)]
                                                        out.push_str(&url.scheme());
                                                        out.push_str("://");
                                                        #[allow(clippy::needless_borrow)]
                                                        out.push_str(&parsed_arn.account_id());
                                                        out.push('.');
                                                        #[allow(clippy::needless_borrow)]
                                                        out.push_str(&url.authority());
                                                        #[allow(clippy::needless_borrow)]
                                                        out.push_str(&url.path());
                                                        out
                                                    })
                                                    .property(
                                                        "authSchemes",
                                                        vec![::aws_smithy_types::Document::from({
                                                            let mut out = ::std::collections::HashMap::<String, ::aws_smithy_types::Document>::new();
                                                            out.insert("name".to_string(), "sigv4a".to_string().into());
                                                            out.insert("signingName".to_string(), "cloudfront-keyvaluestore".to_string().into());
                                                            out.insert(
                                                                "signingRegionSet".to_string(),
                                                                vec![::aws_smithy_types::Document::from("*".to_string())].into(),
                                                            );
                                                            out
                                                        })],
                                                    )
                                                    .build());
                                            }
                                            return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message(
                                                "Provided endpoint is not a valid URL".to_string(),
                                            ));
                                        }
                                        return Ok(::aws_smithy_types::endpoint::Endpoint::builder()
                                            .url({
                                                let mut out = String::new();
                                                out.push_str("https://");
                                                #[allow(clippy::needless_borrow)]
                                                out.push_str(&parsed_arn.account_id());
                                                out.push_str(".cloudfront-kvs.global.api.aws");
                                                out
                                            })
                                            .property(
                                                "authSchemes",
                                                vec![::aws_smithy_types::Document::from({
                                                    let mut out = ::std::collections::HashMap::<String, ::aws_smithy_types::Document>::new();
                                                    out.insert("name".to_string(), "sigv4a".to_string().into());
                                                    out.insert("signingName".to_string(), "cloudfront-keyvaluestore".to_string().into());
                                                    out.insert(
                                                        "signingRegionSet".to_string(),
                                                        vec![::aws_smithy_types::Document::from("*".to_string())].into(),
                                                    );
                                                    out
                                                })],
                                            )
                                            .build());
                                    }
                                    return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message({
                                        let mut out = String::new();
                                        out.push_str("CloudFront-KeyValueStore is not supported in partition `");
                                        #[allow(clippy::needless_borrow)]
                                        out.push_str(&parsed_arn.partition());
                                        out.push('`');
                                        out
                                    }));
                                }
                                return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message({
                                    let mut out = String::new();
                                    out.push_str("ARN resource type is invalid. Expected `key-value-store`, found: `");
                                    #[allow(clippy::needless_borrow)]
                                    out.push_str(&arn_type.as_ref() as &str);
                                    out.push('`');
                                    out
                                }));
                            }
                            return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message(
                                "No resource type found in the KVS ARN. Resource type must be `key-value-store`.".to_string(),
                            ));
                        }
                        return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message(
                            "No resource type found in the KVS ARN. Resource type must be `key-value-store`.".to_string(),
                        ));
                    }
                    return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message({
                        let mut out = String::new();
                        out.push_str("Provided ARN must be a global resource ARN. Found: `");
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&parsed_arn.region());
                        out.push('`');
                        out
                    }));
                }
                return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message({
                    let mut out = String::new();
                    out.push_str("Provided ARN is not a valid CloudFront Service ARN. Found: `");
                    #[allow(clippy::needless_borrow)]
                    out.push_str(&parsed_arn.service());
                    out.push('`');
                    out
                }));
            }
            return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message(
                "KVS ARN must be a valid ARN".to_string(),
            ));
        }
        return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message(
            "KVS ARN must be provided to use this service".to_string(),
        ));
    }
    return Err(::aws_smithy_http::endpoint::ResolveEndpointError::message(
        "Invalid Configuration: FIPS is not supported with CloudFront-KeyValueStore.".to_string(),
    ));
}
