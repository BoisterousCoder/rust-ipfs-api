// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

use request::ApiRequest;


#[derive(Serialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FileLs<'a> {
    #[serde(rename = "arg")]
    pub path: &'a str,
}

impl<'a> ApiRequest for FileLs<'a> {
    #[inline]
    fn path() -> &'static str {
        "/file/ls"
    }
}
