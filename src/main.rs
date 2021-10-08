#![windows_subsystem = "windows"]
/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct FeelTheBasaApp {
    #[nwg_control(size: (300, 150), position: (300, 300), title: "Feel the Basa")]
    #[nwg_events( OnWindowClose: [FeelTheBasaApp::exit])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "DEC")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    dec_edit: nwg::TextInput,

    #[nwg_control(text: "HEX")]
    #[nwg_layout_item(layout: grid, row: 0, col: 1)]
    hex_edit: nwg::TextInput,

    #[nwg_control(text: "ASCII")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0)]
    ascii_edit: nwg::TextInput,

    #[nwg_control(text: "IP")]
    #[nwg_layout_item(layout: grid, row: 1, col: 1)]
    ip_edit: nwg::TextInput,

    #[nwg_control(text: "BIN")]
    #[nwg_layout_item(layout: grid, row: 2, col: 0, col_span: 2)]
    #[nwg_events( OnTextInput: [FeelTheBasaApp::bin_change(SELF, CTRL)])]
    bin_edit: nwg::TextInput,

}

impl FeelTheBasaApp {

    fn bin_change(&self, ti: &nwg::TextInput) {
        if let Ok(r) = isize::from_str_radix(&ti.text(), 2) {
            self.dec_edit.set_text(&format!("{}", r));
            self.hex_edit.set_text(&format!("{:X}", r));
            let x = r.to_be_bytes();
            self.ip_edit.set_text(&format!("{}.{}.{}.{}", x[4], x[5], x[6], x[7]));
            self.ascii_edit.set_text(&format!("{}", x[7] as char))
        }
    }
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = FeelTheBasaApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}