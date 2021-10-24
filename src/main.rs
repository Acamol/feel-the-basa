#![windows_subsystem = "windows"]

mod icon;
mod nwg_extension;
mod bytes;
mod error;

use native_windows_gui as nwg;
use native_windows_derive as nwd;
use nwd::NwgUi;
use nwg::NativeUi;
use std::cell::Cell;
use std::net::{Ipv4Addr, Ipv6Addr};

use tinyget;
use regex::Regex;
use semver::{VersionReq, Version};
use paste;

use nwg_extension::tooltip::OneArgRegister;
use bytes::Int32or64or128;
use bytes::_128bit::{ParseTo128Bit as _, ToStr as _, ToBinStr as _};
use bytes::BitWidth as BitWidth;
use error::Error;


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
    None,
}

macro_rules! on_bit_selected_fn {
    ($bits:ident) => {
        paste::paste! {
            fn [<on $bits bit_selected>](&self) {
                self.bit_menu_check_only_one(BitWidth::[<$bits BIT>]);
                self.bit_width.set(BitWidth::[<$bits BIT>]);
                let signed = self.signed_menu_item.checked();
                let dec_str = self.dec_edit.text();
                if let Ok(bytes) = dec_str.as_str().parse_to_128bit(signed, BitWidth::[<$bits BIT>]) {
                    self.refresh_value_by_dec(&bytes, TextInputType::None);
                }
            }
        }
    };
}

macro_rules! refresh_ioctl_field {
    ($self:ident, $initial:expr, $type:ident) => {
        paste::paste! {
            let bits = $initial << FtBA::[<$type SHIFT>];
            let mask = !(FtBA::[<$type MASK>] << FtBA::[<$type SHIFT>]);

            let signed = $self.signed_menu_item.checked();
            let dec = if let Ok(r) = bytes::_32bit::parse_as_u32_decimal(signed, &$self.dec_edit.text()) {
                r & mask | bits
            } else {
                return
            };

            let mut u = Int32or64or128 { _u128: 0 };
            u._u32 = dec;
            $self.refresh_value_by_dec(&unsafe {u._u128.to_ne_bytes()}, TextInputType::IoctlDir);
        }
    };
}

type FtBA = FeelTheBasaApp;

#[derive(Default, NwgUi)]
pub struct FeelTheBasaApp {
    #[nwg_resource(source_bin: Some(&icon::ICON))]
    icon: nwg::Icon,

    #[nwg_control(size: (445, 225), position: (300, 300), title: "Feel the Basa", icon: Some(&data.icon))]
    #[nwg_events( OnWindowClose: [FtBA::exit] )]
    window: nwg::Window,

    #[nwg_control(text: "&File")]
    window_menu: nwg::Menu,

    #[nwg_control(text: "Close", parent: window_menu)]
    #[nwg_events( OnMenuItemSelected: [FtBA::exit])]
    exit_menu_item: nwg::MenuItem,

    #[nwg_control(text: "&Mode")]
    mode_menu: nwg::Menu,

    #[nwg_control(text: "Signed", parent: mode_menu)]
    #[nwg_events( OnMenuItemSelected: [FtBA::on_signed_selected]) ]
    signed_menu_item: nwg::MenuItem,

    #[nwg_control(parent: mode_menu)]
    mode_separator: nwg::MenuSeparator,

    #[nwg_control(text: "32-bit", parent: mode_menu, check: true)]
    #[nwg_events( OnMenuItemSelected: [FtBA::on_32bit_selected]) ]
    _32bit_menu_item: nwg::MenuItem,

    #[nwg_control(text: "64-bit", parent: mode_menu)]
    #[nwg_events( OnMenuItemSelected: [FtBA::on_64bit_selected]) ]
    _64bit_menu_item: nwg::MenuItem,

    #[nwg_control(text: "128-bit", parent: mode_menu)]
    #[nwg_events( OnMenuItemSelected: [FtBA::on_128bit_selected]) ]
    _128bit_menu_item: nwg::MenuItem,

    #[nwg_control(text: "&Help")]
    help_menu: nwg::Menu,

    #[nwg_control(text: "Hotkeys", parent: help_menu)]
    #[nwg_events( OnMenuItemSelected: [FtBA::on_hotkeys])]
    hotkeys_menu_item: nwg::MenuItem,

