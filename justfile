app_name := "week_calendar" 
linux_gnu_target := "x86_64-unknown-linux-gnu"
relase_folder := "release"
tar_file_name := relase_folder / app_name + "_" + linux_gnu_target + ".gz.tar"
checksum := relase_folder / "checksum_sha256.txt"

local-install:
  cargo install --path . --force

local-release: 
  cargo build --target {{ linux_gnu_target }} --release
  rm -fr {{ relase_folder }}
  mkdir {{ relase_folder }}
  tar --create --gzip --file {{ tar_file_name }} README.md CHANGELOG.md LICENSE-MIT LICENSE-APACHE Examples.md -C ./target/release/ {{ app_name }}
  sha256sum {{ tar_file_name }} > {{ checksum }}
  sd 'release/' '' {{ checksum }}
