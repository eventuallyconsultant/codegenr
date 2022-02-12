use graphql_parser::schema::*;
use serde_json::{Map, Value};

use crate::loader::LoaderError;

pub type Directives = Vec<GraphqlDirective>;

#[derive(Debug, Clone)]
pub struct Graphql {
  pub schema: GraphqlSchema,
  pub definitions: Vec<GraphqlDefinition>,
}

#[derive(Debug, Clone, Default)]
pub struct GraphqlSchema {
  pub query: Option<String>,
  pub mutation: Option<String>,
  pub subscription: Option<String>,
  pub directives: Directives,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct GraphqlDefinition {
  pub definition_type: GraphqlDefinitionType,
}

#[derive(Debug, Copy, Clone)]
pub enum GraphqlDefinitionType {
  Scalar,
  Object,
  Interface,
  Union,
  Enum,
  InputObject,
}

impl<'a> From<TypeDefinition<'a, String>> for GraphqlDefinition {
  fn from(type_def: TypeDefinition<'a, String>) -> Self {
    let def: Self = match type_def {
      TypeDefinition::Scalar(value) => value.into(),
      TypeDefinition::Object(value) => value.into(),
      TypeDefinition::Interface(value) => value.into(),
      TypeDefinition::Union(value) => value.into(),
      TypeDefinition::Enum(value) => value.into(),
      TypeDefinition::InputObject(value) => value.into(),
    };
    def
  }
}

impl<'a> From<ScalarType<'a, String>> for GraphqlDefinition {
  fn from(type_def: ScalarType<'a, String>) -> Self {
    Self {
      definition_type: GraphqlDefinitionType::Scalar,
    }
  }
}

impl<'a> From<ObjectType<'a, String>> for GraphqlDefinition {
  fn from(type_def: ObjectType<'a, String>) -> Self {
    Self {
      definition_type: GraphqlDefinitionType::Object,
    }
  }
}

impl<'a> From<InterfaceType<'a, String>> for GraphqlDefinition {
  fn from(type_def: InterfaceType<'a, String>) -> Self {
    Self {
      definition_type: GraphqlDefinitionType::Interface,
    }
  }
}

impl<'a> From<UnionType<'a, String>> for GraphqlDefinition {
  fn from(type_def: UnionType<'a, String>) -> Self {
    Self {
      definition_type: GraphqlDefinitionType::Union,
    }
  }
}

impl<'a> From<EnumType<'a, String>> for GraphqlDefinition {
  fn from(type_def: EnumType<'a, String>) -> Self {
    Self {
      definition_type: GraphqlDefinitionType::Enum,
    }
  }
}
impl<'a> From<InputObjectType<'a, String>> for GraphqlDefinition {
  fn from(type_def: InputObjectType<'a, String>) -> Self {
    Self {
      definition_type: GraphqlDefinitionType::InputObject,
    }
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
      Definition::TypeDefinition(type_def) => definitions.push(type_def.into()),
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

  #[test]
  fn read_some_openapi() -> Result<(), anyhow::Error> {
    use openapiv3::OpenAPI;

    let mut file = File::open("./_samples/loader/petstore.openapi3.yaml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let openapi: OpenAPI = serde_yaml::from_str(&content).expect("Could not deserialize input");
    println!("{:?}", openapi);

    Ok(())
  }
}
