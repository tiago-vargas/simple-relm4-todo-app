use gtk::prelude::*;
use relm4::prelude::*;

use std::fs;

mod content;
mod settings;
mod task;

pub(crate) const APP_ID: &str = "com.github.tiago_vargas.simple_relm4_todo";
pub(crate) const FILE_NAME: &str = "data.json";

pub(crate) struct AppModel {
    content: Controller<content::ContentModel>,
}

#[derive(Debug)]
pub(crate) enum AppInput {
    SaveTasks,
    LoadTasks,
}


#[relm4::component(pub(crate))]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppInput;
    type Output = ();

    view! {
        main_window = adw::ApplicationWindow {
            set_title: Some("To-Do"),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar,

                model.content.widget(),
            },

            connect_show[sender] => move |_| {
                sender.input(Self::Input::LoadTasks);
            },

            connect_close_request[sender] => move |_| {
                sender.input(Self::Input::SaveTasks);
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

        Self::load_window_state(&widgets);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::SaveTasks => {
                let content_model = self.content.model();
                let tasks: Vec<&task::Task> = content_model.tasks
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
            Self::Input::LoadTasks => {
                let mut path = gtk::glib::user_data_dir();
                path.push(APP_ID);
                path.push(FILE_NAME);

                if let Ok(file) = fs::File::open(path) {
                    let tasks: Vec<task::Task> = serde_json::from_reader(file)
                        .expect("Could not read data from JSON file.");

                    self.content.sender().send(content::ContentInput::RestoreTasks(tasks))
                        .expect("Could not send message to child component.");
                }
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        Self::save_window_state(&widgets);
    }
}

impl AppModel {
    fn save_window_state(widgets: &<Self as SimpleComponent>::Widgets) {
        let settings = gtk::gio::Settings::new(APP_ID);

        let (width, height) = widgets.main_window.default_size();
        let _ = settings.set_int(settings::Settings::WindowWidth.as_str(), width);
        let _ = settings.set_int(settings::Settings::WindowHeight.as_str(), height);

        let _ = settings.set_boolean(
            settings::Settings::WindowIsMaximized.as_str(),
            widgets.main_window.is_maximized(),
        );
    }

    fn load_window_state(widgets: &<Self as SimpleComponent>::Widgets) {
        let settings = gtk::gio::Settings::new(APP_ID);

        let width = settings.int(settings::Settings::WindowWidth.as_str());
        let height = settings.int(settings::Settings::WindowHeight.as_str());

        widgets.main_window.set_default_size(width, height);

        let is_maximized = settings.boolean(settings::Settings::WindowIsMaximized.as_str());
        if is_maximized {
            widgets.main_window.maximize();
        }
    }
}
