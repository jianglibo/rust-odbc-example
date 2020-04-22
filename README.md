# rust-odbc-example

# Oracle database docker images
docker run --name oracle-db -p 1521:1521 -p 5500:5500 -v oracle-db-data:/opt/oracle/oradata  oracle/database:12.2.0.1-se2

-e ORACLE_SID=<your SID> \
-e ORACLE_PDB=<your PDB name> \
-e ORACLE_PWD=<your database passwords> \
-e ORACLE_CHARACTERSET=<your character set> \

docker exec -ti rust-odbc-example_oracle-db_1 sqlplus pdbadmin@ORCLPDB1

set ORACLE_SID=database name

A database instance is a set of memory structures that manage database files.
A database is a set of physical files on disk created by the CREATE DATABASE statement. The instance manages its associated data and serves the users of the database.

SELECT sys_context('USERENV','DB_NAME') AS Instance FROM dual;
select sys_context('userenv','instance_name') from dual;
SELECT owner, table_name FROM all_tables;


* PDB pluggable databases
* SID The Oracle System ID (SID) is used to uniquely identify a particular database on a system

# Oracle client docker image
docker build -t oracle/instantclient:12.2.0.1 .
docker run --name oracle-instantclient -p 2226:22 -v rustup:/usr/local/rustup -v cargo:/usr/local/cargo -v root:/root -d oracle/instantclient:12.2.0.1

# ODBC
http://mvsourcecode.com/how-to-configure-oracle-odbc-driver-on-centos-redhat-mvsourcecode/


https://www.oracle.com/database/technologies/releasenote-odbc-ic.html



yum install unixODBC unixODBC-devel freetds -y
odbcinst -j
isql -v odbc

#open the file
$ vim /etc/odbc.ini

set ORACLE_SID=database name
#modify details as per your configuration
[oracletest]
Trace = yes
TraceFile = /tmp/odbc_oracle.log
Database =// localhost:1521/orcl
UserID = oracle
Password = oracle10


updatedb && locate libsqora to find path.

http://docs.adaptivecomputing.com/9-1-0/MWS/Content/topics/moabWorkloadManager/topics/databases/oracle.html

locate odbc_update_ini.sh

odbcinst -j

isql -v OracleODBC-12c


isql -v OracleODBC-12c PDBADMIN bnvxEAvXs1o=1


~/.odbc.ini
//111.222.111.222:1521/mydatabase


oracle-db_1      | ORACLE PASSWORD FOR SYS, SYSTEM AND PDBADMIN: bnvxEAvXs1o=1
oracle-db_1      |
oracle-db_1      | LSNRCTL for Linux: Version 12.2.0.1.0 - Production on 23-APR-2020 07:48:04
oracle-db_1      |
oracle-db_1      | Copyright (c) 1991, 2016, Oracle.  All rights reserved.
oracle-db_1      |
oracle-db_1      | Starting /opt/oracle/product/12.2.0.1/dbhome_1/bin/tnslsnr: please wait...
oracle-db_1      |
oracle-db_1      | TNSLSNR for Linux: Version 12.2.0.1.0 - Production
oracle-db_1      | System parameter file is /opt/oracle/product/12.2.0.1/dbhome_1/network/admin/listener.ora
oracle-db_1      | Log messages written to /opt/oracle/diag/tnslsnr/2c0edcbe19ac/listener/alert/log.xml
oracle-db_1      | Listening on: (DESCRIPTION=(ADDRESS=(PROTOCOL=ipc)(KEY=EXTPROC1)))
oracle-db_1      | Listening on: (DESCRIPTION=(ADDRESS=(PROTOCOL=tcp)(HOST=0.0.0.0)(PORT=1521)))
oracle-db_1      |
oracle-db_1      | Connecting to (DESCRIPTION=(ADDRESS=(PROTOCOL=IPC)(KEY=EXTPROC1)))
oracle-db_1      | STATUS of the LISTENER
oracle-db_1      | ------------------------
oracle-db_1      | Alias                     LISTENER
oracle-db_1      | Version                   TNSLSNR for Linux: Version 12.2.0.1.0 - Production
oracle-db_1      | Start Date                23-APR-2020 07:48:09
oracle-db_1      | Uptime                    0 days 0 hr. 0 min. 2 sec
oracle-db_1      | Trace Level               off
oracle-db_1      | Security                  ON: Local OS Authentication
oracle-db_1      | SNMP                      OFF
oracle-db_1      | Listener Parameter File   /opt/oracle/product/12.2.0.1/dbhome_1/network/admin/listener.ora
oracle-db_1      | Listener Log File         /opt/oracle/diag/tnslsnr/2c0edcbe19ac/listener/alert/log.xml
oracle-db_1      | Listening Endpoints Summary...
oracle-db_1      |   (DESCRIPTION=(ADDRESS=(PROTOCOL=ipc)(KEY=EXTPROC1)))
oracle-db_1      |   (DESCRIPTION=(ADDRESS=(PROTOCOL=tcp)(HOST=0.0.0.0)(PORT=1521)))
oracle-db_1      | The listener supports no services
oracle-db_1      | The command completed successfully
oracle-db_1      | [WARNING] [DBT-10102] The listener configuration is not selected for the database. EM DB Express URL will not be accessible.
oracle-db_1      |    CAUSE: The database should be registered with a listener in order to access the EM DB Express URL.
oracle-db_1      |    ACTION: Select a listener to be registered or created with the database.
oracle-db_1      | Copying database files
oracle-db_1      | 1% complete

