use bevy::app::App;
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

use crate::model::modetraits::GameMode;
use crate::modes::mode_state::GameModeState;
use crate::modes::mode_state::GameModeState::InDungeon;

pub struct BattleMode;

impl BattleMode {
    fn on_enter() {
        println!("entering battle mode!");
    }

    fn on_exit() {
        println!("exiting battle mode!");
    }

    fn update(
        keyboard_input: Res<Input<KeyCode>>,
        mut next_state: ResMut<NextState<GameModeState>>,
    ) {
        println!("we're in battle mode!");
        if keyboard_input.just_pressed(KeyCode::Semicolon) {
            next_state.set(InDungeon);
        }
    }
}

impl GameMode for BattleMode {
    fn init(app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameModeState::LoadingBattle)
                .continue_to_state(GameModeState::InBattle),
        )
        .add_systems(OnEnter(GameModeState::InBattle), BattleMode::on_enter)
        .add_systems(OnExit(GameModeState::InBattle), BattleMode::on_exit)
        .add_systems(
            Update,
            BattleMode::update.run_if(in_state(GameModeState::InBattle)),
        );
    }
}
