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
use HaveSysReg128::*;
use execute_aarch64_instrs_system_register_system_128::*;
use common::*;
pub fn decode_mrrs_aarch64_instrs_system_register_system_128<T: Tracer>(
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveSysReg128(s_0_0)
        let s_0_1: bool = HaveSysReg128(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b5 b1
        if s_0_2 {
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
        // C s_1_0: const #0s : i
        let s_1_0: i128 = 0;
        // D s_1_1: read-var Rt:u8
        let s_1_1: u8 = fn_state.Rt;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 5u16);
        // C s_1_3: const #1u : u64
        let s_1_3: u64 = 1;
        // D s_1_4: bit-extract s_1_2 s_1_0 s_1_3
        let s_1_4: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_5: cast reint s_1_4 -> u8
        let s_1_5: bool = ((s_1_4.value()) != 0);
        // C s_1_6: const #0s : i
        let s_1_6: i128 = 0;
        // C s_1_7: const #0u : u64
        let s_1_7: u64 = 0;
        // D s_1_8: cast zx s_1_5 -> u64
        let s_1_8: u64 = (s_1_5 as u64);
        // C s_1_9: const #1u : u64
        let s_1_9: u64 = 1;
        // D s_1_10: and s_1_8 s_1_9
        let s_1_10: u64 = ((s_1_8) & (s_1_9));
        // D s_1_11: cmp-eq s_1_10 s_1_9
        let s_1_11: bool = ((s_1_10) == (s_1_9));
        // D s_1_12: lsl s_1_8 s_1_6
        let s_1_12: u64 = s_1_8 << s_1_6;
        // D s_1_13: or s_1_7 s_1_12
        let s_1_13: u64 = ((s_1_7) | (s_1_12));
        // D s_1_14: cmpl s_1_12
        let s_1_14: u64 = !s_1_12;
        // D s_1_15: and s_1_7 s_1_14
        let s_1_15: u64 = ((s_1_7) & (s_1_14));
        // D s_1_16: select s_1_11 s_1_13 s_1_15
        let s_1_16: u64 = if s_1_11 { s_1_13 } else { s_1_15 };
        // D s_1_17: cast trunc s_1_16 -> u8
        let s_1_17: bool = ((s_1_16) != 0);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // C s_1_19: const #1u : u8
        let s_1_19: bool = true;
        // C s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // D s_1_21: cmp-eq s_1_18 s_1_20
        let s_1_21: bool = ((s_1_18) == (s_1_20));
        // N s_1_22: branch s_1_21 b4 b2
        if s_1_21 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #1u : u8
        let s_2_0: bool = true;
        // C s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_2: read-var o0:u8
        let s_2_2: bool = fn_state.o0;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // D s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // D s_2_12: cast reint s_2_11 -> u8
        let s_2_12: u8 = (s_2_11.value() as u8);
        // D s_2_13: read-var op1:u8
        let s_2_13: u8 = fn_state.op1;
        // D s_2_14: read-var CRn:u8
        let s_2_14: u8 = fn_state.CRn;
        // D s_2_15: read-var CRm:u8
        let s_2_15: u8 = fn_state.CRm;
        // D s_2_16: read-var op2:u8
        let s_2_16: u8 = fn_state.op2;
        // D s_2_17: read-var Rt:u8
        let s_2_17: u8 = fn_state.Rt;
        // D s_2_18: read-var L:u8
        let s_2_18: bool = fn_state.L;
        // D s_2_19: call AArch64_CheckSystemAccess(s_2_12, s_2_13, s_2_14, s_2_15, s_2_16, s_2_17, s_2_18)
        let s_2_19: () = AArch64_CheckSystemAccess(
            state,
            tracer,
            s_2_12,
            s_2_13,
            s_2_14,
            s_2_15,
            s_2_16,
            s_2_17,
            s_2_18,
        );
        // N s_2_20: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var Rt:u8
        let s_3_0: u8 = fn_state.Rt;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 5u16);
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // D s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: const #1s : i
        let s_3_4: i128 = 1;
        // D s_3_5: read-var Rt:u8
        let s_3_5: u8 = fn_state.Rt;
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 5u16);
        // C s_3_7: cast cvt s_3_4 -> bv
        let s_3_7: Bits = Bits::new(s_3_4 as u128, 128);
        // D s_3_8: add s_3_6 s_3_7
        let s_3_8: Bits = (s_3_6 + s_3_7);
        // D s_3_9: cast reint s_3_8 -> u8
        let s_3_9: u8 = (s_3_8.value() as u8);
        // D s_3_10: cast zx s_3_9 -> bv
        let s_3_10: Bits = Bits::new(s_3_9 as u128, 5u16);
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (s_3_10.value() as i128);
        // D s_3_12: cast reint s_3_11 -> i64
        let s_3_12: i64 = (s_3_11 as i64);
        // D s_3_13: read-var o0:u8
        let s_3_13: bool = fn_state.o0;
        // D s_3_14: cast zx s_3_13 -> bv
        let s_3_14: Bits = Bits::new(s_3_13 as u128, 1u16);
        // D s_3_15: cast zx s_3_14 -> i
        let s_3_15: i128 = (s_3_14.value() as i128);
        // D s_3_16: cast reint s_3_15 -> i64
        let s_3_16: i64 = (s_3_15 as i64);
        // C s_3_17: const #2s : i
        let s_3_17: i128 = 2;
        // D s_3_18: cast zx s_3_16 -> i
        let s_3_18: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_19: add s_3_17 s_3_18
        let s_3_19: i128 = (s_3_17 + s_3_18);
        // D s_3_20: cast reint s_3_19 -> i64
        let s_3_20: i64 = (s_3_19 as i64);
        // D s_3_21: read-var op1:u8
        let s_3_21: u8 = fn_state.op1;
        // D s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 3u16);
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (s_3_22.value() as i128);
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: read-var op2:u8
        let s_3_25: u8 = fn_state.op2;
        // D s_3_26: cast zx s_3_25 -> bv
        let s_3_26: Bits = Bits::new(s_3_25 as u128, 3u16);
        // D s_3_27: cast zx s_3_26 -> i
        let s_3_27: i128 = (s_3_26.value() as i128);
        // D s_3_28: cast reint s_3_27 -> i64
        let s_3_28: i64 = (s_3_27 as i64);
        // D s_3_29: read-var CRn:u8
        let s_3_29: u8 = fn_state.CRn;
        // D s_3_30: cast zx s_3_29 -> bv
        let s_3_30: Bits = Bits::new(s_3_29 as u128, 4u16);
        // D s_3_31: cast zx s_3_30 -> i
        let s_3_31: i128 = (s_3_30.value() as i128);
        // D s_3_32: cast reint s_3_31 -> i64
        let s_3_32: i64 = (s_3_31 as i64);
        // D s_3_33: read-var CRm:u8
        let s_3_33: u8 = fn_state.CRm;
        // D s_3_34: cast zx s_3_33 -> bv
        let s_3_34: Bits = Bits::new(s_3_33 as u128, 4u16);
        // D s_3_35: cast zx s_3_34 -> i
        let s_3_35: i128 = (s_3_34.value() as i128);
        // D s_3_36: cast reint s_3_35 -> i64
        let s_3_36: i64 = (s_3_35 as i64);
        // D s_3_37: read-var L:u8
        let s_3_37: bool = fn_state.L;
        // D s_3_38: cast zx s_3_37 -> bv
        let s_3_38: Bits = Bits::new(s_3_37 as u128, 1u16);
        // C s_3_39: const #1u : u8
        let s_3_39: bool = true;
        // C s_3_40: cast zx s_3_39 -> bv
        let s_3_40: Bits = Bits::new(s_3_39 as u128, 1u16);
        // D s_3_41: cmp-eq s_3_38 s_3_40
        let s_3_41: bool = ((s_3_38) == (s_3_40));
        // D s_3_42: call execute_aarch64_instrs_system_register_system_128(s_3_41, s_3_36, s_3_32, s_3_20, s_3_24, s_3_28, s_3_3, s_3_12)
        let s_3_42: () = execute_aarch64_instrs_system_register_system_128(
            state,
            tracer,
            s_3_41,
            s_3_36,
            s_3_32,
            s_3_20,
            s_3_24,
            s_3_28,
            s_3_3,
            s_3_12,
        );
        // N s_3_43: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: panic
        panic!("{:?}", ());
        // N s_5_1: return
        return;
    }
}
