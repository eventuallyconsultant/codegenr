# Revue de codegenr

Classée par gravité, la plus grave d'abord. Rien n'est corrigé tant que l'humain n'a pas choisi l'item.
🟢 fait · ⚪ écarté · 🔴 à faire · 🟠 décision de l'humain nécessaire ou reporté

## Bugs visibles par l'utilisateur

1. 🟢 `codegenr/src/loaders/mod.rs:155` — une source mal formée ne donne que `Did not try all the file loaders.` : `DeserialisationError` exige les erreurs des 6 formats, aucune liste de `json_from_string` ne les essaie toutes, la vraie erreur de parsing est perdue.
   Corrigé : l'erreur nomme le document et donne une ligne par format essayé (`- Yaml: … at line 2 column 2`), test `unreadable_content_gives_the_error_of_each_format_tried`.
2. 🟢 `codegenr/src/processor/file.rs:68` — un `### FILE` qui ne peut s'écrire est seulement logué, et le code de sortie vaut 0 : l'écriture a lieu dans le `Drop` et l'erreur de `write_all` est jetée, la génération a l'air de réussir en CI.
   Même cause : une erreur au milieu du rendu laissait les fichiers ouverts écrits à moitié, et un `### FILE` jamais fermé était écrit sans rien dire.
   Corrigé : le fichier s'écrit dans `InstructionLineHandler::finish`, appelé sur `### /FILE`, et son erreur remonte avec le chemin (`ProcessorError::Write`) ; un fichier jamais fermé, ou rouvert avant d'être fermé, donne `MissingClosingTag` et n'est pas écrit ; un fichier fermé avant une erreur reste écrit (choix de l'humain) ; « File content is the same » passe de `warn` à `debug`.
3. 🔴 `codegenr/src/lib.rs:42` — `custom_helpers` et `global_parameters` sont obligatoires dans `codegenr.toml` (`missing field custom_helpers`), alors que le README dit `global_parameters` optionnel : il manque `#[serde(default)]`.
4. 🔴 `codegenr/Cargo.toml:26` — la lib ne compile pas sans la feature `bin` (`default-features = false`) : `toml` n'est activé que par `bin`, le loader toml en a besoin.
5. 🔴 `codegenr/src/loaders/document_path.rs:112` — `source = ""` fait paniquer `load_raw` (`unreachable!`) au lieu d'une erreur ; un `todo!()` reste aussi dans `loaders/mod.rs:150`.
15. 🔴 `codegenr/src/processor/mod.rs:80` — les numéros de ligne des erreurs du processeur partent de 0 (`enumerate`) : `### NOPE` en 3ᵉ ligne est annoncé `Line 2`, et un `### FILE` en tête `opened line 0`.

## Besoins remontés par un projet qui utilise codegenr (`PLAN.md`)

Mesurés sur ses 74 sections : la branche `new` génère exactement les mêmes fichiers que `dev`.

- 🟢 `PLAN.md` n°1, le `WARN` « File content is the same » pour chaque fichier inchangé : passé en `debug` avec l'item 2.
- 🟢 `PLAN.md` n°5, `codegenr gen --help` sans ses options : c'était structopt, corrigé par la migration vers clap (`ec614db`). Revérifié avec le binaire `branch-new` que mise installe chez ce projet : les options s'affichent.

16. 🟢 `codegenr/src/processor/mod.rs:75` — rien ne dit ce qu'une section a fait depuis que le `WARN` est passé en `debug` : une ligne `info` par section, avec le nombre de fichiers écrits et inchangés (`PLAN.md` n°1, « au besoin »).
   Fait : `process` rend un `ProcessReport`, et chaque section logue `2 file(s) written, 1 unchanged, in <output>.` en `info`, test `process_reports_the_files_written_and_unchanged`.
17. 🟢 `codegenr/src/helpers/cases.rs` (où irait le helper, à côté de `uppercase_first_letter`) — un identifiant tiré d'un texte accentué garde ses accents (`to_pascal_case "Œuvre introuvable"` → `ŒuvreIntrouvable`), et `to_snake_case` coupe autour de chaque lettre accentuée (`erreur_requ_ête_mal_form_ée`) et ne met pas en minuscule une capitale accentuée en tête (`Échec_du_g_éocodage_fichier_csv`) (`PLAN.md` n°2).
   Attendu : un helper `to_ascii` à composer, `{{to_snake_case (to_ascii description)}}` → `erreur_requete_mal_formee`, sans changer les conversions existantes ; aucun helper actuel ne le fait (pas de `to_slug` enregistré).
   Fait avec `deunicode` (choix de l'humain), plus `Œ`/`Æ` → `Oe`/`Ae` devant une minuscule pour garder `OeuvreIntrouvable` ; un doc test par cas du tableau de `PLAN.md`.
   Le bug amont est figé par `upstream_bug_cruet_snake_case_splits_accented_words` : `char_is_uppercase` de `cruet` vaut `c == c.to_ascii_uppercase()`, vrai pour toute lettre hors ascii, et les lettres sont baissées par `to_ascii_lowercase` ;
   toujours là sur la branche principale de `cruet` (1.0.0), aucune issue ouverte chez `taidge/cruet` au 2026-11-21.
18. 🔴 `README.md` — section des helpers : un exemple pour un texte sur plusieurs lignes dans un commentaire, `/// {{replace (trim summary) "\n" "\n/// "}}` (le `"\n"` d'un littéral de template est bien un saut de ligne), et une phrase dans la doc de `one_line` pour dire qu'il recolle les lignes sans espace, voulu pour les signatures (`PLAN.md` n°3 et n°4, écartés côté code).

## Dépendances et sécurité

6. 🔴 `codegenr/Cargo.toml:48` — `reqwest` passe par native-tls, donc openssl sous Linux (8 CVE dans la dernière alerte) : rustls le supprimerait.
7. 🟠 `codegenr/Cargo.toml:63` — les features par défaut de `handlebars_misc_helpers` laissent les templates faire des requêtes HTTP (attohttpc) et compilent jsonnet en C++ : les réduire peut casser des templates existants.
8. 🔴 `codegenr/Cargo.toml:62` — la régression de handlebars 6 n'est figée que par un commentaire : un test `upstream_bug_handlebars_…` dirait quand la mise à jour devient possible.
9. 🟠 `codegenr/Cargo.toml:54` — `serde_yaml` 0.8 n'est plus maintenu (yaml-rust) : la 0.9 avec `Deserializer::from_str` (itération des documents) ou un fork maintenu, à évaluer avec un test.

## CI et qualité

10. 🔴 `.github/workflows/dev.yaml:32` — 1 warning clippy à corriger (8 avant les items 1 et 2), puis `clippy -D warnings` et `fmt --check` en CI ; `actions/checkout` v2 → v7.
11. 🔴 `codegenr/src/processor/mod.rs:122` — `process_test` est `#[ignore]` : il écrit dans le dossier courant, un `tempfile` le rendrait exécutable.

## Documentation

12. 🔴 `README.md:380` — `__myhelper__.rhai` ne donne pas le helper `myhelper` : c'est le nom du fichier sans `.rhai` qui compte ; `### CLEAN` n'est pas documenté dans la partie Process (`README.md:450`).

## Plus tard

13. 🟠 `codegenr/src/bin/main.rs:5` — pas de réglage de verbosité (le « Verbose/Tracing mode » de la roadmap), édition 2024, mode watch.
14. 🟠 `codegenr/src/lib.rs:49` — le cache de documents est partagé entre les sections d'un même lancement : si une section prend en source le résultat d'une autre (idée de la roadmap), le fichier régénéré serait relu dans son ancienne version.
