#![windows_subsystem = "windows"]
/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/


extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;
use std::cell::Cell;


#[derive(Default, NwgUi)]
pub struct FeelTheBasaApp {
    #[nwg_control(size: (350, 180), position: (300, 300), title: "Feel the Basa")]
    #[nwg_events( OnWindowClose: [FeelTheBasaApp::exit])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Dec:")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    dec_label: nwg::Label,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0)]
    dec_edit: nwg::TextInput,

    #[nwg_control(text: "Hex:")]
    #[nwg_layout_item(layout: grid, row: 0, col: 1)]
    hex_label: nwg::Label,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 1, col: 1)]
    hex_edit: nwg::TextInput,

    #[nwg_control(text: "ASCII:")]
    #[nwg_layout_item(layout: grid, row: 2, col: 0)]
    ascii_label: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 3, col: 0)]
    ascii_edit: nwg::TextInput,

    #[nwg_control(text: "IP:")]
    #[nwg_layout_item(layout: grid, row: 2, col: 1)]
    ip_label: nwg::Label,

    #[nwg_control(text: "0.0.0.0")]
    #[nwg_events( OnTextInput: [FeelTheBasaApp::ip_change(SELF, CTRL)])]
    #[nwg_layout_item(layout: grid, row: 3, col: 1)]
    ip_edit: nwg::TextInput,

    #[nwg_control(text: "Bin:")]
    #[nwg_layout_item(layout: grid, row: 4, col: 0, col_span: 2)]
    bin_label: nwg::Label,

    #[nwg_control(text: "0", limit: 64)]
    #[nwg_layout_item(layout: grid, row: 5, col: 0, col_span: 2)]
    #[nwg_events( OnTextInput: [FeelTheBasaApp::bin_change(SELF, CTRL)])]
    bin_edit: nwg::TextInput,

    #[nwg_control(text: "IOCTL")]
    #[nwg_layout_item(layout: grid, row: 6, col: 0, col_span: 2)]
    ioctl_edit: nwg::TextInput,

    lock: Cell<bool>,
}

impl FeelTheBasaApp {

    fn ip_change(&self, ti: &nwg::TextInput) {
        if self.lock.get() {
            return;
        }
        self.lock.set(true);

        let to = ti.text();
        let t: Vec<&str> = to.split(".").collect();
        if t.len() != 4 || t.iter().any(|x| x.is_empty() || x.chars().any(|y| !y.is_numeric()) || x.parse::<i32>().unwrap() > 255) {
            self.lock.set(false);
            return;
        }

        let ip: [u8; 4] = [t[3].parse().unwrap(), t[2].parse().unwrap(), t[1].parse().unwrap(), t[0].parse().unwrap()];
        let dec = i32::from_ne_bytes(ip);
        self.dec_edit.set_text(&format!("{}", dec));
        self.bin_edit.set_text(&format!("{:b}", dec));
        self.hex_edit.set_text(&format!("{:X}", dec));
        self.ascii_edit.set_text(&ip.iter().filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());

        self.lock.set(false);
    }

    fn bin_change(&self, ti: &nwg::TextInput) {
        if self.lock.get() {
            return;
        }
        self.lock.set(true);

        if ti.text().chars().any(|x| x != '0' && x != '1') {
            self.lock.set(false);
            return;
        }

        if let Ok(r) = isize::from_str_radix(&ti.text(), 2) {
            self.dec_edit.set_text(&format!("{}", r));
            self.hex_edit.set_text(&format!("{:X}", r));
            let x = r.to_be_bytes();
            self.ip_edit.set_text(&format!("{}.{}.{}.{}", x[4], x[5], x[6], x[7]));
            self.ascii_edit.set_text(&x.iter().filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());
        }

        self.lock.set(false);
    }
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let mut font = nwg::Font::default();
    nwg::Font::builder().family("Segoe UI").size(18).build(&mut font).expect("Failed to set default font");
    nwg::Font::set_global_default(Some(font));
    let _app = FeelTheBasaApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}