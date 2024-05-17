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
use report_tlbi::*;
use u_TLBInvalidate::*;
use common::*;
pub fn TLBI<T: Tracer>(
    state: &mut State,
    tracer: &T,
    r: ProductTypefb7b2cabacce34a2,
    s: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: ProductTypefb7b2cabacce34a2,
        s: u32,
    }
    let fn_state = FunctionState {
        r,
        s,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var r:struct
        let s_0_0: ProductTypefb7b2cabacce34a2 = fn_state.r;
        // D s_0_1: read-var s:u32
        let s_0_1: u32 = fn_state.s;
        // D s_0_2: call report_tlbi(s_0_0, s_0_1)
        let s_0_2: () = report_tlbi(state, tracer, s_0_0, s_0_1);
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call _TLBInvalidate(s_0_3)
        let s_0_4: () = u_TLBInvalidate(state, tracer, s_0_3);
        // N s_0_5: return
        return;
    }
}
