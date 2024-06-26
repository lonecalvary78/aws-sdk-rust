// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateVpcIngressConnection`](crate::operation::update_vpc_ingress_connection::builders::UpdateVpcIngressConnectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vpc_ingress_connection_arn(impl Into<String>)`](crate::operation::update_vpc_ingress_connection::builders::UpdateVpcIngressConnectionFluentBuilder::vpc_ingress_connection_arn) / [`set_vpc_ingress_connection_arn(Option<String>)`](crate::operation::update_vpc_ingress_connection::builders::UpdateVpcIngressConnectionFluentBuilder::set_vpc_ingress_connection_arn):<br>required: **true**<br><p>The Amazon Resource Name (Arn) for the App Runner VPC Ingress Connection resource that you want to update.</p><br>
    ///   - [`ingress_vpc_configuration(IngressVpcConfiguration)`](crate::operation::update_vpc_ingress_connection::builders::UpdateVpcIngressConnectionFluentBuilder::ingress_vpc_configuration) / [`set_ingress_vpc_configuration(Option<IngressVpcConfiguration>)`](crate::operation::update_vpc_ingress_connection::builders::UpdateVpcIngressConnectionFluentBuilder::set_ingress_vpc_configuration):<br>required: **true**<br><p>Specifications for the customer’s Amazon VPC and the related Amazon Web Services PrivateLink VPC endpoint that are used to update the VPC Ingress Connection resource.</p><br>
    /// - On success, responds with [`UpdateVpcIngressConnectionOutput`](crate::operation::update_vpc_ingress_connection::UpdateVpcIngressConnectionOutput) with field(s):
    ///   - [`vpc_ingress_connection(Option<VpcIngressConnection>)`](crate::operation::update_vpc_ingress_connection::UpdateVpcIngressConnectionOutput::vpc_ingress_connection): <p>A description of the App Runner VPC Ingress Connection resource that's updated by this request.</p>
    /// - On failure, responds with [`SdkError<UpdateVpcIngressConnectionError>`](crate::operation::update_vpc_ingress_connection::UpdateVpcIngressConnectionError)
    pub fn update_vpc_ingress_connection(
        &self,
    ) -> crate::operation::update_vpc_ingress_connection::builders::UpdateVpcIngressConnectionFluentBuilder {
        crate::operation::update_vpc_ingress_connection::builders::UpdateVpcIngressConnectionFluentBuilder::new(self.handle.clone())
    }
}
