-- Your SQL goes here
-- I prefer singular table names, as it will make more sense once its converted to an object in the code
CREATE TABLE Country (
    id smallint NOT NULL AUTO_INCREMENT, -- only smallint, as there are not that many countries in the world
    name TEXT NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE City (
    id int NOT NULL AUTO_INCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    country_id smallint, -- smallint to match Country(id)
    PRIMARY KEY (id),
    FOREIGN KEY (country_id) REFERENCES Country(id) ON DELETE SET NULL
);

CREATE TABLE Language (
    id int NOT NULL AUTO_INCREMENT,
    name TEXT NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE CityLanguage (
    city_id int,
    language_id int,
    PRIMARY KEY (city_id, language_id),
    FOREIGN KEY (city_id) REFERENCES City(id) ON DELETE CASCADE,
    FOREIGN KEY (language_id) REFERENCES Language(id) ON DELETE CASCADE
);

CREATE TABLE PointOfInterest (
    id int NOT NULL AUTO_INCREMENT,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    city_id int NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (city_id) REFERENCES City(id) ON DELETE CASCADE
);