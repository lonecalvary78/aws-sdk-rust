// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateVolume`](crate::operation::associate_volume::builders::AssociateVolumeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workspace_instance_id(impl Into<String>)`](crate::operation::associate_volume::builders::AssociateVolumeFluentBuilder::workspace_instance_id) / [`set_workspace_instance_id(Option<String>)`](crate::operation::associate_volume::builders::AssociateVolumeFluentBuilder::set_workspace_instance_id):<br>required: **true**<br><p>WorkSpace Instance to attach volume to.</p><br>
    ///   - [`volume_id(impl Into<String>)`](crate::operation::associate_volume::builders::AssociateVolumeFluentBuilder::volume_id) / [`set_volume_id(Option<String>)`](crate::operation::associate_volume::builders::AssociateVolumeFluentBuilder::set_volume_id):<br>required: **true**<br><p>Volume to be attached.</p><br>
    ///   - [`device(impl Into<String>)`](crate::operation::associate_volume::builders::AssociateVolumeFluentBuilder::device) / [`set_device(Option<String>)`](crate::operation::associate_volume::builders::AssociateVolumeFluentBuilder::set_device):<br>required: **true**<br><p>Device path for volume attachment.</p><br>
    /// - On success, responds with [`AssociateVolumeOutput`](crate::operation::associate_volume::AssociateVolumeOutput)
    /// - On failure, responds with [`SdkError<AssociateVolumeError>`](crate::operation::associate_volume::AssociateVolumeError)
    pub fn associate_volume(&self) -> crate::operation::associate_volume::builders::AssociateVolumeFluentBuilder {
        crate::operation::associate_volume::builders::AssociateVolumeFluentBuilder::new(self.handle.clone())
    }
}
