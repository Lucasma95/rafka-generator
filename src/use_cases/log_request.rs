use crate::http::contracts::post_message::Request;
use std::error::Error;

pub trait RequestLogger {
    fn log(&self, request: &Request) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone, Copy, Default)]
pub struct RequestLoggerImpl;

impl RequestLoggerImpl {
    pub fn new() -> RequestLoggerImpl {
        RequestLoggerImpl{}
    }
}

impl RequestLogger for RequestLoggerImpl {
    fn log(&self, request: &Request) -> Result<(), Box<dyn Error>> {
        println!("Message: {}", request.message);
        println!("Queue: {}", request.queue_name);
        println!("Producer: {}", request.producer);
        Ok(())
    }
}
