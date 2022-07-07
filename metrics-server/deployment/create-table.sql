CREATE database metrics;
USE metrics;

CREATE TABLE Metrics (
    Id INT NOT NULL AUTO_INCREMENT,
    Language VARCHAR(255),
    Name VARCHAR(255),
    Context VARCHAR(255),
    Value VARCHAR(255),
    PRIMARY KEY (Id)
);