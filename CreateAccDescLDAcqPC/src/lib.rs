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
pub fn CreateAccDescLDAcqPC<T: Tracer>(
    state: &mut State,
    tracer: &T,
    tagchecked: bool,
) -> ProductType9878976b5bcce9c9 {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        gs_7276: bool,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
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
        // C s_0_3: const #1u : u8
        let s_0_3: bool = true;
        // D s_0_4: write-var accdesc.2 <= s_0_3
        fn_state.accdesc._2 = s_0_3;
        // C s_0_5: const #1u : u8
        let s_0_5: bool = true;
        // D s_0_6: write-var accdesc.23 <= s_0_5
        fn_state.accdesc._23 = s_0_5;
        // C s_0_7: const #1u : u8
        let s_0_7: bool = true;
        // D s_0_8: write-var accdesc.20 <= s_0_7
        fn_state.accdesc._20 = s_0_7;
        // D s_0_9: read-var tagchecked:u8
        let s_0_9: bool = fn_state.tagchecked;
        // D s_0_10: write-var accdesc.28 <= s_0_9
        fn_state.accdesc._28 = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call HaveTME(s_0_11)
        let s_0_12: bool = HaveTME(state, tracer, s_0_11);
        // N s_0_13: branch s_0_12 b3 b1
        if s_0_12 {
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
        // D s_1_1: write-var gs#7276 <= s_1_0
        fn_state.gs_7276 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_2_0: read-var gs#7276:u8
        let s_2_0: bool = fn_state.gs_7276;
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
        // D s_3_4: write-var gs#7276 <= s_3_3
        fn_state.gs_7276 = s_3_3;
        // N s_3_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
