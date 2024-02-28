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

use deno::deno_ast::*;

#[test]
fn test_ast_transpile() {
  let specifier = ModuleSpecifier::parse("https://deno.land/x/mod.ts").unwrap();
  let ts_code = "function add(x: Number, y: Number) { return x + y; }";
  let expected_js_code = "function add(x, y) {\n  return x + y;\n}";
  let module = parse_module(ParseParams {
    specifier,
    text_info: SourceTextInfo::from_string(ts_code.to_string()),
    media_type: MediaType::TypeScript,
    capture_tokens: false,
    maybe_syntax: None,
    scope_analysis: false,
  })
  .unwrap();
  let transpiled_js_code = module.transpile(&EmitOptions::default()).unwrap();
  assert_eq!(expected_js_code, &transpiled_js_code.text[..expected_js_code.len()]);
  assert!(transpiled_js_code
    .text
    .contains("\n//# sourceMappingURL=data:application/json;base64,"));
  assert!(transpiled_js_code.source_map.is_none());
}
