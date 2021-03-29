pub mod smtp;

pub struct Email
{
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub body: String
}