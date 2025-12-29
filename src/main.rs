/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright (c) 2025 The rml-x Authors.
 * SPDX-FileCopyrightText: 2025 rml-x
 * SPDX-License-Identifier: MPL-2.0 
 */

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("log.txt")?;
    let mut reader = BufReader::with_capacity(
        1024 * 1024 * 8, 
        f
    );

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("First line is {len} bytes long");
    println!("First line: \"{line}\" !");
    Ok(())
}
