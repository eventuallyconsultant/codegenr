# Besoins de wymmo

Ce que la branche `wdev` de wymmo a buté sur codegenr, le 2026-11-21, avec la branche `new` (`6fb2cae`) installée par mise (`"cargo:https://github.com/eventuallyconsultant/codegenr/" = "branch:new"`).
Chaque besoin donne le constat mesuré, ce qui est attendu et comment le vérifier. Ce fichier est un plan de travail : il est supprimé une fois les items faits ou écartés.
🟢 fait · ⚪ écarté · 🔴 à faire · 🟠 décision de l'humain nécessaire ou reporté

## Ce que la branche `new` a déjà passé

🟢 Sur les 74 sections du `codegenr.toml` de wymmo, la branche `new` génère exactement les mêmes fichiers que `dev` : `git status` vide après `just codegen`, lint des specs et specs publiques de wdev inchangés.

## 1. 🔴 Le log d'un fichier inchangé est un `WARN`

- **Constat** : `codegenr/src/processor/file.rs:59` logue `File content is the same, not writing it again : <chemin>` au niveau `warn`.
  Sur un `just codegen` de wymmo, c'est 23 lignes `WARN` pour un lancement où tout est normal : un vrai avertissement s'y noie.
- **Attendu** : `debug`, puisque ne pas réécrire un fichier identique est le comportement voulu (sa date de modification est gardée, cargo ne recompile rien).
  Au besoin, une ligne `info` par section qui compte les fichiers écrits et inchangés.
- **Vérification** : un lancement sans fichier modifié n'affiche plus aucun `WARN`.

## 2. 🔴 Un identifiant tiré d'un texte accentué n'est ni ASCII ni découpé aux bons endroits

wymmo nomme les variantes et les constructeurs des réponses d'après leur `description` (`description: Not found` donne `NotFound404` et `not_found()`).
Les APIs de l'État que wymmo reproduit décrivent leurs réponses en français, et wymmo veut reprendre ces textes tels quels.

- **Constat**, avec les templates `axum-api` de wymmo (`{{to_pascal_case description}}{{@key}}` et `{{to_snake_case description}}`) :

  | `description` | `to_pascal_case` | `to_snake_case` |
  | --- | --- | --- |
  | `Erreur. Requête mal formée` | `ErreurRequêteMalFormée` | `erreur_requ_ête_mal_form_ée` |
  | `Liste de communes associées et/ou déléguées` | `ListeDeCommunesAssociéesEtOuDéléguées` | `liste_de_communes_associ_ées_et_ou_d_él_égu_ées` |
  | `Liste des communes de l'EPCI` | `ListeDesCommunesDeLEPCI` | `liste_des_communes_de_l_epci` |
  | `Échec du géocodage (fichier CSV)` | `ÉchecDuGéocodageFichierCSV` | |
  | `Œuvre introuvable` | `ŒuvreIntrouvable` | |

  Deux défauts distincts :
  - les lettres accentuées restent dans les identifiants. Le Rust les accepte, mais un nom de fonction qu'on ne tape pas au clavier n'est pas utilisable ;
  - `to_snake_case` coupe le mot avant et après chaque lettre accentuée (`requ_ête`), comme s'il la prenait pour une majuscule.
    Les conversions viennent de `handlebars_misc_helpers` 0.16.3, qui s'appuie sur `cruet` 0.14 (`Cargo.lock`) : à confirmer dans leur source, c'est probablement un bug amont.
  La ponctuation est déjà bien retirée (`l'EPCI`, `et/ou`, les parenthèses).
- **Attendu** : un helper de translittération en ASCII, `to_ascii` par exemple, à composer avec les conversions existantes : `{{to_snake_case (to_ascii description)}}`.
  - `é è ê ë` → `e`, `à â` → `a`, `ç` → `c`, `î ï` → `i`, `ô` → `o`, `ù û ü` → `u`, `œ` → `oe`, `æ` → `ae`, les mêmes en majuscules (`É` → `E`, `Œ` → `Oe`).
  - Les conversions existantes ne changent pas : les templates qui ne l'appellent pas génèrent la même chose qu'aujourd'hui.
  - Attendus du tableau : `erreur_requete_mal_formee` / `ErreurRequeteMalFormee`, `liste_de_communes_associees_et_ou_deleguees`, `EchecDuGeocodageFichierCSV`, `OeuvreIntrouvable`.
- 🟠 **Décision de l'humain** : une dépendance (`deunicode` est la crate établie pour ça, sans dépendance elle-même) ou une petite table écrite à la main pour le latin du français.
- **Tests** :
  - un doc test par cas du tableau, dans le style de `helpers/cases.rs` (`exec_template(json!(…), "{{to_ascii value}}")`) ;
  - un test `upstream_bug_cruet_snake_case_splits_accented_words` qui fige la coupure actuelle (`to_snake_case "Requête"` donne `requ_ête`), avec un commentaire qui dit le comportement attendu et l'issue amont, ou qu'il n'y en a pas encore.
    Il échouera le jour où l'amont corrige, ce qui dira de le transformer en test ordinaire.

## 3. ⚪ Les textes sur plusieurs lignes dans un commentaire : rien à faire dans codegenr

- **Constat** : un template qui écrit `/// {{summary}}` laisse la deuxième ligne d'un `summary` sur plusieurs lignes hors du commentaire, et le Rust généré ne compile plus.
- **Écarté** : les helpers actuels suffisent. Mesuré sur la branche `new` : `/// {{replace (trim summary) "\n" "\n/// "}}` donne bien une ligne `/// ` par ligne du texte, et `trim` retire le saut de ligne final d'un bloc YAML `|`.
  La correction est dans les templates de wymmo.
- 🔴 **Documentation** : cet usage mérite un exemple dans le README, section des helpers, car il n'a rien d'évident (le `"\n"` d'un littéral de template est bien un saut de ligne).

## 4. ⚪ `one_line` colle les lignes sans espace

- **Constat** : `{{#one_line}}Recherche,\nsur deux lignes{{/one_line}}` donne `Recherche,sur deux lignes`.
- **Écarté** : c'est voulu, les templates s'en servent pour recoller une signature écrite sur plusieurs lignes. Pour un texte, l'item 3 suffit.
  Au plus, une phrase dans sa doc pour le dire.

## 5. 🟠 `codegenr gen --help` n'affiche pas ses options

- **Constat** : `codegenr gen --help` n'affiche que `Executes one generation from command line parameters.`, sans les options que l'erreur d'usage montre (`codegenr gen [OPTIONS] --output <output> --source <source>`).
- **Attendu** : la liste des options, chacune avec une ligne d'aide. À voir avec l'item 13 de `REVIEW.md` (la verbosité, la CLI).

## Côté wymmo, une fois ces items livrés

Pour mémoire, rien à faire dans ce dépôt :
- les templates de wymmo passent leurs doc comments par `replace (trim …)` (item 228 de la `REVIEW.md` de wymmo, qui n'attend rien de codegenr) ;
- une fois `to_ascii` livré, les noms des réponses de geodata pourront reprendre les descriptions de geo.api.gouv.fr (item 225).
