use super::content::ContentInput;

use gtk::prelude::*;
use relm4::{prelude::*, factory::FactoryView};

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

    menu! {
        row_menu: {
            "Remove" => actions::RemoveTask,
        }
    }

    view! {
        row = gtk::Box {
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

            gtk::MenuButton {
                // set_direction: gtk::ArrowType::Down,
                set_icon_name: "view-more-symbolic",
                set_margin_all: 8,
                set_css_classes: &["flat"],

                set_menu_model: Some(&row_menu),
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

mod actions {
    // use gtk::prelude::*;
    use relm4::{prelude::*, actions::{RelmAction, RelmActionGroup}};

    use super::TaskRow;

    relm4::new_action_group!(pub(crate) RowActions, "row");

    relm4::new_stateless_action!(pub(crate) RemoveTask, RowActions, "remove");

    impl TaskRow {
        pub(crate) fn create_actions(
            index: &DynamicIndex,
            widgets: &<Self as FactoryComponent>::Widgets,
            sender: &FactorySender<Self>,
        ) {
            let mut row_actions = RelmActionGroup::<RowActions>::new();

            let remove_row = {
                let sender = sender.clone();
                let index = index.clone();
                RelmAction::<RemoveTask>::new_stateless(move |_| {
                    sender.output(<Self as FactoryComponent>::Output::Remove(index.clone()));
                })
            };
            row_actions.add_action(remove_row);

            row_actions.register_for_widget(&widgets.row);
        }
    }
}
