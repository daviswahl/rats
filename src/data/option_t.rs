use kind::{Empty, Kind, HKT};
use kinds::OptionKind;
struct OptionT<K, A, Z = Empty>
where
    K: HKT,
{
    value: Kind<K, Kind<OptionKind, A, Z>>,
}
