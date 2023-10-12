use gtk::prelude::*;
use relm4::prelude::*;

use std::fs;

mod content;

pub(crate) const APP_ID: &str = "com.github.tiago-vargas.simple-relm4-todo";
pub(crate) const FILE_NAME: &str = "data.json";

pub(crate) struct AppModel {
    content: Controller<content::ContentModel>,
}

#[derive(Debug)]
pub(crate) enum AppInput {
    SaveTasks,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppInput;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("To-Do"),
            set_default_width: 400,
            set_default_height: 500,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar,

                model.content.widget(),
            },

            connect_close_request[sender] => move |_| {
                sender.input(AppInput::SaveTasks);
                gtk::Inhibit(false)
            },
        }
    }

    /// Initialize the UI and model.
    fn init(
        _init: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let content = content::ContentModel::builder()
            .launch(())
            .detach();
        let model = AppModel { content };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::SaveTasks => {
                let content_model = self.content.model();
                let tasks: Vec<&content::task::Task> = content_model.tasks
                    .iter()
                    .map(|row| &row.task)
                    .collect();

                let mut path = gtk::glib::user_data_dir();
                path.push(APP_ID);
                fs::create_dir_all(&path)
                    .expect("Could not create directory.");

                path.push(FILE_NAME);
                let file = fs::File::create(path)
                    .expect("Could not create JSON file.");

                serde_json::to_writer_pretty(file, &tasks)
                    .expect("Could not write data to JSON file");
            }
        }
    }
}
