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
use AArch64_AllocationTagCheck::*;
use AArch64_CanonicalTagCheck::*;
use common::*;
pub fn AArch64_CheckTag<T: Tracer>(
    state: &mut State,
    tracer: &T,
    memaddrdesc: ProductTypece7c66ccb2cab13e,
    accdesc: ProductType9878976b5bcce9c9,
    ptag: u8,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        ga_11823: ProductTypef170cab34335b70c,
        ga_11819: ProductTypef170cab34335b70c,
        ga_11822: bool,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        accdesc: ProductType9878976b5bcce9c9,
        ptag: u8,
    }
    let fn_state = FunctionState {
        memaddrdesc,
        accdesc,
        ptag,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var memaddrdesc.2:struct
        let s_0_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_0_1: write-var ga#11819 <= s_0_0
        fn_state.ga_11819 = s_0_0;
        // D s_0_2: read-var ga#11819.6:struct
        let s_0_2: u32 = fn_state.ga_11819._6;
        // C s_0_3: const #1u : u32
        let s_0_3: u32 = 1;
        // D s_0_4: cmp-eq s_0_2 s_0_3
        let s_0_4: bool = ((s_0_2) == (s_0_3));
        // N s_0_5: branch s_0_4 b5 b1
        if s_0_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var memaddrdesc.2:struct
        let s_1_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_1_1: write-var ga#11823 <= s_1_0
        fn_state.ga_11823 = s_1_0;
        // D s_1_2: read-var ga#11823.6:struct
        let s_1_2: u32 = fn_state.ga_11823._6;
        // C s_1_3: const #2u : u32
        let s_1_3: u32 = 2;
        // D s_1_4: cmp-eq s_1_2 s_1_3
        let s_1_4: bool = ((s_1_2) == (s_1_3));
        // N s_1_5: branch s_1_4 b4 b2
        if s_1_4 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #1u : u8
        let s_2_0: bool = true;
        // D s_2_1: write-var return_value <= s_2_0
        fn_state.return_value = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var return_value:u8
        let s_3_0: bool = fn_state.return_value;
        // N s_3_1: return s_3_0
        return s_3_0;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var memaddrdesc:struct
        let s_4_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_4_1: read-var ptag:u8
        let s_4_1: u8 = fn_state.ptag;
        // D s_4_2: call AArch64_CanonicalTagCheck(s_4_0, s_4_1)
        let s_4_2: bool = AArch64_CanonicalTagCheck(state, tracer, s_4_0, s_4_1);
        // D s_4_3: write-var return_value <= s_4_2
        fn_state.return_value = s_4_2;
        // N s_4_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var memaddrdesc:struct
        let s_5_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_5_1: read-var accdesc:struct
        let s_5_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_5_2: read-var ptag:u8
        let s_5_2: u8 = fn_state.ptag;
        // D s_5_3: call AArch64_AllocationTagCheck(s_5_0, s_5_1, s_5_2)
        let s_5_3: bool = AArch64_AllocationTagCheck(state, tracer, s_5_0, s_5_1, s_5_2);
        // D s_5_4: write-var ga#11822 <= s_5_3
        fn_state.ga_11822 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var ga#11822:u8
        let s_6_0: bool = fn_state.ga_11822;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
