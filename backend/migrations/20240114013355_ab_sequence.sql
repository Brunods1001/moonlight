CREATE TABLE clone_id (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

    
CREATE TABLE antibody_dna (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    dna_sequence TEXT NOT NULL,
    read_count INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    clone_id INTEGER NOT NULL,

    FOREIGN KEY (clone_id) REFERENCES clone_id (id)
);


CREATE TABLE antibody_amino_acids (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    antibody_id INTEGER NOT NULL,
    f1 TEXT NOT NULL,
    cdr1 TEXT NOT NULL,
    f2 TEXT NOT NULL,
    cdr2 TEXT NOT NULL,
    f3 TEXT NOT NULL,
    cdr3 TEXT NOT NULL,
    f4 TEXT NOT NULL,
    
    read_count INTEGER NOT NULL,
    FOREIGN KEY (antibody_id) REFERENCES antibody_dna (id)
);

CREATE TRIGGER update_antibody_updated_at 
AFTER UPDATE ON antibody_dna 
BEGIN
    UPDATE antibodies SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TABLE uploaded_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    original_filename TEXT NOT NULL,
    stored_filename TEXT NOT NULL,
    file_path TEXT NOT NULL,
    content_type TEXT,
    file_size INTEGER,
    upload_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    description TEXT
);


-- Table to link files to antibodies
CREATE TABLE antibody_files (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    antibody_id INTEGER NOT NULL,
    file_id INTEGER NOT NULL,
    FOREIGN KEY (antibody_id) REFERENCES antibody_dna (id),
    FOREIGN KEY (file_id) REFERENCES uploaded_files (id)
);
