// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListPublicKeys`](crate::operation::list_public_keys::builders::ListPublicKeysFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_public_keys::builders::ListPublicKeysFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`marker(impl Into<String>)`](crate::operation::list_public_keys::builders::ListPublicKeysFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_public_keys::builders::ListPublicKeysFluentBuilder::set_marker):<br>required: **false**<br><p>Use this when paginating results to indicate where to begin in your list of public keys. The results include public keys in the list that occur after the marker. To get the next page of results, set the <code>Marker</code> to the value of the <code>NextMarker</code> from the current page's response (which is also the ID of the last public key on that page).</p><br>
    ///   - [`max_items(i32)`](crate::operation::list_public_keys::builders::ListPublicKeysFluentBuilder::max_items) / [`set_max_items(Option<i32>)`](crate::operation::list_public_keys::builders::ListPublicKeysFluentBuilder::set_max_items):<br>required: **false**<br><p>The maximum number of public keys you want in the response body.</p><br>
    /// - On success, responds with [`ListPublicKeysOutput`](crate::operation::list_public_keys::ListPublicKeysOutput) with field(s):
    ///   - [`public_key_list(Option<PublicKeyList>)`](crate::operation::list_public_keys::ListPublicKeysOutput::public_key_list): <p>Returns a list of all public keys that have been added to CloudFront for this account.</p>
    /// - On failure, responds with [`SdkError<ListPublicKeysError>`](crate::operation::list_public_keys::ListPublicKeysError)
    pub fn list_public_keys(&self) -> crate::operation::list_public_keys::builders::ListPublicKeysFluentBuilder {
        crate::operation::list_public_keys::builders::ListPublicKeysFluentBuilder::new(self.handle.clone())
    }
}
