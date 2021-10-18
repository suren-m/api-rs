CREATE USER dbadmin;
CREATE DATABASE API_DB;
GRANT ALL PRIVILEGES ON DATABASE API_DB TO dbadmin;

-- extension for using crypt function when storing passwords
CREATE EXTENSION pgcrypto;