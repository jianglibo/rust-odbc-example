#[macro_use]
extern crate failure;

#[macro_use]
extern crate clap;

extern crate env_logger;
extern crate odbc;
use log::*;
use clap::App;
use clap::ArgMatches;
use std::env;

fn main() {
    if let Err(err) = main_1() {
        error!("counter error: {:?}", err);
    }
}

/// we change mini_app_conf value here.
fn main_1() -> Result<(), failure::Error> {
    println!("current_dir: {:?}", env::current_dir()?);
    let mut log4rs_file = env::current_dir()?.join("log4rs.yaml");
    let exec_path = std::env::current_exe()?;
    let exec_dir = exec_path.parent().expect("exec_dir failed.");
    env::set_current_dir(&exec_dir)?;

    if exec_dir.join("log4rs.yaml").exists() {
        log4rs_file = exec_dir.join("log4rs.yaml");
    }

    log4rs::init_file(log4rs_file, Default::default())
        .unwrap_or_else(|_| panic!("fail to find log4rs.yaml in {:?}", exec_dir));
    let yml = load_yaml!("17_yaml.yml");
    let app = App::from_yaml(yml);

    let m: ArgMatches = app.get_matches();

    match m.subcommand() {
        ("driver-datasource", Some(__sub_matches)) => {
            print_drivers_and_datasources()?;
        }
        (_, _) => {
            unimplemented!();
        }
    }
    Ok(())
}

fn print_drivers_and_datasources() -> odbc::Result<()> {
    let mut env = odbc::create_environment_v3().map_err(|e| e.unwrap())?;
    println!("Driver list:");
    for driver_info in env.drivers()? {
        println!("\nDriver Name: {}", driver_info.description);
        for (key, value) in driver_info.attributes {
            println!("    {}={}", key, value);
        }
    }

    println!("\nDataSource list:");
    for ds in env.data_sources()? {
        println!("    {}\n    {}\n\n", ds.server_name, ds.driver);
    }
    Ok(())
}



// extern crate odbc;
// // Use this crate and set environmet variable RUST_LOG=odbc to see ODBC warnings
// extern crate env_logger;
// use odbc::*;
// use std::io;
// use odbc_safe::AutocommitOn;

// fn main() {

//     env_logger::init();

//     match connect() {
//         Ok(()) => println!("Success"),
//         Err(diag) => println!("Error: {}", diag),
//     }
// }

// fn connect() -> std::result::Result<(), DiagnosticRecord> {

//     let env = create_environment_v3().map_err(|e| e.unwrap())?;

//     let mut buffer = String::new();
//     println!("Please enter connection string: ");
//     io::stdin().read_line(&mut buffer).unwrap();

//     let conn = env.connect_with_connection_string(&buffer)?;
//     execute_statement(&conn)
// }

// // https://docs.oracle.com/en/database/oracle/oracle-database/19/adfns/odbc-driver.html#GUID-7931EDFB-7A70-4BBE-903E-8A2BB09DBE9D

// fn execute_statement<'env>(conn: &Connection<'env, AutocommitOn>) -> Result<()> {
//     let stmt = Statement::with_parent(conn)?;

//     let mut sql_text = String::new();
//     println!("Please enter SQL statement string: ");
//     io::stdin().read_line(&mut sql_text).unwrap();

//     match stmt.exec_direct(&sql_text)? {
//         Data(mut stmt) => {
//             let cols = stmt.num_result_cols()?;
//             while let Some(mut cursor) = stmt.fetch()? {
//                 for i in 1..(cols + 1) {
//                     match cursor.get_data::<&str>(i as u16)? {
//                         Some(val) => print!(" {}", val),
//                         None => print!(" NULL"),
//                     }
//                 }
//                 println!("");
//             }
//         }
//         NoData(_) => println!("Query executed, no data returned"),
//     }

//     Ok(())
// }
