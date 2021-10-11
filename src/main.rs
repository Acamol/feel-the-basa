#![windows_subsystem = "windows"]

use native_windows_gui as nwg;
use native_windows_derive as nwd;
use nwd::NwgUi;
use nwg::NativeUi;
use std::cell::Cell;
use std::collections::VecDeque;

mod icon;


#[derive(PartialEq)]
enum TextInputType {
    Dec,
    Hex,
    Bin,
    Text,
    IP,
    IoctlFamily,
    IoctlNumber,
    IoctlDir,
    IoctlSize,
}

type FtBA = FeelTheBasaApp;

#[derive(Default, NwgUi)]
pub struct FeelTheBasaApp {

    #[nwg_resource(source_bin: Some(&icon::ICON))]
    icon: nwg::Icon,

    #[nwg_control(size: (445, 225), position: (300, 300), title: &format!("Feel the Basa by Acamol ({})", option_env!("CARGO_PKG_VERSION").unwrap()), icon: Some(&data.icon))]
    #[nwg_events( OnWindowClose: [FtBA::exit], OnKeyRelease: [FtBA::window_key_press(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_control(text: "File")]
    window_menu: nwg::Menu,

    #[nwg_control(text: "Signed", parent: window_menu)]
    #[nwg_events( OnMenuItemSelected: [FtBA::signed_check]) ]
    signed_menu_item: nwg::MenuItem,

    #[nwg_control(text: "About", parent: window_menu)]
    #[nwg_events( OnMenuItemSelected: [FtBA::about])]
    about_menu_item: nwg::MenuItem,

    #[nwg_control(text: "Close", parent: window_menu)]
    #[nwg_events( OnMenuItemSelected: [FtBA::exit])]
    exit_menu_item: nwg::MenuItem,

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
    #[nwg_events( OnTextInput: [FtBA::dec_change], OnKeyRelease: [FtBA::window_key_press(SELF, EVT_DATA)] )]
    dec_edit: nwg::TextInput,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 1, col: 1)]
    #[nwg_events( OnTextInput: [FtBA::hex_change], OnKeyRelease: [FtBA::window_key_press(SELF, EVT_DATA)] )]
    hex_edit: nwg::TextInput,

    #[nwg_control(limit: 4)]
    #[nwg_layout_item(layout: grid, row: 1, col: 2)]
    #[nwg_events( OnTextInput: [FtBA::text_change], OnKeyRelease: [FtBA::window_key_press(SELF, EVT_DATA)] )]
    text_edit: nwg::TextInput,

    #[nwg_control(text: "0.0.0.0", limit: 15)]
    #[nwg_events( OnTextInput: [FtBA::ip_change], OnKeyRelease: [FtBA::window_key_press(SELF, EVT_DATA)] )]
    #[nwg_layout_item(layout: grid, row: 1, col: 3)]
    ip_edit: nwg::TextInput,

    #[nwg_control(text: "Bin:")]
    #[nwg_layout_item(layout: grid, row: 2, col: 0, col_span: 1)]
    bin_label: nwg::Label,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 3, col: 0, col_span: 4)]
    #[nwg_events( OnTextInput: [FtBA::bin_change], OnKeyRelease: [FtBA::window_key_press_on_bin(SELF, EVT_DATA)] )]
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

    #[nwg_control(text: "0", limit: 3)]
    #[nwg_layout_item(layout: grid, row: 7, col: 0, col_span: 1)]
    #[nwg_events( OnTextInput: [FtBA::number_change], OnKeyRelease: [FtBA::window_key_press(SELF, EVT_DATA)] )]
    ioctl_number_edit: nwg::TextInput,

    #[nwg_control()]
    #[nwg_layout_item(layout: grid, row: 7, col: 1, col_span: 1)]
    #[nwg_events( OnTextInput: [FtBA::family_change], OnKeyRelease: [FtBA::window_key_press(SELF, EVT_DATA)] )]
    ioctl_family_edit: nwg::TextInput,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 7, col: 2, col_span: 1)]
    #[nwg_events( OnTextInput: [FtBA::size_change], OnKeyRelease: [FtBA::window_key_press(SELF, EVT_DATA)] )]
    ioctl_size_edit: nwg::TextInput,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 7, col: 3, col_span: 1)]
    #[nwg_events( OnTextInput: [FtBA::dir_change], OnKeyRelease: [FtBA::window_key_press(SELF, EVT_DATA)] )]
    ioctl_dir_edit: nwg::TextInput,

    lock: Cell<bool>,
}

fn partition_bin_to_bytes(s: &str) -> String {
    let mut partitioned = String::with_capacity(s.len());
    let mut count = 0;
    let t = s.chars().rev().filter(|&c| c != ' ');
    for c in t {
        if count > 0 && count % 8 == 0 {
            partitioned.push(' ');
        }
        partitioned.push(c);
        count += 1;
    }

    partitioned.chars().rev().collect()
}

