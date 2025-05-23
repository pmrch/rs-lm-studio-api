use macron::{ Display, From, Error };

// The result alias
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// The error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    #[display = "{0}"]
    Io(std::io::Error),

    #[from]
    #[display = "{0}"]
    Reqwest(reqwest::Error),

    #[from]
    #[display = "{0}"]
    Json(serde_json::Error),
}
