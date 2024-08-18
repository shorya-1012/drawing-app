CREATE TABLE rooms_users (
    id serial not null,
    room_id varchar(255) not null,
    user_id varchar(255) not null,
    socket_id varchar(255) unique,
    PRIMARY KEY(id)
);
