/*
* Copyright (c) 2024. caoccao.com Sam Cao
* All rights reserved.

* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at

* http://www.apache.org/licenses/LICENSE-2.0

* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

use deno::tsc::*;

#[test]
fn test_tsc_get_types_declaration_file_text() {
  let text = get_types_declaration_file_text();
  assert!(text.len() > 4096);
}

#[test]
fn test_tsc_lazily_loaded_static_assets() {
  let map = &LAZILY_LOADED_STATIC_ASSETS;
  assert!(map.len() > 0);
}
