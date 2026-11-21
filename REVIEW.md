# Revue de codegenr

Classée par gravité, la plus grave d'abord. Rien n'est corrigé tant que l'humain n'a pas choisi l'item.
🟢 fait · ⚪ écarté · 🔴 à faire · 🟠 décision de l'humain nécessaire ou reporté

## Bugs visibles par l'utilisateur

1. 🟢 `codegenr/src/loaders/mod.rs:155` — une source mal formée ne donne que `Did not try all the file loaders.` : `DeserialisationError` exige les erreurs des 6 formats, aucune liste de `json_from_string` ne les essaie toutes, la vraie erreur de parsing est perdue.
   Corrigé : l'erreur nomme le document et donne une ligne par format essayé (`- Yaml: … at line 2 column 2`), test `unreadable_content_gives_the_error_of_each_format_tried`.
2. 🔴 `codegenr/src/processor/file.rs:68` — un `### FILE` qui ne peut s'écrire est seulement logué, et le code de sortie vaut 0 : l'écriture a lieu dans le `Drop` et l'erreur de `write_all` est jetée, la génération a l'air de réussir en CI.
3. 🔴 `codegenr/src/lib.rs:42` — `custom_helpers` et `global_parameters` sont obligatoires dans `codegenr.toml` (`missing field custom_helpers`), alors que le README dit `global_parameters` optionnel : il manque `#[serde(default)]`.
4. 🔴 `codegenr/Cargo.toml:26` — la lib ne compile pas sans la feature `bin` (`default-features = false`) : `toml` n'est activé que par `bin`, le loader toml en a besoin.
5. 🔴 `codegenr/src/loaders/document_path.rs:112` — `source = ""` fait paniquer `load_raw` (`unreachable!`) au lieu d'une erreur ; un `todo!()` reste aussi dans `loaders/mod.rs:150`.

## Dépendances et sécurité

6. 🔴 `codegenr/Cargo.toml:48` — `reqwest` passe par native-tls, donc openssl sous Linux (8 CVE dans la dernière alerte) : rustls le supprimerait.
7. 🟠 `codegenr/Cargo.toml:63` — les features par défaut de `handlebars_misc_helpers` laissent les templates faire des requêtes HTTP (attohttpc) et compilent jsonnet en C++ : les réduire peut casser des templates existants.
8. 🔴 `codegenr/Cargo.toml:62` — la régression de handlebars 6 n'est figée que par un commentaire : un test `upstream_bug_handlebars_…` dirait quand la mise à jour devient possible.
9. 🟠 `codegenr/Cargo.toml:54` — `serde_yaml` 0.8 n'est plus maintenu (yaml-rust) : la 0.9 avec `Deserializer::from_str` (itération des documents) ou un fork maintenu, à évaluer avec un test.

## CI et qualité

10. 🔴 `.github/workflows/dev.yaml:32` — 2 warnings clippy à corriger (8 avant l'item 1), puis `clippy -D warnings` et `fmt --check` en CI ; `actions/checkout` v2 → v7.
11. 🔴 `codegenr/src/processor/mod.rs:122` — `process_test` est `#[ignore]` : il écrit dans le dossier courant, un `tempfile` le rendrait exécutable.

## Documentation

12. 🔴 `README.md:380` — `__myhelper__.rhai` ne donne pas le helper `myhelper` : c'est le nom du fichier sans `.rhai` qui compte ; `### CLEAN` n'est pas documenté dans la partie Process (`README.md:450`).

## Plus tard

13. 🟠 `codegenr/src/bin/main.rs:5` — pas de réglage de verbosité (le « Verbose/Tracing mode » de la roadmap), édition 2024, mode watch.
14. 🟠 `codegenr/src/lib.rs:49` — le cache de documents est partagé entre les sections d'un même lancement : si une section prend en source le résultat d'une autre (idée de la roadmap), le fichier régénéré serait relu dans son ancienne version.
