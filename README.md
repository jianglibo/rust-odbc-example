# rust-odbc-example

# Oracle database docker images
docker run --name oracle-db -p 1521:1521 -p 5500:5500 -v oracle-db-data:/opt/oracle/oradata  oracle/database:12.2.0.1-se2

-e ORACLE_SID=<your SID> \
-e ORACLE_PDB=<your PDB name> \
-e ORACLE_PWD=<your database passwords> \
-e ORACLE_CHARACTERSET=<your character set> \

docker exec -ti oracle-db sqlplus pdbadmin@ORCLPDB1

A database instance is a set of memory structures that manage database files.
A database is a set of physical files on disk created by the CREATE DATABASE statement. The instance manages its associated data and serves the users of the database.

SELECT sys_context('USERENV','DB_NAME') AS Instance FROM dual;
SELECT owner, table_name FROM all_tables;


* PDB pluggable databases
* SID The Oracle System ID (SID) is used to uniquely identify a particular database on a system

# Oracle client docker image
docker build -t oracle/instantclient:12.2.0.1 .
docker run --name oracle-instantclient -p 2226:22 -v rustup:/usr/local/rustup -v cargo:/usr/local/cargo -v root:/root -d oracle/instantclient:12.2.0.1