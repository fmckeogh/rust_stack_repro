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
use neq_int::*;
use R_read::*;
use R_set::*;
use integer_subrange::*;
use common::*;
pub fn execute_aarch32_instrs_SMLABB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    a: i64,
    d: i64,
    m: i64,
    m_high: bool,
    n: i64,
    n_high: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand2: u16,
        operand1: u16,
        a: i64,
        d: i64,
        m: i64,
        m_high: bool,
        n: i64,
        n_high: bool,
    }
    let fn_state = FunctionState {
        a,
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
        // N s_0_1: branch s_0_0 b8 b1
        if s_0_0 {
            return block_8(state, tracer, fn_state);
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
        // N s_2_1: branch s_2_0 b7 b3
        if s_2_0 {
            return block_7(state, tracer, fn_state);
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
        // D s_4_12: read-var a:i64
        let s_4_12: i64 = fn_state.a;
        // D s_4_13: cast zx s_4_12 -> i
        let s_4_13: i128 = (i128::try_from(s_4_12).unwrap());
        // D s_4_14: call R_read(s_4_13)
        let s_4_14: u32 = R_read(state, tracer, s_4_13);
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 32u16);
        // D s_4_16: cast sx s_4_15 -> i
        let s_4_16: i128 = {
            let sign_bit = s_4_15.length() - 1;
            let mut result = s_4_15.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_4_17: cast reint s_4_16 -> i64
        let s_4_17: i64 = (s_4_16 as i64);
        // D s_4_18: cast zx s_4_11 -> i
        let s_4_18: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_19: cast zx s_4_17 -> i
        let s_4_19: i128 = (i128::try_from(s_4_17).unwrap());
        // D s_4_20: add s_4_18 s_4_19
        let s_4_20: i128 = (s_4_18 + s_4_19);
        // D s_4_21: cast reint s_4_20 -> i64
        let s_4_21: i64 = (s_4_20 as i64);
        // C s_4_22: const #31s : i
        let s_4_22: i128 = 31;
        // C s_4_23: const #0s : i
        let s_4_23: i128 = 0;
        // D s_4_24: cast zx s_4_21 -> i
        let s_4_24: i128 = (i128::try_from(s_4_21).unwrap());
        // D s_4_25: call integer_subrange(s_4_24, s_4_22, s_4_23)
        let s_4_25: Bits = integer_subrange(state, tracer, s_4_24, s_4_22, s_4_23);
        // D s_4_26: cast reint s_4_25 -> u32
        let s_4_26: u32 = (s_4_25.value() as u32);
        // D s_4_27: read-var d:i64
        let s_4_27: i64 = fn_state.d;
        // D s_4_28: cast zx s_4_27 -> i
        let s_4_28: i128 = (i128::try_from(s_4_27).unwrap());
        // D s_4_29: call R_set(s_4_28, s_4_26)
        let s_4_29: () = R_set(state, tracer, s_4_28, s_4_26);
        // C s_4_30: const #31s : i
        let s_4_30: i128 = 31;
        // C s_4_31: const #0s : i
        let s_4_31: i128 = 0;
        // D s_4_32: cast zx s_4_21 -> i
        let s_4_32: i128 = (i128::try_from(s_4_21).unwrap());
        // D s_4_33: call integer_subrange(s_4_32, s_4_30, s_4_31)
        let s_4_33: Bits = integer_subrange(state, tracer, s_4_32, s_4_30, s_4_31);
        // D s_4_34: cast reint s_4_33 -> u32
        let s_4_34: u32 = (s_4_33.value() as u32);
        // D s_4_35: cast zx s_4_34 -> bv
        let s_4_35: Bits = Bits::new(s_4_34 as u128, 32u16);
        // D s_4_36: cast sx s_4_35 -> i
        let s_4_36: i128 = {
            let sign_bit = s_4_35.length() - 1;
            let mut result = s_4_35.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_4_37: cast reint s_4_36 -> i64
        let s_4_37: i64 = (s_4_36 as i64);
        // D s_4_38: cast zx s_4_21 -> i
        let s_4_38: i128 = (i128::try_from(s_4_21).unwrap());
        // D s_4_39: cast zx s_4_37 -> i
        let s_4_39: i128 = (i128::try_from(s_4_37).unwrap());
        // D s_4_40: call neq_int(s_4_38, s_4_39)
        let s_4_40: bool = neq_int(state, tracer, s_4_38, s_4_39);
        // N s_4_41: branch s_4_40 b6 b5
        if s_4_40 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // C s_6_1: const #16988u : u32
        let s_6_1: u32 = 16988;
        // N s_6_2: write-reg s_6_1 <= s_6_0
        let s_6_2: () = {
            state.write_register::<bool>(s_6_1 as isize, s_6_0);
            tracer.write_register(s_6_1 as isize, s_6_0);
        };
        // N s_6_3: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var m:i64
        let s_7_0: i64 = fn_state.m;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call R_read(s_7_1)
        let s_7_2: u32 = R_read(state, tracer, s_7_1);
        // C s_7_3: const #16s : i
        let s_7_3: i128 = 16;
        // D s_7_4: cast zx s_7_2 -> bv
        let s_7_4: Bits = Bits::new(s_7_2 as u128, 32u16);
        // C s_7_5: const #1s : i64
        let s_7_5: i64 = 1;
        // C s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // C s_7_7: const #15s : i
        let s_7_7: i128 = 15;
        // C s_7_8: add s_7_7 s_7_6
        let s_7_8: i128 = (s_7_7 + s_7_6);
        // D s_7_9: bit-extract s_7_4 s_7_3 s_7_8
        let s_7_9: Bits = (Bits::new(
            ((s_7_4) >> (s_7_3)).value(),
            u16::try_from(s_7_8).unwrap(),
        ));
        // D s_7_10: cast reint s_7_9 -> u16
        let s_7_10: u16 = (s_7_9.value() as u16);
        // D s_7_11: write-var operand2 <= s_7_10
        fn_state.operand2 = s_7_10;
        // N s_7_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var n:i64
        let s_8_0: i64 = fn_state.n;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call R_read(s_8_1)
        let s_8_2: u32 = R_read(state, tracer, s_8_1);
        // C s_8_3: const #16s : i
        let s_8_3: i128 = 16;
        // D s_8_4: cast zx s_8_2 -> bv
        let s_8_4: Bits = Bits::new(s_8_2 as u128, 32u16);
        // C s_8_5: const #1s : i64
        let s_8_5: i64 = 1;
        // C s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // C s_8_7: const #15s : i
        let s_8_7: i128 = 15;
        // C s_8_8: add s_8_7 s_8_6
        let s_8_8: i128 = (s_8_7 + s_8_6);
        // D s_8_9: bit-extract s_8_4 s_8_3 s_8_8
        let s_8_9: Bits = (Bits::new(
            ((s_8_4) >> (s_8_3)).value(),
            u16::try_from(s_8_8).unwrap(),
        ));
        // D s_8_10: cast reint s_8_9 -> u16
        let s_8_10: u16 = (s_8_9.value() as u16);
        // D s_8_11: write-var operand1 <= s_8_10
        fn_state.operand1 = s_8_10;
        // N s_8_12: jump b2
        return block_2(state, tracer, fn_state);
    }
}
