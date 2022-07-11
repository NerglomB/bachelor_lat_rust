# Voraussetzungen
- Rust entsprechend der Installationsanweisungen auf dem System installieren. https://www.rust-lang.org/tools/install

# Tests ausführen
- Den Befehl `cargo test` im Projektverzeichnis ausführen.

# Benchmarks ausführen
- Den Befehl `cargo +nightly bench` im Projektverzeichnis ausführen.

# Programm ausführen
- Datei, die ausgeführt werden soll, in `main.rs` umbennen.
- Den Befehl `cargo run` im Projektverzeichnis ausführen.

# Ausführbare Datei erzeugen
- Den Befehl `cargo build --release` im Projektverzeichnis ausführen.

# Hinweise
- Die Datei `main.rs` enthält Code, mit dem es Möglich ist, das CAS mit dem Typen `PrecisionNum` zu verwenden.
- Die Datei `tests/compare-sympy.rs` enthält alle Codebeispiele, die in der Arbeit mit SymPy verglichen worden sind.
- Die Dateien `main_add.rs`, `main_mem.rs`, `main_mul.rs` und `main_tests.rs` enthalten den Code für den Performancevergleich mit SymPy. Alle ausführbaren Programme wurden mit `cargo build --release` erzeugt.
- Die Bibliothek kann als Abhängigkeit in anderen, lokalen Rust-Projekten verwendet werden. Siehe https://users.rust-lang.org/t/solved-how-to-use-local-unpublished-crate/25738/3 bzw. https://stackoverflow.com/questions/33025887/how-to-use-a-local-unpublished-crate
- Die Bibliothek ist derzeit nicht auf https://crates.io/ veröffentlicht.