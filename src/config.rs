use super::database::executor::DBExecutor;

pub struct State {
    pub db: actix::Addr<DBExecutor>,
}
