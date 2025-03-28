// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_package_scope_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_package_scope::UpdatePackageScopeInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.operation {
        object.key("Operation").string(var_1.as_str());
    }
    if let Some(var_2) = &input.package_id {
        object.key("PackageID").string(var_2.as_str());
    }
    if let Some(var_3) = &input.package_user_list {
        let mut array_4 = object.key("PackageUserList").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}
