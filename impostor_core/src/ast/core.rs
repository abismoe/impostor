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
use crate::ast::json;

///
/// AST for an entire mock file.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImpostorFile {
    pub entries: Vec<Entry>,
    pub line_terminators: Vec<LineTerminator>,
}

/// AST for a single mock entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub request: Request,
    pub response: Response,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub method: Method,
    pub space1: Whitespace,
    pub path: Template,
    pub line_terminator0: LineTerminator,
    pub headers: Vec<Header>,
    pub sections: Vec<RequestSection>,
    pub body: Option<Body>,
    pub source_info: SourceInfo,
}

impl Request {
    /// Returns the captures list of this spec request.
    pub fn captures(&self) -> Vec<Capture> {
        for section in self.sections.iter() {
            if let RequestSectionValue::Captures(captures) = &section.value {
                return captures.clone();
            }
        }
        vec![]
    }

    /// Returns the asserts list of this spec request.
    pub fn asserts(&self) -> Vec<Assert> {
        for section in self.sections.iter() {
            if let RequestSectionValue::Asserts(asserts) = &section.value {
                return asserts.clone();
            }
        }
        vec![]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub line_terminators: Vec<LineTerminator>,
    pub version: Version,
    pub space0: Whitespace,
    pub status: Status,
    pub space1: Whitespace,
    pub line_terminator0: LineTerminator,
    pub headers: Vec<Header>,
    pub body: Option<Body>,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Method(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Version {
    pub value: VersionValue,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VersionValue {
    Version1,
    Version11,
    Version2,
    Version3,
    VersionAny,
    VersionAnyLegacy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Status {
    pub value: u16,
    pub source_info: SourceInfo,
}

pub type Header = KeyValue;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Body {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub value: Bytes,
    pub line_terminator0: LineTerminator,
}

//
// Sections
//

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestSection {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub line_terminator0: LineTerminator,
    pub value: RequestSectionValue,
    pub source_info: SourceInfo,
}

impl RequestSection {
    pub fn name(&self) -> &str {
        match self.value {
            RequestSectionValue::Asserts(_) => "Asserts",
            RequestSectionValue::Captures(_) => "Captures",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum RequestSectionValue {
    Captures(Vec<Capture>),
    Asserts(Vec<Assert>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyValue {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub key: Template,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub value: Template,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capture {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub name: Template,
    pub space1: Whitespace,
    pub space2: Whitespace,
    pub query: Query,
    pub filters: Vec<(Whitespace, Filter)>,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assert {
    pub line_terminators: Vec<LineTerminator>,
    pub space0: Whitespace,
    pub query: Query,
    pub filters: Vec<(Whitespace, Filter)>,
    pub space1: Whitespace,
    pub predicate: Predicate,
    pub line_terminator0: LineTerminator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query {
    pub source_info: SourceInfo,
    pub value: QueryValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum QueryValue {
    Path,
    Header {
        space0: Whitespace,
        name: Template,
    },
    Cookie {
        space0: Whitespace,
        expr: CookiePath,
    },
    Body,
    Xpath {
        space0: Whitespace,
        expr: Template,
    },
    Jsonpath {
        space0: Whitespace,
        expr: Template,
    },
    QueryParam {
        space0: Whitespace,
        name: Template,
    },
    Regex {
        space0: Whitespace,
        value: RegexValue,
    },
    Variable {
        space0: Whitespace,
        name: Template,
    },
    Bytes,
    Sha256,
    Md5,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegexValue {
    Template(Template),
    Regex(Regex),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CookiePath {
    pub name: Template,
    pub attribute: Option<CookieAttribute>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CookieAttribute {
    pub space0: Whitespace,
    pub name: CookieAttributeName,
    pub space1: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CookieAttributeName {
    Value(String),
    Expires(String),
    MaxAge(String),
    Domain(String),
    Path(String),
    Secure(String),
    HttpOnly(String),
    SameSite(String),
}

impl CookieAttributeName {
    pub fn value(&self) -> String {
        match self {
            CookieAttributeName::Value(value)
            | CookieAttributeName::Expires(value)
            | CookieAttributeName::MaxAge(value)
            | CookieAttributeName::Domain(value)
            | CookieAttributeName::Path(value)
            | CookieAttributeName::Secure(value)
            | CookieAttributeName::HttpOnly(value)
            | CookieAttributeName::SameSite(value) => value.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Predicate {
    pub not: bool,
    pub space0: Whitespace,
    pub predicate_func: PredicateFunc,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Not {
    pub value: bool,
    pub space0: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PredicateFunc {
    pub source_info: SourceInfo,
    pub value: PredicateFuncValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum PredicateValue {
    Base64(Base64),
    Bool(bool),
    Expression(Expr),
    File(File),
    Hex(Hex),
    MultilineString(MultilineString),
    Null,
    Number(Number),
    Regex(Regex),
    String(Template),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum PredicateFuncValue {
    Equal {
        space0: Whitespace,
        value: PredicateValue,
        operator: bool,
    },
    NotEqual {
        space0: Whitespace,
        value: PredicateValue,
        operator: bool,
    },
    GreaterThan {
        space0: Whitespace,
        value: PredicateValue,
        operator: bool,
    },
    GreaterThanOrEqual {
        space0: Whitespace,
        value: PredicateValue,
        operator: bool,
    },
    LessThan {
        space0: Whitespace,
        value: PredicateValue,
        operator: bool,
    },
    LessThanOrEqual {
        space0: Whitespace,
        value: PredicateValue,
        operator: bool,
    },
    StartWith {
        space0: Whitespace,
        value: PredicateValue,
    },
    EndWith {
        space0: Whitespace,
        value: PredicateValue,
    },
    Contain {
        space0: Whitespace,
        value: PredicateValue,
    },
    Include {
        space0: Whitespace,
        value: PredicateValue,
    },
    Match {
        space0: Whitespace,
        value: PredicateValue,
    },
    IsInteger,
    IsFloat,
    IsBoolean,
    IsString,
    IsCollection,
    IsDate,
    Exist,
    IsEmpty,
}

//
// Primitives
//
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MultilineString {
    // FIXME: temporary type until we implement oneline as `foo` instead of ```foo```
    OneLineText(Template),
    Text(Text),
    Json(Text),
    Xml(Text),
    GraphQl(GraphQl),
}

impl MultilineString {
    pub fn lang(&self) -> &'static str {
        match self {
            MultilineString::OneLineText(_) | MultilineString::Text(_) => "",
            MultilineString::Json(_) => "json",
            MultilineString::Xml(_) => "xml",
            MultilineString::GraphQl(_) => "graphql",
        }
    }

    pub fn value(&self) -> Template {
        match self {
            MultilineString::OneLineText(template) => template.clone(),
            MultilineString::Text(text)
            | MultilineString::Json(text)
            | MultilineString::Xml(text) => text.value.clone(),
            MultilineString::GraphQl(text) => text.value.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text {
    pub space: Whitespace,
    pub newline: Whitespace,
    pub value: Template,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphQl {
    pub space: Whitespace,
    pub newline: Whitespace,
    pub value: Template,
    pub variables: Option<GraphQlVariables>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphQlVariables {
    pub space: Whitespace,
    pub value: json::Value,
    pub whitespace: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Base64 {
    pub space0: Whitespace,
    pub value: Vec<u8>,
    pub encoded: String,
    pub space1: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    pub space0: Whitespace,
    pub filename: Filename,
    pub space1: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Template {
    pub delimiter: Option<char>,
    pub elements: Vec<TemplateElement>,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TemplateElement {
    // TODO: explain the difference between value and encoded
    String { value: String, encoded: String },
    Expression(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Comment {
    pub value: String,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncodedString {
    pub value: String,
    pub encoded: String,
    pub quotes: bool,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Whitespace {
    pub value: String,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Filename {
    pub value: String,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Number {
    Float(Float),
    Integer(i64),
    BigInteger(String),
}

// keep Number terminology for both Integer and Decimal Numbers
// different representation for the same float value
// 1.01 and 1.010

#[derive(Clone, Debug)]
pub struct Float {
    pub value: f64,
    pub encoded: String, // as defined in Hurl
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.encoded == other.encoded
    }
}
impl Eq for Float {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineTerminator {
    pub space0: Whitespace,
    pub comment: Option<Comment>,
    pub newline: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Bytes {
    Json(json::Value),
    Xml(String),
    MultilineString(MultilineString),
    OnelineString(Template),
    Base64(Base64),
    File(File),
    Hex(Hex),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hex {
    pub space0: Whitespace,
    pub value: Vec<u8>,
    pub encoded: String,
    pub space1: Whitespace,
}

// Literal Regex
#[derive(Clone, Debug)]
pub struct Regex {
    pub inner: regex::Regex,
}

impl PartialEq for Regex {
    fn eq(&self, other: &Self) -> bool {
        self.inner.to_string() == other.inner.to_string()
    }
}
impl Eq for Regex {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}

impl Pos {
    pub fn new(line: usize, column: usize) -> Pos {
        Pos { line, column }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SourceInfo {
    pub start: Pos,
    pub end: Pos,
}

impl SourceInfo {
    pub fn new(start: Pos, end: Pos) -> SourceInfo {
        SourceInfo { start, end }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    pub space0: Whitespace,
    pub variable: Variable,
    pub space1: Whitespace,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub source_info: SourceInfo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BooleanOption {
    Literal(bool),
    Expression(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NaturalOption {
    Literal(u64),
    Expression(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RetryOption {
    Literal(Retry),
    Expression(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariableDefinition {
    pub name: String,
    pub space0: Whitespace,
    pub space1: Whitespace,
    pub value: VariableValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VariableValue {
    Null,
    Bool(bool),
    Number(Number),
    String(Template),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Filter {
    pub source_info: SourceInfo,
    pub value: FilterValue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FilterValue {
    Count,
    DaysAfterNow,
    DaysBeforeNow,
    Decode {
        space0: Whitespace,
        encoding: Template,
    },
    Format {
        space0: Whitespace,
        fmt: Template,
    },
    HtmlEscape,
    HtmlUnescape,
    JsonPath {
        space0: Whitespace,
        expr: Template,
    },
    Nth {
        space0: Whitespace,
        n: u64,
    },
    Regex {
        space0: Whitespace,
        value: RegexValue,
    },
    Replace {
        space0: Whitespace,
        old_value: RegexValue,
        space1: Whitespace,
        new_value: Template,
    },
    Split {
        space0: Whitespace,
        sep: Template,
    },
    ToDate {
        space0: Whitespace,
        fmt: Template,
    },
    ToInt,
    UrlDecode,
    UrlEncode,
    XPath {
        space0: Whitespace,
        expr: Template,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Retry {
    None,
    Finite(usize),
    Infinite,
}