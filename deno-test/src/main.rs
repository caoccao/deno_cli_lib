/*
* Copyright (c) 2024. caoccao.com Sam Cao
* All rights reserved.

* Licensed under the Apache License, Version 2.0 (the "License")
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at

* http://www.apache.org/licenses/LICENSE-2.0

* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

use deno::deno_core::*;

fn main() {
  println!("V8 version: {}", v8_version());
  let mut runtime = JsRuntime::new(RuntimeOptions { ..Default::default() });
  runtime
    .execute_script_static(
      "test.js",
      r#"
    // Print helper function, calling Deno.core.print()
    function print(value) {
      Deno.core.print(value.toString()+"\n");
    }
    print(`1+1=${1+1}`);
    "#,
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
  use deno::deno_core::*;

  #[test]
  fn test_add() {
    let mut runtime = JsRuntime::new(RuntimeOptions { ..Default::default() });
    let result = runtime.execute_script_static("test.js", "1+1").unwrap();
    let value;
    unsafe {
      let mut handle_scope = runtime.handle_scope();
      value = result.into_raw().as_ref().to_int32(&mut handle_scope).unwrap().int32_value(&mut handle_scope).unwrap();
    }
    assert_eq!(2, value);
  }
}
