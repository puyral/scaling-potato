# /usr/bin/bash

printf "
RUST_PATH=./Rust/preprocessor
EXE_PATH=\$(RUST_PATH)/target/release
SQL_PATH=.
PAGE_RANK=--beta 0.2 --epsilon 1e-15
MKDIR_ARGS=-p
SQLITE_FILE=./db.sqlite
"

printf "\n\nall:"
for lang in "$@" 
do
printf " \$(SQL_PATH)/${lang}/${lang}.sql"
done

printf "\n\nsqlite:"
for lang in "$@" 
do
printf " sqlite_${lang}"
done

printf "\n\nclean: clean_rust"
for lang in "$@" 
do
printf " clean_%s" "$lang"
done

printf "\n
# --rust
clean_rust:
	rm -rf \${EXE_PATH}

\$(EXE_PATH)/preprocessor:
	cargo build --manifest-path \$(RUST_PATH)/Cargo.toml --release

"

for lang in "$@"
do

printf "
# --${lang}
\$(SQL_PATH)/${lang}/${lang}wiki-latest-page.sql:
	mkdir \$(MKDIR_ARGS) \${SQL_PATH}/${lang} && wget -O- https://dumps.wikimedia.org/${lang}wiki/latest/${lang}wiki-latest-page.sql.gz | gunzip -c > \$(SQL_PATH)/${lang}/${lang}wiki-latest-page.sql

\$(SQL_PATH)/${lang}/${lang}wiki-latest-categorylinks.sql:
	mkdir \$(MKDIR_ARGS) \${SQL_PATH}/${lang} && wget -O- https://dumps.wikimedia.org/${lang}wiki/latest/${lang}wiki-latest-categorylinks.sql.gz | gunzip -c > \$(SQL_PATH)/${lang}/${lang}wiki-latest-categorylinks.sql

\$(SQL_PATH)/${lang}/${lang}.sql: \$(SQL_PATH)/${lang}/${lang}wiki-latest-page.sql \$(SQL_PATH)/${lang}/${lang}wiki-latest-categorylinks.sql \$(EXE_PATH)/preprocessor
	\$(EXE_PATH)/preprocessor --categories \$(SQL_PATH)/${lang}/${lang}wiki-latest-page.sql --category-links \$(SQL_PATH)/${lang}/${lang}wiki-latest-categorylinks.sql --out \$(SQL_PATH)/${lang}/${lang}.sql --language ${lang} \$(PAGE_RANK)

sqlite_${lang}: \$(SQL_PATH)/${lang}/${lang}.sql
	cat \$(SQL_PATH)/${lang}/${lang}.sql | sqlite3 \$(SQLITE_FILE)
	sqlite3 -line \$(SQLITE_FILE) 'BEGIN; CREATE TABLE IF NOT EXISTS Wikipedias(name TEXT PRIMARY KEY); INSERT OR REPLACE INTO Wikipedias(name) VALUES (\"${lang}\"); COMMIT;'

clean_${lang}:
	rm -rf \$(SQL_PATH)/${lang}
"
done