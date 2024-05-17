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
use u_get_HFGITR_EL2_Type_TLBIVAE1OS::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use VMID_read::*;
use X_read::*;
use AArch64_TLBIP_VA::*;
use u_get_HCR_EL2_Type_TTLB::*;
use u_get_HCRX_EL2_Type_FGTnXS::*;
use u_get_HCR_EL2_Type_TTLBOS::*;
use IsHCRXEL2Enabled::*;
use SecurityStateAtEL::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use common::*;
pub fn TLBIP_VAE1OS_SysOpsWrite128_5f990b510a7de734<T: Tracer>(
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
        gs_139200: bool,
        gs_139203: bool,
        gs_139201: bool,
        gs_139205: bool,
        u__HCRX_EL2_FGTnXS: bool,
        gs_139204: bool,
        gs_139202: bool,
        u__HFGITR_EL2_TLBIVAE1OS: bool,
        gs_139206: bool,
        u__HCR_EL2_TTLBOS: bool,
        u__PSTATE_EL: u8,
        gs_139198: bool,
        u__HCR_EL2_TTLB: bool,
        gs_139199: bool,
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
        // D s_0_5: call _get_HCR_EL2_Type_TTLB(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_TTLB(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_TTLB <= s_0_5
        fn_state.u__HCR_EL2_TTLB = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_TTLBOS(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_TTLBOS(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_TTLBOS <= s_0_9
        fn_state.u__HCR_EL2_TTLBOS = s_0_9;
        // C s_0_11: const #22528u : u32
        let s_0_11: u32 = 22528;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCRX_EL2_Type_FGTnXS(s_0_12)
        let s_0_13: bool = u_get_HCRX_EL2_Type_FGTnXS(state, tracer, s_0_12);
        // D s_0_14: write-var __HCRX_EL2_FGTnXS <= s_0_13
        fn_state.u__HCRX_EL2_FGTnXS = s_0_13;
        // C s_0_15: const #13608u : u32
        let s_0_15: u32 = 13608;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HFGITR_EL2_Type_TLBIVAE1OS(s_0_16)
        let s_0_17: bool = u_get_HFGITR_EL2_Type_TLBIVAE1OS(state, tracer, s_0_16);
        // D s_0_18: write-var __HFGITR_EL2_TLBIVAE1OS <= s_0_17
        fn_state.u__HFGITR_EL2_TLBIVAE1OS = s_0_17;
        // C s_0_19: const #166u : u32
        let s_0_19: u32 = 166;
        // S s_0_20: call IsFeatureImplemented(s_0_19)
        let s_0_20: bool = IsFeatureImplemented(state, tracer, s_0_19);
        // S s_0_21: not s_0_20
        let s_0_21: bool = !s_0_20;
        // N s_0_22: branch s_0_21 b47 b1
        if s_0_21 {
            return block_47(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b46 b2
        if s_1_5 {
            return block_46(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b12 b3
        if s_2_5 {
            return block_12(state, tracer, fn_state);
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
        // N s_3_6: branch s_3_5 b9 b4
        if s_3_5 {
            return block_9(state, tracer, fn_state);
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
        // C s_6_0: const #102552u : u32
        let s_6_0: u32 = 102552;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call _get_HCR_EL2_Type_E2H(s_6_1)
        let s_6_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_6_1);
        // C s_6_3: const #102552u : u32
        let s_6_3: u32 = 102552;
        // D s_6_4: read-reg s_6_3:struct
        let s_6_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_3 as isize);
            tracer.read_register(s_6_3 as isize, value);
            value
        };
        // D s_6_5: call _get_HCR_EL2_Type_TGE(s_6_4)
        let s_6_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_6_4);
        // D s_6_6: cast zx s_6_2 -> bv
        let s_6_6: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_7: cast zx s_6_5 -> bv
        let s_6_7: Bits = Bits::new(s_6_5 as u128, 1u16);
        // D s_6_8: cast reint s_6_6 -> u128
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
        // D s_6_16: cast reint s_6_15 -> u8
        let s_6_16: u8 = (s_6_15.value() as u8);
        // D s_6_17: cast zx s_6_16 -> bv
        let s_6_17: Bits = Bits::new(s_6_16 as u128, 2u16);
        // C s_6_18: const #3u : u8
        let s_6_18: u8 = 3;
        // C s_6_19: cast zx s_6_18 -> bv
        let s_6_19: Bits = Bits::new(s_6_18 as u128, 2u16);
        // D s_6_20: cmp-eq s_6_17 s_6_19
        let s_6_20: bool = ((s_6_17) == (s_6_19));
        // N s_6_21: branch s_6_20 b8 b7
        if s_6_20 {
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
        // C s_7_0: const #440u : u32
        let s_7_0: u32 = 440;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call SecurityStateAtEL(s_7_1)
        let s_7_2: u32 = SecurityStateAtEL(state, tracer, s_7_1);
        // C s_7_3: const #() : ()
        let s_7_3: () = ();
        // S s_7_4: call VMID_read(s_7_3)
        let s_7_4: u16 = VMID_read(state, tracer, s_7_3);
        // C s_7_5: const #64s : i64
        let s_7_5: i64 = 64;
        // D s_7_6: read-var t2:i
        let s_7_6: i128 = fn_state.t2;
        // D s_7_7: call X_read(s_7_6, s_7_5)
        let s_7_7: Bits = X_read(state, tracer, s_7_6, s_7_5);
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7.value() as u64);
        // C s_7_9: const #64s : i64
        let s_7_9: i64 = 64;
        // D s_7_10: read-var t:i
        let s_7_10: i128 = fn_state.t;
        // D s_7_11: call X_read(s_7_10, s_7_9)
        let s_7_11: Bits = X_read(state, tracer, s_7_10, s_7_9);
        // D s_7_12: cast reint s_7_11 -> u64
        let s_7_12: u64 = (s_7_11.value() as u64);
        // D s_7_13: cast zx s_7_8 -> bv
        let s_7_13: Bits = Bits::new(s_7_8 as u128, 64u16);
        // D s_7_14: cast zx s_7_12 -> bv
        let s_7_14: Bits = Bits::new(s_7_12 as u128, 64u16);
        // D s_7_15: cast reint s_7_13 -> u128
        let s_7_15: u128 = (s_7_13.value() as u128);
        // D s_7_16: size-of s_7_13
        let s_7_16: u16 = s_7_13.length();
        // D s_7_17: cast reint s_7_14 -> u128
        let s_7_17: u128 = (s_7_14.value() as u128);
        // D s_7_18: size-of s_7_14
        let s_7_18: u16 = s_7_14.length();
        // D s_7_19: lsl s_7_15 s_7_18
        let s_7_19: u128 = s_7_15 << s_7_18;
        // D s_7_20: or s_7_19 s_7_17
        let s_7_20: u128 = ((s_7_19) | (s_7_17));
        // D s_7_21: add s_7_16 s_7_18
        let s_7_21: u16 = (s_7_16 + s_7_18);
        // D s_7_22: create-bits s_7_20 s_7_21
        let s_7_22: Bits = Bits::new(s_7_20, s_7_21);
        // D s_7_23: cast reint s_7_22 -> u128
        let s_7_23: u128 = (s_7_22.value() as u128);
        // C s_7_24: const #4u : u32
        let s_7_24: u32 = 4;
        // C s_7_25: const #2u : u32
        let s_7_25: u32 = 2;
        // C s_7_26: const #0u : u32
        let s_7_26: u32 = 0;
        // C s_7_27: const #1u : u32
        let s_7_27: u32 = 1;
        // D s_7_28: call AArch64_TLBIP_VA(s_7_2, s_7_24, s_7_4, s_7_25, s_7_26, s_7_27, s_7_23)
        let s_7_28: () = AArch64_TLBIP_VA(
            state,
            tracer,
            s_7_2,
            s_7_24,
            s_7_4,
            s_7_25,
            s_7_26,
            s_7_27,
            s_7_23,
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
        // C s_8_26: const #0u : u32
        let s_8_26: u32 = 0;
        // C s_8_27: const #1u : u32
        let s_8_27: u32 = 1;
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
        // C s_9_0: const #102552u : u32
        let s_9_0: u32 = 102552;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_HCR_EL2_Type_E2H(s_9_1)
        let s_9_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_9_1);
        // C s_9_3: const #102552u : u32
        let s_9_3: u32 = 102552;
        // D s_9_4: read-reg s_9_3:struct
        let s_9_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_3 as isize);
            tracer.read_register(s_9_3 as isize, value);
            value
        };
        // D s_9_5: call _get_HCR_EL2_Type_TGE(s_9_4)
        let s_9_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_9_4);
        // D s_9_6: cast zx s_9_2 -> bv
        let s_9_6: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_7: cast zx s_9_5 -> bv
        let s_9_7: Bits = Bits::new(s_9_5 as u128, 1u16);
        // D s_9_8: cast reint s_9_6 -> u128
        let s_9_8: u128 = (s_9_6.value() as u128);
        // D s_9_9: size-of s_9_6
        let s_9_9: u16 = s_9_6.length();
        // D s_9_10: cast reint s_9_7 -> u128
        let s_9_10: u128 = (s_9_7.value() as u128);
        // D s_9_11: size-of s_9_7
        let s_9_11: u16 = s_9_7.length();
        // D s_9_12: lsl s_9_8 s_9_11
        let s_9_12: u128 = s_9_8 << s_9_11;
        // D s_9_13: or s_9_12 s_9_10
        let s_9_13: u128 = ((s_9_12) | (s_9_10));
        // D s_9_14: add s_9_9 s_9_11
        let s_9_14: u16 = (s_9_9 + s_9_11);
        // D s_9_15: create-bits s_9_13 s_9_14
        let s_9_15: Bits = Bits::new(s_9_13, s_9_14);
        // D s_9_16: cast reint s_9_15 -> u8
        let s_9_16: u8 = (s_9_15.value() as u8);
        // D s_9_17: cast zx s_9_16 -> bv
        let s_9_17: Bits = Bits::new(s_9_16 as u128, 2u16);
        // C s_9_18: const #3u : u8
        let s_9_18: u8 = 3;
        // C s_9_19: cast zx s_9_18 -> bv
        let s_9_19: Bits = Bits::new(s_9_18 as u128, 2u16);
        // D s_9_20: cmp-eq s_9_17 s_9_19
        let s_9_20: bool = ((s_9_17) == (s_9_19));
        // N s_9_21: branch s_9_20 b11 b10
        if s_9_20 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #440u : u32
        let s_10_0: u32 = 440;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call SecurityStateAtEL(s_10_1)
        let s_10_2: u32 = SecurityStateAtEL(state, tracer, s_10_1);
        // C s_10_3: const #() : ()
        let s_10_3: () = ();
        // S s_10_4: call VMID_read(s_10_3)
        let s_10_4: u16 = VMID_read(state, tracer, s_10_3);
        // C s_10_5: const #64s : i64
        let s_10_5: i64 = 64;
        // D s_10_6: read-var t2:i
        let s_10_6: i128 = fn_state.t2;
        // D s_10_7: call X_read(s_10_6, s_10_5)
        let s_10_7: Bits = X_read(state, tracer, s_10_6, s_10_5);
        // D s_10_8: cast reint s_10_7 -> u64
        let s_10_8: u64 = (s_10_7.value() as u64);
        // C s_10_9: const #64s : i64
        let s_10_9: i64 = 64;
        // D s_10_10: read-var t:i
        let s_10_10: i128 = fn_state.t;
        // D s_10_11: call X_read(s_10_10, s_10_9)
        let s_10_11: Bits = X_read(state, tracer, s_10_10, s_10_9);
        // D s_10_12: cast reint s_10_11 -> u64
        let s_10_12: u64 = (s_10_11.value() as u64);
        // D s_10_13: cast zx s_10_8 -> bv
        let s_10_13: Bits = Bits::new(s_10_8 as u128, 64u16);
        // D s_10_14: cast zx s_10_12 -> bv
        let s_10_14: Bits = Bits::new(s_10_12 as u128, 64u16);
        // D s_10_15: cast reint s_10_13 -> u128
        let s_10_15: u128 = (s_10_13.value() as u128);
        // D s_10_16: size-of s_10_13
        let s_10_16: u16 = s_10_13.length();
        // D s_10_17: cast reint s_10_14 -> u128
        let s_10_17: u128 = (s_10_14.value() as u128);
        // D s_10_18: size-of s_10_14
        let s_10_18: u16 = s_10_14.length();
        // D s_10_19: lsl s_10_15 s_10_18
        let s_10_19: u128 = s_10_15 << s_10_18;
        // D s_10_20: or s_10_19 s_10_17
        let s_10_20: u128 = ((s_10_19) | (s_10_17));
        // D s_10_21: add s_10_16 s_10_18
        let s_10_21: u16 = (s_10_16 + s_10_18);
        // D s_10_22: create-bits s_10_20 s_10_21
        let s_10_22: Bits = Bits::new(s_10_20, s_10_21);
        // D s_10_23: cast reint s_10_22 -> u128
        let s_10_23: u128 = (s_10_22.value() as u128);
        // C s_10_24: const #4u : u32
        let s_10_24: u32 = 4;
        // C s_10_25: const #2u : u32
        let s_10_25: u32 = 2;
        // C s_10_26: const #0u : u32
        let s_10_26: u32 = 0;
        // C s_10_27: const #1u : u32
        let s_10_27: u32 = 1;
        // D s_10_28: call AArch64_TLBIP_VA(s_10_2, s_10_24, s_10_4, s_10_25, s_10_26, s_10_27, s_10_23)
        let s_10_28: () = AArch64_TLBIP_VA(
            state,
            tracer,
            s_10_2,
            s_10_24,
            s_10_4,
            s_10_25,
            s_10_26,
            s_10_27,
            s_10_23,
        );
        // N s_10_29: return
        return;
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
        // C s_11_22: const #3u : u32
        let s_11_22: u32 = 3;
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
        // C s_11_26: const #0u : u32
        let s_11_26: u32 = 0;
        // C s_11_27: const #1u : u32
        let s_11_27: u32 = 1;
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
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call EL2Enabled(s_12_0)
        let s_12_1: bool = EL2Enabled(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b45 b13
        if s_12_1 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#139198 <= s_13_0
        fn_state.gs_139198 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#139198:u8
        let s_14_0: bool = fn_state.gs_139198;
        // N s_14_1: branch s_14_0 b44 b15
        if s_14_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call EL2Enabled(s_15_0)
        let s_15_1: bool = EL2Enabled(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b43 b16
        if s_15_1 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#139199 <= s_16_0
        fn_state.gs_139199 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#139199:u8
        let s_17_0: bool = fn_state.gs_139199;
        // N s_17_1: branch s_17_0 b42 b18
        if s_17_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call EL2Enabled(s_18_0)
        let s_18_1: bool = EL2Enabled(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b41 b19
        if s_18_1 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#139200 <= s_19_0
        fn_state.gs_139200 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#139200:u8
        let s_20_0: bool = fn_state.gs_139200;
        // N s_20_1: branch s_20_0 b37 b21
        if s_20_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#139202 <= s_21_0
        fn_state.gs_139202 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#139202:u8
        let s_22_0: bool = fn_state.gs_139202;
        // N s_22_1: branch s_22_0 b36 b23
        if s_22_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#139203 <= s_23_0
        fn_state.gs_139203 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#139203:u8
        let s_24_0: bool = fn_state.gs_139203;
        // N s_24_1: branch s_24_0 b32 b25
        if s_24_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#139205 <= s_25_0
        fn_state.gs_139205 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#139205:u8
        let s_26_0: bool = fn_state.gs_139205;
        // N s_26_1: branch s_26_0 b31 b27
        if s_26_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#139206 <= s_27_0
        fn_state.gs_139206 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#139206:u8
        let s_28_0: bool = fn_state.gs_139206;
        // N s_28_1: branch s_28_0 b30 b29
        if s_28_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #440u : u32
        let s_29_0: u32 = 440;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call SecurityStateAtEL(s_29_1)
        let s_29_2: u32 = SecurityStateAtEL(state, tracer, s_29_1);
        // C s_29_3: const #() : ()
        let s_29_3: () = ();
        // S s_29_4: call VMID_read(s_29_3)
        let s_29_4: u16 = VMID_read(state, tracer, s_29_3);
        // C s_29_5: const #64s : i64
        let s_29_5: i64 = 64;
        // D s_29_6: read-var t2:i
        let s_29_6: i128 = fn_state.t2;
        // D s_29_7: call X_read(s_29_6, s_29_5)
        let s_29_7: Bits = X_read(state, tracer, s_29_6, s_29_5);
        // D s_29_8: cast reint s_29_7 -> u64
        let s_29_8: u64 = (s_29_7.value() as u64);
        // C s_29_9: const #64s : i64
        let s_29_9: i64 = 64;
        // D s_29_10: read-var t:i
        let s_29_10: i128 = fn_state.t;
        // D s_29_11: call X_read(s_29_10, s_29_9)
        let s_29_11: Bits = X_read(state, tracer, s_29_10, s_29_9);
        // D s_29_12: cast reint s_29_11 -> u64
        let s_29_12: u64 = (s_29_11.value() as u64);
        // D s_29_13: cast zx s_29_8 -> bv
        let s_29_13: Bits = Bits::new(s_29_8 as u128, 64u16);
        // D s_29_14: cast zx s_29_12 -> bv
        let s_29_14: Bits = Bits::new(s_29_12 as u128, 64u16);
        // D s_29_15: cast reint s_29_13 -> u128
        let s_29_15: u128 = (s_29_13.value() as u128);
        // D s_29_16: size-of s_29_13
        let s_29_16: u16 = s_29_13.length();
        // D s_29_17: cast reint s_29_14 -> u128
        let s_29_17: u128 = (s_29_14.value() as u128);
        // D s_29_18: size-of s_29_14
        let s_29_18: u16 = s_29_14.length();
        // D s_29_19: lsl s_29_15 s_29_18
        let s_29_19: u128 = s_29_15 << s_29_18;
        // D s_29_20: or s_29_19 s_29_17
        let s_29_20: u128 = ((s_29_19) | (s_29_17));
        // D s_29_21: add s_29_16 s_29_18
        let s_29_21: u16 = (s_29_16 + s_29_18);
        // D s_29_22: create-bits s_29_20 s_29_21
        let s_29_22: Bits = Bits::new(s_29_20, s_29_21);
        // D s_29_23: cast reint s_29_22 -> u128
        let s_29_23: u128 = (s_29_22.value() as u128);
        // C s_29_24: const #4u : u32
        let s_29_24: u32 = 4;
        // C s_29_25: const #2u : u32
        let s_29_25: u32 = 2;
        // C s_29_26: const #0u : u32
        let s_29_26: u32 = 0;
        // C s_29_27: const #1u : u32
        let s_29_27: u32 = 1;
        // D s_29_28: call AArch64_TLBIP_VA(s_29_2, s_29_24, s_29_4, s_29_25, s_29_26, s_29_27, s_29_23)
        let s_29_28: () = AArch64_TLBIP_VA(
            state,
            tracer,
            s_29_2,
            s_29_24,
            s_29_4,
            s_29_25,
            s_29_26,
            s_29_27,
            s_29_23,
        );
        // N s_29_29: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #20u : u8
        let s_30_0: u8 = 20;
        // C s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 8u16);
        // C s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (s_30_1.value() as i128);
        // C s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: cast zx s_30_3 -> i
        let s_30_4: i128 = (i128::try_from(s_30_3).unwrap());
        // C s_30_5: const #432u : u32
        let s_30_5: u32 = 432;
        // D s_30_6: read-reg s_30_5:u8
        let s_30_6: u8 = {
            let value = state.read_register::<u8>(s_30_5 as isize);
            tracer.read_register(s_30_5 as isize, value);
            value
        };
        // D s_30_7: call AArch64_SystemAccessTrap(s_30_6, s_30_4)
        let s_30_7: () = AArch64_SystemAccessTrap(state, tracer, s_30_6, s_30_4);
        // N s_30_8: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var __HFGITR_EL2_TLBIVAE1OS:u8
        let s_31_0: bool = fn_state.u__HFGITR_EL2_TLBIVAE1OS;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#139206 <= s_31_4
        fn_state.gs_139206 = s_31_4;
        // N s_31_6: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call IsHCRXEL2Enabled(s_32_0)
        let s_32_1: bool = IsHCRXEL2Enabled(state, tracer, s_32_0);
        // S s_32_2: not s_32_1
        let s_32_2: bool = !s_32_1;
        // N s_32_3: branch s_32_2 b35 b33
        if s_32_2 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __HCRX_EL2_FGTnXS:u8
        let s_33_0: bool = fn_state.u__HCRX_EL2_FGTnXS;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #0u : u8
        let s_33_2: bool = false;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#139204 <= s_33_4
        fn_state.gs_139204 = s_33_4;
        // N s_33_6: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#139204:u8
        let s_34_0: bool = fn_state.gs_139204;
        // D s_34_1: write-var gs#139205 <= s_34_0
        fn_state.gs_139205 = s_34_0;
        // N s_34_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#139204 <= s_35_0
        fn_state.gs_139204 = s_35_0;
        // N s_35_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #155u : u32
        let s_36_0: u32 = 155;
        // S s_36_1: call IsFeatureImplemented(s_36_0)
        let s_36_1: bool = IsFeatureImplemented(state, tracer, s_36_0);
        // D s_36_2: write-var gs#139203 <= s_36_1
        fn_state.gs_139203 = s_36_1;
        // N s_36_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #424u : u32
        let s_37_0: u32 = 424;
        // D s_37_1: read-reg s_37_0:u8
        let s_37_1: u8 = {
            let value = state.read_register::<u8>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // C s_37_2: const #2u : u8
        let s_37_2: u8 = 2;
        // D s_37_3: cmp-lt s_37_1 s_37_2
        let s_37_3: bool = ((s_37_1) < (s_37_2));
        // D s_37_4: not s_37_3
        let s_37_4: bool = !s_37_3;
        // N s_37_5: branch s_37_4 b40 b38
        if s_37_4 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #90704u : u32
        let s_38_0: u32 = 90704;
        // D s_38_1: read-reg s_38_0:struct
        let s_38_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call _get_SCR_EL3_Type_FGTEn(s_38_1)
        let s_38_2: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_38_1);
        // D s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // C s_38_4: const #1u : u8
        let s_38_4: bool = true;
        // C s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 1u16);
        // D s_38_6: cmp-eq s_38_3 s_38_5
        let s_38_6: bool = ((s_38_3) == (s_38_5));
        // D s_38_7: write-var gs#139201 <= s_38_6
        fn_state.gs_139201 = s_38_6;
        // N s_38_8: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#139201:u8
        let s_39_0: bool = fn_state.gs_139201;
        // D s_39_1: write-var gs#139202 <= s_39_0
        fn_state.gs_139202 = s_39_0;
        // N s_39_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#139201 <= s_40_0
        fn_state.gs_139201 = s_40_0;
        // N s_40_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #146u : u32
        let s_41_0: u32 = 146;
        // S s_41_1: call IsFeatureImplemented(s_41_0)
        let s_41_1: bool = IsFeatureImplemented(state, tracer, s_41_0);
        // D s_41_2: write-var gs#139200 <= s_41_1
        fn_state.gs_139200 = s_41_1;
        // N s_41_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #20u : u8
        let s_42_0: u8 = 20;
        // C s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 8u16);
        // C s_42_2: cast zx s_42_1 -> i
        let s_42_2: i128 = (s_42_1.value() as i128);
        // C s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // C s_42_4: cast zx s_42_3 -> i
        let s_42_4: i128 = (i128::try_from(s_42_3).unwrap());
        // C s_42_5: const #432u : u32
        let s_42_5: u32 = 432;
        // D s_42_6: read-reg s_42_5:u8
        let s_42_6: u8 = {
            let value = state.read_register::<u8>(s_42_5 as isize);
            tracer.read_register(s_42_5 as isize, value);
            value
        };
        // D s_42_7: call AArch64_SystemAccessTrap(s_42_6, s_42_4)
        let s_42_7: () = AArch64_SystemAccessTrap(state, tracer, s_42_6, s_42_4);
        // N s_42_8: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var __HCR_EL2_TTLBOS:u8
        let s_43_0: bool = fn_state.u__HCR_EL2_TTLBOS;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 1u16);
        // C s_43_2: const #1u : u8
        let s_43_2: bool = true;
        // C s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: write-var gs#139199 <= s_43_4
        fn_state.gs_139199 = s_43_4;
        // N s_43_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #20u : u8
        let s_44_0: u8 = 20;
        // C s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 8u16);
        // C s_44_2: cast zx s_44_1 -> i
        let s_44_2: i128 = (s_44_1.value() as i128);
        // C s_44_3: cast reint s_44_2 -> i64
        let s_44_3: i64 = (s_44_2 as i64);
        // C s_44_4: cast zx s_44_3 -> i
        let s_44_4: i128 = (i128::try_from(s_44_3).unwrap());
        // C s_44_5: const #432u : u32
        let s_44_5: u32 = 432;
        // D s_44_6: read-reg s_44_5:u8
        let s_44_6: u8 = {
            let value = state.read_register::<u8>(s_44_5 as isize);
            tracer.read_register(s_44_5 as isize, value);
            value
        };
        // D s_44_7: call AArch64_SystemAccessTrap(s_44_6, s_44_4)
        let s_44_7: () = AArch64_SystemAccessTrap(state, tracer, s_44_6, s_44_4);
        // N s_44_8: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var __HCR_EL2_TTLB:u8
        let s_45_0: bool = fn_state.u__HCR_EL2_TTLB;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 1u16);
        // C s_45_2: const #1u : u8
        let s_45_2: bool = true;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: write-var gs#139198 <= s_45_4
        fn_state.gs_139198 = s_45_4;
        // N s_45_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_46_0: panic
        panic!("{:?}", ());
        // N s_46_1: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_47_0: panic
        panic!("{:?}", ());
        // N s_47_1: return
        return;
    }
}
