use crate::config::InexorClientConfig;

pub struct InexorClientConfigBuilder(InexorClientConfig);

impl InexorClientConfigBuilder {
    pub fn new() -> Self {
        Self(InexorClientConfig::default())
    }

    pub fn hostname(mut self, hostname: String) -> Self {
        self.0.hostname = hostname;
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.0.port = port;
        self
    }

    pub fn secure(mut self, secure: bool) -> Self {
        self.0.secure = secure;
        self
    }

    pub fn endpoint(mut self, endpoint: String) -> Self {
        self.0.endpoint = endpoint;
        self
    }

    pub fn user_agent(mut self, user_agent: String) -> Self {
        self.0.user_agent = user_agent;
        self
    }

    pub fn bearer(mut self, bearer: String) -> Self {
        self.0.bearer = Some(bearer);
        self
    }

    pub fn build(self) -> InexorClientConfig {
        self.0
    }
}

impl Default for InexorClientConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
