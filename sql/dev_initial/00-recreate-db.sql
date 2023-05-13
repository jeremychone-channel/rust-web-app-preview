-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
select pg_terminate_backend(pid) from pg_stat_activity where usename = 'app_user';
DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

-- DEV ONLY - Dev only password (for local dev and unit test).
CREATE USER app_user PASSWORD 'dev_only_pwd';
CREATE DATABASE app_db owner app_user ENCODING = 'UTF-8';
