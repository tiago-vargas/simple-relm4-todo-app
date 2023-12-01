use core::ffi;

use super::content::ContentInput;

use gtk::{gdk, prelude::*};
use relm4::{prelude::*, factory::FactoryView};

use serde::{Deserialize, Serialize};

mod task_row_actions;

pub(crate) struct TaskRow {
    pub(crate) task: Task,
    visible_child_name: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Task {
    pub(crate) description: String,
    pub(crate) completed: bool,
}

#[derive(Debug)]
pub(crate) enum TaskRowInput {
    Toggle,
    Hide,
    Show,
}

#[derive(Debug)]
pub(crate) enum TaskRowOutput {
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
    Remove(DynamicIndex),
    Swap(DynamicIndex, DynamicIndex),
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
        task_row = gtk::Stack {
            add_child = &gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 8,

                gtk::Button {
                    set_icon_name: "drag-handle-symbolic",
                    set_margin_all: 8,
                    set_css_classes: &["flat"],

                    add_controller = gtk::DragSource {
                        set_actions: gtk::gdk::DragAction::COPY,

                        connect_begin[task_row] => move |drag_source, _s| {
                            let p = gtk::WidgetPaintable::new(Some(&task_row));
                            drag_source.set_icon(Some(&p), 24, 24);
                        },

                        connect_prepare[index] => move |_drag_source, _x_start, _y_start| {
                            let boxed_index = Box::new(index.clone());
                            let raw = Box::into_raw(boxed_index) as *mut ffi::c_void;
                            Some(gdk::ContentProvider::for_value(&raw.to_value()))
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
            } -> {
                set_name: "row_content",
            },

            add_child = &gtk::Box {
                // Empty; preserves size of the row
            } -> {
                set_name: "row_placeholder",
            },

            #[watch] set_visible_child_name: self.visible_child_name,

            add_controller = gtk::DropTarget::new(gtk::glib::Type::POINTER, gtk::gdk::DragAction::COPY) {
                // Emitted on the drop site when the user drops the data onto the widget.
                connect_drop[sender, index] => move |_drop_target, dropped_value, _x, _y| {
                    match dropped_value.get::<*mut ffi::c_void>() {
                        Ok(pointer) => {
                            let dropped_index = unsafe { Box::from_raw(pointer as *mut DynamicIndex) };
                            sender.output(Self::Output::Swap(index.clone(), *dropped_index));
                            // Show it here because when it drops, it doesn't leave the target
                            sender.input(Self::Input::Show);
                            true
                        },
                        Err(e) => {
                            println!("Error: dropped value is not a pointer: {e}");
                            false
                        }
                    }
                },

                connect_enter[sender] => move |_d, _x, _y| {
                    sender.input(Self::Input::Hide);
                    gdk::DragAction::COPY
                },

                connect_leave[sender] => move |_| {
                    sender.input(Self::Input::Show);
                },
            },
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(match output {
            Self::Output::Remove(index) => ContentInput::RemoveTask(index),
            Self::Output::MoveUp(index) => ContentInput::MoveTaskUp(index.current_index()),
            Self::Output::MoveDown(index) => ContentInput::MoveTaskDown(index.current_index()),
            Self::Output::Swap(index_1, index_2) => ContentInput::Swap(index_1.current_index(), index_2.current_index()),
        })
    }

    fn init_model(
        task: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self { task, visible_child_name: "row_content" }
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
            Self::Input::Hide => {
                self.visible_child_name = "row_placeholder";
            }
            Self::Input::Show => {
                self.visible_child_name = "row_content";
            }
        }
    }
}
