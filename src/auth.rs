const DEFAULT_NAME: &str = "cassandra";

#[derive(Clone)]
pub enum Auth {
    None(cdrs::authenticators::NoneAuthenticator),
    Some(cdrs::authenticators::StaticPasswordAuthenticator),
}

impl Auth {
    pub fn from_creds(username: Option<String>, password: Option<String>) -> Self {
        if username.is_none() && password.is_none() {
            cdrs::authenticators::NoneAuthenticator.into()
        } else {
            let username = username.unwrap_or_else(|| DEFAULT_NAME.to_owned());
            let password = password.unwrap_or_else(|| DEFAULT_NAME.to_owned());
            cdrs::authenticators::StaticPasswordAuthenticator::new(username, password).into()
        }
    }
}

impl Into<Auth> for cdrs::authenticators::NoneAuthenticator {
    #[inline]
    fn into(self) -> Auth {
        Auth::None(self)
    }
}

impl Into<Auth> for cdrs::authenticators::StaticPasswordAuthenticator {
    #[inline]
    fn into(self) -> Auth {
        Auth::Some(self)
    }
}

impl cdrs::authenticators::Authenticator for Auth {
    #[inline]
    fn get_auth_token(&self) -> cdrs::types::CBytes {
        match self {
            Auth::None(auth) => auth.get_auth_token(),
            Auth::Some(auth) => auth.get_auth_token(),
        }
    }

    #[inline]
    fn get_cassandra_name(&self) -> Option<&str> {
        match self {
            Auth::None(auth) => auth.get_cassandra_name(),
            Auth::Some(auth) => auth.get_cassandra_name(),
        }
    }
}
