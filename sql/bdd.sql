-- Create pks table
CREATE TABLE pks (
    userid BIGINT PRIMARY KEY,
    pk1 TEXT NOT NULL,
    pk2 TEXT NOT NULL,
    pk3 TEXT NOT NULL
);

-- Create login table
CREATE TABLE login (
    userid BIGINT PRIMARY KEY,
    pass TEXT NOT NULL
);

-- Create registered table
CREATE TABLE registered (
    userid BIGINT,
    firstmsgid BIGINT,
    PRIMARY KEY(userid, firstmsgid)
);
