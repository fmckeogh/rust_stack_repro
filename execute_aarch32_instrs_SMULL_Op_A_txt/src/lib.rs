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
use integer_access::*;
use integer_subrange::*;
use IsZeroBit::*;
use common::*;
pub fn execute_aarch32_instrs_SMULL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    dHi: i64,
    dLo: i64,
    m: i64,
    n: i64,
    setflags: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: i64,
        dHi: i64,
        dLo: i64,
        m: i64,
        n: i64,
        setflags: bool,
    }
    let fn_state = FunctionState {
        dHi,
        dLo,
        m,
        n,
        setflags,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 32u16);
        // D s_0_4: cast sx s_0_3 -> i
        let s_0_4: i128 = {
            let sign_bit = s_0_3.length() - 1;
            let mut result = s_0_3.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_5: cast reint s_0_4 -> i64
        let s_0_5: i64 = (s_0_4 as i64);
        // D s_0_6: read-var m:i64
        let s_0_6: i64 = fn_state.m;
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_8: call R_read(s_0_7)
        let s_0_8: u32 = R_read(state, tracer, s_0_7);
        // D s_0_9: cast zx s_0_8 -> bv
        let s_0_9: Bits = Bits::new(s_0_8 as u128, 32u16);
        // D s_0_10: cast sx s_0_9 -> i
        let s_0_10: i128 = {
            let sign_bit = s_0_9.length() - 1;
            let mut result = s_0_9.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // D s_0_12: cast zx s_0_5 -> i
        let s_0_12: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_13: cast zx s_0_11 -> i
        let s_0_13: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_14: mul s_0_12 s_0_13
        let s_0_14: i128 = ((s_0_12) * (s_0_13));
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // D s_0_16: write-var result <= s_0_15
        fn_state.result = s_0_15;
        // C s_0_17: const #63s : i
        let s_0_17: i128 = 63;
        // C s_0_18: const #32s : i
        let s_0_18: i128 = 32;
        // D s_0_19: read-var result:i64
        let s_0_19: i64 = fn_state.result;
        // D s_0_20: cast zx s_0_19 -> i
        let s_0_20: i128 = (i128::try_from(s_0_19).unwrap());
        // D s_0_21: call integer_subrange(s_0_20, s_0_17, s_0_18)
        let s_0_21: Bits = integer_subrange(state, tracer, s_0_20, s_0_17, s_0_18);
        // D s_0_22: cast reint s_0_21 -> u32
        let s_0_22: u32 = (s_0_21.value() as u32);
        // D s_0_23: read-var dHi:i64
        let s_0_23: i64 = fn_state.dHi;
        // D s_0_24: cast zx s_0_23 -> i
        let s_0_24: i128 = (i128::try_from(s_0_23).unwrap());
        // D s_0_25: call R_set(s_0_24, s_0_22)
        let s_0_25: () = R_set(state, tracer, s_0_24, s_0_22);
        // C s_0_26: const #31s : i
        let s_0_26: i128 = 31;
        // C s_0_27: const #0s : i
        let s_0_27: i128 = 0;
        // D s_0_28: read-var result:i64
        let s_0_28: i64 = fn_state.result;
        // D s_0_29: cast zx s_0_28 -> i
        let s_0_29: i128 = (i128::try_from(s_0_28).unwrap());
        // D s_0_30: call integer_subrange(s_0_29, s_0_26, s_0_27)
        let s_0_30: Bits = integer_subrange(state, tracer, s_0_29, s_0_26, s_0_27);
        // D s_0_31: cast reint s_0_30 -> u32
        let s_0_31: u32 = (s_0_30.value() as u32);
        // D s_0_32: read-var dLo:i64
        let s_0_32: i64 = fn_state.dLo;
        // D s_0_33: cast zx s_0_32 -> i
        let s_0_33: i128 = (i128::try_from(s_0_32).unwrap());
        // D s_0_34: call R_set(s_0_33, s_0_31)
        let s_0_34: () = R_set(state, tracer, s_0_33, s_0_31);
        // D s_0_35: read-var setflags:u8
        let s_0_35: bool = fn_state.setflags;
        // N s_0_36: branch s_0_35 b2 b1
        if s_0_35 {
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
        // C s_2_0: const #63s : i
        let s_2_0: i128 = 63;
        // D s_2_1: read-var result:i64
        let s_2_1: i64 = fn_state.result;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: call integer_access(s_2_2, s_2_0)
        let s_2_3: bool = integer_access(state, tracer, s_2_2, s_2_0);
        // C s_2_4: const #0s : i
        let s_2_4: i128 = 0;
        // C s_2_5: const #0u : u64
        let s_2_5: u64 = 0;
        // D s_2_6: cast zx s_2_3 -> u64
        let s_2_6: u64 = (s_2_3 as u64);
        // C s_2_7: const #1u : u64
        let s_2_7: u64 = 1;
        // D s_2_8: and s_2_6 s_2_7
        let s_2_8: u64 = ((s_2_6) & (s_2_7));
        // D s_2_9: cmp-eq s_2_8 s_2_7
        let s_2_9: bool = ((s_2_8) == (s_2_7));
        // D s_2_10: lsl s_2_6 s_2_4
        let s_2_10: u64 = s_2_6 << s_2_4;
        // D s_2_11: or s_2_5 s_2_10
        let s_2_11: u64 = ((s_2_5) | (s_2_10));
        // D s_2_12: cmpl s_2_10
        let s_2_12: u64 = !s_2_10;
        // D s_2_13: and s_2_5 s_2_12
        let s_2_13: u64 = ((s_2_5) & (s_2_12));
        // D s_2_14: select s_2_9 s_2_11 s_2_13
        let s_2_14: u64 = if s_2_9 { s_2_11 } else { s_2_13 };
        // D s_2_15: cast trunc s_2_14 -> u8
        let s_2_15: bool = ((s_2_14) != 0);
        // C s_2_16: const #16984u : u32
        let s_2_16: u32 = 16984;
        // N s_2_17: write-reg s_2_16 <= s_2_15
        let s_2_17: () = {
            state.write_register::<bool>(s_2_16 as isize, s_2_15);
            tracer.write_register(s_2_16 as isize, s_2_15);
        };
        // C s_2_18: const #63s : i
        let s_2_18: i128 = 63;
        // C s_2_19: const #0s : i
        let s_2_19: i128 = 0;
        // D s_2_20: read-var result:i64
        let s_2_20: i64 = fn_state.result;
        // D s_2_21: cast zx s_2_20 -> i
        let s_2_21: i128 = (i128::try_from(s_2_20).unwrap());
        // D s_2_22: call integer_subrange(s_2_21, s_2_18, s_2_19)
        let s_2_22: Bits = integer_subrange(state, tracer, s_2_21, s_2_18, s_2_19);
        // D s_2_23: cast reint s_2_22 -> u64
        let s_2_23: u64 = (s_2_22.value() as u64);
        // D s_2_24: cast zx s_2_23 -> bv
        let s_2_24: Bits = Bits::new(s_2_23 as u128, 64u16);
        // D s_2_25: call IsZeroBit(s_2_24)
        let s_2_25: bool = IsZeroBit(state, tracer, s_2_24);
        // C s_2_26: const #16997u : u32
        let s_2_26: u32 = 16997;
        // N s_2_27: write-reg s_2_26 <= s_2_25
        let s_2_27: () = {
            state.write_register::<bool>(s_2_26 as isize, s_2_25);
            tracer.write_register(s_2_26 as isize, s_2_25);
        };
        // N s_2_28: return
        return;
    }
}
