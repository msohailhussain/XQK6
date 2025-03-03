// fn main() {
//     println!("Hello, world!");
// }

use axum::{
    extract::{Path, State}, http::StatusCode, routing::{get, post, Route}, Json, Router
};
use serde::{Deserialize, Serialize};



use std::{collections::HashMap, io, sync::{Arc, Mutex}};


#[derive(Clone, Default, Serialize, Deserialize)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool
}

#[derive(Clone, Default)]
struct StorageLayer {
    db: Arc<Mutex<HashMap<String, Movie>>>
}


impl StorageLayer {
    pub fn new() -> Self {
        Self { db: Arc::default() }
    }

    pub fn get_movie(&self, id: String) -> Option<Movie> {
        let db = self.db.lock().unwrap();
        let movie = db.get(&id);
        if let Some(movie) = movie {
            return Some(movie.clone());
        }
        None
    }

    pub fn save_movie(&self, movie: Movie) {
        let mut db = self.db.lock().unwrap();
        db.insert(movie.id.clone(), movie);
    }
}


#[tokio::main]
async fn main() {
    let state = StorageLayer::new();

    let app = Router::new()
        .route("/movie/:id", get(get_movie))
        .route("/movie", post(save_movie)
        ).with_state(state);

    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));


    // let app1 = Router::new()
    //     .route("/movie/:id", get( || async {
    //         "Hello World".to_string()
    //     }));


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();


    // Create Axum server with the following endpoints:
    // 1. GET /movie/{id} - This should return back a movie given the id
    // 2. POST /movie - this should save move in a DB (HashMap<String, Movie>). This movie will be sent
    // via a JSON payload.

    // As a bonus: implement a caching layer so we don't need to make expensive "DB" lookups, etc.


}


async fn get_movie(State(state): State<StorageLayer>, Path(id): Path<String>) -> Result<Json<Movie>, StatusCode> {
    let movie = state.get_movie(id);
    if let Some(movie) = movie {
        Ok(Json(movie))
    } else {
        // todo: need to send status code.
        Err(StatusCode::NOT_FOUND)
    }
}

async fn save_movie(State(state): State<StorageLayer>, Json(movie): Json<Movie>) ->   StatusCode {
    state.save_movie(movie);
    StatusCode::OK
}
