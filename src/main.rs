const APP_NAME: &str = "QuantumVault";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("========================================");
    println!("          {} CLI", APP_NAME);
    println!("========================================");

    println!("Application : {}", APP_NAME);
    println!("Version     : {}", VERSION);
    println!("Status      : Initializing...");

    println!("\nQuantumVault CLI is starting successfully.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_name_is_correct() {
        assert_eq!(APP_NAME, "QuantumVault");
    }

    #[test]
    fn application_version_is_not_empty() {
        assert!(!VERSION.is_empty());
    }
}
