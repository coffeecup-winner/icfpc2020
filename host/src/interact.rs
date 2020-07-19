use crate::eval::*;
use crate::modem::*;
use crate::send::request;
use crate::syntax::*;
use crate::types::*;

fn interact(
    state: &mut State,
    protocol: &Var,
    st: NestedList,
    coords: NestedList,
) -> (NestedList, NestedList) {
    let coords = coords.into_value();
    let protocol_run = ap(ap(var(protocol.clone()), st.into_value()), coords);
    let result = state.eval(protocol_run);
    let list = NestedList::from_value(result);
    let (a, bcnil) = list.unwrap_cons();
    let (b, cnil) = bcnil.unwrap_cons();
    let (c, nil) = cnil.unwrap_cons();
    assert_eq!(nil, NestedList::Nil);
    let flag = a.unwrap_number();
    if flag == 0 {
        return (b, c);
    } else {
        let signal = mod_list(&c);
        let signal_data: Vec<_> = signal
            .into_iter()
            .map(|x| if x { b'1' } else { b'0' })
            .collect();
        let endpoint = String::from("https://icfpc2020-api.testkontur.ru/aliens/send");
        let token = std::env::var("ICFPC_TEAM_TOKEN").ok();
        let response = match request(&endpoint, token, signal_data) {
            Ok(val) => val,
            Err(err) => panic!("request failed: {:?}", err),
        };

        let signal: Vec<_> = response
            .into_iter()
            .map(|c| if c == b'1' { true } else { false })
            .collect();

        return interact(state, protocol, b, dem_list(&signal));
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
    let (state, pic_list) = interact(
        state,
        &var_protocol,
        st,
        NestedList::Cons(
            Box::new(NestedList::Number(x)),
            Box::new(NestedList::Number(y)),
        ),
    );
    (state, PictureBuilder::from_nested_list(pic_list))
}
