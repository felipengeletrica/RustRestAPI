extern crate postgres;
extern crate barrel;

use postgres::Client;
use barrel::migration::Migration;
use barrel::types;
use barrel::backend::Pg;

pub trait EdgesMigration {
    fn new() -> Self;
    fn run(&self,pg_client:&mut Client) -> Result<u64, postgres::Error>;
}

pub struct CreateTableEdgesMigration {}
impl EdgesMigration for CreateTableEdgesMigration {
    fn new() -> Self {
        CreateTableEdgesMigration {}
    }
    
    fn run(&self,pg_client:&mut Client) -> Result<u64, postgres::Error> {
        let mut m = Migration::new();
        m.create_table("edges", |t| {
            t.add_column("id", types::uuid().primary(true));
            t.add_column("desc", types::text());
            t.add_column("url", types::text());
        });
        let news_table = m.make::<Pg>().to_owned();
        println!("Table {} will be created",news_table);
    
        pg_client.execute(&news_table[..],&[])
    }
} 

pub struct AddEdgeRecordsMigration {}
impl EdgesMigration for AddEdgeRecordsMigration {
    fn new() -> Self {
        AddEdgeRecordsMigration {}
    }
    
    fn run(&self,pg_client:&mut Client) -> Result<u64, postgres::Error> {
        pg_client.execute("INSERT INTO edges VALUES 
        (uuid_in(md5(random()::text || clock_timestamp()::text)::cstring),$1,$2)",
                &[&"Edge South Brazil - RS POA CLARO",&"172.188.1.235"])
    }
} 