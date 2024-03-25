use std::{
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};

use notify_debouncer_full::{
    notify::{RecommendedWatcher, RecursiveMode, Watcher as _},
    DebounceEventResult, Debouncer, FileIdMap,
};
use tokio::sync::broadcast::Sender;

use crate::MutableAppState;

pub(crate) struct Watcher {
    #[allow(unused)] // this is just here so it doesn't get dropped
    debouncer: Debouncer<RecommendedWatcher, FileIdMap>,
    pub events_tx: Sender<()>,
}

impl Watcher {
    pub(crate) fn new(cfg: &Path, mutable: Arc<Mutex<MutableAppState>>) -> anyhow::Result<Self> {
        let cfg_for_callback = cfg.to_path_buf().clone();

        let (tx, _) = tokio::sync::broadcast::channel(16); // TODO capacity?
        let events_tx = tx.clone();

        let mut debouncer = notify_debouncer_full::new_debouncer(
            Duration::from_secs(2),
            None,
            move |res: DebounceEventResult| {
                if res.is_ok() {
                    // rebuild mutable state
                    match MutableAppState::new(&cfg_for_callback) {
                        Ok(new_state) => {
                            *mutable.lock().expect("mutable state lock poisoned") = new_state;
                            // notify listeners of state change
                            let _ = tx.send(());
                        }
                        Err(err) => {
                            eprintln!("error updating state, not reloading: {}", err)
                        }
                    }
                }
            },
        )
        .unwrap();

        debouncer
            .watcher()
            .watch(cfg, RecursiveMode::NonRecursive)?;

        Ok(Self {
            debouncer,
            events_tx,
        })
    }
}