oracle-db_1      | Look at the log file "/opt/oracle/cfgtoollogs/dbca/ORCLCDB/ORCLCDB.log" for further details.
oracle-db_1      |
oracle-db_1      | SQL*Plus: Release 12.2.0.1.0 Production on Thu Apr 23 07:58:32 2020
oracle-db_1      |
oracle-db_1      | Copyright (c) 1982, 2016, Oracle.  All rights reserved.
oracle-db_1      |
oracle-db_1      |
oracle-db_1      | Connected to:
oracle-db_1      | Oracle Database 12c Standard Edition Release 12.2.0.1.0 - 64bit Production
oracle-db_1      |
oracle-db_1      | SQL>
oracle-db_1      | System altered.
oracle-db_1      |
oracle-db_1      | SQL>
oracle-db_1      | Pluggable database altered.
oracle-db_1      |
oracle-db_1      | SQL>
oracle-db_1      | PL/SQL procedure successfully completed.
oracle-db_1      |
oracle-db_1      | SQL> Disconnected from Oracle Database 12c Standard Edition Release 12.2.0.1.0 - 64bit Production
oracle-db_1      | The Oracle base remains unchanged with value /opt/oracle
oracle-db_1      | #########################
oracle-db_1      | DATABASE IS READY TO USE!
oracle-db_1      | #########################
oracle-db_1      | The following output is now a tail of the alert.log:
oracle-db_1      | Completed: alter pluggable database ORCLPDB1 open
oracle-db_1      | 2020-04-23T07:58:29.252626+00:00
oracle-db_1      | ORCLPDB1(3):CREATE SMALLFILE TABLESPACE "USERS" LOGGING  DATAFILE  '/opt/oracle/oradata/ORCLCDB/ORCLPDB1/users01.dbf' SIZE 5M REUSE AUTOEXTEND ON NEXT  128
0K MAXSIZE UNLIMITED  EXTENT MANAGEMENT LOCAL  SEGMENT SPACE MANAGEMENT  AUTO
oracle-db_1      | ORCLPDB1(3):Completed: CREATE SMALLFILE TABLESPACE "USERS" LOGGING  DATAFILE  '/opt/oracle/oradata/ORCLCDB/ORCLPDB1/users01.dbf' SIZE 5M REUSE AUTOEXTEND O
N NEXT  1280K MAXSIZE UNLIMITED  EXTENT MANAGEMENT LOCAL  SEGMENT SPACE MANAGEMENT  AUTO
oracle-db_1      | ORCLPDB1(3):ALTER DATABASE DEFAULT TABLESPACE "USERS"
oracle-db_1      | ORCLPDB1(3):Completed: ALTER DATABASE DEFAULT TABLESPACE "USERS"
oracle-db_1      | 2020-04-23T07:58:33.074183+00:00
oracle-db_1      | ALTER SYSTEM SET control_files='/opt/oracle/oradata/ORCLCDB/control01.ctl' SCOPE=SPFILE;
oracle-db_1      |    ALTER PLUGGABLE DATABASE ORCLPDB1 SAVE STATE
oracle-db_1      | Completed:    ALTER PLUGGABLE DATABASE ORCLPDB1 SAVE STATE