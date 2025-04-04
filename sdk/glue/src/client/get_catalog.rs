// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetCatalog`](crate::operation::get_catalog::builders::GetCatalogFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::operation::get_catalog::builders::GetCatalogFluentBuilder::catalog_id) / [`set_catalog_id(Option<String>)`](crate::operation::get_catalog::builders::GetCatalogFluentBuilder::set_catalog_id):<br>required: **true**<br><p>The ID of the parent catalog in which the catalog resides. If none is provided, the Amazon Web Services Account Number is used by default.</p><br>
    /// - On success, responds with [`GetCatalogOutput`](crate::operation::get_catalog::GetCatalogOutput) with field(s):
    ///   - [`catalog(Option<Catalog>)`](crate::operation::get_catalog::GetCatalogOutput::catalog): <p>A <code>Catalog</code> object. The definition of the specified catalog in the Glue Data Catalog.</p>
    /// - On failure, responds with [`SdkError<GetCatalogError>`](crate::operation::get_catalog::GetCatalogError)
    pub fn get_catalog(&self) -> crate::operation::get_catalog::builders::GetCatalogFluentBuilder {
        crate::operation::get_catalog::builders::GetCatalogFluentBuilder::new(self.handle.clone())
    }
}
