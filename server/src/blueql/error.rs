/*
 * Created on Sat Jun 11 2022
 *
 * This file is a part of Skytable
 * Skytable (formerly known as TerrabaseDB or Skybase) is a free and open-source
 * NoSQL database written by Sayan Nandan ("the Author") with the
 * vision to provide flexibility in data modelling without compromising
 * on performance, queryability or scalability.
 *
 * Copyright (c) 2022, Sayan Nandan <ohsayan@outlook.com>
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

use core::{num::ParseIntError, str::Utf8Error};
use std::string::FromUtf8Error;

#[derive(Debug, PartialEq)]
pub enum LangError {
    NonUnicodeChar,
    TypeParseFailure,
    InvalidSyntax,
    UnexpectedEOF,
}

impl From<Utf8Error> for LangError {
    fn from(_: Utf8Error) -> Self {
        Self::NonUnicodeChar
    }
}

impl From<ParseIntError> for LangError {
    fn from(_: ParseIntError) -> Self {
        Self::TypeParseFailure
    }
}

impl From<FromUtf8Error> for LangError {
    fn from(_: FromUtf8Error) -> Self {
        Self::NonUnicodeChar
    }
}
