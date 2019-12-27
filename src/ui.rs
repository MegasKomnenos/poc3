use amethyst::{
    ecs::{
        World,
    },
};
use amethyst_imgui::{
    imgui::{
        self, im_str, ImString,
    },
};

pub trait ImguiDrawable: std::fmt::Debug + Send + Sync {
    fn name(&self) -> &'static str;

    fn setup(&mut self, _world: &mut World) {}

    fn draw(&mut self, ui: &imgui::Ui, world: &mut World);

    fn on_toggle_hidden(&mut self, hidden: bool) -> bool {
        hidden
    }
}

#[derive(
    Shrinkwrap,
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
    hidden: bool,
}

#[derive(Default)]
pub struct UiManager {
    windows: HashMap<&'static str, Window>,
}
impl UiManager {
    pub fn draw(&mut self, ui: &imgui::Ui, world: &mut World) {
        self.windows
            .iter_mut()
            .for_each(|(_, window)| window.inner.draw(ui, world));
    }

    pub fn add<W>(mut self, window: W, hidden: bool) -> Self
    where
        W: 'static + ImguiDrawable,
    {
        let id = WindowId(self.windows.len());
        self.windows.insert(
            window.name(),
            Window {
                inner: Box::new(window),
                hidden,
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
            window.hidden = window.inner.on_toggle_hidden(false);
            Ok(())
        } else {
            Err(failure::format_err!("Invalid window"))
        }
    }

    pub fn hide(&mut self, name: &str) -> Result<(), failure::Error> {
        if let Some(window) = self.windows.get_mut(name) {
            window.hidden = window.inner.on_toggle_hidden(true);
            Ok(())
        } else {
            Err(failure::format_err!("Invalid window"))
        }
    }
}