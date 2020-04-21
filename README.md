# rust-odbc-example

# Oracle database docker images
docker run --name oracle-db -p 1521:1521 -p 5500:5500 -v oracle-db-data:/opt/oracle/oradata  oracle/database:12.2.0.1

-e ORACLE_SID=<your SID> \
-e ORACLE_PDB=<your PDB name> \
-e ORACLE_PWD=<your database passwords> \
-e ORACLE_CHARACTERSET=<your character set> \

docker exec -ti oracle-db sqlplus pdbadmin@ORCLPDB1

* PDB pluggable databases
* SID The Oracle System ID (SID) is used to uniquely identify a particular database on a system

# Oracle client docker image
