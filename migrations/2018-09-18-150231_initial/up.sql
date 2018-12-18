-- Your SQL goes here

CREATE TYPE RECORD_STATUS AS ENUM ('APPROVED', 'REJECTED', 'SUBMITTED', 'DELETED');
CREATE TYPE AUDIT_OPERATION AS ENUM (
    'ADD_DEMON','PATCH_DEMON',
    'ADD_RECORD', 'REMOVE_RECORD', 'PATCH_RECORD',
    'ADD_PLAYER', 'REMOVE_PLAYER', 'PATCH_PLAYER',
    'BAN_SUBMITTER'
);

CREATE TABLE players(
    id SERIAL PRIMARY KEY,
    name CITEXT NOT NULL UNIQUE,
    banned BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE submitters (
    submitter_id SERIAL PRIMARY KEY,
    ip_address INET NOT NULL,
    banned BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE members (
    member_id SERIAL PRIMARY KEY,
    name CITEXT UNIQUE NOT NULL,
    display_name CITEXT NULL DEFAULT NULL,
    youtube_channel VARCHAR(200) NULL DEFAULT NULL,
    password_hash BYTEA NOT NULL,
    password_salt BYTEA NOT NULL,
    permissions BIT(16) NOT NULL DEFAULT B'0000000000000000'::BIT(16)
);

CREATE TABLE demons (
    name CITEXT PRIMARY KEY,
    position SMALLINT NOT NULL,
    requirement SMALLINT NOT NULL,
    video VARCHAR(200),
    description TEXT NULL,
    notes TEXT NULL,
    verifier INT NOT NULL REFERENCES players(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    publisher INT NOT NULL REFERENCES players(id) ON DELETE RESTRICT ON UPDATE CASCADE,

    CONSTRAINT unique_position UNIQUE (position) DEFERRABLE INITIALLY IMMEDIATE,
    CONSTRAINT valid_record_req CHECK (requirement >= 0 AND requirement <= 100)
);

CREATE TABLE records (
    id SERIAL PRIMARY KEY,
    progress SMALLINT CHECK (progress >= 0 AND progress <= 100) NOT NULL,
    video VARCHAR(200) UNIQUE,
    status_ RECORD_STATUS NOT NULL,
    player INT NOT NULL REFERENCES players(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    submitter INT NOT NULL REFERENCES submitters(submitter_id) ON DELETE RESTRICT,
    demon CITEXT NOT NULL REFERENCES demons(name) ON DELETE CASCADE ON UPDATE CASCADE,
    UNIQUE (demon, player, status_)
);

CREATE FUNCTION IF NOT EXISTS list_points(
    demon_pos SMALLINT NOT NULL,
    progress SMALLINT NOT NULL,
    list_length SMALLINT NOT NULL,
) RETURNS FLOAT AS $$
BEGIN
    SELECT (list_length / ((list_length / 5.0) + ((-list_length / 5.0) + 1.0) * EXP(-0.008*demon_pos))) AS points_value
    IF (progress < 1) THEN
        points_value *= (progress / 2.0)
    END IF;
    RETURN points_value;
END;

CREATE TABLE creators (
    demon CITEXT NOT NULL REFERENCES demons(name) ON DELETE RESTRICT ON UPDATE CASCADE,
    creator INT NOT NULL REFERENCES players(id) ON DELETE RESTRICT ON UPDATE CASCADE,
    PRIMARY KEY (demon, creator)
);

CREATE TABLE audit_log (
    id SERIAL PRIMARY KEY,
    operation AUDIT_OPERATION NOT NULL,
    target VARCHAR(200),
    old_value VARCHAR(200),
    new_value VARCHAR(200),
    time_ TIMESTAMP WITHOUT TIME ZONE DEFAULT (NOW() AT TIME ZONE 'utc') NOT NULL,
    list_mod CITEXT NULL REFERENCES members(name) ON DELETE SET NULL ON UPDATE CASCADE,
    demon CITEXT NULL REFERENCES demons(name) ON DELETE SET NULL ON UPDATE CASCADE,
    record INT NULL REFERENCES records(id) ON DELETE SET NULL ON UPDATE CASCADE,
    player INT NULL REFERENCES players(id) ON DELETE SET NULL ON UPDATE CASCADE
);