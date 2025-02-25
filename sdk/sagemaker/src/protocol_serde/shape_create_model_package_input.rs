// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_model_package_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_model_package::CreateModelPackageInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.model_package_name {
        object.key("ModelPackageName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.model_package_group_name {
        object.key("ModelPackageGroupName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.model_package_description {
        object.key("ModelPackageDescription").string(var_3.as_str());
    }
    if let Some(var_4) = &input.inference_specification {
        #[allow(unused_mut)]
        let mut object_5 = object.key("InferenceSpecification").start_object();
        crate::protocol_serde::shape_inference_specification::ser_inference_specification(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.validation_specification {
        #[allow(unused_mut)]
        let mut object_7 = object.key("ValidationSpecification").start_object();
        crate::protocol_serde::shape_model_package_validation_specification::ser_model_package_validation_specification(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.source_algorithm_specification {
        #[allow(unused_mut)]
        let mut object_9 = object.key("SourceAlgorithmSpecification").start_object();
        crate::protocol_serde::shape_source_algorithm_specification::ser_source_algorithm_specification(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.certify_for_marketplace {
        object.key("CertifyForMarketplace").boolean(*var_10);
    }
    if let Some(var_11) = &input.tags {
        let mut array_12 = object.key("Tags").start_array();
        for item_13 in var_11 {
            {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.model_approval_status {
        object.key("ModelApprovalStatus").string(var_15.as_str());
    }
    if let Some(var_16) = &input.metadata_properties {
        #[allow(unused_mut)]
        let mut object_17 = object.key("MetadataProperties").start_object();
        crate::protocol_serde::shape_metadata_properties::ser_metadata_properties(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.model_metrics {
        #[allow(unused_mut)]
        let mut object_19 = object.key("ModelMetrics").start_object();
        crate::protocol_serde::shape_model_metrics::ser_model_metrics(&mut object_19, var_18)?;
        object_19.finish();
    }
    if let Some(var_20) = &input.client_token {
        object.key("ClientToken").string(var_20.as_str());
    }
    if let Some(var_21) = &input.domain {
        object.key("Domain").string(var_21.as_str());
    }
    if let Some(var_22) = &input.task {
        object.key("Task").string(var_22.as_str());
    }
    if let Some(var_23) = &input.sample_payload_url {
        object.key("SamplePayloadUrl").string(var_23.as_str());
    }
    if let Some(var_24) = &input.customer_metadata_properties {
        #[allow(unused_mut)]
        let mut object_25 = object.key("CustomerMetadataProperties").start_object();
        for (key_26, value_27) in var_24 {
            {
                object_25.key(key_26.as_str()).string(value_27.as_str());
            }
        }
        object_25.finish();
    }
    if let Some(var_28) = &input.drift_check_baselines {
        #[allow(unused_mut)]
        let mut object_29 = object.key("DriftCheckBaselines").start_object();
        crate::protocol_serde::shape_drift_check_baselines::ser_drift_check_baselines(&mut object_29, var_28)?;
        object_29.finish();
    }
    if let Some(var_30) = &input.additional_inference_specifications {
        let mut array_31 = object.key("AdditionalInferenceSpecifications").start_array();
        for item_32 in var_30 {
            {
                #[allow(unused_mut)]
                let mut object_33 = array_31.value().start_object();
                crate::protocol_serde::shape_additional_inference_specification_definition::ser_additional_inference_specification_definition(
                    &mut object_33,
                    item_32,
                )?;
                object_33.finish();
            }
        }
        array_31.finish();
    }
    if let Some(var_34) = &input.skip_model_validation {
        object.key("SkipModelValidation").string(var_34.as_str());
    }
    if let Some(var_35) = &input.source_uri {
        object.key("SourceUri").string(var_35.as_str());
    }
    if let Some(var_36) = &input.security_config {
        #[allow(unused_mut)]
        let mut object_37 = object.key("SecurityConfig").start_object();
        crate::protocol_serde::shape_model_package_security_config::ser_model_package_security_config(&mut object_37, var_36)?;
        object_37.finish();
    }
    if let Some(var_38) = &input.model_card {
        #[allow(unused_mut)]
        let mut object_39 = object.key("ModelCard").start_object();
        crate::protocol_serde::shape_model_package_model_card::ser_model_package_model_card(&mut object_39, var_38)?;
        object_39.finish();
    }
    if let Some(var_40) = &input.model_life_cycle {
        #[allow(unused_mut)]
        let mut object_41 = object.key("ModelLifeCycle").start_object();
        crate::protocol_serde::shape_model_life_cycle::ser_model_life_cycle(&mut object_41, var_40)?;
        object_41.finish();
    }
    Ok(())
}
