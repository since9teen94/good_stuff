create table users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
insert into users (
    first_name,
    last_name,
    email,
    password
) 
-- password == password
values (
    'Frodo',
    'Baggins',
    'frodo@theshire.com',
    '$argon2id$v=19$m=4096,t=3,p=1$DSllqdd9PjfKOC6d6SklUw$4w3oMNzESXAzrznVFnm7905q2v9XnSARToJf9st7pWw'
);
