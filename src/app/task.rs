use super::content::ContentInput;

use gtk::prelude::*;
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
    Set(String),  // DEBUG!
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
        task_row = gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 8,

            gtk::Button {
                set_icon_name: "drag-handle-symbolic",
                set_margin_all: 8,
                set_css_classes: &["flat"],

                add_controller = gtk::DragSource {
                    set_actions: gtk::gdk::DragAction::COPY,

                    connect_prepare => move |_drag_source, _x_start, _y_start| {
                        Some(gtk::gdk::ContentProvider::for_bytes("text/plain", &gtk::glib::Bytes::from_static(crate::app::APP_ID.as_bytes())))
                    },

                    connect_begin[task_row] => move |drag_source, _s| {
                        let p = gtk::WidgetPaintable::new(Some(&task_row));
                        drag_source.set_icon(Some(&p), 24, 24);
                    },
                },
            },

            gtk::CheckButton {
                #[watch] set_label: Some(self.task.description.as_str()),
                set_halign: gtk::Align::Start,
                set_active: self.task.completed,
                set_hexpand: true,
                set_margin_all: 8,

                connect_toggled[sender] => move |_| {
                    sender.input(Self::Input::Toggle)
                },
            },

            #[name = "menu"]
            gtk::MenuButton {
                set_icon_name: "view-more-symbolic",
                set_margin_all: 8,
                set_css_classes: &["flat"],

                set_menu_model: Some(&row_menu),
            },

            add_controller = gtk::DropTarget::new(gtk::glib::Type::STRING, gtk::gdk::DragAction::COPY) {
                // Emitted on the drop site when the user drops the data onto the widget.
                connect_drop[sender] => move |_drop_target, _dropped_value, _x, _y| {
                    println!("Drop: {_x}, {_y}");
                    // self.task.description = "Dropped".to_string();
                    sender.input(Self::Input::Set("Dropped".to_string()));
                    true
                },
            },
        }
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
            Self::Input::Set(s) => {
                self.task.description = s;
            }
        }
    }
}
