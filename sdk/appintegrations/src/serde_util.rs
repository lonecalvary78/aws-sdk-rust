// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn event_filter_correct_errors(mut builder: crate::types::builders::EventFilterBuilder) -> crate::types::builders::EventFilterBuilder {
    if builder.source.is_none() {
        builder.source = Some(Default::default())
    }
    builder
}

pub(crate) fn file_configuration_correct_errors(
    mut builder: crate::types::builders::FileConfigurationBuilder,
) -> crate::types::builders::FileConfigurationBuilder {
    if builder.folders.is_none() {
        builder.folders = Some(Default::default())
    }
    builder
}

pub(crate) fn schedule_configuration_correct_errors(
    mut builder: crate::types::builders::ScheduleConfigurationBuilder,
) -> crate::types::builders::ScheduleConfigurationBuilder {
    if builder.schedule_expression.is_none() {
        builder.schedule_expression = Some(Default::default())
    }
    builder
}

pub(crate) fn external_url_config_correct_errors(
    mut builder: crate::types::builders::ExternalUrlConfigBuilder,
) -> crate::types::builders::ExternalUrlConfigBuilder {
    if builder.access_url.is_none() {
        builder.access_url = Some(Default::default())
    }
    builder
}

pub(crate) fn publication_correct_errors(mut builder: crate::types::builders::PublicationBuilder) -> crate::types::builders::PublicationBuilder {
    if builder.event.is_none() {
        builder.event = Some(Default::default())
    }
    if builder.schema.is_none() {
        builder.schema = Some(Default::default())
    }
    builder
}

pub(crate) fn subscription_correct_errors(mut builder: crate::types::builders::SubscriptionBuilder) -> crate::types::builders::SubscriptionBuilder {
    if builder.event.is_none() {
        builder.event = Some(Default::default())
    }
    builder
}

pub(crate) fn execution_configuration_correct_errors(
    mut builder: crate::types::builders::ExecutionConfigurationBuilder,
) -> crate::types::builders::ExecutionConfigurationBuilder {
    if builder.execution_mode.is_none() {
        builder.execution_mode = "no value was set".parse::<crate::types::ExecutionMode>().ok()
    }
    builder
}

pub(crate) fn on_demand_configuration_correct_errors(
    mut builder: crate::types::builders::OnDemandConfigurationBuilder,
) -> crate::types::builders::OnDemandConfigurationBuilder {
    if builder.start_time.is_none() {
        builder.start_time = Some(Default::default())
    }
    builder
}
