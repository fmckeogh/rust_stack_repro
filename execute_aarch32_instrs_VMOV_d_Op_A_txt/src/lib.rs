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
use D_set::*;
use CheckVFPEnabled::*;
use R_read::*;
use R_set::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VMOV_d_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    m: i64,
    t: i64,
    t2: i64,
    to_arm_registers: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        t: i64,
        t2: i64,
        to_arm_registers: bool,
    }
    let fn_state = FunctionState {
        m,
        t,
        t2,
        to_arm_registers,
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
        // D s_1_0: read-var to_arm_registers:u8
        let s_1_0: bool = fn_state.to_arm_registers;
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
        // D s_2_0: read-var m:i64
        let s_2_0: i64 = fn_state.m;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call D_read(s_2_1)
        let s_2_2: u64 = D_read(state, tracer, s_2_1);
        // D s_2_3: read-var t:i64
        let s_2_3: i64 = fn_state.t;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: call R_read(s_2_4)
        let s_2_5: u32 = R_read(state, tracer, s_2_4);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // D s_2_7: cast zx s_2_2 -> bv
        let s_2_7: Bits = Bits::new(s_2_2 as u128, 64u16);
        // D s_2_8: cast zx s_2_5 -> bv
        let s_2_8: Bits = Bits::new(s_2_5 as u128, 32u16);
        // C s_2_9: const #31s : i
        let s_2_9: i128 = 31;
        // C s_2_10: const #1u : u64
        let s_2_10: u64 = 1;
        // C s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 64u16);
        // C s_2_12: lsl s_2_11 s_2_9
        let s_2_12: Bits = s_2_11 << s_2_9;
        // C s_2_13: sub s_2_12 s_2_11
        let s_2_13: Bits = ((s_2_12) - (s_2_11));
        // D s_2_14: and s_2_8 s_2_13
        let s_2_14: Bits = ((s_2_8) & (s_2_13));
        // D s_2_15: lsl s_2_14 s_2_6
        let s_2_15: Bits = s_2_14 << s_2_6;
        // C s_2_16: lsl s_2_13 s_2_6
        let s_2_16: Bits = s_2_13 << s_2_6;
        // C s_2_17: cmpl s_2_16
        let s_2_17: Bits = !s_2_16;
        // D s_2_18: and s_2_7 s_2_17
        let s_2_18: Bits = ((s_2_7) & (s_2_17));
        // D s_2_19: or s_2_18 s_2_15
        let s_2_19: Bits = ((s_2_18) | (s_2_15));
        // D s_2_20: cast reint s_2_19 -> u64
        let s_2_20: u64 = (s_2_19.value() as u64);
        // D s_2_21: read-var m:i64
        let s_2_21: i64 = fn_state.m;
        // D s_2_22: cast zx s_2_21 -> i
        let s_2_22: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_23: call D_set(s_2_22, s_2_20)
        let s_2_23: () = D_set(state, tracer, s_2_22, s_2_20);
        // D s_2_24: read-var m:i64
        let s_2_24: i64 = fn_state.m;
        // D s_2_25: cast zx s_2_24 -> i
        let s_2_25: i128 = (i128::try_from(s_2_24).unwrap());
        // D s_2_26: call D_read(s_2_25)
        let s_2_26: u64 = D_read(state, tracer, s_2_25);
        // D s_2_27: read-var t2:i64
        let s_2_27: i64 = fn_state.t2;
        // D s_2_28: cast zx s_2_27 -> i
        let s_2_28: i128 = (i128::try_from(s_2_27).unwrap());
        // D s_2_29: call R_read(s_2_28)
        let s_2_29: u32 = R_read(state, tracer, s_2_28);
        // C s_2_30: const #32s : i
        let s_2_30: i128 = 32;
        // D s_2_31: cast zx s_2_26 -> bv
        let s_2_31: Bits = Bits::new(s_2_26 as u128, 64u16);
        // D s_2_32: cast zx s_2_29 -> bv
        let s_2_32: Bits = Bits::new(s_2_29 as u128, 32u16);
        // C s_2_33: const #31s : i
        let s_2_33: i128 = 31;
        // C s_2_34: const #1u : u64
        let s_2_34: u64 = 1;
        // C s_2_35: cast zx s_2_34 -> bv
        let s_2_35: Bits = Bits::new(s_2_34 as u128, 64u16);
        // C s_2_36: lsl s_2_35 s_2_33
        let s_2_36: Bits = s_2_35 << s_2_33;
        // C s_2_37: sub s_2_36 s_2_35
        let s_2_37: Bits = ((s_2_36) - (s_2_35));
        // D s_2_38: and s_2_32 s_2_37
        let s_2_38: Bits = ((s_2_32) & (s_2_37));
        // D s_2_39: lsl s_2_38 s_2_30
        let s_2_39: Bits = s_2_38 << s_2_30;
        // C s_2_40: lsl s_2_37 s_2_30
        let s_2_40: Bits = s_2_37 << s_2_30;
        // C s_2_41: cmpl s_2_40
        let s_2_41: Bits = !s_2_40;
        // D s_2_42: and s_2_31 s_2_41
        let s_2_42: Bits = ((s_2_31) & (s_2_41));
        // D s_2_43: or s_2_42 s_2_39
        let s_2_43: Bits = ((s_2_42) | (s_2_39));
        // D s_2_44: cast reint s_2_43 -> u64
        let s_2_44: u64 = (s_2_43.value() as u64);
        // D s_2_45: read-var m:i64
        let s_2_45: i64 = fn_state.m;
        // D s_2_46: cast zx s_2_45 -> i
        let s_2_46: i128 = (i128::try_from(s_2_45).unwrap());
        // D s_2_47: call D_set(s_2_46, s_2_44)
        let s_2_47: () = D_set(state, tracer, s_2_46, s_2_44);
        // N s_2_48: return
        return;
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
        // D s_3_2: call D_read(s_3_1)
        let s_3_2: u64 = D_read(state, tracer, s_3_1);
        // C s_3_3: const #0s : i
        let s_3_3: i128 = 0;
        // D s_3_4: cast zx s_3_2 -> bv
        let s_3_4: Bits = Bits::new(s_3_2 as u128, 64u16);
        // C s_3_5: const #1s : i64
        let s_3_5: i64 = 1;
        // C s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // C s_3_7: const #31s : i
        let s_3_7: i128 = 31;
        // C s_3_8: add s_3_7 s_3_6
        let s_3_8: i128 = (s_3_7 + s_3_6);
        // D s_3_9: bit-extract s_3_4 s_3_3 s_3_8
        let s_3_9: Bits = (Bits::new(
            ((s_3_4) >> (s_3_3)).value(),
            u16::try_from(s_3_8).unwrap(),
        ));
        // D s_3_10: cast reint s_3_9 -> u32
        let s_3_10: u32 = (s_3_9.value() as u32);
        // D s_3_11: read-var t:i64
        let s_3_11: i64 = fn_state.t;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: call R_set(s_3_12, s_3_10)
        let s_3_13: () = R_set(state, tracer, s_3_12, s_3_10);
        // D s_3_14: read-var m:i64
        let s_3_14: i64 = fn_state.m;
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (i128::try_from(s_3_14).unwrap());
        // D s_3_16: call D_read(s_3_15)
        let s_3_16: u64 = D_read(state, tracer, s_3_15);
        // C s_3_17: const #32s : i
        let s_3_17: i128 = 32;
        // D s_3_18: cast zx s_3_16 -> bv
        let s_3_18: Bits = Bits::new(s_3_16 as u128, 64u16);
        // C s_3_19: const #1s : i64
        let s_3_19: i64 = 1;
        // C s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // C s_3_21: const #31s : i
        let s_3_21: i128 = 31;
        // C s_3_22: add s_3_21 s_3_20
        let s_3_22: i128 = (s_3_21 + s_3_20);
        // D s_3_23: bit-extract s_3_18 s_3_17 s_3_22
        let s_3_23: Bits = (Bits::new(
            ((s_3_18) >> (s_3_17)).value(),
            u16::try_from(s_3_22).unwrap(),
        ));
        // D s_3_24: cast reint s_3_23 -> u32
        let s_3_24: u32 = (s_3_23.value() as u32);
        // D s_3_25: read-var t2:i64
        let s_3_25: i64 = fn_state.t2;
        // D s_3_26: cast zx s_3_25 -> i
        let s_3_26: i128 = (i128::try_from(s_3_25).unwrap());
        // D s_3_27: call R_set(s_3_26, s_3_24)
        let s_3_27: () = R_set(state, tracer, s_3_26, s_3_24);
        // N s_3_28: return
        return;
    }
}
