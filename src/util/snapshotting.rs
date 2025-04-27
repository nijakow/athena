
pub trait Snapshottable
{
    type Snapshot;

    fn from_snapshot(&mut self, snapshot: Self::Snapshot);
    fn take_snapshot(&self) -> Self::Snapshot;
}
