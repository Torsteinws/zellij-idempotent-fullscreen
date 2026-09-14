use zellij_tile::prelude::*;

use std::collections::BTreeMap;

#[derive(Default)]
struct State {
    permissions_granted: bool,
    tabs: Vec<TabInfo>,
    pane_manifest: PaneManifest,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        self.permissions_granted = false;
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::PermissionRequestResult]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            // ...
            Event::PermissionRequestResult(PermissionStatus::Granted) => {
                // permissions granted, subscribe to events that require them
                subscribe(&[EventType::TabUpdate, EventType::PaneUpdate]);
                self.permissions_granted = true;
            }
            Event::TabUpdate(tab_infos) => {
                self.tabs = tab_infos;
            }
            Event::PaneUpdate(pane_manifest) => {
                self.pane_manifest = pane_manifest;
            }
            _ => {}
        }
        false
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if !self.permissions_granted {
            return false;
        }

        let next_state = match pipe_message.name.trim() {
            "set-normal-screen" => FullscreenState::Normal,
            "set-fullscreen" => FullscreenState::Fullscreen,
            "set-no-ui-fullscreen" => FullscreenState::NoUiFullscreen,
            _ => return false,
        };

        let Some(tab) = get_focused_tab(&self.tabs) else {
            return false;
        };

        let Some(pane) = get_focused_pane(tab.position, &self.pane_manifest) else {
            return false;
        };

        let current_state = get_fullscreen_state(&pane, &tab);
        set_fullscreen_state(current_state, next_state);

        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) {}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FullscreenState {
    Normal,
    Fullscreen,
    NoUiFullscreen,
}

fn get_fullscreen_state(pane: &PaneInfo, tab: &TabInfo) -> FullscreenState {
    if !pane.is_fullscreen {
        return FullscreenState::Normal;
    }

    if pane.pane_x == 0
        && pane.pane_y == 0
        && pane.pane_rows == tab.display_area_rows
        && pane.pane_columns == tab.display_area_columns
    {
        FullscreenState::NoUiFullscreen
    } else {
        FullscreenState::Fullscreen
    }
}

fn set_fullscreen_state(current_state: FullscreenState, next_state: FullscreenState) {
    use FullscreenState::*;
    match (current_state, next_state) {
        (Normal, Fullscreen) => toggle_focus_fullscreen(),
        (Normal, NoUiFullscreen) => toggle_focus_no_ui_fullscreen(),
        (Fullscreen, Normal) => toggle_focus_fullscreen(),
        (Fullscreen, NoUiFullscreen) => toggle_focus_no_ui_fullscreen(),
        (NoUiFullscreen, Normal) => toggle_focus_no_ui_fullscreen(),
        (NoUiFullscreen, Fullscreen) => toggle_focus_fullscreen(),

        (Normal, Normal) | (Fullscreen, Fullscreen) | (NoUiFullscreen, NoUiFullscreen) => {}
    }
}
