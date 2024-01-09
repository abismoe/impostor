/*
 * Copyright (C) 2023 The Impostor Contributors
 * Copyright (C) 2023 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
pub type ParseResult<T> = Result<T, Error>;
pub type ParseFunc<T> = fn(&mut Reader) -> ParseResult<T>;

pub fn parse_impostor_file(s: &str) -> ParseResult<ImpostorFile> {
    let mut reader = Reader::new(s);
    parsers::impostor_file(&mut reader)
}

pub use self::error::{Error, JsonErrorVariant, ParseError};
pub use self::json::{
    boolean_value as parse_json_boolean, null_value as parse_json_null,
    number_value as parse_json_number, parse as parse_json,
};
pub use self::reader::Reader;
pub use self::template::templatize;
use crate::ast::ImpostorFile;

mod base64;
mod bytes;
mod combinators;
mod cookiepath;
mod error;
mod expr;
mod filename;
mod filter;
mod json;
mod key_string;
mod multiline;
mod number;
mod parsers;
mod path;
mod predicate;
mod predicate_value;
mod primitives;
mod query;
mod reader;
mod sections;
mod string;
mod template;
mod xml;
