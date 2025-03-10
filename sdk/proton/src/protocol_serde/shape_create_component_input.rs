// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_component_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_component::CreateComponentInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.service_name {
        object.key("serviceName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.service_instance_name {
        object.key("serviceInstanceName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.environment_name {
        object.key("environmentName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.template_file {
        object.key("templateFile").string(var_6.as_str());
    }
    if let Some(var_7) = &input.manifest {
        object.key("manifest").string(var_7.as_str());
    }
    if let Some(var_8) = &input.service_spec {
        object.key("serviceSpec").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        let mut array_10 = object.key("tags").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.client_token {
        object.key("clientToken").string(var_13.as_str());
    }
    Ok(())
}
