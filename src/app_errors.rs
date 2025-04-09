use app_core::ErrorCode;

pub mod err_kind {
    pub const AUTH: &str = "auth";
    pub const VALIDATION: &str = "validation";
    pub const SERVICE: &str = "external_service";
}

#[derive(Debug)]
pub enum ErrCodes {
    // Trade core errors
    //ETC001,

    E1234,
    E2000,
}

impl ErrorCode for ErrCodes {
    fn code(&self) -> &'static str {
        match self {
            ErrCodes::E1234 => "E1234",
            ErrCodes::E2000 => "E2000",
        }
    }

    fn format(&self) -> &'static str {
        match self {
            ErrCodes::E1234 => "Invalid value for {field}",
            ErrCodes::E2000 => "Missing required config: {key}",
        }
    }

    fn kind(&self) -> &'static str {
        match self {
            ErrCodes::E1234 => err_kind::VALIDATION,
            ErrCodes::E2000 => "config",
        }
    }
}
