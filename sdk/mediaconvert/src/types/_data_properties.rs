// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Properties specific to data tracks.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DataProperties {
    /// the language code of the track
    pub language_code: ::std::option::Option<::std::string::String>,
}
impl DataProperties {
    /// the language code of the track
    pub fn language_code(&self) -> ::std::option::Option<&str> {
        self.language_code.as_deref()
    }
}
impl DataProperties {
    /// Creates a new builder-style object to manufacture [`DataProperties`](crate::types::DataProperties).
    pub fn builder() -> crate::types::builders::DataPropertiesBuilder {
        crate::types::builders::DataPropertiesBuilder::default()
    }
}

/// A builder for [`DataProperties`](crate::types::DataProperties).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DataPropertiesBuilder {
    pub(crate) language_code: ::std::option::Option<::std::string::String>,
}
impl DataPropertiesBuilder {
    /// the language code of the track
    pub fn language_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.language_code = ::std::option::Option::Some(input.into());
        self
    }
    /// the language code of the track
    pub fn set_language_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.language_code = input;
        self
    }
    /// the language code of the track
    pub fn get_language_code(&self) -> &::std::option::Option<::std::string::String> {
        &self.language_code
    }
    /// Consumes the builder and constructs a [`DataProperties`](crate::types::DataProperties).
    pub fn build(self) -> crate::types::DataProperties {
        crate::types::DataProperties {
            language_code: self.language_code,
        }
    }
}
