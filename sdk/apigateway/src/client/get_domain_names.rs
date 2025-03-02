// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDomainNames`](crate::operation::get_domain_names::builders::GetDomainNamesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_domain_names::builders::GetDomainNamesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`position(impl Into<String>)`](crate::operation::get_domain_names::builders::GetDomainNamesFluentBuilder::position) / [`set_position(Option<String>)`](crate::operation::get_domain_names::builders::GetDomainNamesFluentBuilder::set_position):<br>required: **false**<br><p>The current pagination position in the paged result set.</p><br>
    ///   - [`limit(i32)`](crate::operation::get_domain_names::builders::GetDomainNamesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::get_domain_names::builders::GetDomainNamesFluentBuilder::set_limit):<br>required: **false**<br><p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p><br>
    ///   - [`resource_owner(ResourceOwner)`](crate::operation::get_domain_names::builders::GetDomainNamesFluentBuilder::resource_owner) / [`set_resource_owner(Option<ResourceOwner>)`](crate::operation::get_domain_names::builders::GetDomainNamesFluentBuilder::set_resource_owner):<br>required: **false**<br><p>The owner of the domain name access association.</p><br>
    /// - On success, responds with [`GetDomainNamesOutput`](crate::operation::get_domain_names::GetDomainNamesOutput) with field(s):
    ///   - [`items(Option<Vec::<DomainName>>)`](crate::operation::get_domain_names::GetDomainNamesOutput::items): <p>The current page of elements from this collection.</p>
    ///   - [`position(Option<String>)`](crate::operation::get_domain_names::GetDomainNamesOutput::position): <p>The current pagination position in the paged result set.</p>
    /// - On failure, responds with [`SdkError<GetDomainNamesError>`](crate::operation::get_domain_names::GetDomainNamesError)
    pub fn get_domain_names(&self) -> crate::operation::get_domain_names::builders::GetDomainNamesFluentBuilder {
        crate::operation::get_domain_names::builders::GetDomainNamesFluentBuilder::new(self.handle.clone())
    }
}
