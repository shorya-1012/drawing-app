create table rooms (
    room_id VARCHAR(255) NOT NULL,
    members VARCHAR(50)[] NOT NULL,
    PRIMARY KEY(room_id)
)
