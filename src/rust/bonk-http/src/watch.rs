use std::{path::Path, time::Duration};

use notify_debouncer_full::{
    notify::{RecommendedWatcher, RecursiveMode, Watcher as _},
    DebounceEventResult, Debouncer, FileIdMap,
};
use tokio::sync::broadcast::Sender;

pub(crate) struct Watcher {
    #[allow(unused)] // this is just here so it doesn't get dropped
    debouncer: Debouncer<RecommendedWatcher, FileIdMap>,
    pub events_tx: Sender<()>,
}

impl Watcher {
    pub(crate) fn new(cfg: &Path) -> anyhow::Result<Self> {
        let (tx, _) = tokio::sync::broadcast::channel(16); // TODO capacity?
        let events_tx = tx.clone();

        let mut debouncer = notify_debouncer_full::new_debouncer(
            Duration::from_secs(2),
            None,
            move |res: DebounceEventResult| {
                if res.is_ok() {
                    let _ = tx.send(());
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
