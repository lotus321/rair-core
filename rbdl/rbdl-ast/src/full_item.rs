/*
 * full_item.rs: Thin wrappr around AstItem.
 * Copyright (C) 2019  Oddcoder
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use super::{AstField, AstItem, AstItemContent, Table};
use rbdl_syn::{RBDLEnum, RBDLFields, RBDLItem, RBDLStruct};
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use syn::{Error, Ident, Result};

pub struct FullItem(Ident, AstItem);

impl FullItem {
    pub fn unwrap(self) -> (Ident, AstItem) {
        (self.0, self.1)
    }
}
fn prep_fields(rbdl_fields: RBDLFields) -> (Vec<AstField>, Option<Error>) {
    let mut set = HashSet::new();
    let mut fields = Vec::new();
    let mut err = None;
    for field in rbdl_fields.named {
        if set.contains(&field.ident) {
            let e = Error::new(field.ident.span(), format!("Field `{}` is already declared before", &field.ident));
            if err.is_none() {
                err = Some(e);
            } else {
                err.as_mut().unwrap().combine(e);
            }
        } else {
            set.insert(field.ident.clone());
        }
        match AstField::try_from(field) {
            Ok(f) => fields.push(f),
            Err(e) => {
                if err.is_none() {
                    err = Some(e);
                } else {
                    err.as_mut().unwrap().combine(e);
                }
            }
        }
    }
    (fields, err)
}

impl TryFrom<RBDLStruct> for FullItem {
    type Error = Error;
    fn try_from(parse_tree: RBDLStruct) -> Result<FullItem> {
        let ident = parse_tree.ident;
        let (fields, mut err) = prep_fields(parse_tree.fields);
        let mut attrs = HashMap::new();
        if let Some(rbdl_attrs) = parse_tree.attrs {
            match Table::try_from(rbdl_attrs) {
                Ok(table) => attrs = table.unwrap(),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    } else {
                        err.as_mut().unwrap().combine(e);
                    }
                }
            }
        }
        match err {
            Some(e) => Err(e),
            None => Ok(FullItem(ident, AstItem::Struct(AstItemContent { fields, attrs }))),
        }
    }
}
impl TryFrom<RBDLEnum> for FullItem {
    type Error = Error;
    fn try_from(parse_tree: RBDLEnum) -> Result<FullItem> {
        let ident = parse_tree.ident;
        let (fields, mut err) = prep_fields(parse_tree.fields);
        let mut attrs = HashMap::new();
        if let Some(rbdl_attrs) = parse_tree.attrs {
            match Table::try_from(rbdl_attrs) {
                Ok(table) => attrs = table.unwrap(),
                Err(e) => {
                    if err.is_none() {
                        err = Some(e);
                    } else {
                        err.as_mut().unwrap().combine(e);
                    }
                }
            }
        }
        match err {
            Some(e) => Err(e),
            None => Ok(FullItem(ident, AstItem::Enum(AstItemContent { fields, attrs }))),
        }
    }
}

impl TryFrom<RBDLItem> for FullItem {
    type Error = Error;
    fn try_from(parse_tree: RBDLItem) -> Result<FullItem> {
        match parse_tree {
            RBDLItem::Struct(s) => s.try_into(),
            RBDLItem::Enum(e) => e.try_into(),
        }
    }
}
