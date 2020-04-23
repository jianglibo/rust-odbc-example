#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate clap;

extern crate env_logger;
extern crate odbc_safe;

use odbc_safe::{Environment, Connection, AutocommitMode, NoCursor, Indicator,
     Version, DataSource, Statement, ReturnOption, DataType, ResultSet, Unprepared, Prepared, sys,
      sys::SQLRETURN,
    sys::SqlCDataType};
use clap::App;
use clap::ArgMatches;
use log::*;
use std::env;
use std::str::from_utf8;


// lazy_static! {
//     /// This is an example for using doc comment attributes
//     static ref ENV: odbc::Environment<odbc::Version3> = odbc::create_environment_v3().map_err(|e| e.unwrap())?;
// }


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
            // print_drivers_and_datasources()?;
        }
        (_, _) => {
            unimplemented!();
        }
    }
    Ok(())
}

// fn print_drivers_and_datasources() -> odbc_safe::Result<()> {
//     let mut env = odbc_safe::create_environment_v3().map_err(|e| e.unwrap())?;
//     println!("Driver list:");
//     for driver_info in env.drivers()? {
//         println!("\nDriver Name: {}", driver_info.description);
//         for (key, value) in driver_info.attributes {
//             println!("    {}={}", key, value);
//         }
//     }

//     println!("\nDataSource list:");
//     for ds in env.data_sources()? {
//         println!("    {}\n    {}\n\n", ds.server_name, ds.driver);
//     }
//     Ok(())
// }

// con = DriverManager.getConnection(this.jdbcUrl, this.user, this.password);
// String sql = "{call sp_docList(?, ?)}";
// callsta = con.prepareCall(sql);//收发文
// callsta.setString(1, this.user);
// callsta.registerOutParameter(2, oracle.jdbc.OracleTypes.CURSOR);
// callsta.execute();

// fn call_stored_procedures<'env>(conn: &safe::Connection<'env, safe::AutocommitOn>) -> Result<(), failure::Error> {
//     let stmt = safe::Statement::with_parent(&conn);
//     let param = 1968;
//     let mut stmt = stmt.bind_parameter(1, &param);
//     let v = Vec::<u8>::new();
//     let state_ptr: *mut core::ffi::c_void = &mut v as *mut _ as *mut core::ffi::c_void;
//     let mut i = 100_i64;

//     let mut my_speed= 88_i64;
//     let my_speed_ptr: *mut i64 = &mut my_speed;

//     ffi::SQLBindParameter(stmt.as_raw(), 1_u16, ffi::InputOutput::SQL_PARAM_OUTPUT, ffi::SqlCDataType::SQL_C_BINARY, 
//         ffi::SqlDataType::SQL_EXT_VARBINARY, 100_1000, 5, state_ptr, 5, my_speed_ptr);
//     // stmt.bind_parameter(1, &odbc::ffi::SQL_PARAM_OUTPUT_STREAM);
//     let sql_text = "{call sp_docList(?, ?)}";
//     if let odbc::Data(mut stmt) = stmt.exec_direct(sql_text)? {

//     }
//     Ok(())
// }

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

// fn connect<'env>() -> std::result::Result<odbc::Connection<'env, odbc::safe::AutocommitOn>, odbc::DiagnosticRecord> {
//     let oenv= odbc::create_environment_v3().map_err(|e| e.unwrap())?;
//     let mut buffer = "DRIVER={Oracle 12c ODBC driver};DBQ=10.68.132.154:1521/orcl;UID=ZZK_FENGHUA_W;PWD=FENGHUA316316".to_string();
//     Ok(oenv.connect_with_connection_string(&buffer)?)
// }

// // https://docs.oracle.com/en/database/oracle/oracle-database/19/adfns/odbc-driver.html#GUID-7931EDFB-7A70-4BBE-903E-8A2BB09DBE9D

// fn execute_statement<'env>(conn: &odbc::Connection<'env, odbc::safe::AutocommitOn>, sql_text: &str) -> odbc::Result<()> {
//     let stmt = odbc::Statement::with_parent(conn)?;

//     // let mut sql_text = String::new();
//     // println!("Please enter SQL statement string: ");

//     match stmt.exec_direct(&sql_text)? {
//         odbc::Data(mut stmt) => {
//             let cols = stmt.num_result_cols()?;
//             while let Some(mut cursor) = stmt.fetch()? {
//                 for i in 1..(cols + 1) {
//                     match cursor.get_data::<&str>(i as u16)? {
//                         Some(val) => print!(" {}", val),
//                         None => print!(" NULL"),
//                     }
//                 }
//                 println!("2222222222222");
//             }
//         }
//         odbc::NoData(_) => println!("Query executed, no data returned"),
//     }

//     Ok(())
// }

