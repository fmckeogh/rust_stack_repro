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
pub fn SPSel_SysRegRead_40cc9773567bb9cf<T: Tracer>(
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
        u__PSTATE_SP: bool,
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
        // C s_0_3: const #16990u : u32
        let s_0_3: u32 = 16990;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: bool = {
            let value = state.read_register::<bool>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: write-var __PSTATE_SP <= s_0_4
        fn_state.u__PSTATE_SP = s_0_4;
        // D s_0_6: read-var __PSTATE_EL:u8
        let s_0_6: u8 = fn_state.u__PSTATE_EL;
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // C s_0_8: const #448u : u32
        let s_0_8: u32 = 448;
        // D s_0_9: read-reg s_0_8:u8
        let s_0_9: u8 = {
            let value = state.read_register::<u8>(s_0_8 as isize);
            tracer.read_register(s_0_8 as isize, value);
            value
        };
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 2u16);
        // D s_0_11: cmp-eq s_0_7 s_0_10
        let s_0_11: bool = ((s_0_7) == (s_0_10));
        // N s_0_12: branch s_0_11 b8 b1
        if s_0_11 {
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
        // C s_5_1: const #63s : i
        let s_5_1: i128 = 63;
        // S s_5_2: call Zeros(s_5_1)
        let s_5_2: Bits = Zeros(state, tracer, s_5_1);
        // S s_5_3: cast reint s_5_2 -> u63
        let s_5_3: u64 = (s_5_2.value() as u64);
        // S s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 63u16);
        // D s_5_5: read-var __PSTATE_SP:u8
        let s_5_5: bool = fn_state.u__PSTATE_SP;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 1u16);
        // S s_5_7: cast reint s_5_4 -> u128
        let s_5_7: u128 = (s_5_4.value() as u128);
        // D s_5_8: size-of s_5_4
        let s_5_8: u16 = s_5_4.length();
        // D s_5_9: cast reint s_5_6 -> u128
        let s_5_9: u128 = (s_5_6.value() as u128);
        // D s_5_10: size-of s_5_6
        let s_5_10: u16 = s_5_6.length();
        // D s_5_11: lsl s_5_7 s_5_10
        let s_5_11: u128 = s_5_7 << s_5_10;
        // D s_5_12: or s_5_11 s_5_9
        let s_5_12: u128 = ((s_5_11) | (s_5_9));
        // D s_5_13: add s_5_8 s_5_10
        let s_5_13: u16 = (s_5_8 + s_5_10);
        // D s_5_14: create-bits s_5_12 s_5_13
        let s_5_14: Bits = Bits::new(s_5_12, s_5_13);
        // D s_5_15: cast reint s_5_14 -> u64
        let s_5_15: u64 = (s_5_14.value() as u64);
        // D s_5_16: cast zx s_5_15 -> bv
        let s_5_16: Bits = Bits::new(s_5_15 as u128, 64u16);
        // D s_5_17: read-var t:i
        let s_5_17: i128 = fn_state.t;
        // D s_5_18: call X_set(s_5_17, s_5_0, s_5_16)
        let s_5_18: () = X_set(state, tracer, s_5_17, s_5_0, s_5_16);
        // N s_5_19: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #63s : i
        let s_6_1: i128 = 63;
        // S s_6_2: call Zeros(s_6_1)
        let s_6_2: Bits = Zeros(state, tracer, s_6_1);
        // S s_6_3: cast reint s_6_2 -> u63
        let s_6_3: u64 = (s_6_2.value() as u64);
        // S s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 63u16);
        // D s_6_5: read-var __PSTATE_SP:u8
        let s_6_5: bool = fn_state.u__PSTATE_SP;
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 1u16);
        // S s_6_7: cast reint s_6_4 -> u128
        let s_6_7: u128 = (s_6_4.value() as u128);
        // D s_6_8: size-of s_6_4
        let s_6_8: u16 = s_6_4.length();
        // D s_6_9: cast reint s_6_6 -> u128
        let s_6_9: u128 = (s_6_6.value() as u128);
        // D s_6_10: size-of s_6_6
        let s_6_10: u16 = s_6_6.length();
        // D s_6_11: lsl s_6_7 s_6_10
        let s_6_11: u128 = s_6_7 << s_6_10;
        // D s_6_12: or s_6_11 s_6_9
        let s_6_12: u128 = ((s_6_11) | (s_6_9));
        // D s_6_13: add s_6_8 s_6_10
        let s_6_13: u16 = (s_6_8 + s_6_10);
        // D s_6_14: create-bits s_6_12 s_6_13
        let s_6_14: Bits = Bits::new(s_6_12, s_6_13);
        // D s_6_15: cast reint s_6_14 -> u64
        let s_6_15: u64 = (s_6_14.value() as u64);
        // D s_6_16: cast zx s_6_15 -> bv
        let s_6_16: Bits = Bits::new(s_6_15 as u128, 64u16);
        // D s_6_17: read-var t:i
        let s_6_17: i128 = fn_state.t;
        // D s_6_18: call X_set(s_6_17, s_6_0, s_6_16)
        let s_6_18: () = X_set(state, tracer, s_6_17, s_6_0, s_6_16);
        // N s_6_19: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #63s : i
        let s_7_1: i128 = 63;
        // S s_7_2: call Zeros(s_7_1)
        let s_7_2: Bits = Zeros(state, tracer, s_7_1);
        // S s_7_3: cast reint s_7_2 -> u63
        let s_7_3: u64 = (s_7_2.value() as u64);
        // S s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 63u16);
        // D s_7_5: read-var __PSTATE_SP:u8
        let s_7_5: bool = fn_state.u__PSTATE_SP;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 1u16);
        // S s_7_7: cast reint s_7_4 -> u128
        let s_7_7: u128 = (s_7_4.value() as u128);
        // D s_7_8: size-of s_7_4
        let s_7_8: u16 = s_7_4.length();
        // D s_7_9: cast reint s_7_6 -> u128
        let s_7_9: u128 = (s_7_6.value() as u128);
        // D s_7_10: size-of s_7_6
        let s_7_10: u16 = s_7_6.length();
        // D s_7_11: lsl s_7_7 s_7_10
        let s_7_11: u128 = s_7_7 << s_7_10;
        // D s_7_12: or s_7_11 s_7_9
        let s_7_12: u128 = ((s_7_11) | (s_7_9));
        // D s_7_13: add s_7_8 s_7_10
        let s_7_13: u16 = (s_7_8 + s_7_10);
        // D s_7_14: create-bits s_7_12 s_7_13
        let s_7_14: Bits = Bits::new(s_7_12, s_7_13);
        // D s_7_15: cast reint s_7_14 -> u64
        let s_7_15: u64 = (s_7_14.value() as u64);
        // D s_7_16: cast zx s_7_15 -> bv
        let s_7_16: Bits = Bits::new(s_7_15 as u128, 64u16);
        // D s_7_17: read-var t:i
        let s_7_17: i128 = fn_state.t;
        // D s_7_18: call X_set(s_7_17, s_7_0, s_7_16)
        let s_7_18: () = X_set(state, tracer, s_7_17, s_7_0, s_7_16);
        // N s_7_19: return
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
