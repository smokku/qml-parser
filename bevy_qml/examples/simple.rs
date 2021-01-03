/* Creating separate module for fake parser, that does not know anything about Bevy. */
mod parser {
    pub struct Parser<'a> {
        generators: Vec<&'a mut dyn Generator>,
    }

    // Parser skeleton, that uses registered generators during parsing
    impl<'a> Parser<'a> {
        pub fn new() -> Self {
            Self {
                generators: Vec::new(),
            }
        }

        pub fn register_generator(&mut self, generator: &'a mut dyn Generator) {
            self.generators.push(generator);
        }

        pub fn process(&mut self, _content: &str) -> Result<(), String> {
            for gen in &mut self.generators {
                gen.create("Object"); // Let's assume we parsed "Object" symbol
            }
            Ok(())
        }
    }

    // Generator trait to be implemented by user
    pub trait Generator {
        fn create(&mut self, name: &str);
    }
}

/* Create separate module, that uses parser internals and Bevy Commands for parsing. */
mod app {
    use super::parser::{Generator, Parser};
    use bevy::{app::ScheduleRunnerSettings, prelude::*};

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
        let file = "Object {}"; // fake file content
        let mut parser = Parser::new();
        let mut generator = AppGenerator { commands };
        parser.register_generator(&mut generator);
        if let Err(e) = parser.process(file) {
            eprintln!("Parsing error:\n{}", e);
            std::process::exit(1);
        }

        println!("Parsing successful");
    }

    fn count_entities(world: &mut World, _resources: &mut Resources) {
        println!("entities: {}", world.iter().count());
    }

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

    struct Object {}
}

fn main() {
    app::main()
}