    #[nwg_control(text: "Check for Updates", parent: help_menu)]
    #[nwg_events( OnMenuItemSelected: [FtBA::on_check_for_updates])]
    check_for_updates_menu_item: nwg::MenuItem,

    #[nwg_control(parent: help_menu)]
    help_separator: nwg::MenuSeparator,

    #[nwg_control(text: "About", parent: help_menu)]
    #[nwg_events( OnMenuItemSelected: [FtBA::about])]
    about_menu_item: nwg::MenuItem,

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
    #[nwg_events( OnTextInput: [FtBA::on_dec_change] )]
    dec_edit: nwg::TextInput,

    #[nwg_control(text: "0", limit: 8)]
    #[nwg_layout_item(layout: grid, row: 1, col: 1)]
    #[nwg_events( OnTextInput: [FtBA::on_hex_change] )]
    hex_edit: nwg::TextInput,

    #[nwg_control(limit: 4)]
    #[nwg_layout_item(layout: grid, row: 1, col: 2)]
    #[nwg_events( OnTextInput: [FtBA::on_text_change] )]
    text_edit: nwg::TextInput,

    #[nwg_control(text: "0.0.0.0", limit: 15)]
    #[nwg_events( OnTextInput: [FtBA::on_ip_change] )]
    #[nwg_layout_item(layout: grid, row: 1, col: 3)]
    ip_edit: nwg::TextInput,

