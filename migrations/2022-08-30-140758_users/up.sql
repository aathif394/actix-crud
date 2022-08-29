CREATE TABLE users (
    id serial NOT NULL,
    name character varying(255) NOT NULL,
    email character varying(255) NOT NULL,
    created_at timestamp NOT NULL,
    CONSTRAINT users_pkey PRIMARY KEY (id)
);
