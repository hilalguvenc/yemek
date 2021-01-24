CREATE TABLE category (
  id uuid DEFAULT uuid_generate_v4(),
  name VARCHAR(50) NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE recipe (
  id uuid DEFAULT uuid_generate_v4(),
  name VARCHAR(50) NOT NULL,
  description VARCHAR(500) NOT NULL,
  calories SMALLINT NOT NULL,
  prep_minutes SMALLINT NOT NULL,
  cook_minutes SMALLINT NOT NULL,
  keywords VARCHAR(50) [] NOT NULL,
  images VARCHAR(500) [] NOT NULL,
  ingredient jsonb NOT NULL,
  category_id uuid NOT NULL,
  people SMALLINT NOT NULL,
  rating_value REAL NOT NULL,
  rating_count SMALLINT NOT NULL,
  instructions: jsonb not null,
  created_at timestamp NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id),
  CONSTRAINT fk_category FOREIGN KEY(category_id) REFERENCES categories(id)
);