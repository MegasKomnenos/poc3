use amethyst::{
    ecs::{
        World,
    },
};
use amethyst_imgui::{
    imgui::{
        self, im_str, ImString, Condition,
    },
};

use std::collections::HashMap;

pub trait ImguiDrawable: std::fmt::Debug + Send + Sync {
    fn name(&self) -> &'static str;

    fn setup(&mut self, _world: &mut World) {}

    fn draw(&mut self, ui: &imgui::Ui, world: &mut World, open: &mut bool);

    fn on_toggle_open(&mut self, open: bool) -> bool {
        open
    }
}

#[derive(Debug, Default)]
pub struct TestUi;
impl ImguiDrawable for TestUi {
    fn name(&self) -> &'static str {
        "TestUi"
    }

    fn draw(&mut self, ui: &imgui::Ui, _world: &mut World, open: &mut bool) {
        imgui::Window::new(im_str!("Hello world"))
            .opened(open)
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });
    }
}

#[derive(
    Default,
    Copy,
    Clone,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct WindowId(usize);

pub struct Window {
    id: WindowId,
    inner: Box<dyn ImguiDrawable>,
    open: bool,
}

#[derive(Default)]
pub struct UiManager {
    windows: HashMap<&'static str, Window>,
}
impl UiManager {
    pub fn draw(&mut self, ui: &imgui::Ui, world: &mut World) {
        self.windows
            .iter_mut()
            .filter(|(_, window)| window.open)
            .for_each(|(_, window)| window.inner.draw(ui, world, &mut window.open));
    }

    pub fn add<W>(mut self, window: W, open: bool) -> Self
    where
        W: 'static + ImguiDrawable,
    {
        let id = WindowId(self.windows.len());
        self.windows.insert(
            window.name(),
            Window {
                inner: Box::new(window),
                open,
                id,
            },
        );

        self
    }

    pub fn build(mut self, world: &mut World) -> Self {
        self.windows
            .iter_mut()
            .for_each(|(_, window)| window.inner.setup(world));

        self
    }

    pub fn open(&mut self, name: &str) -> Result<(), failure::Error> {
        if let Some(window) = self.windows.get_mut(name) {
            window.open = window.inner.on_toggle_open(true);
            Ok(())
        } else {
            Err(failure::format_err!("Invalid window"))
        }
    }

    pub fn hide(&mut self, name: &str) -> Result<(), failure::Error> {
        if let Some(window) = self.windows.get_mut(name) {
            window.open = window.inner.on_toggle_open(false);
            Ok(())
        } else {
            Err(failure::format_err!("Invalid window"))
        }
    }
}