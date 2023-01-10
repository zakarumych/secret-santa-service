CREATE TABLE users (
                       name text,
                       user_id integer PRIMARY KEY AUTOINCREMENT,
                       token text,
                       is_logged integer,
                       hash_psw text
);

CREATE TABLE groups (
                        group_id integer PRIMARY KEY AUTOINCREMENT,
                        is_closed integer
);

CREATE TABLE group_users (
                             is_admin integer,
                             user_id integer,
                             group_id integer,
                             gift_recipient_id integer
);



