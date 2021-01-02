mod parser {
    pub struct Parser {}

    impl Parser {
        pub fn new() -> Self {
            Self {}
        }

        pub fn process(&mut self, _content: &str) -> Result<(), String> {
            Ok(())
        }
    }
}

mod app {
    use super::parser::Parser;
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
        if let Err(e) = parser.process(file) {
            eprintln!("QML parsing error:\n{}", e);
            std::process::exit(1);
        }

        println!("parsed");
    }
}

fn main() {
    app::main()
}
