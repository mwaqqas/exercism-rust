#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let l1 = _first_list.len();
    let l2 = _second_list.len();

    match (l1, l2) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (l1, l2) if l1 > l2 => match _first_list.windows(l2).any(|slice| slice == _second_list) {
            true => Comparison::Superlist,
            false => Comparison::Unequal,
        },
        (l1, l2) if l1 < l2 => match _second_list.windows(l1).any(|slice| slice == _first_list) {
            true => Comparison::Sublist,
            false => Comparison::Unequal,
        },
        (_, _) => match _first_list == _second_list {
            true => Comparison::Equal,
            false => Comparison::Unequal,
        },
    }
}
