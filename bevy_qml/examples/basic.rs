use bevy::{app::ScheduleRunnerSettings, prelude::*};
use bevy_qml::{Generator, QMLParser};

const FILE: &str = "
import Test 1.0

Object {
    Object {}
}
";

struct AppGenerator<'a> {
    commands: &'a mut Commands,
}

impl Generator for AppGenerator<'_> {
    fn create(&mut self, name: &str) {
        if name == "Object" {
            println!("Creating Object");
            self.commands.spawn((Object {},));
        }
    }
}

impl Drop for AppGenerator<'_> {
    fn drop(&mut self) {
        println!("Dropping Generator");
    }
}

pub fn main() {
    App::build()
        .add_resource(ScheduleRunnerSettings::run_once())
        .add_plugins(MinimalPlugins)
        .add_system_to_stage(stage::FIRST, count_entities.system())
        .add_system(test_parser.system())
        .add_system_to_stage(stage::LAST, count_entities.system())
        .run();
}

fn test_parser(commands: &mut Commands) {
    let mut parser = QMLParser::new();
    let mut generator = AppGenerator { commands };
    parser.register_import("Test", "1.0", &mut generator);
    if let Err(e) = parser.process(FILE) {
        eprintln!("Parsing error:\n{}", e);
        std::process::exit(1);
    }

    println!("Parsing successful");
}

fn count_entities(world: &mut World, _resources: &mut Resources) {
    println!("entities: {}", world.iter().count());
}

struct Object {}
