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
use AArch64_S1TTWParamsEL10::*;
use AArch64_S1TTWParamsEL3::*;
use AArch64_GetVARange::*;
use AArch64_S1TTWParamsEL2::*;
use AArch64_S1TTWParamsEL20::*;
use common::*;
pub fn AArch64_GetS1TTWParams<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    ss: u32,
    va: u64,
) -> ProductTypeef284266e139aee2 {
    #[derive(Default)]
    struct FunctionState {
        varange: u32,
        walkparams: ProductTypeef284266e139aee2,
        regime: u32,
        ss: u32,
        va: u64,
    }
    let fn_state = FunctionState {
        regime,
        ss,
        va,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_0_0: read-var va:u64
        let s_0_0: u64 = fn_state.va;
        // D s_0_1: call AArch64_GetVARange(s_0_0)
        let s_0_1: u32 = AArch64_GetVARange(state, tracer, s_0_0);
        // D s_0_2: write-var varange <= s_0_1
        fn_state.varange = s_0_1;
        // C s_0_3: const #0u : u32
        let s_0_3: u32 = 0;
        // D s_0_4: read-var regime:u32
        let s_0_4: u32 = fn_state.regime;
        // D s_0_5: cmp-eq s_0_3 s_0_4
        let s_0_5: bool = ((s_0_3) == (s_0_4));
        // D s_0_6: not s_0_5
        let s_0_6: bool = !s_0_5;
        // N s_0_7: branch s_0_6 b3 b1
        if s_0_6 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call AArch64_S1TTWParamsEL3(s_1_0)
        let s_1_1: ProductTypeef284266e139aee2 = AArch64_S1TTWParamsEL3(
            state,
            tracer,
            s_1_0,
        );
        // D s_1_2: write-var walkparams <= s_1_1
        fn_state.walkparams = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_2_0: read-var walkparams:struct
        let s_2_0: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_3_0: const #2u : u32
        let s_3_0: u32 = 2;
        // D s_3_1: read-var regime:u32
        let s_3_1: u32 = fn_state.regime;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_4_0: read-var ss:u32
        let s_4_0: u32 = fn_state.ss;
        // D s_4_1: call AArch64_S1TTWParamsEL2(s_4_0)
        let s_4_1: ProductTypeef284266e139aee2 = AArch64_S1TTWParamsEL2(
            state,
            tracer,
            s_4_0,
        );
        // D s_4_2: write-var walkparams <= s_4_1
        fn_state.walkparams = s_4_1;
        // N s_4_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_5_0: const #3u : u32
        let s_5_0: u32 = 3;
        // D s_5_1: read-var regime:u32
        let s_5_1: u32 = fn_state.regime;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_6_0: read-var ss:u32
        let s_6_0: u32 = fn_state.ss;
        // D s_6_1: read-var varange:u32
        let s_6_1: u32 = fn_state.varange;
        // D s_6_2: call AArch64_S1TTWParamsEL20(s_6_0, s_6_1)
        let s_6_2: ProductTypeef284266e139aee2 = AArch64_S1TTWParamsEL20(
            state,
            tracer,
            s_6_0,
            s_6_1,
        );
        // D s_6_3: write-var walkparams <= s_6_2
        fn_state.walkparams = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_7_0: const #4u : u32
        let s_7_0: u32 = 4;
        // D s_7_1: read-var regime:u32
        let s_7_1: u32 = fn_state.regime;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b9 b8
        if s_7_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // D s_8_0: read-var varange:u32
        let s_8_0: u32 = fn_state.varange;
        // D s_8_1: call AArch64_S1TTWParamsEL10(s_8_0)
        let s_8_1: ProductTypeef284266e139aee2 = AArch64_S1TTWParamsEL10(
            state,
            tracer,
            s_8_0,
        );
        // D s_8_2: write-var walkparams <= s_8_1
        fn_state.walkparams = s_8_1;
        // N s_8_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_9_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
