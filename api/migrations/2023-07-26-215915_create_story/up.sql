CREATE TABLE "user_account" (
	id serial NOT NULL UNIQUE,
	username varchar(100) NOT NULL UNIQUE,
	email varchar(100) NOT NULL UNIQUE,
	password varchar(100) NOT NULL,
	first_name varchar(100) NOT NULL,
	last_name varchar(100) NOT NULL,
	CONSTRAINT user_pk PRIMARY KEY (id)
);

CREATE TABLE story (
	id serial NOT NULL UNIQUE,
	data TEXT NOT NULL,
	CONSTRAINT story_pk PRIMARY KEY (id)
);

CREATE TABLE user_story (
	id serial NOT NULL,
	story_id integer NOT NULL,
	user_id integer NOT NULL,
	CONSTRAINT user_story_pk PRIMARY KEY (id)
);



CREATE TABLE comment (
	id serial NOT NULL UNIQUE,
	data TEXT NOT NULL,
	story_id integer NOT NULL,
	user_id integer NOT NULL,
	CONSTRAINT comment_pk PRIMARY KEY (id)
);



ALTER TABLE user_story ADD CONSTRAINT user_story_fk0 FOREIGN KEY (story_id) REFERENCES story(id);
ALTER TABLE user_story ADD CONSTRAINT user_story_fk1 FOREIGN KEY (user_id) REFERENCES user_account(id);

ALTER TABLE comment ADD CONSTRAINT comment_fk0 FOREIGN KEY (story_id) REFERENCES story(id);
ALTER TABLE comment ADD CONSTRAINT comment_fk1 FOREIGN KEY (user_id) REFERENCES user_account(id);

