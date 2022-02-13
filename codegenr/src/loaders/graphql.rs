use super::{DocumentLoader, LoaderError};
use graphql_parser::schema::*;
use serde::Serialize;
use serde_json::{Map, Value};

pub type Directives = Vec<GraphqlDirective>;

pub struct GraphqlLoader {}
impl DocumentLoader for GraphqlLoader {
  type Error = LoaderError;
  fn json_from_str(content: &str) -> Result<Value, Self::Error> {
    let ast = parse_schema::<String>(content)?;
    let obj = graphql_to_object(ast)?;
    Ok(serde_json::to_value(&obj)?)
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct Graphql {
  pub schema: GraphqlSchema,
  pub definitions: Vec<GraphqlDefinition>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GraphqlSchema {
  pub query: Option<String>,
  pub mutation: Option<String>,
  pub subscription: Option<String>,
  pub directives: Directives,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphqlDirective {
  pub name: String,
  pub arguments: Vec<(String, Value)>,
}

impl<'a> TryFrom<Directive<'a, String>> for GraphqlDirective {
  type Error = LoaderError;
  fn try_from(d: Directive<String>) -> Result<Self, Self::Error> {
    Ok(Self {
      name: d.name,
      arguments: d
        .arguments
        .into_iter()
        .map(|(key, value)| graphql_to_json(value).map(|v| (key, v)))
        .collect::<Result<Vec<_>, _>>()?,
    })
  }
}

fn graphql_directives_to_object(graphql: Vec<Directive<'_, String>>) -> Result<Directives, LoaderError> {
  graphql.into_iter().map(|d| d.try_into()).collect::<Result<Directives, _>>()
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphqlDefinition {
  #[serde(rename = "type")]
  pub definition_type: GraphqlDefinitionType,
  pub name: String,
  pub description: Option<String>,
  pub directives: Directives,
  #[serde(rename = "implementsInterfaces")]
  pub implements_interfaces: Vec<String>,
  pub fields: Vec<GraphqlField>,
  pub inputs: Vec<GraphqlInputValue>,
  #[serde(rename = "unionOf")]
  pub union_of: Vec<String>,
  pub values: Vec<GraphqlEnumValue>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphqlField {
  pub name: String,
  pub description: Option<String>,
  pub arguments: Vec<GraphqlInputValue>,
  #[serde(rename = "fieldType")]
  pub field_type: GraphqlType,
  pub directives: Directives,
}

impl<'a> TryFrom<Field<'a, String>> for GraphqlField {
  type Error = LoaderError;
  fn try_from(field: Field<'a, String>) -> Result<Self, Self::Error> {
    Ok(Self {
      name: field.name,
      description: field.description,
      arguments: field.arguments.into_iter().map(|a| a.try_into()).collect::<Result<Vec<_>, _>>()?,
      field_type: field.field_type.try_into()?,
      directives: graphql_directives_to_object(field.directives)?,
    })
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphqlInputValue {
  pub name: String,
  pub description: Option<String>,
  #[serde(rename = "valueType")]
  pub value_type: GraphqlType,
  #[serde(rename = "defaultValue")]
  pub default_value: Option<Value>,
  pub directives: Directives,
}

impl<'a> TryFrom<InputValue<'a, String>> for GraphqlInputValue {
  type Error = LoaderError;
  fn try_from(input: InputValue<'a, String>) -> Result<Self, Self::Error> {
    Ok(Self {
      name: input.name,
      description: input.description,
      value_type: input.value_type.try_into()?,
      default_value: input.default_value.map(graphql_to_json).transpose()?,
      directives: graphql_directives_to_object(input.directives)?,
    })
  }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct GraphqlType {
  pub name: Option<String>,
  pub required: bool,
  #[serde(rename = "listOf")]
  pub list_of: Option<Box<GraphqlType>>,
}

impl<'a> TryFrom<Type<'a, String>> for GraphqlType {
  type Error = LoaderError;
  fn try_from(typ: Type<'a, String>) -> Result<Self, Self::Error> {
    Ok(match typ {
      Type::NamedType(s) => Self {
        name: Some(s),
        required: false,
        list_of: None,
      },
      Type::ListType(b) => {
        let t: GraphqlType = (*b).try_into()?;
        Self {
          name: None,
          required: true,
          list_of: Some(Box::new(t)),
        }
      }
      Type::NonNullType(b) => {
        let t: GraphqlType = (*b).try_into()?;
        Self { required: true, ..t }
      }
    })
  }
}

#[derive(Debug, Clone, Serialize)]
pub struct GraphqlEnumValue {
  pub name: String,
  pub description: Option<String>,
  pub directives: Directives,
}

impl<'a> TryFrom<EnumValue<'a, String>> for GraphqlEnumValue {
  type Error = LoaderError;
  fn try_from(value: EnumValue<'a, String>) -> Result<Self, Self::Error> {
    Ok(Self {
      name: value.name,
      description: value.description,
      directives: graphql_directives_to_object(value.directives)?,
    })
  }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub enum GraphqlDefinitionType {
  Scalar,
  Object,
  Interface,
  Union,
  Enum,
  InputObject,
}

impl<'a> TryFrom<TypeDefinition<'a, String>> for GraphqlDefinition {
  type Error = LoaderError;
  fn try_from(type_def: TypeDefinition<'a, String>) -> Result<Self, Self::Error> {
    match type_def {
      TypeDefinition::Scalar(value) => value.try_into(),
      TypeDefinition::Object(value) => value.try_into(),
      TypeDefinition::Interface(value) => value.try_into(),
      TypeDefinition::Union(value) => value.try_into(),
      TypeDefinition::Enum(value) => value.try_into(),
      TypeDefinition::InputObject(value) => value.try_into(),
    }
  }
}

impl GraphqlDefinition {
  fn default(
    definition_type: GraphqlDefinitionType,
    name: String,
    description: Option<String>,
    directives: Vec<Directive<'_, String>>,
  ) -> Result<Self, LoaderError> {
    Ok(Self {
      definition_type,
      name,
      description,
      directives: graphql_directives_to_object(directives)?,
      implements_interfaces: vec![],
      fields: vec![],
      inputs: vec![],
      union_of: vec![],
      values: vec![],
    })
  }
}

impl<'a> TryFrom<ScalarType<'a, String>> for GraphqlDefinition {
  type Error = LoaderError;
  fn try_from(def: ScalarType<'a, String>) -> Result<Self, Self::Error> {
    Self::default(GraphqlDefinitionType::Scalar, def.name, def.description, def.directives)
  }
}
impl<'a> TryFrom<ObjectType<'a, String>> for GraphqlDefinition {
  type Error = LoaderError;
  fn try_from(def: ObjectType<'a, String>) -> Result<Self, Self::Error> {
    Ok(Self {
      fields: def.fields.into_iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()?,
      implements_interfaces: def.implements_interfaces,
      ..Self::default(GraphqlDefinitionType::Object, def.name, def.description, def.directives)?
    })
  }
}
impl<'a> TryFrom<InterfaceType<'a, String>> for GraphqlDefinition {
  type Error = LoaderError;
  fn try_from(def: InterfaceType<'a, String>) -> Result<Self, Self::Error> {
    Ok(Self {
      fields: def.fields.into_iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()?,
      implements_interfaces: def.implements_interfaces,
      ..Self::default(GraphqlDefinitionType::Interface, def.name, def.description, def.directives)?
    })
  }
}
impl<'a> TryFrom<UnionType<'a, String>> for GraphqlDefinition {
  type Error = LoaderError;
  fn try_from(def: UnionType<'a, String>) -> Result<Self, Self::Error> {
    Ok(Self {
      union_of: def.types,
      ..Self::default(GraphqlDefinitionType::Union, def.name, def.description, def.directives)?
    })
  }
}
impl<'a> TryFrom<EnumType<'a, String>> for GraphqlDefinition {
  type Error = LoaderError;
  fn try_from(def: EnumType<'a, String>) -> Result<Self, Self::Error> {
    Ok(Self {
      values: def.values.into_iter().map(|v| v.try_into()).collect::<Result<Vec<_>, _>>()?,
      ..Self::default(GraphqlDefinitionType::Enum, def.name, def.description, def.directives)?
    })
  }
}
impl<'a> TryFrom<InputObjectType<'a, String>> for GraphqlDefinition {
  type Error = LoaderError;
  fn try_from(def: InputObjectType<'a, String>) -> Result<Self, Self::Error> {
    Ok(Self {
      inputs: def.fields.into_iter().map(|f| f.try_into()).collect::<Result<Vec<_>, _>>()?,
      ..Self::default(GraphqlDefinitionType::InputObject, def.name, def.description, def.directives)?
    })
  }
}

pub fn graphql_to_object(graphql: Document<String>) -> Result<Graphql, LoaderError> {
  let mut schema = GraphqlSchema::default();
  let mut definitions: Vec<GraphqlDefinition> = Vec::<_>::with_capacity(graphql.definitions.len());

  for definition in graphql.definitions {
    match definition {
      Definition::SchemaDefinition(schema_def) => {
        schema.query = schema_def.query;
        schema.mutation = schema_def.mutation;
        schema.subscription = schema_def.subscription;
        schema.directives = graphql_directives_to_object(schema_def.directives)?
      }
      Definition::TypeDefinition(type_def) => definitions.push(type_def.try_into()?),
      Definition::TypeExtension(_) => {
        // ignored
      }
      Definition::DirectiveDefinition(_) => {
        // ignored
      }
    }
  }

  Ok(Graphql { schema, definitions })
}

fn graphql_to_json(value: graphql_parser::schema::Value<String>) -> Result<Value, LoaderError> {
  use graphql_parser::schema as gql;
  use serde_json::Number;
  let json = match value {
    gql::Value::Variable(_) => todo!(),
    gql::Value::Int(n) => Value::Number(Number::from(
      n.as_i64().ok_or(LoaderError::GraphqlToJsonError("The number should be an i64."))?,
    )),
    gql::Value::Float(f) => Value::Number(Number::from_f64(f).ok_or(LoaderError::GraphqlToJsonError("The number should be an f64."))?),
    gql::Value::String(s) => Value::String(s),
    gql::Value::Boolean(b) => Value::Bool(b),
    gql::Value::Null => Value::Null,
    gql::Value::Enum(e) => Value::String(e),
    gql::Value::List(l) => Value::Array(l.into_iter().map(graphql_to_json).collect::<Result<Vec<_>, _>>()?),
    gql::Value::Object(map) => {
      let mut json = Map::<_, _>::with_capacity(map.len());
      for (key, value) in map {
        json.insert(key, graphql_to_json(value)?);
      }
      Value::Object(json)
    }
  };
  Ok(json)
}

#[cfg(test)]
mod test {
  use super::*;
  use std::{fs::File, io::Read};

  #[test]
  fn read_some_graphql() -> Result<(), anyhow::Error> {
    use graphql_parser::schema::parse_schema;

    let mut file = File::open("./_samples/graphql/schema.graphql")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let ast = parse_schema::<String>(&content)?;
    dbg!(&ast);

    let obj = graphql_to_object(ast)?;
    dbg!(obj);

    Ok(())
  }
}
