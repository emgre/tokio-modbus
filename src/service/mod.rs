#[cfg(feature = "tcp")]
pub mod tcp;
#[cfg(feature = "rtu")]
pub mod rtu;

/// An synchronous function from Request to a Response
#[cfg(feature = "sync")]
pub trait SyncService {
    /// Requests handled by the service.
    type Request;
    /// Responses given by the service.
    type Response;
    /// Errors produced by the service.
    type Error;
    /// Process the request and return the response synchronously.
    fn call(&mut self, Self::Request) -> Result<Self::Response, Self::Error>;
}
