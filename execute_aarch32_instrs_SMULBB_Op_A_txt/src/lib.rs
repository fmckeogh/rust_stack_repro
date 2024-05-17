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
use R_read::*;
use R_set::*;
use integer_subrange::*;
use common::*;
pub fn execute_aarch32_instrs_SMULBB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    m_high: bool,
    n: i64,
    n_high: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand1: u16,
        operand2: u16,
        d: i64,
        m: i64,
        m_high: bool,
        n: i64,
        n_high: bool,
    }
    let fn_state = FunctionState {
        d,
        m,
        m_high,
        n,
        n_high,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n_high:u8
        let s_0_0: bool = fn_state.n_high;
        // N s_0_1: branch s_0_0 b6 b1
        if s_0_0 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // C s_1_3: const #0s : i
        let s_1_3: i128 = 0;
        // D s_1_4: cast zx s_1_2 -> bv
        let s_1_4: Bits = Bits::new(s_1_2 as u128, 32u16);
        // C s_1_5: const #1s : i64
        let s_1_5: i64 = 1;
        // C s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (i128::try_from(s_1_5).unwrap());
        // C s_1_7: const #15s : i
        let s_1_7: i128 = 15;
        // C s_1_8: add s_1_7 s_1_6
        let s_1_8: i128 = (s_1_7 + s_1_6);
        // D s_1_9: bit-extract s_1_4 s_1_3 s_1_8
        let s_1_9: Bits = (Bits::new(
            ((s_1_4) >> (s_1_3)).value(),
            u16::try_from(s_1_8).unwrap(),
        ));
        // D s_1_10: cast reint s_1_9 -> u16
        let s_1_10: u16 = (s_1_9.value() as u16);
        // D s_1_11: write-var operand1 <= s_1_10
        fn_state.operand1 = s_1_10;
        // N s_1_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var m_high:u8
        let s_2_0: bool = fn_state.m_high;
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
    ) -> () {
        // D s_3_0: read-var m:i64
        let s_3_0: i64 = fn_state.m;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call R_read(s_3_1)
        let s_3_2: u32 = R_read(state, tracer, s_3_1);
        // C s_3_3: const #0s : i
        let s_3_3: i128 = 0;
        // D s_3_4: cast zx s_3_2 -> bv
        let s_3_4: Bits = Bits::new(s_3_2 as u128, 32u16);
        // C s_3_5: const #1s : i64
        let s_3_5: i64 = 1;
        // C s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // C s_3_7: const #15s : i
        let s_3_7: i128 = 15;
        // C s_3_8: add s_3_7 s_3_6
        let s_3_8: i128 = (s_3_7 + s_3_6);
        // D s_3_9: bit-extract s_3_4 s_3_3 s_3_8
        let s_3_9: Bits = (Bits::new(
            ((s_3_4) >> (s_3_3)).value(),
            u16::try_from(s_3_8).unwrap(),
        ));
        // D s_3_10: cast reint s_3_9 -> u16
        let s_3_10: u16 = (s_3_9.value() as u16);
        // D s_3_11: write-var operand2 <= s_3_10
        fn_state.operand2 = s_3_10;
        // N s_3_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var operand1:u16
        let s_4_0: u16 = fn_state.operand1;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 16u16);
        // D s_4_2: cast sx s_4_1 -> i
        let s_4_2: i128 = {
            let sign_bit = s_4_1.length() - 1;
            let mut result = s_4_1.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: read-var operand2:u16
        let s_4_4: u16 = fn_state.operand2;
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 16u16);
        // D s_4_6: cast sx s_4_5 -> i
        let s_4_6: i128 = {
            let sign_bit = s_4_5.length() - 1;
            let mut result = s_4_5.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_4_7: cast reint s_4_6 -> i64
        let s_4_7: i64 = (s_4_6 as i64);
        // D s_4_8: cast zx s_4_3 -> i
        let s_4_8: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_9: cast zx s_4_7 -> i
        let s_4_9: i128 = (i128::try_from(s_4_7).unwrap());
        // D s_4_10: mul s_4_8 s_4_9
        let s_4_10: i128 = ((s_4_8) * (s_4_9));
        // D s_4_11: cast reint s_4_10 -> i64
        let s_4_11: i64 = (s_4_10 as i64);
        // C s_4_12: const #31s : i
        let s_4_12: i128 = 31;
        // C s_4_13: const #0s : i
        let s_4_13: i128 = 0;
        // D s_4_14: cast zx s_4_11 -> i
        let s_4_14: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_15: call integer_subrange(s_4_14, s_4_12, s_4_13)
        let s_4_15: Bits = integer_subrange(state, tracer, s_4_14, s_4_12, s_4_13);
        // D s_4_16: cast reint s_4_15 -> u32
        let s_4_16: u32 = (s_4_15.value() as u32);
        // D s_4_17: read-var d:i64
        let s_4_17: i64 = fn_state.d;
        // D s_4_18: cast zx s_4_17 -> i
        let s_4_18: i128 = (i128::try_from(s_4_17).unwrap());
        // D s_4_19: call R_set(s_4_18, s_4_16)
        let s_4_19: () = R_set(state, tracer, s_4_18, s_4_16);
        // N s_4_20: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var m:i64
        let s_5_0: i64 = fn_state.m;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call R_read(s_5_1)
        let s_5_2: u32 = R_read(state, tracer, s_5_1);
        // C s_5_3: const #16s : i
        let s_5_3: i128 = 16;
        // D s_5_4: cast zx s_5_2 -> bv
        let s_5_4: Bits = Bits::new(s_5_2 as u128, 32u16);
        // C s_5_5: const #1s : i64
        let s_5_5: i64 = 1;
        // C s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // C s_5_7: const #15s : i
        let s_5_7: i128 = 15;
        // C s_5_8: add s_5_7 s_5_6
        let s_5_8: i128 = (s_5_7 + s_5_6);
        // D s_5_9: bit-extract s_5_4 s_5_3 s_5_8
        let s_5_9: Bits = (Bits::new(
            ((s_5_4) >> (s_5_3)).value(),
            u16::try_from(s_5_8).unwrap(),
        ));
        // D s_5_10: cast reint s_5_9 -> u16
        let s_5_10: u16 = (s_5_9.value() as u16);
        // D s_5_11: write-var operand2 <= s_5_10
        fn_state.operand2 = s_5_10;
        // N s_5_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var n:i64
        let s_6_0: i64 = fn_state.n;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call R_read(s_6_1)
        let s_6_2: u32 = R_read(state, tracer, s_6_1);
        // C s_6_3: const #16s : i
        let s_6_3: i128 = 16;
        // D s_6_4: cast zx s_6_2 -> bv
        let s_6_4: Bits = Bits::new(s_6_2 as u128, 32u16);
        // C s_6_5: const #1s : i64
        let s_6_5: i64 = 1;
        // C s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // C s_6_7: const #15s : i
        let s_6_7: i128 = 15;
        // C s_6_8: add s_6_7 s_6_6
        let s_6_8: i128 = (s_6_7 + s_6_6);
        // D s_6_9: bit-extract s_6_4 s_6_3 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_4) >> (s_6_3)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u16
        let s_6_10: u16 = (s_6_9.value() as u16);
        // D s_6_11: write-var operand1 <= s_6_10
        fn_state.operand1 = s_6_10;
        // N s_6_12: jump b2
        return block_2(state, tracer, fn_state);
    }
}
