# Revue de codegenr

Classée par gravité, la plus grave d'abord. Rien n'est corrigé tant que l'humain n'a pas choisi l'item.
🟢 fait · ⚪ écarté · 🔴 à faire · 🟠 décision de l'humain nécessaire ou reporté

## Bugs visibles par l'utilisateur

1. 🟢 `codegenr/src/loaders/mod.rs:155` — une source mal formée ne donne que `Did not try all the file loaders.` : `DeserialisationError` exige les erreurs des 6 formats, aucune liste de `json_from_string` ne les essaie toutes, la vraie erreur de parsing est perdue.
   Corrigé : l'erreur nomme le document et donne une ligne par format essayé (`- Yaml: … at line 2 column 2`), test `unreadable_content_gives_the_error_of_each_format_tried`.
2. 🟢 `codegenr/src/processor/file.rs:68` — un `### FILE` qui ne peut s'écrire est seulement logué, et le code de sortie vaut 0 : l'écriture a lieu dans le `Drop` et l'erreur de `write_all` est jetée, la génération a l'air de réussir en CI.
   Même cause : une erreur au milieu du rendu laissait les fichiers ouverts écrits à moitié, et un `### FILE` jamais fermé était écrit sans rien dire.
   Corrigé : le fichier s'écrit dans `InstructionLineHandler::finish`, appelé sur `### /FILE`, et son erreur remonte avec le chemin (`ProcessorError::Write`) ; un fichier jamais fermé, ou rouvert avant d'être fermé, donne `MissingClosingTag` et n'est pas écrit ; un fichier fermé avant une erreur reste écrit (choix de l'humain) ; « File content is the same » passe de `warn` à `debug`.
20. 🟢 `codegenr/src/processor/clean.rs:26` — `### CLEAN` supprime hors du dossier `output` : `Path::join` garde tel quel un chemin absolu, et `..` remonte au-dessus.
   Mesuré : avec `-o out`, `### CLEAN ../victim/*` et `### CLEAN /…/victim/*` vident tous deux le dossier `victim` voisin, sans erreur.
   `### FILE` a la même cause (`make_path_from_root`, `filesystem.rs:9`) et écrit où on veut.
   Décision de l'humain : refuser un chemin qui sort de `output` pour `CLEAN` seulement, ou pour `FILE` aussi (des templates écrivent peut-être exprès dans un dossier voisin).
   Corrigé pour les deux, contre un template ou un document source malveillant (choix de l'humain) : `path_in_output` refuse un chemin absolu ou un `..` (`PathOutsideOutput`), tests `a_path_out_of_the_output_folder_is_refused` (3 cas), `an_absolute_path_is_refused` et `a_path_in_the_output_folder_is_accepted`, qui échouent sans la correction.
   Un template qui écrivait dans un dossier voisin doit maintenant prendre pour `output` leur dossier parent.
   Reste un `TO_HUMAN:` : un lien symbolique déjà présent dans `output` peut encore mener dehors, sa cible n'est pas vérifiée.
22. 🟢 `codegenr/src/loaders/document_path.rs:32` — les `$ref` d'un document source sont suivis partout : un chemin absolu lit n'importe quel fichier local (`Path::join` le garde tel quel), une URL est téléchargée, même depuis une spec distante.
   Une spec reçue d'un tiers peut ainsi faire entrer un fichier JSON ou YAML local (un kubeconfig…) dans le document résolu, puis dans le dump `intermediate` et les fichiers générés, ou faire appeler une adresse interne en CI (les métadonnées cloud).
   Les `..` ne peuvent pas être refusés comme à l'item 20 : l'autre projet en a 579 dans ses `$ref`, et certains remontent au-dessus de `_specs`.
   Décision de l'humain : refuser un `$ref` absolu (aucun dans l'autre projet), garder les relatifs sous le dossier courant (celui du `codegenr.toml`), et peut-être refuser un `$ref` URL depuis un fichier local (une spec locale peut référencer exprès une spec publique).
   Décision de l'humain : une configuration, chaque téléchargement doit être autorisé par l'utilisateur, toujours, et l'erreur doit le guider.
   Corrigé : `allowed_sources` dans une section (`--allowed-sources` pour `gen`), des urls et des dossiers ; `AllowedSources::check` passe avant chaque lecture, cache compris :
   un fichier sous le dossier courant, celui de `source` ou un dossier autorisé, une url seulement sous une url autorisée (même schéma, hôte et port, chemin par segments), la `source` comprise ;
   l'erreur donne le dossier ou l'url parente à ajouter, `allowed_sources = ['https://example.invalid/api/']`.
   Trouvé en route : le cache des documents résolus était partagé entre les sections, une section recevait ce qu'une autre avait autorisé ; il devient propre à chaque section (74 sections de l'autre projet : 1,86 s → 2,10 s).
   Tests : `an_url_is_allowed_only_under_an_allowed_url` (12 cas, dont `..` et `%2e%2e`), `an_url_not_allowed_is_not_downloaded`, `a_ref_to_a_file_out_of_the_allowed_folders_is_refused` (relatif et absolu), `a_section_does_not_get_what_another_section_allowed` ; les 4 derniers échouent sans la vérification.
   L'autre projet n'a aucune source en url et génère les mêmes fichiers ; reste le même `TO_HUMAN:` qu'à l'item 20 sur les liens symboliques.