fn connect<V>(env: &Environment<V>) -> Connection<impl AutocommitMode>
where
    V: Version,
{
    let conn = DataSource::with_parent(env).unwrap();
    conn.connect("OracleODBC-12c", "ZZK_FENGHUA_W", "FENGHUA316316").unwrap()
}

fn print_fields<'a>(
    result_set: ResultSet<'a, 'a, 'a, Unprepared>,
) -> Statement<'a, 'a, 'a, NoCursor, Unprepared> {
    let mut buffer = [0u8; 512];
    let mut cursor = match result_set.fetch() {
        ReturnOption::Success(r) |
        ReturnOption::Info(r) => r,
        ReturnOption::NoData(r) |
        ReturnOption::Error(r) => return r,
    };
    loop {
        match cursor.get_data(1, &mut buffer as &mut [u8]) {
            ReturnOption::Success(ind) |
            ReturnOption::Info(ind) => {
                match ind {
                    Indicator::NoTotal => panic!("No Total"),
                    Indicator::Null => println!("NULL"),
                    Indicator::Length(l) => {
                        println!("{}", from_utf8(&buffer[0..l as usize]).unwrap());
                    }
                }
            }
            ReturnOption::NoData(_) |
            ReturnOption::Error(_) => panic!("No Field Data"),
        }
        cursor = match cursor.fetch() {
            ReturnOption::Success(r) |
            ReturnOption::Info(r) => r,
            ReturnOption::NoData(r) |
            ReturnOption::Error(r) => break r,
        };
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sys::SQLBindParameter;

    #[test]
    fn t_show_tables() ->  Result<(), failure::Error> {
        let env = Environment::new().unwrap();
        let env = env.declare_version_3().unwrap();
        let conn = connect(&env);

        let stmt = Statement::with_parent(&conn).unwrap();

        let stmt = match stmt.exec_direct("select sys_context('userenv','instance_name') from dual") {
            ReturnOption::Success(s) |
            ReturnOption::Info(s) => s,
            ReturnOption::NoData(_) |
            ReturnOption::Error(_) => panic!("No Result Set"),
        };
        let result_set = stmt.reset_parameters();

        print_fields(result_set);
        Ok(())
    }

    #[test]
    fn t_show_call() ->  Result<(), failure::Error> { 
        let env = Environment::new().unwrap();
        let env = env.declare_version_3().unwrap();
        let conn = connect(&env);

        let stmt = Statement::with_parent(&conn).unwrap();


        let stmt = stmt.bind_input_parameter(1, DataType::Char(13), "ZZK_FENGHUA_W".as_bytes(), None)
        .unwrap();

        unsafe {
            bnd_parameter(stmt.as_raw());
        }

        let stmt = match stmt.exec_direct("{call sp_docList(?, ?)}") {
            ReturnOption::Success(s) |
            ReturnOption::Info(s) => s,
            ReturnOption::NoData(err) |
            ReturnOption::Error(err) => panic!("No Result Set: {:?}", err),
        };

        // https://docs.rs/odbc-sys/0.8.2/odbc_sys/fn.SQLBindParameter.html

        // pub unsafe extern "system" fn SQLBindParameter(
        //     hstmt: SQLHSTMT, 
        //     parameter_number: SQLUSMALLINT, 
        //     input_output_type: InputOutput, 
        //     value_type: SqlCDataType, 
        //     parmeter_type: SqlDataType, 
        //     column_size: SQLULEN, 
        //     decimal_digits: SQLSMALLINT, 
        //     parameter_value_ptr: SQLPOINTER, 
        //     buffer_length: SQLLEN, 
        //     str_len_or_ind_ptr: *mut SQLLEN
        // ) -> SQLRETURN

        let result_set = stmt.reset_parameters();
        Ok(())
    }

    // https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlbindparameter-function?view=sql-server-ver15
    // https://docs.microsoft.com/en-us/sql/odbc/reference/appendixes/column-size-decimal-digits-transfer-octet-length-and-display-size?view=sql-server-ver15

    unsafe fn bnd_parameter(stmt: sys::SQLHSTMT) {
        let mut v = Vec::<u8>::new();
        let parameter_value_ptr: *mut core::ffi::c_void = &mut v as *mut _ as *mut core::ffi::c_void;
        let mut i = 100_i64;
        let mut my_speed= 0_i64;
        let my_speed_ptr: *mut i64 = &mut my_speed;

        match SQLBindParameter(stmt, 2, sys::InputOutput::SQL_PARAM_OUTPUT, SqlCDataType::SQL_C_BINARY, 
        sys::SqlDataType::SQL_EXT_VARBINARY, 100_1000, 0, parameter_value_ptr, 0, my_speed_ptr) {
            SQLRETURN::SQL_SUCCESS => {
                println!("success bind.");
            }
            SQLRETURN::SQL_ERROR => {
                println!("err");
            }

            e  => {
                print!("{:?}", e);
            }
        }
    }

}