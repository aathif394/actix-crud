CREATE TABLE users (
    id serial NOT NULL,
    name character varying(255) NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
);