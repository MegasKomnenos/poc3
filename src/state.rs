use amethyst::{
    input::{
        is_close_requested, is_key_down,
    },
    winit, GameData, StateData, StateEvent, Trans, State,
};

use crate::ui::{ 
    UiManager, TestUi,
};

#[derive(Default)]
pub struct TestState {
    ui_manager: Option<UiManager>,
}
impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for TestState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        self.ui_manager = Some(
            UiManager::default()
                .add(TestUi, true)
                .build(world),
        );
    }

    fn update(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        let StateData { world, .. } = data;

        data.data.update(world);

        amethyst_imgui::with(|ui| {
            self.ui_manager.as_mut().unwrap().draw(ui, world);
        });

        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, winit::VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}