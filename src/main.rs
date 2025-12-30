/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright (c) 2025 The rml-x Authors.
 * SPDX-FileCopyrightText: 2025 rml-x
 * SPDX-License-Identifier: MPL-2.0 
 */

#![allow(unused)]
 
use std::{env, fs};


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }
    
    let src = fs::read_to_string(&args[1])?;
    
    match rml_x::parse(&src) {
        Ok(()) => {
            println!("Parse successful!");
            // println!("{:#?}", tree);
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
