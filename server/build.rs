use std::path::Path;

fn main() {
    dotenv_build::output(dotenv_build::Config {
        filename: Path::new(".env"),
        recursive_search: false,
        fail_if_missing_dotenv: true,
    })
    .unwrap();
}
