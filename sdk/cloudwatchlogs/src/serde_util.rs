// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn rejected_entity_info_correct_errors(
    mut builder: crate::types::builders::RejectedEntityInfoBuilder,
) -> crate::types::builders::RejectedEntityInfoBuilder {
    if builder.error_type.is_none() {
        builder.error_type = "no value was set".parse::<crate::types::EntityRejectionErrorType>().ok()
    }
    builder
}

pub(crate) fn anomaly_correct_errors(mut builder: crate::types::builders::AnomalyBuilder) -> crate::types::builders::AnomalyBuilder {
    if builder.anomaly_id.is_none() {
        builder.anomaly_id = Some(Default::default())
    }
    if builder.pattern_id.is_none() {
        builder.pattern_id = Some(Default::default())
    }
    if builder.anomaly_detector_arn.is_none() {
        builder.anomaly_detector_arn = Some(Default::default())
    }
    if builder.pattern_string.is_none() {
        builder.pattern_string = Some(Default::default())
    }
    if builder.first_seen.is_none() {
        builder.first_seen = Some(Default::default())
    }
    if builder.last_seen.is_none() {
        builder.last_seen = Some(Default::default())
    }
    if builder.description.is_none() {
        builder.description = Some(Default::default())
    }
    if builder.active.is_none() {
        builder.active = Some(Default::default())
    }
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::State>().ok()
    }
    if builder.histogram.is_none() {
        builder.histogram = Some(Default::default())
    }
    if builder.log_samples.is_none() {
        builder.log_samples = Some(Default::default())
    }
    if builder.pattern_tokens.is_none() {
        builder.pattern_tokens = Some(Default::default())
    }
    if builder.log_group_arn_list.is_none() {
        builder.log_group_arn_list = Some(Default::default())
    }
    builder
}

pub(crate) fn delivery_destination_configuration_correct_errors(
    mut builder: crate::types::builders::DeliveryDestinationConfigurationBuilder,
) -> crate::types::builders::DeliveryDestinationConfigurationBuilder {
    if builder.destination_resource_arn.is_none() {
        builder.destination_resource_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn metric_transformation_correct_errors(
    mut builder: crate::types::builders::MetricTransformationBuilder,
) -> crate::types::builders::MetricTransformationBuilder {
    if builder.metric_name.is_none() {
        builder.metric_name = Some(Default::default())
    }
    if builder.metric_namespace.is_none() {
        builder.metric_namespace = Some(Default::default())
    }
    if builder.metric_value.is_none() {
        builder.metric_value = Some(Default::default())
    }
    builder
}
