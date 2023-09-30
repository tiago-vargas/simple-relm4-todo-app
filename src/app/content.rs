use relm4::prelude::*;
use gtk::prelude::*;

pub(crate) struct ContentModel;

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

            gtk::Entry {
                set_placeholder_text: Some("Enter a Task..."),

                connect_activate[sender] => move |entry| {
                    sender.input(Self::Input::AddTask(entry.text().to_string()));
                    sender.input(Self::Input::ClearBuffer(entry.buffer()));
                },
            },
        },
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ContentModel;

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::AddTask(text) if text.is_empty() => (),
            Self::Input::AddTask(text) => println!("{text:?}"),  // TODO: Implement actual action
            Self::Input::ClearBuffer(buffer) => buffer.set_text(""),
        }
    }
}
