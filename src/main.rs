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
    #[nwg_control(size: (350, 200), position: (300, 300), title: "Feel the Basa")]
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

    #[nwg_control(text: "0", limit: 32)]
    #[nwg_layout_item(layout: grid, row: 3, col: 0, col_span: 4)]
    #[nwg_events( OnTextInput: [FeelTheBasaApp::bin_change(SELF, CTRL)], OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    bin_edit: nwg::TextInput,

    #[nwg_control(text: "IOCTL", h_align: nwg::HTextAlign::Center)]
    #[nwg_layout_item(layout: grid, row: 5, col: 0, col_span: 4)]
    ioctl_label: nwg::Label,

    #[nwg_control(text: "Number:")]
    #[nwg_layout_item(layout: grid, row: 6, col: 0, col_span: 1)]
    ioctl_number_label: nwg::Label,

    #[nwg_control(text: "Family:")]
    #[nwg_layout_item(layout: grid, row: 6, col: 1, col_span: 1)]
    ioctl_type_label: nwg::Label,

    #[nwg_control(text: "Size:")]
    #[nwg_layout_item(layout: grid, row: 6, col: 2, col_span: 1)]
    ioctl_size_label: nwg::Label,

    #[nwg_control(text: "Dir:")]
    #[nwg_layout_item(layout: grid, row: 6, col: 3, col_span: 1)]
    ioctl_dir_label: nwg::Label,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 7, col: 0, col_span: 1)]
    #[nwg_events( OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    ioctl_number_edit: nwg::TextInput,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 7, col: 1, col_span: 1)]
    #[nwg_events( OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    ioctl_family_edit: nwg::TextInput,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 7, col: 2, col_span: 1)]
    #[nwg_events( OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    ioctl_size_edit: nwg::TextInput,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 7, col: 3, col_span: 1)]
    #[nwg_events( OnTextInput: [FeelTheBasaApp::dir_change], OnKeyRelease: [FeelTheBasaApp::window_key_press(SELF, EVT_DATA)] )]
    ioctl_dir_edit: nwg::TextInput,

    lock: Cell<bool>,
}

