CREATE TABLE users (
    user_id integer PRIMARY KEY AUTOINCREMENT,
	name text
);

CREATE TABLE groups (
    group_id integer PRIMARY KEY AUTOINCREMENT,
	is_closed integer
);

CREATE TABLE group_users (
	user_id integer,
	group_id integer,
	gift_recipent_id integer,
    is_admin integer
);




