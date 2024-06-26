// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDeliverySource`](crate::operation::get_delivery_source::builders::GetDeliverySourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::get_delivery_source::builders::GetDeliverySourceFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::get_delivery_source::builders::GetDeliverySourceFluentBuilder::set_name):<br>required: **true**<br><p>The name of the delivery source that you want to retrieve.</p><br>
    /// - On success, responds with [`GetDeliverySourceOutput`](crate::operation::get_delivery_source::GetDeliverySourceOutput) with field(s):
    ///   - [`delivery_source(Option<DeliverySource>)`](crate::operation::get_delivery_source::GetDeliverySourceOutput::delivery_source): <p>A structure containing information about the delivery source.</p>
    /// - On failure, responds with [`SdkError<GetDeliverySourceError>`](crate::operation::get_delivery_source::GetDeliverySourceError)
    pub fn get_delivery_source(&self) -> crate::operation::get_delivery_source::builders::GetDeliverySourceFluentBuilder {
        crate::operation::get_delivery_source::builders::GetDeliverySourceFluentBuilder::new(self.handle.clone())
    }
}
