
pub trait Snapshottable
{
    type Snapshot;

    fn from_snapshot(snapshot: Self::Snapshot) -> Self;
    fn take_snapshot(&self) -> Self::Snapshot;
}
