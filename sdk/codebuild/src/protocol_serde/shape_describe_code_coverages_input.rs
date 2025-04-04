// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_code_coverages_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_code_coverages::DescribeCodeCoveragesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.report_arn {
        object.key("reportArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.next_token {
        object.key("nextToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.sort_order {
        object.key("sortOrder").string(var_4.as_str());
    }
    if let Some(var_5) = &input.sort_by {
        object.key("sortBy").string(var_5.as_str());
    }
    if let Some(var_6) = &input.min_line_coverage_percentage {
        object.key("minLineCoveragePercentage").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.max_line_coverage_percentage {
        object.key("maxLineCoveragePercentage").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_7).into()),
        );
    }
    Ok(())
}
