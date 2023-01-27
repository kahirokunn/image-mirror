mod cache;
mod models;
mod registry_client;

use crate::models::Tag;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <repository_url>", args[0]);
        std::process::exit(1);
    }
    let mut exit_code = 0;
    let repository_url = &args[1];
    let repository =
        cache::find_or_initialize(&repository_url).expect("failed to find or initialize");
    println!("cached tags: {:?}", repository.tags);
    let current_tags = registry_client::get_tags(&repository).expect("failed to get tags");
    println!("current tags: {:?}", current_tags);
    let new_tags = diff(&repository.tags, &current_tags);
    println!("new tags: {:?}", new_tags);
    for mut tag in new_tags {
        match tag.sync(&repository_url) {
            Ok(_) => cache::update_tag(repository_url, &tag).expect("failed to update tag"),
            Err(e) => {
                println!("failed to sync: {:?}", e);
                exit_code = 1;
            }
        };
    }
    std::process::exit(exit_code);
}

fn diff(before: &Vec<Tag>, after: &Vec<Tag>) -> Vec<Tag> {
    after
        .iter()
        .filter(|after_tag| {
            before
                .iter()
                .find(|before_tag| after_tag.eq(before_tag))
                .is_none()
        })
        .cloned()
        .collect()
}
