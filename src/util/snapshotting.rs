
pub trait Snapshottable<'a>
{
    type Snapshot;
    type Parameter;

    fn from_snapshot(&mut self, snapshot: Self::Snapshot);
    fn take_snapshot(&self, parameter: Self::Parameter) -> Self::Snapshot;
}
