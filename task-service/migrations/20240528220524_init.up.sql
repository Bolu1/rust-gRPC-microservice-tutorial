CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "tasks" (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    title VARCHAR(225) NOT NULL,
    description VARCHAR(225),
    is_completed Boolean NOT NULL DEFAULT False,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);