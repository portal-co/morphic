use crate::*;


pub(crate)struct CoreWindow {
    window: Window,
}
impl Core {
    fn create_win(&self) -> Arc<CoreWindow> {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        let c = Reflect::apply(
            self.create_canvas.unchecked_into(),
            &JsValue::undefined(),
            &Array::new(),
        )
        .unwrap();
        let mut builder = winit::window::WindowBuilder::new();
        builder = builder.with_canvas(c.unchecked_into());
        let window = Arc::new(CoreWindow {
            window: builder.build(self.event_loop()).unwrap(),
        });
        self.core_windows.lock().unwrap().insert(window.window.id(), window.clone());
        return window;
    }
    fn event_loop(&self) -> &EventLoop<()> {
        return self.event_loop.get_or_init(|| {
            let e = EventLoop::builder().build().unwrap();
            use winit::platform::web::EventLoopExtWebSys;
            e.spawn_app(self.clone());
            e
        });
    }
}
impl ApplicationHandler for Core {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {}


    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let WindowEvent::Destroyed = event && let Some(core) = self.core_windows.lock().unwrap().remove(&window_id){
            
        }
    }
}
