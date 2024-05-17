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
use HandleExternalAbort::*;
use common::*;
pub fn HandleExternalWriteAbort<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memstatus: ProductTypef8c3639b88223255,
    memaddrdesc: ProductTypece7c66ccb2cab13e,
    size: i128,
    accdesc: ProductType9878976b5bcce9c9,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        memstatus: ProductTypef8c3639b88223255,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        size: i128,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        memstatus,
        memaddrdesc,
        size,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // D s_0_1: read-var memstatus:struct
        let s_0_1: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_0_2: read-var memaddrdesc:struct
        let s_0_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_0_3: read-var size:i
        let s_0_3: i128 = fn_state.size;
        // D s_0_4: read-var accdesc:struct
        let s_0_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_0_5: call HandleExternalAbort(s_0_1, s_0_0, s_0_2, s_0_3, s_0_4)
        let s_0_5: () = HandleExternalAbort(
            state,
            tracer,
            s_0_1,
            s_0_0,
            s_0_2,
            s_0_3,
            s_0_4,
        );
        // N s_0_6: return
        return;
    }
}
