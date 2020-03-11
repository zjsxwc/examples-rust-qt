#![windows_subsystem = "windows"]

use qt_core::{QBox, QString, SlotOfBool};
use qt_ui_tools::QUiLoader;
use qt_widgets::{QApplication, QCheckBox, QLabel, QWidget};

struct Form {
    _widget: QBox<QWidget>,
}

impl Form {
    fn new() -> Form {
        unsafe {
            let form_data = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/form1.ui"));
            let ui_loader = QUiLoader::new_0a();
            let widget = ui_loader.load_bytes(form_data);
            assert!(!widget.is_null(), "invalid ui file");
            widget.show();

            let check_box = widget
                .find_child_q_object_1a(&QString::from_std_str("checkBox"))
                .as_ref()
                .expect("child not found")
                .dynamic_cast::<QCheckBox>()
                .expect("widget type mismatch");

            let label = widget
                .find_child_q_object_1a(&QString::from_std_str("label"))
                .as_ref()
                .expect("child not found")
                .dynamic_cast::<QLabel>()
                .expect("widget type mismatch");

            check_box
                .toggled()
                .connect(&SlotOfBool::new(&widget, move |checked| {
                    let text = if checked { "Checked!" } else { "Unchecked!" };
                    label.set_text(&QString::from_std_str(text));
                }));

            Form { _widget: widget }
        }
    }
}

fn main() {
    QApplication::init(|_| unsafe {
        let _form = Form::new();
        QApplication::exec()
    })
}