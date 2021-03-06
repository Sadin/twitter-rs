// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate egg_mode;

mod common;

use common::tokio_core::reactor;

use egg_mode::search::{self, ResultType};

fn main() {
    let mut core = reactor::Core::new().unwrap();

    let config = common::Config::load(&mut core);
    let handle = core.handle();

    //rust tweets around dallas
    let search = core.run(search::search("rustlang")
                                 .result_type(ResultType::Recent)
                                 .count(10)
                                 .call(&config.token, &handle)).unwrap();

    for tweet in &search.statuses {
        common::print_tweet(tweet);
    }
}
