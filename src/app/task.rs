use super::content::ContentInput;

use adw::prelude::*;
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
        adw::ActionRow {
            set_title: self.task.description.as_str(),

            add_prefix = &gtk::CheckButton {
                set_active: self.task.completed,

                connect_toggled[sender] => move |_| {
                    sender.input(Self::Input::Toggle)
                },
            },

            add_suffix = &gtk::Button {
                set_icon_name: "edit-delete-symbolic",
                set_css_classes: &["destructive-action"],
                set_valign: gtk::Align::Center,

                connect_clicked[sender, index] => move |_| {
                    sender.output(Self::Output::Remove(index.clone()));
                },
            },
        },
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
