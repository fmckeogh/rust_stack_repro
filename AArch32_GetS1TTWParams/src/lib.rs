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
use AArch32_S1TTWParamsEL2::*;
use AArch32_S1TTWParamsEL30::*;
use AArch32_S1TTWParamsEL10::*;
use common::*;
pub fn AArch32_GetS1TTWParams<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    va: u32,
) -> ProductTypeef284266e139aee2 {
    #[derive(Default)]
    struct FunctionState {
        walkparams: ProductTypeef284266e139aee2,
        regime: u32,
        va: u32,
    }
    let fn_state = FunctionState {
        regime,
        va,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // C s_0_0: const #2u : u32
        let s_0_0: u32 = 2;
        // D s_0_1: read-var regime:u32
        let s_0_1: u32 = fn_state.regime;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b3 b1
        if s_0_3 {
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
        // S s_1_1: call AArch32_S1TTWParamsEL2(s_1_0)
        let s_1_1: ProductTypeef284266e139aee2 = AArch32_S1TTWParamsEL2(
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
        // C s_3_0: const #4u : u32
        let s_3_0: u32 = 4;
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
        // D s_4_0: read-var va:u32
        let s_4_0: u32 = fn_state.va;
        // D s_4_1: call AArch32_S1TTWParamsEL10(s_4_0)
        let s_4_1: ProductTypeef284266e139aee2 = AArch32_S1TTWParamsEL10(
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
        // C s_5_0: const #1u : u32
        let s_5_0: u32 = 1;
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
        // D s_6_0: read-var va:u32
        let s_6_0: u32 = fn_state.va;
        // D s_6_1: call AArch32_S1TTWParamsEL30(s_6_0)
        let s_6_1: ProductTypeef284266e139aee2 = AArch32_S1TTWParamsEL30(
            state,
            tracer,
            s_6_0,
        );
        // D s_6_2: write-var walkparams <= s_6_1
        fn_state.walkparams = s_6_1;
        // N s_6_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeef284266e139aee2 {
        // N s_7_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
