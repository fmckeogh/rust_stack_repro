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
use X_set::*;
use CheckSMEEnabled::*;
use X_read::*;
use SP_set::*;
use SP_read::*;
use common::*;
pub fn execute_ADDSVL_R_RI__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    SVL: i64,
    d: i64,
    imm: i128,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: u64,
        len: i128,
        operand1: u64,
        SVL: i64,
        d: i64,
        imm: i128,
        n: i64,
    }
    let fn_state = FunctionState {
        SVL,
        d,
        imm,
        n,
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
        // S s_0_1: call CheckSMEEnabled(s_0_0)
        let s_0_1: () = CheckSMEEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var SVL:i64
        let s_1_0: i64 = fn_state.SVL;
        // C s_1_1: const #8s : i
        let s_1_1: i128 = 8;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: div s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) / (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: read-var imm:i
        let s_1_6: i128 = fn_state.imm;
        // D s_1_7: mul s_1_6 s_1_5
        let s_1_7: i128 = ((s_1_6) * (s_1_5));
        // D s_1_8: write-var len <= s_1_7
        fn_state.len = s_1_7;
        // C s_1_9: const #31s : i
        let s_1_9: i128 = 31;
        // D s_1_10: read-var n:i64
        let s_1_10: i64 = fn_state.n;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cmp-eq s_1_11 s_1_9
        let s_1_12: bool = ((s_1_11) == (s_1_9));
        // N s_1_13: branch s_1_12 b6 b2
        if s_1_12 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call X_read(s_2_2, s_2_0)
        let s_2_3: Bits = X_read(state, tracer, s_2_2, s_2_0);
        // D s_2_4: cast reint s_2_3 -> u64
        let s_2_4: u64 = (s_2_3.value() as u64);
        // D s_2_5: write-var operand1 <= s_2_4
        fn_state.operand1 = s_2_4;
        // N s_2_6: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var operand1:u64
        let s_3_0: u64 = fn_state.operand1;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 64u16);
        // D s_3_2: read-var len:i
        let s_3_2: i128 = fn_state.len;
        // D s_3_3: cast cvt s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 128);
        // D s_3_4: add s_3_1 s_3_3
        let s_3_4: Bits = (s_3_1 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> u64
        let s_3_5: u64 = (s_3_4.value() as u64);
        // D s_3_6: write-var result <= s_3_5
        fn_state.result = s_3_5;
        // C s_3_7: const #31s : i
        let s_3_7: i128 = 31;
        // D s_3_8: read-var d:i64
        let s_3_8: i64 = fn_state.d;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cmp-eq s_3_9 s_3_7
        let s_3_10: bool = ((s_3_9) == (s_3_7));
        // N s_3_11: branch s_3_10 b5 b4
        if s_3_10 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64s : i64
        let s_4_0: i64 = 64;
        // D s_4_1: read-var d:i64
        let s_4_1: i64 = fn_state.d;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: read-var result:u64
        let s_4_3: u64 = fn_state.result;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 64u16);
        // D s_4_5: call X_set(s_4_2, s_4_0, s_4_4)
        let s_4_5: () = X_set(state, tracer, s_4_2, s_4_0, s_4_4);
        // N s_4_6: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var result:u64
        let s_5_0: u64 = fn_state.result;
        // D s_5_1: call SP_set(s_5_0)
        let s_5_1: () = SP_set(state, tracer, s_5_0);
        // N s_5_2: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call SP_read(s_6_0)
        let s_6_1: u64 = SP_read(state, tracer, s_6_0);
        // D s_6_2: write-var operand1 <= s_6_1
        fn_state.operand1 = s_6_1;
        // N s_6_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
