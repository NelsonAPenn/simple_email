extern crate openssl;
extern crate base64;

use openssl::ssl::{SslMethod, SslConnector};
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;
use base64::encode;
use crate::Email;

trait Stream: Read + Write {}
impl<T> Stream for T where T: Read + Write {}

pub struct SmtpServer
{
    pub mail_host: String,
    pub port: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub supports_ssl: bool
}


impl SmtpServer
{
    pub fn send(&self, email: Email) -> Result<(), ()>
    {
        let stream = TcpStream::connect(format!("{}:{}", &self.mail_host, &self.port)).map_err(|_e| () )?;
        let mut stream = if self.supports_ssl
        {
            let unbuilt_connector = SslConnector::builder(SslMethod::tls()).map_err(|_e| () )?;
            let stream = unbuilt_connector.build().connect(&format!("{}", &self.mail_host)[..], stream).map_err(|_e| ())?;
            Box::new(stream) as Box<dyn Stream>
        }
        else
        {
            Box::new(stream) as Box<dyn Stream>
        };

        let mut messages:Vec<String> = vec![
            format!("EHLO {}\r\n", &self.mail_host)
        ];

        if let Some(username) = &self.username
        {
            if let Some(password) = &self.password
            {
                messages.push(format!("AUTH PLAIN {}\r\n", encode(format!("\0{}\0{}", &username, &password))));
            }
        }
            
        messages.push( format!("MAIL FROM:<{}>\r\n", &email.from) );

        for victim in &email.to
        {
            messages.push(format!("RCPT TO:<{}>\r\n", victim));
        }

        messages.push( String::from("DATA\r\n") );
        messages.push( format!("From: {}\r\nSubject: {}\r\n\r\n{}\r\n.\r\n", &email.from, &email.subject, &email.body) );

        let messages = messages; // change to immutable

        // read connection message
        let mut buffer = [0; 256];

        match stream.read(&mut buffer)
        {
            Ok(_) => {
                println!("{}", from_utf8(&buffer).unwrap());
            },
            Err(_e) => {
                return Err(())
            }
        }

        buffer = [0; 256];

        for msg in &messages
        {
            stream.write(msg.as_bytes()).unwrap();

            match stream.read(&mut buffer)
            {
                Ok(_) => {
                    println!("{}", from_utf8(&buffer).unwrap());
                },
                Err(_e) => {
                    return Err(());
                }
            }

            buffer = [0; 256];


        }
        Ok(())
    }
}
