// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The additional TO CC recipients information of inbound email.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InboundAdditionalRecipients {
    /// <p>The additional recipients information present in to list.</p>
    pub to_addresses: ::std::option::Option<::std::vec::Vec<crate::types::EmailAddressInfo>>,
    /// <p>The additional recipients information present in cc list.</p>
    pub cc_addresses: ::std::option::Option<::std::vec::Vec<crate::types::EmailAddressInfo>>,
}
impl InboundAdditionalRecipients {
    /// <p>The additional recipients information present in to list.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.to_addresses.is_none()`.
    pub fn to_addresses(&self) -> &[crate::types::EmailAddressInfo] {
        self.to_addresses.as_deref().unwrap_or_default()
    }
    /// <p>The additional recipients information present in cc list.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.cc_addresses.is_none()`.
    pub fn cc_addresses(&self) -> &[crate::types::EmailAddressInfo] {
        self.cc_addresses.as_deref().unwrap_or_default()
    }
}
impl InboundAdditionalRecipients {
    /// Creates a new builder-style object to manufacture [`InboundAdditionalRecipients`](crate::types::InboundAdditionalRecipients).
    pub fn builder() -> crate::types::builders::InboundAdditionalRecipientsBuilder {
        crate::types::builders::InboundAdditionalRecipientsBuilder::default()
    }
}

/// A builder for [`InboundAdditionalRecipients`](crate::types::InboundAdditionalRecipients).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct InboundAdditionalRecipientsBuilder {
    pub(crate) to_addresses: ::std::option::Option<::std::vec::Vec<crate::types::EmailAddressInfo>>,
    pub(crate) cc_addresses: ::std::option::Option<::std::vec::Vec<crate::types::EmailAddressInfo>>,
}
impl InboundAdditionalRecipientsBuilder {
    /// Appends an item to `to_addresses`.
    ///
    /// To override the contents of this collection use [`set_to_addresses`](Self::set_to_addresses).
    ///
    /// <p>The additional recipients information present in to list.</p>
    pub fn to_addresses(mut self, input: crate::types::EmailAddressInfo) -> Self {
        let mut v = self.to_addresses.unwrap_or_default();
        v.push(input);
        self.to_addresses = ::std::option::Option::Some(v);
        self
    }
    /// <p>The additional recipients information present in to list.</p>
    pub fn set_to_addresses(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::EmailAddressInfo>>) -> Self {
        self.to_addresses = input;
        self
    }
    /// <p>The additional recipients information present in to list.</p>
    pub fn get_to_addresses(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::EmailAddressInfo>> {
        &self.to_addresses
    }
    /// Appends an item to `cc_addresses`.
    ///
    /// To override the contents of this collection use [`set_cc_addresses`](Self::set_cc_addresses).
    ///
    /// <p>The additional recipients information present in cc list.</p>
    pub fn cc_addresses(mut self, input: crate::types::EmailAddressInfo) -> Self {
        let mut v = self.cc_addresses.unwrap_or_default();
        v.push(input);
        self.cc_addresses = ::std::option::Option::Some(v);
        self
    }
    /// <p>The additional recipients information present in cc list.</p>
    pub fn set_cc_addresses(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::EmailAddressInfo>>) -> Self {
        self.cc_addresses = input;
        self
    }
    /// <p>The additional recipients information present in cc list.</p>
    pub fn get_cc_addresses(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::EmailAddressInfo>> {
        &self.cc_addresses
    }
    /// Consumes the builder and constructs a [`InboundAdditionalRecipients`](crate::types::InboundAdditionalRecipients).
    pub fn build(self) -> crate::types::InboundAdditionalRecipients {
        crate::types::InboundAdditionalRecipients {
            to_addresses: self.to_addresses,
            cc_addresses: self.cc_addresses,
        }
    }
}
