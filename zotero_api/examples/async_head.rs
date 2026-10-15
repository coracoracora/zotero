use tokio;
use zotero_api::{Zotero, ZoteroApi, ZoteroApiAsyncExecutor, LAST_MODIFIED_VERSION};

#[tokio::main]
async fn main() {
    let zotero_api = Zotero::set_group("1234567", None);
    let result = zotero_api
        .as_head_request(ZoteroApi::get_items, None)
        .fetch_headers(Some(vec![LAST_MODIFIED_VERSION]))
        .await;

    match result {
        Ok(headers) => println!("{headers:#?}"),
        Err(err) => println!("{err:?}"),
    };

    let result = zotero_api
        .as_head_request(ZoteroApi::get_items, None)
        .fetch_header(LAST_MODIFIED_VERSION)
        .await;

    match result {
        Ok(header) => println!("{header:#?}"),
        Err(err) => println!("{err:?}"),
    };
}
