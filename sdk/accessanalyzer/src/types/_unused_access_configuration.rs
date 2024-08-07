// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about an unused access analyzer.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UnusedAccessConfiguration {
    /// <p>The specified access age in days for which to generate findings for unused access. For example, if you specify 90 days, the analyzer will generate findings for IAM entities within the accounts of the selected organization for any access that hasn't been used in 90 or more days since the analyzer's last scan. You can choose a value between 1 and 180 days.</p>
    pub unused_access_age: ::std::option::Option<i32>,
}
impl UnusedAccessConfiguration {
    /// <p>The specified access age in days for which to generate findings for unused access. For example, if you specify 90 days, the analyzer will generate findings for IAM entities within the accounts of the selected organization for any access that hasn't been used in 90 or more days since the analyzer's last scan. You can choose a value between 1 and 180 days.</p>
    pub fn unused_access_age(&self) -> ::std::option::Option<i32> {
        self.unused_access_age
    }
}
impl UnusedAccessConfiguration {
    /// Creates a new builder-style object to manufacture [`UnusedAccessConfiguration`](crate::types::UnusedAccessConfiguration).
    pub fn builder() -> crate::types::builders::UnusedAccessConfigurationBuilder {
        crate::types::builders::UnusedAccessConfigurationBuilder::default()
    }
}

/// A builder for [`UnusedAccessConfiguration`](crate::types::UnusedAccessConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UnusedAccessConfigurationBuilder {
    pub(crate) unused_access_age: ::std::option::Option<i32>,
}
impl UnusedAccessConfigurationBuilder {
    /// <p>The specified access age in days for which to generate findings for unused access. For example, if you specify 90 days, the analyzer will generate findings for IAM entities within the accounts of the selected organization for any access that hasn't been used in 90 or more days since the analyzer's last scan. You can choose a value between 1 and 180 days.</p>
    pub fn unused_access_age(mut self, input: i32) -> Self {
        self.unused_access_age = ::std::option::Option::Some(input);
        self
    }
    /// <p>The specified access age in days for which to generate findings for unused access. For example, if you specify 90 days, the analyzer will generate findings for IAM entities within the accounts of the selected organization for any access that hasn't been used in 90 or more days since the analyzer's last scan. You can choose a value between 1 and 180 days.</p>
    pub fn set_unused_access_age(mut self, input: ::std::option::Option<i32>) -> Self {
        self.unused_access_age = input;
        self
    }
    /// <p>The specified access age in days for which to generate findings for unused access. For example, if you specify 90 days, the analyzer will generate findings for IAM entities within the accounts of the selected organization for any access that hasn't been used in 90 or more days since the analyzer's last scan. You can choose a value between 1 and 180 days.</p>
    pub fn get_unused_access_age(&self) -> &::std::option::Option<i32> {
        &self.unused_access_age
    }
    /// Consumes the builder and constructs a [`UnusedAccessConfiguration`](crate::types::UnusedAccessConfiguration).
    pub fn build(self) -> crate::types::UnusedAccessConfiguration {
        crate::types::UnusedAccessConfiguration {
            unused_access_age: self.unused_access_age,
        }
    }
}