impl FeelTheBasaApp {
    const NRBITS: u32 = 8;
    const TYPEBITS: u32 = 8;
    const SIZEBITS: u32 = 14;
    const DIRBITS: u32 = 2;
    const NRMASK: u32 = (1 << FeelTheBasaApp::NRBITS) - 1;
    const TYPEMASK: u32 = (1 << FeelTheBasaApp::TYPEBITS) - 1;
    const SIZEMASK: u32 = (1 << FeelTheBasaApp::SIZEBITS) - 1;
    const DIRMASK: u32 = (1 << FeelTheBasaApp::DIRBITS) -1;
    const NRSHIFT: u32 = 0;
    const TYPESHIFT: u32 = FeelTheBasaApp::NRSHIFT + FeelTheBasaApp::NRBITS;
    const SIZESHIFT: u32 = FeelTheBasaApp::TYPEBITS + FeelTheBasaApp::TYPEBITS;
    const DIRSHIFT: u32 = FeelTheBasaApp::SIZESHIFT + FeelTheBasaApp::SIZEBITS;

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
        self.dec_edit.set_text(&format!("{}", dec));
        self.bin_edit.set_text(&format!("{:b}", dec));
        self.hex_edit.set_text(&format!("{:X}", dec));
        self.text_edit.set_text(&ip.iter().filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());
        self.ioctl_family_edit.set_text(&format!("{}", ((dec >> FeelTheBasaApp::TYPESHIFT) & FeelTheBasaApp::TYPEMASK) as u8 as char));
        self.ioctl_size_edit.set_text(&format!("{}", (dec >> FeelTheBasaApp::SIZESHIFT) & FeelTheBasaApp::SIZEMASK));
        let dir = match (dec >> FeelTheBasaApp::DIRSHIFT) & FeelTheBasaApp::DIRMASK {
            0b0 => "None0",
            0b1 => "None1",
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

        if let Ok(r) = u32::from_str_radix(&ti.text(), 2) {
            self.dec_edit.set_text(&format!("{}", r));
            self.hex_edit.set_text(&format!("{:X}", r));
            let x = r.to_be_bytes();
            self.ip_edit.set_text(&format!("{}.{}.{}.{}", x[0], x[1], x[2], x[3]));
            self.text_edit.set_text(&x.iter().filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());
            self.ioctl_number_edit.set_text(&format!("{}", (r >> FeelTheBasaApp::NRSHIFT) & FeelTheBasaApp::NRMASK));
            self.ioctl_family_edit.set_text(&format!("{}", ((r >> FeelTheBasaApp::TYPESHIFT) & FeelTheBasaApp::TYPEMASK) as u8 as char));
            self.ioctl_size_edit.set_text(&format!("{}", (r >> FeelTheBasaApp::SIZESHIFT) & FeelTheBasaApp::SIZEMASK));
            let dir = match (r >> FeelTheBasaApp::DIRSHIFT) & FeelTheBasaApp::DIRMASK {
                0b0 => "None0",
                0b1 => "None1",
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

        if let Ok(r) = u32::from_str_radix(s, 16) {
            self.dec_edit.set_text(&format!("{}", r));
            let x = r.to_be_bytes();
            self.ip_edit.set_text(&format!("{}.{}.{}.{}", x[0], x[1], x[2], x[3]));
            self.text_edit.set_text(&x.iter().filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());
            self.bin_edit.set_text(&format!("{:b}", r));
            self.ioctl_number_edit.set_text(&format!("{}", (r >> FeelTheBasaApp::NRSHIFT) & FeelTheBasaApp::NRMASK));
            self.ioctl_family_edit.set_text(&format!("{}", ((r >> FeelTheBasaApp::TYPESHIFT) & FeelTheBasaApp::TYPEMASK) as u8 as char));
            self.ioctl_size_edit.set_text(&format!("{}", (r >> FeelTheBasaApp::SIZESHIFT) & FeelTheBasaApp::SIZEMASK));
            let dir = match (r >> FeelTheBasaApp::DIRSHIFT) & FeelTheBasaApp::DIRMASK {
                0b0 => "None0",
                0b1 => "None1",
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

        if let Ok(r) = u32::from_str_radix(s, 10) {
            let x = r.to_be_bytes();
            self.ip_edit.set_text(&format!("{}.{}.{}.{}", x[0], x[1], x[2], x[3]));
            self.text_edit.set_text(&x.iter().filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());
            self.bin_edit.set_text(&format!("{:b}", r));
            self.hex_edit.set_text(&format!("{:X}", r));
            self.ioctl_number_edit.set_text(&format!("{}", (r >> FeelTheBasaApp::NRSHIFT) & FeelTheBasaApp::NRMASK));
            self.ioctl_family_edit.set_text(&format!("{}", ((r >> FeelTheBasaApp::TYPESHIFT) & FeelTheBasaApp::TYPEMASK) as u8 as char));
            self.ioctl_size_edit.set_text(&format!("{}", (r >> FeelTheBasaApp::SIZESHIFT) & FeelTheBasaApp::SIZEMASK));
            let dir = match (r >> FeelTheBasaApp::DIRSHIFT) & FeelTheBasaApp::DIRMASK {
                0b0 => "None0",
                0b1 => "None1",
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
        if s.len() > 4 {
            return;
        }
        self.lock.set(true);

        let mut bytes = s.bytes().collect::<VecDeque<_>>();
        while bytes.len() < 4 {
            bytes.push_front(0u8)
        }
        let b = [bytes[3], bytes[2], bytes[1], bytes[0]];
        let dec = u32::from_ne_bytes(b);
        self.dec_edit.set_text(&format!("{}", dec));
        self.ip_edit.set_text(&format!("{}.{}.{}.{}", b[3], b[2], b[1], b[0]));
        self.bin_edit.set_text(&format!("{:b}", dec));
        self.hex_edit.set_text(&format!("{:X}", dec));
        self.ioctl_number_edit.set_text(&format!("{}", (dec >> FeelTheBasaApp::NRSHIFT) & FeelTheBasaApp::NRMASK));
        self.ioctl_family_edit.set_text(&format!("{}", ((dec >> FeelTheBasaApp::TYPESHIFT) & FeelTheBasaApp::TYPEMASK) as u8 as char));
        self.ioctl_size_edit.set_text(&format!("{}", (dec >> FeelTheBasaApp::SIZESHIFT) & FeelTheBasaApp::SIZEMASK));
        let dir = match (dec >> FeelTheBasaApp::DIRSHIFT) & FeelTheBasaApp::DIRMASK {
            0b0 => "None0",
            0b1 => "None1",
            0b10 => "Read",
            0b11 => "Write",
            _ => "ERROR"
        };
        self.ioctl_dir_edit.set_text(dir);

        self.lock.set(false);
    }

    fn dir_change(&self) {
        if self.lock.get() {
            return;
        }

        let s: &str = &self.ioctl_dir_edit.text();
        let dir_r = match &s.to_uppercase()[..] {
            "NONE0" | "NONE" => 0b0,
            "NONE1" => 0b1,
            "READ" => 0b10,
            "WRITE" => 0b11,
            _ => return
        };
        let dirbits = dir_r << FeelTheBasaApp::DIRSHIFT;
        let mask = u32::MAX >> FeelTheBasaApp::DIRBITS;

        self.lock.set(true);
        let dec = self.dec_edit.text().parse::<u32>().unwrap() & mask | dirbits;
        let x = dec.to_be_bytes();
        self.ip_edit.set_text(&format!("{}.{}.{}.{}", x[0], x[1], x[2], x[3]));
        self.dec_edit.set_text(&format!("{}", dec));
        self.bin_edit.set_text(&format!("{:b}", dec));
        self.hex_edit.set_text(&format!("{:X}", dec));
        self.text_edit.set_text(&x.iter().filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());
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