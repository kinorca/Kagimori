// Copyright 2025 SiLeader.
//
// This file is part of Kagimori.
//
// Kagimori is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any later version.
//
// Kagimori is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Kagimori.
// If not, see <https://www.gnu.org/licenses/>.

use std::env::var;
use std::path::PathBuf;

fn main() {
    let mut builder = tonic_prost_build::configure()
        .build_server(true)
        .build_client(false);
    #[cfg(feature = "reflection")]
    {
        builder = builder.file_descriptor_set_path(
            PathBuf::from(var("OUT_DIR").unwrap()).join("kagimori_descriptor.bin"),
        );
    }

    builder
        .compile_protos(&["proto/kagimori.proto", "proto/api.proto"], &["proto"])
        .unwrap();
}
