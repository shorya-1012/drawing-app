CREATE TABLE verification_codes (
    code CHAR(6) NOT NULL,
    expiration_time TIMESTAMP DEFAULT (now() + interval '15 minutes'),
    userId varchar(255) NOT NULL,
    PRIMARY KEY(code),
    FOREIGN KEY(userId) REFERENCES app_users(userid) ON DELETE CASCADE
);
