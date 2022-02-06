use bevy::prelude::{ResMut, Res};
use bevy_egui::{
    egui::{self, pos2},
    EguiContext,
};

use crate::Global;

pub fn ui_system(egui_context: ResMut<EguiContext>, app_state:Res<Global>) {
    egui::Area::new("main")
        .fixed_pos(egui::pos2(16.0, 16.0))
        .show(egui_context.ctx(), |ui| {
            let s = format!("{:?}", app_state);

            ui.label(s);
        });
}
