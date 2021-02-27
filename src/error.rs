// Copyright 2020 The RustExample Authors.
//
// Code is licensed under Apache License, Version 2.0.

#[derive(thiserror::Error, Debug)]
pub enum RustExampleError {
    #[error("Internal Error: {0}")]
    Internal(String),
}
