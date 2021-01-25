use bevy::prelude::*;

struct Person;
struct Name(String);

pub struct HelloPlugin;

fn hello_world() {
    println!("hello world!");
}


// Creates a
fn add_people(commands: &mut Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(add_people.system())
            .add_system(hello_world.system())
            .add_system(greet_people.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
