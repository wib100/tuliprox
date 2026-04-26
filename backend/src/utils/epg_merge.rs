use shared::model::{EpgChannel, EpgProgramme};
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
pub(crate) struct ProgrammeMergeKey {
    start: i64,
    stop: i64,
}

impl From<&EpgProgramme> for ProgrammeMergeKey {
    fn from(programme: &EpgProgramme) -> Self {
        Self { start: programme.start, stop: programme.stop }
    }
}

pub(crate) fn dedupe_channel_programmes(channel: &mut EpgChannel) -> HashSet<ProgrammeMergeKey> {
    let mut seen = HashSet::new();
    channel.programmes.retain(|programme| seen.insert(ProgrammeMergeKey::from(programme)));
    seen
}

pub(crate) fn merge_missing_channel_programmes<I>(
    channel: &mut EpgChannel,
    programmes: &mut HashSet<ProgrammeMergeKey>,
    incoming: I,
) where
    I: IntoIterator<Item = EpgProgramme>,
{
    for programme in incoming {
        let key = ProgrammeMergeKey::from(&programme);
        if programmes.insert(key) {
            channel.programmes.push(programme);
        }
    }
}
