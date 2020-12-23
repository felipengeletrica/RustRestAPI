extern crate postgres;

use postgres::Client;
use postgres::tls::NoTls;

mod migrations;
use migrations::*;

fn main() -> Result<(), postgres::Error> {
    let mut client = Client::connect("postgres://admin:admin@127.0.0.1:5432/mydb", NoTls).unwrap();
    
    let mut migrations = Vec::new();
    migrations.push(CreateTableEdgesMigration::new().run(&mut client));
    migrations.push(AddEdgeRecordsMigration::new().run(&mut client));
    
    for result in migrations.iter() {
        match result{
            Ok(changes) => print!("Migration Success: {} \n",changes),
            Err(error)  => print!("Migration Failure: {} \n",error),
        };
    }
    
    Ok(())
}