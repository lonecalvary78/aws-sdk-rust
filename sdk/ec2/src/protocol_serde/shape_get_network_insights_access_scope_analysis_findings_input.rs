// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_network_insights_access_scope_analysis_findings_input_input_input(
    input: &crate::operation::get_network_insights_access_scope_analysis_findings::GetNetworkInsightsAccessScopeAnalysisFindingsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetNetworkInsightsAccessScopeAnalysisFindings", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("NetworkInsightsAccessScopeAnalysisId");
    if let Some(var_2) = &input.network_insights_access_scope_analysis_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("MaxResults");
    if let Some(var_4) = &input.max_results {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("NextToken");
    if let Some(var_6) = &input.next_token {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DryRun");
    if let Some(var_8) = &input.dry_run {
        scope_7.boolean(*var_8);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
