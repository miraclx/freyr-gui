use qt_ui_tools::ui_form;
use qt_widgets::{qt_core::QBox, QApplication, QWidget};
use std::rc::Rc;

#[ui_form("../ui/main.ui")]
struct MainWindow {
    widget: QBox<QWidget>,
}

struct FreyrGUI {
    main_window: MainWindow,
}

impl FreyrGUI {
    fn new() -> Rc<Self> {
        let this = Rc::new(FreyrGUI {
            main_window: MainWindow::load(),
        });
        unsafe { this.init() }
        this
    }
    unsafe fn init(self: &Rc<Self>) {}
    fn show(self: &Rc<Self>) {
        unsafe { self.main_window.widget.show() }
    }
}

fn main() {
    QApplication::init(|_| {
        let freyr_gui = FreyrGUI::new();
        freyr_gui.show();
        unsafe { QApplication::exec() }
    })
}
