// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_branch_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_branch::UpdateBranchInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.backend {
        #[allow(unused_mut)]
        let mut object_2 = object.key("backend").start_object();
        crate::protocol_serde::shape_backend::ser_backend(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.backend_environment_arn {
        object.key("backendEnvironmentArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.basic_auth_credentials {
        object.key("basicAuthCredentials").string(var_4.as_str());
    }
    if let Some(var_5) = &input.build_spec {
        object.key("buildSpec").string(var_5.as_str());
    }
    if let Some(var_6) = &input.compute_role_arn {
        object.key("computeRoleArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.description {
        object.key("description").string(var_7.as_str());
    }
    if let Some(var_8) = &input.display_name {
        object.key("displayName").string(var_8.as_str());
    }
    if let Some(var_9) = &input.enable_auto_build {
        object.key("enableAutoBuild").boolean(*var_9);
    }
    if let Some(var_10) = &input.enable_basic_auth {
        object.key("enableBasicAuth").boolean(*var_10);
    }
    if let Some(var_11) = &input.enable_notification {
        object.key("enableNotification").boolean(*var_11);
    }
    if let Some(var_12) = &input.enable_performance_mode {
        object.key("enablePerformanceMode").boolean(*var_12);
    }
    if let Some(var_13) = &input.enable_pull_request_preview {
        object.key("enablePullRequestPreview").boolean(*var_13);
    }
    if let Some(var_14) = &input.enable_skew_protection {
        object.key("enableSkewProtection").boolean(*var_14);
    }
    if let Some(var_15) = &input.environment_variables {
        #[allow(unused_mut)]
        let mut object_16 = object.key("environmentVariables").start_object();
        for (key_17, value_18) in var_15 {
            {
                object_16.key(key_17.as_str()).string(value_18.as_str());
            }
        }
        object_16.finish();
    }
    if let Some(var_19) = &input.framework {
        object.key("framework").string(var_19.as_str());
    }
    if let Some(var_20) = &input.pull_request_environment_name {
        object.key("pullRequestEnvironmentName").string(var_20.as_str());
    }
    if let Some(var_21) = &input.stage {
        object.key("stage").string(var_21.as_str());
    }
    if let Some(var_22) = &input.ttl {
        object.key("ttl").string(var_22.as_str());
    }
    Ok(())
}
