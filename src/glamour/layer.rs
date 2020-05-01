pub trait Layer {
    fn init(&mut self, _app_context: &mut crate::AppContext) {}
    fn on_event(
        &mut self,
        _event: &glutin::event::Event<()>,
        _app_context: &mut crate::AppContext,
    ) {
    }
    fn on_fixed_update(&mut self, _app_context: &mut crate::AppContext) {}
    fn on_frame_update(&mut self, _app_context: &mut crate::AppContext) {}
    fn on_imgui_update(&mut self, _ui: &imgui::Ui, _app_context: &mut crate::AppContext) {}
    fn name(&self) -> &String;
}
