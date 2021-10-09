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
use std::collections::VecDeque;


#[derive(Default, NwgUi)]
pub struct FeelTheBasaApp {
    #[nwg_control(size: (350, 180), position: (300, 300), title: "Feel the Basa")]
    #[nwg_events( OnWindowClose: [FeelTheBasaApp::exit], OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Dec:")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    dec_label: nwg::Label,

    #[nwg_control(text: "Hex:")]
    #[nwg_layout_item(layout: grid, row: 0, col: 1)]
    hex_label: nwg::Label,

    #[nwg_control(text: "Text:")]
    #[nwg_layout_item(layout: grid, row: 0, col: 2)]
    text_label: nwg::Label,

    #[nwg_control(text: "IP:")]
    #[nwg_layout_item(layout: grid, row: 0, col: 3)]
    ip_label: nwg::Label,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0)]
    #[nwg_events( OnTextInput: [FeelTheBasaApp::dec_change], OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    dec_edit: nwg::TextInput,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 1, col: 1)]
    #[nwg_events( OnTextInput: [FeelTheBasaApp::hex_change], OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    hex_edit: nwg::TextInput,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: grid, row: 1, col: 2)]
    #[nwg_events( OnTextInput: [FeelTheBasaApp::text_change], OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    text_edit: nwg::TextInput,

    #[nwg_control(text: "0.0.0.0")]
    #[nwg_events( OnTextInput: [FeelTheBasaApp::ip_change(SELF, CTRL)], OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    #[nwg_layout_item(layout: grid, row: 1, col: 3)]
    ip_edit: nwg::TextInput,

    #[nwg_control(text: "Bin:")]
    #[nwg_layout_item(layout: grid, row: 2, col: 0, col_span: 1)]
    bin_label: nwg::Label,

    #[nwg_control(text: "0", limit: 64)]
    #[nwg_layout_item(layout: grid, row: 3, col: 0, col_span: 4)]
    #[nwg_events( OnTextInput: [FeelTheBasaApp::bin_change(SELF, CTRL)], OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    bin_edit: nwg::TextInput,

    #[nwg_control(text: "Number:")]
    #[nwg_layout_item(layout: grid, row: 4, col: 0, col_span: 1)]
    ioctl_number_label: nwg::Label,

    #[nwg_control(text: "Family:")]
    #[nwg_layout_item(layout: grid, row: 4, col: 1, col_span: 1)]
    ioctl_type_label: nwg::Label,

    #[nwg_control(text: "Size:")]
    #[nwg_layout_item(layout: grid, row: 4, col: 2, col_span: 1)]
    ioctl_size_label: nwg::Label,

    #[nwg_control(text: "Dir:")]
    #[nwg_layout_item(layout: grid, row: 4, col: 3, col_span: 1)]
    ioctl_dir_label: nwg::Label,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 5, col: 0, col_span: 1)]
    #[nwg_events( OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    ioctl_number_edit: nwg::TextInput,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 5, col: 1, col_span: 1)]
    #[nwg_events( OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    ioctl_family_edit: nwg::TextInput,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 5, col: 2, col_span: 1)]
    #[nwg_events( OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    ioctl_size_edit: nwg::TextInput,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 5, col: 3, col_span: 1)]
    #[nwg_events( OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    ioctl_dir_edit: nwg::TextInput,
    lock: Cell<bool>,
}

impl FeelTheBasaApp {
    const NRBITS: i32 = 8;
    const TYPEBITS: i32 = 8;
    const SIZEBITS: i32 = 14;
    const DIRBITS: i32 = 2;
    const NRMASK: i32 = (1 << FeelTheBasaApp::NRBITS) - 1;
    const TYPEMASK: i32 = (1 << FeelTheBasaApp::TYPEBITS) - 1;
    const SIZEMASK: i32 = (1 << FeelTheBasaApp::SIZEBITS) - 1;
    const DIRMASK: i32 = (1 << FeelTheBasaApp::DIRBITS) -1;
    const NRSHIFT: i32 = 0;
    const TYPESHIFT: i32 = FeelTheBasaApp::NRSHIFT + FeelTheBasaApp::NRBITS;
    const SIZESHIFT: i32 = FeelTheBasaApp::TYPEBITS + FeelTheBasaApp::TYPEBITS;
    const DIRSHIFT: i32 = FeelTheBasaApp::SIZESHIFT + FeelTheBasaApp::SIZEBITS;

