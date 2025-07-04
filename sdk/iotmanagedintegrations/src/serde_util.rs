// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn create_account_association_output_output_correct_errors(
    mut builder: crate::operation::create_account_association::builders::CreateAccountAssociationOutputBuilder,
) -> crate::operation::create_account_association::builders::CreateAccountAssociationOutputBuilder {
    if builder.o_auth_authorization_url.is_none() {
        builder.o_auth_authorization_url = Some(Default::default())
    }
    if builder.account_association_id.is_none() {
        builder.account_association_id = Some(Default::default())
    }
    if builder.association_state.is_none() {
        builder.association_state = "no value was set".parse::<crate::types::AssociationState>().ok()
    }
    builder
}

pub(crate) fn get_account_association_output_output_correct_errors(
    mut builder: crate::operation::get_account_association::builders::GetAccountAssociationOutputBuilder,
) -> crate::operation::get_account_association::builders::GetAccountAssociationOutputBuilder {
    if builder.account_association_id.is_none() {
        builder.account_association_id = Some(Default::default())
    }
    if builder.association_state.is_none() {
        builder.association_state = "no value was set".parse::<crate::types::AssociationState>().ok()
    }
    if builder.o_auth_authorization_url.is_none() {
        builder.o_auth_authorization_url = Some(Default::default())
    }
    builder
}

