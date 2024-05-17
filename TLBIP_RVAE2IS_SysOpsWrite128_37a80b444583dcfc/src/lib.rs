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
use AArch64_TLBIP_RVA::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HCR_EL2_Type_NV::*;
use IsFeatureImplemented::*;
use SecurityStateAtEL::*;
use VMID_read::*;
use EL2Enabled::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use common::*;
pub fn TLBIP_RVAE2IS_SysOpsWrite128_37a80b444583dcfc<T: Tracer>(
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
        gs_138627: bool,
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
        // C s_0_11: const #166u : u32
        let s_0_11: u32 = 166;
        // S s_0_12: call IsFeatureImplemented(s_0_11)
        let s_0_12: bool = IsFeatureImplemented(state, tracer, s_0_11);
        // S s_0_13: not s_0_12
        let s_0_13: bool = !s_0_12;
        // N s_0_14: branch s_0_13 b21 b1
        if s_0_13 {
            return block_21(state, tracer, fn_state);
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
        // C s_1_2: const #448u : u32
        let s_1_2: u32 = 448;
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
        // N s_1_6: branch s_1_5 b20 b2
        if s_1_5 {
            return block_20(state, tracer, fn_state);
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
        // C s_2_2: const #440u : u32
        let s_2_2: u32 = 440;
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
        // N s_2_6: branch s_2_5 b14 b3
        if s_2_5 {
            return block_14(state, tracer, fn_state);
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
        // C s_3_2: const #432u : u32
        let s_3_2: u32 = 432;
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
        // N s_3_6: branch s_3_5 b11 b4
        if s_3_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var __PSTATE_EL:u8
        let s_4_0: u8 = fn_state.u__PSTATE_EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #424u : u32
        let s_4_2: u32 = 424;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call EL2Enabled(s_6_0)
        let s_6_1: bool = EL2Enabled(state, tracer, s_6_0);
        // S s_6_2: not s_6_1
        let s_6_2: bool = !s_6_1;
        // N s_6_3: branch s_6_2 b10 b7
        if s_6_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var __HCR_EL2_E2H:u8
        let s_7_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
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
        // C s_8_3: const #() : ()
        let s_8_3: () = ();
        // S s_8_4: call VMID_read(s_8_3)
        let s_8_4: u16 = VMID_read(state, tracer, s_8_3);
        // C s_8_5: const #64s : i64
        let s_8_5: i64 = 64;
        // D s_8_6: read-var t2:i
        let s_8_6: i128 = fn_state.t2;
        // D s_8_7: call X_read(s_8_6, s_8_5)
        let s_8_7: Bits = X_read(state, tracer, s_8_6, s_8_5);
        // D s_8_8: cast reint s_8_7 -> u64
        let s_8_8: u64 = (s_8_7.value() as u64);
        // C s_8_9: const #64s : i64
        let s_8_9: i64 = 64;
        // D s_8_10: read-var t:i
        let s_8_10: i128 = fn_state.t;
        // D s_8_11: call X_read(s_8_10, s_8_9)
        let s_8_11: Bits = X_read(state, tracer, s_8_10, s_8_9);
        // D s_8_12: cast reint s_8_11 -> u64
        let s_8_12: u64 = (s_8_11.value() as u64);
        // D s_8_13: cast zx s_8_8 -> bv
        let s_8_13: Bits = Bits::new(s_8_8 as u128, 64u16);
        // D s_8_14: cast zx s_8_12 -> bv
        let s_8_14: Bits = Bits::new(s_8_12 as u128, 64u16);
        // D s_8_15: cast reint s_8_13 -> u128
        let s_8_15: u128 = (s_8_13.value() as u128);
        // D s_8_16: size-of s_8_13
        let s_8_16: u16 = s_8_13.length();
        // D s_8_17: cast reint s_8_14 -> u128
        let s_8_17: u128 = (s_8_14.value() as u128);
        // D s_8_18: size-of s_8_14
        let s_8_18: u16 = s_8_14.length();
        // D s_8_19: lsl s_8_15 s_8_18
        let s_8_19: u128 = s_8_15 << s_8_18;
        // D s_8_20: or s_8_19 s_8_17
        let s_8_20: u128 = ((s_8_19) | (s_8_17));
        // D s_8_21: add s_8_16 s_8_18
        let s_8_21: u16 = (s_8_16 + s_8_18);
        // D s_8_22: create-bits s_8_20 s_8_21
        let s_8_22: Bits = Bits::new(s_8_20, s_8_21);
        // D s_8_23: cast reint s_8_22 -> u128
        let s_8_23: u128 = (s_8_22.value() as u128);
        // C s_8_24: const #2u : u32
        let s_8_24: u32 = 2;
        // C s_8_25: const #1u : u32
        let s_8_25: u32 = 1;
        // C s_8_26: const #0u : u32
        let s_8_26: u32 = 0;
        // C s_8_27: const #1u : u32
        let s_8_27: u32 = 1;
        // D s_8_28: call AArch64_TLBIP_RVA(s_8_2, s_8_24, s_8_4, s_8_25, s_8_26, s_8_27, s_8_23)
        let s_8_28: () = AArch64_TLBIP_RVA(
            state,
            tracer,
            s_8_2,
            s_8_24,
            s_8_4,
            s_8_25,
            s_8_26,
            s_8_27,
            s_8_23,
        );
        // N s_8_29: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #432u : u32
        let s_9_0: u32 = 432;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call SecurityStateAtEL(s_9_1)
        let s_9_2: u32 = SecurityStateAtEL(state, tracer, s_9_1);
        // C s_9_3: const #64s : i64
        let s_9_3: i64 = 64;
        // D s_9_4: read-var t2:i
        let s_9_4: i128 = fn_state.t2;
        // D s_9_5: call X_read(s_9_4, s_9_3)
        let s_9_5: Bits = X_read(state, tracer, s_9_4, s_9_3);
        // D s_9_6: cast reint s_9_5 -> u64
        let s_9_6: u64 = (s_9_5.value() as u64);
        // C s_9_7: const #64s : i64
        let s_9_7: i64 = 64;
        // D s_9_8: read-var t:i
        let s_9_8: i128 = fn_state.t;
        // D s_9_9: call X_read(s_9_8, s_9_7)
        let s_9_9: Bits = X_read(state, tracer, s_9_8, s_9_7);
        // D s_9_10: cast reint s_9_9 -> u64
        let s_9_10: u64 = (s_9_9.value() as u64);
        // D s_9_11: cast zx s_9_6 -> bv
        let s_9_11: Bits = Bits::new(s_9_6 as u128, 64u16);
        // D s_9_12: cast zx s_9_10 -> bv
        let s_9_12: Bits = Bits::new(s_9_10 as u128, 64u16);
        // D s_9_13: cast reint s_9_11 -> u128
        let s_9_13: u128 = (s_9_11.value() as u128);
        // D s_9_14: size-of s_9_11
        let s_9_14: u16 = s_9_11.length();
        // D s_9_15: cast reint s_9_12 -> u128
        let s_9_15: u128 = (s_9_12.value() as u128);
        // D s_9_16: size-of s_9_12
        let s_9_16: u16 = s_9_12.length();
        // D s_9_17: lsl s_9_13 s_9_16
        let s_9_17: u128 = s_9_13 << s_9_16;
        // D s_9_18: or s_9_17 s_9_15
        let s_9_18: u128 = ((s_9_17) | (s_9_15));
        // D s_9_19: add s_9_14 s_9_16
        let s_9_19: u16 = (s_9_14 + s_9_16);
        // D s_9_20: create-bits s_9_18 s_9_19
        let s_9_20: Bits = Bits::new(s_9_18, s_9_19);
        // D s_9_21: cast reint s_9_20 -> u128
        let s_9_21: u128 = (s_9_20.value() as u128);
        // C s_9_22: const #3u : u32
        let s_9_22: u32 = 3;
        // C s_9_23: const #1080u : u32
        let s_9_23: u32 = 1080;
        // D s_9_24: read-reg s_9_23:u16
        let s_9_24: u16 = {
            let value = state.read_register::<u16>(s_9_23 as isize);
            tracer.read_register(s_9_23 as isize, value);
            value
        };
        // C s_9_25: const #1u : u32
        let s_9_25: u32 = 1;
        // C s_9_26: const #0u : u32
        let s_9_26: u32 = 0;
        // C s_9_27: const #1u : u32
        let s_9_27: u32 = 1;
        // D s_9_28: call AArch64_TLBIP_RVA(s_9_2, s_9_22, s_9_24, s_9_25, s_9_26, s_9_27, s_9_21)
        let s_9_28: () = AArch64_TLBIP_RVA(
            state,
            tracer,
            s_9_2,
            s_9_22,
            s_9_24,
            s_9_25,
            s_9_26,
            s_9_27,
            s_9_21,
        );
        // N s_9_29: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var __HCR_EL2_E2H:u8
        let s_11_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b13 b12
        if s_11_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
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
        // C s_12_3: const #() : ()
        let s_12_3: () = ();
        // S s_12_4: call VMID_read(s_12_3)
        let s_12_4: u16 = VMID_read(state, tracer, s_12_3);
        // C s_12_5: const #64s : i64
        let s_12_5: i64 = 64;
        // D s_12_6: read-var t2:i
        let s_12_6: i128 = fn_state.t2;
        // D s_12_7: call X_read(s_12_6, s_12_5)
        let s_12_7: Bits = X_read(state, tracer, s_12_6, s_12_5);
        // D s_12_8: cast reint s_12_7 -> u64
        let s_12_8: u64 = (s_12_7.value() as u64);
        // C s_12_9: const #64s : i64
        let s_12_9: i64 = 64;
        // D s_12_10: read-var t:i
        let s_12_10: i128 = fn_state.t;
        // D s_12_11: call X_read(s_12_10, s_12_9)
        let s_12_11: Bits = X_read(state, tracer, s_12_10, s_12_9);
        // D s_12_12: cast reint s_12_11 -> u64
        let s_12_12: u64 = (s_12_11.value() as u64);
        // D s_12_13: cast zx s_12_8 -> bv
        let s_12_13: Bits = Bits::new(s_12_8 as u128, 64u16);
        // D s_12_14: cast zx s_12_12 -> bv
        let s_12_14: Bits = Bits::new(s_12_12 as u128, 64u16);
        // D s_12_15: cast reint s_12_13 -> u128
        let s_12_15: u128 = (s_12_13.value() as u128);
        // D s_12_16: size-of s_12_13
        let s_12_16: u16 = s_12_13.length();
        // D s_12_17: cast reint s_12_14 -> u128
        let s_12_17: u128 = (s_12_14.value() as u128);
        // D s_12_18: size-of s_12_14
        let s_12_18: u16 = s_12_14.length();
        // D s_12_19: lsl s_12_15 s_12_18
        let s_12_19: u128 = s_12_15 << s_12_18;
        // D s_12_20: or s_12_19 s_12_17
        let s_12_20: u128 = ((s_12_19) | (s_12_17));
        // D s_12_21: add s_12_16 s_12_18
        let s_12_21: u16 = (s_12_16 + s_12_18);
        // D s_12_22: create-bits s_12_20 s_12_21
        let s_12_22: Bits = Bits::new(s_12_20, s_12_21);
        // D s_12_23: cast reint s_12_22 -> u128
        let s_12_23: u128 = (s_12_22.value() as u128);
        // C s_12_24: const #2u : u32
        let s_12_24: u32 = 2;
        // C s_12_25: const #1u : u32
        let s_12_25: u32 = 1;
        // C s_12_26: const #0u : u32
        let s_12_26: u32 = 0;
        // C s_12_27: const #1u : u32
        let s_12_27: u32 = 1;
        // D s_12_28: call AArch64_TLBIP_RVA(s_12_2, s_12_24, s_12_4, s_12_25, s_12_26, s_12_27, s_12_23)
        let s_12_28: () = AArch64_TLBIP_RVA(
            state,
            tracer,
            s_12_2,
            s_12_24,
            s_12_4,
            s_12_25,
            s_12_26,
            s_12_27,
            s_12_23,
        );
        // N s_12_29: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #432u : u32
        let s_13_0: u32 = 432;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call SecurityStateAtEL(s_13_1)
        let s_13_2: u32 = SecurityStateAtEL(state, tracer, s_13_1);
        // C s_13_3: const #64s : i64
        let s_13_3: i64 = 64;
        // D s_13_4: read-var t2:i
        let s_13_4: i128 = fn_state.t2;
        // D s_13_5: call X_read(s_13_4, s_13_3)
        let s_13_5: Bits = X_read(state, tracer, s_13_4, s_13_3);
        // D s_13_6: cast reint s_13_5 -> u64
        let s_13_6: u64 = (s_13_5.value() as u64);
        // C s_13_7: const #64s : i64
        let s_13_7: i64 = 64;
        // D s_13_8: read-var t:i
        let s_13_8: i128 = fn_state.t;
        // D s_13_9: call X_read(s_13_8, s_13_7)
        let s_13_9: Bits = X_read(state, tracer, s_13_8, s_13_7);
        // D s_13_10: cast reint s_13_9 -> u64
        let s_13_10: u64 = (s_13_9.value() as u64);
        // D s_13_11: cast zx s_13_6 -> bv
        let s_13_11: Bits = Bits::new(s_13_6 as u128, 64u16);
        // D s_13_12: cast zx s_13_10 -> bv
        let s_13_12: Bits = Bits::new(s_13_10 as u128, 64u16);
        // D s_13_13: cast reint s_13_11 -> u128
        let s_13_13: u128 = (s_13_11.value() as u128);
        // D s_13_14: size-of s_13_11
        let s_13_14: u16 = s_13_11.length();
        // D s_13_15: cast reint s_13_12 -> u128
        let s_13_15: u128 = (s_13_12.value() as u128);
        // D s_13_16: size-of s_13_12
        let s_13_16: u16 = s_13_12.length();
        // D s_13_17: lsl s_13_13 s_13_16
        let s_13_17: u128 = s_13_13 << s_13_16;
        // D s_13_18: or s_13_17 s_13_15
        let s_13_18: u128 = ((s_13_17) | (s_13_15));
        // D s_13_19: add s_13_14 s_13_16
        let s_13_19: u16 = (s_13_14 + s_13_16);
        // D s_13_20: create-bits s_13_18 s_13_19
        let s_13_20: Bits = Bits::new(s_13_18, s_13_19);
        // D s_13_21: cast reint s_13_20 -> u128
        let s_13_21: u128 = (s_13_20.value() as u128);
        // C s_13_22: const #3u : u32
        let s_13_22: u32 = 3;
        // C s_13_23: const #1080u : u32
        let s_13_23: u32 = 1080;
        // D s_13_24: read-reg s_13_23:u16
        let s_13_24: u16 = {
            let value = state.read_register::<u16>(s_13_23 as isize);
            tracer.read_register(s_13_23 as isize, value);
            value
        };
        // C s_13_25: const #1u : u32
        let s_13_25: u32 = 1;
        // C s_13_26: const #0u : u32
        let s_13_26: u32 = 0;
        // C s_13_27: const #1u : u32
        let s_13_27: u32 = 1;
        // D s_13_28: call AArch64_TLBIP_RVA(s_13_2, s_13_22, s_13_24, s_13_25, s_13_26, s_13_27, s_13_21)
        let s_13_28: () = AArch64_TLBIP_RVA(
            state,
            tracer,
            s_13_2,
            s_13_22,
            s_13_24,
            s_13_25,
            s_13_26,
            s_13_27,
            s_13_21,
        );
        // N s_13_29: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call EL2Enabled(s_14_0)
        let s_14_1: bool = EL2Enabled(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b19 b15
        if s_14_1 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#138627 <= s_15_0
        fn_state.gs_138627 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#138627:u8
        let s_16_0: bool = fn_state.gs_138627;
        // N s_16_1: branch s_16_0 b18 b17
        if s_16_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #20u : u8
        let s_18_0: u8 = 20;
        // C s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 8u16);
        // C s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (s_18_1.value() as i128);
        // C s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // C s_18_5: const #432u : u32
        let s_18_5: u32 = 432;
        // D s_18_6: read-reg s_18_5:u8
        let s_18_6: u8 = {
            let value = state.read_register::<u8>(s_18_5 as isize);
            tracer.read_register(s_18_5 as isize, value);
            value
        };
        // D s_18_7: call AArch64_SystemAccessTrap(s_18_6, s_18_4)
        let s_18_7: () = AArch64_SystemAccessTrap(state, tracer, s_18_6, s_18_4);
        // N s_18_8: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var __HCR_EL2_NV:u8
        let s_19_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #1u : u8
        let s_19_2: bool = true;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var gs#138627 <= s_19_4
        fn_state.gs_138627 = s_19_4;
        // N s_19_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
}
