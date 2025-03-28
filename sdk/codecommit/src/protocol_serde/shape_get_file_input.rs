// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_file_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_file::GetFileInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.repository_name {
        object.key("repositoryName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.commit_specifier {
        object.key("commitSpecifier").string(var_2.as_str());
    }
    if let Some(var_3) = &input.file_path {
        object.key("filePath").string(var_3.as_str());
    }
    Ok(())
}
