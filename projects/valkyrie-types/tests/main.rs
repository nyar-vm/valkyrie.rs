use valkyrie_types::testing::assert_type;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_primitive() {
    let value: usize = 0;
    assert_type(value, "Unsigned64", "std::primitive::Unsigned64");
    let value: f64 = 0.0;
    assert_type(value, "Float64", "std::primitive::Float64");
}

#[test]
fn test_list() {
    let value: Vec<usize> = vec![];
    assert_type(value, "Array[Unsigned64]", "std::collection::Array[std::primitive::Unsigned64]");
    // let value: Option<usize> = None;
    // assert_type(value, "Option[u64]", "std::primitive::Option[std::primitive::u64]");
    // let value: Option<Option<usize>> = Some(None);
    // assert_type(value, "Option[Option[u64]]", "std::primitive::Option[std::primitive::Option]");
}

#[test]
fn test_option() {
    let value: Option<usize> = Some(0);
    assert_type(value, "Option[Unsigned64]", "std::primitive::Option[std::primitive::Unsigned64]");
    let value: Option<usize> = None;
    assert_type(value, "Option[Unsigned64]", "std::primitive::Option[std::primitive::Unsigned64]");
    // let value: Option<Option<usize>> = Some(None);
    // assert_type(value, "Option[Option[u64]]", "std::primitive::Option[std::primitive::Option]");
}

#[test]
fn test_tuple() {
    let value: (u8, u16, u32, u64) = (0, 0, 0, 0);
    assert_type(
        value,
        "Tuple[Unsigned8, Unsigned16, Unsigned32, Unsigned64]",
        "std::primitive::Tuple[std::primitive::Unsigned8, std::primitive::Unsigned16, std::primitive::Unsigned32, std::primitive::Unsigned64]",
    );
    // let value: Option<Option<usize>> = Some(None);
    // assert_type(value, "Option[Option[u64]]", "std::primitive::Option[std::primitive::Option]");
}

pub fn check_raw(list: &[u32]) -> bool {
    for i in list {
        if i % 4 == 0 {
            continue;
        }
        else if i & 2 == 0 {
            break;
        }
        else {
            return true;
        }
    }
    println!("Even!");
    false
}

pub struct JumpTable {
    save_state: u32,
    this_state: u32,
    table: Vec<fn() -> u32>,
}

// pub fn check_raw(list: &[u8]) {
//     let mut iter = list.iter();
//     let mut next = iter.next();
//     @1
//     while next.is_some() {
//         @2
//         let i = next.unwrap();
//         if i % 4 == 0 {
//             next = iter.next();
//             goto @2
//         }
//         else if i & 2 == 0 {
//             goto @3
//         }
//         else {
//             ret = true
//             goto @4
//         }
//     }
//     @3
//     println!("Even!")
//     ret = false
//     @4
//     ret
// }
pub fn check_fsm(list: &[u32]) -> bool {
    let mut _state: u32 = 1;
    let mut _iter = list.iter();
    let mut _next = _iter.next();
    let mut _ret = false;
    while _state > 0 {
        match _state {
            1 => {
                if _next.is_some() {
                    _state = 2
                }
                else {
                    _state = 3
                }
            }
            2 => {
                let i = unsafe { _next.unwrap_unchecked() };
                if i % 4 == 0 {
                    _next = _iter.next();
                    _state = 2
                }
                else if i & 2 == 0 {
                    _state = 3
                }
                else {
                    _ret = true;
                    break;
                }
            }
            3 => {
                println!("Even!");
                break;
            }
            _ => break,
        }
    }
    _ret
}

#[test]
fn test() {
    let list = vec![6, 7, 8];
    println!("RAW: {}", check_raw(&list));
    println!("FSM: {}", check_fsm(&list));
}
