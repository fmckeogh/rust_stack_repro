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
use u__IMPDEF_boolean::*;
use HaveTME::*;
use common::*;
pub fn CreateAccDescSME<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memop: u32,
    nontemporal: bool,
    contiguous: bool,
    tagchecked: bool,
) -> ProductType9878976b5bcce9c9 {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        gs_7388: bool,
        memop: u32,
        nontemporal: bool,
        contiguous: bool,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        memop,
        nontemporal,
        contiguous,
        tagchecked,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_0_0: const #4u : u32
        let s_0_0: u32 = 4;
        // S s_0_1: call NewAccDesc(s_0_0)
        let s_0_1: ProductType9878976b5bcce9c9 = NewAccDesc(state, tracer, s_0_0);
        // D s_0_2: write-var accdesc <= s_0_1
        fn_state.accdesc = s_0_1;
        // D s_0_3: read-var nontemporal:u8
        let s_0_3: bool = fn_state.nontemporal;
        // D s_0_4: write-var accdesc.18 <= s_0_3
        fn_state.accdesc._18 = s_0_3;
        // D s_0_5: read-var memop:u32
        let s_0_5: u32 = fn_state.memop;
        // C s_0_6: const #0u : u32
        let s_0_6: u32 = 0;
        // D s_0_7: cmp-eq s_0_5 s_0_6
        let s_0_7: bool = ((s_0_5) == (s_0_6));
        // D s_0_8: write-var accdesc.23 <= s_0_7
        fn_state.accdesc._23 = s_0_7;
        // D s_0_9: read-var memop:u32
        let s_0_9: u32 = fn_state.memop;
        // C s_0_10: const #1u : u32
        let s_0_10: u32 = 1;
        // D s_0_11: cmp-eq s_0_9 s_0_10
        let s_0_11: bool = ((s_0_9) == (s_0_10));
        // D s_0_12: write-var accdesc.32 <= s_0_11
        fn_state.accdesc._32 = s_0_11;
        // C s_0_13: const #1u : u8
        let s_0_13: bool = true;
        // D s_0_14: write-var accdesc.20 <= s_0_13
        fn_state.accdesc._20 = s_0_13;
        // D s_0_15: read-var contiguous:u8
        let s_0_15: bool = fn_state.contiguous;
        // D s_0_16: write-var accdesc.7 <= s_0_15
        fn_state.accdesc._7 = s_0_15;
        // C s_0_17: const #1u : u8
        let s_0_17: bool = true;
        // D s_0_18: write-var accdesc.26 <= s_0_17
        fn_state.accdesc._26 = s_0_17;
        // C s_0_19: const #"No tag checking of SME LDR & STR instructions" : str
        let s_0_19: &'static str = "No tag checking of SME LDR & STR instructions";
        // S s_0_20: call __IMPDEF_boolean(s_0_19)
        let s_0_20: bool = u__IMPDEF_boolean(state, tracer, s_0_19);
        // N s_0_21: branch s_0_20 b6 b1
        if s_0_20 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_1_0: read-var tagchecked:u8
        let s_1_0: bool = fn_state.tagchecked;
        // D s_1_1: write-var accdesc.28 <= s_1_0
        fn_state.accdesc._28 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call HaveTME(s_2_0)
        let s_2_1: bool = HaveTME(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b5 b3
        if s_2_1 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#7388 <= s_3_0
        fn_state.gs_7388 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // D s_4_0: read-var gs#7388:u8
        let s_4_0: bool = fn_state.gs_7388;
        // D s_4_1: write-var accdesc.30 <= s_4_0
        fn_state.accdesc._30 = s_4_0;
        // D s_4_2: read-var accdesc:struct
        let s_4_2: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // N s_4_3: return s_4_2
        return s_4_2;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_5_0: const #100180u : u32
        let s_5_0: u32 = 100180;
        // D s_5_1: read-reg s_5_0:i
        let s_5_1: i128 = {
            let value = state.read_register::<i128>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #0s : i
        let s_5_2: i128 = 0;
        // D s_5_3: cmp-gt s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) > (s_5_2));
        // D s_5_4: write-var gs#7388 <= s_5_3
        fn_state.gs_7388 = s_5_3;
        // N s_5_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType9878976b5bcce9c9 {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var accdesc.28 <= s_6_0
        fn_state.accdesc._28 = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
