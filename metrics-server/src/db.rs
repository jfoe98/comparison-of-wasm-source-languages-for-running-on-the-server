use crate::models::Metric;
use mysql::*;
use mysql::prelude::*;

#[derive(Clone)]
pub struct MySqlDb {
    user: String,
    password: String,
    host: String,
}

impl MySqlDb {
    pub fn new(user: String, password: String, host: String) -> MySqlDb {
        MySqlDb {user, password, host}
    }

    pub fn add_metric(self, metric: Metric) -> std::result::Result<(), &'static str> {
        println!("Add new metric to database. Name: '{0}', Context: '{1}', Language: '{2}', Value: {3} ", metric.name, metric.context, metric.language, metric.value);

        let mut conn = self.get_db_connection();

        conn.exec_drop(r"INSERT INTO Metrics (language, name, context, value)
            VALUES (:language, :name, :context, :value)",
            params! {
                "language" => metric.language,
                "name" => metric.name,
                "context" => metric.context,
                "value" => metric.value
            }).unwrap();

        Ok(())
    }

    fn get_db_connection(self) -> PooledConn {
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some(self.host))
            .user(Some(self.user))
            .pass(Some(self.password))
            .db_name(Some("metrics"));

        let pool = Pool::new(opts).unwrap();

        pool.get_conn().unwrap()
    }
}