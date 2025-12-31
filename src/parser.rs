/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 * Copyright (c) 2025 The rml-x Authors.
 * SPDX-FileCopyrightText: 2025 rml-x
 * SPDX-License-Identifier: MPL-2.0 
 */

pub struct Parser<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src, 
            pos: 0
        }
    }
    
    pub fn parse(mut self) -> Result<(), String> {
        loop {
            
        }
        
        Ok(())
    }
}


pub fn parse(src: &str) -> Result<(), String> {
    Parser::new(&src).parse()
}