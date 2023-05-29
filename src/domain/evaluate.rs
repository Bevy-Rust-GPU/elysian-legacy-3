use crate::{Subtree, Split, SplitF, SplitT, DistanceF32};

pub type Evaluate = Split<DistanceF32, Subtree>;
pub type EvaluateT<T> = SplitT<T, DistanceF32, Subtree>;
pub type EvaluateF = SplitF<DistanceF32, Subtree>;

