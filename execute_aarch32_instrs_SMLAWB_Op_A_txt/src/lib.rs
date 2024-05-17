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
pub fn execute_aarch32_instrs_SMLAWB_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    a: i64,
    d: i64,
    m: i64,
    m_high: bool,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand2: u16,
        a: i64,
        d: i64,
        m: i64,
        m_high: bool,
        n: i64,
    }
    let fn_state = FunctionState {
        a,
        d,
        m,
        m_high,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var m_high:u8
        let s_0_0: bool = fn_state.m_high;
        // N s_0_1: branch s_0_0 b5 b1
        if s_0_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var m:i64
        let s_1_0: i64 = fn_state.m;
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
        // D s_1_11: write-var operand2 <= s_1_10
        fn_state.operand2 = s_1_10;
        // N s_1_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var n:i64
        let s_2_0: i64 = fn_state.n;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call R_read(s_2_1)
        let s_2_2: u32 = R_read(state, tracer, s_2_1);
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 32u16);
        // D s_2_4: cast sx s_2_3 -> i
        let s_2_4: i128 = {
            let sign_bit = s_2_3.length() - 1;
            let mut result = s_2_3.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_5: cast reint s_2_4 -> i64
        let s_2_5: i64 = (s_2_4 as i64);
        // D s_2_6: read-var operand2:u16
        let s_2_6: u16 = fn_state.operand2;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 16u16);
        // D s_2_8: cast sx s_2_7 -> i
        let s_2_8: i128 = {
            let sign_bit = s_2_7.length() - 1;
            let mut result = s_2_7.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: cast zx s_2_5 -> i
        let s_2_10: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_11: cast zx s_2_9 -> i
        let s_2_11: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_12: mul s_2_10 s_2_11
        let s_2_12: i128 = ((s_2_10) * (s_2_11));
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: read-var a:i64
        let s_2_14: i64 = fn_state.a;
        // D s_2_15: cast zx s_2_14 -> i
        let s_2_15: i128 = (i128::try_from(s_2_14).unwrap());
        // D s_2_16: call R_read(s_2_15)
        let s_2_16: u32 = R_read(state, tracer, s_2_15);
        // D s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 32u16);
        // D s_2_18: cast sx s_2_17 -> i
        let s_2_18: i128 = {
            let sign_bit = s_2_17.length() - 1;
            let mut result = s_2_17.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_19: cast reint s_2_18 -> i64
        let s_2_19: i64 = (s_2_18 as i64);
        // C s_2_20: const #16s : i
        let s_2_20: i128 = 16;
        // D s_2_21: cast zx s_2_19 -> i
        let s_2_21: i128 = (i128::try_from(s_2_19).unwrap());
        // D s_2_22: lsl s_2_21 s_2_20
        let s_2_22: i128 = s_2_21 << s_2_20;
        // D s_2_23: cast zx s_2_13 -> i
        let s_2_23: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_24: add s_2_23 s_2_22
        let s_2_24: i128 = (s_2_23 + s_2_22);
        // C s_2_25: const #47s : i
        let s_2_25: i128 = 47;
        // C s_2_26: const #16s : i
        let s_2_26: i128 = 16;
        // D s_2_27: call integer_subrange(s_2_24, s_2_25, s_2_26)
        let s_2_27: Bits = integer_subrange(state, tracer, s_2_24, s_2_25, s_2_26);
        // D s_2_28: cast reint s_2_27 -> u32
        let s_2_28: u32 = (s_2_27.value() as u32);
        // D s_2_29: read-var d:i64
        let s_2_29: i64 = fn_state.d;
        // D s_2_30: cast zx s_2_29 -> i
        let s_2_30: i128 = (i128::try_from(s_2_29).unwrap());
        // D s_2_31: call R_set(s_2_30, s_2_28)
        let s_2_31: () = R_set(state, tracer, s_2_30, s_2_28);
        // C s_2_32: const #16s : i
        let s_2_32: i128 = 16;
        // D s_2_33: lsr s_2_24 s_2_32
        let s_2_33: i128 = s_2_24 >> s_2_32;
        // D s_2_34: read-var d:i64
        let s_2_34: i64 = fn_state.d;
        // D s_2_35: cast zx s_2_34 -> i
        let s_2_35: i128 = (i128::try_from(s_2_34).unwrap());
        // D s_2_36: call R_read(s_2_35)
        let s_2_36: u32 = R_read(state, tracer, s_2_35);
        // D s_2_37: cast zx s_2_36 -> bv
        let s_2_37: Bits = Bits::new(s_2_36 as u128, 32u16);
        // D s_2_38: cast sx s_2_37 -> i
        let s_2_38: i128 = {
            let sign_bit = s_2_37.length() - 1;
            let mut result = s_2_37.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_2_39: cast reint s_2_38 -> i64
        let s_2_39: i64 = (s_2_38 as i64);
        // D s_2_40: cast zx s_2_39 -> i
        let s_2_40: i128 = (i128::try_from(s_2_39).unwrap());
        // D s_2_41: call neq_int(s_2_33, s_2_40)
        let s_2_41: bool = neq_int(state, tracer, s_2_33, s_2_40);
        // N s_2_42: branch s_2_41 b4 b3
        if s_2_41 {
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
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // C s_4_1: const #16988u : u32
        let s_4_1: u32 = 16988;
        // N s_4_2: write-reg s_4_1 <= s_4_0
        let s_4_2: () = {
            state.write_register::<bool>(s_4_1 as isize, s_4_0);
            tracer.write_register(s_4_1 as isize, s_4_0);
        };
        // N s_4_3: return
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
        // N s_5_12: jump b2
        return block_2(state, tracer, fn_state);
    }
}
