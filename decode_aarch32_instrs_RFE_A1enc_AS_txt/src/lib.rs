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
use ConditionPassed::*;
use execute_aarch32_instrs_RFE_Op_AS_txt::*;
use common::*;
pub fn decode_aarch32_instrs_RFE_A1enc_AS_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    P: bool,
    U: bool,
    W: bool,
    Rn: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        wordhigher: bool,
        n: i64,
        increment_name: bool,
        wback: bool,
        P: bool,
        U: bool,
        W: bool,
        Rn: u8,
    }
    let fn_state = FunctionState {
        P,
        U,
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
        // D s_2_11: read-var U:u8
        let s_2_11: bool = fn_state.U;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 1u16);
        // C s_2_13: const #1u : u8
        let s_2_13: bool = true;
        // C s_2_14: cast zx s_2_13 -> bv
        let s_2_14: Bits = Bits::new(s_2_13 as u128, 1u16);
        // D s_2_15: cmp-eq s_2_12 s_2_14
        let s_2_15: bool = ((s_2_12) == (s_2_14));
        // D s_2_16: write-var increment_name <= s_2_15
        fn_state.increment_name = s_2_15;
        // D s_2_17: read-var P:u8
        let s_2_17: bool = fn_state.P;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // D s_2_19: read-var U:u8
        let s_2_19: bool = fn_state.U;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_21: cmp-eq s_2_18 s_2_20
        let s_2_21: bool = ((s_2_18) == (s_2_20));
        // D s_2_22: write-var wordhigher <= s_2_21
        fn_state.wordhigher = s_2_21;
        // C s_2_23: const #15s : i
        let s_2_23: i128 = 15;
        // D s_2_24: read-var n:i64
        let s_2_24: i64 = fn_state.n;
        // D s_2_25: cast zx s_2_24 -> i
        let s_2_25: i128 = (i128::try_from(s_2_24).unwrap());
        // D s_2_26: cmp-eq s_2_25 s_2_23
        let s_2_26: bool = ((s_2_25) == (s_2_23));
        // N s_2_27: branch s_2_26 b4 b3
        if s_2_26 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var increment_name:u8
        let s_3_0: bool = fn_state.increment_name;
        // D s_3_1: read-var n:i64
        let s_3_1: i64 = fn_state.n;
        // D s_3_2: read-var wback:u8
        let s_3_2: bool = fn_state.wback;
        // D s_3_3: read-var wordhigher:u8
        let s_3_3: bool = fn_state.wordhigher;
        // D s_3_4: call execute_aarch32_instrs_RFE_Op_AS_txt(s_3_0, s_3_1, s_3_2, s_3_3)
        let s_3_4: () = execute_aarch32_instrs_RFE_Op_AS_txt(
            state,
            tracer,
            s_3_0,
            s_3_1,
            s_3_2,
            s_3_3,
        );
        // N s_3_5: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
}
