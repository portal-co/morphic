use std::{
    collections::BTreeMap,
    mem::take,
    sync::{Arc, Mutex, OnceLock},
    task::Waker,
};

use atomic_waker::AtomicWaker;
// use js_sys::{Array, Object, Reflect};
// use wasm_bindgen::prelude::*;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{self, EventLoop, EventLoopProxy},
    window::{Window, WindowAttributes, WindowId},
};
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct Core {
    pub pipe: CorePipe,
}
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct CorePipe {
    core_windows: Arc<Mutex<BTreeMap<WindowId, Arc<CoreWindow>>>>,
    event_loop: Arc<OnceLock<EventLoopProxy<()>>>,
    queue: Arc<Mutex<Vec<(QueueEntry, Option<Arc<AtomicWaker>>)>>>,
}
enum QueueEntry {
    CreateWindow(WindowAttributes, Arc<OnceLock<WindowId>>),
}
pub struct CoreWindow {
    window: Window,
}
impl Core {
    pub fn run_event_loop(&self) {
        let e = EventLoop::builder().build().unwrap();
        self.pipe.event_loop.set(e.create_proxy()).unwrap();
        if cfg!(target_arch = "wasm32") && cfg!(target_os = "unknown") && cfg!(feature = "web") {
            #[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "web"))]
            use winit::platform::web::EventLoopExtWebSys;
            #[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "web"))]
            e.spawn_app(self.pipe.clone());
        } else {
            let this = self.pipe.clone();
            e.run_app(&mut self.pipe.clone());
        }
    }
}
impl CorePipe {
    pub fn create_window(&self, attrs: WindowAttributes) -> CreateWindow {
        CreateWindow {
            pipe: self.clone(),
            id: OnceLock::new(),
            attrs,
        }
    }
}
pub struct CreateWindow {
    pipe: CorePipe,
    id: OnceLock<(Arc<OnceLock<WindowId>>, Arc<AtomicWaker>)>,
    attrs: WindowAttributes,
}
impl Future for CreateWindow {
    type Output = Arc<CoreWindow>;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let (id, w) = self.id.get_or_init(|| {
            let lock: Arc<OnceLock<WindowId>> = Arc::new(OnceLock::new());
            let b = Arc::new(AtomicWaker::new());
            b.register(cx.waker());
            self.pipe.queue.lock().unwrap().push((
                QueueEntry::CreateWindow(self.attrs.clone(), lock.clone()),
                Some(b.clone()),
            ));
            (lock, b)
        });
        if let Some(id) = id
            .get()
            .and_then(|w| self.pipe.core_windows.lock().ok()?.get(w).cloned())
        {
            std::task::Poll::Ready(id)
        } else {
            w.register(cx.waker());
            std::task::Poll::Pending
        }
    }
}
impl CorePipe {
    fn sync(&self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut q = self.queue.lock().unwrap();
        for (a, c) in take(&mut *q) {
            match a {
                QueueEntry::CreateWindow(a, b) => {
                    if let Ok(w) = event_loop.create_window(a) {
                        let window = Arc::new(CoreWindow { window: w });
                        self.core_windows
                            .lock()
                            .unwrap()
                            .insert(window.window.id(), window.clone());
                        b.set(window.window.id());
                        if let Some(waker) = c {
                            waker.wake();
                        }
                    }
                }
            }
        }
    }
}
impl ApplicationHandler for CorePipe {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.sync(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.sync(event_loop);
        if let WindowEvent::Destroyed = event
            && let Some(core) = self.core_windows.lock().unwrap().remove(&window_id)
        {}
    }
}
