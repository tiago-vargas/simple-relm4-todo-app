use gtk::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;

mod task;

pub(crate) struct ContentModel {
    tasks: FactoryVecDeque<task::Task>,
}

#[derive(Debug)]
pub(crate) enum ContentInput {
    AddTask(String),
    ClearBuffer(gtk::EntryBuffer),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for ContentModel {
    type Init = ();

    type Input = ContentInput;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_all: 12,
            set_spacing: 12,

            gtk::Entry {
                set_placeholder_text: Some("Enter a Task..."),

                connect_activate[sender] => move |entry| {
                    sender.input(Self::Input::AddTask(entry.text().to_string()));
                    sender.input(Self::Input::ClearBuffer(entry.buffer()));
                },
            },

            #[local_ref]
            task_list_box -> gtk::ListBox {
                set_css_classes: &["boxed-list"],

                #[watch]
                set_visible: !model.tasks.is_empty(),
            },
        },
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let tasks = FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender());
        let model = ContentModel { tasks };

        let task_list_box = model.tasks.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::AddTask(text) if text.is_empty() => (),
            Self::Input::AddTask(text) => {
                self.tasks.guard().push_front(text);
            }
            Self::Input::ClearBuffer(buffer) => buffer.set_text(""),
        }
    }
}
