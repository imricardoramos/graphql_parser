use graphql_parser::Pos;
use graphql_parser::query;
use rustler::{Atom, NifMap, NifStruct, NifUntaggedEnum};
use std::collections::BTreeMap;

mod atoms {
    rustler::atoms! {
         query,
         mutation,
         subscription
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.Document"]
pub struct Document {
    definitions: Vec<Definition>,
    loc: Option<Loc>,
}
impl Document {
    pub fn new(document: query::Document<String>) -> Self {
        let definitions = document
            .definitions
            .iter()
            .map(|d| match d {
                query::Definition::Operation(operation) => {
                    Definition::Operation(OperationDefinition::new(operation.clone()))
                }
                query::Definition::Fragment(fragment) => {
                    Definition::Fragment(FragmentDefinition::new(fragment.clone()))
                }
            })
            .collect();
        Self {
            definitions,
            loc: Loc::new(None),
        }
    }
}

#[derive(NifUntaggedEnum)]
pub enum Definition {
    Operation(OperationDefinition),
    Fragment(FragmentDefinition),
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.OperationDefinition"]
pub struct OperationDefinition {
    operation: Atom,
    loc: Option<Loc>,
    name: Option<String>,
    variable_definitions: Vec<VariableDefinition>,
    directives: Vec<Directive>,
    selection_set: SelectionSet,
    shorthand: bool,
}

impl OperationDefinition {
    fn new(operation: query::OperationDefinition<String>) -> Self {
        match operation {
            query::OperationDefinition::Query(query) => Self {
                operation: atoms::query(),
                loc: Loc::new(Some(query.position)),
                name: query.name,
                shorthand: false,
                selection_set: SelectionSet::new(query.selection_set),
                variable_definitions: query
                    .variable_definitions
                    .iter()
                    .map(|vd| VariableDefinition::new(vd.clone()))
                    .collect(),
                directives: query
                    .directives
                    .iter()
                    .map(|d| Directive::new(d.clone()))
                    .collect(),
            },
            query::OperationDefinition::Mutation(mutation) => Self {
                operation: atoms::mutation(),
                loc: Loc::new(Some(mutation.position)),
                name: mutation.name,
                shorthand: false,
                selection_set: SelectionSet::new(mutation.selection_set),
                variable_definitions: mutation
                    .variable_definitions
                    .iter()
                    .map(|vd| VariableDefinition::new(vd.clone()))
                    .collect(),
                directives: mutation
                    .directives
                    .iter()
                    .map(|d| Directive::new(d.clone()))
                    .collect(),
            },
            query::OperationDefinition::Subscription(subscription) => Self {
                operation: atoms::subscription(),
                loc: Loc::new(Some(subscription.position)),
                name: subscription.name,
                shorthand: false,
                selection_set: SelectionSet::new(subscription.selection_set),
                variable_definitions: subscription
                    .variable_definitions
                    .iter()
                    .map(|vd| VariableDefinition::new(vd.clone()))
                    .collect(),
                directives: subscription
                    .directives
                    .iter()
                    .map(|d| Directive::new(d.clone()))
                    .collect(),
            },
            query::OperationDefinition::SelectionSet(selection_set) => Self {
                operation: atoms::query(),
                loc: Loc::new(Some(selection_set.span.1)),
                name: None,
                shorthand: true,
                selection_set: SelectionSet::new(selection_set),
                variable_definitions: vec![],
                directives: vec![],
            },
        }
    }
}

type TypeCondition = Type;

#[derive(NifStruct)]
#[module = "Absinthe.Language.Fragment"]
pub struct FragmentDefinition {
    name: String,
    type_condition: TypeCondition,
    directives: Vec<Directive>,
    selection_set: SelectionSet,
    loc: Option<Loc>,
}

impl FragmentDefinition {
    fn new(definition: query::FragmentDefinition<String>) -> Self {
        Self {
            name: definition.name,
            type_condition: match definition.type_condition {
                query::TypeCondition::On(value) => Type::NamedType(NamedType::new(value, None)),
            },
            directives: definition
                .directives
                .iter()
                .map(|d| Directive::new(d.clone()))
                .collect(),
            selection_set: SelectionSet::new(definition.selection_set),
            loc: Loc::new(Some(definition.position)),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.VariableDefinition"]
pub struct VariableDefinition {
    default_value: Option<Value>,
    variable: Variable,
    r#type: Type,
    directives: Vec<Directive>,
    loc: Option<Loc>,
}

impl VariableDefinition {
    fn new(definition: query::VariableDefinition<String>) -> Self {
        Self {
            loc: Loc::new(Some(definition.position)),
            variable: Variable::new(definition.name, Some(definition.position)),
            directives: vec![],
            r#type: Type::build(definition.var_type),
            default_value: match definition.default_value {
                Some(value) => Some(Value::build(value)),
                None => None,
            },
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.Variable"]
pub struct Variable {
    name: String,
    loc: Option<Loc>,
}

impl Variable {
    fn new(name: String, position: Option<Pos>) -> Self {
        Self {
            name,
            loc: Loc::new(position),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.Directive"]
pub struct Directive {
    name: String,
    arguments: Vec<Argument>,
    loc: Option<Loc>,
}

impl Directive {
    fn new(directive: query::Directive<String>) -> Self {
        Self {
            loc: Loc::new(Some(directive.position)),
            name: directive.name,
            arguments: directive
                .arguments
                .iter()
                .map(|a| Argument::new(a.clone()))
                .collect(),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.SelectionSet"]
pub struct SelectionSet {
    loc: Option<Loc>,
    selections: Vec<Selection>,
}

impl SelectionSet {
    fn new(selection_set: query::SelectionSet<String>) -> Self {
        Self {
            loc: Loc::new(Some(selection_set.span.0)),
            selections: selection_set
                .items
                .iter()
                .map(|s| match s {
                    query::Selection::Field(field) => Selection::Field(Field::new(field.clone())),
                    query::Selection::FragmentSpread(fragment_spread) => {
                        Selection::FragmentSpread(FragmentSpread::new(fragment_spread.clone()))
                    }
                    query::Selection::InlineFragment(inline_fragment) => {
                        Selection::InlineFragment(InlineFragment::new(inline_fragment.clone()))
                    }
                })
                .collect(),
        }
    }
}

#[derive(NifUntaggedEnum)]
pub enum Type {
    NamedType(NamedType),
    NonNullType(NonNullType),
    ListType(ListType),
}
impl Type {
    fn build(r#type: query::Type<String>) -> Self {
        match r#type {
            query::Type::NamedType(name) => Type::NamedType(NamedType::new(name, None)),
            query::Type::NonNullType(name) => {
                Type::NonNullType(NonNullType::new(Type::build(*name), None))
            }
            query::Type::ListType(name) => Type::ListType(ListType::new(Type::build(*name), None)),
        }
    }
}

#[derive(NifUntaggedEnum)]
enum Selection {
    Field(Field),
    FragmentSpread(FragmentSpread),
    InlineFragment(InlineFragment),
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.Field"]
pub struct Field {
    loc: Option<Loc>,
    name: String,
    alias: Option<String>,
    directives: Vec<Directive>,
    arguments: Vec<Argument>,
    selection_set: Option<SelectionSet>,
}

impl Field {
    fn new(field: query::Field<String>) -> Self {
        Self {
            alias: field.alias,
            directives: field
                .directives
                .iter()
                .map(|d| Directive::new(d.clone()))
                .collect(),
            arguments: field
                .arguments
                .iter()
                .map(|a| Argument::new(a.clone()))
                .collect(),
            name: field.name,
            loc: Loc::new(Some(field.position)),
            selection_set: if field.selection_set.items.len() > 0 {
                Some(SelectionSet::new(field.selection_set))
            } else {
                None
            },
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.FragmentSpread"]
pub struct FragmentSpread {
    loc: Option<Loc>,
    name: String,
    directives: Vec<Directive>,
}

impl FragmentSpread {
    fn new(fragment_spread: query::FragmentSpread<String>) -> Self {
        Self {
            directives: fragment_spread
                .directives
                .iter()
                .map(|d| Directive::new(d.clone()))
                .collect(),

            name: fragment_spread.fragment_name,
            loc: Loc::new(Some(fragment_spread.position)),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.InlineFragment"]
pub struct InlineFragment {
    loc: Option<Loc>,
    type_condition: Option<TypeCondition>,
    directives: Vec<Directive>,
    selection_set: SelectionSet,
}

impl InlineFragment {
    fn new(inline_fragment: query::InlineFragment<String>) -> Self {
        Self {
            directives: inline_fragment
                .directives
                .iter()
                .map(|d| Directive::new(d.clone()))
                .collect(),
            type_condition: match inline_fragment.type_condition {
                Some(value) => match value {
                    query::TypeCondition::On(value) => {
                        Some(Type::NamedType(NamedType::new(value, None)))
                    }
                },
                None => None,
            },
            selection_set: SelectionSet::new(inline_fragment.selection_set),
            loc: Loc::new(Some(inline_fragment.position)),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.Argument"]
pub struct Argument {
    loc: Option<Loc>,
    name: String,
    value: Value,
}

impl Argument {
    fn new(argument: (String, query::Value<String>)) -> Self {
        Self {
            loc: Loc::new(None),
            name: argument.0,
            value: Value::build(argument.1),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.NamedType"]
pub struct NamedType {
    name: String,
    loc: Option<Loc>,
}

impl NamedType {
    fn new(name: String, position: Option<Pos>) -> Self {
        Self {
            name,
            loc: Loc::new(position),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.NonNullType"]
pub struct NonNullType {
    r#type: Box<Type>,
    loc: Option<Loc>,
}

impl NonNullType {
    fn new(r#type: Type, position: Option<Pos>) -> Self {
        Self {
            r#type: Box::new(r#type),
            loc: Loc::new(position),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.ListType"]
pub struct ListType {
    r#type: Box<Type>,
    loc: Option<Loc>,
}

impl ListType {
    fn new(r#type: Type, position: Option<Pos>) -> Self {
        Self {
            r#type: Box::new(r#type),
            loc: Loc::new(position),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.StringValue"]
pub struct StringValue {
    value: String,
    loc: Option<Loc>,
}

impl StringValue {
    fn new(value: String) -> Self {
        Self {
            value,
            loc: Loc::new(None),
        }
    }
}

#[derive(NifMap)]
pub struct Loc {
    line: Option<usize>,
    column: Option<usize>,
}

impl Loc {
    fn new(pos: Option<Pos>) -> Option<Self> {
        match pos {
            Some(pos) => Some(Self {
                line: Some(pos.line),
                column: Some(pos.column),
            }),
            None => None,
        }
    }
}

#[derive(NifUntaggedEnum)]
enum Value {
    StringValue(StringValue),
    IntValue(IntValue),
    FloatValue(FloatValue),
    BooleanValue(BooleanValue),
    EnumValue(EnumValue),
    NullValue(NullValue),
    Variable(Variable),
    ListValue(ListValue),
    ObjectValue(ObjectValue),
}
impl Value {
    fn build(value: query::Value<String>) -> Self {
        match value {
            query::Value::String(value) => Value::StringValue(StringValue::new(value)),
            query::Value::Int(value) => Value::IntValue(IntValue::new(value)),
            query::Value::Float(value) => Value::FloatValue(FloatValue::new(value)),
            query::Value::Boolean(value) => Value::BooleanValue(BooleanValue::new(value)),
            query::Value::Enum(value) => Value::EnumValue(EnumValue::new(value)),
            query::Value::Variable(value) => Value::Variable(Variable::new(value, None)),
            query::Value::List(values) => Value::ListValue(ListValue::new(values)),
            query::Value::Object(map) => Value::ObjectValue(ObjectValue::new(map)),
            query::Value::Null => Value::NullValue(NullValue::new()),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.IntValue"]
pub struct IntValue {
    value: i64,
    loc: Option<Loc>,
}
impl IntValue {
    fn new(value: query::Number) -> Self {
        Self {
            value: value.as_i64().unwrap(),
            loc: Loc::new(None),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.FloatValue"]
pub struct FloatValue {
    value: f64,
    loc: Option<Loc>,
}
impl FloatValue {
    fn new(value: f64) -> Self {
        Self {
            value,
            loc: Loc::new(None),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.BooleanValue"]
pub struct BooleanValue {
    value: bool,
    loc: Option<Loc>,
}
impl BooleanValue {
    fn new(value: bool) -> Self {
        Self {
            value,
            loc: Loc::new(None),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.NullValue"]
pub struct NullValue {
    loc: Option<Loc>,
}
impl NullValue {
    fn new() -> Self {
        Self {
            loc: Loc::new(None),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.EnumValue"]
pub struct EnumValue {
    value: String,
    loc: Option<Loc>,
}
impl EnumValue {
    fn new(value: String) -> Self {
        Self {
            value,
            loc: Loc::new(None),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.ListValue"]
pub struct ListValue {
    values: Vec<Value>,
    loc: Option<Loc>,
}
impl ListValue {
    fn new(values: Vec<query::Value<String>>) -> Self {
        Self {
            values: values.iter().map(|v| Value::build(v.clone())).collect(),
            loc: Loc::new(None),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.ObjectValue"]
pub struct ObjectValue {
    fields: Vec<ObjectField>,
    loc: Option<Loc>,
}
impl ObjectValue {
    fn new(fields: BTreeMap<String, query::Value<String>>) -> Self {
        Self {
            fields: fields
                .iter()
                .map(|(k, v)| ObjectField::new(k.clone(), v.clone()))
                .collect(),
            loc: Loc::new(None),
        }
    }
}

#[derive(NifStruct)]
#[module = "Absinthe.Language.ObjectField"]
pub struct ObjectField {
    name: String,
    value: Value,
    loc: Option<Loc>,
}
impl ObjectField {
    fn new(name: String, value: query::Value<String>) -> Self {
        Self {
            name,
            value: Value::build(value),
            loc: Loc::new(None),
        }
    }
}
// decoder to satisfy rustler
// impl Decoder<'_> for Box<Type> {
//     fn decode(_a: Term<'_>) -> NifResult<Self> {
//         Ok(Box::new(Type::build(query::Type::NamedType(
//             "".to_string(),
//         ))))
//     }
// }
