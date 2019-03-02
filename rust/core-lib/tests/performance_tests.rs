// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate serde_json;
extern crate test;
extern crate xi_core_lib;
extern crate xi_rpc;

use test::Bencher;
use std::io;

use xi_core_lib::test_helpers;
use xi_core_lib::XiCore;
use xi_rpc::test_utils::{make_reader, test_channel};
use xi_rpc::{ReadError, RpcLoop};

#[bench]
/// Tests the handler's responsiveness to a standard startup sequence.
fn test_startup(b: &mut Bencher) {
    let mut state = XiCore::new();
    let (tx, mut rx) = test_channel();
    let mut rpc_looper = RpcLoop::new(tx);
    let json = make_reader(
        r#"{"method":"client_started","params":{}}
{"method":"set_theme","params":{"theme_name":"InspiredGitHub"}}"#,
    );
    assert!(rpc_looper.mainloop(|| json, &mut state).is_ok());
    rx.expect_rpc("available_languages");
    rx.expect_rpc("available_themes");
    rx.expect_rpc("theme_changed");

    let json = make_reader(r#"{"id":0,"method":"new_view","params":{}}"#);
    assert!(rpc_looper.mainloop(|| json, &mut state).is_ok());
    assert_eq!(rx.expect_response(), Ok(json!("view-id-1")));
    rx.expect_rpc("available_plugins");
    rx.expect_rpc("config_changed");
    rx.expect_rpc("language_changed");
    rx.expect_rpc("update");
    rx.expect_rpc("scroll_to");
    rx.expect_nothing();
}