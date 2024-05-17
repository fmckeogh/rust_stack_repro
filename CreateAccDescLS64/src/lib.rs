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
use NewAccDesc::*;
use HaveTME::*;
use common::*;
pub fn CreateAccDescLS64<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memop: u32,
    tagchecked: bool,
) -> ProductType9878976b5bcce9c9 {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        gs_7290: bool,
        memop: u32,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        memop,
        tagchecked,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_0_0: const #1u : u32
        let s_0_0: u32 = 1;
        // S s_0_1: call NewAccDesc(s_0_0)
        let s_0_1: ProductType9878976b5bcce9c9 = NewAccDesc(state, tracer, s_0_0);
        // D s_0_2: write-var accdesc <= s_0_1
        fn_state.accdesc = s_0_1;
        // D s_0_3: read-var memop:u32
        let s_0_3: u32 = fn_state.memop;
        // C s_0_4: const #0u : u32
        let s_0_4: u32 = 0;
        // D s_0_5: cmp-eq s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) == (s_0_4));
        // D s_0_6: write-var accdesc.23 <= s_0_5
        fn_state.accdesc._23 = s_0_5;
        // D s_0_7: read-var memop:u32
        let s_0_7: u32 = fn_state.memop;
        // C s_0_8: const #1u : u32
        let s_0_8: u32 = 1;
        // D s_0_9: cmp-eq s_0_7 s_0_8
        let s_0_9: bool = ((s_0_7) == (s_0_8));
        // D s_0_10: write-var accdesc.32 <= s_0_9
        fn_state.accdesc._32 = s_0_9;
        // C s_0_11: const #1u : u8
        let s_0_11: bool = true;
        // D s_0_12: write-var accdesc.20 <= s_0_11
        fn_state.accdesc._20 = s_0_11;
        // C s_0_13: const #1u : u8
        let s_0_13: bool = true;
        // D s_0_14: write-var accdesc.13 <= s_0_13
        fn_state.accdesc._13 = s_0_13;
        // D s_0_15: read-var tagchecked:u8
        let s_0_15: bool = fn_state.tagchecked;
        // D s_0_16: write-var accdesc.28 <= s_0_15
        fn_state.accdesc._28 = s_0_15;
        // C s_0_17: const #() : ()
        let s_0_17: () = ();
        // S s_0_18: call HaveTME(s_0_17)
        let s_0_18: bool = HaveTME(state, tracer, s_0_17);
        // N s_0_19: branch s_0_18 b3 b1
        if s_0_18 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#7290 <= s_1_0
        fn_state.gs_7290 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_2_0: read-var gs#7290:u8
        let s_2_0: bool = fn_state.gs_7290;
        // D s_2_1: write-var accdesc.30 <= s_2_0
        fn_state.accdesc._30 = s_2_0;
        // D s_2_2: read-var accdesc:struct
        let s_2_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // N s_2_3: return s_2_2
        return s_2_2;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_3_0: const #100180u : u32
        let s_3_0: u32 = 100180;
        // D s_3_1: read-reg s_3_0:i
        let s_3_1: i128 = {
            let value = state.read_register::<i128>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // C s_3_2: const #0s : i
        let s_3_2: i128 = 0;
        // D s_3_3: cmp-gt s_3_1 s_3_2
        let s_3_3: bool = ((s_3_1) > (s_3_2));
        // D s_3_4: write-var gs#7290 <= s_3_3
        fn_state.gs_7290 = s_3_3;
        // N s_3_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
