mod engine;
mod scenes;

use crate::common::cancel_token::CancelToken;
use crate::comm::gui::GuiComm;
use crate::gui::engine::gui_context;
use crate::gui::engine::gui_context::GuiContext;

pub fn run(comm: GuiComm, cancel_token: CancelToken) {
    let mut gui_context = GuiContext::new(gui_context::Settings::default(), cancel_token, comm);

    scenes::intro::run(&mut gui_context);
    loop {
        let player_datas = scenes::pregame::run(&mut gui_context);
        let player_datas = scenes::game::run(&mut gui_context, player_datas);
        scenes::scoreboard::run(&mut gui_context, player_datas);
    }
    //scenes::sandbox::run(&mut gui_context);
}
