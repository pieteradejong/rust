use std::str::FromStr;

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::GET),
            "PUT" => Ok(Self::GET),
            "DELETE" => Ok(Self::GET),
            "HEAD" => Ok(Self::GET),
            "CONNECT" => Ok(Self::GET),
            "OPTIONS" => Ok(Self::GET),
            "TRACE" => Ok(Self::GET),
            "PATCH" => Ok(Self::GET),
            _  => Ok(MethodError)
        }
    }
}

pub stuct MethodError = {};
