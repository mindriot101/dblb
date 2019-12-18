use dblb::LoadBalancer;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

fn main() {
    env_logger::init();

    let m1 = SqliteConnectionManager::file("a.db");
    let m2 = SqliteConnectionManager::file("b.db");
    let m3 = SqliteConnectionManager::file("c.db");

    let lb = LoadBalancer::new().add(m1).add(m2).add(m3).build().unwrap();
    let pool = r2d2::Pool::new(lb).unwrap();

    let conn = pool.get().unwrap();

    conn.execute("drop table if exists foo", params![]).unwrap();
}
