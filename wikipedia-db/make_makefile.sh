# /usr/bin/bash

printf "
RUST_PATH=./Rust/preprocessor
EXE_PATH=\$(RUST_PATH)/target/release
SQL_PATH=.
PAGE_RANK=--beta 0.2 --epsilon 1e-15
MKDIR_ARGS=-p

\$(EXE_PATH)/preprocessor:
	cargo build --manifest-path \$(RUST_PATH)/Cargo.toml --release

clean_rust:
	rm -rf \${EXE_PATH}
"


for lang in "$@"
do

printf "
\$(SQL_PATH)/${lang}/${lang}wiki-latest-page.sql:
	mkdir \$(MKDIR_ARGS) \${SQL_PATH}/${lang} && wget -O- https://dumps.wikimedia.org/${lang}wiki/latest/${lang}wiki-latest-page.sql.gz | gunzip -c > \$(SQL_PATH)/${lang}/${lang}wiki-latest-page.sql

\$(SQL_PATH)/${lang}/${lang}wiki-latest-categorylinks.sql:
	wget -O- https://dumps.wikimedia.org/${lang}wiki/latest/${lang}wiki-latest-categorylinks.sql.gz | gunzip -c > \$(SQL_PATH)/${lang}/${lang}wiki-latest-categorylinks.sql

\$(SQL_PATH)/${lang}/${lang}.sql: \$(SQL_PATH)/${lang}/${lang}wiki-latest-page.sql \$(SQL_PATH)/${lang}/${lang}wiki-latest-categorylinks.sql \$(EXE_PATH)/preprocessor
	\$(EXE_PATH)/preprocessor -c \$(SQL_PATH)/${lang}/${lang}wiki-latest-page.sql -C \$(SQL_PATH)/${lang}/${lang}wiki-latest-categorylinks.sql \$(PAGE_RANK) -o \$(SQL_PATH)/${lang}/${lang}.sql

${lang}: \$(SQL_PATH)/${lang}/${lang}.sql

clean_${lang}:
	rm -rf \$(SQL_PATH)/${lang}
"
done

printf "\nclean: clean_rust"
for lang in "$@" 
do
printf " clean_%s" "$lang"
done

printf "\nall: "
for lang in "$@" 
do
printf " %s" "$lang"
done