3. 🟢 `codegenr/src/lib.rs:42` — `custom_helpers` et `global_parameters` sont obligatoires dans `codegenr.toml` (`missing field custom_helpers`), alors que le README dit `global_parameters` optionnel : il manque `#[serde(default)]`.
   Corrigé : les deux en `#[serde(default)]`, `custom_helpers` aussi dit « (Optional) » dans le README, test `custom_helpers_and_global_parameters_are_optional`.
4. 🟢 `codegenr/Cargo.toml:26` — la lib ne compile pas sans la feature `bin` (`default-features = false`) : `toml` n'est activé que par `bin`, le loader toml en a besoin.
   Corrigé : `toml` devient une dépendance ordinaire, à côté des autres formats ; une étape `cargo check -p codegenr --lib --no-default-features` des deux workflows empêche le retour du bug.
21. 🟢 `codegenr/src/loaders/mod.rs:82` — une source qui commence par un BOM UTF-8 ne se lit pas (`- Json: expected value at line 1 column 1`), alors que les éditeurs et outils Windows en mettent souvent : les `sample_swagger.json` et `schema.json` de `_samples/render/test1` à `test3` en ont un.
   Trouvé en rejouant les samples pour l'item 8 ; retirer le BOM avant d'essayer les formats, dans `json_from_string`.
   Pire que prévu : sans erreur, le yaml garde le BOM dans la première clé (`"\u{feff}a"`), et un json court, refusé par le loader json, est relu de travers par le loader yaml.
   Corrigé : `json_from_string` retire le BOM pour tous les formats, test `a_source_starting_with_a_bom_is_read` (4 cas, 3 échouent sans la correction, le toml acceptait déjà le BOM) ; aucune source de l'autre projet n'en a.
5. 🟢 `codegenr/src/loaders/document_path.rs:112` — `source = ""` fait paniquer `load_raw` (`unreachable!`) au lieu d'une erreur ; un `todo!()` reste aussi dans `loaders/mod.rs:150`.
   Corrigé : `load_raw` d'un chemin vide rend `LoaderError::EmptyPath` (`No document to load: its path is empty.`, code de sortie 1), test `loading_an_empty_path_is_an_error` ;
   le `todo!()` disparaît avec `FormatHint::NoIdea`, que `guess_format` remplace par `None` (`Option<FormatHint>`), ce qui évite d'avoir à traiter `NoIdea` dans `try_loaders`.
