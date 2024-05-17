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
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HCR_EL2_Type_E2H::*;
use IsFeatureImplemented::*;
use X_read::*;
use VMID_read::*;
use AArch64_SystemAccessTrap::*;
use u_get_HFGITR_EL2_Type_TLBIRVAE1::*;
use AArch64_TLBIP_RVA::*;
use u_get_HCR_EL2_Type_TTLB::*;
use IsHCRXEL2Enabled::*;
use SecurityStateAtEL::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCRX_EL2_Type_FnXS::*;
use u_get_HCR_EL2_Type_FB::*;
use common::*;
pub fn TLBIP_RVAE1_SysOpsWrite128_f48006a3d9b88992<T: Tracer>(
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
        gs_138598: bool,
        u__HCR_EL2_FB: bool,
        gs_138603: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_138593: bool,
        gs_138596: bool,
        gs_138591: bool,
        u__HFGITR_EL2_TLBIRVAE1: bool,
        gs_138595: bool,
        gs_138592: bool,
        gs_138590: bool,
        gs_138604: bool,
        u__HCRX_EL2_FnXS: bool,
        gs_138605: bool,
        u__PSTATE_EL: u8,
        u__HCR_EL2_TTLB: bool,
        gs_138594: bool,
        gs_138597: bool,
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
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_FGTEn(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_FGTEn <= s_0_9
        fn_state.u__SCR_EL3_FGTEn = s_0_9;
        // C s_0_11: const #13608u : u32
        let s_0_11: u32 = 13608;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HFGITR_EL2_Type_TLBIRVAE1(s_0_12)
        let s_0_13: bool = u_get_HFGITR_EL2_Type_TLBIRVAE1(state, tracer, s_0_12);
        // D s_0_14: write-var __HFGITR_EL2_TLBIRVAE1 <= s_0_13
        fn_state.u__HFGITR_EL2_TLBIRVAE1 = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_FB(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_FB(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_FB <= s_0_17
        fn_state.u__HCR_EL2_FB = s_0_17;
        // C s_0_19: const #22528u : u32
        let s_0_19: u32 = 22528;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCRX_EL2_Type_FnXS(s_0_20)
        let s_0_21: bool = u_get_HCRX_EL2_Type_FnXS(state, tracer, s_0_20);
        // D s_0_22: write-var __HCRX_EL2_FnXS <= s_0_21
        fn_state.u__HCRX_EL2_FnXS = s_0_21;
        // D s_0_23: read-var __PSTATE_EL:u8
        let s_0_23: u8 = fn_state.u__PSTATE_EL;
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 2u16);
        // C s_0_25: const #448u : u32
        let s_0_25: u32 = 448;
        // D s_0_26: read-reg s_0_25:u8
        let s_0_26: u8 = {
            let value = state.read_register::<u8>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 2u16);
        // D s_0_28: cmp-eq s_0_24 s_0_27
        let s_0_28: bool = ((s_0_24) == (s_0_27));
        // N s_0_29: branch s_0_28 b58 b1
        if s_0_28 {
            return block_58(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b11 b2
        if s_1_5 {
            return block_11(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b8 b3
        if s_2_5 {
            return block_8(state, tracer, fn_state);
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
        // C s_5_0: const #102552u : u32
        let s_5_0: u32 = 102552;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_HCR_EL2_Type_E2H(s_5_1)
        let s_5_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_5_1);
        // C s_5_3: const #102552u : u32
        let s_5_3: u32 = 102552;
        // D s_5_4: read-reg s_5_3:struct
        let s_5_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_3 as isize);
            tracer.read_register(s_5_3 as isize, value);
            value
        };
        // D s_5_5: call _get_HCR_EL2_Type_TGE(s_5_4)
        let s_5_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_5_4);
        // D s_5_6: cast zx s_5_2 -> bv
        let s_5_6: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_7: cast zx s_5_5 -> bv
        let s_5_7: Bits = Bits::new(s_5_5 as u128, 1u16);
        // D s_5_8: cast reint s_5_6 -> u128
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
        // D s_5_16: cast reint s_5_15 -> u8
        let s_5_16: u8 = (s_5_15.value() as u8);
        // D s_5_17: cast zx s_5_16 -> bv
        let s_5_17: Bits = Bits::new(s_5_16 as u128, 2u16);
        // C s_5_18: const #3u : u8
        let s_5_18: u8 = 3;
        // C s_5_19: cast zx s_5_18 -> bv
        let s_5_19: Bits = Bits::new(s_5_18 as u128, 2u16);
        // D s_5_20: cmp-eq s_5_17 s_5_19
        let s_5_20: bool = ((s_5_17) == (s_5_19));
        // N s_5_21: branch s_5_20 b7 b6
        if s_5_20 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #440u : u32
        let s_6_0: u32 = 440;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call SecurityStateAtEL(s_6_1)
        let s_6_2: u32 = SecurityStateAtEL(state, tracer, s_6_1);
        // C s_6_3: const #() : ()
        let s_6_3: () = ();
        // S s_6_4: call VMID_read(s_6_3)
        let s_6_4: u16 = VMID_read(state, tracer, s_6_3);
        // C s_6_5: const #64s : i64
        let s_6_5: i64 = 64;
        // D s_6_6: read-var t2:i
        let s_6_6: i128 = fn_state.t2;
        // D s_6_7: call X_read(s_6_6, s_6_5)
        let s_6_7: Bits = X_read(state, tracer, s_6_6, s_6_5);
        // D s_6_8: cast reint s_6_7 -> u64
        let s_6_8: u64 = (s_6_7.value() as u64);
        // C s_6_9: const #64s : i64
        let s_6_9: i64 = 64;
        // D s_6_10: read-var t:i
        let s_6_10: i128 = fn_state.t;
        // D s_6_11: call X_read(s_6_10, s_6_9)
        let s_6_11: Bits = X_read(state, tracer, s_6_10, s_6_9);
        // D s_6_12: cast reint s_6_11 -> u64
        let s_6_12: u64 = (s_6_11.value() as u64);
        // D s_6_13: cast zx s_6_8 -> bv
        let s_6_13: Bits = Bits::new(s_6_8 as u128, 64u16);
        // D s_6_14: cast zx s_6_12 -> bv
        let s_6_14: Bits = Bits::new(s_6_12 as u128, 64u16);
        // D s_6_15: cast reint s_6_13 -> u128
        let s_6_15: u128 = (s_6_13.value() as u128);
        // D s_6_16: size-of s_6_13
        let s_6_16: u16 = s_6_13.length();
        // D s_6_17: cast reint s_6_14 -> u128
        let s_6_17: u128 = (s_6_14.value() as u128);
        // D s_6_18: size-of s_6_14
        let s_6_18: u16 = s_6_14.length();
        // D s_6_19: lsl s_6_15 s_6_18
        let s_6_19: u128 = s_6_15 << s_6_18;
        // D s_6_20: or s_6_19 s_6_17
        let s_6_20: u128 = ((s_6_19) | (s_6_17));
        // D s_6_21: add s_6_16 s_6_18
        let s_6_21: u16 = (s_6_16 + s_6_18);
        // D s_6_22: create-bits s_6_20 s_6_21
        let s_6_22: Bits = Bits::new(s_6_20, s_6_21);
        // D s_6_23: cast reint s_6_22 -> u128
        let s_6_23: u128 = (s_6_22.value() as u128);
        // C s_6_24: const #4u : u32
        let s_6_24: u32 = 4;
        // C s_6_25: const #0u : u32
        let s_6_25: u32 = 0;
        // C s_6_26: const #0u : u32
        let s_6_26: u32 = 0;
        // C s_6_27: const #0u : u32
        let s_6_27: u32 = 0;
        // D s_6_28: call AArch64_TLBIP_RVA(s_6_2, s_6_24, s_6_4, s_6_25, s_6_26, s_6_27, s_6_23)
        let s_6_28: () = AArch64_TLBIP_RVA(
            state,
            tracer,
            s_6_2,
            s_6_24,
            s_6_4,
            s_6_25,
            s_6_26,
            s_6_27,
            s_6_23,
        );
        // N s_6_29: return
        return;
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
        // C s_7_22: const #3u : u32
        let s_7_22: u32 = 3;
        // C s_7_23: const #1080u : u32
        let s_7_23: u32 = 1080;
        // D s_7_24: read-reg s_7_23:u16
        let s_7_24: u16 = {
            let value = state.read_register::<u16>(s_7_23 as isize);
            tracer.read_register(s_7_23 as isize, value);
            value
        };
        // C s_7_25: const #0u : u32
        let s_7_25: u32 = 0;
        // C s_7_26: const #0u : u32
        let s_7_26: u32 = 0;
        // C s_7_27: const #0u : u32
        let s_7_27: u32 = 0;
        // D s_7_28: call AArch64_TLBIP_RVA(s_7_2, s_7_22, s_7_24, s_7_25, s_7_26, s_7_27, s_7_21)
        let s_7_28: () = AArch64_TLBIP_RVA(
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
        // C s_8_0: const #102552u : u32
        let s_8_0: u32 = 102552;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_HCR_EL2_Type_E2H(s_8_1)
        let s_8_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_8_1);
        // C s_8_3: const #102552u : u32
        let s_8_3: u32 = 102552;
        // D s_8_4: read-reg s_8_3:struct
        let s_8_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_3 as isize);
            tracer.read_register(s_8_3 as isize, value);
            value
        };
        // D s_8_5: call _get_HCR_EL2_Type_TGE(s_8_4)
        let s_8_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_8_4);
        // D s_8_6: cast zx s_8_2 -> bv
        let s_8_6: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_7: cast zx s_8_5 -> bv
        let s_8_7: Bits = Bits::new(s_8_5 as u128, 1u16);
        // D s_8_8: cast reint s_8_6 -> u128
        let s_8_8: u128 = (s_8_6.value() as u128);
        // D s_8_9: size-of s_8_6
        let s_8_9: u16 = s_8_6.length();
        // D s_8_10: cast reint s_8_7 -> u128
        let s_8_10: u128 = (s_8_7.value() as u128);
        // D s_8_11: size-of s_8_7
        let s_8_11: u16 = s_8_7.length();
        // D s_8_12: lsl s_8_8 s_8_11
        let s_8_12: u128 = s_8_8 << s_8_11;
        // D s_8_13: or s_8_12 s_8_10
        let s_8_13: u128 = ((s_8_12) | (s_8_10));
        // D s_8_14: add s_8_9 s_8_11
        let s_8_14: u16 = (s_8_9 + s_8_11);
        // D s_8_15: create-bits s_8_13 s_8_14
        let s_8_15: Bits = Bits::new(s_8_13, s_8_14);
        // D s_8_16: cast reint s_8_15 -> u8
        let s_8_16: u8 = (s_8_15.value() as u8);
        // D s_8_17: cast zx s_8_16 -> bv
        let s_8_17: Bits = Bits::new(s_8_16 as u128, 2u16);
        // C s_8_18: const #3u : u8
        let s_8_18: u8 = 3;
        // C s_8_19: cast zx s_8_18 -> bv
        let s_8_19: Bits = Bits::new(s_8_18 as u128, 2u16);
        // D s_8_20: cmp-eq s_8_17 s_8_19
        let s_8_20: bool = ((s_8_17) == (s_8_19));
        // N s_8_21: branch s_8_20 b10 b9
        if s_8_20 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #440u : u32
        let s_9_0: u32 = 440;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call SecurityStateAtEL(s_9_1)
        let s_9_2: u32 = SecurityStateAtEL(state, tracer, s_9_1);
        // C s_9_3: const #() : ()
        let s_9_3: () = ();
        // S s_9_4: call VMID_read(s_9_3)
        let s_9_4: u16 = VMID_read(state, tracer, s_9_3);
        // C s_9_5: const #64s : i64
        let s_9_5: i64 = 64;
        // D s_9_6: read-var t2:i
        let s_9_6: i128 = fn_state.t2;
        // D s_9_7: call X_read(s_9_6, s_9_5)
        let s_9_7: Bits = X_read(state, tracer, s_9_6, s_9_5);
        // D s_9_8: cast reint s_9_7 -> u64
        let s_9_8: u64 = (s_9_7.value() as u64);
        // C s_9_9: const #64s : i64
        let s_9_9: i64 = 64;
        // D s_9_10: read-var t:i
        let s_9_10: i128 = fn_state.t;
        // D s_9_11: call X_read(s_9_10, s_9_9)
        let s_9_11: Bits = X_read(state, tracer, s_9_10, s_9_9);
        // D s_9_12: cast reint s_9_11 -> u64
        let s_9_12: u64 = (s_9_11.value() as u64);
        // D s_9_13: cast zx s_9_8 -> bv
        let s_9_13: Bits = Bits::new(s_9_8 as u128, 64u16);
        // D s_9_14: cast zx s_9_12 -> bv
        let s_9_14: Bits = Bits::new(s_9_12 as u128, 64u16);
        // D s_9_15: cast reint s_9_13 -> u128
        let s_9_15: u128 = (s_9_13.value() as u128);
        // D s_9_16: size-of s_9_13
        let s_9_16: u16 = s_9_13.length();
        // D s_9_17: cast reint s_9_14 -> u128
        let s_9_17: u128 = (s_9_14.value() as u128);
        // D s_9_18: size-of s_9_14
        let s_9_18: u16 = s_9_14.length();
        // D s_9_19: lsl s_9_15 s_9_18
        let s_9_19: u128 = s_9_15 << s_9_18;
        // D s_9_20: or s_9_19 s_9_17
        let s_9_20: u128 = ((s_9_19) | (s_9_17));
        // D s_9_21: add s_9_16 s_9_18
        let s_9_21: u16 = (s_9_16 + s_9_18);
        // D s_9_22: create-bits s_9_20 s_9_21
        let s_9_22: Bits = Bits::new(s_9_20, s_9_21);
        // D s_9_23: cast reint s_9_22 -> u128
        let s_9_23: u128 = (s_9_22.value() as u128);
        // C s_9_24: const #4u : u32
        let s_9_24: u32 = 4;
        // C s_9_25: const #0u : u32
        let s_9_25: u32 = 0;
        // C s_9_26: const #0u : u32
        let s_9_26: u32 = 0;
        // C s_9_27: const #0u : u32
        let s_9_27: u32 = 0;
        // D s_9_28: call AArch64_TLBIP_RVA(s_9_2, s_9_24, s_9_4, s_9_25, s_9_26, s_9_27, s_9_23)
        let s_9_28: () = AArch64_TLBIP_RVA(
            state,
            tracer,
            s_9_2,
            s_9_24,
            s_9_4,
            s_9_25,
            s_9_26,
            s_9_27,
            s_9_23,
        );
        // N s_9_29: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #432u : u32
        let s_10_0: u32 = 432;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: u8 = {
            let value = state.read_register::<u8>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call SecurityStateAtEL(s_10_1)
        let s_10_2: u32 = SecurityStateAtEL(state, tracer, s_10_1);
        // C s_10_3: const #64s : i64
        let s_10_3: i64 = 64;
        // D s_10_4: read-var t2:i
        let s_10_4: i128 = fn_state.t2;
        // D s_10_5: call X_read(s_10_4, s_10_3)
        let s_10_5: Bits = X_read(state, tracer, s_10_4, s_10_3);
        // D s_10_6: cast reint s_10_5 -> u64
        let s_10_6: u64 = (s_10_5.value() as u64);
        // C s_10_7: const #64s : i64
        let s_10_7: i64 = 64;
        // D s_10_8: read-var t:i
        let s_10_8: i128 = fn_state.t;
        // D s_10_9: call X_read(s_10_8, s_10_7)
        let s_10_9: Bits = X_read(state, tracer, s_10_8, s_10_7);
        // D s_10_10: cast reint s_10_9 -> u64
        let s_10_10: u64 = (s_10_9.value() as u64);
        // D s_10_11: cast zx s_10_6 -> bv
        let s_10_11: Bits = Bits::new(s_10_6 as u128, 64u16);
        // D s_10_12: cast zx s_10_10 -> bv
        let s_10_12: Bits = Bits::new(s_10_10 as u128, 64u16);
        // D s_10_13: cast reint s_10_11 -> u128
        let s_10_13: u128 = (s_10_11.value() as u128);
        // D s_10_14: size-of s_10_11
        let s_10_14: u16 = s_10_11.length();
        // D s_10_15: cast reint s_10_12 -> u128
        let s_10_15: u128 = (s_10_12.value() as u128);
        // D s_10_16: size-of s_10_12
        let s_10_16: u16 = s_10_12.length();
        // D s_10_17: lsl s_10_13 s_10_16
        let s_10_17: u128 = s_10_13 << s_10_16;
        // D s_10_18: or s_10_17 s_10_15
        let s_10_18: u128 = ((s_10_17) | (s_10_15));
        // D s_10_19: add s_10_14 s_10_16
        let s_10_19: u16 = (s_10_14 + s_10_16);
        // D s_10_20: create-bits s_10_18 s_10_19
        let s_10_20: Bits = Bits::new(s_10_18, s_10_19);
        // D s_10_21: cast reint s_10_20 -> u128
        let s_10_21: u128 = (s_10_20.value() as u128);
        // C s_10_22: const #3u : u32
        let s_10_22: u32 = 3;
        // C s_10_23: const #1080u : u32
        let s_10_23: u32 = 1080;
        // D s_10_24: read-reg s_10_23:u16
        let s_10_24: u16 = {
            let value = state.read_register::<u16>(s_10_23 as isize);
            tracer.read_register(s_10_23 as isize, value);
            value
        };
        // C s_10_25: const #0u : u32
        let s_10_25: u32 = 0;
        // C s_10_26: const #0u : u32
        let s_10_26: u32 = 0;
        // C s_10_27: const #0u : u32
        let s_10_27: u32 = 0;
        // D s_10_28: call AArch64_TLBIP_RVA(s_10_2, s_10_22, s_10_24, s_10_25, s_10_26, s_10_27, s_10_21)
        let s_10_28: () = AArch64_TLBIP_RVA(
            state,
            tracer,
            s_10_2,
            s_10_22,
            s_10_24,
            s_10_25,
            s_10_26,
            s_10_27,
            s_10_21,
        );
        // N s_10_29: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call EL2Enabled(s_11_0)
        let s_11_1: bool = EL2Enabled(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b57 b12
        if s_11_1 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#138590 <= s_12_0
        fn_state.gs_138590 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#138590:u8
        let s_13_0: bool = fn_state.gs_138590;
        // N s_13_1: branch s_13_0 b56 b14
        if s_13_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
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
        // N s_14_2: branch s_14_1 b55 b15
        if s_14_1 {
            return block_55(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#138591 <= s_15_0
        fn_state.gs_138591 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#138591:u8
        let s_16_0: bool = fn_state.gs_138591;
        // N s_16_1: branch s_16_0 b51 b17
        if s_16_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#138593 <= s_17_0
        fn_state.gs_138593 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#138593:u8
        let s_18_0: bool = fn_state.gs_138593;
        // N s_18_1: branch s_18_0 b50 b19
        if s_18_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#138594 <= s_19_0
        fn_state.gs_138594 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#138594:u8
        let s_20_0: bool = fn_state.gs_138594;
        // N s_20_1: branch s_20_0 b49 b21
        if s_20_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call EL2Enabled(s_21_0)
        let s_21_1: bool = EL2Enabled(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b48 b22
        if s_21_1 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#138595 <= s_22_0
        fn_state.gs_138595 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#138595:u8
        let s_23_0: bool = fn_state.gs_138595;
        // N s_23_1: branch s_23_0 b36 b24
        if s_23_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #166u : u32
        let s_24_0: u32 = 166;
        // S s_24_1: call IsFeatureImplemented(s_24_0)
        let s_24_1: bool = IsFeatureImplemented(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b35 b25
        if s_24_1 {
            return block_35(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#138596 <= s_25_0
        fn_state.gs_138596 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#138596:u8
        let s_26_0: bool = fn_state.gs_138596;
        // N s_26_1: branch s_26_0 b34 b27
        if s_26_0 {
            return block_34(state, tracer, fn_state);
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
        // D s_27_1: write-var gs#138597 <= s_27_0
        fn_state.gs_138597 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#138597:u8
        let s_28_0: bool = fn_state.gs_138597;
        // N s_28_1: branch s_28_0 b33 b29
        if s_28_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#138598 <= s_29_0
        fn_state.gs_138598 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#138598:u8
        let s_30_0: bool = fn_state.gs_138598;
        // N s_30_1: branch s_30_0 b32 b31
        if s_30_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #440u : u32
        let s_31_0: u32 = 440;
        // D s_31_1: read-reg s_31_0:u8
        let s_31_1: u8 = {
            let value = state.read_register::<u8>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call SecurityStateAtEL(s_31_1)
        let s_31_2: u32 = SecurityStateAtEL(state, tracer, s_31_1);
        // C s_31_3: const #() : ()
        let s_31_3: () = ();
        // S s_31_4: call VMID_read(s_31_3)
        let s_31_4: u16 = VMID_read(state, tracer, s_31_3);
        // C s_31_5: const #64s : i64
        let s_31_5: i64 = 64;
        // D s_31_6: read-var t2:i
        let s_31_6: i128 = fn_state.t2;
        // D s_31_7: call X_read(s_31_6, s_31_5)
        let s_31_7: Bits = X_read(state, tracer, s_31_6, s_31_5);
        // D s_31_8: cast reint s_31_7 -> u64
        let s_31_8: u64 = (s_31_7.value() as u64);
        // C s_31_9: const #64s : i64
        let s_31_9: i64 = 64;
        // D s_31_10: read-var t:i
        let s_31_10: i128 = fn_state.t;
        // D s_31_11: call X_read(s_31_10, s_31_9)
        let s_31_11: Bits = X_read(state, tracer, s_31_10, s_31_9);
        // D s_31_12: cast reint s_31_11 -> u64
        let s_31_12: u64 = (s_31_11.value() as u64);
        // D s_31_13: cast zx s_31_8 -> bv
        let s_31_13: Bits = Bits::new(s_31_8 as u128, 64u16);
        // D s_31_14: cast zx s_31_12 -> bv
        let s_31_14: Bits = Bits::new(s_31_12 as u128, 64u16);
        // D s_31_15: cast reint s_31_13 -> u128
        let s_31_15: u128 = (s_31_13.value() as u128);
        // D s_31_16: size-of s_31_13
        let s_31_16: u16 = s_31_13.length();
        // D s_31_17: cast reint s_31_14 -> u128
        let s_31_17: u128 = (s_31_14.value() as u128);
        // D s_31_18: size-of s_31_14
        let s_31_18: u16 = s_31_14.length();
        // D s_31_19: lsl s_31_15 s_31_18
        let s_31_19: u128 = s_31_15 << s_31_18;
        // D s_31_20: or s_31_19 s_31_17
        let s_31_20: u128 = ((s_31_19) | (s_31_17));
        // D s_31_21: add s_31_16 s_31_18
        let s_31_21: u16 = (s_31_16 + s_31_18);
        // D s_31_22: create-bits s_31_20 s_31_21
        let s_31_22: Bits = Bits::new(s_31_20, s_31_21);
        // D s_31_23: cast reint s_31_22 -> u128
        let s_31_23: u128 = (s_31_22.value() as u128);
        // C s_31_24: const #4u : u32
        let s_31_24: u32 = 4;
        // C s_31_25: const #0u : u32
        let s_31_25: u32 = 0;
        // C s_31_26: const #0u : u32
        let s_31_26: u32 = 0;
        // C s_31_27: const #0u : u32
        let s_31_27: u32 = 0;
        // D s_31_28: call AArch64_TLBIP_RVA(s_31_2, s_31_24, s_31_4, s_31_25, s_31_26, s_31_27, s_31_23)
        let s_31_28: () = AArch64_TLBIP_RVA(
            state,
            tracer,
            s_31_2,
            s_31_24,
            s_31_4,
            s_31_25,
            s_31_26,
            s_31_27,
            s_31_23,
        );
        // N s_31_29: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #440u : u32
        let s_32_0: u32 = 440;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: call SecurityStateAtEL(s_32_1)
        let s_32_2: u32 = SecurityStateAtEL(state, tracer, s_32_1);
        // C s_32_3: const #() : ()
        let s_32_3: () = ();
        // S s_32_4: call VMID_read(s_32_3)
        let s_32_4: u16 = VMID_read(state, tracer, s_32_3);
        // C s_32_5: const #64s : i64
        let s_32_5: i64 = 64;
        // D s_32_6: read-var t2:i
        let s_32_6: i128 = fn_state.t2;
        // D s_32_7: call X_read(s_32_6, s_32_5)
        let s_32_7: Bits = X_read(state, tracer, s_32_6, s_32_5);
        // D s_32_8: cast reint s_32_7 -> u64
        let s_32_8: u64 = (s_32_7.value() as u64);
        // C s_32_9: const #64s : i64
        let s_32_9: i64 = 64;
        // D s_32_10: read-var t:i
        let s_32_10: i128 = fn_state.t;
        // D s_32_11: call X_read(s_32_10, s_32_9)
        let s_32_11: Bits = X_read(state, tracer, s_32_10, s_32_9);
        // D s_32_12: cast reint s_32_11 -> u64
        let s_32_12: u64 = (s_32_11.value() as u64);
        // D s_32_13: cast zx s_32_8 -> bv
        let s_32_13: Bits = Bits::new(s_32_8 as u128, 64u16);
        // D s_32_14: cast zx s_32_12 -> bv
        let s_32_14: Bits = Bits::new(s_32_12 as u128, 64u16);
        // D s_32_15: cast reint s_32_13 -> u128
        let s_32_15: u128 = (s_32_13.value() as u128);
        // D s_32_16: size-of s_32_13
        let s_32_16: u16 = s_32_13.length();
        // D s_32_17: cast reint s_32_14 -> u128
        let s_32_17: u128 = (s_32_14.value() as u128);
        // D s_32_18: size-of s_32_14
        let s_32_18: u16 = s_32_14.length();
        // D s_32_19: lsl s_32_15 s_32_18
        let s_32_19: u128 = s_32_15 << s_32_18;
        // D s_32_20: or s_32_19 s_32_17
        let s_32_20: u128 = ((s_32_19) | (s_32_17));
        // D s_32_21: add s_32_16 s_32_18
        let s_32_21: u16 = (s_32_16 + s_32_18);
        // D s_32_22: create-bits s_32_20 s_32_21
        let s_32_22: Bits = Bits::new(s_32_20, s_32_21);
        // D s_32_23: cast reint s_32_22 -> u128
        let s_32_23: u128 = (s_32_22.value() as u128);
        // C s_32_24: const #4u : u32
        let s_32_24: u32 = 4;
        // C s_32_25: const #0u : u32
        let s_32_25: u32 = 0;
        // C s_32_26: const #0u : u32
        let s_32_26: u32 = 0;
        // C s_32_27: const #1u : u32
        let s_32_27: u32 = 1;
        // D s_32_28: call AArch64_TLBIP_RVA(s_32_2, s_32_24, s_32_4, s_32_25, s_32_26, s_32_27, s_32_23)
        let s_32_28: () = AArch64_TLBIP_RVA(
            state,
            tracer,
            s_32_2,
            s_32_24,
            s_32_4,
            s_32_25,
            s_32_26,
            s_32_27,
            s_32_23,
        );
        // N s_32_29: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __HCRX_EL2_FnXS:u8
        let s_33_0: bool = fn_state.u__HCRX_EL2_FnXS;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#138598 <= s_33_4
        fn_state.gs_138598 = s_33_4;
        // N s_33_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call IsHCRXEL2Enabled(s_34_0)
        let s_34_1: bool = IsHCRXEL2Enabled(state, tracer, s_34_0);
        // D s_34_2: write-var gs#138597 <= s_34_1
        fn_state.gs_138597 = s_34_1;
        // N s_34_3: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #155u : u32
        let s_35_0: u32 = 155;
        // S s_35_1: call IsFeatureImplemented(s_35_0)
        let s_35_1: bool = IsFeatureImplemented(state, tracer, s_35_0);
        // D s_35_2: write-var gs#138596 <= s_35_1
        fn_state.gs_138596 = s_35_1;
        // N s_35_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #166u : u32
        let s_36_0: u32 = 166;
        // S s_36_1: call IsFeatureImplemented(s_36_0)
        let s_36_1: bool = IsFeatureImplemented(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b47 b37
        if s_36_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#138603 <= s_37_0
        fn_state.gs_138603 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#138603:u8
        let s_38_0: bool = fn_state.gs_138603;
        // N s_38_1: branch s_38_0 b46 b39
        if s_38_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#138604 <= s_39_0
        fn_state.gs_138604 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#138604:u8
        let s_40_0: bool = fn_state.gs_138604;
        // N s_40_1: branch s_40_0 b45 b41
        if s_40_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#138605 <= s_41_0
        fn_state.gs_138605 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#138605:u8
        let s_42_0: bool = fn_state.gs_138605;
        // N s_42_1: branch s_42_0 b44 b43
        if s_42_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #440u : u32
        let s_43_0: u32 = 440;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call SecurityStateAtEL(s_43_1)
        let s_43_2: u32 = SecurityStateAtEL(state, tracer, s_43_1);
        // C s_43_3: const #() : ()
        let s_43_3: () = ();
        // S s_43_4: call VMID_read(s_43_3)
        let s_43_4: u16 = VMID_read(state, tracer, s_43_3);
        // C s_43_5: const #64s : i64
        let s_43_5: i64 = 64;
        // D s_43_6: read-var t2:i
        let s_43_6: i128 = fn_state.t2;
        // D s_43_7: call X_read(s_43_6, s_43_5)
        let s_43_7: Bits = X_read(state, tracer, s_43_6, s_43_5);
        // D s_43_8: cast reint s_43_7 -> u64
        let s_43_8: u64 = (s_43_7.value() as u64);
        // C s_43_9: const #64s : i64
        let s_43_9: i64 = 64;
        // D s_43_10: read-var t:i
        let s_43_10: i128 = fn_state.t;
        // D s_43_11: call X_read(s_43_10, s_43_9)
        let s_43_11: Bits = X_read(state, tracer, s_43_10, s_43_9);
        // D s_43_12: cast reint s_43_11 -> u64
        let s_43_12: u64 = (s_43_11.value() as u64);
        // D s_43_13: cast zx s_43_8 -> bv
        let s_43_13: Bits = Bits::new(s_43_8 as u128, 64u16);
        // D s_43_14: cast zx s_43_12 -> bv
        let s_43_14: Bits = Bits::new(s_43_12 as u128, 64u16);
        // D s_43_15: cast reint s_43_13 -> u128
        let s_43_15: u128 = (s_43_13.value() as u128);
        // D s_43_16: size-of s_43_13
        let s_43_16: u16 = s_43_13.length();
        // D s_43_17: cast reint s_43_14 -> u128
        let s_43_17: u128 = (s_43_14.value() as u128);
        // D s_43_18: size-of s_43_14
        let s_43_18: u16 = s_43_14.length();
        // D s_43_19: lsl s_43_15 s_43_18
        let s_43_19: u128 = s_43_15 << s_43_18;
        // D s_43_20: or s_43_19 s_43_17
        let s_43_20: u128 = ((s_43_19) | (s_43_17));
        // D s_43_21: add s_43_16 s_43_18
        let s_43_21: u16 = (s_43_16 + s_43_18);
        // D s_43_22: create-bits s_43_20 s_43_21
        let s_43_22: Bits = Bits::new(s_43_20, s_43_21);
        // D s_43_23: cast reint s_43_22 -> u128
        let s_43_23: u128 = (s_43_22.value() as u128);
        // C s_43_24: const #4u : u32
        let s_43_24: u32 = 4;
        // C s_43_25: const #1u : u32
        let s_43_25: u32 = 1;
        // C s_43_26: const #0u : u32
        let s_43_26: u32 = 0;
        // C s_43_27: const #0u : u32
        let s_43_27: u32 = 0;
        // D s_43_28: call AArch64_TLBIP_RVA(s_43_2, s_43_24, s_43_4, s_43_25, s_43_26, s_43_27, s_43_23)
        let s_43_28: () = AArch64_TLBIP_RVA(
            state,
            tracer,
            s_43_2,
            s_43_24,
            s_43_4,
            s_43_25,
            s_43_26,
            s_43_27,
            s_43_23,
        );
        // N s_43_29: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #440u : u32
        let s_44_0: u32 = 440;
        // D s_44_1: read-reg s_44_0:u8
        let s_44_1: u8 = {
            let value = state.read_register::<u8>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call SecurityStateAtEL(s_44_1)
        let s_44_2: u32 = SecurityStateAtEL(state, tracer, s_44_1);
        // C s_44_3: const #() : ()
        let s_44_3: () = ();
        // S s_44_4: call VMID_read(s_44_3)
        let s_44_4: u16 = VMID_read(state, tracer, s_44_3);
        // C s_44_5: const #64s : i64
        let s_44_5: i64 = 64;
        // D s_44_6: read-var t2:i
        let s_44_6: i128 = fn_state.t2;
        // D s_44_7: call X_read(s_44_6, s_44_5)
        let s_44_7: Bits = X_read(state, tracer, s_44_6, s_44_5);
        // D s_44_8: cast reint s_44_7 -> u64
        let s_44_8: u64 = (s_44_7.value() as u64);
        // C s_44_9: const #64s : i64
        let s_44_9: i64 = 64;
        // D s_44_10: read-var t:i
        let s_44_10: i128 = fn_state.t;
        // D s_44_11: call X_read(s_44_10, s_44_9)
        let s_44_11: Bits = X_read(state, tracer, s_44_10, s_44_9);
        // D s_44_12: cast reint s_44_11 -> u64
        let s_44_12: u64 = (s_44_11.value() as u64);
        // D s_44_13: cast zx s_44_8 -> bv
        let s_44_13: Bits = Bits::new(s_44_8 as u128, 64u16);
        // D s_44_14: cast zx s_44_12 -> bv
        let s_44_14: Bits = Bits::new(s_44_12 as u128, 64u16);
        // D s_44_15: cast reint s_44_13 -> u128
        let s_44_15: u128 = (s_44_13.value() as u128);
        // D s_44_16: size-of s_44_13
        let s_44_16: u16 = s_44_13.length();
        // D s_44_17: cast reint s_44_14 -> u128
        let s_44_17: u128 = (s_44_14.value() as u128);
        // D s_44_18: size-of s_44_14
        let s_44_18: u16 = s_44_14.length();
        // D s_44_19: lsl s_44_15 s_44_18
        let s_44_19: u128 = s_44_15 << s_44_18;
        // D s_44_20: or s_44_19 s_44_17
        let s_44_20: u128 = ((s_44_19) | (s_44_17));
        // D s_44_21: add s_44_16 s_44_18
        let s_44_21: u16 = (s_44_16 + s_44_18);
        // D s_44_22: create-bits s_44_20 s_44_21
        let s_44_22: Bits = Bits::new(s_44_20, s_44_21);
        // D s_44_23: cast reint s_44_22 -> u128
        let s_44_23: u128 = (s_44_22.value() as u128);
        // C s_44_24: const #4u : u32
        let s_44_24: u32 = 4;
        // C s_44_25: const #1u : u32
        let s_44_25: u32 = 1;
        // C s_44_26: const #0u : u32
        let s_44_26: u32 = 0;
        // C s_44_27: const #1u : u32
        let s_44_27: u32 = 1;
        // D s_44_28: call AArch64_TLBIP_RVA(s_44_2, s_44_24, s_44_4, s_44_25, s_44_26, s_44_27, s_44_23)
        let s_44_28: () = AArch64_TLBIP_RVA(
            state,
            tracer,
            s_44_2,
            s_44_24,
            s_44_4,
            s_44_25,
            s_44_26,
            s_44_27,
            s_44_23,
        );
        // N s_44_29: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var __HCRX_EL2_FnXS:u8
        let s_45_0: bool = fn_state.u__HCRX_EL2_FnXS;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 1u16);
        // C s_45_2: const #1u : u8
        let s_45_2: bool = true;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: write-var gs#138605 <= s_45_4
        fn_state.gs_138605 = s_45_4;
        // N s_45_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call IsHCRXEL2Enabled(s_46_0)
        let s_46_1: bool = IsHCRXEL2Enabled(state, tracer, s_46_0);
        // D s_46_2: write-var gs#138604 <= s_46_1
        fn_state.gs_138604 = s_46_1;
        // N s_46_3: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #155u : u32
        let s_47_0: u32 = 155;
        // S s_47_1: call IsFeatureImplemented(s_47_0)
        let s_47_1: bool = IsFeatureImplemented(state, tracer, s_47_0);
        // D s_47_2: write-var gs#138603 <= s_47_1
        fn_state.gs_138603 = s_47_1;
        // N s_47_3: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __HCR_EL2_FB:u8
        let s_48_0: bool = fn_state.u__HCR_EL2_FB;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #1u : u8
        let s_48_2: bool = true;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#138595 <= s_48_4
        fn_state.gs_138595 = s_48_4;
        // N s_48_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #20u : u8
        let s_49_0: u8 = 20;
        // C s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 8u16);
        // C s_49_2: cast zx s_49_1 -> i
        let s_49_2: i128 = (s_49_1.value() as i128);
        // C s_49_3: cast reint s_49_2 -> i64
        let s_49_3: i64 = (s_49_2 as i64);
        // C s_49_4: cast zx s_49_3 -> i
        let s_49_4: i128 = (i128::try_from(s_49_3).unwrap());
        // C s_49_5: const #432u : u32
        let s_49_5: u32 = 432;
        // D s_49_6: read-reg s_49_5:u8
        let s_49_6: u8 = {
            let value = state.read_register::<u8>(s_49_5 as isize);
            tracer.read_register(s_49_5 as isize, value);
            value
        };
        // D s_49_7: call AArch64_SystemAccessTrap(s_49_6, s_49_4)
        let s_49_7: () = AArch64_SystemAccessTrap(state, tracer, s_49_6, s_49_4);
        // N s_49_8: return
        return;
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var __HFGITR_EL2_TLBIRVAE1:u8
        let s_50_0: bool = fn_state.u__HFGITR_EL2_TLBIRVAE1;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 1u16);
        // C s_50_2: const #1u : u8
        let s_50_2: bool = true;
        // C s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // D s_50_4: cmp-eq s_50_1 s_50_3
        let s_50_4: bool = ((s_50_1) == (s_50_3));
        // D s_50_5: write-var gs#138594 <= s_50_4
        fn_state.gs_138594 = s_50_4;
        // N s_50_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #424u : u32
        let s_51_0: u32 = 424;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: u8 = {
            let value = state.read_register::<u8>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // C s_51_2: const #2u : u8
        let s_51_2: u8 = 2;
        // D s_51_3: cmp-lt s_51_1 s_51_2
        let s_51_3: bool = ((s_51_1) < (s_51_2));
        // D s_51_4: not s_51_3
        let s_51_4: bool = !s_51_3;
        // N s_51_5: branch s_51_4 b54 b52
        if s_51_4 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var __SCR_EL3_FGTEn:u8
        let s_52_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 1u16);
        // C s_52_2: const #1u : u8
        let s_52_2: bool = true;
        // C s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // D s_52_5: write-var gs#138592 <= s_52_4
        fn_state.gs_138592 = s_52_4;
        // N s_52_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#138592:u8
        let s_53_0: bool = fn_state.gs_138592;
        // D s_53_1: write-var gs#138593 <= s_53_0
        fn_state.gs_138593 = s_53_0;
        // N s_53_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#138592 <= s_54_0
        fn_state.gs_138592 = s_54_0;
        // N s_54_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #146u : u32
        let s_55_0: u32 = 146;
        // S s_55_1: call IsFeatureImplemented(s_55_0)
        let s_55_1: bool = IsFeatureImplemented(state, tracer, s_55_0);
        // D s_55_2: write-var gs#138591 <= s_55_1
        fn_state.gs_138591 = s_55_1;
        // N s_55_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #20u : u8
        let s_56_0: u8 = 20;
        // C s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 8u16);
        // C s_56_2: cast zx s_56_1 -> i
        let s_56_2: i128 = (s_56_1.value() as i128);
        // C s_56_3: cast reint s_56_2 -> i64
        let s_56_3: i64 = (s_56_2 as i64);
        // C s_56_4: cast zx s_56_3 -> i
        let s_56_4: i128 = (i128::try_from(s_56_3).unwrap());
        // C s_56_5: const #432u : u32
        let s_56_5: u32 = 432;
        // D s_56_6: read-reg s_56_5:u8
        let s_56_6: u8 = {
            let value = state.read_register::<u8>(s_56_5 as isize);
            tracer.read_register(s_56_5 as isize, value);
            value
        };
        // D s_56_7: call AArch64_SystemAccessTrap(s_56_6, s_56_4)
        let s_56_7: () = AArch64_SystemAccessTrap(state, tracer, s_56_6, s_56_4);
        // N s_56_8: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var __HCR_EL2_TTLB:u8
        let s_57_0: bool = fn_state.u__HCR_EL2_TTLB;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 1u16);
        // C s_57_2: const #1u : u8
        let s_57_2: bool = true;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // D s_57_4: cmp-eq s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) == (s_57_3));
        // D s_57_5: write-var gs#138590 <= s_57_4
        fn_state.gs_138590 = s_57_4;
        // N s_57_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_58_0: panic
        panic!("{:?}", ());
        // N s_58_1: return
        return;
    }
}
