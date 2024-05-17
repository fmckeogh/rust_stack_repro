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
use SecurityStateAtEL::*;
use u_get_HCR_EL2_Type_E2H::*;
use X_read::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_NV::*;
use AArch64_TLBIP_VA::*;
use AArch64_SystemAccessTrap::*;
use common::*;
pub fn TLBIP_VALE2OS_SysOpsWrite128_843e0c96a4c62b9d<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u__HCR_EL2_E2H: bool,
        gs_139479: bool,
        u__PSTATE_EL: u8,
        u__HCR_EL2_NV: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        t2,
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
        // C s_0_3: const #102552u : u32
        let s_0_3: u32 = 102552;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HCR_EL2_Type_NV(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_NV <= s_0_5
        fn_state.u__HCR_EL2_NV = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_E2H(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_E2H <= s_0_9
        fn_state.u__HCR_EL2_E2H = s_0_9;
        // D s_0_11: read-var __PSTATE_EL:u8
        let s_0_11: u8 = fn_state.u__PSTATE_EL;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_13: const #448u : u32
        let s_0_13: u32 = 448;
        // D s_0_14: read-reg s_0_13:u8
        let s_0_14: u8 = {
            let value = state.read_register::<u8>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cmp-eq s_0_12 s_0_15
        let s_0_16: bool = ((s_0_12) == (s_0_15));
        // N s_0_17: branch s_0_16 b19 b1
        if s_0_16 {
            return block_19(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b13 b2
        if s_1_5 {
            return block_13(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b10 b3
        if s_2_5 {
            return block_10(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call EL2Enabled(s_5_0)
        let s_5_1: bool = EL2Enabled(state, tracer, s_5_0);
        // S s_5_2: not s_5_1
        let s_5_2: bool = !s_5_1;
        // N s_5_3: branch s_5_2 b9 b6
        if s_5_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __HCR_EL2_E2H:u8
        let s_6_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #432u : u32
        let s_7_0: u32 = 432;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call SecurityStateAtEL(s_7_1)
        let s_7_2: u32 = SecurityStateAtEL(state, tracer, s_7_1);
        // C s_7_3: const #64s : i64
        let s_7_3: i64 = 64;
        // D s_7_4: read-var t2:i
        let s_7_4: i128 = fn_state.t2;
        // D s_7_5: call X_read(s_7_4, s_7_3)
        let s_7_5: Bits = X_read(state, tracer, s_7_4, s_7_3);
        // D s_7_6: cast reint s_7_5 -> u64
        let s_7_6: u64 = (s_7_5.value() as u64);
        // C s_7_7: const #64s : i64
        let s_7_7: i64 = 64;
        // D s_7_8: read-var t:i
        let s_7_8: i128 = fn_state.t;
        // D s_7_9: call X_read(s_7_8, s_7_7)
        let s_7_9: Bits = X_read(state, tracer, s_7_8, s_7_7);
        // D s_7_10: cast reint s_7_9 -> u64
        let s_7_10: u64 = (s_7_9.value() as u64);
        // D s_7_11: cast zx s_7_6 -> bv
        let s_7_11: Bits = Bits::new(s_7_6 as u128, 64u16);
        // D s_7_12: cast zx s_7_10 -> bv
        let s_7_12: Bits = Bits::new(s_7_10 as u128, 64u16);
        // D s_7_13: cast reint s_7_11 -> u128
        let s_7_13: u128 = (s_7_11.value() as u128);
        // D s_7_14: size-of s_7_11
        let s_7_14: u16 = s_7_11.length();
        // D s_7_15: cast reint s_7_12 -> u128
        let s_7_15: u128 = (s_7_12.value() as u128);
        // D s_7_16: size-of s_7_12
        let s_7_16: u16 = s_7_12.length();
        // D s_7_17: lsl s_7_13 s_7_16
        let s_7_17: u128 = s_7_13 << s_7_16;
        // D s_7_18: or s_7_17 s_7_15
        let s_7_18: u128 = ((s_7_17) | (s_7_15));
        // D s_7_19: add s_7_14 s_7_16
        let s_7_19: u16 = (s_7_14 + s_7_16);
        // D s_7_20: create-bits s_7_18 s_7_19
        let s_7_20: Bits = Bits::new(s_7_18, s_7_19);
        // D s_7_21: cast reint s_7_20 -> u128
        let s_7_21: u128 = (s_7_20.value() as u128);
        // C s_7_22: const #2u : u32
        let s_7_22: u32 = 2;
        // C s_7_23: const #1080u : u32
        let s_7_23: u32 = 1080;
        // D s_7_24: read-reg s_7_23:u16
        let s_7_24: u16 = {
            let value = state.read_register::<u16>(s_7_23 as isize);
            tracer.read_register(s_7_23 as isize, value);
            value
        };
        // C s_7_25: const #2u : u32
        let s_7_25: u32 = 2;
        // C s_7_26: const #1u : u32
        let s_7_26: u32 = 1;
        // C s_7_27: const #0u : u32
        let s_7_27: u32 = 0;
        // D s_7_28: call AArch64_TLBIP_VA(s_7_2, s_7_22, s_7_24, s_7_25, s_7_26, s_7_27, s_7_21)
        let s_7_28: () = AArch64_TLBIP_VA(
            state,
            tracer,
            s_7_2,
            s_7_22,
            s_7_24,
            s_7_25,
            s_7_26,
            s_7_27,
            s_7_21,
        );
        // N s_7_29: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #432u : u32
        let s_8_0: u32 = 432;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call SecurityStateAtEL(s_8_1)
        let s_8_2: u32 = SecurityStateAtEL(state, tracer, s_8_1);
        // C s_8_3: const #64s : i64
        let s_8_3: i64 = 64;
        // D s_8_4: read-var t2:i
        let s_8_4: i128 = fn_state.t2;
        // D s_8_5: call X_read(s_8_4, s_8_3)
        let s_8_5: Bits = X_read(state, tracer, s_8_4, s_8_3);
        // D s_8_6: cast reint s_8_5 -> u64
        let s_8_6: u64 = (s_8_5.value() as u64);
        // C s_8_7: const #64s : i64
        let s_8_7: i64 = 64;
        // D s_8_8: read-var t:i
        let s_8_8: i128 = fn_state.t;
        // D s_8_9: call X_read(s_8_8, s_8_7)
        let s_8_9: Bits = X_read(state, tracer, s_8_8, s_8_7);
        // D s_8_10: cast reint s_8_9 -> u64
        let s_8_10: u64 = (s_8_9.value() as u64);
        // D s_8_11: cast zx s_8_6 -> bv
        let s_8_11: Bits = Bits::new(s_8_6 as u128, 64u16);
        // D s_8_12: cast zx s_8_10 -> bv
        let s_8_12: Bits = Bits::new(s_8_10 as u128, 64u16);
        // D s_8_13: cast reint s_8_11 -> u128
        let s_8_13: u128 = (s_8_11.value() as u128);
        // D s_8_14: size-of s_8_11
        let s_8_14: u16 = s_8_11.length();
        // D s_8_15: cast reint s_8_12 -> u128
        let s_8_15: u128 = (s_8_12.value() as u128);
        // D s_8_16: size-of s_8_12
        let s_8_16: u16 = s_8_12.length();
        // D s_8_17: lsl s_8_13 s_8_16
        let s_8_17: u128 = s_8_13 << s_8_16;
        // D s_8_18: or s_8_17 s_8_15
        let s_8_18: u128 = ((s_8_17) | (s_8_15));
        // D s_8_19: add s_8_14 s_8_16
        let s_8_19: u16 = (s_8_14 + s_8_16);
        // D s_8_20: create-bits s_8_18 s_8_19
        let s_8_20: Bits = Bits::new(s_8_18, s_8_19);
        // D s_8_21: cast reint s_8_20 -> u128
        let s_8_21: u128 = (s_8_20.value() as u128);
        // C s_8_22: const #3u : u32
        let s_8_22: u32 = 3;
        // C s_8_23: const #1080u : u32
        let s_8_23: u32 = 1080;
        // D s_8_24: read-reg s_8_23:u16
        let s_8_24: u16 = {
            let value = state.read_register::<u16>(s_8_23 as isize);
            tracer.read_register(s_8_23 as isize, value);
            value
        };
        // C s_8_25: const #2u : u32
        let s_8_25: u32 = 2;
        // C s_8_26: const #1u : u32
        let s_8_26: u32 = 1;
        // C s_8_27: const #0u : u32
        let s_8_27: u32 = 0;
        // D s_8_28: call AArch64_TLBIP_VA(s_8_2, s_8_22, s_8_24, s_8_25, s_8_26, s_8_27, s_8_21)
        let s_8_28: () = AArch64_TLBIP_VA(
            state,
            tracer,
            s_8_2,
            s_8_22,
            s_8_24,
            s_8_25,
            s_8_26,
            s_8_27,
            s_8_21,
        );
        // N s_8_29: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var __HCR_EL2_E2H:u8
        let s_10_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #432u : u32
        let s_11_0: u32 = 432;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: u8 = {
            let value = state.read_register::<u8>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call SecurityStateAtEL(s_11_1)
        let s_11_2: u32 = SecurityStateAtEL(state, tracer, s_11_1);
        // C s_11_3: const #64s : i64
        let s_11_3: i64 = 64;
        // D s_11_4: read-var t2:i
        let s_11_4: i128 = fn_state.t2;
        // D s_11_5: call X_read(s_11_4, s_11_3)
        let s_11_5: Bits = X_read(state, tracer, s_11_4, s_11_3);
        // D s_11_6: cast reint s_11_5 -> u64
        let s_11_6: u64 = (s_11_5.value() as u64);
        // C s_11_7: const #64s : i64
        let s_11_7: i64 = 64;
        // D s_11_8: read-var t:i
        let s_11_8: i128 = fn_state.t;
        // D s_11_9: call X_read(s_11_8, s_11_7)
        let s_11_9: Bits = X_read(state, tracer, s_11_8, s_11_7);
        // D s_11_10: cast reint s_11_9 -> u64
        let s_11_10: u64 = (s_11_9.value() as u64);
        // D s_11_11: cast zx s_11_6 -> bv
        let s_11_11: Bits = Bits::new(s_11_6 as u128, 64u16);
        // D s_11_12: cast zx s_11_10 -> bv
        let s_11_12: Bits = Bits::new(s_11_10 as u128, 64u16);
        // D s_11_13: cast reint s_11_11 -> u128
        let s_11_13: u128 = (s_11_11.value() as u128);
        // D s_11_14: size-of s_11_11
        let s_11_14: u16 = s_11_11.length();
        // D s_11_15: cast reint s_11_12 -> u128
        let s_11_15: u128 = (s_11_12.value() as u128);
        // D s_11_16: size-of s_11_12
        let s_11_16: u16 = s_11_12.length();
        // D s_11_17: lsl s_11_13 s_11_16
        let s_11_17: u128 = s_11_13 << s_11_16;
        // D s_11_18: or s_11_17 s_11_15
        let s_11_18: u128 = ((s_11_17) | (s_11_15));
        // D s_11_19: add s_11_14 s_11_16
        let s_11_19: u16 = (s_11_14 + s_11_16);
        // D s_11_20: create-bits s_11_18 s_11_19
        let s_11_20: Bits = Bits::new(s_11_18, s_11_19);
        // D s_11_21: cast reint s_11_20 -> u128
        let s_11_21: u128 = (s_11_20.value() as u128);
        // C s_11_22: const #2u : u32
        let s_11_22: u32 = 2;
        // C s_11_23: const #1080u : u32
        let s_11_23: u32 = 1080;
        // D s_11_24: read-reg s_11_23:u16
        let s_11_24: u16 = {
            let value = state.read_register::<u16>(s_11_23 as isize);
            tracer.read_register(s_11_23 as isize, value);
            value
        };
        // C s_11_25: const #2u : u32
        let s_11_25: u32 = 2;
        // C s_11_26: const #1u : u32
        let s_11_26: u32 = 1;
        // C s_11_27: const #0u : u32
        let s_11_27: u32 = 0;
        // D s_11_28: call AArch64_TLBIP_VA(s_11_2, s_11_22, s_11_24, s_11_25, s_11_26, s_11_27, s_11_21)
        let s_11_28: () = AArch64_TLBIP_VA(
            state,
            tracer,
            s_11_2,
            s_11_22,
            s_11_24,
            s_11_25,
            s_11_26,
            s_11_27,
            s_11_21,
        );
        // N s_11_29: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #432u : u32
        let s_12_0: u32 = 432;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: u8 = {
            let value = state.read_register::<u8>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call SecurityStateAtEL(s_12_1)
        let s_12_2: u32 = SecurityStateAtEL(state, tracer, s_12_1);
        // C s_12_3: const #64s : i64
        let s_12_3: i64 = 64;
        // D s_12_4: read-var t2:i
        let s_12_4: i128 = fn_state.t2;
        // D s_12_5: call X_read(s_12_4, s_12_3)
        let s_12_5: Bits = X_read(state, tracer, s_12_4, s_12_3);
        // D s_12_6: cast reint s_12_5 -> u64
        let s_12_6: u64 = (s_12_5.value() as u64);
        // C s_12_7: const #64s : i64
        let s_12_7: i64 = 64;
        // D s_12_8: read-var t:i
        let s_12_8: i128 = fn_state.t;
        // D s_12_9: call X_read(s_12_8, s_12_7)
        let s_12_9: Bits = X_read(state, tracer, s_12_8, s_12_7);
        // D s_12_10: cast reint s_12_9 -> u64
        let s_12_10: u64 = (s_12_9.value() as u64);
        // D s_12_11: cast zx s_12_6 -> bv
        let s_12_11: Bits = Bits::new(s_12_6 as u128, 64u16);
        // D s_12_12: cast zx s_12_10 -> bv
        let s_12_12: Bits = Bits::new(s_12_10 as u128, 64u16);
        // D s_12_13: cast reint s_12_11 -> u128
        let s_12_13: u128 = (s_12_11.value() as u128);
        // D s_12_14: size-of s_12_11
        let s_12_14: u16 = s_12_11.length();
        // D s_12_15: cast reint s_12_12 -> u128
        let s_12_15: u128 = (s_12_12.value() as u128);
        // D s_12_16: size-of s_12_12
        let s_12_16: u16 = s_12_12.length();
        // D s_12_17: lsl s_12_13 s_12_16
        let s_12_17: u128 = s_12_13 << s_12_16;
        // D s_12_18: or s_12_17 s_12_15
        let s_12_18: u128 = ((s_12_17) | (s_12_15));
        // D s_12_19: add s_12_14 s_12_16
        let s_12_19: u16 = (s_12_14 + s_12_16);
        // D s_12_20: create-bits s_12_18 s_12_19
        let s_12_20: Bits = Bits::new(s_12_18, s_12_19);
        // D s_12_21: cast reint s_12_20 -> u128
        let s_12_21: u128 = (s_12_20.value() as u128);
        // C s_12_22: const #3u : u32
        let s_12_22: u32 = 3;
        // C s_12_23: const #1080u : u32
        let s_12_23: u32 = 1080;
        // D s_12_24: read-reg s_12_23:u16
        let s_12_24: u16 = {
            let value = state.read_register::<u16>(s_12_23 as isize);
            tracer.read_register(s_12_23 as isize, value);
            value
        };
        // C s_12_25: const #2u : u32
        let s_12_25: u32 = 2;
        // C s_12_26: const #1u : u32
        let s_12_26: u32 = 1;
        // C s_12_27: const #0u : u32
        let s_12_27: u32 = 0;
        // D s_12_28: call AArch64_TLBIP_VA(s_12_2, s_12_22, s_12_24, s_12_25, s_12_26, s_12_27, s_12_21)
        let s_12_28: () = AArch64_TLBIP_VA(
            state,
            tracer,
            s_12_2,
            s_12_22,
            s_12_24,
            s_12_25,
            s_12_26,
            s_12_27,
            s_12_21,
        );
        // N s_12_29: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call EL2Enabled(s_13_0)
        let s_13_1: bool = EL2Enabled(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b18 b14
        if s_13_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#139479 <= s_14_0
        fn_state.gs_139479 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#139479:u8
        let s_15_0: bool = fn_state.gs_139479;
        // N s_15_1: branch s_15_0 b17 b16
        if s_15_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #20u : u8
        let s_17_0: u8 = 20;
        // C s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 8u16);
        // C s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (s_17_1.value() as i128);
        // C s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // C s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // C s_17_5: const #432u : u32
        let s_17_5: u32 = 432;
        // D s_17_6: read-reg s_17_5:u8
        let s_17_6: u8 = {
            let value = state.read_register::<u8>(s_17_5 as isize);
            tracer.read_register(s_17_5 as isize, value);
            value
        };
        // D s_17_7: call AArch64_SystemAccessTrap(s_17_6, s_17_4)
        let s_17_7: () = AArch64_SystemAccessTrap(state, tracer, s_17_6, s_17_4);
        // N s_17_8: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var __HCR_EL2_NV:u8
        let s_18_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: write-var gs#139479 <= s_18_4
        fn_state.gs_139479 = s_18_4;
        // N s_18_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
}
