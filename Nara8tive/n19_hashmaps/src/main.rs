use std::collections::HashMap;

#[derive(Debug)]
struct Content {
    content: String,
}

fn main() {
    let mut content_list = HashMap::new();

    content_list.insert(
        1,
        Content {
            content: "content1".to_string(),
        },
    );
    content_list.insert(
        2,
        Content {
            content: "content2".to_string(),
        },
    );
    content_list.insert(
        3,
        Content {
            content: "content3".to_string(),
        },
    );
    content_list.insert(
        4,
        Content {
            content: "content4".to_string(),
        },
    );

    for (key, value) in content_list {
        println!("key is {:#?} and content is {:#?}", key, value)
    }
}
