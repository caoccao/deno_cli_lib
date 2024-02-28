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
fn test_ast_parse_module() {
  let url = ModuleSpecifier::parse("file:///test.ts").unwrap();
  let ts_code = "function add(x: Number, y: Number) { return x + y; }";
  let expected_js_code = "function add(x, y) {\n  return x + y;\n}";
  let parsed_source = parse_module(ParseParams {
    specifier: url,
    text_info: SourceTextInfo::from_string(ts_code.to_string()),
    media_type: MediaType::TypeScript,
    capture_tokens: false,
    maybe_syntax: None,
    scope_analysis: false,
  })
  .unwrap();
  assert_eq!(parsed_source.specifier().as_str(), "file:///test.ts");
  assert_eq!(parsed_source.text_info().text_str(), ts_code);
  assert_eq!(parsed_source.media_type(), MediaType::TypeScript);
  let transpiled_js_code = parsed_source.transpile(&EmitOptions::default()).unwrap();
  assert_eq!(expected_js_code, &transpiled_js_code.text[..expected_js_code.len()]);
  assert!(transpiled_js_code
    .text
    .contains("\n//# sourceMappingURL=data:application/json;base64,"));
  assert!(transpiled_js_code.source_map.is_none());
}

#[test]
fn test_ast_parse_program() {
  let url = ModuleSpecifier::parse("file:///test.ts").unwrap();
  let ts_code = "function add(x: Number, y: Number) { return x + y; }";
  let expected_js_code = "function add(x, y) {\n  return x + y;\n}";
  let parsed_source = parse_program(ParseParams {
    specifier: url,
    text_info: SourceTextInfo::from_string(ts_code.to_string()),
    media_type: MediaType::TypeScript,
    capture_tokens: false,
    maybe_syntax: None,
    scope_analysis: false,
  })
  .unwrap();
  assert_eq!(parsed_source.specifier().as_str(), "file:///test.ts");
  assert_eq!(parsed_source.text_info().text_str(), ts_code);
  assert_eq!(parsed_source.media_type(), MediaType::TypeScript);
  assert!(matches!(
    parsed_source.script().body[0],
    crate::swc::ast::Stmt::Decl(..)
  ));
  let transpiled_js_code = parsed_source.transpile(&EmitOptions::default()).unwrap();
  assert_eq!(expected_js_code, &transpiled_js_code.text[..expected_js_code.len()]);
  assert!(transpiled_js_code
    .text
    .contains("\n//# sourceMappingURL=data:application/json;base64,"));
  assert!(transpiled_js_code.source_map.is_none());
}
