use crate::helpers::handlebars_ext::HandlebarsExt;
use crate::helpers::string_ext::StringExt;
use handlebars::HelperDef;
use serde_json::Value;

pub const UPPERCASE_FIRST_LETTER_HELPER: &str = "uppercase_first_letter";
pub const LOWERCASE_FIRST_LETTER_HELPER: &str = "lowercase_first_letter";
pub const TO_ASCII_HELPER: &str = "to_ascii";

/// Returns a string with the first letter in Uppercase
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "tEsT" }), "{{uppercase_first_letter value}}"),
///   "TEsT"
/// );
/// ```
pub struct UppercaseFirstLetterHelper;

impl HelperDef for UppercaseFirstLetterHelper {
  // fn call_inner<'reg: 'rc, 'rc>(
  //   &self,
  //   h: &handlebars::Helper<'rc>,
  //   _: &'reg handlebars::Handlebars<'reg>,
  //   _: &'rc handlebars::Context,
  //   _: &mut handlebars::RenderContext<'reg, 'rc>,
  // ) -> Result<handlebars::ScopedJson<'rc>, handlebars::RenderError> {
  // }

  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'rc>, handlebars::RenderError> {
    h.ensure_arguments_count(1, UPPERCASE_FIRST_LETTER_HELPER)?;
    let to_case = h.get_param_as_str_or_fail(0, UPPERCASE_FIRST_LETTER_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(to_case.uppercase_first_letter())))
  }
}

/// Returns a string with the first letter in Lowercase
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "TEST" }), "{{lowercase_first_letter value}}"),
///   "tEST"
/// );
/// ```
pub struct LowercaseFirstLetterHelper;

impl HelperDef for LowercaseFirstLetterHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'rc>, handlebars::RenderError> {
    h.ensure_arguments_count(1, LOWERCASE_FIRST_LETTER_HELPER)?;
    let to_case = h.get_param_as_str_or_fail(0, LOWERCASE_FIRST_LETTER_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(to_case.lowercase_first_letter())))
  }
}

/// Returns the text transliterated to ascii (*translittéré en ascii*), to make an identifier of it with a case helper:
/// the case helpers keep the accented letters, and `to_snake_case` splits the words around them (`requ_ête`).
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "Erreur. Requête mal formée" }), "{{to_ascii value}}"),
///   "Erreur. Requete mal formee"
/// );
/// assert_eq!(
///   exec_template(json!({ "value": "Erreur. Requête mal formée" }), "{{to_snake_case (to_ascii value)}}"),
///   "erreur_requete_mal_formee"
/// );
/// assert_eq!(
///   exec_template(json!({ "value": "Erreur. Requête mal formée" }), "{{to_pascal_case (to_ascii value)}}"),
///   "ErreurRequeteMalFormee"
/// );
/// assert_eq!(
///   exec_template(json!({ "value": "Liste de communes associées et/ou déléguées" }), "{{to_snake_case (to_ascii value)}}"),
///   "liste_de_communes_associees_et_ou_deleguees"
/// );
/// assert_eq!(
///   exec_template(json!({ "value": "Échec du géocodage (fichier CSV)" }), "{{to_pascal_case (to_ascii value)}}"),
///   "EchecDuGeocodageFichierCSV"
/// );
/// ```
/// A ligature before a lowercase letter gives one capital (`Œuvre` → `Oeuvre`), so that the case helpers do not read two words in it:
/// ```
/// # use codegenr_lib::helpers::*;
/// # use serde_json::json;
/// assert_eq!(
///   exec_template(json!({ "value": "Œuvre introuvable" }), "{{to_pascal_case (to_ascii value)}}"),
///   "OeuvreIntrouvable"
/// );
/// assert_eq!(exec_template(json!({ "value": "ŒUVRE, Æsir, cœur" }), "{{to_ascii value}}"), "OEUVRE, Aesir, coeur");
/// ```
pub struct ToAsciiHelper;

impl HelperDef for ToAsciiHelper {
  fn call_inner<'reg: 'rc, 'rc>(
    &self,
    h: &handlebars::Helper<'rc>,
    _: &'reg handlebars::Handlebars<'reg>,
    _: &'rc handlebars::Context,
    _: &mut handlebars::RenderContext<'reg, 'rc>,
  ) -> Result<handlebars::ScopedJson<'rc>, handlebars::RenderError> {
    h.ensure_arguments_count(1, TO_ASCII_HELPER)?;
    let text = h.get_param_as_str_or_fail(0, TO_ASCII_HELPER)?;
    Ok(handlebars::ScopedJson::Derived(Value::String(to_ascii(text))))
  }
}

fn to_ascii(text: &str) -> String {
  // deunicode gives `OE` and `AE` for the capital ligatures, which `Œuvre` would turn into the two words `O` and `Euvre`.
  let mut text_with_ligatures = String::with_capacity(text.len());
  let mut chars = text.chars().peekable();
  while let Some(char) = chars.next() {
    let before_a_lowercase = chars.peek().is_some_and(|next| next.is_lowercase());
    match char {
      'Œ' if before_a_lowercase => text_with_ligatures.push_str("Oe"),
      'Æ' if before_a_lowercase => text_with_ligatures.push_str("Ae"),
      _ => text_with_ligatures.push(char),
    }
  }
  deunicode::deunicode(&text_with_ligatures)
}

#[cfg(test)]
mod tests {
  use crate::helpers::exec_template;
  use serde_json::json;

  /// Upstream bug of `cruet` 0.14, the case conversions of `handlebars_misc_helpers` 0.16: `char_is_uppercase` is
  /// `c == c.to_ascii_uppercase()`, true for any letter out of ascii, so an accented letter is taken for a capital and the word split around it.
  /// The letters are also lowered with `to_ascii_lowercase`, which leaves a leading `É` as it is.
  /// No upstream issue yet (github.com/taidge/cruet, checked 2026-11-21), the code is the same on its main branch (1.0.0).
  /// Expected: the words kept whole, `requête_mal_formée` and `échec_du_géocodage`; `to_ascii` works around it.
  /// When this test fails, the bug is fixed: turn it into a regular test of the expected output.
  #[test]
  fn upstream_bug_cruet_snake_case_splits_accented_words() {
    assert_eq!(
      exec_template(json!({ "value": "Requête mal formée" }), "{{to_snake_case value}}"),
      "requ_ête_mal_form_ée"
    );
    assert_eq!(
      exec_template(json!({ "value": "Échec du géocodage" }), "{{to_snake_case value}}"),
      "Échec_du_g_éocodage"
    );
  }
}
