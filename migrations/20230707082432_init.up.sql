CREATE TABLE nurls (
    id uuid NOT NULL,
    title varchar(255) NOT NULL,
    PRIMARY KEY (id),
    created_at timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    views integer NOT NULL DEFAULT 0
);

CREATE TABLE urls (
    id serial PRIMARY KEY,
    url varchar(255) NOT NULL,
    nurl uuid,
    FOREIGN KEY (nurl) REFERENCES nurls (id)
);
