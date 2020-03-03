#![allow(dead_code)]

use crate::error::TableError;
use crate::gen::osquery;
use crate::plugin::{Plugin, PluginVariant};
use std::collections::{BTreeMap, HashMap};

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct TablePlugin {
    columns: Vec<ColumnDefinition>,
    name: String,
}

impl Plugin for TablePlugin {
    fn registry_name(&self) -> PluginVariant {
        PluginVariant::Table
    }

    fn routes(&self) -> osquery::ExtensionPluginResponse {
        let mut routes = osquery::ExtensionPluginResponse::new();
        for column in &self.columns {
            let mut map: BTreeMap<String, String> = BTreeMap::new();
            map.insert("id".into(), "column".into());
            map.insert("name".into(), column.name.clone().into());
            map.insert("type".into(), column.coltype.as_str().into());
            map.insert("op".into(), "0".into());
            routes.push(map);
        }
        routes
    }

    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn call(
        &self,
        request: osquery::ExtensionPluginRequest,
    ) -> Result<osquery::ExtensionResponse, crate::error::Error> {
        let ok_status = osquery::ExtensionStatus::new(
            0,
            String::from("OK"),
            osquery::ExtensionRouteUUID::default(),
        );
        match request
            .get("action")
            .and_then(|action| RequestAction::from_str(action))
        {
            Some(RequestAction::Generate) => unimplemented!(),
            Some(RequestAction::Columns) => {
                Ok(osquery::ExtensionResponse::new(ok_status, self.routes()))
            }
            _ => {
                // TODO: Tidy up
                let s = String::from("");
                let action = request.get("action").unwrap_or(&s);
                Err(TableError::UnknownAction(action.to_string()).into())
            }
        }
    }
    fn ping(&self) -> osquery::ExtensionStatus {
        osquery::ExtensionStatus::new(
            0,
            String::from("OK"),
            osquery::ExtensionRouteUUID::default(),
        )
    }
    fn shutdown(&self) {}
}

impl TablePlugin {
    fn parse_query_context(context_str: String) -> Result<QueryContext, crate::error::TableError> {
        let context: QueryContext = serde_json::from_str(&context_str)?;
        Ok(context)
    }
}

#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    name: String,
    coltype: ColumnType,
}

#[derive(Debug, Deserialize)]
pub struct QueryContext {
    constraints: HashMap<String, ConstraintList>,
}

impl QueryContext {
    fn from_json(_context: &str) -> Result<Self, crate::error::TableError> {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize)]
pub struct ConstraintList {
    affinity: ColumnType,
    constraints: Vec<Constraint>,
}

#[derive(Debug, Deserialize)]
pub struct Constraint {
    operator: Operator,
    expressions: String,
}

#[derive(Debug, Deserialize)]
pub enum Operator {
    OperatorEquals = 2,
    OperatorGreaterThan = 4,
    OperatorLessThanOrEquals = 8,
    OperatorLessThan = 16,
    OperatorGreaterThanOrEquals = 32,
    OperatorMatch = 64,
    OperatorLike = 65,
    OperatorGlob = 66,
    OperatorRegexp = 67,
    OperatorUnique = 1,
}

enum RequestAction {
    Generate,
    Columns,
}

impl RequestAction {
    pub fn from_str(s: &str) -> Option<RequestAction> {
        match s {
            "generate" => Some(RequestAction::Generate),
            "columns" => Some(RequestAction::Columns),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RequestAction::Generate => "generate",
            RequestAction::Columns => "columns",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum ColumnType {
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "INTEGER")]
    Integer,
    #[serde(rename = "BIGINT")]
    BigInt,
    #[serde(rename = "DOUBLE")]
    Double,
}

impl ColumnType {
    pub fn from_str(s: &str) -> Option<ColumnType> {
        match s {
            "TEXT" => Some(ColumnType::Text),
            "INTEGER" => Some(ColumnType::Text),
            "BIGINT" => Some(ColumnType::Text),
            "DOUBLE" => Some(ColumnType::Text),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ColumnType::Text => "TEXT",
            ColumnType::Integer => "INTEGER",
            ColumnType::BigInt => "BIGINT",
            ColumnType::Double => "DOUBLE",
        }
    }
}
