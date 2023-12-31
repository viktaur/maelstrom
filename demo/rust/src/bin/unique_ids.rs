use std::any::Any;
use std::collections::{HashMap, HashSet};
/// ```bash
/// $ cargo build
/// $ maelstrom test -w echo --bin ./target/debug/unique-ids --node-count 1 --time-limit 10 --log-stderr
/// ````
use async_trait::async_trait;
use maelstrom::protocol::Message;
use maelstrom::{done, Node, Result, Runtime};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use serde_json::{json};
use uuid::Uuid;

pub(crate) fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let handler = Arc::new(Handler::default());
    Runtime::new().with_handler(handler).run().await
}

#[derive(Clone, Default)]
struct Handler {
    inner: Arc<Mutex<HashSet<u64>>>
}

#[async_trait]
impl Node for Handler {
    async fn process(&self, runtime: Runtime, req: Message) -> Result<()> {

        let msg: Result<Request> = req.body.as_obj();

        match msg {
            Ok(Request::Init {}) => {
                let res_body = req.body.clone().with_type("init_ok");
                return runtime.reply(req, res_body).await;
            },
            Ok(Request::Generate {}) => {
                let id = Uuid::new_v4().hyphenated().to_string();

                let mut res_body = req.body.clone().with_type("generate_ok");
                res_body.extra.insert("id".into(), json!(id));
                return runtime.reply(req, res_body).await;
            }
            _ => done(runtime, req)

        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Request {
    Init {},
    Generate {}
}
