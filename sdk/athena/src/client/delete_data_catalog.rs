// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDataCatalog`](crate::operation::delete_data_catalog::builders::DeleteDataCatalogFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::delete_data_catalog::builders::DeleteDataCatalogFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_data_catalog::builders::DeleteDataCatalogFluentBuilder::set_name):<br>required: **true**<br><p>The name of the data catalog to delete.</p><br>
    ///   - [`delete_catalog_only(bool)`](crate::operation::delete_data_catalog::builders::DeleteDataCatalogFluentBuilder::delete_catalog_only) / [`set_delete_catalog_only(Option<bool>)`](crate::operation::delete_data_catalog::builders::DeleteDataCatalogFluentBuilder::set_delete_catalog_only):<br>required: **false**<br><p>Deletes the Athena Data Catalog. You can only use this with the <code>FEDERATED</code> catalogs. You usually perform this before registering the connector with Glue Data Catalog. After deletion, you will have to manage the Glue Connection and Lambda function.</p><br>
    /// - On success, responds with [`DeleteDataCatalogOutput`](crate::operation::delete_data_catalog::DeleteDataCatalogOutput) with field(s):
    ///   - [`data_catalog(Option<DataCatalog>)`](crate::operation::delete_data_catalog::DeleteDataCatalogOutput::data_catalog): <p>Contains information about a data catalog in an Amazon Web Services account.</p><note>  <p>In the Athena console, data catalogs are listed as "data sources" on the <b>Data sources</b> page under the <b>Data source name</b> column.</p> </note>
    /// - On failure, responds with [`SdkError<DeleteDataCatalogError>`](crate::operation::delete_data_catalog::DeleteDataCatalogError)
    pub fn delete_data_catalog(&self) -> crate::operation::delete_data_catalog::builders::DeleteDataCatalogFluentBuilder {
        crate::operation::delete_data_catalog::builders::DeleteDataCatalogFluentBuilder::new(self.handle.clone())
    }
}
