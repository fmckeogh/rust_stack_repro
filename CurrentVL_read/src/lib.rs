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
use HaveSME::*;
use CurrentNSVL_read::*;
use CurrentSVL_read::*;
use common::*;
pub fn CurrentVL_read<T: Tracer>(state: &mut State, tracer: &T, gs_3896: ()) -> i64 {
    #[derive(Default)]
    struct FunctionState {
        ga_2402: i64,
        gs_3898: bool,
        SVL: i64,
        gs_3896: (),
    }
    let fn_state = FunctionState {
        gs_3896,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CurrentSVL_read(s_0_0)
        let s_0_1: i64 = CurrentSVL_read(state, tracer, s_0_0);
        // D s_0_2: write-var SVL <= s_0_1
        fn_state.SVL = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call HaveSME(s_0_3)
        let s_0_4: bool = HaveSME(state, tracer, s_0_3);
        // N s_0_5: branch s_0_4 b6 b1
        if s_0_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#3898 <= s_1_0
        fn_state.gs_3898 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_2_0: read-var gs#3898:u8
        let s_2_0: bool = fn_state.gs_3898;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call CurrentNSVL_read(s_3_0)
        let s_3_1: i64 = CurrentNSVL_read(state, tracer, s_3_0);
        // D s_3_2: write-var ga#2402 <= s_3_1
        fn_state.ga_2402 = s_3_1;
        // N s_3_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_4_0: read-var ga#2402:i64
        let s_4_0: i64 = fn_state.ga_2402;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // D s_5_0: read-var SVL:i64
        let s_5_0: i64 = fn_state.SVL;
        // D s_5_1: write-var ga#2402 <= s_5_0
        fn_state.ga_2402 = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i64 {
        // C s_6_0: const #16989u : u32
        let s_6_0: u32 = 16989;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: bool = {
            let value = state.read_register::<bool>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 1u16);
        // C s_6_3: const #1u : u8
        let s_6_3: bool = true;
        // C s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 1u16);
        // D s_6_5: cmp-eq s_6_2 s_6_4
        let s_6_5: bool = ((s_6_2) == (s_6_4));
        // D s_6_6: write-var gs#3898 <= s_6_5
        fn_state.gs_3898 = s_6_5;
        // N s_6_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
