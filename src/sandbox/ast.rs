use super::{HKT,OptionKind, Empty};
enum Node<K,A,L> {
    Op {
        op: A,
        next: L,
    },
    Result {
        result: A,
        next: L,
    },
    __MARKER(K),
}

trait Lazy {}

}

