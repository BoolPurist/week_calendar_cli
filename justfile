app_name := "week_calendar" 
linux_gnu_target := "x86_64-unknown-linux-gnu"
release_folder := "release"
tar_file_name := release_folder / app_name + "_" + linux_gnu_target + ".gz.tar"
checksum := release_folder / "checksum_sha256.txt"

# Executes all commands which should be before any push or pull request
check_all: ci spelling

# These steps should be executed within a CI.
# Note: have to keep spelling outside the CI check.
# Reason => Typos is not locatable by just even under the usage of the official action of typos
ci: format_check test lint
  
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
  cargo xtask generate-manual
  
build_readme: generate_manual
  cargo xtask build-readme

local-install:
  cargo install --path . --force

local-release: 
  cargo build --target {{ linux_gnu_target }} --release
  rm -fr {{ release_folder }}
  mkdir {{ release_folder }}
  tar --create --gzip --file {{ tar_file_name }} README.md CHANGELOG.md LICENSE-MIT LICENSE-APACHE Examples.md -C ./target/release/ {{ app_name }}
  sha256sum {{ tar_file_name }} > {{ checksum }}
  sd 'release/' '' {{ checksum }}
