use graphql_parser::schema::*;
use serde_json::Value;

#[derive(Debug, Clone)]
struct GraphQlDefinition {
  pub definition_type: GraphQlDefinitionType,
}

#[derive(Debug, Copy, Clone)]
enum GraphQlDefinitionType {
  Scalar,
  Object,
  Interface,
  Union,
  Enum,
  InputObject,
}

impl<'a> From<TypeDefinition<'a, String>> for GraphQlDefinition {
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

impl<'a> From<ScalarType<'a, String>> for GraphQlDefinition {
  fn from(type_def: ScalarType<'a, String>) -> Self {
    Self {
      definition_type: GraphQlDefinitionType::Scalar,
    }
  }
}

impl<'a> From<ObjectType<'a, String>> for GraphQlDefinition {
  fn from(type_def: ObjectType<'a, String>) -> Self {
    Self {
      definition_type: GraphQlDefinitionType::Object,
    }
  }
}

impl<'a> From<InterfaceType<'a, String>> for GraphQlDefinition {
  fn from(type_def: InterfaceType<'a, String>) -> Self {
    Self {
      definition_type: GraphQlDefinitionType::Interface,
    }
  }
}

impl<'a> From<UnionType<'a, String>> for GraphQlDefinition {
  fn from(type_def: UnionType<'a, String>) -> Self {
    Self {
      definition_type: GraphQlDefinitionType::Union,
    }
  }
}

impl<'a> From<EnumType<'a, String>> for GraphQlDefinition {
  fn from(type_def: EnumType<'a, String>) -> Self {
    Self {
      definition_type: GraphQlDefinitionType::Enum,
    }
  }
}
impl<'a> From<InputObjectType<'a, String>> for GraphQlDefinition {
  fn from(type_def: InputObjectType<'a, String>) -> Self {
    Self {
      definition_type: GraphQlDefinitionType::InputObject,
    }
  }
}

fn graphql_to_json(graphql: Document<String>) -> Vec<GraphQlDefinition> {
  let mut defs: Vec<GraphQlDefinition> = Vec::<_>::with_capacity(graphql.definitions.len());

  for definition in graphql.definitions {
    match definition {
      Definition::SchemaDefinition(_schema_def) => {
        // ignored
      }
      Definition::TypeDefinition(type_def) => defs.push(type_def.into()),
      Definition::TypeExtension(_) => {
        // ignored
      }
      Definition::DirectiveDefinition(_) => {
        // ignored
      }
    }
  }

  defs
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

    let json = graphql_to_json(ast);
    dbg!(json);

    Ok(())
  }

  #[test]
  fn read_some_openapi() -> Result<(), anyhow::Error> {
    use openapiv3::OpenAPI;

    let mut file = File::open("./_samples/resolver/petshop.yaml")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let openapi: OpenAPI = serde_yaml::from_str(&content).expect("Could not deserialize input");
    println!("{:?}", openapi);

    Ok(())
  }
}
