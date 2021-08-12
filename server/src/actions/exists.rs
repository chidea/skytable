/*
 * Created on Wed Aug 19 2020
 *
 * This file is a part of Skytable
 * Skytable (formerly known as TerrabaseDB or Skybase) is a free and open-source
 * NoSQL database written by Sayan Nandan ("the Author") with the
 * vision to provide flexibility in data modelling without compromising
 * on performance, queryability or scalability.
 *
 * Copyright (c) 2020, Sayan Nandan <ohsayan@outlook.com>
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

//! # `EXISTS` queries
//! This module provides functions to work with `EXISTS` queries

use crate::dbnet::connection::prelude::*;
use crate::queryengine::ActionIter;

action!(
    /// Run an `EXISTS` query
    fn exists(handle: &Corestore, con: &mut T, act: ActionIter) {
        err_if_len_is!(act, con, eq 0);
        let mut how_many_of_them_exist = 0usize;
        {
            let cmap = kve!(con, handle);
            act.for_each(|key| {
                if not_enc_err!(cmap.exists(&key)) {
                    how_many_of_them_exist += 1;
                }
            });
        }
        con.write_response(how_many_of_them_exist).await?;
        Ok(())
    }
);
