#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use common::*;
pub fn ConstrainUnpredictableBits<T: Tracer>(
    state: &mut State,
    tracer: &T,
    which: u32,
    width: i128,
) -> ProductType690b94b58c91cec7 {
    #[derive(Default)]
    struct FunctionState {
        ga_398838: ProductType690b94b58c91cec7,
        which: u32,
        width: i128,
    }
    let fn_state = FunctionState {
        which,
        width,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType690b94b58c91cec7 {
        // D s_0_0: read-var ga#398838:struct
        let s_0_0: ProductType690b94b58c91cec7 = fn_state.ga_398838;
        // N s_0_1: return s_0_0
        return s_0_0;
    }
}
