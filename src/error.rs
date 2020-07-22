use {
    crate::algorithm::algorithm::ServerSelectionError,
    std::fmt
};

#[derive(Debug, Clone)]
pub struct CookieError;

impl fmt::Display for CookieError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request didn't contain any cookies.")
    }
}

#[derive(Debug, Clone)]
pub enum ServerMappingError {
    Cookie(CookieError),
    ServerSelection(ServerSelectionError),
}

impl fmt::Display for ServerMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ServerMappingError::Cookie(ref e) => e.fmt(f),
            ServerMappingError::ServerSelection(ref e) => e.fmt(f),
        }
    }
}

impl From<CookieError> for ServerMappingError {
    fn from(e: CookieError) -> Self {
        ServerMappingError::Cookie(e)
    }
}

impl From<ServerSelectionError> for ServerMappingError {
    fn from(e: ServerSelectionError) -> Self {
        ServerMappingError::ServerSelection(e)
    }
}
