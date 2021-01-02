use bevy_ecs::Commands;
use bevy_reflect::DynamicStruct;
use qml_parser::QMLParser;
use std::cell::Cell;

pub fn register_ui_import<'a>(_parser: &mut QMLParser, commands: &'a mut Commands) {
    let _gen = Box::new(Cell::new(UiGenerator::new(commands)));
    // parser.register_import("BevyUi", "0.4", gen);
}

pub struct UiGenerator<'a> {
    created: bool,
    _dynamic_struct: DynamicStruct,
    _commands: &'a mut Commands,
}

impl<'a> UiGenerator<'a> {
    pub fn new(commands: &'a mut Commands) -> Self {
        Self {
            created: false,
            _dynamic_struct: DynamicStruct::default(),
            _commands: commands,
        }
    }
}

// impl Generator for UiGenerator<'_> {
//     fn create(&mut self, name: &str) -> Option<Box<dyn Generator>> {
//         Some(Box::new(Self::new(self.commands)))
//     }

//     fn insert_integer(&mut self, attribute: &str, value: i32) {
//         self.dynamic_struct.insert(attribute, value);
//     }

//     fn insert_float(&mut self, attribute: &str, value: f32) {
//         self.dynamic_struct.insert(attribute, value);
//     }

//     fn insert_string(&mut self, attribute: &str, value: String) {
//         self.dynamic_struct.insert(attribute, value);
//     }

//     fn done(&mut self) {
//         self.created = true;
//         println!("Done");
//     }
// }

impl Drop for UiGenerator<'_> {
    fn drop(&mut self) {
        if !self.created {
            // self.done();
        }
    }
}
