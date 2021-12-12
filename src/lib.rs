mod dive;
mod sonar;

pub(crate) type Error = Box<dyn std::error::Error>;

/// Load in file as string
pub(crate) fn load_input(name: &str) -> String {
    let file = format!("data/{}", name);
    std::fs::read_to_string(&file).expect("could not read file")
}
