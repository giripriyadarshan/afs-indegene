#[derive(Debug)]
pub enum ServerErrors {
    ZohoError,
    VaultError,
    InternalServerError,
}

impl From<hyper::Error> for ServerErrors {
    fn from(_: hyper::Error) -> Self {
        ServerErrors::InternalServerError
    }
}

impl std::fmt::Display for ServerErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerErrors::ZohoError => write!(f, "Zoho Error"),
            ServerErrors::VaultError => write!(f, "Vault Error"),
            ServerErrors::InternalServerError => write!(f, "Internal Server Error"),
        }
    }
}

impl From<ServerErrors> for axum::body::Body {
    fn from(err: ServerErrors) -> Self {
        axum::body::Body::from(format!("{}", err))
    }
}