pub(crate) fn get_cloud_connector_output_output_correct_errors(
    mut builder: crate::operation::get_cloud_connector::builders::GetCloudConnectorOutputBuilder,
) -> crate::operation::get_cloud_connector::builders::GetCloudConnectorOutputBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.endpoint_config.is_none() {
        builder.endpoint_config = {
            let builder = crate::types::builders::EndpointConfigBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn get_custom_endpoint_output_output_correct_errors(
    mut builder: crate::operation::get_custom_endpoint::builders::GetCustomEndpointOutputBuilder,
) -> crate::operation::get_custom_endpoint::builders::GetCustomEndpointOutputBuilder {
    if builder.endpoint_address.is_none() {
        builder.endpoint_address = Some(Default::default())
    }
    builder
}

pub(crate) fn get_default_encryption_configuration_output_output_correct_errors(
    mut builder: crate::operation::get_default_encryption_configuration::builders::GetDefaultEncryptionConfigurationOutputBuilder,
) -> crate::operation::get_default_encryption_configuration::builders::GetDefaultEncryptionConfigurationOutputBuilder {
    if builder.configuration_status.is_none() {
        builder.configuration_status = {
            let builder = crate::types::builders::ConfigurationStatusBuilder::default();
            crate::serde_util::configuration_status_correct_errors(builder).build().ok()
        }
    }
    if builder.encryption_type.is_none() {
        builder.encryption_type = "no value was set".parse::<crate::types::EncryptionType>().ok()
    }
    builder
}

pub(crate) fn get_device_discovery_output_output_correct_errors(
    mut builder: crate::operation::get_device_discovery::builders::GetDeviceDiscoveryOutputBuilder,
) -> crate::operation::get_device_discovery::builders::GetDeviceDiscoveryOutputBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.discovery_type.is_none() {
        builder.discovery_type = "no value was set".parse::<crate::types::DiscoveryType>().ok()
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::DeviceDiscoveryStatus>().ok()
    }
    if builder.started_at.is_none() {
        builder.started_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn get_managed_thing_state_output_output_correct_errors(
    mut builder: crate::operation::get_managed_thing_state::builders::GetManagedThingStateOutputBuilder,
) -> crate::operation::get_managed_thing_state::builders::GetManagedThingStateOutputBuilder {
    if builder.endpoints.is_none() {
        builder.endpoints = Some(Default::default())
    }
    builder
}

pub(crate) fn put_default_encryption_configuration_output_output_correct_errors(
    mut builder: crate::operation::put_default_encryption_configuration::builders::PutDefaultEncryptionConfigurationOutputBuilder,
) -> crate::operation::put_default_encryption_configuration::builders::PutDefaultEncryptionConfigurationOutputBuilder {
    if builder.configuration_status.is_none() {
        builder.configuration_status = {
            let builder = crate::types::builders::ConfigurationStatusBuilder::default();
            crate::serde_util::configuration_status_correct_errors(builder).build().ok()
        }
    }
    if builder.encryption_type.is_none() {
        builder.encryption_type = "no value was set".parse::<crate::types::EncryptionType>().ok()
    }
    builder
}

pub(crate) fn register_custom_endpoint_output_output_correct_errors(
    mut builder: crate::operation::register_custom_endpoint::builders::RegisterCustomEndpointOutputBuilder,
) -> crate::operation::register_custom_endpoint::builders::RegisterCustomEndpointOutputBuilder {
    if builder.endpoint_address.is_none() {
        builder.endpoint_address = Some(Default::default())
    }
    builder
}

pub(crate) fn send_connector_event_output_output_correct_errors(
    mut builder: crate::operation::send_connector_event::builders::SendConnectorEventOutputBuilder,
) -> crate::operation::send_connector_event::builders::SendConnectorEventOutputBuilder {
    if builder.connector_id.is_none() {
        builder.connector_id = Some(Default::default())
    }
    builder
}

pub(crate) fn start_account_association_refresh_output_output_correct_errors(
    mut builder: crate::operation::start_account_association_refresh::builders::StartAccountAssociationRefreshOutputBuilder,
) -> crate::operation::start_account_association_refresh::builders::StartAccountAssociationRefreshOutputBuilder {
    if builder.o_auth_authorization_url.is_none() {
        builder.o_auth_authorization_url = Some(Default::default())
    }
    builder
}

pub(crate) fn configuration_status_correct_errors(
    mut builder: crate::types::builders::ConfigurationStatusBuilder,
) -> crate::types::builders::ConfigurationStatusBuilder {
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::ConfigurationState>().ok()
    }
    builder
}

pub(crate) fn capability_report_correct_errors(
    mut builder: crate::types::builders::CapabilityReportBuilder,
) -> crate::types::builders::CapabilityReportBuilder {
    if builder.version.is_none() {
        builder.version = Some(Default::default())
    }
    if builder.endpoints.is_none() {
        builder.endpoints = Some(Default::default())
    }
    builder
}

pub(crate) fn secrets_manager_correct_errors(
    mut builder: crate::types::builders::SecretsManagerBuilder,
) -> crate::types::builders::SecretsManagerBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.version_id.is_none() {
        builder.version_id = Some(Default::default())
    }
    builder
}

pub(crate) fn account_association_item_correct_errors(
    mut builder: crate::types::builders::AccountAssociationItemBuilder,
) -> crate::types::builders::AccountAssociationItemBuilder {
    if builder.account_association_id.is_none() {
        builder.account_association_id = Some(Default::default())
    }
    if builder.association_state.is_none() {
        builder.association_state = "no value was set".parse::<crate::types::AssociationState>().ok()
    }
    builder
}

pub(crate) fn connector_item_correct_errors(
    mut builder: crate::types::builders::ConnectorItemBuilder,
) -> crate::types::builders::ConnectorItemBuilder {
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.endpoint_config.is_none() {
        builder.endpoint_config = {
            let builder = crate::types::builders::EndpointConfigBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn lambda_config_correct_errors(mut builder: crate::types::builders::LambdaConfigBuilder) -> crate::types::builders::LambdaConfigBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    builder
}

pub(crate) fn o_auth_config_correct_errors(mut builder: crate::types::builders::OAuthConfigBuilder) -> crate::types::builders::OAuthConfigBuilder {
    if builder.auth_url.is_none() {
        builder.auth_url = Some(Default::default())
    }
    if builder.token_url.is_none() {
        builder.token_url = Some(Default::default())
    }
    if builder.token_endpoint_authentication_scheme.is_none() {
        builder.token_endpoint_authentication_scheme = "no value was set".parse::<crate::types::TokenEndpointAuthenticationScheme>().ok()
    }
    builder
}

pub(crate) fn state_endpoint_correct_errors(
    mut builder: crate::types::builders::StateEndpointBuilder,
) -> crate::types::builders::StateEndpointBuilder {
    if builder.endpoint_id.is_none() {
        builder.endpoint_id = Some(Default::default())
    }
    if builder.capabilities.is_none() {
        builder.capabilities = Some(Default::default())
    }
    builder
}

pub(crate) fn capability_report_endpoint_correct_errors(
    mut builder: crate::types::builders::CapabilityReportEndpointBuilder,
) -> crate::types::builders::CapabilityReportEndpointBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.device_types.is_none() {
        builder.device_types = Some(Default::default())
    }
    if builder.capabilities.is_none() {
        builder.capabilities = Some(Default::default())
    }
    builder
}

pub(crate) fn state_capability_correct_errors(
    mut builder: crate::types::builders::StateCapabilityBuilder,
) -> crate::types::builders::StateCapabilityBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.version.is_none() {
        builder.version = Some(Default::default())
    }
    builder
}

pub(crate) fn capability_report_capability_correct_errors(
    mut builder: crate::types::builders::CapabilityReportCapabilityBuilder,
) -> crate::types::builders::CapabilityReportCapabilityBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    if builder.version.is_none() {
        builder.version = Some(Default::default())
    }
    if builder.properties.is_none() {
        builder.properties = Some(Default::default())
    }
    if builder.actions.is_none() {
        builder.actions = Some(Default::default())
    }
    if builder.events.is_none() {
        builder.events = Some(Default::default())
    }
    builder
}
