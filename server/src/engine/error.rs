/*
 * Created on Sat Feb 04 2023
 *
 * This file is a part of Skytable
 * Skytable (formerly known as TerrabaseDB or Skybase) is a free and open-source
 * NoSQL database written by Sayan Nandan ("the Author") with the
 * vision to provide flexibility in data modelling without compromising
 * on performance, queryability or scalability.
 *
 * Copyright (c) 2023, Sayan Nandan <ohsayan@outlook.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
*/

pub type LexResult<T> = Result<T, LexError>;
pub type LangResult<T> = Result<T, LangError>;

#[derive(Debug, PartialEq)]
pub enum LexError {
    // insecure lex
    /// Invalid signed numeric literal
    InvalidSignedNumericLit,
    /// Invalid unsigned literal
    InvalidUnsignedLiteral,
    /// Invaid binary literal
    InvalidBinaryLiteral,
    /// Invalid string literal
    InvalidStringLiteral,
    // secure lex
    /// Dataframe params are invalid
    BadPframe,
    // generic
    /// Unrecognized byte in stream   
    UnexpectedByte,
}
#[derive(Debug, PartialEq)]
pub enum LangError {
    // generic
    /// Unexpected end of syntax
    UnexpectedEOS,
    /// Last resort error kind when error specificity is hard to trace
    BadSyntax,
    /// Expected a token that defines a statement, found something else
    ExpectedStatement,
    // ast nodes: usually parents at heigher hights
    /// Expected an entity, but found invalid tokens
    ExpectedEntity,
    // ast nodes: usually children wrt height
    /// Bad syn tymeta element
    SynBadTyMeta,
    /// Bad syn map element
    SynBadMap,
    /// Bad expr: relational
    ExprBadRel,
    // ast nodes: usually the root
    /// Unknown `create` statement
    StmtUnknownCreate,
    /// Unknown `alter` statement
    StmtUnknownAlter,
    /// unknown `drop` statement
    StmtUnknownDrop,
}