    #[nwg_control(text: "Bin:")]
    #[nwg_layout_item(layout: grid, row: 2, col: 0, col_span: 1)]
    bin_label: nwg::Label,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 3, col: 0, col_span: 4)]
    #[nwg_events( OnInit: [FtBA::exit], OnTextInput: [FtBA::on_bin_change], OnKeyRelease: [FtBA::on_bin_key_press(SELF, EVT_DATA)] )]
    bin_edit: nwg::TextInput,

    #[nwg_control(one_arg_register: (&data.bin_edit, "Press Enter to split into bytes"))]
    bin_edit_tooltip: nwg::Tooltip,

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

    #[nwg_control(text: "Access Mode:")]
    #[nwg_layout_item(layout: grid, row: 6, col: 3, col_span: 1)]
    ioctl_dir_label: nwg::Label,

    #[nwg_control(text: "0", limit: 3)]
    #[nwg_layout_item(layout: grid, row: 7, col: 0, col_span: 1)]
    #[nwg_events( OnTextInput: [FtBA::on_number_change] )]
    ioctl_number_edit: nwg::TextInput,

    #[nwg_control()]
    #[nwg_layout_item(layout: grid, row: 7, col: 1, col_span: 1)]
    #[nwg_events( OnTextInput: [FtBA::on_family_change] )]
    ioctl_family_edit: nwg::TextInput,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: grid, row: 7, col: 2, col_span: 1)]
    #[nwg_events( OnTextInput: [FtBA::on_size_change] )]
    ioctl_size_edit: nwg::TextInput,

    #[nwg_control(text: "None")]
    #[nwg_layout_item(layout: grid, row: 7, col: 3, col_span: 1)]
    #[nwg_events( OnTextInput: [FtBA::on_dir_change] )]
    ioctl_dir_edit: nwg::TextInput,

    lock: Cell<bool>,
    bit_width: Cell<BitWidth>,
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

    fn partition_bin_to_bytes(s: &str) -> String {
        let mut partitioned = String::with_capacity(s.len());
        let mut filtered_s = s.chars()
            .filter(|&c| c != ' ')
            .skip_while(|&c| c == '0')
            .peekable();
        let len = filtered_s.clone().count();
        let first_chunk_size = if len % 8 == 0 {8} else {len % 8};
        let it = filtered_s.by_ref();
        partitioned.extend(it.take(first_chunk_size));
        while it.peek().is_some() {
            partitioned.push(' ');
            partitioned.extend(it.take(8));
        }

        partitioned
    }

    fn refresh_value_by_dec(&self, dec: &[u8; 16], tip: TextInputType) {
        union Int32or64or128 {
            _u32: u32,
            _u64: u64,
            _u128: u128
        }

        self.lock.set(true);
        let bw = self.bit_width.get();
        let u = Int32or64or128 { _u128: u128::from_ne_bytes(*dec) };

        if tip != TextInputType::Dec {
            self.dec_edit.set_text(&dec.to_str(self.signed_menu_item.checked(), bw));
        }

        if tip != TextInputType::Bin {
            self.bin_edit.set_text(&Self::partition_bin_to_bytes(&dec.to_bin_str(bw)));
        }

        if tip != TextInputType::Hex {
            let hex = match bw {
                BitWidth::_32BIT =>
                    format!("{:X}", unsafe { u._u32 }),
                BitWidth::_64BIT =>
                    format!("{:X}", unsafe { u._u64 }),
                BitWidth::_128BIT =>
                    format!("{:X}", unsafe { u._u128 }),
            };
            self.hex_edit.set_text(&hex);
        }

        if tip != TextInputType::Text {
            let iter = match bw {
                BitWidth::_32BIT => dec.iter().take(4),
                BitWidth::_64BIT => dec.iter().take(8),
                BitWidth::_128BIT => dec.iter().take(16)
            };
            self.text_edit.set_text(&iter.filter(|&&c| c != 0).map(|&c| c as char).collect::<String>());
        }

        if tip != TextInputType::IP {
            match bw {
                BitWidth::_32BIT => {
                    let mut it = dec.iter().take(4).rev();
                    self.ip_edit.set_text(&format!("{}.{}.{}.{}", it.next().unwrap(), it.next().unwrap(), it.next().unwrap(), it.next().unwrap()));
                },
                BitWidth::_128BIT => {
                        let mut rev = *dec;
                        rev.reverse();
                        let ip = Ipv6Addr::from(rev);
                        self.ip_edit.set_text(&ip.to_string());
                },
                _ => (),
            }
        }

        if bw == BitWidth::_32BIT && tip != TextInputType::IoctlNumber {
            self.ioctl_number_edit.set_text(&format!("{}", (unsafe { u._u32 } >> FtBA::NRSHIFT) & FtBA::NRMASK));
        }

        if bw == BitWidth::_32BIT && tip != TextInputType::IoctlFamily {
            self.ioctl_family_edit.set_text(&format!("{}", ((unsafe { u._u32 } >> FtBA::TYPESHIFT) & FtBA::TYPEMASK) as u8 as char));
        }
        
        if bw == BitWidth::_32BIT && tip != TextInputType::IoctlSize {
            self.ioctl_size_edit.set_text(&format!("{}", (unsafe { u._u32 } >> FtBA::SIZESHIFT) & FtBA::SIZEMASK));
        }

        if bw == BitWidth::_32BIT && tip != TextInputType::IoctlDir {
            let dir = match (unsafe { u._u32 } >> FtBA::DIRSHIFT) & FtBA::DIRMASK {
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

    fn on_ip_change_bw(&self, bw: BitWidth) {
        let s = self.ip_edit.text();
        if self.lock.get() {
            return;
        }

        let dec = match bw {
            BitWidth::_32BIT => {
                let mut bytes = [0u8; 16];
                if let Ok(r) = s.parse::<Ipv4Addr>() {
                    let octets = r.octets();
                    for (i, b) in bytes.iter_mut().take(4).rev().enumerate() {
                        *b = octets[i];
                    }
                } else {
                    return
                }
                bytes
            }
            BitWidth::_64BIT =>
                return,
            BitWidth::_128BIT => {
                if let Ok(r) = s.parse::<Ipv6Addr>() {
                    let mut rev = r.octets();
                    rev.reverse();
                    rev
                } else {
                    return
                }
            }
        };

        self.refresh_value_by_dec(&dec, TextInputType::IP);
    }

    fn on_ip_change(&self) {
        self.on_ip_change_bw(self.bit_width.get())
    }

    fn on_bin_change(&self) {
        if self.lock.get() {
            return;
        }

        let bin_str: String = self.bin_edit.text().chars().filter(|&c| c != ' ').collect();
        if bin_str.chars().any(|x| x != '0' && x != '1') {
            return;
        }

        if let Ok(r) = u128::from_str_radix(&bin_str, 2) {
            self.refresh_value_by_dec(&r.to_ne_bytes(), TextInputType::Bin);
        }
    }

    fn on_hex_change(&self) {
        if self.lock.get() {
            return;
        }

        let s = &self.hex_edit.text().to_uppercase();
        if s.chars().any(|c| (c < '0' || c > '9') && (c < 'A' || c > 'F')) {
            return;
        }

        if let Ok(r) = u128::from_str_radix(s, 16) {
            self.refresh_value_by_dec(&r.to_ne_bytes(), TextInputType::Hex);
        }
    }

    fn on_dec_change(&self) {
        if self.lock.get() {
            return;
        }

        let s = self.dec_edit.text();
        let signed = self.signed_menu_item.checked();
        if let Ok(bytes) = s.as_str().parse_to_128bit(signed, self.bit_width.get()) {
            self.refresh_value_by_dec(&bytes, TextInputType::Dec);
        };
    }

    fn on_text_change(&self) {
        if self.lock.get() {
            return;
        }

        let s = &self.text_edit.text();
        if s.len() == 0 {
            return;
        }

        if s.len() > self.bit_width.get().to_num_bytes() {
            return;
        }

        let mut bytes = [0u8; 16];
        s.bytes().rev().enumerate().for_each(|(i, b)| bytes[i] = b);
        self.refresh_value_by_dec(&bytes, TextInputType::Text);
    }

    fn on_dir_change(&self) {
        if self.lock.get() {
            return;
        }

        if self.bit_width.get() != BitWidth::_32BIT {
            return;
        }

        let s: &str = &self.ioctl_dir_edit.text();
        let dir_r = match s.to_uppercase().as_ref() {
            "N" | "NONE" => 0b0,
            "R" | "READ" => 0b1,
            "W" | "WRITE" => 0b10,
            "READ/WRITE" | "WRITE/READ" | "READ\\WRITE" | "WRITE\\READ"
            | "R/W" | "W/R" | "R\\W" | "W\\R" => 0b11,
            _ => return
        };

        refresh_ioctl_field!(self, dir_r, DIR);
    }

    fn on_number_change(&self) {
        if self.lock.get() {
            return;
        }

        if self.bit_width.get() != BitWidth::_32BIT {
            return;
        }

        let s = &self.ioctl_number_edit.text();
        let number = match s.parse::<u32>() {
            Ok(r @ 0..=255) => r,
            _ => return
        };

        refresh_ioctl_field!(self, number, NR);
    }

    fn on_family_change(&self) {
        if self.lock.get() {
            return;
        }

        if self.bit_width.get() != BitWidth::_32BIT {
            return;
        }

        let s = &self.ioctl_family_edit.text();
        if s.len() != 1 {
            return
        }

        let b = s.chars().next().unwrap() as u32;

        refresh_ioctl_field!(self, b, TYPE);
    }

    fn on_size_change(&self) {
        if self.lock.get() {
            return;
        }

        if self.bit_width.get() != BitWidth::_32BIT {
            return;
        }

        let s = &self.ioctl_size_edit.text();
        let size = match s.parse::<u32>() {
            Ok(r @ 0..=FtBA::SIZEMASK) => r,
            _ => return
        };

        refresh_ioctl_field!(self, size, SIZE);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

    fn about(&self) {
        let p = nwg::MessageParams {
            title: "About",
            content: &format!("Coded by Acamol, inspired by FeelTheBase.\nReach me at gaf@duck.com.\n\nVersion: {}", option_env!("CARGO_PKG_VERSION").unwrap()),
            buttons: nwg::MessageButtons::Ok,
            icons: nwg::MessageIcons::None
        };
        nwg::modal_message(&self.window, &p);
    }

    fn on_hotkeys(&self) {
        let p = nwg::MessageParams {
            title: "Hotkeys",
            content: "Enter (Bin only): partition the input to bytes\nAlt + f: open File menu\nAlt + m: open Mode menu\nAlt + h: open Help menu",
            buttons: nwg::MessageButtons::Ok,
            icons: nwg::MessageIcons::None
        };
        nwg::modal_message(&self.window, &p);
    }

    fn on_bin_key_press(&self, ent_data: &nwg::EventData) {
        match ent_data.on_key() {
            nwg::keys::RETURN => {
                self.lock.set(true);
                self.bin_edit.set_text(&Self::partition_bin_to_bytes(&self.bin_edit.text()));
                self.lock.set(false);
            }
            _ => ()
        }
    }

    fn on_signed_selected_with_result(was_signed: bool, bw: BitWidth, dec_text: &str) -> Result<u128, std::num::ParseIntError> {
        let dec = if was_signed {
            match bw {
                BitWidth::_32BIT =>
                    i32::from_str_radix(&dec_text, 10)? as u128,
                BitWidth::_64BIT =>
                    i64::from_str_radix(&dec_text, 10)? as u128,
                BitWidth::_128BIT =>
                    i128::from_str_radix(&dec_text, 10)? as u128,
            }
        } else {
            match bw {
                BitWidth::_32BIT =>
                    u32::from_str_radix(&dec_text, 10)? as u128,
                BitWidth::_64BIT =>
                    u64::from_str_radix(&dec_text, 10)? as u128,
                BitWidth::_128BIT =>
                    u128::from_str_radix(&dec_text, 10)? as u128,
            }
        };
        Ok(dec)
    }

    fn on_signed_selected(&self) {
        let was_signed = self.signed_menu_item.checked();
        self.signed_menu_item.set_checked(!was_signed);

        if let Ok(dec) = Self::on_signed_selected_with_result(was_signed, self.bit_width.get(), &self.dec_edit.text()) {
            self.refresh_value_by_dec(&dec.to_ne_bytes(), TextInputType::Bin)
        }
    }

    fn bit_menu_check_only_one(&self, bits: BitWidth) {
        self._32bit_menu_item.set_checked(false);
        self._64bit_menu_item.set_checked(false);
        self._128bit_menu_item.set_checked(false);
        self.ioctl_dir_edit.set_readonly(true);
        self.ioctl_family_edit.set_readonly(true);
        self.ioctl_number_edit.set_readonly(true);
        self.ioctl_size_edit.set_readonly(true);

        match bits {
            BitWidth::_32BIT => {
                self._32bit_menu_item.set_checked(true);
                self.ip_edit.set_readonly(false);
                self.ioctl_dir_edit.set_readonly(false);
                self.ioctl_family_edit.set_readonly(false);
                self.ioctl_number_edit.set_readonly(false);
                self.ioctl_size_edit.set_limit(15);
                self.ioctl_size_edit.set_readonly(false);
            },
            BitWidth::_64BIT => {
                self._64bit_menu_item.set_checked(true);
                self.ip_edit.set_readonly(true);
            }
            BitWidth::_128BIT => {
                self._128bit_menu_item.set_checked(true);
                self.ip_edit.set_readonly(false);
                self.ip_edit.set_limit(39);
            }
        };

        self.hex_edit.set_limit(bits.to_num_bytes() * 2);
        self.text_edit.set_limit(bits.to_num_bytes());
    }

    on_bit_selected_fn!(_32);

    on_bit_selected_fn!(_64);

    on_bit_selected_fn!(_128);

    fn on_check_for_updates_result(&self) -> Result<String, Error> {
        let re = Regex::new(r"v(\d+\.\d+\.\d+)").unwrap();
        let version = option_env!("CARGO_PKG_VERSION").unwrap();
        let res = tinyget::get("https://api.github.com/repos/Acamol/feel-the-basa/releases")
            .with_header("User-Agent", "FeelTheBasa")
            .send()?;
        let cap = re.captures(res.as_str()?).ok_or_else(|| Error::RegexError)?;
        let req = VersionReq::parse(&format!("<{}", cap.get(1).ok_or_else(|| Error::ReqwestError)?.as_str()))?;
        let content = if req.matches(&Version::parse(version).unwrap()) {
            "New version is available.\nDownload the latest version from GitHub."
        } else {
            "No updates are available.\nYou are already using the latest version."
        };
        Ok(String::from(content))
    }

    fn on_check_for_updates(&self) {
        let title = "Check for Updates";
        if let Ok(content) = self.on_check_for_updates_result() {
            nwg::modal_info_message(&self.window, title, &content);
        } else {
            nwg::modal_error_message(&self.window, title, "Failed to check for updates.\n\nPlease try again later, or report an issue on GitHub if it persists.");
        }
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