15. 🟢 `codegenr/src/processor/mod.rs:80` — les numéros de ligne des erreurs du processeur partent de 0 (`enumerate`) : `### NOPE` en 3ᵉ ligne est annoncé `Line 2`, et un `### FILE` en tête `opened line 0`.
   Corrigé : numérotées à partir de 1, comme les lignes du `<intermediate>.rendered.txt` dans un éditeur ; les tests de l'item 2 attendent maintenant `Line 3` et `opened line 1`.

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
18. 🟢 `README.md` — section des helpers : un exemple pour un texte sur plusieurs lignes dans un commentaire, avec `prefix_lines` (item 19), et une phrase dans la doc de `one_line` pour dire qu'il recolle les lignes sans espace, voulu pour les signatures (`PLAN.md` n°4).
   Fait : l'exemple du README, rejoué avec le binaire, suit l'exemple rhai ; la doc de `one_line` remplace celle d'un autre helper copiée par erreur (« Trim end of a block output… »), donne ses trois arguments et renvoie vers `prefix_lines`.
19. 🟢 `codegenr/src/helpers/strings.rs` — un texte sur plusieurs lignes dans un commentaire de code (`PLAN.md` n°3) : `/// {{replace (trim summary) "\n" "\n/// "}}` marche, mais les lignes suivantes perdent l'indentation du commentaire et une ligne vide garde l'espace du préfixe (`/// `).
   Fait à la demande de l'humain, alors que `PLAN.md` l'avait écarté côté code : le helper `prefix_lines`, `{{prefix_lines summary "  /// "}}` en début de ligne, met le préfixe et son indentation devant chaque ligne, retire les sauts de ligne de la fin et donne `///` sans espace à une ligne vide ; doc tests dans sa doc.

## Dépendances et sécurité

6. 🟢 `codegenr/Cargo.toml:48` — `reqwest` passe par native-tls, donc openssl sous Linux (8 CVE dans la dernière alerte) : rustls le supprimerait.
   Fait : `rustls-tls-native-roots`, qui garde les certificats du magasin de l'OS comme native-tls (une autorité d'entreprise reste reconnue), et les autres features par défaut de `reqwest` (`charset`, `http2`, `macos-system-configuration`).
   `openssl`, `native-tls` et `hyper-tls` sortent du `Cargo.lock` ; rustls passe par `ring`, sans `aws-lc` qui demanderait cmake en CI Windows.
   Vérifié : `read_beezup_openapi` (ignoré, réseau) passe, et le binaire lit l'openapi de `petstore3.swagger.io` en https.
