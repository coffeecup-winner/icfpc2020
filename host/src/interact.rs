use crate::eval::*;
use crate::syntax::*;
use crate::types::*;

fn interact(
    state: &mut State,
    protocol: &Var,
    st: NestedList,
    x: i64,
    y: i64,
) -> (Box<NestedList>, Box<NestedList>) {
    let coords = ap(ap(b(BuiltIn::Cons), number(x)), number(y));
    let protocol_run = ap(ap(var(protocol.clone()), st.into_value()), coords);
    let result = state.eval(protocol_run);
    let list = NestedList::from_value(result);
    if let NestedList::Cons(a, bc) = list {
        if let NestedList::Cons(b, cnil) = *bc {
            if let NestedList::Cons(c, nil) = *cnil {
                assert_eq!(*nil, NestedList::Nil);
                if let NestedList::Number(flag) = *a {
                    if flag == 0 {
                        return (b, c);
                    } else {
                        panic!("TODO: implement recursion");
                    }
                } else {
                    panic!("Invalid result");
                }
            } else {
                panic!("Invalid result")
            }
        } else {
            panic!("Invalid result");
        }
    } else {
        panic!("Invalid result");
    }
}

pub fn run_interaction(
    state: &mut State,
    protocol: &str,
    st: NestedList,
    x: i64,
    y: i64,
) -> (NestedList, Vec<Picture>) {
    let var_protocol = Var::Named(protocol.to_string());
    let (state, pic_list) = interact(state, &var_protocol, st, x, y);
    (*state, PictureBuilder::from_nested_list(*pic_list))
}
