// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_inference_recommendations_job_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobOutput,
    crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFound" => crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobError::ResourceNotFound({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found::de_resource_not_found_json_err(_response_body, output)
                    .map_err(crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_inference_recommendations_job_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobOutput,
    crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::describe_inference_recommendations_job::builders::DescribeInferenceRecommendationsJobOutputBuilder::default();
        output =
            crate::protocol_serde::shape_describe_inference_recommendations_job::de_describe_inference_recommendations_job(_response_body, output)
                .map_err(crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::describe_inference_recommendations_job_output_output_correct_errors(output).build()
    })
}

pub fn ser_describe_inference_recommendations_job_input(
    input: &crate::operation::describe_inference_recommendations_job::DescribeInferenceRecommendationsJobInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_inference_recommendations_job_input::ser_describe_inference_recommendations_job_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_describe_inference_recommendations_job(
    value: &[u8],
    mut builder: crate::operation::describe_inference_recommendations_job::builders::DescribeInferenceRecommendationsJobOutputBuilder,
) -> ::std::result::Result<
    crate::operation::describe_inference_recommendations_job::builders::DescribeInferenceRecommendationsJobOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "JobName" => {
                    builder = builder.set_job_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "JobDescription" => {
                    builder = builder.set_job_description(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "JobType" => {
                    builder = builder.set_job_type(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::RecommendationJobType::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "JobArn" => {
                    builder = builder.set_job_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "RoleArn" => {
                    builder = builder.set_role_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "Status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::RecommendationJobStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "CreationTime" => {
                    builder = builder.set_creation_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "CompletionTime" => {
                    builder = builder.set_completion_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "LastModifiedTime" => {
                    builder = builder.set_last_modified_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "FailureReason" => {
                    builder = builder.set_failure_reason(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "InputConfig" => {
                    builder = builder
                        .set_input_config(crate::protocol_serde::shape_recommendation_job_input_config::de_recommendation_job_input_config(tokens)?);
                }
                "StoppingConditions" => {
                    builder = builder.set_stopping_conditions(
                        crate::protocol_serde::shape_recommendation_job_stopping_conditions::de_recommendation_job_stopping_conditions(tokens)?,
                    );
                }
                "InferenceRecommendations" => {
                    builder = builder.set_inference_recommendations(
                        crate::protocol_serde::shape_inference_recommendations::de_inference_recommendations(tokens)?,
                    );
                }
                "EndpointPerformances" => {
                    builder =
                        builder.set_endpoint_performances(crate::protocol_serde::shape_endpoint_performances::de_endpoint_performances(tokens)?);
                }
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
