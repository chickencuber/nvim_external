use std::{env::args, path::{PathBuf, Path}, process::Command};

macro_rules! arguments {
    ($name:ident, $usage:expr, $($value:ident: $type:ty => $index:expr),+$(,)?) => {
        #[derive(Debug)]
        struct $name {
            $(
                $value: $type
            ),+
        } 
        impl $name {
            pub fn from(vec: Vec<String>) -> Self {
                Self {
                    $(
                        $value: vec.get($index).expect(format!("usage: {}", $usage).as_str()).into()
                    ),+
                }
            }
        }
    };
}

arguments!{
    Data,
    "nvim_external [server] [file]",
    server: String => 1,
    file: PathBuf => 2,
}

fn main() {
    let data = Data::from(args().collect());
    let server = PathBuf::from(format!("//./pipe/nvim_{}.pipe", &data.server));
    if server_exists(&server) {
        open_file(&server, &data);
    } else {
        open_nvim(&server, &data);
    }
}

fn open_file(server: &PathBuf, data: &Data) {
    let status = Command::new("nvim")
        .arg("--server")
        .arg(server.display().to_string())
        .arg("--remote")
        .arg(data.file.display().to_string())
        .status()
        .expect("Failed to open file");

    if !status.success() {
        eprintln!("Command failed with status: {:?}", status);
    }
}

fn server_exists(server: &PathBuf) -> bool {
    return Path::exists(server);
}

fn open_nvim(server: &PathBuf, data: &Data) {
    let status = Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("nvim")
        .arg("--listen")
        .arg(server.display().to_string())
        .arg(data.file.display().to_string())
        .status()
        .expect("failed to start nvim");

    if !status.success() {
        eprintln!("Command failed with status: {:?}", status);
    }
}