impl FeelTheBasaApp {
    const NRBITS: u32 = 8;
    const TYPEBITS: u32 = 8;
    const SIZEBITS: u32 = 14;
    const DIRBITS: u32 = 2;
    const NRMASK: u32 = (1 << FtBA::NRBITS) - 1;
    const TYPEMASK: u32 = (1 << FtBA::TYPEBITS) - 1;
    const SIZEMASK: u32 = (1 << FtBA::SIZEBITS) - 1;
    const DIRMASK: u32 = (1 << FtBA::DIRBITS) -1;
    const NRSHIFT: u32 = 0;
    const TYPESHIFT: u32 = FtBA::NRSHIFT + FtBA::NRBITS;
    const SIZESHIFT: u32 = FtBA::TYPEBITS + FtBA::TYPEBITS;
    const DIRSHIFT: u32 = FtBA::SIZESHIFT + FtBA::SIZEBITS;

    fn refresh_value_by_dec(&self, dec: u32, tip: TextInputType) {
        self.lock.set(true);
        let bytes = dec.to_be_bytes();

        if tip != TextInputType::Dec {
            let msg;
            if self.signed_menu_item.checked() {
                msg = format!("{}", dec as i32);
            } else {
                msg = format!("{}", dec);
            }
            self.dec_edit.set_text(&msg);
        }

        if tip != TextInputType::Bin {
            self.bin_edit.set_text(&partition_bin_to_bytes(&format!("{:b}", dec)));
        }

        if tip != TextInputType::Hex {
            self.hex_edit.set_text(&format!("{:X}", dec));
        }

        if tip != TextInputType::Text {
            self.text_edit.set_text(&bytes.iter().filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());
        }

        if tip != TextInputType::IP {
            self.ip_edit.set_text(&format!("{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3]));
        }

        if tip != TextInputType::IoctlNumber {
            self.ioctl_number_edit.set_text(&format!("{}", (dec >> FtBA::NRSHIFT) & FtBA::NRMASK));
        }

        if tip != TextInputType::IoctlFamily {
            self.ioctl_family_edit.set_text(&format!("{}", ((dec >> FtBA::TYPESHIFT) & FtBA::TYPEMASK) as u8 as char));
        }
        
        if tip != TextInputType::IoctlSize {
            self.ioctl_size_edit.set_text(&format!("{}", (dec >> FtBA::SIZESHIFT) & FtBA::SIZEMASK));
        }

        if tip != TextInputType::IoctlDir {
            let dir = match (dec >> FtBA::DIRSHIFT) & FtBA::DIRMASK {
                0b0 => "None",
                0b1 => "Read",
                0b10 => "Write",
                0b11 => "Read/Write",
                _ => "ERROR"
            };
            self.ioctl_dir_edit.set_text(dir);
        }

        self.lock.set(false);
    }

    fn ip_change(&self) {
        if self.lock.get() {
            return;
        }

        let to = self.ip_edit.text();
        let t: Vec<&str> = to.split(".").collect();
        if t.len() != 4 || t.iter().any(|x| x.is_empty() || x.chars().any(|y| !y.is_numeric()) || x.parse::<i32>().unwrap() > 255) {
            return;
        }

        let ip: [u8; 4] = [t[3].parse().unwrap(), t[2].parse().unwrap(), t[1].parse().unwrap(), t[0].parse().unwrap()];
        let dec = u32::from_ne_bytes(ip);
        self.refresh_value_by_dec(dec, TextInputType::IP);
    }

    fn bin_change(&self) {
        if self.lock.get() {
            return;
        }

        let bin_str: String = self.bin_edit.text().chars().filter(|&c| c != ' ').collect();
        if bin_str.chars().any(|x| x != '0' && x != '1') {
            return;
        }

        if let Ok(r) = u32::from_str_radix(&bin_str, 2) {
            self.refresh_value_by_dec(r, TextInputType::Bin);
        }
    }

    fn hex_change(&self) {
        if self.lock.get() {
            return;
        }

        let s = &self.hex_edit.text().to_uppercase();
        if s.chars().any(|c| (c < '0' || c > '9') && (c < 'A' || c > 'F')) {
            return;
        }

        if let Ok(r) = u32::from_str_radix(s, 16) {
            self.refresh_value_by_dec(r, TextInputType::Hex);
        }
    }

    fn dec_change(&self) {
        if self.lock.get() {
            return;
        }

        let s = self.dec_edit.text();
        let signed = self.signed_menu_item.checked();
        let dec = if signed {
            match i32::from_str_radix(&s, 10) {
                Ok(r) => r as u32,
                _ => return
            }
        } else {
            match u32::from_str_radix(&s, 10) {
                Ok(r) => r,
                _ => return
            }
        };
        self.refresh_value_by_dec(dec, TextInputType::Dec);
    }

    fn text_change(&self) {
        if self.lock.get() {
            return;
        }

        let s = &self.text_edit.text();
        if s.len() > 4 {
            return;
        }

        let mut bytes = s.bytes().collect::<VecDeque<_>>();
        while bytes.len() < 4 {
            bytes.push_front(0u8)
        }
        let b = [bytes[3], bytes[2], bytes[1], bytes[0]];
        let dec = u32::from_ne_bytes(b);
        self.refresh_value_by_dec(dec, TextInputType::Text);
    }

    fn dir_change(&self) {
        if self.lock.get() {
            return;
        }

        let s: &str = &self.ioctl_dir_edit.text();
        let dir_r = match &s.to_uppercase()[..] {
            "NONE" => 0b0,
            "READ" => 0b1,
            "WRITE" => 0b10,
            "READ/WRITE" | "WRITE/READ" => 0b11,
            _ => return
        };
        let dirbits = dir_r << FtBA::DIRSHIFT;
        let mask = !(FtBA::DIRMASK << FtBA::DIRSHIFT);

        let dec = self.dec_edit.text().parse::<u32>().unwrap() & mask | dirbits;
        self.refresh_value_by_dec(dec, TextInputType::IoctlDir);
    }
    
    fn number_change(&self) {
        if self.lock.get() {
            return;
        }

        let s = &self.ioctl_number_edit.text();
        let number = match s.parse::<u32>() {
            Ok(r @ 0..=255) => r,
            _ => return
        };
        let nrbits = number << FtBA::NRSHIFT;
        let mask = !(FtBA::NRMASK << FtBA::NRSHIFT);

        let dec = self.dec_edit.text().parse::<u32>().unwrap() & mask | nrbits;
        self.refresh_value_by_dec(dec, TextInputType::IoctlNumber);
    }

    fn family_change(&self) {
        if self.lock.get() {
            return;
        }

        let s = &self.ioctl_family_edit.text();
        if s.len() != 1 {
            return
        }

        let b = s.chars().next().unwrap() as u32;

        let typebits = b << FtBA::TYPESHIFT;
        let mask = !(FtBA::TYPEMASK << FtBA::TYPESHIFT);

        let dec = self.dec_edit.text().parse::<u32>().unwrap() & mask | typebits;
        self.refresh_value_by_dec(dec, TextInputType::IoctlFamily);
    }

    fn size_change(&self) {
        if self.lock.get() {
            return;
        }

        let s = &self.ioctl_size_edit.text();
        let size = match s.parse::<u32>() {
            Ok(r @ 0..=FtBA::SIZEMASK) => r,
            _ => return
        };

        let sizebits = size << FtBA::SIZESHIFT;
        let mask = !(FtBA::SIZEMASK << FtBA::SIZESHIFT);

        let dec = self.dec_edit.text().parse::<u32>().unwrap() & mask | sizebits;
        self.refresh_value_by_dec(dec, TextInputType::IoctlSize);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn about(&self) {
        nwg::modal_info_message(&self.window, "Feel the Basa", &format!("Coded by Acamol, inspired by FeelTheBase.\nReach me at gaf@duck.com.\n\nVersion {}", option_env!("CARGO_PKG_VERSION").unwrap()));
    }

    fn window_key_press(&self, ent_data: &nwg::EventData) {
        match ent_data.on_key() {
            nwg::keys::ESCAPE => self.window.close(),
            _ => ()
        }
    }

    fn window_key_press_on_bin(&self, ent_data: &nwg::EventData) {
        match ent_data.on_key() {
            nwg::keys::ESCAPE => self.window.close(),
            nwg::keys::RETURN => {
                self.lock.set(true);
                self.bin_edit.set_text(&partition_bin_to_bytes(&self.bin_edit.text()));
                self.lock.set(false);
            }
            _ => ()
        }
    }

    fn signed_check(&self) {
        let checked = self.signed_menu_item.checked();
        self.signed_menu_item.set_checked(!checked);

        let dec = if checked {
            match i32::from_str_radix(&self.dec_edit.text(), 10) {
                Ok(r) => r as u32,
                _ => return
            }
        } else {
            match u32::from_str_radix(&self.dec_edit.text(), 10) {
                Ok(r) => r,
                _ => return
            }
        };
        self.refresh_value_by_dec(dec, TextInputType::Bin)
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let mut font = nwg::Font::default();
    nwg::Font::builder().family("Segoe UI").size(18).build(&mut font).expect("Failed to set default font");
    nwg::Font::set_global_default(Some(font));
    let _app = FtBA::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
