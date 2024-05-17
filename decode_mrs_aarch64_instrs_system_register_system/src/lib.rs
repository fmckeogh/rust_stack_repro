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
use AArch64_CheckSystemAccess::*;
use execute_aarch64_instrs_system_register_system::*;
use common::*;
pub fn decode_mrs_aarch64_instrs_system_register_system<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    op2: u8,
    CRm: u8,
    CRn: u8,
    op1: u8,
    o0: bool,
    L: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rt: u8,
        op2: u8,
        CRm: u8,
        CRn: u8,
        op1: u8,
        o0: bool,
        L: bool,
    }
    let fn_state = FunctionState {
        Rt,
        op2,
        CRm,
        CRn,
        op1,
        o0,
        L,
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
        // C s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // D s_0_2: read-var o0:u8
        let s_0_2: bool = fn_state.o0;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // C s_0_4: cast reint s_0_1 -> u128
        let s_0_4: u128 = (s_0_1.value() as u128);
        // D s_0_5: size-of s_0_1
        let s_0_5: u16 = s_0_1.length();
        // D s_0_6: cast reint s_0_3 -> u128
        let s_0_6: u128 = (s_0_3.value() as u128);
        // D s_0_7: size-of s_0_3
        let s_0_7: u16 = s_0_3.length();
        // D s_0_8: lsl s_0_4 s_0_7
        let s_0_8: u128 = s_0_4 << s_0_7;
        // D s_0_9: or s_0_8 s_0_6
        let s_0_9: u128 = ((s_0_8) | (s_0_6));
        // D s_0_10: add s_0_5 s_0_7
        let s_0_10: u16 = (s_0_5 + s_0_7);
        // D s_0_11: create-bits s_0_9 s_0_10
        let s_0_11: Bits = Bits::new(s_0_9, s_0_10);
        // D s_0_12: cast reint s_0_11 -> u8
        let s_0_12: u8 = (s_0_11.value() as u8);
        // D s_0_13: read-var op1:u8
        let s_0_13: u8 = fn_state.op1;
        // D s_0_14: read-var CRn:u8
        let s_0_14: u8 = fn_state.CRn;
        // D s_0_15: read-var CRm:u8
        let s_0_15: u8 = fn_state.CRm;
        // D s_0_16: read-var op2:u8
        let s_0_16: u8 = fn_state.op2;
        // D s_0_17: read-var Rt:u8
        let s_0_17: u8 = fn_state.Rt;
        // D s_0_18: read-var L:u8
        let s_0_18: bool = fn_state.L;
        // D s_0_19: call AArch64_CheckSystemAccess(s_0_12, s_0_13, s_0_14, s_0_15, s_0_16, s_0_17, s_0_18)
        let s_0_19: () = AArch64_CheckSystemAccess(
            state,
            tracer,
            s_0_12,
            s_0_13,
            s_0_14,
            s_0_15,
            s_0_16,
            s_0_17,
            s_0_18,
        );
        // N s_0_20: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rt:u8
        let s_1_0: u8 = fn_state.Rt;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: read-var o0:u8
        let s_1_4: bool = fn_state.o0;
        // D s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 1u16);
        // D s_1_6: cast zx s_1_5 -> i
        let s_1_6: i128 = (s_1_5.value() as i128);
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // C s_1_8: const #2s : i
        let s_1_8: i128 = 2;
        // D s_1_9: cast zx s_1_7 -> i
        let s_1_9: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_10: add s_1_8 s_1_9
        let s_1_10: i128 = (s_1_8 + s_1_9);
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var op1:u8
        let s_1_12: u8 = fn_state.op1;
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 3u16);
        // D s_1_14: cast zx s_1_13 -> i
        let s_1_14: i128 = (s_1_13.value() as i128);
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var op2:u8
        let s_1_16: u8 = fn_state.op2;
        // D s_1_17: cast zx s_1_16 -> bv
        let s_1_17: Bits = Bits::new(s_1_16 as u128, 3u16);
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (s_1_17.value() as i128);
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // D s_1_20: read-var CRn:u8
        let s_1_20: u8 = fn_state.CRn;
        // D s_1_21: cast zx s_1_20 -> bv
        let s_1_21: Bits = Bits::new(s_1_20 as u128, 4u16);
        // D s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (s_1_21.value() as i128);
        // D s_1_23: cast reint s_1_22 -> i64
        let s_1_23: i64 = (s_1_22 as i64);
        // D s_1_24: read-var CRm:u8
        let s_1_24: u8 = fn_state.CRm;
        // D s_1_25: cast zx s_1_24 -> bv
        let s_1_25: Bits = Bits::new(s_1_24 as u128, 4u16);
        // D s_1_26: cast zx s_1_25 -> i
        let s_1_26: i128 = (s_1_25.value() as i128);
        // D s_1_27: cast reint s_1_26 -> i64
        let s_1_27: i64 = (s_1_26 as i64);
        // D s_1_28: read-var L:u8
        let s_1_28: bool = fn_state.L;
        // D s_1_29: cast zx s_1_28 -> bv
        let s_1_29: Bits = Bits::new(s_1_28 as u128, 1u16);
        // C s_1_30: const #1u : u8
        let s_1_30: bool = true;
        // C s_1_31: cast zx s_1_30 -> bv
        let s_1_31: Bits = Bits::new(s_1_30 as u128, 1u16);
        // D s_1_32: cmp-eq s_1_29 s_1_31
        let s_1_32: bool = ((s_1_29) == (s_1_31));
        // D s_1_33: call execute_aarch64_instrs_system_register_system(s_1_32, s_1_27, s_1_23, s_1_11, s_1_15, s_1_19, s_1_3)
        let s_1_33: () = execute_aarch64_instrs_system_register_system(
            state,
            tracer,
            s_1_32,
            s_1_27,
            s_1_23,
            s_1_11,
            s_1_15,
            s_1_19,
            s_1_3,
        );
        // N s_1_34: return
        return;
    }
}
