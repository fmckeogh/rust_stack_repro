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
use X_set::*;
use Zeros::*;
use common::*;
pub fn PM_SysRegRead_801ea1615b4051f7<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u__PSTATE_EL: u8,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var __PSTATE_EL <= s_0_1
        fn_state.u__PSTATE_EL = s_0_1;
        // D s_0_3: read-var __PSTATE_EL:u8
        let s_0_3: u8 = fn_state.u__PSTATE_EL;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // C s_0_5: const #448u : u32
        let s_0_5: u32 = 448;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: u8 = {
            let value = state.read_register::<u8>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // D s_0_8: cmp-eq s_0_4 s_0_7
        let s_0_8: bool = ((s_0_4) == (s_0_7));
        // N s_0_9: branch s_0_8 b8 b1
        if s_0_8 {
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
        // D s_1_0: read-var __PSTATE_EL:u8
        let s_1_0: u8 = fn_state.u__PSTATE_EL;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #440u : u32
        let s_1_2: u32 = 440;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // D s_1_5: cmp-eq s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) == (s_1_4));
        // N s_1_6: branch s_1_5 b7 b2
        if s_1_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var __PSTATE_EL:u8
        let s_2_0: u8 = fn_state.u__PSTATE_EL;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #432u : u32
        let s_2_2: u32 = 432;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cmp-eq s_2_1 s_2_4
        let s_2_5: bool = ((s_2_1) == (s_2_4));
        // N s_2_6: branch s_2_5 b6 b3
        if s_2_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var __PSTATE_EL:u8
        let s_3_0: u8 = fn_state.u__PSTATE_EL;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #424u : u32
        let s_3_2: u32 = 424;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // C s_5_1: const #31s : i
        let s_5_1: i128 = 31;
        // S s_5_2: call Zeros(s_5_1)
        let s_5_2: Bits = Zeros(state, tracer, s_5_1);
        // S s_5_3: cast reint s_5_2 -> u31
        let s_5_3: u32 = (s_5_2.value() as u32);
        // C s_5_4: const #16986u : u32
        let s_5_4: u32 = 16986;
        // D s_5_5: read-reg s_5_4:u8
        let s_5_5: bool = {
            let value = state.read_register::<bool>(s_5_4 as isize);
            tracer.read_register(s_5_4 as isize, value);
            value
        };
        // S s_5_6: cast zx s_5_3 -> bv
        let s_5_6: Bits = Bits::new(s_5_3 as u128, 31u16);
        // D s_5_7: cast zx s_5_5 -> bv
        let s_5_7: Bits = Bits::new(s_5_5 as u128, 1u16);
        // S s_5_8: cast reint s_5_6 -> u128
        let s_5_8: u128 = (s_5_6.value() as u128);
        // D s_5_9: size-of s_5_6
        let s_5_9: u16 = s_5_6.length();
        // D s_5_10: cast reint s_5_7 -> u128
        let s_5_10: u128 = (s_5_7.value() as u128);
        // D s_5_11: size-of s_5_7
        let s_5_11: u16 = s_5_7.length();
        // D s_5_12: lsl s_5_8 s_5_11
        let s_5_12: u128 = s_5_8 << s_5_11;
        // D s_5_13: or s_5_12 s_5_10
        let s_5_13: u128 = ((s_5_12) | (s_5_10));
        // D s_5_14: add s_5_9 s_5_11
        let s_5_14: u16 = (s_5_9 + s_5_11);
        // D s_5_15: create-bits s_5_13 s_5_14
        let s_5_15: Bits = Bits::new(s_5_13, s_5_14);
        // D s_5_16: cast reint s_5_15 -> u32
        let s_5_16: u32 = (s_5_15.value() as u32);
        // C s_5_17: const #32s : i
        let s_5_17: i128 = 32;
        // S s_5_18: call Zeros(s_5_17)
        let s_5_18: Bits = Zeros(state, tracer, s_5_17);
        // S s_5_19: cast reint s_5_18 -> u32
        let s_5_19: u32 = (s_5_18.value() as u32);
        // D s_5_20: cast zx s_5_16 -> bv
        let s_5_20: Bits = Bits::new(s_5_16 as u128, 32u16);
        // S s_5_21: cast zx s_5_19 -> bv
        let s_5_21: Bits = Bits::new(s_5_19 as u128, 32u16);
        // D s_5_22: cast reint s_5_20 -> u128
        let s_5_22: u128 = (s_5_20.value() as u128);
        // D s_5_23: size-of s_5_20
        let s_5_23: u16 = s_5_20.length();
        // S s_5_24: cast reint s_5_21 -> u128
        let s_5_24: u128 = (s_5_21.value() as u128);
        // D s_5_25: size-of s_5_21
        let s_5_25: u16 = s_5_21.length();
        // D s_5_26: lsl s_5_22 s_5_25
        let s_5_26: u128 = s_5_22 << s_5_25;
        // D s_5_27: or s_5_26 s_5_24
        let s_5_27: u128 = ((s_5_26) | (s_5_24));
        // D s_5_28: add s_5_23 s_5_25
        let s_5_28: u16 = (s_5_23 + s_5_25);
        // D s_5_29: create-bits s_5_27 s_5_28
        let s_5_29: Bits = Bits::new(s_5_27, s_5_28);
        // D s_5_30: cast reint s_5_29 -> u64
        let s_5_30: u64 = (s_5_29.value() as u64);
        // D s_5_31: cast zx s_5_30 -> bv
        let s_5_31: Bits = Bits::new(s_5_30 as u128, 64u16);
        // D s_5_32: read-var t:i
        let s_5_32: i128 = fn_state.t;
        // D s_5_33: call X_set(s_5_32, s_5_0, s_5_31)
        let s_5_33: () = X_set(state, tracer, s_5_32, s_5_0, s_5_31);
        // N s_5_34: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #31s : i
        let s_6_1: i128 = 31;
        // S s_6_2: call Zeros(s_6_1)
        let s_6_2: Bits = Zeros(state, tracer, s_6_1);
        // S s_6_3: cast reint s_6_2 -> u31
        let s_6_3: u32 = (s_6_2.value() as u32);
        // C s_6_4: const #16986u : u32
        let s_6_4: u32 = 16986;
        // D s_6_5: read-reg s_6_4:u8
        let s_6_5: bool = {
            let value = state.read_register::<bool>(s_6_4 as isize);
            tracer.read_register(s_6_4 as isize, value);
            value
        };
        // S s_6_6: cast zx s_6_3 -> bv
        let s_6_6: Bits = Bits::new(s_6_3 as u128, 31u16);
        // D s_6_7: cast zx s_6_5 -> bv
        let s_6_7: Bits = Bits::new(s_6_5 as u128, 1u16);
        // S s_6_8: cast reint s_6_6 -> u128
        let s_6_8: u128 = (s_6_6.value() as u128);
        // D s_6_9: size-of s_6_6
        let s_6_9: u16 = s_6_6.length();
        // D s_6_10: cast reint s_6_7 -> u128
        let s_6_10: u128 = (s_6_7.value() as u128);
        // D s_6_11: size-of s_6_7
        let s_6_11: u16 = s_6_7.length();
        // D s_6_12: lsl s_6_8 s_6_11
        let s_6_12: u128 = s_6_8 << s_6_11;
        // D s_6_13: or s_6_12 s_6_10
        let s_6_13: u128 = ((s_6_12) | (s_6_10));
        // D s_6_14: add s_6_9 s_6_11
        let s_6_14: u16 = (s_6_9 + s_6_11);
        // D s_6_15: create-bits s_6_13 s_6_14
        let s_6_15: Bits = Bits::new(s_6_13, s_6_14);
        // D s_6_16: cast reint s_6_15 -> u32
        let s_6_16: u32 = (s_6_15.value() as u32);
        // C s_6_17: const #32s : i
        let s_6_17: i128 = 32;
        // S s_6_18: call Zeros(s_6_17)
        let s_6_18: Bits = Zeros(state, tracer, s_6_17);
        // S s_6_19: cast reint s_6_18 -> u32
        let s_6_19: u32 = (s_6_18.value() as u32);
        // D s_6_20: cast zx s_6_16 -> bv
        let s_6_20: Bits = Bits::new(s_6_16 as u128, 32u16);
        // S s_6_21: cast zx s_6_19 -> bv
        let s_6_21: Bits = Bits::new(s_6_19 as u128, 32u16);
        // D s_6_22: cast reint s_6_20 -> u128
        let s_6_22: u128 = (s_6_20.value() as u128);
        // D s_6_23: size-of s_6_20
        let s_6_23: u16 = s_6_20.length();
        // S s_6_24: cast reint s_6_21 -> u128
        let s_6_24: u128 = (s_6_21.value() as u128);
        // D s_6_25: size-of s_6_21
        let s_6_25: u16 = s_6_21.length();
        // D s_6_26: lsl s_6_22 s_6_25
        let s_6_26: u128 = s_6_22 << s_6_25;
        // D s_6_27: or s_6_26 s_6_24
        let s_6_27: u128 = ((s_6_26) | (s_6_24));
        // D s_6_28: add s_6_23 s_6_25
        let s_6_28: u16 = (s_6_23 + s_6_25);
        // D s_6_29: create-bits s_6_27 s_6_28
        let s_6_29: Bits = Bits::new(s_6_27, s_6_28);
        // D s_6_30: cast reint s_6_29 -> u64
        let s_6_30: u64 = (s_6_29.value() as u64);
        // D s_6_31: cast zx s_6_30 -> bv
        let s_6_31: Bits = Bits::new(s_6_30 as u128, 64u16);
        // D s_6_32: read-var t:i
        let s_6_32: i128 = fn_state.t;
        // D s_6_33: call X_set(s_6_32, s_6_0, s_6_31)
        let s_6_33: () = X_set(state, tracer, s_6_32, s_6_0, s_6_31);
        // N s_6_34: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #31s : i
        let s_7_1: i128 = 31;
        // S s_7_2: call Zeros(s_7_1)
        let s_7_2: Bits = Zeros(state, tracer, s_7_1);
        // S s_7_3: cast reint s_7_2 -> u31
        let s_7_3: u32 = (s_7_2.value() as u32);
        // C s_7_4: const #16986u : u32
        let s_7_4: u32 = 16986;
        // D s_7_5: read-reg s_7_4:u8
        let s_7_5: bool = {
            let value = state.read_register::<bool>(s_7_4 as isize);
            tracer.read_register(s_7_4 as isize, value);
            value
        };
        // S s_7_6: cast zx s_7_3 -> bv
        let s_7_6: Bits = Bits::new(s_7_3 as u128, 31u16);
        // D s_7_7: cast zx s_7_5 -> bv
        let s_7_7: Bits = Bits::new(s_7_5 as u128, 1u16);
        // S s_7_8: cast reint s_7_6 -> u128
        let s_7_8: u128 = (s_7_6.value() as u128);
        // D s_7_9: size-of s_7_6
        let s_7_9: u16 = s_7_6.length();
        // D s_7_10: cast reint s_7_7 -> u128
        let s_7_10: u128 = (s_7_7.value() as u128);
        // D s_7_11: size-of s_7_7
        let s_7_11: u16 = s_7_7.length();
        // D s_7_12: lsl s_7_8 s_7_11
        let s_7_12: u128 = s_7_8 << s_7_11;
        // D s_7_13: or s_7_12 s_7_10
        let s_7_13: u128 = ((s_7_12) | (s_7_10));
        // D s_7_14: add s_7_9 s_7_11
        let s_7_14: u16 = (s_7_9 + s_7_11);
        // D s_7_15: create-bits s_7_13 s_7_14
        let s_7_15: Bits = Bits::new(s_7_13, s_7_14);
        // D s_7_16: cast reint s_7_15 -> u32
        let s_7_16: u32 = (s_7_15.value() as u32);
        // C s_7_17: const #32s : i
        let s_7_17: i128 = 32;
        // S s_7_18: call Zeros(s_7_17)
        let s_7_18: Bits = Zeros(state, tracer, s_7_17);
        // S s_7_19: cast reint s_7_18 -> u32
        let s_7_19: u32 = (s_7_18.value() as u32);
        // D s_7_20: cast zx s_7_16 -> bv
        let s_7_20: Bits = Bits::new(s_7_16 as u128, 32u16);
        // S s_7_21: cast zx s_7_19 -> bv
        let s_7_21: Bits = Bits::new(s_7_19 as u128, 32u16);
        // D s_7_22: cast reint s_7_20 -> u128
        let s_7_22: u128 = (s_7_20.value() as u128);
        // D s_7_23: size-of s_7_20
        let s_7_23: u16 = s_7_20.length();
        // S s_7_24: cast reint s_7_21 -> u128
        let s_7_24: u128 = (s_7_21.value() as u128);
        // D s_7_25: size-of s_7_21
        let s_7_25: u16 = s_7_21.length();
        // D s_7_26: lsl s_7_22 s_7_25
        let s_7_26: u128 = s_7_22 << s_7_25;
        // D s_7_27: or s_7_26 s_7_24
        let s_7_27: u128 = ((s_7_26) | (s_7_24));
        // D s_7_28: add s_7_23 s_7_25
        let s_7_28: u16 = (s_7_23 + s_7_25);
        // D s_7_29: create-bits s_7_27 s_7_28
        let s_7_29: Bits = Bits::new(s_7_27, s_7_28);
        // D s_7_30: cast reint s_7_29 -> u64
        let s_7_30: u64 = (s_7_29.value() as u64);
        // D s_7_31: cast zx s_7_30 -> bv
        let s_7_31: Bits = Bits::new(s_7_30 as u128, 64u16);
        // D s_7_32: read-var t:i
        let s_7_32: i128 = fn_state.t;
        // D s_7_33: call X_set(s_7_32, s_7_0, s_7_31)
        let s_7_33: () = X_set(state, tracer, s_7_32, s_7_0, s_7_31);
        // N s_7_34: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
}
