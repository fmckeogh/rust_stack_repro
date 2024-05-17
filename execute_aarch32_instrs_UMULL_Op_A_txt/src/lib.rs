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
pub fn execute_aarch32_instrs_UMULL_Op_A_txt<T: Tracer>(
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
        result: i128,
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
        // D s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (s_0_3.value() as i128);
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
        // D s_0_10: cast zx s_0_9 -> i
        let s_0_10: i128 = (s_0_9.value() as i128);
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // D s_0_12: cast zx s_0_5 -> i
        let s_0_12: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_13: cast zx s_0_11 -> i
        let s_0_13: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_14: mul s_0_12 s_0_13
        let s_0_14: i128 = ((s_0_12) * (s_0_13));
        // D s_0_15: write-var result <= s_0_14
        fn_state.result = s_0_14;
        // C s_0_16: const #63s : i
        let s_0_16: i128 = 63;
        // C s_0_17: const #32s : i
        let s_0_17: i128 = 32;
        // D s_0_18: read-var result:i
        let s_0_18: i128 = fn_state.result;
        // D s_0_19: call integer_subrange(s_0_18, s_0_16, s_0_17)
        let s_0_19: Bits = integer_subrange(state, tracer, s_0_18, s_0_16, s_0_17);
        // D s_0_20: cast reint s_0_19 -> u32
        let s_0_20: u32 = (s_0_19.value() as u32);
        // D s_0_21: read-var dHi:i64
        let s_0_21: i64 = fn_state.dHi;
        // D s_0_22: cast zx s_0_21 -> i
        let s_0_22: i128 = (i128::try_from(s_0_21).unwrap());
        // D s_0_23: call R_set(s_0_22, s_0_20)
        let s_0_23: () = R_set(state, tracer, s_0_22, s_0_20);
        // C s_0_24: const #31s : i
        let s_0_24: i128 = 31;
        // C s_0_25: const #0s : i
        let s_0_25: i128 = 0;
        // D s_0_26: read-var result:i
        let s_0_26: i128 = fn_state.result;
        // D s_0_27: call integer_subrange(s_0_26, s_0_24, s_0_25)
        let s_0_27: Bits = integer_subrange(state, tracer, s_0_26, s_0_24, s_0_25);
        // D s_0_28: cast reint s_0_27 -> u32
        let s_0_28: u32 = (s_0_27.value() as u32);
        // D s_0_29: read-var dLo:i64
        let s_0_29: i64 = fn_state.dLo;
        // D s_0_30: cast zx s_0_29 -> i
        let s_0_30: i128 = (i128::try_from(s_0_29).unwrap());
        // D s_0_31: call R_set(s_0_30, s_0_28)
        let s_0_31: () = R_set(state, tracer, s_0_30, s_0_28);
        // D s_0_32: read-var setflags:u8
        let s_0_32: bool = fn_state.setflags;
        // N s_0_33: branch s_0_32 b2 b1
        if s_0_32 {
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
        // D s_2_1: read-var result:i
        let s_2_1: i128 = fn_state.result;
        // D s_2_2: call integer_access(s_2_1, s_2_0)
        let s_2_2: bool = integer_access(state, tracer, s_2_1, s_2_0);
        // C s_2_3: const #0s : i
        let s_2_3: i128 = 0;
        // C s_2_4: const #0u : u64
        let s_2_4: u64 = 0;
        // D s_2_5: cast zx s_2_2 -> u64
        let s_2_5: u64 = (s_2_2 as u64);
        // C s_2_6: const #1u : u64
        let s_2_6: u64 = 1;
        // D s_2_7: and s_2_5 s_2_6
        let s_2_7: u64 = ((s_2_5) & (s_2_6));
        // D s_2_8: cmp-eq s_2_7 s_2_6
        let s_2_8: bool = ((s_2_7) == (s_2_6));
        // D s_2_9: lsl s_2_5 s_2_3
        let s_2_9: u64 = s_2_5 << s_2_3;
        // D s_2_10: or s_2_4 s_2_9
        let s_2_10: u64 = ((s_2_4) | (s_2_9));
        // D s_2_11: cmpl s_2_9
        let s_2_11: u64 = !s_2_9;
        // D s_2_12: and s_2_4 s_2_11
        let s_2_12: u64 = ((s_2_4) & (s_2_11));
        // D s_2_13: select s_2_8 s_2_10 s_2_12
        let s_2_13: u64 = if s_2_8 { s_2_10 } else { s_2_12 };
        // D s_2_14: cast trunc s_2_13 -> u8
        let s_2_14: bool = ((s_2_13) != 0);
        // C s_2_15: const #16984u : u32
        let s_2_15: u32 = 16984;
        // N s_2_16: write-reg s_2_15 <= s_2_14
        let s_2_16: () = {
            state.write_register::<bool>(s_2_15 as isize, s_2_14);
            tracer.write_register(s_2_15 as isize, s_2_14);
        };
        // C s_2_17: const #63s : i
        let s_2_17: i128 = 63;
        // C s_2_18: const #0s : i
        let s_2_18: i128 = 0;
        // D s_2_19: read-var result:i
        let s_2_19: i128 = fn_state.result;
        // D s_2_20: call integer_subrange(s_2_19, s_2_17, s_2_18)
        let s_2_20: Bits = integer_subrange(state, tracer, s_2_19, s_2_17, s_2_18);
        // D s_2_21: cast reint s_2_20 -> u64
        let s_2_21: u64 = (s_2_20.value() as u64);
        // D s_2_22: cast zx s_2_21 -> bv
        let s_2_22: Bits = Bits::new(s_2_21 as u128, 64u16);
        // D s_2_23: call IsZeroBit(s_2_22)
        let s_2_23: bool = IsZeroBit(state, tracer, s_2_22);
        // C s_2_24: const #16997u : u32
        let s_2_24: u32 = 16997;
        // N s_2_25: write-reg s_2_24 <= s_2_23
        let s_2_25: () = {
            state.write_register::<bool>(s_2_24 as isize, s_2_23);
            tracer.write_register(s_2_24 as isize, s_2_23);
        };
        // N s_2_26: return
        return;
    }
}
