#![feature(async_closure)]

use std::{collections::HashMap, time::Instant, sync::Arc, num::NonZeroU16, env};

mod lua;
mod db;

use common::{FindGame, GameInfo, FoundGame};
use db::get_info_from_game_id;
use lua::new_lua;
use mlua::Lua;
use parking_lot::Mutex;
use anyhow::anyhow;
use tide::{Response, prelude::Listener};
use tide_websockets::{WebSocket, WebSocketConnection};

type Request = tide::Request<Arc<Mutex<State>>>;

#[tokio::main]
async fn main() -> Result<(), tide::Error> {
    femme::start();
    let state: Arc<Mutex<State>> = Arc::new(Mutex::new(State::new()));
    let mut app = tide::with_state(state);
    app.at("/play/:sourceid/:gameid").get(WebSocket::new(play));
    app.at("/findgame").post(findgame);

    db::init();
    app.bind(env::var("HYPATIA_SERVER_ENDPOINT").expect("Please supply a server endpoint")).await?.accept().await?;
    Ok(())
}

async fn play(req: Request, mut _stream: WebSocketConnection) -> Result<(), tide::Error> {
    let sourceid = req.param("sourceid")?.parse::<u128>()?;
    let gameid = req.param("gameid")?.parse::<u128>()?;
    let state = req.state().lock();

    let v = state.games.get(&sourceid);

    if let None = v {
        return Err(tide::Error::new(404, anyhow!(serde_json::to_string(&common::Error {
            code: NonZeroU16::new(404).unwrap(),
            reason: "Game not found".into()
        })?)))
    }

    let v = v.unwrap().get(&gameid);

    if let None = v {
        return Err(tide::Error::new(404, anyhow!(serde_json::to_string(&common::Error {
            code: NonZeroU16::new(404).unwrap(),
            reason: "Game not found".into()
        })?)))
    }

    let v = v.unwrap();

    
    
    Ok(())
}

async fn findgame(mut req: Request) -> tide::Result {
    let find: FindGame = req.body_json().await?;
    let mut resp = Response::new(200);
    let mut state = req.state().lock();
    
    if state.games.contains_key(&find.game) {
        state.games.insert(find.game, HashMap::new());
    }

    let mut found: Option<(&u128, &Game)> = None;
    for (id, game) in &state.games[&find.game] {
        match &mut found {
            Some(found) => {
                if found.1.players.len() > game.players.len() {
                    *found = (id, game)
                }
            }
            None => {
                found = Some((id, game))
            }
        }
    }

    let mut id: Option<u128> = match found {
        Some((v, _)) => {
            Some(*v)
        }
        None => {
            None
        }
    };


    if let None = found {
        let info = get_info_from_game_id(find.game).await?;
        let game = Game {
            players: HashMap::new(),
            started_at: Instant::now(),
            lua: new_lua(find.game),
            info
        };
        let tid = uuid::Uuid::new_v4().as_u128();
        state.games.get_mut(&find.game).unwrap().insert(tid, game);
        id = Some(tid);
    }

    resp.set_body(serde_json::to_string(&FoundGame {
        id: id.unwrap()
    })?);

    Ok::<tide::Response, tide::Error>(resp)
}

struct State {
    games: HashMap<u128, HashMap<u128, Game>>
}

impl State {
    fn new() -> Self {
        Self { 
            games: HashMap::new()
        }
    }
}

struct Game {
    info: GameInfo,
    players: HashMap<u128, ()>,
    started_at: Instant,
    lua: Lua
}