use bevy_ecs::Commands;
use bevy_reflect::DynamicStruct;
use qml_parser::{Generator, QMLParser};

pub fn register_ui_import(parser: &mut QMLParser, _commands: &mut Commands) {
    parser.register_import(
        "BevyUi",
        "0.4",
        Box::new(|name: &str| {
            let generator = UiGenerator::default();

            Some(Box::new(generator))
        }),
    );
}

#[derive(Default)]
pub struct UiGenerator {
    created: bool,
    dynamic_struct: DynamicStruct,
}

impl Generator for UiGenerator {
    fn insert_integer(&mut self, attribute: &str, value: i32) {
        self.dynamic_struct.insert(attribute, value);
    }

    fn insert_float(&mut self, attribute: &str, value: f32) {
        self.dynamic_struct.insert(attribute, value);
    }

    fn insert_string(&mut self, attribute: &str, value: String) {
        self.dynamic_struct.insert(attribute, value);
    }

    fn done(&mut self) {
        self.created = true;
        println!("Done");
    }
}

impl Drop for UiGenerator {
    fn drop(&mut self) {
        if !self.created {
            self.done();
        }
    }
}
