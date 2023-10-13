use super::content::ContentInput;

use gtk::prelude::*;
use relm4::prelude::*;

use serde::{Deserialize, Serialize};

pub(crate) struct TaskRow {
    pub(crate) task: Task,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Task {
    pub(crate) description: String,
    pub(crate) completed: bool,
}

#[derive(Debug)]
pub(crate) enum TaskRowInput {
    Toggle,
}

#[derive(Debug)]
pub(crate) enum TaskRowOutput {
    Remove(DynamicIndex),
}

#[relm4::factory(pub(crate))]
impl FactoryComponent for TaskRow {
    type Init = Task;

    type Input = TaskRowInput;
    type Output = TaskRowOutput;

    type CommandOutput = ();
    type ParentInput = ContentInput;
    type ParentWidget = gtk::ListBox;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 8,

            gtk::CheckButton {
                set_label: Some(self.task.description.as_str()),
                set_halign: gtk::Align::Start,
                set_active: self.task.completed,
                set_hexpand: true,
                set_margin_all: 8,

                connect_toggled[sender] => move |_| {
                    sender.input(Self::Input::Toggle)
                },
            },

            gtk::Button {
                set_icon_name: "edit-delete-symbolic",
                set_css_classes: &["destructive-action"],
                set_margin_all: 8,

                connect_clicked[sender, index] => move |_| {
                    sender.output(Self::Output::Remove(index.clone()));
                },
            },
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(match output {
            Self::Output::Remove(index) => ContentInput::RemoveTask(index),
        })
    }

    fn init_model(
        task: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self { task }
    }

    fn update(&mut self, input: Self::Input, _sender: FactorySender<Self>) {
        match input {
            Self::Input::Toggle => {
                self.task.completed = !self.task.completed;
            }
        }
    }
}
