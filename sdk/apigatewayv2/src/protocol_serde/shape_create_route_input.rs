// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_route_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_route::CreateRouteInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.api_key_required {
        object.key("apiKeyRequired").boolean(*var_1);
    }
    if let Some(var_2) = &input.authorization_scopes {
        let mut array_3 = object.key("authorizationScopes").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.authorization_type {
        object.key("authorizationType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.authorizer_id {
        object.key("authorizerId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.model_selection_expression {
        object.key("modelSelectionExpression").string(var_7.as_str());
    }
    if let Some(var_8) = &input.operation_name {
        object.key("operationName").string(var_8.as_str());
    }
    if let Some(var_9) = &input.request_models {
        #[allow(unused_mut)]
        let mut object_10 = object.key("requestModels").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    if let Some(var_13) = &input.request_parameters {
        #[allow(unused_mut)]
        let mut object_14 = object.key("requestParameters").start_object();
        for (key_15, value_16) in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_17 = object_14.key(key_15.as_str()).start_object();
                crate::protocol_serde::shape_parameter_constraints::ser_parameter_constraints(&mut object_17, value_16)?;
                object_17.finish();
            }
        }
        object_14.finish();
    }
    if let Some(var_18) = &input.route_key {
        object.key("routeKey").string(var_18.as_str());
    }
    if let Some(var_19) = &input.route_response_selection_expression {
        object.key("routeResponseSelectionExpression").string(var_19.as_str());
    }
    if let Some(var_20) = &input.target {
        object.key("target").string(var_20.as_str());
    }
    Ok(())
}
