use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let version = option_env!("CARGO_PKG_VERSION").unwrap();
    let authors = option_env!("CARGO_PKG_AUTHORS").unwrap();
    let version_path = "resource/version.rc";
    let mut version_file = File::create(version_path)?;
    let version_with_args = format!(
        "1 VERSIONINFO
        FILEVERSION {}
        FILEOS 0x4
        FILETYPE 0x1
        {{
            BLOCK \"StringFileInfo\"
            {{
                BLOCK \"040904b0\"
                {{
                    VALUE \"CompanyName\", \"{authors}\"
                    VALUE \"FileDescription\", \"FeelTheBasa\"
                    VALUE \"FileVersion\", \"{version}\"
                    VALUE \"InternalName\", \"feel_the_basa\"
                    VALUE \"LegalCopyright\", \"Copyright 2021 Acamol. All rights reserved.\"
                    VALUE \"ProductName\", \"FeelTheBasa\"
                    VALUE \"OriginalFilename\", \"feel_the_basa.exe\"
                    VALUE \"ProductVersion\", \"{version}\"
                    VALUE \"CompanyShortName\", \"Acamol\"
                    VALUE \"ProductShortName\", \"FeelTheBasa\"
                }}
            }}

            BLOCK \"VarFileInfo\"
            {{
                VALUE \"Translation\", 0x0409 0x04B0
            }}
        }}",
        version.replace(".", ","), authors = authors, version = version);

    write!(version_file, "{}", version_with_args)?;
    embed_resource::compile("resource/icon.rc");
    embed_resource::compile("resource/version.rc");
    std::io::Result::Ok(())
}
