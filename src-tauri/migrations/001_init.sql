-- WAL permet aux lecteurs de continuer pendant une ingestion.
PRAGMA journal_mode=WAL;
-- Les contraintes de références restent actives sur chaque connexion.
PRAGMA foreign_keys=ON;
-- Le délai évite une erreur immédiate si un hook écrit simultanément.
PRAGMA busy_timeout=5000;
-- La commande masquée est la seule représentation conservée sur disque.
CREATE TABLE IF NOT EXISTS commands (
 id INTEGER PRIMARY KEY, hash TEXT NOT NULL UNIQUE, raw TEXT NOT NULL,
 clean TEXT NOT NULL, binary TEXT NOT NULL, category TEXT NOT NULL,
 privileged INTEGER NOT NULL, shell TEXT NOT NULL, cwd TEXT NOT NULL,
 terminal TEXT NOT NULL, exit_code INTEGER, duration_ms INTEGER,
 count INTEGER NOT NULL DEFAULT 1, favorite INTEGER NOT NULL DEFAULT 0,
 first_seen INTEGER NOT NULL, last_seen INTEGER NOT NULL
);
-- Le trigramme permet une recherche de sous-chaîne sans réseau.
CREATE VIRTUAL TABLE IF NOT EXISTS command_fts USING fts5(raw, content='commands', content_rowid='id', tokenize='trigram');
-- Les trois triggers empêchent la désynchronisation entre données et index.
CREATE TRIGGER IF NOT EXISTS commands_ai AFTER INSERT ON commands BEGIN INSERT INTO command_fts(rowid,raw) VALUES(new.id,new.raw); END;
CREATE TRIGGER IF NOT EXISTS commands_ad AFTER DELETE ON commands BEGIN INSERT INTO command_fts(command_fts,rowid,raw) VALUES('delete',old.id,old.raw); END;
CREATE TRIGGER IF NOT EXISTS commands_au AFTER UPDATE OF raw ON commands BEGIN INSERT INTO command_fts(command_fts,rowid,raw) VALUES('delete',old.id,old.raw); INSERT INTO command_fts(rowid,raw) VALUES(new.id,new.raw); END;
-- Un curseur par fichier rend les réimportations idempotentes.
CREATE TABLE IF NOT EXISTS imports(path TEXT PRIMARY KEY, digest TEXT NOT NULL);
-- Les captures indépendantes restent lisibles quand Docker s'arrête.
CREATE TABLE IF NOT EXISTS snapshots(kind TEXT NOT NULL, key TEXT NOT NULL, payload TEXT NOT NULL, captured_at INTEGER NOT NULL, PRIMARY KEY(kind,key));
-- Les modèles SQL sont conservés indépendamment des historiques shell.
CREATE TABLE IF NOT EXISTS models(name TEXT PRIMARY KEY, ddl TEXT NOT NULL, updated_at INTEGER NOT NULL);
