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
use u_get_HCR_EL2_Type_E2H::*;
use u_get_CNTHCTL_EL2_Type_EL1PTEN::*;
use IsFeatureImplemented::*;
use NVMem_read::*;
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_HCR_EL2_Type_NV1::*;
use X_set::*;
use u_get_CNTHCTL_EL2_Type_EL1PCEN::*;
use u_get_CNTHCTL_EL2_Type_EL0PTEN::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_CNTKCTL_EL1_Type_EL0PTEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn CNTHP_CVAL_EL2_SysRegRead_ed1c928371f03392<T: Tracer>(
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
        gs_57810: bool,
        gs_57802: bool,
        gs_57821: bool,
        gs_57809: bool,
        gs_57800: bool,
        gs_57824: bool,
        ga_55761: u64,
        gs_57814: bool,
        gs_57825: bool,
        gs_57822: bool,
        gs_57823: bool,
        gs_57815: bool,
        u__PSTATE_EL: u8,
        u__CNTHCTL_EL2_EL1PCEN: bool,
        gs_57813: bool,
        u__HCR_EL2_TGE: bool,
        gs_57808: bool,
        u__CNTHCTL_EL2_EL0PTEN: bool,
        gs_57817: bool,
        gs_57801: bool,
        gs_57807: bool,
        gs_57818: bool,
        u__CNTKCTL_EL1_EL0PTEN: bool,
        gs_57819: bool,
        gs_57820: bool,
        gs_57816: bool,
        u__CNTHCTL_EL2_EL1PTEN: bool,
        gs_57806: bool,
        gs_57829: bool,
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
        // N s_0_29: branch s_0_28 b43 b1
        if s_0_28 {
            return block_43(state, tracer, fn_state);
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
        // C s_5_1: const #20800u : u32
        let s_5_1: u32 = 20800;
        // D s_5_2: read-reg s_5_1:u64
        let s_5_2: u64 = {
            let value = state.read_register::<u64>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 64u16);
        // D s_5_4: read-var t:i
        let s_5_4: i128 = fn_state.t;
        // D s_5_5: call X_set(s_5_4, s_5_0, s_5_3)
        let s_5_5: () = X_set(state, tracer, s_5_4, s_5_0, s_5_3);
        // N s_5_6: return
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
        // D s_7_1: write-var gs#57800 <= s_7_0
        fn_state.gs_57800 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#57800:u8
        let s_8_0: bool = fn_state.gs_57800;
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
        // D s_9_1: write-var gs#57801 <= s_9_0
        fn_state.gs_57801 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#57801:u8
        let s_10_0: bool = fn_state.gs_57801;
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
        // D s_12_1: write-var gs#57802 <= s_12_0
        fn_state.gs_57802 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#57802:u8
        let s_13_0: bool = fn_state.gs_57802;
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
        // C s_14_1: const #20800u : u32
        let s_14_1: u32 = 20800;
        // D s_14_2: read-reg s_14_1:u64
        let s_14_2: u64 = {
            let value = state.read_register::<u64>(s_14_1 as isize);
            tracer.read_register(s_14_1 as isize, value);
            value
        };
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 64u16);
        // D s_14_4: read-var t:i
        let s_14_4: i128 = fn_state.t;
        // D s_14_5: call X_set(s_14_4, s_14_0, s_14_3)
        let s_14_5: () = X_set(state, tracer, s_14_4, s_14_0, s_14_3);
        // N s_14_6: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // C s_15_1: const #16640u : u32
        let s_15_1: u32 = 16640;
        // D s_15_2: read-reg s_15_1:u64
        let s_15_2: u64 = {
            let value = state.read_register::<u64>(s_15_1 as isize);
            tracer.read_register(s_15_1 as isize, value);
            value
        };
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 64u16);
        // D s_15_4: read-var t:i
        let s_15_4: i128 = fn_state.t;
        // D s_15_5: call X_set(s_15_4, s_15_0, s_15_3)
        let s_15_5: () = X_set(state, tracer, s_15_4, s_15_0, s_15_3);
        // N s_15_6: return
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
        // D s_16_7: write-var gs#57802 <= s_16_6
        fn_state.gs_57802 = s_16_6;
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
        // C s_17_1: const #22672u : u32
        let s_17_1: u32 = 22672;
        // D s_17_2: read-reg s_17_1:u64
        let s_17_2: u64 = {
            let value = state.read_register::<u64>(s_17_1 as isize);
            tracer.read_register(s_17_1 as isize, value);
            value
        };
        // D s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 64u16);
        // D s_17_4: read-var t:i
        let s_17_4: i128 = fn_state.t;
        // D s_17_5: call X_set(s_17_4, s_17_0, s_17_3)
        let s_17_5: () = X_set(state, tracer, s_17_4, s_17_0, s_17_3);
        // N s_17_6: return
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
        // D s_18_2: write-var gs#57801 <= s_18_1
        fn_state.gs_57801 = s_18_1;
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
        // D s_19_7: write-var gs#57800 <= s_19_6
        fn_state.gs_57800 = s_19_6;
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
        // N s_20_2: branch s_20_1 b42 b21
        if s_20_1 {
            return block_42(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#57806 <= s_21_0
        fn_state.gs_57806 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#57806:u8
        let s_22_0: bool = fn_state.gs_57806;
        // N s_22_1: branch s_22_0 b41 b23
        if s_22_0 {
            return block_41(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#57807 <= s_23_0
        fn_state.gs_57807 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#57807:u8
        let s_24_0: bool = fn_state.gs_57807;
        // N s_24_1: branch s_24_0 b40 b25
        if s_24_0 {
            return block_40(state, tracer, fn_state);
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
        // N s_25_2: branch s_25_1 b39 b26
        if s_25_1 {
            return block_39(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#57808 <= s_26_0
        fn_state.gs_57808 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#57808:u8
        let s_27_0: bool = fn_state.gs_57808;
        // N s_27_1: branch s_27_0 b38 b28
        if s_27_0 {
            return block_38(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#57809 <= s_28_0
        fn_state.gs_57809 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#57809:u8
        let s_29_0: bool = fn_state.gs_57809;
        // N s_29_1: branch s_29_0 b37 b30
        if s_29_0 {
            return block_37(state, tracer, fn_state);
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
        // N s_30_2: branch s_30_1 b36 b31
        if s_30_1 {
            return block_36(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#57810 <= s_31_0
        fn_state.gs_57810 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#57810:u8
        let s_32_0: bool = fn_state.gs_57810;
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
        // C s_33_1: const #20800u : u32
        let s_33_1: u32 = 20800;
        // D s_33_2: read-reg s_33_1:u64
        let s_33_2: u64 = {
            let value = state.read_register::<u64>(s_33_1 as isize);
            tracer.read_register(s_33_1 as isize, value);
            value
        };
        // D s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 64u16);
        // D s_33_4: read-var t:i
        let s_33_4: i128 = fn_state.t;
        // D s_33_5: call X_set(s_33_4, s_33_0, s_33_3)
        let s_33_5: () = X_set(state, tracer, s_33_4, s_33_0, s_33_3);
        // N s_33_6: return
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
        // C s_34_4: cast zx s_34_3 -> i
        let s_34_4: i128 = (i128::try_from(s_34_3).unwrap());
        // S s_34_5: call NVMem_read(s_34_4)
        let s_34_5: u64 = NVMem_read(state, tracer, s_34_4);
        // D s_34_6: write-var ga#55761 <= s_34_5
        fn_state.ga_55761 = s_34_5;
        // N s_34_7: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var ga#55761:u64
        let s_35_0: u64 = fn_state.ga_55761;
        // D s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 64u16);
        // D s_35_2: read-var t:i
        let s_35_2: i128 = fn_state.t;
        // C s_35_3: const #64s : i64
        let s_35_3: i64 = 64;
        // D s_35_4: call X_set(s_35_2, s_35_3, s_35_1)
        let s_35_4: () = X_set(state, tracer, s_35_2, s_35_3, s_35_1);
        // N s_35_5: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #102552u : u32
        let s_36_0: u32 = 102552;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_HCR_EL2_Type_NV2(s_36_1)
        let s_36_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_36_1);
        // C s_36_3: const #102552u : u32
        let s_36_3: u32 = 102552;
        // D s_36_4: read-reg s_36_3:struct
        let s_36_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_3 as isize);
            tracer.read_register(s_36_3 as isize, value);
            value
        };
        // D s_36_5: call _get_HCR_EL2_Type_NV1(s_36_4)
        let s_36_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_36_4);
        // C s_36_6: const #102552u : u32
        let s_36_6: u32 = 102552;
        // D s_36_7: read-reg s_36_6:struct
        let s_36_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_6 as isize);
            tracer.read_register(s_36_6 as isize, value);
            value
        };
        // D s_36_8: call _get_HCR_EL2_Type_NV(s_36_7)
        let s_36_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_36_7);
        // D s_36_9: cast zx s_36_5 -> bv
        let s_36_9: Bits = Bits::new(s_36_5 as u128, 1u16);
        // D s_36_10: cast zx s_36_8 -> bv
        let s_36_10: Bits = Bits::new(s_36_8 as u128, 1u16);
        // D s_36_11: cast reint s_36_9 -> u128
        let s_36_11: u128 = (s_36_9.value() as u128);
        // D s_36_12: size-of s_36_9
        let s_36_12: u16 = s_36_9.length();
        // D s_36_13: cast reint s_36_10 -> u128
        let s_36_13: u128 = (s_36_10.value() as u128);
        // D s_36_14: size-of s_36_10
        let s_36_14: u16 = s_36_10.length();
        // D s_36_15: lsl s_36_11 s_36_14
        let s_36_15: u128 = s_36_11 << s_36_14;
        // D s_36_16: or s_36_15 s_36_13
        let s_36_16: u128 = ((s_36_15) | (s_36_13));
        // D s_36_17: add s_36_12 s_36_14
        let s_36_17: u16 = (s_36_12 + s_36_14);
        // D s_36_18: create-bits s_36_16 s_36_17
        let s_36_18: Bits = Bits::new(s_36_16, s_36_17);
        // D s_36_19: cast reint s_36_18 -> u8
        let s_36_19: u8 = (s_36_18.value() as u8);
        // D s_36_20: cast zx s_36_2 -> bv
        let s_36_20: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_21: cast zx s_36_19 -> bv
        let s_36_21: Bits = Bits::new(s_36_19 as u128, 2u16);
        // D s_36_22: cast reint s_36_20 -> u128
        let s_36_22: u128 = (s_36_20.value() as u128);
        // D s_36_23: size-of s_36_20
        let s_36_23: u16 = s_36_20.length();
        // D s_36_24: cast reint s_36_21 -> u128
        let s_36_24: u128 = (s_36_21.value() as u128);
        // D s_36_25: size-of s_36_21
        let s_36_25: u16 = s_36_21.length();
        // D s_36_26: lsl s_36_22 s_36_25
        let s_36_26: u128 = s_36_22 << s_36_25;
        // D s_36_27: or s_36_26 s_36_24
        let s_36_27: u128 = ((s_36_26) | (s_36_24));
        // D s_36_28: add s_36_23 s_36_25
        let s_36_28: u16 = (s_36_23 + s_36_25);
        // D s_36_29: create-bits s_36_27 s_36_28
        let s_36_29: Bits = Bits::new(s_36_27, s_36_28);
        // D s_36_30: cast reint s_36_29 -> u8
        let s_36_30: u8 = (s_36_29.value() as u8);
        // D s_36_31: cast zx s_36_30 -> bv
        let s_36_31: Bits = Bits::new(s_36_30 as u128, 3u16);
        // C s_36_32: const #7u : u8
        let s_36_32: u8 = 7;
        // C s_36_33: cast zx s_36_32 -> bv
        let s_36_33: Bits = Bits::new(s_36_32 as u128, 3u16);
        // D s_36_34: cmp-eq s_36_31 s_36_33
        let s_36_34: bool = ((s_36_31) == (s_36_33));
        // D s_36_35: write-var gs#57810 <= s_36_34
        fn_state.gs_57810 = s_36_34;
        // N s_36_36: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #24u : u8
        let s_37_0: u8 = 24;
        // C s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 8u16);
        // C s_37_2: cast zx s_37_1 -> i
        let s_37_2: i128 = (s_37_1.value() as i128);
        // C s_37_3: cast reint s_37_2 -> i64
        let s_37_3: i64 = (s_37_2 as i64);
        // C s_37_4: cast zx s_37_3 -> i
        let s_37_4: i128 = (i128::try_from(s_37_3).unwrap());
        // C s_37_5: const #432u : u32
        let s_37_5: u32 = 432;
        // D s_37_6: read-reg s_37_5:u8
        let s_37_6: u8 = {
            let value = state.read_register::<u8>(s_37_5 as isize);
            tracer.read_register(s_37_5 as isize, value);
            value
        };
        // D s_37_7: call AArch64_SystemAccessTrap(s_37_6, s_37_4)
        let s_37_7: () = AArch64_SystemAccessTrap(state, tracer, s_37_6, s_37_4);
        // N s_37_8: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_38_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // C s_38_2: const #0u : u8
        let s_38_2: bool = false;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: write-var gs#57809 <= s_38_4
        fn_state.gs_57809 = s_38_4;
        // N s_38_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #102552u : u32
        let s_39_0: u32 = 102552;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_HCR_EL2_Type_E2H(s_39_1)
        let s_39_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_39_1);
        // D s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #1u : u8
        let s_39_4: bool = true;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // D s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // D s_39_7: write-var gs#57808 <= s_39_6
        fn_state.gs_57808 = s_39_6;
        // N s_39_8: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #24u : u8
        let s_40_0: u8 = 24;
        // C s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 8u16);
        // C s_40_2: cast zx s_40_1 -> i
        let s_40_2: i128 = (s_40_1.value() as i128);
        // C s_40_3: cast reint s_40_2 -> i64
        let s_40_3: i64 = (s_40_2 as i64);
        // C s_40_4: cast zx s_40_3 -> i
        let s_40_4: i128 = (i128::try_from(s_40_3).unwrap());
        // C s_40_5: const #432u : u32
        let s_40_5: u32 = 432;
        // D s_40_6: read-reg s_40_5:u8
        let s_40_6: u8 = {
            let value = state.read_register::<u8>(s_40_5 as isize);
            tracer.read_register(s_40_5 as isize, value);
            value
        };
        // D s_40_7: call AArch64_SystemAccessTrap(s_40_6, s_40_4)
        let s_40_7: () = AArch64_SystemAccessTrap(state, tracer, s_40_6, s_40_4);
        // N s_40_8: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_41_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 1u16);
        // C s_41_2: const #0u : u8
        let s_41_2: bool = false;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: write-var gs#57807 <= s_41_4
        fn_state.gs_57807 = s_41_4;
        // N s_41_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #102552u : u32
        let s_42_0: u32 = 102552;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_HCR_EL2_Type_E2H(s_42_1)
        let s_42_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_42_1);
        // D s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // C s_42_4: const #0u : u8
        let s_42_4: bool = false;
        // C s_42_5: cast zx s_42_4 -> bv
        let s_42_5: Bits = Bits::new(s_42_4 as u128, 1u16);
        // D s_42_6: cmp-eq s_42_3 s_42_5
        let s_42_6: bool = ((s_42_3) == (s_42_5));
        // D s_42_7: write-var gs#57806 <= s_42_6
        fn_state.gs_57806 = s_42_6;
        // N s_42_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call EL2Enabled(s_43_0)
        let s_43_1: bool = EL2Enabled(state, tracer, s_43_0);
        // N s_43_2: branch s_43_1 b99 b44
        if s_43_1 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#57813 <= s_44_0
        fn_state.gs_57813 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#57813:u8
        let s_45_0: bool = fn_state.gs_57813;
        // D s_45_1: not s_45_0
        let s_45_1: bool = !s_45_0;
        // N s_45_2: branch s_45_1 b98 b46
        if s_45_1 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#57814 <= s_46_0
        fn_state.gs_57814 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#57814:u8
        let s_47_0: bool = fn_state.gs_57814;
        // N s_47_1: branch s_47_0 b92 b48
        if s_47_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EL2Enabled(s_48_0)
        let s_48_1: bool = EL2Enabled(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b91 b49
        if s_48_1 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#57815 <= s_49_0
        fn_state.gs_57815 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#57815:u8
        let s_50_0: bool = fn_state.gs_57815;
        // N s_50_1: branch s_50_0 b90 b51
        if s_50_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#57816 <= s_51_0
        fn_state.gs_57816 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#57816:u8
        let s_52_0: bool = fn_state.gs_57816;
        // N s_52_1: branch s_52_0 b89 b53
        if s_52_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EL2Enabled(s_53_0)
        let s_53_1: bool = EL2Enabled(state, tracer, s_53_0);
        // N s_53_2: branch s_53_1 b88 b54
        if s_53_1 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#57817 <= s_54_0
        fn_state.gs_57817 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#57817:u8
        let s_55_0: bool = fn_state.gs_57817;
        // N s_55_1: branch s_55_0 b87 b56
        if s_55_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#57818 <= s_56_0
        fn_state.gs_57818 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#57818:u8
        let s_57_0: bool = fn_state.gs_57818;
        // N s_57_1: branch s_57_0 b86 b58
        if s_57_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call EL2Enabled(s_58_0)
        let s_58_1: bool = EL2Enabled(state, tracer, s_58_0);
        // N s_58_2: branch s_58_1 b85 b59
        if s_58_1 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#57819 <= s_59_0
        fn_state.gs_57819 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#57819:u8
        let s_60_0: bool = fn_state.gs_57819;
        // N s_60_1: branch s_60_0 b84 b61
        if s_60_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#57820 <= s_61_0
        fn_state.gs_57820 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#57820:u8
        let s_62_0: bool = fn_state.gs_57820;
        // N s_62_1: branch s_62_0 b83 b63
        if s_62_0 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call EL2Enabled(s_63_0)
        let s_63_1: bool = EL2Enabled(state, tracer, s_63_0);
        // N s_63_2: branch s_63_1 b82 b64
        if s_63_1 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #0u : u8
        let s_64_0: bool = false;
        // D s_64_1: write-var gs#57821 <= s_64_0
        fn_state.gs_57821 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#57821:u8
        let s_65_0: bool = fn_state.gs_57821;
        // N s_65_1: branch s_65_0 b81 b66
        if s_65_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // D s_66_1: write-var gs#57822 <= s_66_0
        fn_state.gs_57822 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#57822:u8
        let s_67_0: bool = fn_state.gs_57822;
        // N s_67_1: branch s_67_0 b80 b68
        if s_67_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#57823 <= s_68_0
        fn_state.gs_57823 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#57823:u8
        let s_69_0: bool = fn_state.gs_57823;
        // N s_69_1: branch s_69_0 b79 b70
        if s_69_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call EL2Enabled(s_70_0)
        let s_70_1: bool = EL2Enabled(state, tracer, s_70_0);
        // N s_70_2: branch s_70_1 b78 b71
        if s_70_1 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#57824 <= s_71_0
        fn_state.gs_57824 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#57824:u8
        let s_72_0: bool = fn_state.gs_57824;
        // N s_72_1: branch s_72_0 b77 b73
        if s_72_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#57825 <= s_73_0
        fn_state.gs_57825 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#57825:u8
        let s_74_0: bool = fn_state.gs_57825;
        // N s_74_1: branch s_74_0 b76 b75
        if s_74_0 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #64s : i64
        let s_75_0: i64 = 64;
        // C s_75_1: const #20800u : u32
        let s_75_1: u32 = 20800;
        // D s_75_2: read-reg s_75_1:u64
        let s_75_2: u64 = {
            let value = state.read_register::<u64>(s_75_1 as isize);
            tracer.read_register(s_75_1 as isize, value);
            value
        };
        // D s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 64u16);
        // D s_75_4: read-var t:i
        let s_75_4: i128 = fn_state.t;
        // D s_75_5: call X_set(s_75_4, s_75_0, s_75_3)
        let s_75_5: () = X_set(state, tracer, s_75_4, s_75_0, s_75_3);
        // N s_75_6: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #64s : i64
        let s_76_0: i64 = 64;
        // C s_76_1: const #16640u : u32
        let s_76_1: u32 = 16640;
        // D s_76_2: read-reg s_76_1:u64
        let s_76_2: u64 = {
            let value = state.read_register::<u64>(s_76_1 as isize);
            tracer.read_register(s_76_1 as isize, value);
            value
        };
        // D s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 64u16);
        // D s_76_4: read-var t:i
        let s_76_4: i128 = fn_state.t;
        // D s_76_5: call X_set(s_76_4, s_76_0, s_76_3)
        let s_76_5: () = X_set(state, tracer, s_76_4, s_76_0, s_76_3);
        // N s_76_6: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #90704u : u32
        let s_77_0: u32 = 90704;
        // D s_77_1: read-reg s_77_0:struct
        let s_77_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // D s_77_2: call _get_SCR_EL3_Type_NS(s_77_1)
        let s_77_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_77_1);
        // D s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // C s_77_4: const #1u : u8
        let s_77_4: bool = true;
        // C s_77_5: cast zx s_77_4 -> bv
        let s_77_5: Bits = Bits::new(s_77_4 as u128, 1u16);
        // D s_77_6: cmp-eq s_77_3 s_77_5
        let s_77_6: bool = ((s_77_3) == (s_77_5));
        // D s_77_7: write-var gs#57825 <= s_77_6
        fn_state.gs_57825 = s_77_6;
        // N s_77_8: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #102552u : u32
        let s_78_0: u32 = 102552;
        // D s_78_1: read-reg s_78_0:struct
        let s_78_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call _get_HCR_EL2_Type_E2H(s_78_1)
        let s_78_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_78_1);
        // C s_78_3: const #102552u : u32
        let s_78_3: u32 = 102552;
        // D s_78_4: read-reg s_78_3:struct
        let s_78_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_3 as isize);
            tracer.read_register(s_78_3 as isize, value);
            value
        };
        // D s_78_5: call _get_HCR_EL2_Type_TGE(s_78_4)
        let s_78_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_78_4);
        // D s_78_6: cast zx s_78_2 -> bv
        let s_78_6: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_7: cast zx s_78_5 -> bv
        let s_78_7: Bits = Bits::new(s_78_5 as u128, 1u16);
        // D s_78_8: cast reint s_78_6 -> u128
        let s_78_8: u128 = (s_78_6.value() as u128);
        // D s_78_9: size-of s_78_6
        let s_78_9: u16 = s_78_6.length();
        // D s_78_10: cast reint s_78_7 -> u128
        let s_78_10: u128 = (s_78_7.value() as u128);
        // D s_78_11: size-of s_78_7
        let s_78_11: u16 = s_78_7.length();
        // D s_78_12: lsl s_78_8 s_78_11
        let s_78_12: u128 = s_78_8 << s_78_11;
        // D s_78_13: or s_78_12 s_78_10
        let s_78_13: u128 = ((s_78_12) | (s_78_10));
        // D s_78_14: add s_78_9 s_78_11
        let s_78_14: u16 = (s_78_9 + s_78_11);
        // D s_78_15: create-bits s_78_13 s_78_14
        let s_78_15: Bits = Bits::new(s_78_13, s_78_14);
        // D s_78_16: cast reint s_78_15 -> u8
        let s_78_16: u8 = (s_78_15.value() as u8);
        // D s_78_17: cast zx s_78_16 -> bv
        let s_78_17: Bits = Bits::new(s_78_16 as u128, 2u16);
        // C s_78_18: const #3u : u8
        let s_78_18: u8 = 3;
        // C s_78_19: cast zx s_78_18 -> bv
        let s_78_19: Bits = Bits::new(s_78_18 as u128, 2u16);
        // D s_78_20: cmp-eq s_78_17 s_78_19
        let s_78_20: bool = ((s_78_17) == (s_78_19));
        // D s_78_21: write-var gs#57824 <= s_78_20
        fn_state.gs_57824 = s_78_20;
        // N s_78_22: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #64s : i64
        let s_79_0: i64 = 64;
        // C s_79_1: const #22672u : u32
        let s_79_1: u32 = 22672;
        // D s_79_2: read-reg s_79_1:u64
        let s_79_2: u64 = {
            let value = state.read_register::<u64>(s_79_1 as isize);
            tracer.read_register(s_79_1 as isize, value);
            value
        };
        // D s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 64u16);
        // D s_79_4: read-var t:i
        let s_79_4: i128 = fn_state.t;
        // D s_79_5: call X_set(s_79_4, s_79_0, s_79_3)
        let s_79_5: () = X_set(state, tracer, s_79_4, s_79_0, s_79_3);
        // N s_79_6: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #117u : u32
        let s_80_0: u32 = 117;
        // S s_80_1: call IsFeatureImplemented(s_80_0)
        let s_80_1: bool = IsFeatureImplemented(state, tracer, s_80_0);
        // D s_80_2: write-var gs#57823 <= s_80_1
        fn_state.gs_57823 = s_80_1;
        // N s_80_3: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #90704u : u32
        let s_81_0: u32 = 90704;
        // D s_81_1: read-reg s_81_0:struct
        let s_81_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call _get_SCR_EL3_Type_NS(s_81_1)
        let s_81_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_81_1);
        // D s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // C s_81_4: const #0u : u8
        let s_81_4: bool = false;
        // C s_81_5: cast zx s_81_4 -> bv
        let s_81_5: Bits = Bits::new(s_81_4 as u128, 1u16);
        // D s_81_6: cmp-eq s_81_3 s_81_5
        let s_81_6: bool = ((s_81_3) == (s_81_5));
        // D s_81_7: write-var gs#57822 <= s_81_6
        fn_state.gs_57822 = s_81_6;
        // N s_81_8: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #102552u : u32
        let s_82_0: u32 = 102552;
        // D s_82_1: read-reg s_82_0:struct
        let s_82_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call _get_HCR_EL2_Type_E2H(s_82_1)
        let s_82_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_82_1);
        // C s_82_3: const #102552u : u32
        let s_82_3: u32 = 102552;
        // D s_82_4: read-reg s_82_3:struct
        let s_82_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_3 as isize);
            tracer.read_register(s_82_3 as isize, value);
            value
        };
        // D s_82_5: call _get_HCR_EL2_Type_TGE(s_82_4)
        let s_82_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_82_4);
        // D s_82_6: cast zx s_82_2 -> bv
        let s_82_6: Bits = Bits::new(s_82_2 as u128, 1u16);
        // D s_82_7: cast zx s_82_5 -> bv
        let s_82_7: Bits = Bits::new(s_82_5 as u128, 1u16);
        // D s_82_8: cast reint s_82_6 -> u128
        let s_82_8: u128 = (s_82_6.value() as u128);
        // D s_82_9: size-of s_82_6
        let s_82_9: u16 = s_82_6.length();
        // D s_82_10: cast reint s_82_7 -> u128
        let s_82_10: u128 = (s_82_7.value() as u128);
        // D s_82_11: size-of s_82_7
        let s_82_11: u16 = s_82_7.length();
        // D s_82_12: lsl s_82_8 s_82_11
        let s_82_12: u128 = s_82_8 << s_82_11;
        // D s_82_13: or s_82_12 s_82_10
        let s_82_13: u128 = ((s_82_12) | (s_82_10));
        // D s_82_14: add s_82_9 s_82_11
        let s_82_14: u16 = (s_82_9 + s_82_11);
        // D s_82_15: create-bits s_82_13 s_82_14
        let s_82_15: Bits = Bits::new(s_82_13, s_82_14);
        // D s_82_16: cast reint s_82_15 -> u8
        let s_82_16: u8 = (s_82_15.value() as u8);
        // D s_82_17: cast zx s_82_16 -> bv
        let s_82_17: Bits = Bits::new(s_82_16 as u128, 2u16);
        // C s_82_18: const #3u : u8
        let s_82_18: u8 = 3;
        // C s_82_19: cast zx s_82_18 -> bv
        let s_82_19: Bits = Bits::new(s_82_18 as u128, 2u16);
        // D s_82_20: cmp-eq s_82_17 s_82_19
        let s_82_20: bool = ((s_82_17) == (s_82_19));
        // D s_82_21: write-var gs#57821 <= s_82_20
        fn_state.gs_57821 = s_82_20;
        // N s_82_22: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #24u : u8
        let s_83_0: u8 = 24;
        // C s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 8u16);
        // C s_83_2: cast zx s_83_1 -> i
        let s_83_2: i128 = (s_83_1.value() as i128);
        // C s_83_3: cast reint s_83_2 -> i64
        let s_83_3: i64 = (s_83_2 as i64);
        // C s_83_4: cast zx s_83_3 -> i
        let s_83_4: i128 = (i128::try_from(s_83_3).unwrap());
        // C s_83_5: const #432u : u32
        let s_83_5: u32 = 432;
        // D s_83_6: read-reg s_83_5:u8
        let s_83_6: u8 = {
            let value = state.read_register::<u8>(s_83_5 as isize);
            tracer.read_register(s_83_5 as isize, value);
            value
        };
        // D s_83_7: call AArch64_SystemAccessTrap(s_83_6, s_83_4)
        let s_83_7: () = AArch64_SystemAccessTrap(state, tracer, s_83_6, s_83_4);
        // N s_83_8: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __CNTHCTL_EL2_EL0PTEN:u8
        let s_84_0: bool = fn_state.u__CNTHCTL_EL2_EL0PTEN;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #0u : u8
        let s_84_2: bool = false;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // D s_84_5: write-var gs#57820 <= s_84_4
        fn_state.gs_57820 = s_84_4;
        // N s_84_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #102552u : u32
        let s_85_0: u32 = 102552;
        // D s_85_1: read-reg s_85_0:struct
        let s_85_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call _get_HCR_EL2_Type_E2H(s_85_1)
        let s_85_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_85_1);
        // C s_85_3: const #102552u : u32
        let s_85_3: u32 = 102552;
        // D s_85_4: read-reg s_85_3:struct
        let s_85_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_85_3 as isize);
            tracer.read_register(s_85_3 as isize, value);
            value
        };
        // D s_85_5: call _get_HCR_EL2_Type_TGE(s_85_4)
        let s_85_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_85_4);
        // D s_85_6: cast zx s_85_2 -> bv
        let s_85_6: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_7: cast zx s_85_5 -> bv
        let s_85_7: Bits = Bits::new(s_85_5 as u128, 1u16);
        // D s_85_8: cast reint s_85_6 -> u128
        let s_85_8: u128 = (s_85_6.value() as u128);
        // D s_85_9: size-of s_85_6
        let s_85_9: u16 = s_85_6.length();
        // D s_85_10: cast reint s_85_7 -> u128
        let s_85_10: u128 = (s_85_7.value() as u128);
        // D s_85_11: size-of s_85_7
        let s_85_11: u16 = s_85_7.length();
        // D s_85_12: lsl s_85_8 s_85_11
        let s_85_12: u128 = s_85_8 << s_85_11;
        // D s_85_13: or s_85_12 s_85_10
        let s_85_13: u128 = ((s_85_12) | (s_85_10));
        // D s_85_14: add s_85_9 s_85_11
        let s_85_14: u16 = (s_85_9 + s_85_11);
        // D s_85_15: create-bits s_85_13 s_85_14
        let s_85_15: Bits = Bits::new(s_85_13, s_85_14);
        // D s_85_16: cast reint s_85_15 -> u8
        let s_85_16: u8 = (s_85_15.value() as u8);
        // D s_85_17: cast zx s_85_16 -> bv
        let s_85_17: Bits = Bits::new(s_85_16 as u128, 2u16);
        // C s_85_18: const #3u : u8
        let s_85_18: u8 = 3;
        // C s_85_19: cast zx s_85_18 -> bv
        let s_85_19: Bits = Bits::new(s_85_18 as u128, 2u16);
        // D s_85_20: cmp-eq s_85_17 s_85_19
        let s_85_20: bool = ((s_85_17) == (s_85_19));
        // D s_85_21: write-var gs#57819 <= s_85_20
        fn_state.gs_57819 = s_85_20;
        // N s_85_22: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #24u : u8
        let s_86_0: u8 = 24;
        // C s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 8u16);
        // C s_86_2: cast zx s_86_1 -> i
        let s_86_2: i128 = (s_86_1.value() as i128);
        // C s_86_3: cast reint s_86_2 -> i64
        let s_86_3: i64 = (s_86_2 as i64);
        // C s_86_4: cast zx s_86_3 -> i
        let s_86_4: i128 = (i128::try_from(s_86_3).unwrap());
        // C s_86_5: const #432u : u32
        let s_86_5: u32 = 432;
        // D s_86_6: read-reg s_86_5:u8
        let s_86_6: u8 = {
            let value = state.read_register::<u8>(s_86_5 as isize);
            tracer.read_register(s_86_5 as isize, value);
            value
        };
        // D s_86_7: call AArch64_SystemAccessTrap(s_86_6, s_86_4)
        let s_86_7: () = AArch64_SystemAccessTrap(state, tracer, s_86_6, s_86_4);
        // N s_86_8: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_87_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 1u16);
        // C s_87_2: const #0u : u8
        let s_87_2: bool = false;
        // C s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 1u16);
        // D s_87_4: cmp-eq s_87_1 s_87_3
        let s_87_4: bool = ((s_87_1) == (s_87_3));
        // D s_87_5: write-var gs#57818 <= s_87_4
        fn_state.gs_57818 = s_87_4;
        // N s_87_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #102552u : u32
        let s_88_0: u32 = 102552;
        // D s_88_1: read-reg s_88_0:struct
        let s_88_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_88_0 as isize);
            tracer.read_register(s_88_0 as isize, value);
            value
        };
        // D s_88_2: call _get_HCR_EL2_Type_E2H(s_88_1)
        let s_88_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_88_1);
        // C s_88_3: const #102552u : u32
        let s_88_3: u32 = 102552;
        // D s_88_4: read-reg s_88_3:struct
        let s_88_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_88_3 as isize);
            tracer.read_register(s_88_3 as isize, value);
            value
        };
        // D s_88_5: call _get_HCR_EL2_Type_TGE(s_88_4)
        let s_88_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_88_4);
        // D s_88_6: cast zx s_88_2 -> bv
        let s_88_6: Bits = Bits::new(s_88_2 as u128, 1u16);
        // D s_88_7: cast zx s_88_5 -> bv
        let s_88_7: Bits = Bits::new(s_88_5 as u128, 1u16);
        // D s_88_8: cast reint s_88_6 -> u128
        let s_88_8: u128 = (s_88_6.value() as u128);
        // D s_88_9: size-of s_88_6
        let s_88_9: u16 = s_88_6.length();
        // D s_88_10: cast reint s_88_7 -> u128
        let s_88_10: u128 = (s_88_7.value() as u128);
        // D s_88_11: size-of s_88_7
        let s_88_11: u16 = s_88_7.length();
        // D s_88_12: lsl s_88_8 s_88_11
        let s_88_12: u128 = s_88_8 << s_88_11;
        // D s_88_13: or s_88_12 s_88_10
        let s_88_13: u128 = ((s_88_12) | (s_88_10));
        // D s_88_14: add s_88_9 s_88_11
        let s_88_14: u16 = (s_88_9 + s_88_11);
        // D s_88_15: create-bits s_88_13 s_88_14
        let s_88_15: Bits = Bits::new(s_88_13, s_88_14);
        // D s_88_16: cast reint s_88_15 -> u8
        let s_88_16: u8 = (s_88_15.value() as u8);
        // D s_88_17: cast zx s_88_16 -> bv
        let s_88_17: Bits = Bits::new(s_88_16 as u128, 2u16);
        // C s_88_18: const #2u : u8
        let s_88_18: u8 = 2;
        // C s_88_19: cast zx s_88_18 -> bv
        let s_88_19: Bits = Bits::new(s_88_18 as u128, 2u16);
        // D s_88_20: cmp-eq s_88_17 s_88_19
        let s_88_20: bool = ((s_88_17) == (s_88_19));
        // D s_88_21: write-var gs#57817 <= s_88_20
        fn_state.gs_57817 = s_88_20;
        // N s_88_22: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #24u : u8
        let s_89_0: u8 = 24;
        // C s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 8u16);
        // C s_89_2: cast zx s_89_1 -> i
        let s_89_2: i128 = (s_89_1.value() as i128);
        // C s_89_3: cast reint s_89_2 -> i64
        let s_89_3: i64 = (s_89_2 as i64);
        // C s_89_4: cast zx s_89_3 -> i
        let s_89_4: i128 = (i128::try_from(s_89_3).unwrap());
        // C s_89_5: const #432u : u32
        let s_89_5: u32 = 432;
        // D s_89_6: read-reg s_89_5:u8
        let s_89_6: u8 = {
            let value = state.read_register::<u8>(s_89_5 as isize);
            tracer.read_register(s_89_5 as isize, value);
            value
        };
        // D s_89_7: call AArch64_SystemAccessTrap(s_89_6, s_89_4)
        let s_89_7: () = AArch64_SystemAccessTrap(state, tracer, s_89_6, s_89_4);
        // N s_89_8: return
        return;
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_90_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 1u16);
        // C s_90_2: const #0u : u8
        let s_90_2: bool = false;
        // C s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // D s_90_5: write-var gs#57816 <= s_90_4
        fn_state.gs_57816 = s_90_4;
        // N s_90_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #102552u : u32
        let s_91_0: u32 = 102552;
        // D s_91_1: read-reg s_91_0:struct
        let s_91_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // D s_91_2: call _get_HCR_EL2_Type_E2H(s_91_1)
        let s_91_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_91_1);
        // D s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // C s_91_4: const #0u : u8
        let s_91_4: bool = false;
        // C s_91_5: cast zx s_91_4 -> bv
        let s_91_5: Bits = Bits::new(s_91_4 as u128, 1u16);
        // D s_91_6: cmp-eq s_91_3 s_91_5
        let s_91_6: bool = ((s_91_3) == (s_91_5));
        // D s_91_7: write-var gs#57815 <= s_91_6
        fn_state.gs_57815 = s_91_6;
        // N s_91_8: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #() : ()
        let s_92_0: () = ();
        // S s_92_1: call EL2Enabled(s_92_0)
        let s_92_1: bool = EL2Enabled(state, tracer, s_92_0);
        // N s_92_2: branch s_92_1 b97 b93
        if s_92_1 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var gs#57829 <= s_93_0
        fn_state.gs_57829 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#57829:u8
        let s_94_0: bool = fn_state.gs_57829;
        // N s_94_1: branch s_94_0 b96 b95
        if s_94_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
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
        // C s_95_5: const #440u : u32
        let s_95_5: u32 = 440;
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
        // C s_96_0: const #24u : u8
        let s_96_0: u8 = 24;
        // C s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 8u16);
        // C s_96_2: cast zx s_96_1 -> i
        let s_96_2: i128 = (s_96_1.value() as i128);
        // C s_96_3: cast reint s_96_2 -> i64
        let s_96_3: i64 = (s_96_2 as i64);
        // C s_96_4: cast zx s_96_3 -> i
        let s_96_4: i128 = (i128::try_from(s_96_3).unwrap());
        // C s_96_5: const #432u : u32
        let s_96_5: u32 = 432;
        // D s_96_6: read-reg s_96_5:u8
        let s_96_6: u8 = {
            let value = state.read_register::<u8>(s_96_5 as isize);
            tracer.read_register(s_96_5 as isize, value);
            value
        };
        // D s_96_7: call AArch64_SystemAccessTrap(s_96_6, s_96_4)
        let s_96_7: () = AArch64_SystemAccessTrap(state, tracer, s_96_6, s_96_4);
        // N s_96_8: return
        return;
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var __HCR_EL2_TGE:u8
        let s_97_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 1u16);
        // C s_97_2: const #1u : u8
        let s_97_2: bool = true;
        // C s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 1u16);
        // D s_97_4: cmp-eq s_97_1 s_97_3
        let s_97_4: bool = ((s_97_1) == (s_97_3));
        // D s_97_5: write-var gs#57829 <= s_97_4
        fn_state.gs_57829 = s_97_4;
        // N s_97_6: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var __CNTKCTL_EL1_EL0PTEN:u8
        let s_98_0: bool = fn_state.u__CNTKCTL_EL1_EL0PTEN;
        // D s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 1u16);
        // C s_98_2: const #0u : u8
        let s_98_2: bool = false;
        // C s_98_3: cast zx s_98_2 -> bv
        let s_98_3: Bits = Bits::new(s_98_2 as u128, 1u16);
        // D s_98_4: cmp-eq s_98_1 s_98_3
        let s_98_4: bool = ((s_98_1) == (s_98_3));
        // D s_98_5: write-var gs#57814 <= s_98_4
        fn_state.gs_57814 = s_98_4;
        // N s_98_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #102552u : u32
        let s_99_0: u32 = 102552;
        // D s_99_1: read-reg s_99_0:struct
        let s_99_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // D s_99_2: call _get_HCR_EL2_Type_E2H(s_99_1)
        let s_99_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_99_1);
        // C s_99_3: const #102552u : u32
        let s_99_3: u32 = 102552;
        // D s_99_4: read-reg s_99_3:struct
        let s_99_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_99_3 as isize);
            tracer.read_register(s_99_3 as isize, value);
            value
        };
        // D s_99_5: call _get_HCR_EL2_Type_TGE(s_99_4)
        let s_99_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_99_4);
        // D s_99_6: cast zx s_99_2 -> bv
        let s_99_6: Bits = Bits::new(s_99_2 as u128, 1u16);
        // D s_99_7: cast zx s_99_5 -> bv
        let s_99_7: Bits = Bits::new(s_99_5 as u128, 1u16);
        // D s_99_8: cast reint s_99_6 -> u128
        let s_99_8: u128 = (s_99_6.value() as u128);
        // D s_99_9: size-of s_99_6
        let s_99_9: u16 = s_99_6.length();
        // D s_99_10: cast reint s_99_7 -> u128
        let s_99_10: u128 = (s_99_7.value() as u128);
        // D s_99_11: size-of s_99_7
        let s_99_11: u16 = s_99_7.length();
        // D s_99_12: lsl s_99_8 s_99_11
        let s_99_12: u128 = s_99_8 << s_99_11;
        // D s_99_13: or s_99_12 s_99_10
        let s_99_13: u128 = ((s_99_12) | (s_99_10));
        // D s_99_14: add s_99_9 s_99_11
        let s_99_14: u16 = (s_99_9 + s_99_11);
        // D s_99_15: create-bits s_99_13 s_99_14
        let s_99_15: Bits = Bits::new(s_99_13, s_99_14);
        // D s_99_16: cast reint s_99_15 -> u8
        let s_99_16: u8 = (s_99_15.value() as u8);
        // D s_99_17: cast zx s_99_16 -> bv
        let s_99_17: Bits = Bits::new(s_99_16 as u128, 2u16);
        // C s_99_18: const #3u : u8
        let s_99_18: u8 = 3;
        // C s_99_19: cast zx s_99_18 -> bv
        let s_99_19: Bits = Bits::new(s_99_18 as u128, 2u16);
        // D s_99_20: cmp-eq s_99_17 s_99_19
        let s_99_20: bool = ((s_99_17) == (s_99_19));
        // D s_99_21: write-var gs#57813 <= s_99_20
        fn_state.gs_57813 = s_99_20;
        // N s_99_22: jump b45
        return block_45(state, tracer, fn_state);
    }
}