    fn ip_change(&self, ti: &nwg::TextInput) {
        if self.lock.get() {
            return;
        }

        let to = ti.text();
        let t: Vec<&str> = to.split(".").collect();
        if t.len() != 4 || t.iter().any(|x| x.is_empty() || x.chars().any(|y| !y.is_numeric()) || x.parse::<i32>().unwrap() > 255) {
            return;
        }
        self.lock.set(true);

        let ip: [u8; 4] = [t[3].parse().unwrap(), t[2].parse().unwrap(), t[1].parse().unwrap(), t[0].parse().unwrap()];
        let dec = u32::from_ne_bytes(ip);
        let dec32 = dec as i32;
        self.dec_edit.set_text(&format!("{}", dec));
        self.bin_edit.set_text(&format!("{:b}", dec));
        self.hex_edit.set_text(&format!("{:X}", dec));
        self.text_edit.set_text(&ip.iter().filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());
        self.ioctl_family_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::TYPESHIFT) & FeelTheBasaApp::TYPEMASK));
        self.ioctl_size_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::SIZESHIFT) & FeelTheBasaApp::SIZEMASK));
        let dir = match (dec32 >> FeelTheBasaApp::DIRSHIFT) & FeelTheBasaApp::DIRMASK {
            0b0..=0b1 => "None",
            0b10 => "Read",
            0b11 => "Write",
            _ => "ERROR"
        };
        self.ioctl_dir_edit.set_text(dir);

        self.lock.set(false);
    }

    fn bin_change(&self, ti: &nwg::TextInput) {
        if self.lock.get() {
            return;
        }

        if ti.text().chars().any(|x| x != '0' && x != '1') {
            return;
        }
        self.lock.set(true);

        if let Ok(r) = isize::from_str_radix(&ti.text(), 2) {
            let dec32 = r as i32;
            self.dec_edit.set_text(&format!("{}", r));
            self.hex_edit.set_text(&format!("{:X}", r));
            let x = r.to_be_bytes();
            self.ip_edit.set_text(&format!("{}.{}.{}.{}", x[4], x[5], x[6], x[7]));
            self.text_edit.set_text(&x.iter().filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());
            self.ioctl_number_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::NRSHIFT) & FeelTheBasaApp::NRMASK));
            self.ioctl_family_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::TYPESHIFT) & FeelTheBasaApp::TYPEMASK));
            self.ioctl_size_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::SIZESHIFT) & FeelTheBasaApp::SIZEMASK));
            let dir = match (dec32 >> FeelTheBasaApp::DIRSHIFT) & FeelTheBasaApp::DIRMASK {
                0b0..=0b1 => "None",
                0b10 => "Read",
                0b11 => "Write",
                _ => "ERROR"
            };
            self.ioctl_dir_edit.set_text(dir);
        }

        self.lock.set(false);
    }

    fn hex_change(&self) {
        if self.lock.get() {
            return;
        }

        let s = &self.hex_edit.text().to_uppercase();
        if s.chars().any(|c| (c < '0' || c > '9') && (c < 'A' || c > 'F')) {
            return;
        }
        self.lock.set(true);

        if let Ok(r) = isize::from_str_radix(s, 16) {
            let dec32 = r as i32;
            self.dec_edit.set_text(&format!("{}", r));
            let x = r.to_be_bytes();
            self.ip_edit.set_text(&format!("{}.{}.{}.{}", x[4], x[5], x[6], x[7]));
            self.text_edit.set_text(&x.iter().filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());
            self.bin_edit.set_text(&format!("{:b}", r));
            self.ioctl_number_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::NRSHIFT) & FeelTheBasaApp::NRMASK));
            self.ioctl_number_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::NRSHIFT) & FeelTheBasaApp::NRMASK));
            self.ioctl_family_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::TYPESHIFT) & FeelTheBasaApp::TYPEMASK));
            self.ioctl_size_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::SIZESHIFT) & FeelTheBasaApp::SIZEMASK));
            let dir = match (dec32 >> FeelTheBasaApp::DIRSHIFT) & FeelTheBasaApp::DIRMASK {
                0b0..=0b1 => "None",
                0b10 => "Read",
                0b11 => "Write",
                _ => "ERROR"
            };
            self.ioctl_dir_edit.set_text(dir);
        }
        self.lock.set(false);
    }

    fn dec_change(&self) {
        if self.lock.get() {
            return;
        }

        let s = &self.dec_edit.text();
        if s.chars().any(|c| c < '0' || c > '9') {
            return;
        }
        self.lock.set(true);

        if let Ok(r) = i64::from_str_radix(s, 10) {
            let x = r.to_be_bytes();
            let dec32 = r as i32;
            self.ip_edit.set_text(&format!("{}.{}.{}.{}", x[4], x[5], x[6], x[7]));
            self.text_edit.set_text(&x.iter().filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());
            self.bin_edit.set_text(&format!("{:b}", r));
            self.hex_edit.set_text(&format!("{:X}", r));
            self.ioctl_number_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::NRSHIFT) & FeelTheBasaApp::NRMASK));
            self.ioctl_family_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::TYPESHIFT) & FeelTheBasaApp::TYPEMASK));
            self.ioctl_size_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::SIZESHIFT) & FeelTheBasaApp::SIZEMASK));
            let dir = match (dec32 >> FeelTheBasaApp::DIRSHIFT) & FeelTheBasaApp::DIRMASK {
                0b0..=0b1 => "None",
                0b10 => "Read",
                0b11 => "Write",
                _ => "ERROR"
            };
            self.ioctl_dir_edit.set_text(dir);
        }
        self.lock.set(false);
    }

    fn text_change(&self) {
        if self.lock.get() {
            return;
        }

        let s = &self.text_edit.text();
        if s.len() > 8 {
            return;
        }
        self.lock.set(true);

        let mut bytes = s.bytes().collect::<VecDeque<_>>();
        while bytes.len() < 8 {
            bytes.push_front(0u8)
        }
        let b = [bytes[7], bytes[6], bytes[5], bytes[4], bytes[3], bytes[2], bytes[1], bytes[0]];
        let dec = i64::from_ne_bytes(b);
        let dec32 = dec as i32;
        self.dec_edit.set_text(&format!("{}", dec as u64));
        self.ip_edit.set_text(&format!("{}.{}.{}.{}", b[3], b[2], b[1], b[0]));
        self.bin_edit.set_text(&format!("{:b}", dec));
        self.hex_edit.set_text(&format!("{:X}", dec));
        self.ioctl_number_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::NRSHIFT) & FeelTheBasaApp::NRMASK));
        self.ioctl_family_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::TYPESHIFT) & FeelTheBasaApp::TYPEMASK));
        self.ioctl_size_edit.set_text(&format!("{}", (dec32 >> FeelTheBasaApp::SIZESHIFT) & FeelTheBasaApp::SIZEMASK));
        let dir = match (dec32 >> FeelTheBasaApp::DIRSHIFT) & FeelTheBasaApp::DIRMASK {
            0b0..=0b1 => "None",
            0b10 => "Read",
            0b11 => "Write",
            _ => "ERROR"
        };
        self.ioctl_dir_edit.set_text(dir);

        self.lock.set(false);
    }
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn window_key_press(&self, ent_data: &nwg::EventData) {
        match ent_data.on_key() {
            nwg::keys::ESCAPE => self.window.close(),
            _ => ()
        }
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