use crate::auth::Auth;

pub struct Shell {
    session: cdrs::cluster::session::Session<cdrs::load_balancing::RoundRobin<cdrs::cluster::TcpConnectionPool<Auth>>>
}

impl Shell {
    pub fn new(host: &str, auth: Auth) -> Result<Self, cdrs::error::Error> {
        let node = cdrs::cluster::NodeTcpConfigBuilder::new(host, auth).build();
        let config = cdrs::cluster::ClusterTcpConfig(vec![node]);
        let load_balancing = cdrs::load_balancing::RoundRobin::new();
        let session = cdrs::cluster::session::new(&config, load_balancing)?;

        Ok(Self {
            session,
        })
    }

    pub fn interactive(&self) -> bool {
        println!("Interactive shell is not implemented yet... :(");
        false
    }

    pub fn execute(&self, statement: &str) -> bool {
        use cdrs::query::QueryExecutor;

        match self.session.query(statement) {
            Ok(_) => true,
            Err(error) => {
                eprintln!("Failed to execute query: '{}'. Error: {}", statement, error);
                false
            }
        }
    }
}
