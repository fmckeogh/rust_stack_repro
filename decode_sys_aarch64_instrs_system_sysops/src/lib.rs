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
use execute_aarch64_instrs_system_sysops::*;
use common::*;
pub fn decode_sys_aarch64_instrs_system_sysops<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rt: u8,
    op2: u8,
    CRm: u8,
    CRn: u8,
    op1: u8,
    L: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        Rt: u8,
        op2: u8,
        CRm: u8,
        CRn: u8,
        op1: u8,
        L: bool,
    }
    let fn_state = FunctionState {
        Rt,
        op2,
        CRm,
        CRn,
        op1,
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
        let s_0_0: u8 = 1;
        // D s_0_1: read-var op1:u8
        let s_0_1: u8 = fn_state.op1;
        // D s_0_2: read-var CRn:u8
        let s_0_2: u8 = fn_state.CRn;
        // D s_0_3: read-var CRm:u8
        let s_0_3: u8 = fn_state.CRm;
        // D s_0_4: read-var op2:u8
        let s_0_4: u8 = fn_state.op2;
        // D s_0_5: read-var Rt:u8
        let s_0_5: u8 = fn_state.Rt;
        // D s_0_6: read-var L:u8
        let s_0_6: bool = fn_state.L;
        // D s_0_7: call AArch64_CheckSystemAccess(s_0_0, s_0_1, s_0_2, s_0_3, s_0_4, s_0_5, s_0_6)
        let s_0_7: () = AArch64_CheckSystemAccess(
            state,
            tracer,
            s_0_0,
            s_0_1,
            s_0_2,
            s_0_3,
            s_0_4,
            s_0_5,
            s_0_6,
        );
        // N s_0_8: jump b1
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
        // C s_1_4: const #1s : i64
        let s_1_4: i64 = 1;
        // D s_1_5: read-var op1:u8
        let s_1_5: u8 = fn_state.op1;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 3u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: read-var op2:u8
        let s_1_9: u8 = fn_state.op2;
        // D s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 3u16);
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (s_1_10.value() as i128);
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: read-var CRn:u8
        let s_1_13: u8 = fn_state.CRn;
        // D s_1_14: cast zx s_1_13 -> bv
        let s_1_14: Bits = Bits::new(s_1_13 as u128, 4u16);
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (s_1_14.value() as i128);
        // D s_1_16: cast reint s_1_15 -> i64
        let s_1_16: i64 = (s_1_15 as i64);
        // D s_1_17: read-var CRm:u8
        let s_1_17: u8 = fn_state.CRm;
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 4u16);
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (s_1_18.value() as i128);
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: read-var L:u8
        let s_1_21: bool = fn_state.L;
        // D s_1_22: cast zx s_1_21 -> bv
        let s_1_22: Bits = Bits::new(s_1_21 as u128, 1u16);
        // C s_1_23: const #1u : u8
        let s_1_23: bool = true;
        // C s_1_24: cast zx s_1_23 -> bv
        let s_1_24: Bits = Bits::new(s_1_23 as u128, 1u16);
        // D s_1_25: cmp-eq s_1_22 s_1_24
        let s_1_25: bool = ((s_1_22) == (s_1_24));
        // D s_1_26: call execute_aarch64_instrs_system_sysops(s_1_25, s_1_20, s_1_16, s_1_4, s_1_8, s_1_12, s_1_3)
        let s_1_26: () = execute_aarch64_instrs_system_sysops(
            state,
            tracer,
            s_1_25,
            s_1_20,
            s_1_16,
            s_1_4,
            s_1_8,
            s_1_12,
            s_1_3,
        );
        // N s_1_27: return
        return;
    }
}
