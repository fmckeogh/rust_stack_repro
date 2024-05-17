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
use CheckVFPEnabled::*;
use R_read::*;
use Zeros::*;
use S_set::*;
use S_read::*;
use R_set::*;
use common::*;
pub fn execute_aarch32_instrs_VMOV_h_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    t: i64,
    to_arm_register: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        n: i64,
        t: i64,
        to_arm_register: bool,
    }
    let fn_state = FunctionState {
        n,
        t,
        to_arm_register,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var to_arm_register:u8
        let s_1_0: bool = fn_state.to_arm_register;
        // N s_1_1: branch s_1_0 b3 b2
        if s_1_0 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #16s : i
        let s_2_0: i128 = 16;
        // S s_2_1: call Zeros(s_2_0)
        let s_2_1: Bits = Zeros(state, tracer, s_2_0);
        // S s_2_2: cast reint s_2_1 -> u16
        let s_2_2: u16 = (s_2_1.value() as u16);
        // D s_2_3: read-var t:i64
        let s_2_3: i64 = fn_state.t;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: call R_read(s_2_4)
        let s_2_5: u32 = R_read(state, tracer, s_2_4);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // D s_2_7: cast zx s_2_5 -> bv
        let s_2_7: Bits = Bits::new(s_2_5 as u128, 32u16);
        // C s_2_8: const #1s : i64
        let s_2_8: i64 = 1;
        // C s_2_9: cast zx s_2_8 -> i
        let s_2_9: i128 = (i128::try_from(s_2_8).unwrap());
        // C s_2_10: const #15s : i
        let s_2_10: i128 = 15;
        // C s_2_11: add s_2_10 s_2_9
        let s_2_11: i128 = (s_2_10 + s_2_9);
        // D s_2_12: bit-extract s_2_7 s_2_6 s_2_11
        let s_2_12: Bits = (Bits::new(
            ((s_2_7) >> (s_2_6)).value(),
            u16::try_from(s_2_11).unwrap(),
        ));
        // D s_2_13: cast reint s_2_12 -> u16
        let s_2_13: u16 = (s_2_12.value() as u16);
        // S s_2_14: cast zx s_2_2 -> bv
        let s_2_14: Bits = Bits::new(s_2_2 as u128, 16u16);
        // D s_2_15: cast zx s_2_13 -> bv
        let s_2_15: Bits = Bits::new(s_2_13 as u128, 16u16);
        // S s_2_16: cast reint s_2_14 -> u128
        let s_2_16: u128 = (s_2_14.value() as u128);
        // D s_2_17: size-of s_2_14
        let s_2_17: u16 = s_2_14.length();
        // D s_2_18: cast reint s_2_15 -> u128
        let s_2_18: u128 = (s_2_15.value() as u128);
        // D s_2_19: size-of s_2_15
        let s_2_19: u16 = s_2_15.length();
        // D s_2_20: lsl s_2_16 s_2_19
        let s_2_20: u128 = s_2_16 << s_2_19;
        // D s_2_21: or s_2_20 s_2_18
        let s_2_21: u128 = ((s_2_20) | (s_2_18));
        // D s_2_22: add s_2_17 s_2_19
        let s_2_22: u16 = (s_2_17 + s_2_19);
        // D s_2_23: create-bits s_2_21 s_2_22
        let s_2_23: Bits = Bits::new(s_2_21, s_2_22);
        // D s_2_24: cast reint s_2_23 -> u32
        let s_2_24: u32 = (s_2_23.value() as u32);
        // D s_2_25: read-var n:i64
        let s_2_25: i64 = fn_state.n;
        // D s_2_26: cast zx s_2_25 -> i
        let s_2_26: i128 = (i128::try_from(s_2_25).unwrap());
        // D s_2_27: call S_set(s_2_26, s_2_24)
        let s_2_27: () = S_set(state, tracer, s_2_26, s_2_24);
        // N s_2_28: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #16s : i
        let s_3_0: i128 = 16;
        // S s_3_1: call Zeros(s_3_0)
        let s_3_1: Bits = Zeros(state, tracer, s_3_0);
        // S s_3_2: cast reint s_3_1 -> u16
        let s_3_2: u16 = (s_3_1.value() as u16);
        // D s_3_3: read-var n:i64
        let s_3_3: i64 = fn_state.n;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: call S_read(s_3_4)
        let s_3_5: u32 = S_read(state, tracer, s_3_4);
        // C s_3_6: const #0s : i
        let s_3_6: i128 = 0;
        // D s_3_7: cast zx s_3_5 -> bv
        let s_3_7: Bits = Bits::new(s_3_5 as u128, 32u16);
        // C s_3_8: const #1s : i64
        let s_3_8: i64 = 1;
        // C s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // C s_3_10: const #15s : i
        let s_3_10: i128 = 15;
        // C s_3_11: add s_3_10 s_3_9
        let s_3_11: i128 = (s_3_10 + s_3_9);
        // D s_3_12: bit-extract s_3_7 s_3_6 s_3_11
        let s_3_12: Bits = (Bits::new(
            ((s_3_7) >> (s_3_6)).value(),
            u16::try_from(s_3_11).unwrap(),
        ));
        // D s_3_13: cast reint s_3_12 -> u16
        let s_3_13: u16 = (s_3_12.value() as u16);
        // S s_3_14: cast zx s_3_2 -> bv
        let s_3_14: Bits = Bits::new(s_3_2 as u128, 16u16);
        // D s_3_15: cast zx s_3_13 -> bv
        let s_3_15: Bits = Bits::new(s_3_13 as u128, 16u16);
        // S s_3_16: cast reint s_3_14 -> u128
        let s_3_16: u128 = (s_3_14.value() as u128);
        // D s_3_17: size-of s_3_14
        let s_3_17: u16 = s_3_14.length();
        // D s_3_18: cast reint s_3_15 -> u128
        let s_3_18: u128 = (s_3_15.value() as u128);
        // D s_3_19: size-of s_3_15
        let s_3_19: u16 = s_3_15.length();
        // D s_3_20: lsl s_3_16 s_3_19
        let s_3_20: u128 = s_3_16 << s_3_19;
        // D s_3_21: or s_3_20 s_3_18
        let s_3_21: u128 = ((s_3_20) | (s_3_18));
        // D s_3_22: add s_3_17 s_3_19
        let s_3_22: u16 = (s_3_17 + s_3_19);
        // D s_3_23: create-bits s_3_21 s_3_22
        let s_3_23: Bits = Bits::new(s_3_21, s_3_22);
        // D s_3_24: cast reint s_3_23 -> u32
        let s_3_24: u32 = (s_3_23.value() as u32);
        // D s_3_25: read-var t:i64
        let s_3_25: i64 = fn_state.t;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: call R_set(s_3_26, s_3_24)
        let s_3_27: () = R_set(state, tracer, s_3_26, s_3_24);
        // N s_3_28: return
        return;
    }
}
