CREATE DATABASE rust_project
CREATE TABLE user
(
    id INT PRIMARY KEY,
    username VARCHAR(100),
    sender VARCHAR(100),
)
SELECT rust_project
FROM `user`
WHERE username,sender IS NOT NULL

CREATE TABLE rcv
(
    id CHAR PRIMARY KEY,
    <ip_server> VARCHAR(100),
)