7. 🟢 `codegenr/Cargo.toml:72` — les features par défaut de `handlebars_misc_helpers` laissent les templates faire des requêtes HTTP (attohttpc) et compilent jsonnet en C++ : les réduire peut casser des templates existants.
   Plus grave que prévu : `env_var` et `read_to_str` sont toujours enregistrés, un template pouvait lire un secret et l'envoyer par `http_get` pendant la génération, sans laisser de trace.
   Corrigé (choix de l'humain) : features `string`, `json`, `regex` et `uuid` seulement ; `http_get`, `gitignore_io` et `jsonnet` disparaissent, aucun template connu ne s'en servait.
   Test `a_template_cannot_reach_the_network_nor_run_jsonnet` (3 cas, échoue sans la correction) ; 10 crates sortent du `Cargo.lock` (`attohttpc`, `jsonnet-sys` et sa lib C++, `webpki-roots`…), et la licence `CDLA-Permissive-2.0` de `deny.toml`.
   Gardés : `env_var` et `read_to_str`, qui ne peuvent plus que recopier un secret dans un fichier généré, visible dans le diff ; le README dit de n'utiliser que des templates de confiance.
8. 🟢 `codegenr/Cargo.toml:62` — la régression de handlebars 6 n'est figée que par un commentaire : un test `upstream_bug_handlebars_…` dirait quand la mise à jour devient possible.
   Fait : pas un `upstream_bug_…`, qui ne peut tourner que sur la version boguée, mais un test ordinaire, `partials_called_from_an_each_see_its_variables`, qui garde toute mise à jour.
   Mesuré : sur 5.1.2 ses 8 cas passent ; sur 6.3.2, la version de la régression, `@key`, `@index`, `@first` et `@last` sont perdus dans le partial (4 cas échouent), corrigé par 6.4.2 (#758 de `sunng87/handlebars-rust`).
   Mais 6.4.4 (avec `handlebars_misc_helpers` 0.17.0) ne suffit pas : ce test et toute la suite passaient, et pourtant les 74 sections de l'autre projet, rejouées sur une copie, changent 21 fichiers générés.
   Des propriétés `required` deviennent `Option<…>`, et des champs GraphQL perdent leur `!` : `is_oapi3_property_required @key ../required` dans un partial appelé depuis `#each properties`, `../` y est perdu.
   Deux cas `dot dot` ajoutés au test le montrent (ils échouent sur 6.4.4).
   Puis : ce `../` perdu n'est pas un bug, handlebars.js 4.7.8 fait pareil (seul son mode `compat` remonte au contexte de l'appelant), c'est la 5.1.2 qui s'en écartait.
   Monté en 6.4.4, avec `handlebars_misc_helpers` 0.17.0 : les cas `dot dot` disent maintenant que `../` est vide dans le partial, et deux cas montrent le paramètre de partial qui le remplace (`{{> part required=../required}}`).
   L'autre projet passe désormais les valeurs du parent en paramètres de partial (8 templates) : ses 74 sections génèrent les mêmes fichiers en 5.1.2 et en 6.4.4.
9. 🟢 `codegenr/Cargo.toml:62` — `serde_yaml` 0.8 n'est plus maintenu (yaml-rust) : la 0.9 avec `Deserializer::from_str` (itération des documents) ou un fork maintenu, à évaluer avec un test.
   Mesuré : le « more than one document » de la 0.9 venait du BOM UTF-8, que libyaml prend pour un second document ; les 6 samples refusés en ont un, aucun n'a de `---`, et la 0.8 refuse aussi un vrai fichier à deux documents.
   BOM retiré (item 21), la 0.9 lit les 13 samples comme la 0.8, et 38 des 39 yaml de l'autre projet ; le dernier écrit `False`, booléen en 0.9 (YAML 1.2) au lieu d'une chaîne, et n'est la source d'aucune section.
   Passé en 0.9.34 (choix de l'humain : sa dernière version, dépréciée en amont mais déjà compilée pour `handlebars_misc_helpers`, quand un fork serait une nouvelle dépendance) ;
   `yaml-rust` et son advisory ignorée sortent, avec les doublons `serde_yaml`, `indexmap` et `hashbrown` ; seul le texte d'une erreur yaml change dans `unreadable_content_gives_the_error_of_each_format_tried` ; l'autre projet génère les mêmes fichiers.

## CI et qualité

10. 🟢 `.github/workflows/dev.yaml:32` — 1 warning clippy à corriger (8 avant les items 1 et 2), puis `clippy -D warnings` et `fmt --check` en CI ; `actions/checkout` v2 → v7.
   Corrigé : le warning `for_kv_map` de `render.rs:63` (`partials.values()`) ; un job `lint` dans les deux workflows (`cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`), dont dépend maintenant `publish` ; `actions/checkout` en v7 (sa seule rupture touche `pull_request_target`, qu'on n'utilise pas).
   Vérifié en local : les deux commandes passent, avec et sans `--all-features`, et les deux YAML se lisent ; la CI elle-même ne tournera qu'au prochain push sur `dev` ou `main`.
11. 🟢 `codegenr/src/processor/mod.rs:262` — `process_test` est `#[ignore]` : il écrit dans le dossier courant, un `tempfile` le rendrait exécutable.
   Corrigé : renommé `instructions_are_read_whatever_their_case_and_spaces`, il écrit dans un `TempDir` et vérifie ses deux fichiers (`###/ console` et `### / FILE` ferment bien) ; la ligne `####` passe dans un `FILE`, pour vérifier qu'elle y reste du contenu.

## Documentation

12. 🟢 `README.md:380` — `__myhelper__.rhai` ne donne pas le helper `myhelper` : c'est le nom du fichier sans `.rhai` qui compte ; `### CLEAN` n'est pas documenté dans la partie Process (`README.md:450`).
   Corrigé : `myhelper.rhai`, cherché aussi dans les sous-dossiers ; la partie Process documente `### CLEAN <glob>` avec un exemple, rejoué avec le binaire (les anciens `*.generated.rs` sont supprimés, `keep.txt` reste).
   C'est ce test qui a fait trouver l'item 20.

## Plus tard

13. 🟢 `codegenr/src/bin/main.rs:5` — pas de réglage de verbosité (le « Verbose/Tracing mode » de la roadmap), édition 2024, mode watch.
   Édition 2024 faite (choix de l'humain) : `cargo fix --edition` ne change aucune ligne, ses deux avertissements d'ordre de `drop` (`lib.rs:56`, `processor/mod.rs:172`) sont sans effet, codegenr n'a aucun `impl Drop` ;
   rustfmt 2024 retrie les imports de 5 fichiers, le workspace passe au `resolver = "3"` (sans changer le `Cargo.lock`), les doctests tournent en un seul binaire (5 s → 0,1 s) ; il faut Rust 1.85 pour compiler.
   Restent : la verbosité (`fmt::init()` sans la feature `env-filter` ignore `RUST_LOG`, le `RUST_LOG: debug` de la CI ne sert à rien) et le mode watch (nouvelle dépendance `notify`).
   Verbosité faite (choix de l'humain) : `-v` (debug), `-vv` (trace, avec les arguments des fonctions `#[instrument]`) et `-q` (warn), globaux, lus avant d'installer `tracing_subscriber` ;
   tests `the_log_level_follows_the_flags` (6 cas), `quiet_and_verbose_together_is_an_error` et `the_command_line_is_well_formed` (`debug_assert` de clap) ; deux lignes dans le README. Reste le mode watch.
   Mode watch fait (choix de l'humain : pour le développement des templates, donc seulement les templates et les custom helpers) : `codegenr -w`, nouvelle dépendance `notify` 9.0.0 sous la feature `bin` ;
   `run_codegenr` se coupe en `resolve_section` et `render_section` : les sources sont résolues une fois, une section dont un `.hbs` ou un `.rhai` change est rendue à nouveau ; sans les événements d'accès (le rendu lit les templates), anti-rebond de 200 ms à la main ;
   un fichier généré, même dans un dossier surveillé, ne relance rien ; une erreur de rendu est affichée et la surveillance continue.
   Tests `a_change_concerns_the_templates_and_helpers_of_a_section` (8 cas), `a_section_renders_again_from_its_resolved_source` (la source supprimée n'est pas relue), `the_watch_flag_is_read` ;
   essayé à la main (template, helper, erreur puis correction), puis sur l'autre projet : un partial de `axum-api` rend 11 sections en 0,3 s, un de `rust/common` 66 sections en 1,6 s, contre 2,1 s pour tout relancer.
14. ⚪ `codegenr/src/lib.rs:49` — le cache de documents est partagé entre les sections d'un même lancement : si une section prend en source le résultat d'une autre (idée de la roadmap), le fichier régénéré serait relu dans son ancienne version.
   Depuis l'item 22, seul le cache des documents originaux reste partagé, celui des documents résolus est propre à chaque section ; le problème reste pour les originaux.
   Écarté pour l'instant : aucune section ne lit la sortie d'une autre, et le mode watch ne relit pas les sources ; à rouvrir avec l'idée de la roadmap (oublier un fichier réécrit par `### FILE`, ordonner les sections par dépendance).

## Revue du mode watch (`8633114`, `/code-review`)

Classée par gravité ; les 🔴 sont de petites corrections, les 🟠 demandent un choix.

23. 🟢 `codegenr/src/render.rs:123` — un fichier caché qui finit par `.hbs`, comme le verrou d'Emacs `.#_partial.hbs`, est pris pour un template principal : `TwoMainTemp` tant qu'un buffer n'est pas enregistré, et le watch rend justement à ce moment-là ; une génération normale casse aussi s'il en traîne un. Ignorer les fichiers qui commencent par `.`.
   Corrigé : `get_templates_from_directory` saute les fichiers et dossiers cachés (sauf le dossier donné, qui peut être `.`), `custom::handlebars_setup` aussi (`require_literal_leading_dot`) ; tests `hidden_files_and_folders_are_not_templates` et `hidden_helper_files_are_not_loaded`, qui échouaient avant ; essayé : un `.#main.hbs` laissé dans le dossier ne casse plus le rendu.
24. 🟢 `codegenr/src/watch.rs:120` — une resynchronisation de notify (`EventKind::Other` avec `Flag::Rescan`, quand des événements sont perdus : file inotify pleine, `MustScanSubDirs`, tampon Windows) est jetée sans un mot, par exemple après un `git checkout` ; il faudrait tout rendre à nouveau, avec un avertissement.
   Corrigé : `need_rescan` met `lost` dans les changements, toutes les sections sont rendues à nouveau avec un avertissement ; test `lost_events_render_every_section`.
25. 🟢 `codegenr/src/watch.rs:130` — déplacer un dossier de partials dans un dossier surveillé, ou l'en sortir, donne un seul événement sur le dossier, sans extension : rien n'est rendu à nouveau.
   Sous Linux, un dossier de templates supprimé puis recréé perd aussi sa surveillance, qui n'est jamais remise.
   Corrigé : un chemin sans extension sous un dossier surveillé (un dossier créé, déplacé ou supprimé) relance la section ; essayé sur macOS, `Create(Folder)` puis rendu 210 ms après. Reste, pour l'item 37 : sous Linux, un dossier racine supprimé puis recréé perd sa surveillance.
26. 🟢 `codegenr/src/watch.rs:105` — n'importe quel événement relance l'attente de 200 ms, même sur un fichier qui n'est pas un template : une activité continue dans un dossier surveillé (`templates = ["."]` avec un `target/`) repousse le rendu sans fin. Seuls les `.hbs` et `.rhai` devraient ouvrir ou prolonger l'attente.
   Corrigé : seuls les chemins d'une section ouvrent l'attente, qui a une échéance fixe ; tests `only_the_relevant_files_make_a_change` et `other_files_do_not_delay_the_rendering` (un autre fichier écrit toutes les 20 ms pendant 3 s ne retarde plus le rendu).
27. 🟢 `codegenr/src/watch.rs:56` — la surveillance commence après la première génération complète (2 s pour l'autre projet) : un template enregistré pendant ce temps n'est jamais vu. Créer le watcher avant le premier rendu.
   Corrigé : le watcher est créé avant le premier rendu, un événement arrivé pendant ce rendu attend dans le canal.
28. 🟢 `codegenr/src/watch.rs:41` — un dossier de templates ou un helper introuvable au démarrage (faute de frappe, dossier pas encore créé) est ignoré sans avertissement, donc jamais surveillé.
   Corrigé : un avertissement nomme le chemin et la section.
29. 🟢 `codegenr/src/watch.rs:61` — sans aucune section à surveiller (toutes les sources en erreur, ou aucune section), le watch annonce « 0 section(s) » et attend indéfiniment, au lieu de sortir en erreur.
   Corrigé : `WatchError::NothingToWatch`, test `nothing_to_watch_is_an_error` (aucune section, ou une source introuvable).
30. 🟢 `codegenr/src/errors.rs:29` — `Watch` et `WatchStopped` n'existent dans `CodegenrError` que sous la feature `bin` : un `match` exhaustif d'un utilisateur de la bibliothèque casse dès qu'une autre crate du même build active `bin`, et `notify` apparaît dans l'API de la bibliothèque. Garder ces erreurs dans `watch.rs`, ou côté binaire.
   Corrigé : `WatchError` dans `watch.rs`, `CodegenrError` ne change plus avec la feature `bin`.
31. 🟢 `codegenr/Cargo.toml:36` — `notify` 9.0.0 demande Rust 1.91, contre 1.85 pour l'édition 2024 : `cargo install codegenr` casse sur un Rust 1.85 à 1.90 sans prévenir. Le déclarer dans `rust-version`.
   Corrigé : `rust-version = "1.91"` dans `codegenr/Cargo.toml`, et le README le dit à l'installation.
32. 🟢 `codegenr/src/watch.rs:30` — le cache des documents originaux reste en mémoire pendant toute la surveillance alors que plus rien ne s'en sert : le libérer après la première génération.
   Corrigé : `resolve_sections` garde le cache pour lui, il est libéré avant la surveillance.
33. 🟢 `AGENTS.md:87` — la description du pipeline cite encore `run_codegenr`, qui n'existe plus (`resolve_section` puis `render_section`).
   Corrigé.
34. 🟢 `README.md:22` — première occurrence de « templates » dans le README, sans sa traduction (*gabarit*) demandée par AGENTS.md.
   Corrigé : templates (*gabarits*).
35. ⚪ `codegenr/src/watch.rs:128` — le commentaire promet qu'un fichier généré ne relance jamais de rendu, mais un `.hbs` ou un `.rhai` généré dans un dossier surveillé le fait, et avec `### CLEAN` il boucle sans fin. Corriger seulement le commentaire, ou ignorer les fichiers que codegenr vient lui-même d'écrire ?
   Écarté (choix de l'humain) : un cas limite, celui qui génère des templates dans un dossier surveillé verra bien que ça ne s'arrête pas ; seul le commentaire de `concerns` est corrigé, il dit maintenant cette limite.
36. ⚪ `codegenr/src/filesystem.rs:22` — un Ctrl-C pendant un rendu peut laisser un fichier généré tronqué (`File::create` vide le fichier, puis `write_all`), ce qui devient plus probable avec le watch. Écrire dans un fichier temporaire puis le renommer servirait aussi aux générations normales.
   Écarté (choix de l'humain) : un Ctrl-C pendant l'écriture est peu probable.
37. 🟢 `codegenr/src/watch.rs:130` — le watch devine par l'extension les fichiers dont dépend une section, sans passer par ce que lit vraiment `render_section` : un helper listé avec une autre extension (`custom_helpers = ["upper.script"]`) n'est jamais surveillé. Faire renvoyer à `get_templates_from_directory` et `custom::handlebars_setup` les fichiers qu'ils lisent, et surveiller ceux-là.
   Décision de l'humain : le watch surveille les mêmes fichiers que la génération, sources comprises ; idée : relever pendant le premier passage les fichiers réellement lus (sources, fichiers des `$ref`, templates, helpers) et ne surveiller que ceux-là. À concevoir ensemble.
   Fait : les dossiers de templates et de custom helpers du toml, comme avant ; les fichiers que la source et ses `$ref` ont lus, que `AllowedSources` relève même quand la résolution échoue,
   pour qu'une source cassée reparte une fois réparée ; une source ou un `$ref` en url n'est pas surveillé (une ligne de log) ; `codegenr.toml` modifié relance tout depuis zéro, et cassé, le watch attend qu'il soit réparé.
   Un fichier lu est surveillé par son dossier, sans les sous-dossiers ; un `$ref` ajouté fait surveiller son fichier après la nouvelle résolution.
   Tests : `the_documents_read_are_recorded` (fichier manquant compris), `changes_plan_what_to_do_again` (6 cas), `a_missing_file_is_compared_through_its_folder` ;
   essayé à la main (un `$ref` modifié, une source cassée puis réparée, un toml modifié, cassé puis réparé), puis sur l'autre projet : `framework_core.yaml` modifié relit et rend ses 50 sections en 1,6 s.
