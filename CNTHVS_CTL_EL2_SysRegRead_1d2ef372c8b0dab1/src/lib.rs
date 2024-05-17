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
use NVMem_read::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_HCR_EL2_Type_NV1::*;
use u_get_CNTHCTL_EL2_Type_EL1TVT::*;
use X_set::*;
use u__get_CNTHV_CTL_EL2::*;
use u__get_CNTV_CTL_EL0::*;
use u_get_HCR_EL2_Type_NV::*;
use u__get_CNTHVS_CTL_EL2::*;
use u_get_CNTHCTL_EL2_Type_EL0VTEN::*;
use u_get_CNTKCTL_EL1_Type_EL0VTEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn CNTHVS_CTL_EL2_SysRegRead_1d2ef372c8b0dab1<T: Tracer>(
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
        gs_57850: bool,
        ga_55861: ProductType5c790c8ef59cc8b2,
        gs_57862: bool,
        ga_55901: ProductType5c790c8ef59cc8b2,
        u__HCR_EL2_TGE: bool,
        gs_57845: bool,
        gs_57839: bool,
        ga_55887: ProductType5c790c8ef59cc8b2,
        gs_57856: bool,
        u__CNTHCTL_EL2_EL0VTEN: bool,
        u__CNTHCTL_EL2_EL1TVT: bool,
        gs_57857: bool,
        gs_57858: bool,
        ga_55878: ProductType5c790c8ef59cc8b2,
        gs_57855: bool,
        ga_55894: ProductType5c790c8ef59cc8b2,
        gs_57844: bool,
        gs_57852: bool,
        gs_57849: bool,
        gs_57851: bool,
        ga_55877: u64,
        gs_57838: bool,
        ga_55897: ProductType5c790c8ef59cc8b2,
        ga_55848: ProductType5c790c8ef59cc8b2,
        gs_57848: bool,
        gs_57854: bool,
        u__CNTKCTL_EL1_EL0VTEN: bool,
        ga_55858: ProductType5c790c8ef59cc8b2,
        u__PSTATE_EL: u8,
        gs_57840: bool,
        gs_57853: bool,
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
        // D s_0_5: call _get_CNTKCTL_EL1_Type_EL0VTEN(s_0_4)
        let s_0_5: bool = u_get_CNTKCTL_EL1_Type_EL0VTEN(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTKCTL_EL1_EL0VTEN <= s_0_5
        fn_state.u__CNTKCTL_EL1_EL0VTEN = s_0_5;
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
        // D s_0_13: call _get_CNTHCTL_EL2_Type_EL0VTEN(s_0_12)
        let s_0_13: bool = u_get_CNTHCTL_EL2_Type_EL0VTEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTHCTL_EL2_EL0VTEN <= s_0_13
        fn_state.u__CNTHCTL_EL2_EL0VTEN = s_0_13;
        // C s_0_15: const #12808u : u32
        let s_0_15: u32 = 12808;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_CNTHCTL_EL2_Type_EL1TVT(s_0_16)
        let s_0_17: bool = u_get_CNTHCTL_EL2_Type_EL1TVT(state, tracer, s_0_16);
        // D s_0_18: write-var __CNTHCTL_EL2_EL1TVT <= s_0_17
        fn_state.u__CNTHCTL_EL2_EL1TVT = s_0_17;
        // D s_0_19: read-var __PSTATE_EL:u8
        let s_0_19: u8 = fn_state.u__PSTATE_EL;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 2u16);
        // C s_0_21: const #448u : u32
        let s_0_21: u32 = 448;
        // D s_0_22: read-reg s_0_21:u8
        let s_0_22: u8 = {
            let value = state.read_register::<u8>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 2u16);
        // D s_0_24: cmp-eq s_0_20 s_0_23
        let s_0_24: bool = ((s_0_20) == (s_0_23));
        // N s_0_25: branch s_0_24 b32 b1
        if s_0_24 {
            return block_32(state, tracer, fn_state);
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
        // C s_5_1: const #17200u : u32
        let s_5_1: u32 = 17200;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_CNTV_CTL_EL0(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_CNTV_CTL_EL0(
            state,
            tracer,
            s_5_2,
        );
        // D s_5_4: write-var ga#55901 <= s_5_3
        fn_state.ga_55901 = s_5_3;
        // D s_5_5: read-var ga#55901.0:struct
        let s_5_5: u64 = fn_state.ga_55901._0;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 64u16);
        // D s_5_7: read-var t:i
        let s_5_7: i128 = fn_state.t;
        // D s_5_8: call X_set(s_5_7, s_5_0, s_5_6)
        let s_5_8: () = X_set(state, tracer, s_5_7, s_5_0, s_5_6);
        // N s_5_9: return
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
        // D s_7_1: write-var gs#57838 <= s_7_0
        fn_state.gs_57838 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#57838:u8
        let s_8_0: bool = fn_state.gs_57838;
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
        // D s_9_1: write-var gs#57839 <= s_9_0
        fn_state.gs_57839 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#57839:u8
        let s_10_0: bool = fn_state.gs_57839;
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
        // D s_12_1: write-var gs#57840 <= s_12_0
        fn_state.gs_57840 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#57840:u8
        let s_13_0: bool = fn_state.gs_57840;
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
        // C s_14_1: const #17200u : u32
        let s_14_1: u32 = 17200;
        // D s_14_2: read-reg s_14_1:struct
        let s_14_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_14_1 as isize);
            tracer.read_register(s_14_1 as isize, value);
            value
        };
        // D s_14_3: call __get_CNTV_CTL_EL0(s_14_2)
        let s_14_3: ProductType5c790c8ef59cc8b2 = u__get_CNTV_CTL_EL0(
            state,
            tracer,
            s_14_2,
        );
        // D s_14_4: write-var ga#55897 <= s_14_3
        fn_state.ga_55897 = s_14_3;
        // D s_14_5: read-var ga#55897.0:struct
        let s_14_5: u64 = fn_state.ga_55897._0;
        // D s_14_6: cast zx s_14_5 -> bv
        let s_14_6: Bits = Bits::new(s_14_5 as u128, 64u16);
        // D s_14_7: read-var t:i
        let s_14_7: i128 = fn_state.t;
        // D s_14_8: call X_set(s_14_7, s_14_0, s_14_6)
        let s_14_8: () = X_set(state, tracer, s_14_7, s_14_0, s_14_6);
        // N s_14_9: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // C s_15_1: const #19280u : u32
        let s_15_1: u32 = 19280;
        // D s_15_2: read-reg s_15_1:struct
        let s_15_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_15_1 as isize);
            tracer.read_register(s_15_1 as isize, value);
            value
        };
        // D s_15_3: call __get_CNTHV_CTL_EL2(s_15_2)
        let s_15_3: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_CTL_EL2(
            state,
            tracer,
            s_15_2,
        );
        // D s_15_4: write-var ga#55894 <= s_15_3
        fn_state.ga_55894 = s_15_3;
        // D s_15_5: read-var ga#55894.0:struct
        let s_15_5: u64 = fn_state.ga_55894._0;
        // D s_15_6: cast zx s_15_5 -> bv
        let s_15_6: Bits = Bits::new(s_15_5 as u128, 64u16);
        // D s_15_7: read-var t:i
        let s_15_7: i128 = fn_state.t;
        // D s_15_8: call X_set(s_15_7, s_15_0, s_15_6)
        let s_15_8: () = X_set(state, tracer, s_15_7, s_15_0, s_15_6);
        // N s_15_9: return
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
        // D s_16_7: write-var gs#57840 <= s_16_6
        fn_state.gs_57840 = s_16_6;
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
        // C s_17_1: const #14872u : u32
        let s_17_1: u32 = 14872;
        // D s_17_2: read-reg s_17_1:struct
        let s_17_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_1 as isize);
            tracer.read_register(s_17_1 as isize, value);
            value
        };
        // D s_17_3: call __get_CNTHVS_CTL_EL2(s_17_2)
        let s_17_3: ProductType5c790c8ef59cc8b2 = u__get_CNTHVS_CTL_EL2(
            state,
            tracer,
            s_17_2,
        );
        // D s_17_4: write-var ga#55887 <= s_17_3
        fn_state.ga_55887 = s_17_3;
        // D s_17_5: read-var ga#55887.0:struct
        let s_17_5: u64 = fn_state.ga_55887._0;
        // D s_17_6: cast zx s_17_5 -> bv
        let s_17_6: Bits = Bits::new(s_17_5 as u128, 64u16);
        // D s_17_7: read-var t:i
        let s_17_7: i128 = fn_state.t;
        // D s_17_8: call X_set(s_17_7, s_17_0, s_17_6)
        let s_17_8: () = X_set(state, tracer, s_17_7, s_17_0, s_17_6);
        // N s_17_9: return
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
        // D s_18_2: write-var gs#57839 <= s_18_1
        fn_state.gs_57839 = s_18_1;
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
        // D s_19_7: write-var gs#57838 <= s_19_6
        fn_state.gs_57838 = s_19_6;
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
        // N s_20_2: branch s_20_1 b31 b21
        if s_20_1 {
            return block_31(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#57844 <= s_21_0
        fn_state.gs_57844 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#57844:u8
        let s_22_0: bool = fn_state.gs_57844;
        // N s_22_1: branch s_22_0 b30 b23
        if s_22_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call EL2Enabled(s_23_0)
        let s_23_1: bool = EL2Enabled(state, tracer, s_23_0);
        // N s_23_2: branch s_23_1 b29 b24
        if s_23_1 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#57845 <= s_24_0
        fn_state.gs_57845 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#57845:u8
        let s_25_0: bool = fn_state.gs_57845;
        // N s_25_1: branch s_25_0 b27 b26
        if s_25_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #64s : i64
        let s_26_0: i64 = 64;
        // C s_26_1: const #17200u : u32
        let s_26_1: u32 = 17200;
        // D s_26_2: read-reg s_26_1:struct
        let s_26_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_1 as isize);
            tracer.read_register(s_26_1 as isize, value);
            value
        };
        // D s_26_3: call __get_CNTV_CTL_EL0(s_26_2)
        let s_26_3: ProductType5c790c8ef59cc8b2 = u__get_CNTV_CTL_EL0(
            state,
            tracer,
            s_26_2,
        );
        // D s_26_4: write-var ga#55878 <= s_26_3
        fn_state.ga_55878 = s_26_3;
        // D s_26_5: read-var ga#55878.0:struct
        let s_26_5: u64 = fn_state.ga_55878._0;
        // D s_26_6: cast zx s_26_5 -> bv
        let s_26_6: Bits = Bits::new(s_26_5 as u128, 64u16);
        // D s_26_7: read-var t:i
        let s_26_7: i128 = fn_state.t;
        // D s_26_8: call X_set(s_26_7, s_26_0, s_26_6)
        let s_26_8: () = X_set(state, tracer, s_26_7, s_26_0, s_26_6);
        // N s_26_9: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #368u : u12
        let s_27_0: u16 = 368;
        // C s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 12u16);
        // C s_27_2: cast zx s_27_1 -> i
        let s_27_2: i128 = (s_27_1.value() as i128);
        // C s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // S s_27_5: call NVMem_read(s_27_4)
        let s_27_5: u64 = NVMem_read(state, tracer, s_27_4);
        // D s_27_6: write-var ga#55877 <= s_27_5
        fn_state.ga_55877 = s_27_5;
        // N s_27_7: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var ga#55877:u64
        let s_28_0: u64 = fn_state.ga_55877;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 64u16);
        // D s_28_2: read-var t:i
        let s_28_2: i128 = fn_state.t;
        // C s_28_3: const #64s : i64
        let s_28_3: i64 = 64;
        // D s_28_4: call X_set(s_28_2, s_28_3, s_28_1)
        let s_28_4: () = X_set(state, tracer, s_28_2, s_28_3, s_28_1);
        // N s_28_5: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #102552u : u32
        let s_29_0: u32 = 102552;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call _get_HCR_EL2_Type_NV2(s_29_1)
        let s_29_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_29_1);
        // C s_29_3: const #102552u : u32
        let s_29_3: u32 = 102552;
        // D s_29_4: read-reg s_29_3:struct
        let s_29_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_3 as isize);
            tracer.read_register(s_29_3 as isize, value);
            value
        };
        // D s_29_5: call _get_HCR_EL2_Type_NV1(s_29_4)
        let s_29_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_29_4);
        // C s_29_6: const #102552u : u32
        let s_29_6: u32 = 102552;
        // D s_29_7: read-reg s_29_6:struct
        let s_29_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_6 as isize);
            tracer.read_register(s_29_6 as isize, value);
            value
        };
        // D s_29_8: call _get_HCR_EL2_Type_NV(s_29_7)
        let s_29_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_29_7);
        // D s_29_9: cast zx s_29_5 -> bv
        let s_29_9: Bits = Bits::new(s_29_5 as u128, 1u16);
        // D s_29_10: cast zx s_29_8 -> bv
        let s_29_10: Bits = Bits::new(s_29_8 as u128, 1u16);
        // D s_29_11: cast reint s_29_9 -> u128
        let s_29_11: u128 = (s_29_9.value() as u128);
        // D s_29_12: size-of s_29_9
        let s_29_12: u16 = s_29_9.length();
        // D s_29_13: cast reint s_29_10 -> u128
        let s_29_13: u128 = (s_29_10.value() as u128);
        // D s_29_14: size-of s_29_10
        let s_29_14: u16 = s_29_10.length();
        // D s_29_15: lsl s_29_11 s_29_14
        let s_29_15: u128 = s_29_11 << s_29_14;
        // D s_29_16: or s_29_15 s_29_13
        let s_29_16: u128 = ((s_29_15) | (s_29_13));
        // D s_29_17: add s_29_12 s_29_14
        let s_29_17: u16 = (s_29_12 + s_29_14);
        // D s_29_18: create-bits s_29_16 s_29_17
        let s_29_18: Bits = Bits::new(s_29_16, s_29_17);
        // D s_29_19: cast reint s_29_18 -> u8
        let s_29_19: u8 = (s_29_18.value() as u8);
        // D s_29_20: cast zx s_29_2 -> bv
        let s_29_20: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_21: cast zx s_29_19 -> bv
        let s_29_21: Bits = Bits::new(s_29_19 as u128, 2u16);
        // D s_29_22: cast reint s_29_20 -> u128
        let s_29_22: u128 = (s_29_20.value() as u128);
        // D s_29_23: size-of s_29_20
        let s_29_23: u16 = s_29_20.length();
        // D s_29_24: cast reint s_29_21 -> u128
        let s_29_24: u128 = (s_29_21.value() as u128);
        // D s_29_25: size-of s_29_21
        let s_29_25: u16 = s_29_21.length();
        // D s_29_26: lsl s_29_22 s_29_25
        let s_29_26: u128 = s_29_22 << s_29_25;
        // D s_29_27: or s_29_26 s_29_24
        let s_29_27: u128 = ((s_29_26) | (s_29_24));
        // D s_29_28: add s_29_23 s_29_25
        let s_29_28: u16 = (s_29_23 + s_29_25);
        // D s_29_29: create-bits s_29_27 s_29_28
        let s_29_29: Bits = Bits::new(s_29_27, s_29_28);
        // D s_29_30: cast reint s_29_29 -> u8
        let s_29_30: u8 = (s_29_29.value() as u8);
        // D s_29_31: cast zx s_29_30 -> bv
        let s_29_31: Bits = Bits::new(s_29_30 as u128, 3u16);
        // C s_29_32: const #7u : u8
        let s_29_32: u8 = 7;
        // C s_29_33: cast zx s_29_32 -> bv
        let s_29_33: Bits = Bits::new(s_29_32 as u128, 3u16);
        // D s_29_34: cmp-eq s_29_31 s_29_33
        let s_29_34: bool = ((s_29_31) == (s_29_33));
        // D s_29_35: write-var gs#57845 <= s_29_34
        fn_state.gs_57845 = s_29_34;
        // N s_29_36: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #24u : u8
        let s_30_0: u8 = 24;
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
        // D s_31_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_31_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#57844 <= s_31_4
        fn_state.gs_57844 = s_31_4;
        // N s_31_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call EL2Enabled(s_32_0)
        let s_32_1: bool = EL2Enabled(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b80 b33
        if s_32_1 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#57848 <= s_33_0
        fn_state.gs_57848 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#57848:u8
        let s_34_0: bool = fn_state.gs_57848;
        // D s_34_1: not s_34_0
        let s_34_1: bool = !s_34_0;
        // N s_34_2: branch s_34_1 b79 b35
        if s_34_1 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#57849 <= s_35_0
        fn_state.gs_57849 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#57849:u8
        let s_36_0: bool = fn_state.gs_57849;
        // N s_36_1: branch s_36_0 b73 b37
        if s_36_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call EL2Enabled(s_37_0)
        let s_37_1: bool = EL2Enabled(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b72 b38
        if s_37_1 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#57850 <= s_38_0
        fn_state.gs_57850 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#57850:u8
        let s_39_0: bool = fn_state.gs_57850;
        // N s_39_1: branch s_39_0 b71 b40
        if s_39_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#57851 <= s_40_0
        fn_state.gs_57851 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#57851:u8
        let s_41_0: bool = fn_state.gs_57851;
        // N s_41_1: branch s_41_0 b70 b42
        if s_41_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
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
        // N s_42_2: branch s_42_1 b69 b43
        if s_42_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#57852 <= s_43_0
        fn_state.gs_57852 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#57852:u8
        let s_44_0: bool = fn_state.gs_57852;
        // N s_44_1: branch s_44_0 b68 b45
        if s_44_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#57853 <= s_45_0
        fn_state.gs_57853 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#57853:u8
        let s_46_0: bool = fn_state.gs_57853;
        // N s_46_1: branch s_46_0 b67 b47
        if s_46_0 {
            return block_67(state, tracer, fn_state);
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
        // N s_47_2: branch s_47_1 b66 b48
        if s_47_1 {
            return block_66(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#57854 <= s_48_0
        fn_state.gs_57854 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#57854:u8
        let s_49_0: bool = fn_state.gs_57854;
        // N s_49_1: branch s_49_0 b65 b50
        if s_49_0 {
            return block_65(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#57855 <= s_50_0
        fn_state.gs_57855 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#57855:u8
        let s_51_0: bool = fn_state.gs_57855;
        // N s_51_1: branch s_51_0 b64 b52
        if s_51_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#57856 <= s_52_0
        fn_state.gs_57856 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#57856:u8
        let s_53_0: bool = fn_state.gs_57856;
        // N s_53_1: branch s_53_0 b63 b54
        if s_53_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call EL2Enabled(s_54_0)
        let s_54_1: bool = EL2Enabled(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b62 b55
        if s_54_1 {
            return block_62(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#57857 <= s_55_0
        fn_state.gs_57857 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#57857:u8
        let s_56_0: bool = fn_state.gs_57857;
        // N s_56_1: branch s_56_0 b61 b57
        if s_56_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#57858 <= s_57_0
        fn_state.gs_57858 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#57858:u8
        let s_58_0: bool = fn_state.gs_57858;
        // N s_58_1: branch s_58_0 b60 b59
        if s_58_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #64s : i64
        let s_59_0: i64 = 64;
        // C s_59_1: const #17200u : u32
        let s_59_1: u32 = 17200;
        // D s_59_2: read-reg s_59_1:struct
        let s_59_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_1 as isize);
            tracer.read_register(s_59_1 as isize, value);
            value
        };
        // D s_59_3: call __get_CNTV_CTL_EL0(s_59_2)
        let s_59_3: ProductType5c790c8ef59cc8b2 = u__get_CNTV_CTL_EL0(
            state,
            tracer,
            s_59_2,
        );
        // D s_59_4: write-var ga#55861 <= s_59_3
        fn_state.ga_55861 = s_59_3;
        // D s_59_5: read-var ga#55861.0:struct
        let s_59_5: u64 = fn_state.ga_55861._0;
        // D s_59_6: cast zx s_59_5 -> bv
        let s_59_6: Bits = Bits::new(s_59_5 as u128, 64u16);
        // D s_59_7: read-var t:i
        let s_59_7: i128 = fn_state.t;
        // D s_59_8: call X_set(s_59_7, s_59_0, s_59_6)
        let s_59_8: () = X_set(state, tracer, s_59_7, s_59_0, s_59_6);
        // N s_59_9: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #64s : i64
        let s_60_0: i64 = 64;
        // C s_60_1: const #19280u : u32
        let s_60_1: u32 = 19280;
        // D s_60_2: read-reg s_60_1:struct
        let s_60_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_1 as isize);
            tracer.read_register(s_60_1 as isize, value);
            value
        };
        // D s_60_3: call __get_CNTHV_CTL_EL2(s_60_2)
        let s_60_3: ProductType5c790c8ef59cc8b2 = u__get_CNTHV_CTL_EL2(
            state,
            tracer,
            s_60_2,
        );
        // D s_60_4: write-var ga#55858 <= s_60_3
        fn_state.ga_55858 = s_60_3;
        // D s_60_5: read-var ga#55858.0:struct
        let s_60_5: u64 = fn_state.ga_55858._0;
        // D s_60_6: cast zx s_60_5 -> bv
        let s_60_6: Bits = Bits::new(s_60_5 as u128, 64u16);
        // D s_60_7: read-var t:i
        let s_60_7: i128 = fn_state.t;
        // D s_60_8: call X_set(s_60_7, s_60_0, s_60_6)
        let s_60_8: () = X_set(state, tracer, s_60_7, s_60_0, s_60_6);
        // N s_60_9: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #90704u : u32
        let s_61_0: u32 = 90704;
        // D s_61_1: read-reg s_61_0:struct
        let s_61_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call _get_SCR_EL3_Type_NS(s_61_1)
        let s_61_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_61_1);
        // D s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 1u16);
        // C s_61_4: const #1u : u8
        let s_61_4: bool = true;
        // C s_61_5: cast zx s_61_4 -> bv
        let s_61_5: Bits = Bits::new(s_61_4 as u128, 1u16);
        // D s_61_6: cmp-eq s_61_3 s_61_5
        let s_61_6: bool = ((s_61_3) == (s_61_5));
        // D s_61_7: write-var gs#57858 <= s_61_6
        fn_state.gs_57858 = s_61_6;
        // N s_61_8: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #102552u : u32
        let s_62_0: u32 = 102552;
        // D s_62_1: read-reg s_62_0:struct
        let s_62_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call _get_HCR_EL2_Type_E2H(s_62_1)
        let s_62_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_62_1);
        // C s_62_3: const #102552u : u32
        let s_62_3: u32 = 102552;
        // D s_62_4: read-reg s_62_3:struct
        let s_62_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_3 as isize);
            tracer.read_register(s_62_3 as isize, value);
            value
        };
        // D s_62_5: call _get_HCR_EL2_Type_TGE(s_62_4)
        let s_62_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_62_4);
        // D s_62_6: cast zx s_62_2 -> bv
        let s_62_6: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_7: cast zx s_62_5 -> bv
        let s_62_7: Bits = Bits::new(s_62_5 as u128, 1u16);
        // D s_62_8: cast reint s_62_6 -> u128
        let s_62_8: u128 = (s_62_6.value() as u128);
        // D s_62_9: size-of s_62_6
        let s_62_9: u16 = s_62_6.length();
        // D s_62_10: cast reint s_62_7 -> u128
        let s_62_10: u128 = (s_62_7.value() as u128);
        // D s_62_11: size-of s_62_7
        let s_62_11: u16 = s_62_7.length();
        // D s_62_12: lsl s_62_8 s_62_11
        let s_62_12: u128 = s_62_8 << s_62_11;
        // D s_62_13: or s_62_12 s_62_10
        let s_62_13: u128 = ((s_62_12) | (s_62_10));
        // D s_62_14: add s_62_9 s_62_11
        let s_62_14: u16 = (s_62_9 + s_62_11);
        // D s_62_15: create-bits s_62_13 s_62_14
        let s_62_15: Bits = Bits::new(s_62_13, s_62_14);
        // D s_62_16: cast reint s_62_15 -> u8
        let s_62_16: u8 = (s_62_15.value() as u8);
        // D s_62_17: cast zx s_62_16 -> bv
        let s_62_17: Bits = Bits::new(s_62_16 as u128, 2u16);
        // C s_62_18: const #3u : u8
        let s_62_18: u8 = 3;
        // C s_62_19: cast zx s_62_18 -> bv
        let s_62_19: Bits = Bits::new(s_62_18 as u128, 2u16);
        // D s_62_20: cmp-eq s_62_17 s_62_19
        let s_62_20: bool = ((s_62_17) == (s_62_19));
        // D s_62_21: write-var gs#57857 <= s_62_20
        fn_state.gs_57857 = s_62_20;
        // N s_62_22: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #64s : i64
        let s_63_0: i64 = 64;
        // C s_63_1: const #14872u : u32
        let s_63_1: u32 = 14872;
        // D s_63_2: read-reg s_63_1:struct
        let s_63_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_1 as isize);
            tracer.read_register(s_63_1 as isize, value);
            value
        };
        // D s_63_3: call __get_CNTHVS_CTL_EL2(s_63_2)
        let s_63_3: ProductType5c790c8ef59cc8b2 = u__get_CNTHVS_CTL_EL2(
            state,
            tracer,
            s_63_2,
        );
        // D s_63_4: write-var ga#55848 <= s_63_3
        fn_state.ga_55848 = s_63_3;
        // D s_63_5: read-var ga#55848.0:struct
        let s_63_5: u64 = fn_state.ga_55848._0;
        // D s_63_6: cast zx s_63_5 -> bv
        let s_63_6: Bits = Bits::new(s_63_5 as u128, 64u16);
        // D s_63_7: read-var t:i
        let s_63_7: i128 = fn_state.t;
        // D s_63_8: call X_set(s_63_7, s_63_0, s_63_6)
        let s_63_8: () = X_set(state, tracer, s_63_7, s_63_0, s_63_6);
        // N s_63_9: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #117u : u32
        let s_64_0: u32 = 117;
        // S s_64_1: call IsFeatureImplemented(s_64_0)
        let s_64_1: bool = IsFeatureImplemented(state, tracer, s_64_0);
        // D s_64_2: write-var gs#57856 <= s_64_1
        fn_state.gs_57856 = s_64_1;
        // N s_64_3: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #90704u : u32
        let s_65_0: u32 = 90704;
        // D s_65_1: read-reg s_65_0:struct
        let s_65_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // D s_65_2: call _get_SCR_EL3_Type_NS(s_65_1)
        let s_65_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_65_1);
        // D s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // C s_65_4: const #0u : u8
        let s_65_4: bool = false;
        // C s_65_5: cast zx s_65_4 -> bv
        let s_65_5: Bits = Bits::new(s_65_4 as u128, 1u16);
        // D s_65_6: cmp-eq s_65_3 s_65_5
        let s_65_6: bool = ((s_65_3) == (s_65_5));
        // D s_65_7: write-var gs#57855 <= s_65_6
        fn_state.gs_57855 = s_65_6;
        // N s_65_8: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #102552u : u32
        let s_66_0: u32 = 102552;
        // D s_66_1: read-reg s_66_0:struct
        let s_66_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_0 as isize);
            tracer.read_register(s_66_0 as isize, value);
            value
        };
        // D s_66_2: call _get_HCR_EL2_Type_E2H(s_66_1)
        let s_66_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_66_1);
        // C s_66_3: const #102552u : u32
        let s_66_3: u32 = 102552;
        // D s_66_4: read-reg s_66_3:struct
        let s_66_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_66_3 as isize);
            tracer.read_register(s_66_3 as isize, value);
            value
        };
        // D s_66_5: call _get_HCR_EL2_Type_TGE(s_66_4)
        let s_66_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_66_4);
        // D s_66_6: cast zx s_66_2 -> bv
        let s_66_6: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_7: cast zx s_66_5 -> bv
        let s_66_7: Bits = Bits::new(s_66_5 as u128, 1u16);
        // D s_66_8: cast reint s_66_6 -> u128
        let s_66_8: u128 = (s_66_6.value() as u128);
        // D s_66_9: size-of s_66_6
        let s_66_9: u16 = s_66_6.length();
        // D s_66_10: cast reint s_66_7 -> u128
        let s_66_10: u128 = (s_66_7.value() as u128);
        // D s_66_11: size-of s_66_7
        let s_66_11: u16 = s_66_7.length();
        // D s_66_12: lsl s_66_8 s_66_11
        let s_66_12: u128 = s_66_8 << s_66_11;
        // D s_66_13: or s_66_12 s_66_10
        let s_66_13: u128 = ((s_66_12) | (s_66_10));
        // D s_66_14: add s_66_9 s_66_11
        let s_66_14: u16 = (s_66_9 + s_66_11);
        // D s_66_15: create-bits s_66_13 s_66_14
        let s_66_15: Bits = Bits::new(s_66_13, s_66_14);
        // D s_66_16: cast reint s_66_15 -> u8
        let s_66_16: u8 = (s_66_15.value() as u8);
        // D s_66_17: cast zx s_66_16 -> bv
        let s_66_17: Bits = Bits::new(s_66_16 as u128, 2u16);
        // C s_66_18: const #3u : u8
        let s_66_18: u8 = 3;
        // C s_66_19: cast zx s_66_18 -> bv
        let s_66_19: Bits = Bits::new(s_66_18 as u128, 2u16);
        // D s_66_20: cmp-eq s_66_17 s_66_19
        let s_66_20: bool = ((s_66_17) == (s_66_19));
        // D s_66_21: write-var gs#57854 <= s_66_20
        fn_state.gs_57854 = s_66_20;
        // N s_66_22: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #24u : u8
        let s_67_0: u8 = 24;
        // C s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 8u16);
        // C s_67_2: cast zx s_67_1 -> i
        let s_67_2: i128 = (s_67_1.value() as i128);
        // C s_67_3: cast reint s_67_2 -> i64
        let s_67_3: i64 = (s_67_2 as i64);
        // C s_67_4: cast zx s_67_3 -> i
        let s_67_4: i128 = (i128::try_from(s_67_3).unwrap());
        // C s_67_5: const #432u : u32
        let s_67_5: u32 = 432;
        // D s_67_6: read-reg s_67_5:u8
        let s_67_6: u8 = {
            let value = state.read_register::<u8>(s_67_5 as isize);
            tracer.read_register(s_67_5 as isize, value);
            value
        };
        // D s_67_7: call AArch64_SystemAccessTrap(s_67_6, s_67_4)
        let s_67_7: () = AArch64_SystemAccessTrap(state, tracer, s_67_6, s_67_4);
        // N s_67_8: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var __CNTHCTL_EL2_EL1TVT:u8
        let s_68_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVT;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 1u16);
        // C s_68_2: const #1u : u8
        let s_68_2: bool = true;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // D s_68_5: write-var gs#57853 <= s_68_4
        fn_state.gs_57853 = s_68_4;
        // N s_68_6: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #102552u : u32
        let s_69_0: u32 = 102552;
        // D s_69_1: read-reg s_69_0:struct
        let s_69_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: call _get_HCR_EL2_Type_E2H(s_69_1)
        let s_69_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_69_1);
        // C s_69_3: const #102552u : u32
        let s_69_3: u32 = 102552;
        // D s_69_4: read-reg s_69_3:struct
        let s_69_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_3 as isize);
            tracer.read_register(s_69_3 as isize, value);
            value
        };
        // D s_69_5: call _get_HCR_EL2_Type_TGE(s_69_4)
        let s_69_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_69_4);
        // D s_69_6: cast zx s_69_2 -> bv
        let s_69_6: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_7: cast zx s_69_5 -> bv
        let s_69_7: Bits = Bits::new(s_69_5 as u128, 1u16);
        // D s_69_8: cast reint s_69_6 -> u128
        let s_69_8: u128 = (s_69_6.value() as u128);
        // D s_69_9: size-of s_69_6
        let s_69_9: u16 = s_69_6.length();
        // D s_69_10: cast reint s_69_7 -> u128
        let s_69_10: u128 = (s_69_7.value() as u128);
        // D s_69_11: size-of s_69_7
        let s_69_11: u16 = s_69_7.length();
        // D s_69_12: lsl s_69_8 s_69_11
        let s_69_12: u128 = s_69_8 << s_69_11;
        // D s_69_13: or s_69_12 s_69_10
        let s_69_13: u128 = ((s_69_12) | (s_69_10));
        // D s_69_14: add s_69_9 s_69_11
        let s_69_14: u16 = (s_69_9 + s_69_11);
        // D s_69_15: create-bits s_69_13 s_69_14
        let s_69_15: Bits = Bits::new(s_69_13, s_69_14);
        // D s_69_16: cast reint s_69_15 -> u8
        let s_69_16: u8 = (s_69_15.value() as u8);
        // D s_69_17: cast zx s_69_16 -> bv
        let s_69_17: Bits = Bits::new(s_69_16 as u128, 2u16);
        // C s_69_18: const #3u : u8
        let s_69_18: u8 = 3;
        // C s_69_19: cast zx s_69_18 -> bv
        let s_69_19: Bits = Bits::new(s_69_18 as u128, 2u16);
        // D s_69_20: cmp-ne s_69_17 s_69_19
        let s_69_20: bool = ((s_69_17) != (s_69_19));
        // D s_69_21: write-var gs#57852 <= s_69_20
        fn_state.gs_57852 = s_69_20;
        // N s_69_22: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #24u : u8
        let s_70_0: u8 = 24;
        // C s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 8u16);
        // C s_70_2: cast zx s_70_1 -> i
        let s_70_2: i128 = (s_70_1.value() as i128);
        // C s_70_3: cast reint s_70_2 -> i64
        let s_70_3: i64 = (s_70_2 as i64);
        // C s_70_4: cast zx s_70_3 -> i
        let s_70_4: i128 = (i128::try_from(s_70_3).unwrap());
        // C s_70_5: const #432u : u32
        let s_70_5: u32 = 432;
        // D s_70_6: read-reg s_70_5:u8
        let s_70_6: u8 = {
            let value = state.read_register::<u8>(s_70_5 as isize);
            tracer.read_register(s_70_5 as isize, value);
            value
        };
        // D s_70_7: call AArch64_SystemAccessTrap(s_70_6, s_70_4)
        let s_70_7: () = AArch64_SystemAccessTrap(state, tracer, s_70_6, s_70_4);
        // N s_70_8: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __CNTHCTL_EL2_EL0VTEN:u8
        let s_71_0: bool = fn_state.u__CNTHCTL_EL2_EL0VTEN;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #0u : u8
        let s_71_2: bool = false;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#57851 <= s_71_4
        fn_state.gs_57851 = s_71_4;
        // N s_71_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #102552u : u32
        let s_72_0: u32 = 102552;
        // D s_72_1: read-reg s_72_0:struct
        let s_72_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // D s_72_2: call _get_HCR_EL2_Type_E2H(s_72_1)
        let s_72_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_72_1);
        // C s_72_3: const #102552u : u32
        let s_72_3: u32 = 102552;
        // D s_72_4: read-reg s_72_3:struct
        let s_72_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_72_3 as isize);
            tracer.read_register(s_72_3 as isize, value);
            value
        };
        // D s_72_5: call _get_HCR_EL2_Type_TGE(s_72_4)
        let s_72_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_72_4);
        // D s_72_6: cast zx s_72_2 -> bv
        let s_72_6: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_7: cast zx s_72_5 -> bv
        let s_72_7: Bits = Bits::new(s_72_5 as u128, 1u16);
        // D s_72_8: cast reint s_72_6 -> u128
        let s_72_8: u128 = (s_72_6.value() as u128);
        // D s_72_9: size-of s_72_6
        let s_72_9: u16 = s_72_6.length();
        // D s_72_10: cast reint s_72_7 -> u128
        let s_72_10: u128 = (s_72_7.value() as u128);
        // D s_72_11: size-of s_72_7
        let s_72_11: u16 = s_72_7.length();
        // D s_72_12: lsl s_72_8 s_72_11
        let s_72_12: u128 = s_72_8 << s_72_11;
        // D s_72_13: or s_72_12 s_72_10
        let s_72_13: u128 = ((s_72_12) | (s_72_10));
        // D s_72_14: add s_72_9 s_72_11
        let s_72_14: u16 = (s_72_9 + s_72_11);
        // D s_72_15: create-bits s_72_13 s_72_14
        let s_72_15: Bits = Bits::new(s_72_13, s_72_14);
        // D s_72_16: cast reint s_72_15 -> u8
        let s_72_16: u8 = (s_72_15.value() as u8);
        // D s_72_17: cast zx s_72_16 -> bv
        let s_72_17: Bits = Bits::new(s_72_16 as u128, 2u16);
        // C s_72_18: const #3u : u8
        let s_72_18: u8 = 3;
        // C s_72_19: cast zx s_72_18 -> bv
        let s_72_19: Bits = Bits::new(s_72_18 as u128, 2u16);
        // D s_72_20: cmp-eq s_72_17 s_72_19
        let s_72_20: bool = ((s_72_17) == (s_72_19));
        // D s_72_21: write-var gs#57850 <= s_72_20
        fn_state.gs_57850 = s_72_20;
        // N s_72_22: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call EL2Enabled(s_73_0)
        let s_73_1: bool = EL2Enabled(state, tracer, s_73_0);
        // N s_73_2: branch s_73_1 b78 b74
        if s_73_1 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#57862 <= s_74_0
        fn_state.gs_57862 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#57862:u8
        let s_75_0: bool = fn_state.gs_57862;
        // N s_75_1: branch s_75_0 b77 b76
        if s_75_0 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #24u : u8
        let s_76_0: u8 = 24;
        // C s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 8u16);
        // C s_76_2: cast zx s_76_1 -> i
        let s_76_2: i128 = (s_76_1.value() as i128);
        // C s_76_3: cast reint s_76_2 -> i64
        let s_76_3: i64 = (s_76_2 as i64);
        // C s_76_4: cast zx s_76_3 -> i
        let s_76_4: i128 = (i128::try_from(s_76_3).unwrap());
        // C s_76_5: const #440u : u32
        let s_76_5: u32 = 440;
        // D s_76_6: read-reg s_76_5:u8
        let s_76_6: u8 = {
            let value = state.read_register::<u8>(s_76_5 as isize);
            tracer.read_register(s_76_5 as isize, value);
            value
        };
        // D s_76_7: call AArch64_SystemAccessTrap(s_76_6, s_76_4)
        let s_76_7: () = AArch64_SystemAccessTrap(state, tracer, s_76_6, s_76_4);
        // N s_76_8: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #24u : u8
        let s_77_0: u8 = 24;
        // C s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 8u16);
        // C s_77_2: cast zx s_77_1 -> i
        let s_77_2: i128 = (s_77_1.value() as i128);
        // C s_77_3: cast reint s_77_2 -> i64
        let s_77_3: i64 = (s_77_2 as i64);
        // C s_77_4: cast zx s_77_3 -> i
        let s_77_4: i128 = (i128::try_from(s_77_3).unwrap());
        // C s_77_5: const #432u : u32
        let s_77_5: u32 = 432;
        // D s_77_6: read-reg s_77_5:u8
        let s_77_6: u8 = {
            let value = state.read_register::<u8>(s_77_5 as isize);
            tracer.read_register(s_77_5 as isize, value);
            value
        };
        // D s_77_7: call AArch64_SystemAccessTrap(s_77_6, s_77_4)
        let s_77_7: () = AArch64_SystemAccessTrap(state, tracer, s_77_6, s_77_4);
        // N s_77_8: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var __HCR_EL2_TGE:u8
        let s_78_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #1u : u8
        let s_78_2: bool = true;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#57862 <= s_78_4
        fn_state.gs_57862 = s_78_4;
        // N s_78_6: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var __CNTKCTL_EL1_EL0VTEN:u8
        let s_79_0: bool = fn_state.u__CNTKCTL_EL1_EL0VTEN;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 1u16);
        // C s_79_2: const #0u : u8
        let s_79_2: bool = false;
        // C s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_4: cmp-eq s_79_1 s_79_3
        let s_79_4: bool = ((s_79_1) == (s_79_3));
        // D s_79_5: write-var gs#57849 <= s_79_4
        fn_state.gs_57849 = s_79_4;
        // N s_79_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #102552u : u32
        let s_80_0: u32 = 102552;
        // D s_80_1: read-reg s_80_0:struct
        let s_80_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // D s_80_2: call _get_HCR_EL2_Type_E2H(s_80_1)
        let s_80_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_80_1);
        // C s_80_3: const #102552u : u32
        let s_80_3: u32 = 102552;
        // D s_80_4: read-reg s_80_3:struct
        let s_80_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_3 as isize);
            tracer.read_register(s_80_3 as isize, value);
            value
        };
        // D s_80_5: call _get_HCR_EL2_Type_TGE(s_80_4)
        let s_80_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_80_4);
        // D s_80_6: cast zx s_80_2 -> bv
        let s_80_6: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_7: cast zx s_80_5 -> bv
        let s_80_7: Bits = Bits::new(s_80_5 as u128, 1u16);
        // D s_80_8: cast reint s_80_6 -> u128
        let s_80_8: u128 = (s_80_6.value() as u128);
        // D s_80_9: size-of s_80_6
        let s_80_9: u16 = s_80_6.length();
        // D s_80_10: cast reint s_80_7 -> u128
        let s_80_10: u128 = (s_80_7.value() as u128);
        // D s_80_11: size-of s_80_7
        let s_80_11: u16 = s_80_7.length();
        // D s_80_12: lsl s_80_8 s_80_11
        let s_80_12: u128 = s_80_8 << s_80_11;
        // D s_80_13: or s_80_12 s_80_10
        let s_80_13: u128 = ((s_80_12) | (s_80_10));
        // D s_80_14: add s_80_9 s_80_11
        let s_80_14: u16 = (s_80_9 + s_80_11);
        // D s_80_15: create-bits s_80_13 s_80_14
        let s_80_15: Bits = Bits::new(s_80_13, s_80_14);
        // D s_80_16: cast reint s_80_15 -> u8
        let s_80_16: u8 = (s_80_15.value() as u8);
        // D s_80_17: cast zx s_80_16 -> bv
        let s_80_17: Bits = Bits::new(s_80_16 as u128, 2u16);
        // C s_80_18: const #3u : u8
        let s_80_18: u8 = 3;
        // C s_80_19: cast zx s_80_18 -> bv
        let s_80_19: Bits = Bits::new(s_80_18 as u128, 2u16);
        // D s_80_20: cmp-eq s_80_17 s_80_19
        let s_80_20: bool = ((s_80_17) == (s_80_19));
        // D s_80_21: write-var gs#57848 <= s_80_20
        fn_state.gs_57848 = s_80_20;
        // N s_80_22: jump b34
        return block_34(state, tracer, fn_state);
    }
}
