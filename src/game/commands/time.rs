use bevy::prelude::*;
use bevy_console::ConsoleCommand;
use clap::Parser;

use crate::game::world::time::DayTime;

#[derive(clap::ValueEnum, Parser, PartialEq, Clone)]
#[clap(rename_all = "kebab_case")]
pub enum Factor {
    Default,
    Seconds,
    Minutes,
    Hours,
}

#[derive(clap::ValueEnum, Parser, PartialEq, Clone)]
#[clap(rename_all = "kebab_case")]
pub enum Action {
    Set,
    Add,
    Remove,
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "time")]
pub struct TimeCommand {
    #[clap(value_enum)]
    pub action: Action,
    pub time: f32,
    #[clap(value_enum, default_value_t = Factor::Default)]
    pub factor: Factor,
}

pub fn time(mut log: ConsoleCommand<TimeCommand>, mut day_time: ResMut<DayTime>) {
    if let Some(Ok(TimeCommand {
        action,
        time,
        factor,
    })) = log.take()
    {
        let factor = match factor {
            Factor::Default => 1.0,
            Factor::Seconds => 1.0 / 86400.0,
            Factor::Minutes => 1.0 / 1440.0,
            Factor::Hours => 1.0 / 24.0,
        };

        let value = time * factor;

        match action {
            Action::Set => day_time.time = value,
            Action::Add => day_time.time += value,
            Action::Remove => day_time.time -= value,
        }
    }
}
