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
use u_get_HCR_EL2_Type_NV2::*;
use u_get_HCR_EL2_Type_E2H::*;
use Mk_CNTHPS_CVAL_EL2_Type::*;
use u_get_CNTHCTL_EL2_Type_EL1PTEN::*;
use IsFeatureImplemented::*;
use X_read::*;
use NVMem_set::*;
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_HCR_EL2_Type_NV1::*;
use u_get_CNTHCTL_EL2_Type_EL1PCEN::*;
use u_get_CNTHCTL_EL2_Type_EL0PTEN::*;
use Mk_CNTP_CVAL_EL0_Type::*;
use u_get_HCR_EL2_Type_NV::*;
use EL2Enabled::*;
use u_get_CNTKCTL_EL1_Type_EL0PTEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use Mk_CNTHP_CVAL_EL2_Type::*;
use common::*;
pub fn CNTHP_CVAL_EL2_SysRegWrite_ead9355a45d8e73b<T: Tracer>(
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
        gs_81634: bool,
        gs_81616: bool,
        gs_81626: bool,
        gs_81635: bool,
        gs_81618: bool,
        gs_81625: bool,
        gs_81610: bool,
        gs_81612: bool,
        gs_81630: bool,
        gs_81619: bool,
        gs_81627: bool,
        gs_81633: bool,
        gs_81624: bool,
        u__PSTATE_EL: u8,
        u__CNTHCTL_EL2_EL1PCEN: bool,
        gs_81631: bool,
        gs_81628: bool,
        u__HCR_EL2_TGE: bool,
        gs_81620: bool,
        gs_81623: bool,
        u__CNTHCTL_EL2_EL0PTEN: bool,
        gs_81632: bool,
        u__CNTKCTL_EL1_EL0PTEN: bool,
        gs_81629: bool,
        gs_81611: bool,
        gs_81639: bool,
        u__CNTHCTL_EL2_EL1PTEN: bool,
        gs_81617: bool,
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
        // C s_0_3: const #22056u : u32
        let s_0_3: u32 = 22056;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_CNTKCTL_EL1_Type_EL0PTEN(s_0_4)
        let s_0_5: bool = u_get_CNTKCTL_EL1_Type_EL0PTEN(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTKCTL_EL1_EL0PTEN <= s_0_5
        fn_state.u__CNTKCTL_EL1_EL0PTEN = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_TGE(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_TGE <= s_0_9
        fn_state.u__HCR_EL2_TGE = s_0_9;
        // C s_0_11: const #12808u : u32
        let s_0_11: u32 = 12808;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_CNTHCTL_EL2_Type_EL1PCEN(s_0_12)
        let s_0_13: bool = u_get_CNTHCTL_EL2_Type_EL1PCEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTHCTL_EL2_EL1PCEN <= s_0_13
        fn_state.u__CNTHCTL_EL2_EL1PCEN = s_0_13;
        // C s_0_15: const #12808u : u32
        let s_0_15: u32 = 12808;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_CNTHCTL_EL2_Type_EL1PTEN(s_0_16)
        let s_0_17: bool = u_get_CNTHCTL_EL2_Type_EL1PTEN(state, tracer, s_0_16);
        // D s_0_18: write-var __CNTHCTL_EL2_EL1PTEN <= s_0_17
        fn_state.u__CNTHCTL_EL2_EL1PTEN = s_0_17;
        // C s_0_19: const #12808u : u32
        let s_0_19: u32 = 12808;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_CNTHCTL_EL2_Type_EL0PTEN(s_0_20)
        let s_0_21: bool = u_get_CNTHCTL_EL2_Type_EL0PTEN(state, tracer, s_0_20);
        // D s_0_22: write-var __CNTHCTL_EL2_EL0PTEN <= s_0_21
        fn_state.u__CNTHCTL_EL2_EL0PTEN = s_0_21;
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
        // N s_0_29: branch s_0_28 b42 b1
        if s_0_28 {
            return block_42(state, tracer, fn_state);
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
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // D s_5_4: call Mk_CNTP_CVAL_EL0_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_EL0_Type(
            state,
            tracer,
            s_5_3,
        );
        // C s_5_5: const #20800u : u32
        let s_5_5: u32 = 20800;
        // N s_5_6: write-reg s_5_5 <= s_5_4
        let s_5_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_5 as isize, s_5_4);
            tracer.write_register(s_5_5 as isize, s_5_4);
        };
        // N s_5_7: return
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
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #1u : u8
        let s_6_4: bool = true;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b19 b7
        if s_6_6 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#81610 <= s_7_0
        fn_state.gs_81610 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#81610:u8
        let s_8_0: bool = fn_state.gs_81610;
        // N s_8_1: branch s_8_0 b18 b9
        if s_8_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#81611 <= s_9_0
        fn_state.gs_81611 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#81611:u8
        let s_10_0: bool = fn_state.gs_81611;
        // N s_10_1: branch s_10_0 b17 b11
        if s_10_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #102552u : u32
        let s_11_0: u32 = 102552;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_HCR_EL2_Type_E2H(s_11_1)
        let s_11_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_11_1);
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #1u : u8
        let s_11_4: bool = true;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // N s_11_7: branch s_11_6 b16 b12
        if s_11_6 {
            return block_16(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#81612 <= s_12_0
        fn_state.gs_81612 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#81612:u8
        let s_13_0: bool = fn_state.gs_81612;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #64s : i64
        let s_14_0: i64 = 64;
        // D s_14_1: read-var t:i
        let s_14_1: i128 = fn_state.t;
        // D s_14_2: call X_read(s_14_1, s_14_0)
        let s_14_2: Bits = X_read(state, tracer, s_14_1, s_14_0);
        // D s_14_3: cast reint s_14_2 -> u64
        let s_14_3: u64 = (s_14_2.value() as u64);
        // D s_14_4: call Mk_CNTP_CVAL_EL0_Type(s_14_3)
        let s_14_4: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_EL0_Type(
            state,
            tracer,
            s_14_3,
        );
        // C s_14_5: const #20800u : u32
        let s_14_5: u32 = 20800;
        // N s_14_6: write-reg s_14_5 <= s_14_4
        let s_14_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_14_5 as isize, s_14_4);
            tracer.write_register(s_14_5 as isize, s_14_4);
        };
        // N s_14_7: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // D s_15_1: read-var t:i
        let s_15_1: i128 = fn_state.t;
        // D s_15_2: call X_read(s_15_1, s_15_0)
        let s_15_2: Bits = X_read(state, tracer, s_15_1, s_15_0);
        // D s_15_3: cast reint s_15_2 -> u64
        let s_15_3: u64 = (s_15_2.value() as u64);
        // D s_15_4: call Mk_CNTHP_CVAL_EL2_Type(s_15_3)
        let s_15_4: ProductType5c790c8ef59cc8b2 = Mk_CNTHP_CVAL_EL2_Type(
            state,
            tracer,
            s_15_3,
        );
        // C s_15_5: const #16640u : u32
        let s_15_5: u32 = 16640;
        // N s_15_6: write-reg s_15_5 <= s_15_4
        let s_15_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_15_5 as isize, s_15_4);
            tracer.write_register(s_15_5 as isize, s_15_4);
        };
        // N s_15_7: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #90704u : u32
        let s_16_0: u32 = 90704;
        // D s_16_1: read-reg s_16_0:struct
        let s_16_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call _get_SCR_EL3_Type_NS(s_16_1)
        let s_16_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_16_1);
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // C s_16_4: const #1u : u8
        let s_16_4: bool = true;
        // C s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: cmp-eq s_16_3 s_16_5
        let s_16_6: bool = ((s_16_3) == (s_16_5));
        // D s_16_7: write-var gs#81612 <= s_16_6
        fn_state.gs_81612 = s_16_6;
        // N s_16_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #64s : i64
        let s_17_0: i64 = 64;
        // D s_17_1: read-var t:i
        let s_17_1: i128 = fn_state.t;
        // D s_17_2: call X_read(s_17_1, s_17_0)
        let s_17_2: Bits = X_read(state, tracer, s_17_1, s_17_0);
        // D s_17_3: cast reint s_17_2 -> u64
        let s_17_3: u64 = (s_17_2.value() as u64);
        // D s_17_4: call Mk_CNTHPS_CVAL_EL2_Type(s_17_3)
        let s_17_4: ProductType5c790c8ef59cc8b2 = Mk_CNTHPS_CVAL_EL2_Type(
            state,
            tracer,
            s_17_3,
        );
        // C s_17_5: const #22672u : u32
        let s_17_5: u32 = 22672;
        // N s_17_6: write-reg s_17_5 <= s_17_4
        let s_17_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_17_5 as isize, s_17_4);
            tracer.write_register(s_17_5 as isize, s_17_4);
        };
        // N s_17_7: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #117u : u32
        let s_18_0: u32 = 117;
        // S s_18_1: call IsFeatureImplemented(s_18_0)
        let s_18_1: bool = IsFeatureImplemented(state, tracer, s_18_0);
        // D s_18_2: write-var gs#81611 <= s_18_1
        fn_state.gs_81611 = s_18_1;
        // N s_18_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #90704u : u32
        let s_19_0: u32 = 90704;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_SCR_EL3_Type_NS(s_19_1)
        let s_19_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // C s_19_4: const #0u : u8
        let s_19_4: bool = false;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // D s_19_7: write-var gs#81610 <= s_19_6
        fn_state.gs_81610 = s_19_6;
        // N s_19_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call EL2Enabled(s_20_0)
        let s_20_1: bool = EL2Enabled(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b41 b21
        if s_20_1 {
            return block_41(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#81616 <= s_21_0
        fn_state.gs_81616 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#81616:u8
        let s_22_0: bool = fn_state.gs_81616;
        // N s_22_1: branch s_22_0 b40 b23
        if s_22_0 {
            return block_40(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#81617 <= s_23_0
        fn_state.gs_81617 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#81617:u8
        let s_24_0: bool = fn_state.gs_81617;
        // N s_24_1: branch s_24_0 b39 b25
        if s_24_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call EL2Enabled(s_25_0)
        let s_25_1: bool = EL2Enabled(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b38 b26
        if s_25_1 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#81618 <= s_26_0
        fn_state.gs_81618 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#81618:u8
        let s_27_0: bool = fn_state.gs_81618;
        // N s_27_1: branch s_27_0 b37 b28
        if s_27_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#81619 <= s_28_0
        fn_state.gs_81619 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#81619:u8
        let s_29_0: bool = fn_state.gs_81619;
        // N s_29_1: branch s_29_0 b36 b30
        if s_29_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call EL2Enabled(s_30_0)
        let s_30_1: bool = EL2Enabled(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b35 b31
        if s_30_1 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#81620 <= s_31_0
        fn_state.gs_81620 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#81620:u8
        let s_32_0: bool = fn_state.gs_81620;
        // N s_32_1: branch s_32_0 b34 b33
        if s_32_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #64s : i64
        let s_33_0: i64 = 64;
        // D s_33_1: read-var t:i
        let s_33_1: i128 = fn_state.t;
        // D s_33_2: call X_read(s_33_1, s_33_0)
        let s_33_2: Bits = X_read(state, tracer, s_33_1, s_33_0);
        // D s_33_3: cast reint s_33_2 -> u64
        let s_33_3: u64 = (s_33_2.value() as u64);
        // D s_33_4: call Mk_CNTP_CVAL_EL0_Type(s_33_3)
        let s_33_4: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_EL0_Type(
            state,
            tracer,
            s_33_3,
        );
        // C s_33_5: const #20800u : u32
        let s_33_5: u32 = 20800;
        // N s_33_6: write-reg s_33_5 <= s_33_4
        let s_33_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_33_5 as isize, s_33_4);
            tracer.write_register(s_33_5 as isize, s_33_4);
        };
        // N s_33_7: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #376u : u12
        let s_34_0: u16 = 376;
        // C s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 12u16);
        // C s_34_2: cast zx s_34_1 -> i
        let s_34_2: i128 = (s_34_1.value() as i128);
        // C s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: const #64s : i64
        let s_34_4: i64 = 64;
        // D s_34_5: read-var t:i
        let s_34_5: i128 = fn_state.t;
        // D s_34_6: call X_read(s_34_5, s_34_4)
        let s_34_6: Bits = X_read(state, tracer, s_34_5, s_34_4);
        // D s_34_7: cast reint s_34_6 -> u64
        let s_34_7: u64 = (s_34_6.value() as u64);
        // C s_34_8: cast zx s_34_3 -> i
        let s_34_8: i128 = (i128::try_from(s_34_3).unwrap());
        // D s_34_9: call NVMem_set(s_34_8, s_34_7)
        let s_34_9: () = NVMem_set(state, tracer, s_34_8, s_34_7);
        // N s_34_10: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #102552u : u32
        let s_35_0: u32 = 102552;
        // D s_35_1: read-reg s_35_0:struct
        let s_35_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call _get_HCR_EL2_Type_NV2(s_35_1)
        let s_35_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_35_1);
        // C s_35_3: const #102552u : u32
        let s_35_3: u32 = 102552;
        // D s_35_4: read-reg s_35_3:struct
        let s_35_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_3 as isize);
            tracer.read_register(s_35_3 as isize, value);
            value
        };
        // D s_35_5: call _get_HCR_EL2_Type_NV1(s_35_4)
        let s_35_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_35_4);
        // C s_35_6: const #102552u : u32
        let s_35_6: u32 = 102552;
        // D s_35_7: read-reg s_35_6:struct
        let s_35_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_6 as isize);
            tracer.read_register(s_35_6 as isize, value);
            value
        };
        // D s_35_8: call _get_HCR_EL2_Type_NV(s_35_7)
        let s_35_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_35_7);
        // D s_35_9: cast zx s_35_5 -> bv
        let s_35_9: Bits = Bits::new(s_35_5 as u128, 1u16);
        // D s_35_10: cast zx s_35_8 -> bv
        let s_35_10: Bits = Bits::new(s_35_8 as u128, 1u16);
        // D s_35_11: cast reint s_35_9 -> u128
        let s_35_11: u128 = (s_35_9.value() as u128);
        // D s_35_12: size-of s_35_9
        let s_35_12: u16 = s_35_9.length();
        // D s_35_13: cast reint s_35_10 -> u128
        let s_35_13: u128 = (s_35_10.value() as u128);
        // D s_35_14: size-of s_35_10
        let s_35_14: u16 = s_35_10.length();
        // D s_35_15: lsl s_35_11 s_35_14
        let s_35_15: u128 = s_35_11 << s_35_14;
        // D s_35_16: or s_35_15 s_35_13
        let s_35_16: u128 = ((s_35_15) | (s_35_13));
        // D s_35_17: add s_35_12 s_35_14
        let s_35_17: u16 = (s_35_12 + s_35_14);
        // D s_35_18: create-bits s_35_16 s_35_17
        let s_35_18: Bits = Bits::new(s_35_16, s_35_17);
        // D s_35_19: cast reint s_35_18 -> u8
        let s_35_19: u8 = (s_35_18.value() as u8);
        // D s_35_20: cast zx s_35_2 -> bv
        let s_35_20: Bits = Bits::new(s_35_2 as u128, 1u16);
        // D s_35_21: cast zx s_35_19 -> bv
        let s_35_21: Bits = Bits::new(s_35_19 as u128, 2u16);
        // D s_35_22: cast reint s_35_20 -> u128
        let s_35_22: u128 = (s_35_20.value() as u128);
        // D s_35_23: size-of s_35_20
        let s_35_23: u16 = s_35_20.length();
        // D s_35_24: cast reint s_35_21 -> u128
        let s_35_24: u128 = (s_35_21.value() as u128);
        // D s_35_25: size-of s_35_21
        let s_35_25: u16 = s_35_21.length();
        // D s_35_26: lsl s_35_22 s_35_25
        let s_35_26: u128 = s_35_22 << s_35_25;
        // D s_35_27: or s_35_26 s_35_24
        let s_35_27: u128 = ((s_35_26) | (s_35_24));
        // D s_35_28: add s_35_23 s_35_25
        let s_35_28: u16 = (s_35_23 + s_35_25);
        // D s_35_29: create-bits s_35_27 s_35_28
        let s_35_29: Bits = Bits::new(s_35_27, s_35_28);
        // D s_35_30: cast reint s_35_29 -> u8
        let s_35_30: u8 = (s_35_29.value() as u8);
        // D s_35_31: cast zx s_35_30 -> bv
        let s_35_31: Bits = Bits::new(s_35_30 as u128, 3u16);
        // C s_35_32: const #7u : u8
        let s_35_32: u8 = 7;
        // C s_35_33: cast zx s_35_32 -> bv
        let s_35_33: Bits = Bits::new(s_35_32 as u128, 3u16);
        // D s_35_34: cmp-eq s_35_31 s_35_33
        let s_35_34: bool = ((s_35_31) == (s_35_33));
        // D s_35_35: write-var gs#81620 <= s_35_34
        fn_state.gs_81620 = s_35_34;
        // N s_35_36: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #24u : u8
        let s_36_0: u8 = 24;
        // C s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 8u16);
        // C s_36_2: cast zx s_36_1 -> i
        let s_36_2: i128 = (s_36_1.value() as i128);
        // C s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: cast zx s_36_3 -> i
        let s_36_4: i128 = (i128::try_from(s_36_3).unwrap());
        // C s_36_5: const #432u : u32
        let s_36_5: u32 = 432;
        // D s_36_6: read-reg s_36_5:u8
        let s_36_6: u8 = {
            let value = state.read_register::<u8>(s_36_5 as isize);
            tracer.read_register(s_36_5 as isize, value);
            value
        };
        // D s_36_7: call AArch64_SystemAccessTrap(s_36_6, s_36_4)
        let s_36_7: () = AArch64_SystemAccessTrap(state, tracer, s_36_6, s_36_4);
        // N s_36_8: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_37_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #0u : u8
        let s_37_2: bool = false;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#81619 <= s_37_4
        fn_state.gs_81619 = s_37_4;
        // N s_37_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #102552u : u32
        let s_38_0: u32 = 102552;
        // D s_38_1: read-reg s_38_0:struct
        let s_38_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call _get_HCR_EL2_Type_E2H(s_38_1)
        let s_38_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_38_1);
        // D s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // C s_38_4: const #1u : u8
        let s_38_4: bool = true;
        // C s_38_5: cast zx s_38_4 -> bv
        let s_38_5: Bits = Bits::new(s_38_4 as u128, 1u16);
        // D s_38_6: cmp-eq s_38_3 s_38_5
        let s_38_6: bool = ((s_38_3) == (s_38_5));
        // D s_38_7: write-var gs#81618 <= s_38_6
        fn_state.gs_81618 = s_38_6;
        // N s_38_8: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #24u : u8
        let s_39_0: u8 = 24;
        // C s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 8u16);
        // C s_39_2: cast zx s_39_1 -> i
        let s_39_2: i128 = (s_39_1.value() as i128);
        // C s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: cast zx s_39_3 -> i
        let s_39_4: i128 = (i128::try_from(s_39_3).unwrap());
        // C s_39_5: const #432u : u32
        let s_39_5: u32 = 432;
        // D s_39_6: read-reg s_39_5:u8
        let s_39_6: u8 = {
            let value = state.read_register::<u8>(s_39_5 as isize);
            tracer.read_register(s_39_5 as isize, value);
            value
        };
        // D s_39_7: call AArch64_SystemAccessTrap(s_39_6, s_39_4)
        let s_39_7: () = AArch64_SystemAccessTrap(state, tracer, s_39_6, s_39_4);
        // N s_39_8: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_40_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #0u : u8
        let s_40_2: bool = false;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#81617 <= s_40_4
        fn_state.gs_81617 = s_40_4;
        // N s_40_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #102552u : u32
        let s_41_0: u32 = 102552;
        // D s_41_1: read-reg s_41_0:struct
        let s_41_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: call _get_HCR_EL2_Type_E2H(s_41_1)
        let s_41_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_41_1);
        // D s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // C s_41_4: const #0u : u8
        let s_41_4: bool = false;
        // C s_41_5: cast zx s_41_4 -> bv
        let s_41_5: Bits = Bits::new(s_41_4 as u128, 1u16);
        // D s_41_6: cmp-eq s_41_3 s_41_5
        let s_41_6: bool = ((s_41_3) == (s_41_5));
        // D s_41_7: write-var gs#81616 <= s_41_6
        fn_state.gs_81616 = s_41_6;
        // N s_41_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call EL2Enabled(s_42_0)
        let s_42_1: bool = EL2Enabled(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b98 b43
        if s_42_1 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#81623 <= s_43_0
        fn_state.gs_81623 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#81623:u8
        let s_44_0: bool = fn_state.gs_81623;
        // D s_44_1: not s_44_0
        let s_44_1: bool = !s_44_0;
        // N s_44_2: branch s_44_1 b97 b45
        if s_44_1 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#81624 <= s_45_0
        fn_state.gs_81624 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#81624:u8
        let s_46_0: bool = fn_state.gs_81624;
        // N s_46_1: branch s_46_0 b91 b47
        if s_46_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EL2Enabled(s_47_0)
        let s_47_1: bool = EL2Enabled(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b90 b48
        if s_47_1 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#81625 <= s_48_0
        fn_state.gs_81625 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#81625:u8
        let s_49_0: bool = fn_state.gs_81625;
        // N s_49_1: branch s_49_0 b89 b50
        if s_49_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#81626 <= s_50_0
        fn_state.gs_81626 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#81626:u8
        let s_51_0: bool = fn_state.gs_81626;
        // N s_51_1: branch s_51_0 b88 b52
        if s_51_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EL2Enabled(s_52_0)
        let s_52_1: bool = EL2Enabled(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b87 b53
        if s_52_1 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#81627 <= s_53_0
        fn_state.gs_81627 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#81627:u8
        let s_54_0: bool = fn_state.gs_81627;
        // N s_54_1: branch s_54_0 b86 b55
        if s_54_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#81628 <= s_55_0
        fn_state.gs_81628 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#81628:u8
        let s_56_0: bool = fn_state.gs_81628;
        // N s_56_1: branch s_56_0 b85 b57
        if s_56_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call EL2Enabled(s_57_0)
        let s_57_1: bool = EL2Enabled(state, tracer, s_57_0);
        // N s_57_2: branch s_57_1 b84 b58
        if s_57_1 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#81629 <= s_58_0
        fn_state.gs_81629 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#81629:u8
        let s_59_0: bool = fn_state.gs_81629;
        // N s_59_1: branch s_59_0 b83 b60
        if s_59_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#81630 <= s_60_0
        fn_state.gs_81630 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#81630:u8
        let s_61_0: bool = fn_state.gs_81630;
        // N s_61_1: branch s_61_0 b82 b62
        if s_61_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call EL2Enabled(s_62_0)
        let s_62_1: bool = EL2Enabled(state, tracer, s_62_0);
        // N s_62_2: branch s_62_1 b81 b63
        if s_62_1 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#81631 <= s_63_0
        fn_state.gs_81631 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#81631:u8
        let s_64_0: bool = fn_state.gs_81631;
        // N s_64_1: branch s_64_0 b80 b65
        if s_64_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#81632 <= s_65_0
        fn_state.gs_81632 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#81632:u8
        let s_66_0: bool = fn_state.gs_81632;
        // N s_66_1: branch s_66_0 b79 b67
        if s_66_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#81633 <= s_67_0
        fn_state.gs_81633 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#81633:u8
        let s_68_0: bool = fn_state.gs_81633;
        // N s_68_1: branch s_68_0 b78 b69
        if s_68_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #() : ()
        let s_69_0: () = ();
        // S s_69_1: call EL2Enabled(s_69_0)
        let s_69_1: bool = EL2Enabled(state, tracer, s_69_0);
        // N s_69_2: branch s_69_1 b77 b70
        if s_69_1 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#81634 <= s_70_0
        fn_state.gs_81634 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#81634:u8
        let s_71_0: bool = fn_state.gs_81634;
        // N s_71_1: branch s_71_0 b76 b72
        if s_71_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var gs#81635 <= s_72_0
        fn_state.gs_81635 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#81635:u8
        let s_73_0: bool = fn_state.gs_81635;
        // N s_73_1: branch s_73_0 b75 b74
        if s_73_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #64s : i64
        let s_74_0: i64 = 64;
        // D s_74_1: read-var t:i
        let s_74_1: i128 = fn_state.t;
        // D s_74_2: call X_read(s_74_1, s_74_0)
        let s_74_2: Bits = X_read(state, tracer, s_74_1, s_74_0);
        // D s_74_3: cast reint s_74_2 -> u64
        let s_74_3: u64 = (s_74_2.value() as u64);
        // D s_74_4: call Mk_CNTP_CVAL_EL0_Type(s_74_3)
        let s_74_4: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_EL0_Type(
            state,
            tracer,
            s_74_3,
        );
        // C s_74_5: const #20800u : u32
        let s_74_5: u32 = 20800;
        // N s_74_6: write-reg s_74_5 <= s_74_4
        let s_74_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_74_5 as isize, s_74_4);
            tracer.write_register(s_74_5 as isize, s_74_4);
        };
        // N s_74_7: return
        return;
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #64s : i64
        let s_75_0: i64 = 64;
        // D s_75_1: read-var t:i
        let s_75_1: i128 = fn_state.t;
        // D s_75_2: call X_read(s_75_1, s_75_0)
        let s_75_2: Bits = X_read(state, tracer, s_75_1, s_75_0);
        // D s_75_3: cast reint s_75_2 -> u64
        let s_75_3: u64 = (s_75_2.value() as u64);
        // D s_75_4: call Mk_CNTHP_CVAL_EL2_Type(s_75_3)
        let s_75_4: ProductType5c790c8ef59cc8b2 = Mk_CNTHP_CVAL_EL2_Type(
            state,
            tracer,
            s_75_3,
        );
        // C s_75_5: const #16640u : u32
        let s_75_5: u32 = 16640;
        // N s_75_6: write-reg s_75_5 <= s_75_4
        let s_75_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_75_5 as isize, s_75_4);
            tracer.write_register(s_75_5 as isize, s_75_4);
        };
        // N s_75_7: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #90704u : u32
        let s_76_0: u32 = 90704;
        // D s_76_1: read-reg s_76_0:struct
        let s_76_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // D s_76_2: call _get_SCR_EL3_Type_NS(s_76_1)
        let s_76_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_76_1);
        // D s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // C s_76_4: const #1u : u8
        let s_76_4: bool = true;
        // C s_76_5: cast zx s_76_4 -> bv
        let s_76_5: Bits = Bits::new(s_76_4 as u128, 1u16);
        // D s_76_6: cmp-eq s_76_3 s_76_5
        let s_76_6: bool = ((s_76_3) == (s_76_5));
        // D s_76_7: write-var gs#81635 <= s_76_6
        fn_state.gs_81635 = s_76_6;
        // N s_76_8: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #102552u : u32
        let s_77_0: u32 = 102552;
        // D s_77_1: read-reg s_77_0:struct
        let s_77_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call _get_HCR_EL2_Type_E2H(s_77_1)
        let s_77_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_77_1);
        // C s_77_3: const #102552u : u32
        let s_77_3: u32 = 102552;
        // D s_77_4: read-reg s_77_3:struct
        let s_77_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_77_3 as isize);
            tracer.read_register(s_77_3 as isize, value);
            value
        };
        // D s_77_5: call _get_HCR_EL2_Type_TGE(s_77_4)
        let s_77_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_77_4);
        // D s_77_6: cast zx s_77_2 -> bv
        let s_77_6: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_7: cast zx s_77_5 -> bv
        let s_77_7: Bits = Bits::new(s_77_5 as u128, 1u16);
        // D s_77_8: cast reint s_77_6 -> u128
        let s_77_8: u128 = (s_77_6.value() as u128);
        // D s_77_9: size-of s_77_6
        let s_77_9: u16 = s_77_6.length();
        // D s_77_10: cast reint s_77_7 -> u128
        let s_77_10: u128 = (s_77_7.value() as u128);
        // D s_77_11: size-of s_77_7
        let s_77_11: u16 = s_77_7.length();
        // D s_77_12: lsl s_77_8 s_77_11
        let s_77_12: u128 = s_77_8 << s_77_11;
        // D s_77_13: or s_77_12 s_77_10
        let s_77_13: u128 = ((s_77_12) | (s_77_10));
        // D s_77_14: add s_77_9 s_77_11
        let s_77_14: u16 = (s_77_9 + s_77_11);
        // D s_77_15: create-bits s_77_13 s_77_14
        let s_77_15: Bits = Bits::new(s_77_13, s_77_14);
        // D s_77_16: cast reint s_77_15 -> u8
        let s_77_16: u8 = (s_77_15.value() as u8);
        // D s_77_17: cast zx s_77_16 -> bv
        let s_77_17: Bits = Bits::new(s_77_16 as u128, 2u16);
        // C s_77_18: const #3u : u8
        let s_77_18: u8 = 3;
        // C s_77_19: cast zx s_77_18 -> bv
        let s_77_19: Bits = Bits::new(s_77_18 as u128, 2u16);
        // D s_77_20: cmp-eq s_77_17 s_77_19
        let s_77_20: bool = ((s_77_17) == (s_77_19));
        // D s_77_21: write-var gs#81634 <= s_77_20
        fn_state.gs_81634 = s_77_20;
        // N s_77_22: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #64s : i64
        let s_78_0: i64 = 64;
        // D s_78_1: read-var t:i
        let s_78_1: i128 = fn_state.t;
        // D s_78_2: call X_read(s_78_1, s_78_0)
        let s_78_2: Bits = X_read(state, tracer, s_78_1, s_78_0);
        // D s_78_3: cast reint s_78_2 -> u64
        let s_78_3: u64 = (s_78_2.value() as u64);
        // D s_78_4: call Mk_CNTHPS_CVAL_EL2_Type(s_78_3)
        let s_78_4: ProductType5c790c8ef59cc8b2 = Mk_CNTHPS_CVAL_EL2_Type(
            state,
            tracer,
            s_78_3,
        );
        // C s_78_5: const #22672u : u32
        let s_78_5: u32 = 22672;
        // N s_78_6: write-reg s_78_5 <= s_78_4
        let s_78_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_78_5 as isize, s_78_4);
            tracer.write_register(s_78_5 as isize, s_78_4);
        };
        // N s_78_7: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #117u : u32
        let s_79_0: u32 = 117;
        // S s_79_1: call IsFeatureImplemented(s_79_0)
        let s_79_1: bool = IsFeatureImplemented(state, tracer, s_79_0);
        // D s_79_2: write-var gs#81633 <= s_79_1
        fn_state.gs_81633 = s_79_1;
        // N s_79_3: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #90704u : u32
        let s_80_0: u32 = 90704;
        // D s_80_1: read-reg s_80_0:struct
        let s_80_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // D s_80_2: call _get_SCR_EL3_Type_NS(s_80_1)
        let s_80_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_80_1);
        // D s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // C s_80_4: const #0u : u8
        let s_80_4: bool = false;
        // C s_80_5: cast zx s_80_4 -> bv
        let s_80_5: Bits = Bits::new(s_80_4 as u128, 1u16);
        // D s_80_6: cmp-eq s_80_3 s_80_5
        let s_80_6: bool = ((s_80_3) == (s_80_5));
        // D s_80_7: write-var gs#81632 <= s_80_6
        fn_state.gs_81632 = s_80_6;
        // N s_80_8: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #102552u : u32
        let s_81_0: u32 = 102552;
        // D s_81_1: read-reg s_81_0:struct
        let s_81_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call _get_HCR_EL2_Type_E2H(s_81_1)
        let s_81_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_81_1);
        // C s_81_3: const #102552u : u32
        let s_81_3: u32 = 102552;
        // D s_81_4: read-reg s_81_3:struct
        let s_81_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_81_3 as isize);
            tracer.read_register(s_81_3 as isize, value);
            value
        };
        // D s_81_5: call _get_HCR_EL2_Type_TGE(s_81_4)
        let s_81_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_81_4);
        // D s_81_6: cast zx s_81_2 -> bv
        let s_81_6: Bits = Bits::new(s_81_2 as u128, 1u16);
        // D s_81_7: cast zx s_81_5 -> bv
        let s_81_7: Bits = Bits::new(s_81_5 as u128, 1u16);
        // D s_81_8: cast reint s_81_6 -> u128
        let s_81_8: u128 = (s_81_6.value() as u128);
        // D s_81_9: size-of s_81_6
        let s_81_9: u16 = s_81_6.length();
        // D s_81_10: cast reint s_81_7 -> u128
        let s_81_10: u128 = (s_81_7.value() as u128);
        // D s_81_11: size-of s_81_7
        let s_81_11: u16 = s_81_7.length();
        // D s_81_12: lsl s_81_8 s_81_11
        let s_81_12: u128 = s_81_8 << s_81_11;
        // D s_81_13: or s_81_12 s_81_10
        let s_81_13: u128 = ((s_81_12) | (s_81_10));
        // D s_81_14: add s_81_9 s_81_11
        let s_81_14: u16 = (s_81_9 + s_81_11);
        // D s_81_15: create-bits s_81_13 s_81_14
        let s_81_15: Bits = Bits::new(s_81_13, s_81_14);
        // D s_81_16: cast reint s_81_15 -> u8
        let s_81_16: u8 = (s_81_15.value() as u8);
        // D s_81_17: cast zx s_81_16 -> bv
        let s_81_17: Bits = Bits::new(s_81_16 as u128, 2u16);
        // C s_81_18: const #3u : u8
        let s_81_18: u8 = 3;
        // C s_81_19: cast zx s_81_18 -> bv
        let s_81_19: Bits = Bits::new(s_81_18 as u128, 2u16);
        // D s_81_20: cmp-eq s_81_17 s_81_19
        let s_81_20: bool = ((s_81_17) == (s_81_19));
        // D s_81_21: write-var gs#81631 <= s_81_20
        fn_state.gs_81631 = s_81_20;
        // N s_81_22: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #24u : u8
        let s_82_0: u8 = 24;
        // C s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 8u16);
        // C s_82_2: cast zx s_82_1 -> i
        let s_82_2: i128 = (s_82_1.value() as i128);
        // C s_82_3: cast reint s_82_2 -> i64
        let s_82_3: i64 = (s_82_2 as i64);
        // C s_82_4: cast zx s_82_3 -> i
        let s_82_4: i128 = (i128::try_from(s_82_3).unwrap());
        // C s_82_5: const #432u : u32
        let s_82_5: u32 = 432;
        // D s_82_6: read-reg s_82_5:u8
        let s_82_6: u8 = {
            let value = state.read_register::<u8>(s_82_5 as isize);
            tracer.read_register(s_82_5 as isize, value);
            value
        };
        // D s_82_7: call AArch64_SystemAccessTrap(s_82_6, s_82_4)
        let s_82_7: () = AArch64_SystemAccessTrap(state, tracer, s_82_6, s_82_4);
        // N s_82_8: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var __CNTHCTL_EL2_EL0PTEN:u8
        let s_83_0: bool = fn_state.u__CNTHCTL_EL2_EL0PTEN;
        // D s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 1u16);
        // C s_83_2: const #0u : u8
        let s_83_2: bool = false;
        // C s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 1u16);
        // D s_83_4: cmp-eq s_83_1 s_83_3
        let s_83_4: bool = ((s_83_1) == (s_83_3));
        // D s_83_5: write-var gs#81630 <= s_83_4
        fn_state.gs_81630 = s_83_4;
        // N s_83_6: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #102552u : u32
        let s_84_0: u32 = 102552;
        // D s_84_1: read-reg s_84_0:struct
        let s_84_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call _get_HCR_EL2_Type_E2H(s_84_1)
        let s_84_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_84_1);
        // C s_84_3: const #102552u : u32
        let s_84_3: u32 = 102552;
        // D s_84_4: read-reg s_84_3:struct
        let s_84_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_3 as isize);
            tracer.read_register(s_84_3 as isize, value);
            value
        };
        // D s_84_5: call _get_HCR_EL2_Type_TGE(s_84_4)
        let s_84_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_84_4);
        // D s_84_6: cast zx s_84_2 -> bv
        let s_84_6: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_7: cast zx s_84_5 -> bv
        let s_84_7: Bits = Bits::new(s_84_5 as u128, 1u16);
        // D s_84_8: cast reint s_84_6 -> u128
        let s_84_8: u128 = (s_84_6.value() as u128);
        // D s_84_9: size-of s_84_6
        let s_84_9: u16 = s_84_6.length();
        // D s_84_10: cast reint s_84_7 -> u128
        let s_84_10: u128 = (s_84_7.value() as u128);
        // D s_84_11: size-of s_84_7
        let s_84_11: u16 = s_84_7.length();
        // D s_84_12: lsl s_84_8 s_84_11
        let s_84_12: u128 = s_84_8 << s_84_11;
        // D s_84_13: or s_84_12 s_84_10
        let s_84_13: u128 = ((s_84_12) | (s_84_10));
        // D s_84_14: add s_84_9 s_84_11
        let s_84_14: u16 = (s_84_9 + s_84_11);
        // D s_84_15: create-bits s_84_13 s_84_14
        let s_84_15: Bits = Bits::new(s_84_13, s_84_14);
        // D s_84_16: cast reint s_84_15 -> u8
        let s_84_16: u8 = (s_84_15.value() as u8);
        // D s_84_17: cast zx s_84_16 -> bv
        let s_84_17: Bits = Bits::new(s_84_16 as u128, 2u16);
        // C s_84_18: const #3u : u8
        let s_84_18: u8 = 3;
        // C s_84_19: cast zx s_84_18 -> bv
        let s_84_19: Bits = Bits::new(s_84_18 as u128, 2u16);
        // D s_84_20: cmp-eq s_84_17 s_84_19
        let s_84_20: bool = ((s_84_17) == (s_84_19));
        // D s_84_21: write-var gs#81629 <= s_84_20
        fn_state.gs_81629 = s_84_20;
        // N s_84_22: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #24u : u8
        let s_85_0: u8 = 24;
        // C s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 8u16);
        // C s_85_2: cast zx s_85_1 -> i
        let s_85_2: i128 = (s_85_1.value() as i128);
        // C s_85_3: cast reint s_85_2 -> i64
        let s_85_3: i64 = (s_85_2 as i64);
        // C s_85_4: cast zx s_85_3 -> i
        let s_85_4: i128 = (i128::try_from(s_85_3).unwrap());
        // C s_85_5: const #432u : u32
        let s_85_5: u32 = 432;
        // D s_85_6: read-reg s_85_5:u8
        let s_85_6: u8 = {
            let value = state.read_register::<u8>(s_85_5 as isize);
            tracer.read_register(s_85_5 as isize, value);
            value
        };
        // D s_85_7: call AArch64_SystemAccessTrap(s_85_6, s_85_4)
        let s_85_7: () = AArch64_SystemAccessTrap(state, tracer, s_85_6, s_85_4);
        // N s_85_8: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_86_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 1u16);
        // C s_86_2: const #0u : u8
        let s_86_2: bool = false;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: write-var gs#81628 <= s_86_4
        fn_state.gs_81628 = s_86_4;
        // N s_86_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #102552u : u32
        let s_87_0: u32 = 102552;
        // D s_87_1: read-reg s_87_0:struct
        let s_87_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_87_0 as isize);
            tracer.read_register(s_87_0 as isize, value);
            value
        };
        // D s_87_2: call _get_HCR_EL2_Type_E2H(s_87_1)
        let s_87_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_87_1);
        // C s_87_3: const #102552u : u32
        let s_87_3: u32 = 102552;
        // D s_87_4: read-reg s_87_3:struct
        let s_87_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_87_3 as isize);
            tracer.read_register(s_87_3 as isize, value);
            value
        };
        // D s_87_5: call _get_HCR_EL2_Type_TGE(s_87_4)
        let s_87_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_87_4);
        // D s_87_6: cast zx s_87_2 -> bv
        let s_87_6: Bits = Bits::new(s_87_2 as u128, 1u16);
        // D s_87_7: cast zx s_87_5 -> bv
        let s_87_7: Bits = Bits::new(s_87_5 as u128, 1u16);
        // D s_87_8: cast reint s_87_6 -> u128
        let s_87_8: u128 = (s_87_6.value() as u128);
        // D s_87_9: size-of s_87_6
        let s_87_9: u16 = s_87_6.length();
        // D s_87_10: cast reint s_87_7 -> u128
        let s_87_10: u128 = (s_87_7.value() as u128);
        // D s_87_11: size-of s_87_7
        let s_87_11: u16 = s_87_7.length();
        // D s_87_12: lsl s_87_8 s_87_11
        let s_87_12: u128 = s_87_8 << s_87_11;
        // D s_87_13: or s_87_12 s_87_10
        let s_87_13: u128 = ((s_87_12) | (s_87_10));
        // D s_87_14: add s_87_9 s_87_11
        let s_87_14: u16 = (s_87_9 + s_87_11);
        // D s_87_15: create-bits s_87_13 s_87_14
        let s_87_15: Bits = Bits::new(s_87_13, s_87_14);
        // D s_87_16: cast reint s_87_15 -> u8
        let s_87_16: u8 = (s_87_15.value() as u8);
        // D s_87_17: cast zx s_87_16 -> bv
        let s_87_17: Bits = Bits::new(s_87_16 as u128, 2u16);
        // C s_87_18: const #2u : u8
        let s_87_18: u8 = 2;
        // C s_87_19: cast zx s_87_18 -> bv
        let s_87_19: Bits = Bits::new(s_87_18 as u128, 2u16);
        // D s_87_20: cmp-eq s_87_17 s_87_19
        let s_87_20: bool = ((s_87_17) == (s_87_19));
        // D s_87_21: write-var gs#81627 <= s_87_20
        fn_state.gs_81627 = s_87_20;
        // N s_87_22: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #24u : u8
        let s_88_0: u8 = 24;
        // C s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 8u16);
        // C s_88_2: cast zx s_88_1 -> i
        let s_88_2: i128 = (s_88_1.value() as i128);
        // C s_88_3: cast reint s_88_2 -> i64
        let s_88_3: i64 = (s_88_2 as i64);
        // C s_88_4: cast zx s_88_3 -> i
        let s_88_4: i128 = (i128::try_from(s_88_3).unwrap());
        // C s_88_5: const #432u : u32
        let s_88_5: u32 = 432;
        // D s_88_6: read-reg s_88_5:u8
        let s_88_6: u8 = {
            let value = state.read_register::<u8>(s_88_5 as isize);
            tracer.read_register(s_88_5 as isize, value);
            value
        };
        // D s_88_7: call AArch64_SystemAccessTrap(s_88_6, s_88_4)
        let s_88_7: () = AArch64_SystemAccessTrap(state, tracer, s_88_6, s_88_4);
        // N s_88_8: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_89_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #0u : u8
        let s_89_2: bool = false;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // D s_89_5: write-var gs#81626 <= s_89_4
        fn_state.gs_81626 = s_89_4;
        // N s_89_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #102552u : u32
        let s_90_0: u32 = 102552;
        // D s_90_1: read-reg s_90_0:struct
        let s_90_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // D s_90_2: call _get_HCR_EL2_Type_E2H(s_90_1)
        let s_90_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_90_1);
        // D s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // C s_90_4: const #0u : u8
        let s_90_4: bool = false;
        // C s_90_5: cast zx s_90_4 -> bv
        let s_90_5: Bits = Bits::new(s_90_4 as u128, 1u16);
        // D s_90_6: cmp-eq s_90_3 s_90_5
        let s_90_6: bool = ((s_90_3) == (s_90_5));
        // D s_90_7: write-var gs#81625 <= s_90_6
        fn_state.gs_81625 = s_90_6;
        // N s_90_8: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #() : ()
        let s_91_0: () = ();
        // S s_91_1: call EL2Enabled(s_91_0)
        let s_91_1: bool = EL2Enabled(state, tracer, s_91_0);
        // N s_91_2: branch s_91_1 b96 b92
        if s_91_1 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#81639 <= s_92_0
        fn_state.gs_81639 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#81639:u8
        let s_93_0: bool = fn_state.gs_81639;
        // N s_93_1: branch s_93_0 b95 b94
        if s_93_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #24u : u8
        let s_94_0: u8 = 24;
        // C s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 8u16);
        // C s_94_2: cast zx s_94_1 -> i
        let s_94_2: i128 = (s_94_1.value() as i128);
        // C s_94_3: cast reint s_94_2 -> i64
        let s_94_3: i64 = (s_94_2 as i64);
        // C s_94_4: cast zx s_94_3 -> i
        let s_94_4: i128 = (i128::try_from(s_94_3).unwrap());
        // C s_94_5: const #440u : u32
        let s_94_5: u32 = 440;
        // D s_94_6: read-reg s_94_5:u8
        let s_94_6: u8 = {
            let value = state.read_register::<u8>(s_94_5 as isize);
            tracer.read_register(s_94_5 as isize, value);
            value
        };
        // D s_94_7: call AArch64_SystemAccessTrap(s_94_6, s_94_4)
        let s_94_7: () = AArch64_SystemAccessTrap(state, tracer, s_94_6, s_94_4);
        // N s_94_8: return
        return;
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #24u : u8
        let s_95_0: u8 = 24;
        // C s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 8u16);
        // C s_95_2: cast zx s_95_1 -> i
        let s_95_2: i128 = (s_95_1.value() as i128);
        // C s_95_3: cast reint s_95_2 -> i64
        let s_95_3: i64 = (s_95_2 as i64);
        // C s_95_4: cast zx s_95_3 -> i
        let s_95_4: i128 = (i128::try_from(s_95_3).unwrap());
        // C s_95_5: const #432u : u32
        let s_95_5: u32 = 432;
        // D s_95_6: read-reg s_95_5:u8
        let s_95_6: u8 = {
            let value = state.read_register::<u8>(s_95_5 as isize);
            tracer.read_register(s_95_5 as isize, value);
            value
        };
        // D s_95_7: call AArch64_SystemAccessTrap(s_95_6, s_95_4)
        let s_95_7: () = AArch64_SystemAccessTrap(state, tracer, s_95_6, s_95_4);
        // N s_95_8: return
        return;
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var __HCR_EL2_TGE:u8
        let s_96_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 1u16);
        // C s_96_2: const #1u : u8
        let s_96_2: bool = true;
        // C s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 1u16);
        // D s_96_4: cmp-eq s_96_1 s_96_3
        let s_96_4: bool = ((s_96_1) == (s_96_3));
        // D s_96_5: write-var gs#81639 <= s_96_4
        fn_state.gs_81639 = s_96_4;
        // N s_96_6: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var __CNTKCTL_EL1_EL0PTEN:u8
        let s_97_0: bool = fn_state.u__CNTKCTL_EL1_EL0PTEN;
        // D s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 1u16);
        // C s_97_2: const #0u : u8
        let s_97_2: bool = false;
        // C s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 1u16);
        // D s_97_4: cmp-eq s_97_1 s_97_3
        let s_97_4: bool = ((s_97_1) == (s_97_3));
        // D s_97_5: write-var gs#81624 <= s_97_4
        fn_state.gs_81624 = s_97_4;
        // N s_97_6: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #102552u : u32
        let s_98_0: u32 = 102552;
        // D s_98_1: read-reg s_98_0:struct
        let s_98_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // D s_98_2: call _get_HCR_EL2_Type_E2H(s_98_1)
        let s_98_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_98_1);
        // C s_98_3: const #102552u : u32
        let s_98_3: u32 = 102552;
        // D s_98_4: read-reg s_98_3:struct
        let s_98_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_98_3 as isize);
            tracer.read_register(s_98_3 as isize, value);
            value
        };
        // D s_98_5: call _get_HCR_EL2_Type_TGE(s_98_4)
        let s_98_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_98_4);
        // D s_98_6: cast zx s_98_2 -> bv
        let s_98_6: Bits = Bits::new(s_98_2 as u128, 1u16);
        // D s_98_7: cast zx s_98_5 -> bv
        let s_98_7: Bits = Bits::new(s_98_5 as u128, 1u16);
        // D s_98_8: cast reint s_98_6 -> u128
        let s_98_8: u128 = (s_98_6.value() as u128);
        // D s_98_9: size-of s_98_6
        let s_98_9: u16 = s_98_6.length();
        // D s_98_10: cast reint s_98_7 -> u128
        let s_98_10: u128 = (s_98_7.value() as u128);
        // D s_98_11: size-of s_98_7
        let s_98_11: u16 = s_98_7.length();
        // D s_98_12: lsl s_98_8 s_98_11
        let s_98_12: u128 = s_98_8 << s_98_11;
        // D s_98_13: or s_98_12 s_98_10
        let s_98_13: u128 = ((s_98_12) | (s_98_10));
        // D s_98_14: add s_98_9 s_98_11
        let s_98_14: u16 = (s_98_9 + s_98_11);
        // D s_98_15: create-bits s_98_13 s_98_14
        let s_98_15: Bits = Bits::new(s_98_13, s_98_14);
        // D s_98_16: cast reint s_98_15 -> u8
        let s_98_16: u8 = (s_98_15.value() as u8);
        // D s_98_17: cast zx s_98_16 -> bv
        let s_98_17: Bits = Bits::new(s_98_16 as u128, 2u16);
        // C s_98_18: const #3u : u8
        let s_98_18: u8 = 3;
        // C s_98_19: cast zx s_98_18 -> bv
        let s_98_19: Bits = Bits::new(s_98_18 as u128, 2u16);
        // D s_98_20: cmp-eq s_98_17 s_98_19
        let s_98_20: bool = ((s_98_17) == (s_98_19));
        // D s_98_21: write-var gs#81623 <= s_98_20
        fn_state.gs_81623 = s_98_20;
        // N s_98_22: jump b44
        return block_44(state, tracer, fn_state);
    }
}
