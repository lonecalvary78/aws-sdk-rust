// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_recommendation_preferences_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_recommendation_preferences::PutRecommendationPreferencesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.resource_type {
        object.key("resourceType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.scope {
        #[allow(unused_mut)]
        let mut object_3 = object.key("scope").start_object();
        crate::protocol_serde::shape_scope::ser_scope(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.enhanced_infrastructure_metrics {
        object.key("enhancedInfrastructureMetrics").string(var_4.as_str());
    }
    if let Some(var_5) = &input.inferred_workload_types {
        object.key("inferredWorkloadTypes").string(var_5.as_str());
    }
    if let Some(var_6) = &input.external_metrics_preference {
        #[allow(unused_mut)]
        let mut object_7 = object.key("externalMetricsPreference").start_object();
        crate::protocol_serde::shape_external_metrics_preference::ser_external_metrics_preference(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.look_back_period {
        object.key("lookBackPeriod").string(var_8.as_str());
    }
    if let Some(var_9) = &input.utilization_preferences {
        let mut array_10 = object.key("utilizationPreferences").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_utilization_preference::ser_utilization_preference(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.preferred_resources {
        let mut array_14 = object.key("preferredResources").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_preferred_resource::ser_preferred_resource(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.savings_estimation_mode {
        object.key("savingsEstimationMode").string(var_17.as_str());
    }
    Ok(())
}
