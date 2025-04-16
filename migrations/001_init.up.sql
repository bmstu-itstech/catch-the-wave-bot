DO $$ BEGIN
    CREATE TYPE NEXT_TASK_STATUS AS ENUM ('pending', 'accepted', 'rejected');
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

DO $$ BEGIN
    CREATE TYPE USER_TASK_STATE AS ENUM ('active', 'completed');
EXCEPTION
    WHEN duplicate_object THEN NULL;
END $$;

CREATE TABLE IF NOT EXISTS tasks (
    year        INTEGER         NOT NULL,
    week        INTEGER         NOT NULL,
    title       VARCHAR(128)    NOT NULL,
    description TEXT            NOT NULL,

    PRIMARY KEY (year, week)
);

CREATE TABLE IF NOT EXISTS users (
    id                  BIGINT              PRIMARY KEY,
    username            VARCHAR(64)         UNIQUE NOT NULL,
    full_name           VARCHAR(128)        DEFAULT NULL,
    group_name          VARCHAR(16)         DEFAULT NULL,
    next_task_status    NEXT_TASK_STATUS    NOT NULL,
    completed_tasks     INTEGER             NOT NULL
);

CREATE TABLE IF NOT EXISTS user_tasks (
    user_id    BIGINT          PRIMARY KEY,
    task_year  INTEGER         NOT NULL,
    task_week  INTEGER         NOT NULL,
    partner_id BIGINT          NOT NULL,
    state      USER_TASK_STATE NOT NULL,

    FOREIGN KEY (task_year, task_week)
        REFERENCES tasks (year, week)
);
