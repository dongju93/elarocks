-- show users
SELECT usename AS role_name,
    CASE
        WHEN usesuper
        AND usecreatedb THEN CAST('superuser, create database' AS pg_catalog.text)
        WHEN usesuper THEN CAST('superuser' AS pg_catalog.text)
        WHEN usecreatedb THEN CAST('create database' AS pg_catalog.text)
        ELSE CAST('' AS pg_catalog.text)
    END role_attributes
FROM pg_catalog.pg_user
ORDER BY role_name desc;
-- create schema
CREATE SCHEMA "sysmon" AUTHORIZATION "dong-ju";
-- create table
CREATE TABLE sysmon.reg_eve (savedtime TIMESTAMP(8));
-- alter table
ALTER TABLE sysmon.reg_eve
ALTER COLUMN "savedtime" TYPE character varying;
-- query
SELECT *
FROM sysmon.reg_eve
WHERE savedtime BETWEEN '2023-09-07 01:59:00.00000000' and '2023-09-07 01:59:30.00000000';
-- delete duplicated
DELETE FROM sysmon.reg_eve
WHERE ctid NOT IN (
        SELECT DISTINCT ON (savedtime) ctid
        FROM sysmon.reg_eve
        ORDER BY savedtime,
            ctid
    );