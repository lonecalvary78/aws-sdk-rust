// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_oauth2_provider_config_input(
    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Oauth2ProviderConfigInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::Oauth2ProviderConfigInput::CustomOauth2ProviderConfig(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_4.key("customOauth2ProviderConfig").start_object();
            crate::protocol_serde::shape_custom_oauth2_provider_config_input::ser_custom_oauth2_provider_config_input(&mut object_1, inner)?;
            object_1.finish();
        }
        crate::types::Oauth2ProviderConfigInput::GoogleOauth2ProviderConfig(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_4.key("googleOauth2ProviderConfig").start_object();
            crate::protocol_serde::shape_google_oauth2_provider_config_input::ser_google_oauth2_provider_config_input(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::Oauth2ProviderConfigInput::GithubOauth2ProviderConfig(inner) => {
            #[allow(unused_mut)]
            let mut object_3 = object_4.key("githubOauth2ProviderConfig").start_object();
            crate::protocol_serde::shape_github_oauth2_provider_config_input::ser_github_oauth2_provider_config_input(&mut object_3, inner)?;
            object_3.finish();
        }
        crate::types::Oauth2ProviderConfigInput::SlackOauth2ProviderConfig(inner) => {
            #[allow(unused_mut)]
            let mut object_4 = object_4.key("slackOauth2ProviderConfig").start_object();
            crate::protocol_serde::shape_slack_oauth2_provider_config_input::ser_slack_oauth2_provider_config_input(&mut object_4, inner)?;
            object_4.finish();
        }
        crate::types::Oauth2ProviderConfigInput::SalesforceOauth2ProviderConfig(inner) => {
            #[allow(unused_mut)]
            let mut object_5 = object_4.key("salesforceOauth2ProviderConfig").start_object();
            crate::protocol_serde::shape_salesforce_oauth2_provider_config_input::ser_salesforce_oauth2_provider_config_input(&mut object_5, inner)?;
            object_5.finish();
        }
        crate::types::Oauth2ProviderConfigInput::MicrosoftOauth2ProviderConfig(inner) => {
            #[allow(unused_mut)]
            let mut object_6 = object_4.key("microsoftOauth2ProviderConfig").start_object();
            crate::protocol_serde::shape_microsoft_oauth2_provider_config_input::ser_microsoft_oauth2_provider_config_input(&mut object_6, inner)?;
            object_6.finish();
        }
        crate::types::Oauth2ProviderConfigInput::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "Oauth2ProviderConfigInput",
            ))
        }
    }
    Ok(())
}
