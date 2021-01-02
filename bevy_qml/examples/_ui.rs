use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(CameraUiBundle::default())
        // absolute positioning
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(210.0),
                    bottom: Val::Px(10.0),
                    ..Default::default()
                },
                // border: Rect::all(Val::Px(20.0)),
                ..Default::default()
            },
            // material: materials.add(Color::rgb(0.4, 0.4, 1.0).into()),
            ..Default::default()
        });

    let file = std::fs::read_to_string("bevy_qml/examples/ui.qml").unwrap();
    let mut parser = bevy_qml::QMLParser::new();
    bevy_qml::register_ui_import(&mut parser, commands);
    if let Err(e) = parser.process(file.as_str()) {
        eprintln!("QML parsing error:\n{}", e);
        std::process::exit(1);
    }

    // .spawn(NodeBundle {
    //     style: Style {
    //         size: Size::new(Val::Px(200.0), Val::Px(200.0)),
    //         position_type: PositionType::Absolute,
    //         position: Rect {
    //             left: Val::Px(210.0),
    //             top: Val::Px(10.0),
    //             ..Default::default()
    //         },
    //         // border: Rect::all(Val::Px(20.0)),
    //         ..Default::default()
    //     },
    //     // material: materials.add(Color::rgb(0.4, 0.4, 1.0).into()),
    //     ..Default::default()
    // });
}
