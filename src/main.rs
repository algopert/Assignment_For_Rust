use warp::{http, Filter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    list: Vec<Vec<String>>,
}

async fn parse_body_func(item: Item) -> Result<impl warp::Reply, warp::Rejection> {
    let a = find_source(item.list[0][0].clone(), item.list.clone());
    let b = find_destination(item.list[0][1].clone(), item.list.clone());

    Ok(warp::reply::with_status(
        "[".to_owned() + &a + ", " + &b + "]",
        http::StatusCode::CREATED,
    ))
}

fn find_source(stp: String, list: Vec<Vec<String>>) -> String {
    for _item in &list {
        if _item[1] == stp {
            return find_source(_item[0].clone(), list);
        }
    }
    return stp;
}

fn find_destination(desp: String, list: Vec<Vec<String>>) -> String {
    for _item in &list {
        if _item[0] == desp {
            return find_destination(_item[1].clone(), list);
        }
    }
    return desp;
}

fn post_json() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let get_list = warp::post()
        .and(warp::path("list"))
        .and(warp::path::end())
        .and(post_json())
        .and_then(parse_body_func);

    warp::serve(get_list).run(([127, 0, 0, 1], 3030)).await;
}
