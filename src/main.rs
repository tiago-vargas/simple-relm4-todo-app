use relm4::prelude::*;

mod app;

fn main() {
    let app = RelmApp::new("com.github.tiago-vargas.simple-relm4-todo");
    app.run::<app::AppModel>(());
}
