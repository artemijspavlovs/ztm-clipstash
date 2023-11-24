use std::{collections::HashMap, sync::Arc, time::Duration};

use crossbeam_channel::{unbounded, Sender, TryRecvError};
use parking_lot::Mutex;
use tokio::runtime::Handle;

use crate::{data::DatabasePool, service, ServiceError, ShortCode};

#[derive(Debug, thiserror::Error)]
enum HitCountError {
    #[error("service error: {0}")]
    Service(#[from] ServiceError),
    #[error("communication error: {0}")]
    Channel(#[from] crossbeam_channel::SendError<HitCountMsg>),
}

type HitStore = Arc<Mutex<HashMap<ShortCode, u32>>>;

enum HitCountMsg {
    Commit,
    Hit(ShortCode, u32),
}

pub struct HitCounter {
    tx: Sender<HitCountMsg>,
}

impl HitCounter {
    fn commit_hits(
        hits: HitStore,
        handle: Handle,
        pool: DatabasePool,
    ) -> Result<(), HitCountError> {
        let hits = Arc::clone(&hits);
        let hits: Vec<(ShortCode, u32)> =
            {
                let mut hits = hits.lock();
                let hits_vec = hits.iter().map(|(k, v)| (k.clone(), *v)).collect();
                hits.clear();
                hits_vec
            };

        handle.block_on(async move {
            let transaction = service::action::begin_transaction(&pool).await?;

            for (shortcode, hits) in hits {
                if let Err(e) = service::action::increase_hit_count(&shortcode, hits, &pool).await {
                    eprintln!("failed to increase hit count: {}", e);
                }
            }

            Ok(service::action::end_transaction(transaction).await?)
        })
    }

    fn process_msg(
        msg: HitCountMsg,
        hits: HitStore,
        handle: Handle,
        pool: DatabasePool,
    ) -> Result<(), HitCountError> {
        match msg {
            HitCountMsg::Commit => Self::commit_hits(hits.clone(), handle.clone(), pool.clone())?,
            HitCountMsg::Hit(shortcode, count) => {
                let mut hitcount = hits.lock();
                let hitcount = hitcount.entry(shortcode).or_insert(0);

                *hitcount += count;
            }
        }

        Ok(())
    }

    pub fn new(pool: DatabasePool, handle: Handle) -> Self {
        let (sx, rx) = unbounded();
        let sx_clone = sx.clone();
        let rx_clone = rx.clone();

        let _ = std::thread::spawn(move || {
            println!("HitCounter thread spawned");

            let store: HitStore = Arc::new(Mutex::new(HashMap::new()));

            loop {
                match rx_clone.try_recv() {
                    Ok(msg) => {
                        if let Err(e) =
                            Self::process_msg(msg, store.clone(), handle.clone(), pool.clone())
                        {
                            eprintln!("message processing failed: {}", e);
                        }
                    }

                    Err(e) => match e {
                        TryRecvError::Empty => {
                            std::thread::sleep(Duration::from_secs(5));
                            if let Err(e) = sx_clone.send(HitCountMsg::Commit) {
                                eprintln!("failed to send commit msg to hits channel: {}", e)
                            }
                        }
                        _ => break,
                    },
                }
            }
        });
        Self { tx: sx }
    }

    pub fn hit(&self, shortcode: ShortCode, count: u32) {
        if let Err(e) = self.tx.send(HitCountMsg::Hit(shortcode, count)) {
            eprintln!("hit count error: {}", e);
        }
    }
}
