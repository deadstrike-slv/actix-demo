-- Your SQL goes here
CREATE TYPE task_state AS ENUM (
    'NotStarted',
    'InProgress',
    'Completed',
    'Paused',
    'Failed'
);
CREATE TABLE ac_task (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT,
  "type" VARCHAR NOT NULL,
  "state" task_state NOT NULL
);