mod parser {
    pub struct Parser<'a> {
        generators: Vec<&'a mut dyn Generator>,
    }

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
                gen.create("Object");
            }
            Ok(())
        }
    }

    pub trait Generator {
        fn create(&mut self, name: &str);
    }
}

mod app {
    use super::parser::{Generator, Parser};
    use bevy::{app::ScheduleRunnerSettings, prelude::*};

    pub fn main() {
        App::build()
            .add_resource(ScheduleRunnerSettings::run_once())
            .add_plugins(MinimalPlugins)
            .add_system(test_parser.system())
            .run();
    }

    fn test_parser() {
        let file = "Object {}";
        let mut parser = Parser::new();
        let mut generator = AppGenerator {};
        parser.register_generator(&mut generator);
        if let Err(e) = parser.process(file) {
            eprintln!("QML parsing error:\n{}", e);
            std::process::exit(1);
        }

        println!("parsed");
    }

    struct AppGenerator {}

    impl Generator for AppGenerator {
        fn create(&mut self, name: &str) {
            if name == "Object" {
                println!("Creating Object");
            }
        }
    }
}

fn main() {
    app::main()
}
