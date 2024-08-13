create table app_users (
    userid VARCHAR(255) NOT NULL ,
    username VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL ,
    gender VARCHAR(255) NOT NULL ,
    PRIMARY KEY(userid),
    CHECK (char_length(username) > 3),
    CHECK (char_length(password) > 6)
)
