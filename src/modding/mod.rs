pub mod view;
//pub mod annotate;
pub mod extract;
pub mod merge;
pub mod patch;
pub mod database;
pub mod texture;
pub mod config;


//todo implement conflict checker
struct _Change {
    path: String, 
    original_value: String, 
    new_value: String, 

    patch: Option<String>,
    attribute: Option<String>
}

impl _Change {
    fn _new(path: String, original_value: String, new_value: String) -> _Change {
        _Change {
            path,
            original_value,
            new_value,

            patch: None,
            attribute: None,
        }
    }
}
