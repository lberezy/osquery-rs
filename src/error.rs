use crate::gen::osquery;

//TODO: Properly structure errors
#[derive(Debug)]
pub enum Error {
    TableError(TableError),
    ManagerError,
    ThriftError,
}

impl From<thrift::Error> for Error {
    fn from(err: thrift::Error) -> Self {
        Error::ThriftError
    }
}

#[derive(Debug)]
pub enum ManagerError {
    GenericIo,
}

impl From<std::io::Error> for ManagerError {
    fn from(_io_err: std::io::Error) -> Self {
        // TODO: Embiggen this
        ManagerError::GenericIo
    }
}

// TODO: Review which of these are generic and can hoiseted to higher level error enum
#[derive(Debug)]
pub enum TableError {
    ContextUnmarshall,
    UnknownAction(String),
    Generic,
}

impl From<serde_json::error::Error> for TableError {
    fn from(_serde_err: serde_json::error::Error) -> Self {
        TableError::ContextUnmarshall
        // TODO: Match serde_json ErrorCode and enrich error context
        // use serde_json::error::Category
        // match serde_err.classify() {

        // }
    }
}

impl From<TableError> for Error {
    fn from(err: TableError) -> Self {
        Error::TableError(err)
    }
}

impl From<TableError> for osquery::ExtensionResponse {
    fn from(err: TableError) -> Self {
        match err {
            TableError::ContextUnmarshall => {
                let status = osquery::ExtensionStatus::new(
                    0,
                    String::from("Generic table error."),
                    osquery::ExtensionRouteUUID::default(),
                );
                osquery::ExtensionResponse::new(status, None)
            }
            TableError::UnknownAction(msg) => {
                let status = osquery::ExtensionStatus::new(
                    1,
                    String::from(format!("Unknown action: {}", msg)),
                    osquery::ExtensionRouteUUID::default(),
                );
                osquery::ExtensionResponse::new(status, None)
            }
            _ => {
                let status = osquery::ExtensionStatus::new(
                    0,
                    String::from("Generic table error."),
                    osquery::ExtensionRouteUUID::default(),
                );
                osquery::ExtensionResponse::new(status, None)
            }
        }
    }
}
