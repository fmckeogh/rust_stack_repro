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
use LastInITBlock::*;
use ConditionPassed::*;
use execute_aarch32_instrs_RFE_Op_AS_txt::*;
use InITBlock::*;
use common::*;
pub fn decode_aarch32_instrs_RFE_T1enc_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    W: bool,
    Rn: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        gs_323556: bool,
        wback: bool,
        W: bool,
        Rn: u8,
    }
    let fn_state = FunctionState {
        W,
        Rn,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call ConditionPassed(s_0_0)
        let s_0_1: bool = ConditionPassed(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b2 b1
        if s_0_1 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var Rn:u8
        let s_2_0: u8 = fn_state.Rn;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // D s_2_4: write-var n <= s_2_3
        fn_state.n = s_2_3;
        // D s_2_5: read-var W:u8
        let s_2_5: bool = fn_state.W;
        // D s_2_6: cast zx s_2_5 -> bv
        let s_2_6: Bits = Bits::new(s_2_5 as u128, 1u16);
        // C s_2_7: const #1u : u8
        let s_2_7: bool = true;
        // C s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 1u16);
        // D s_2_9: cmp-eq s_2_6 s_2_8
        let s_2_9: bool = ((s_2_6) == (s_2_8));
        // D s_2_10: write-var wback <= s_2_9
        fn_state.wback = s_2_9;
        // C s_2_11: const #15s : i
        let s_2_11: i128 = 15;
        // D s_2_12: read-var n:i64
        let s_2_12: i64 = fn_state.n;
        // D s_2_13: cast zx s_2_12 -> i
        let s_2_13: i128 = (i128::try_from(s_2_12).unwrap());
        // D s_2_14: cmp-eq s_2_13 s_2_11
        let s_2_14: bool = ((s_2_13) == (s_2_11));
        // N s_2_15: branch s_2_14 b9 b3
        if s_2_14 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call InITBlock(s_3_0)
        let s_3_1: bool = InITBlock(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b8 b4
        if s_3_1 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#323556 <= s_4_0
        fn_state.gs_323556 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#323556:u8
        let s_5_0: bool = fn_state.gs_323556;
        // N s_5_1: branch s_5_0 b7 b6
        if s_5_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: read-var n:i64
        let s_6_1: i64 = fn_state.n;
        // D s_6_2: read-var wback:u8
        let s_6_2: bool = fn_state.wback;
        // C s_6_3: const #0u : u8
        let s_6_3: bool = false;
        // D s_6_4: call execute_aarch32_instrs_RFE_Op_AS_txt(s_6_0, s_6_1, s_6_2, s_6_3)
        let s_6_4: () = execute_aarch32_instrs_RFE_Op_AS_txt(
            state,
            tracer,
            s_6_0,
            s_6_1,
            s_6_2,
            s_6_3,
        );
        // N s_6_5: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call LastInITBlock(s_8_0)
        let s_8_1: bool = LastInITBlock(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // D s_8_3: write-var gs#323556 <= s_8_2
        fn_state.gs_323556 = s_8_2;
        // N s_8_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
}
