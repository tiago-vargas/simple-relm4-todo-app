use super::content::ContentInput;

use adw::prelude::*;
use relm4::{prelude::*, factory::FactoryView};

use serde::{Deserialize, Serialize};

mod task_row_actions;

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
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
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

    menu! {
        row_menu: {
            section! {
                "Move Up" => task_row_actions::MoveRowUp,
                "Move Down" => task_row_actions::MoveRowDown,
            },
            section! {
                "Remove" => task_row_actions::RemoveRow,
            },
        }
    }

    view! {
        adw::ActionRow {
            set_title: self.task.description.as_str(),

            add_prefix = &gtk::CheckButton {
                set_active: self.task.completed,

                connect_toggled[sender] => move |_| {
                    sender.input(Self::Input::Toggle)
                },
            },

            #[name = "menu"]
            add_suffix = &gtk::MenuButton {
                set_icon_name: "view-more-symbolic",
                set_valign: gtk::Align::Center,
                set_css_classes: &["flat"],

                set_menu_model: Some(&row_menu),
            },
        },
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(match output {
            Self::Output::Remove(index) => ContentInput::RemoveTask(index),
            Self::Output::MoveUp(index) => ContentInput::MoveTaskUp(index),
            Self::Output::MoveDown(index) => ContentInput::MoveTaskDown(index),
        })
    }

    fn init_model(
        task: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self { task }
    }

    fn init_widgets(
        &mut self,
        index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as FactoryView>::ReturnedWidget,
        sender: FactorySender<Self>,
    ) -> Self::Widgets {
        let widgets = view_output!();

        Self::create_actions(index, &widgets, &sender);

        widgets
    }

    fn update(&mut self, input: Self::Input, _sender: FactorySender<Self>) {
        match input {
            Self::Input::Toggle => {
                self.task.completed = !self.task.completed;
            }
        }
    }
}
