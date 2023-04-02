/*
 * Created on Fri Jan 06 2023
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

#[cfg(test)]
use super::WhereClauseCollection;
use {
    super::WhereClause,
    crate::{
        engine::{
            error::{LangError, LangResult},
            ql::ast::{Entity, QueryData, State},
        },
        util::{compiler, MaybeInit},
    },
};

/*
    Impls for delete
    ---
    Smallest statement:
    delete model:primary_key
*/

#[derive(Debug, PartialEq)]
pub struct DeleteStatement<'a> {
    pub(super) entity: Entity<'a>,
    pub(super) wc: WhereClause<'a>,
}

impl<'a> DeleteStatement<'a> {
    #[inline(always)]
    pub(super) fn new(entity: Entity<'a>, wc: WhereClause<'a>) -> Self {
        Self { entity, wc }
    }
    #[inline(always)]
    #[cfg(test)]
    pub fn new_test(entity: Entity<'a>, wc: WhereClauseCollection<'a>) -> Self {
        Self::new(entity, WhereClause::new(wc))
    }
    #[inline(always)]
    pub fn parse_delete<Qd: QueryData<'a>>(state: &mut State<'a, Qd>) -> LangResult<Self> {
        /*
            TODO(@ohsayan): Volcano
            smallest tt:
            delete from model where x = 1
                   ^1   ^2    ^3    ^4  ^5
        */
        if compiler::unlikely(state.remaining() < 5) {
            return compiler::cold_rerr(LangError::UnexpectedEOS);
        }
        // from + entity
        state.poison_if_not(state.cursor_eq(Token![from]));
        state.cursor_ahead(); // ignore errors (if any)
        let mut entity = MaybeInit::uninit();
        Entity::parse_from_state_len_unchecked(state, &mut entity);
        // where + clauses
        state.poison_if_not(state.cursor_eq(Token![where]));
        state.cursor_ahead(); // ignore errors
        let wc = WhereClause::parse_where(state);
        if compiler::likely(state.okay()) {
            Ok(Self {
                entity: unsafe {
                    // UNSAFE(@ohsayan): Safety guaranteed by state
                    entity.assume_init()
                },
                wc,
            })
        } else {
            compiler::cold_rerr(LangError::BadSyntax)
        }
    }
}

mod impls {
    use {
        super::DeleteStatement,
        crate::engine::{
            error::LangResult,
            ql::ast::{traits::ASTNode, QueryData, State},
        },
    };
    impl<'a> ASTNode<'a> for DeleteStatement<'a> {
        fn _from_state<Qd: QueryData<'a>>(state: &mut State<'a, Qd>) -> LangResult<Self> {
            Self::parse_delete(state)
        }
    }
}
