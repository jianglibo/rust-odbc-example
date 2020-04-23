# rust-odbc-example

# Oracle database docker images
docker run --name oracle-db -p 1521:1521 -p 5500:5500 -v oracle-db-data:/opt/oracle/oradata  oracle/database:12.2.0.1-se2

-e ORACLE_SID=<your SID> \
-e ORACLE_PDB=<your PDB name> \
-e ORACLE_PWD=<your database passwords> \
-e ORACLE_CHARACTERSET=<your character set> \

docker exec -ti oracle-db sqlplus pdbadmin@ORCLPDB1

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

[FreeTDS]
Description=FreeTDS unixODBC Driver
Driver=/usr/lib64/libtdsodbc.so.0
Setup=/usr/lib64/libtdsodbc.so.0
TDS_Version=7.2
UsageCount=1


~/.odbc.ini
//111.222.111.222:1521/mydatabase