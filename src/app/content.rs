use gtk::prelude::*;
use relm4::factory::FactoryVecDeque;
use relm4::prelude::*;

use crate::app::task;

pub(crate) struct ContentModel {
    pub(crate) tasks: FactoryVecDeque<task::TaskRow>,
}

#[derive(Debug)]
pub(crate) enum ContentInput {
    AddTask(task::Task),
    RemoveTask(DynamicIndex),
    MoveTaskUp(DynamicIndex),
    MoveTaskDown(DynamicIndex),
    RestoreTasks(Vec<task::Task>),
    ClearBuffer(gtk::EntryBuffer),
    Swap(DynamicIndex, DynamicIndex),
}

#[relm4::component(pub(crate))]
impl SimpleComponent for ContentModel {
    type Init = ();

    type Input = ContentInput;
    type Output = ();

    view! {
        gtk::ScrolledWindow {
            set_vexpand: true,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 12,
                set_spacing: 12,

                gtk::Entry {
                    set_placeholder_text: Some("Enter a Task..."),

                    connect_activate[sender] => move |entry| {
                        let task = task::Task {
                            description: entry.text().to_string(),
                            completed: false,
                        };
                        sender.input(Self::Input::AddTask(task));
                        sender.input(Self::Input::ClearBuffer(entry.buffer()));
                    },
                },

                #[local_ref]
                task_list_box -> gtk::ListBox {
                    set_css_classes: &["boxed-list"],

                    #[watch]
                    set_visible: !model.tasks.is_empty(),
                },
            }
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
            Self::Input::AddTask(t) if t.description.is_empty() => (),
            Self::Input::AddTask(t) => {
                _ = self.tasks.guard().push_front(t);
            }
            Self::Input::RemoveTask(index) => {
                _ = self.tasks.guard().remove(index.current_index());
            }
            Self::Input::MoveTaskUp(index) => {
                let index = index.current_index();

                let is_at_top = index == 0;
                if !is_at_top {
                    self.tasks.guard().move_to(index, index - 1);
                }
            }
            Self::Input::MoveTaskDown(index) => {
                let index = index.current_index();

                let is_at_bottom = index == self.tasks.len() - 1;
                if !is_at_bottom {
                    self.tasks.guard().move_to(index, index + 1);
                }
            }
            Self::Input::RestoreTasks(tasks) => {
                for t in tasks {
                    _ = self.tasks.guard().push_back(t);
                }
            }
            Self::Input::ClearBuffer(buffer) => buffer.set_text(""),
            Self::Input::Swap(index_1, index_2) => self
                .tasks
                .guard()
                .swap(index_1.current_index(), index_2.current_index()),
        }
    }
}
