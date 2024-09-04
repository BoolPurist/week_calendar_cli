app_name := "week_calendar" 
linux_gnu_target := "x86_64-unknown-linux-gnu"
relase_folder := "release"
tar_file_name := relase_folder / app_name + "_" + linux_gnu_target + ".gz.tar"
checksum := relase_folder / "checksum_sha256.txt"

ci: format_check test lint spelling generate_manual
  
# Runs static analyses to improve code quality
lint:
    cargo clippy -- -Dwarnings

# Checks if code is formatted correctly 
format_check:
    cargo fmt --check --all

test: 
  cargo test --all
  
spelling:
  typos
  
generate_manual: 
  cargo xtask

local-install:
  cargo install --path . --force

local-release: 
  cargo build --target {{ linux_gnu_target }} --release
  rm -fr {{ relase_folder }}
  mkdir {{ relase_folder }}
  tar --create --gzip --file {{ tar_file_name }} README.md CHANGELOG.md LICENSE-MIT LICENSE-APACHE Examples.md -C ./target/release/ {{ app_name }}
  sha256sum {{ tar_file_name }} > {{ checksum }}
  sd 'release/' '' {{ checksum }}
