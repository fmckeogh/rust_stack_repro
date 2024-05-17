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
use u_get_HDFGRTR_EL2_Type_nBRBCTL::*;
use Halted::*;
use u_get_MDCR_EL3_Type_SBRBE::*;
use NVMem_read::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u__get_BRBCR_EL1::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_HCR_EL2_Type_NV1::*;
use IsFeatureImplemented::*;
use X_set::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u__get_BRBCR_EL2::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn BRBCR_EL2_SysRegRead_3e399240c4f4d549<T: Tracer>(
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
        gs_49917: bool,
        gs_49903: bool,
        u__SCR_EL3_NS: bool,
        gs_49885: bool,
        u__HCR_EL2_E2H: bool,
        gs_49893: bool,
        gs_49871: bool,
        gs_49912: bool,
        gs_49899: bool,
        gs_49919: bool,
        ga_44794: ProductType5c790c8ef59cc8b2,
        gs_49915: bool,
        gs_49887: bool,
        gs_49918: bool,
        gs_49878: bool,
        gs_49897: bool,
        gs_49905: bool,
        gs_49898: bool,
        u__PSTATE_EL: u8,
        gs_49926: bool,
        gs_49888: bool,
        gs_49913: bool,
        gs_49927: bool,
        gs_49902: bool,
        gs_49925: bool,
        u__EDSCR_SDD: bool,
        gs_49879: bool,
        gs_49873: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_49900: bool,
        ga_44797: ProductType5c790c8ef59cc8b2,
        u__HDFGRTR_EL2_nBRBCTL: bool,
        gs_49904: bool,
        gs_49931: bool,
        gs_49906: bool,
        gs_49884: bool,
        gs_49886: bool,
        gs_49894: bool,
        gs_49872: bool,
        gs_49920: bool,
        gs_49901: bool,
        ga_44801: ProductType5c790c8ef59cc8b2,
        u__MDCR_EL3_SBRBE: u8,
        gs_49916: bool,
        ga_44759: ProductType5c790c8ef59cc8b2,
        gs_49875: bool,
        gs_49877: bool,
        ga_44758: u64,
        gs_49907: bool,
        gs_49876: bool,
        gs_49874: bool,
        gs_49914: bool,
        gs_49930: bool,
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
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call EDSCR_read(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_3);
        // S s_0_5: call _get_EDSCR_Type_SDD(s_0_4)
        let s_0_5: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_4);
        // D s_0_6: write-var __EDSCR_SDD <= s_0_5
        fn_state.u__EDSCR_SDD = s_0_5;
        // C s_0_7: const #22712u : u32
        let s_0_7: u32 = 22712;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDCR_EL3_Type_SBRBE(s_0_8)
        let s_0_9: u8 = u_get_MDCR_EL3_Type_SBRBE(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL3_SBRBE <= s_0_9
        fn_state.u__MDCR_EL3_SBRBE = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_NS(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_NS <= s_0_13
        fn_state.u__SCR_EL3_NS = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_FGTEn(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_FGTEn <= s_0_17
        fn_state.u__SCR_EL3_FGTEn = s_0_17;
        // C s_0_19: const #19144u : u32
        let s_0_19: u32 = 19144;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HDFGRTR_EL2_Type_nBRBCTL(s_0_20)
        let s_0_21: bool = u_get_HDFGRTR_EL2_Type_nBRBCTL(state, tracer, s_0_20);
        // D s_0_22: write-var __HDFGRTR_EL2_nBRBCTL <= s_0_21
        fn_state.u__HDFGRTR_EL2_nBRBCTL = s_0_21;
        // C s_0_23: const #102552u : u32
        let s_0_23: u32 = 102552;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HCR_EL2_Type_E2H(s_0_24)
        let s_0_25: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_24);
        // D s_0_26: write-var __HCR_EL2_E2H <= s_0_25
        fn_state.u__HCR_EL2_E2H = s_0_25;
        // D s_0_27: read-var __PSTATE_EL:u8
        let s_0_27: u8 = fn_state.u__PSTATE_EL;
        // D s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 2u16);
        // C s_0_29: const #448u : u32
        let s_0_29: u32 = 448;
        // D s_0_30: read-reg s_0_29:u8
        let s_0_30: u8 = {
            let value = state.read_register::<u8>(s_0_29 as isize);
            tracer.read_register(s_0_29 as isize, value);
            value
        };
        // D s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 2u16);
        // D s_0_32: cmp-eq s_0_28 s_0_31
        let s_0_32: bool = ((s_0_28) == (s_0_31));
        // N s_0_33: branch s_0_32 b162 b1
        if s_0_32 {
            return block_162(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b75 b2
        if s_1_5 {
            return block_75(state, tracer, fn_state);
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
        // C s_5_1: const #90640u : u32
        let s_5_1: u32 = 90640;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_BRBCR_EL1(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_BRBCR_EL1(state, tracer, s_5_2);
        // D s_5_4: write-var ga#44801 <= s_5_3
        fn_state.ga_44801 = s_5_3;
        // D s_5_5: read-var ga#44801.0:struct
        let s_5_5: u64 = fn_state.ga_44801._0;
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
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call Halted(s_6_0)
        let s_6_1: bool = Halted(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b74 b7
        if s_6_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#49871 <= s_7_0
        fn_state.gs_49871 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#49871:u8
        let s_8_0: bool = fn_state.gs_49871;
        // N s_8_1: branch s_8_0 b73 b9
        if s_8_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#49872 <= s_9_0
        fn_state.gs_49872 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#49872:u8
        let s_10_0: bool = fn_state.gs_49872;
        // N s_10_1: branch s_10_0 b72 b11
        if s_10_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#49873 <= s_11_0
        fn_state.gs_49873 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#49873:u8
        let s_12_0: bool = fn_state.gs_49873;
        // N s_12_1: branch s_12_0 b71 b13
        if s_12_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#49874 <= s_13_0
        fn_state.gs_49874 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#49874:u8
        let s_14_0: bool = fn_state.gs_49874;
        // N s_14_1: branch s_14_0 b70 b15
        if s_14_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#49875 <= s_15_0
        fn_state.gs_49875 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#49875:u8
        let s_16_0: bool = fn_state.gs_49875;
        // N s_16_1: branch s_16_0 b69 b17
        if s_16_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call Halted(s_17_0)
        let s_17_1: bool = Halted(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b68 b18
        if s_17_1 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#49876 <= s_18_0
        fn_state.gs_49876 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#49876:u8
        let s_19_0: bool = fn_state.gs_49876;
        // N s_19_1: branch s_19_0 b67 b20
        if s_19_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#49877 <= s_20_0
        fn_state.gs_49877 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#49877:u8
        let s_21_0: bool = fn_state.gs_49877;
        // N s_21_1: branch s_21_0 b66 b22
        if s_21_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#49878 <= s_22_0
        fn_state.gs_49878 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#49878:u8
        let s_23_0: bool = fn_state.gs_49878;
        // N s_23_1: branch s_23_0 b62 b24
        if s_23_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#49884 <= s_24_0
        fn_state.gs_49884 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#49884:u8
        let s_25_0: bool = fn_state.gs_49884;
        // N s_25_1: branch s_25_0 b61 b26
        if s_25_0 {
            return block_61(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#49885 <= s_26_0
        fn_state.gs_49885 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#49885:u8
        let s_27_0: bool = fn_state.gs_49885;
        // N s_27_1: branch s_27_0 b60 b28
        if s_27_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #424u : u32
        let s_28_0: u32 = 424;
        // D s_28_1: read-reg s_28_0:u8
        let s_28_1: u8 = {
            let value = state.read_register::<u8>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // C s_28_2: const #2u : u8
        let s_28_2: u8 = 2;
        // D s_28_3: cmp-lt s_28_1 s_28_2
        let s_28_3: bool = ((s_28_1) < (s_28_2));
        // N s_28_4: branch s_28_3 b59 b29
        if s_28_3 {
            return block_59(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#49886 <= s_29_0
        fn_state.gs_49886 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#49886:u8
        let s_30_0: bool = fn_state.gs_49886;
        // N s_30_1: branch s_30_0 b58 b31
        if s_30_0 {
            return block_58(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#49887 <= s_31_0
        fn_state.gs_49887 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#49887:u8
        let s_32_0: bool = fn_state.gs_49887;
        // N s_32_1: branch s_32_0 b52 b33
        if s_32_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #424u : u32
        let s_33_0: u32 = 424;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // C s_33_2: const #2u : u8
        let s_33_2: u8 = 2;
        // D s_33_3: cmp-lt s_33_1 s_33_2
        let s_33_3: bool = ((s_33_1) < (s_33_2));
        // N s_33_4: branch s_33_3 b48 b34
        if s_33_3 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#49893 <= s_34_0
        fn_state.gs_49893 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#49893:u8
        let s_35_0: bool = fn_state.gs_49893;
        // N s_35_1: branch s_35_0 b47 b36
        if s_35_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#49894 <= s_36_0
        fn_state.gs_49894 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#49894:u8
        let s_37_0: bool = fn_state.gs_49894;
        // N s_37_1: branch s_37_0 b41 b38
        if s_37_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var __HCR_EL2_E2H:u8
        let s_38_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // C s_38_2: const #1u : u8
        let s_38_2: bool = true;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // N s_38_5: branch s_38_4 b40 b39
        if s_38_4 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #64s : i64
        let s_39_0: i64 = 64;
        // C s_39_1: const #90640u : u32
        let s_39_1: u32 = 90640;
        // D s_39_2: read-reg s_39_1:struct
        let s_39_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_1 as isize);
            tracer.read_register(s_39_1 as isize, value);
            value
        };
        // D s_39_3: call __get_BRBCR_EL1(s_39_2)
        let s_39_3: ProductType5c790c8ef59cc8b2 = u__get_BRBCR_EL1(
            state,
            tracer,
            s_39_2,
        );
        // D s_39_4: write-var ga#44797 <= s_39_3
        fn_state.ga_44797 = s_39_3;
        // D s_39_5: read-var ga#44797.0:struct
        let s_39_5: u64 = fn_state.ga_44797._0;
        // D s_39_6: cast zx s_39_5 -> bv
        let s_39_6: Bits = Bits::new(s_39_5 as u128, 64u16);
        // D s_39_7: read-var t:i
        let s_39_7: i128 = fn_state.t;
        // D s_39_8: call X_set(s_39_7, s_39_0, s_39_6)
        let s_39_8: () = X_set(state, tracer, s_39_7, s_39_0, s_39_6);
        // N s_39_9: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #64s : i64
        let s_40_0: i64 = 64;
        // C s_40_1: const #18272u : u32
        let s_40_1: u32 = 18272;
        // D s_40_2: read-reg s_40_1:struct
        let s_40_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_40_1 as isize);
            tracer.read_register(s_40_1 as isize, value);
            value
        };
        // D s_40_3: call __get_BRBCR_EL2(s_40_2)
        let s_40_3: ProductType5c790c8ef59cc8b2 = u__get_BRBCR_EL2(
            state,
            tracer,
            s_40_2,
        );
        // D s_40_4: write-var ga#44794 <= s_40_3
        fn_state.ga_44794 = s_40_3;
        // D s_40_5: read-var ga#44794.0:struct
        let s_40_5: u64 = fn_state.ga_44794._0;
        // D s_40_6: cast zx s_40_5 -> bv
        let s_40_6: Bits = Bits::new(s_40_5 as u128, 64u16);
        // D s_40_7: read-var t:i
        let s_40_7: i128 = fn_state.t;
        // D s_40_8: call X_set(s_40_7, s_40_0, s_40_6)
        let s_40_8: () = X_set(state, tracer, s_40_7, s_40_0, s_40_6);
        // N s_40_9: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call Halted(s_41_0)
        let s_41_1: bool = Halted(state, tracer, s_41_0);
        // N s_41_2: branch s_41_1 b46 b42
        if s_41_1 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#49897 <= s_42_0
        fn_state.gs_49897 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#49897:u8
        let s_43_0: bool = fn_state.gs_49897;
        // N s_43_1: branch s_43_0 b45 b44
        if s_43_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #24u : u8
        let s_44_0: u8 = 24;
        // C s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 8u16);
        // C s_44_2: cast zx s_44_1 -> i
        let s_44_2: i128 = (s_44_1.value() as i128);
        // C s_44_3: cast reint s_44_2 -> i64
        let s_44_3: i64 = (s_44_2 as i64);
        // C s_44_4: cast zx s_44_3 -> i
        let s_44_4: i128 = (i128::try_from(s_44_3).unwrap());
        // C s_44_5: const #424u : u32
        let s_44_5: u32 = 424;
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
        // N s_45_0: panic
        panic!("{:?}", ());
        // N s_45_1: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var __EDSCR_SDD:u8
        let s_46_0: bool = fn_state.u__EDSCR_SDD;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#49897 <= s_46_4
        fn_state.gs_49897 = s_46_4;
        // N s_46_6: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var __SCR_EL3_NS:u8
        let s_47_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 1u16);
        // C s_47_2: const #1u : u8
        let s_47_2: bool = true;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: write-var gs#49894 <= s_47_4
        fn_state.gs_49894 = s_47_4;
        // N s_47_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __MDCR_EL3_SBRBE:u8
        let s_48_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_48_1: const #0s : i
        let s_48_1: i128 = 0;
        // D s_48_2: cast zx s_48_0 -> bv
        let s_48_2: Bits = Bits::new(s_48_0 as u128, 2u16);
        // C s_48_3: const #1s : i64
        let s_48_3: i64 = 1;
        // C s_48_4: cast zx s_48_3 -> i
        let s_48_4: i128 = (i128::try_from(s_48_3).unwrap());
        // C s_48_5: const #0s : i
        let s_48_5: i128 = 0;
        // C s_48_6: add s_48_5 s_48_4
        let s_48_6: i128 = (s_48_5 + s_48_4);
        // D s_48_7: bit-extract s_48_2 s_48_1 s_48_6
        let s_48_7: Bits = (Bits::new(
            ((s_48_2) >> (s_48_1)).value(),
            u16::try_from(s_48_6).unwrap(),
        ));
        // D s_48_8: cast reint s_48_7 -> u8
        let s_48_8: bool = ((s_48_7.value()) != 0);
        // D s_48_9: cast zx s_48_8 -> bv
        let s_48_9: Bits = Bits::new(s_48_8 as u128, 1u16);
        // C s_48_10: const #0u : u8
        let s_48_10: bool = false;
        // C s_48_11: cast zx s_48_10 -> bv
        let s_48_11: Bits = Bits::new(s_48_10 as u128, 1u16);
        // D s_48_12: cmp-eq s_48_9 s_48_11
        let s_48_12: bool = ((s_48_9) == (s_48_11));
        // D s_48_13: not s_48_12
        let s_48_13: bool = !s_48_12;
        // N s_48_14: branch s_48_13 b51 b49
        if s_48_13 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #1u : u8
        let s_49_0: bool = true;
        // D s_49_1: write-var gs#49888 <= s_49_0
        fn_state.gs_49888 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#49888:u8
        let s_50_0: bool = fn_state.gs_49888;
        // D s_50_1: write-var gs#49893 <= s_50_0
        fn_state.gs_49893 = s_50_0;
        // N s_50_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#49888 <= s_51_0
        fn_state.gs_49888 = s_51_0;
        // N s_51_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call Halted(s_52_0)
        let s_52_1: bool = Halted(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b57 b53
        if s_52_1 {
            return block_57(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#49898 <= s_53_0
        fn_state.gs_49898 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#49898:u8
        let s_54_0: bool = fn_state.gs_49898;
        // N s_54_1: branch s_54_0 b56 b55
        if s_54_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #24u : u8
        let s_55_0: u8 = 24;
        // C s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 8u16);
        // C s_55_2: cast zx s_55_1 -> i
        let s_55_2: i128 = (s_55_1.value() as i128);
        // C s_55_3: cast reint s_55_2 -> i64
        let s_55_3: i64 = (s_55_2 as i64);
        // C s_55_4: cast zx s_55_3 -> i
        let s_55_4: i128 = (i128::try_from(s_55_3).unwrap());
        // C s_55_5: const #424u : u32
        let s_55_5: u32 = 424;
        // D s_55_6: read-reg s_55_5:u8
        let s_55_6: u8 = {
            let value = state.read_register::<u8>(s_55_5 as isize);
            tracer.read_register(s_55_5 as isize, value);
            value
        };
        // D s_55_7: call AArch64_SystemAccessTrap(s_55_6, s_55_4)
        let s_55_7: () = AArch64_SystemAccessTrap(state, tracer, s_55_6, s_55_4);
        // N s_55_8: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_56_0: panic
        panic!("{:?}", ());
        // N s_56_1: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var __EDSCR_SDD:u8
        let s_57_0: bool = fn_state.u__EDSCR_SDD;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 1u16);
        // C s_57_2: const #1u : u8
        let s_57_2: bool = true;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // D s_57_4: cmp-eq s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) == (s_57_3));
        // D s_57_5: write-var gs#49898 <= s_57_4
        fn_state.gs_49898 = s_57_4;
        // N s_57_6: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var __SCR_EL3_NS:u8
        let s_58_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 1u16);
        // C s_58_2: const #0u : u8
        let s_58_2: bool = false;
        // C s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_4: cmp-eq s_58_1 s_58_3
        let s_58_4: bool = ((s_58_1) == (s_58_3));
        // D s_58_5: write-var gs#49887 <= s_58_4
        fn_state.gs_49887 = s_58_4;
        // N s_58_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var __MDCR_EL3_SBRBE:u8
        let s_59_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 2u16);
        // C s_59_2: const #3u : u8
        let s_59_2: u8 = 3;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 2u16);
        // D s_59_4: cmp-ne s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) != (s_59_3));
        // D s_59_5: write-var gs#49886 <= s_59_4
        fn_state.gs_49886 = s_59_4;
        // N s_59_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_60_0: panic
        panic!("{:?}", ());
        // N s_60_1: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var __SCR_EL3_NS:u8
        let s_61_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 1u16);
        // C s_61_2: const #1u : u8
        let s_61_2: bool = true;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 1u16);
        // D s_61_4: cmp-eq s_61_1 s_61_3
        let s_61_4: bool = ((s_61_1) == (s_61_3));
        // D s_61_5: write-var gs#49885 <= s_61_4
        fn_state.gs_49885 = s_61_4;
        // N s_61_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var __MDCR_EL3_SBRBE:u8
        let s_62_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_62_1: const #0s : i
        let s_62_1: i128 = 0;
        // D s_62_2: cast zx s_62_0 -> bv
        let s_62_2: Bits = Bits::new(s_62_0 as u128, 2u16);
        // C s_62_3: const #1s : i64
        let s_62_3: i64 = 1;
        // C s_62_4: cast zx s_62_3 -> i
        let s_62_4: i128 = (i128::try_from(s_62_3).unwrap());
        // C s_62_5: const #0s : i
        let s_62_5: i128 = 0;
        // C s_62_6: add s_62_5 s_62_4
        let s_62_6: i128 = (s_62_5 + s_62_4);
        // D s_62_7: bit-extract s_62_2 s_62_1 s_62_6
        let s_62_7: Bits = (Bits::new(
            ((s_62_2) >> (s_62_1)).value(),
            u16::try_from(s_62_6).unwrap(),
        ));
        // D s_62_8: cast reint s_62_7 -> u8
        let s_62_8: bool = ((s_62_7.value()) != 0);
        // D s_62_9: cast zx s_62_8 -> bv
        let s_62_9: Bits = Bits::new(s_62_8 as u128, 1u16);
        // C s_62_10: const #0u : u8
        let s_62_10: bool = false;
        // C s_62_11: cast zx s_62_10 -> bv
        let s_62_11: Bits = Bits::new(s_62_10 as u128, 1u16);
        // D s_62_12: cmp-eq s_62_9 s_62_11
        let s_62_12: bool = ((s_62_9) == (s_62_11));
        // D s_62_13: not s_62_12
        let s_62_13: bool = !s_62_12;
        // N s_62_14: branch s_62_13 b65 b63
        if s_62_13 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // D s_63_1: write-var gs#49879 <= s_63_0
        fn_state.gs_49879 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#49879:u8
        let s_64_0: bool = fn_state.gs_49879;
        // D s_64_1: write-var gs#49884 <= s_64_0
        fn_state.gs_49884 = s_64_0;
        // N s_64_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#49879 <= s_65_0
        fn_state.gs_49879 = s_65_0;
        // N s_65_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_66_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_66_1: call __IMPDEF_boolean(s_66_0)
        let s_66_1: bool = u__IMPDEF_boolean(state, tracer, s_66_0);
        // D s_66_2: write-var gs#49878 <= s_66_1
        fn_state.gs_49878 = s_66_1;
        // N s_66_3: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __EDSCR_SDD:u8
        let s_67_0: bool = fn_state.u__EDSCR_SDD;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#49877 <= s_67_4
        fn_state.gs_49877 = s_67_4;
        // N s_67_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #424u : u32
        let s_68_0: u32 = 424;
        // D s_68_1: read-reg s_68_0:u8
        let s_68_1: u8 = {
            let value = state.read_register::<u8>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // C s_68_2: const #2u : u8
        let s_68_2: u8 = 2;
        // D s_68_3: cmp-lt s_68_1 s_68_2
        let s_68_3: bool = ((s_68_1) < (s_68_2));
        // D s_68_4: write-var gs#49876 <= s_68_3
        fn_state.gs_49876 = s_68_3;
        // N s_68_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_69_0: panic
        panic!("{:?}", ());
        // N s_69_1: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __SCR_EL3_NS:u8
        let s_70_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #0u : u8
        let s_70_2: bool = false;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#49875 <= s_70_4
        fn_state.gs_49875 = s_70_4;
        // N s_70_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __MDCR_EL3_SBRBE:u8
        let s_71_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 2u16);
        // C s_71_2: const #3u : u8
        let s_71_2: u8 = 3;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 2u16);
        // D s_71_4: cmp-ne s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) != (s_71_3));
        // D s_71_5: write-var gs#49874 <= s_71_4
        fn_state.gs_49874 = s_71_4;
        // N s_71_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_72_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_72_1: call __IMPDEF_boolean(s_72_0)
        let s_72_1: bool = u__IMPDEF_boolean(state, tracer, s_72_0);
        // D s_72_2: write-var gs#49873 <= s_72_1
        fn_state.gs_49873 = s_72_1;
        // N s_72_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __EDSCR_SDD:u8
        let s_73_0: bool = fn_state.u__EDSCR_SDD;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#49872 <= s_73_4
        fn_state.gs_49872 = s_73_4;
        // N s_73_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #424u : u32
        let s_74_0: u32 = 424;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // C s_74_2: const #2u : u8
        let s_74_2: u8 = 2;
        // D s_74_3: cmp-lt s_74_1 s_74_2
        let s_74_3: bool = ((s_74_1) < (s_74_2));
        // D s_74_4: write-var gs#49871 <= s_74_3
        fn_state.gs_49871 = s_74_3;
        // N s_74_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #() : ()
        let s_75_0: () = ();
        // S s_75_1: call Halted(s_75_0)
        let s_75_1: bool = Halted(state, tracer, s_75_0);
        // N s_75_2: branch s_75_1 b161 b76
        if s_75_1 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#49899 <= s_76_0
        fn_state.gs_49899 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#49899:u8
        let s_77_0: bool = fn_state.gs_49899;
        // N s_77_1: branch s_77_0 b160 b78
        if s_77_0 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #0u : u8
        let s_78_0: bool = false;
        // D s_78_1: write-var gs#49900 <= s_78_0
        fn_state.gs_49900 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#49900:u8
        let s_79_0: bool = fn_state.gs_49900;
        // N s_79_1: branch s_79_0 b159 b80
        if s_79_0 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var gs#49901 <= s_80_0
        fn_state.gs_49901 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#49901:u8
        let s_81_0: bool = fn_state.gs_49901;
        // N s_81_1: branch s_81_0 b158 b82
        if s_81_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #0u : u8
        let s_82_0: bool = false;
        // D s_82_1: write-var gs#49902 <= s_82_0
        fn_state.gs_49902 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#49902:u8
        let s_83_0: bool = fn_state.gs_49902;
        // N s_83_1: branch s_83_0 b157 b84
        if s_83_0 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var gs#49903 <= s_84_0
        fn_state.gs_49903 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#49903:u8
        let s_85_0: bool = fn_state.gs_49903;
        // N s_85_1: branch s_85_0 b156 b86
        if s_85_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #() : ()
        let s_86_0: () = ();
        // S s_86_1: call Halted(s_86_0)
        let s_86_1: bool = Halted(state, tracer, s_86_0);
        // N s_86_2: branch s_86_1 b155 b87
        if s_86_1 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#49904 <= s_87_0
        fn_state.gs_49904 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#49904:u8
        let s_88_0: bool = fn_state.gs_49904;
        // N s_88_1: branch s_88_0 b154 b89
        if s_88_0 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#49905 <= s_89_0
        fn_state.gs_49905 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#49905:u8
        let s_90_0: bool = fn_state.gs_49905;
        // N s_90_1: branch s_90_0 b153 b91
        if s_90_0 {
            return block_153(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #0u : u8
        let s_91_0: bool = false;
        // D s_91_1: write-var gs#49906 <= s_91_0
        fn_state.gs_49906 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#49906:u8
        let s_92_0: bool = fn_state.gs_49906;
        // N s_92_1: branch s_92_0 b149 b93
        if s_92_0 {
            return block_149(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#49912 <= s_93_0
        fn_state.gs_49912 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#49912:u8
        let s_94_0: bool = fn_state.gs_49912;
        // N s_94_1: branch s_94_0 b148 b95
        if s_94_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #0u : u8
        let s_95_0: bool = false;
        // D s_95_1: write-var gs#49913 <= s_95_0
        fn_state.gs_49913 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#49913:u8
        let s_96_0: bool = fn_state.gs_49913;
        // N s_96_1: branch s_96_0 b147 b97
        if s_96_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #() : ()
        let s_97_0: () = ();
        // S s_97_1: call EL2Enabled(s_97_0)
        let s_97_1: bool = EL2Enabled(state, tracer, s_97_0);
        // N s_97_2: branch s_97_1 b146 b98
        if s_97_1 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#49914 <= s_98_0
        fn_state.gs_49914 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#49914:u8
        let s_99_0: bool = fn_state.gs_49914;
        // N s_99_1: branch s_99_0 b142 b100
        if s_99_0 {
            return block_142(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #0u : u8
        let s_100_0: bool = false;
        // D s_100_1: write-var gs#49916 <= s_100_0
        fn_state.gs_49916 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#49916:u8
        let s_101_0: bool = fn_state.gs_49916;
        // N s_101_1: branch s_101_0 b141 b102
        if s_101_0 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #0u : u8
        let s_102_0: bool = false;
        // D s_102_1: write-var gs#49917 <= s_102_0
        fn_state.gs_49917 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#49917:u8
        let s_103_0: bool = fn_state.gs_49917;
        // N s_103_1: branch s_103_0 b140 b104
        if s_103_0 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #424u : u32
        let s_104_0: u32 = 424;
        // D s_104_1: read-reg s_104_0:u8
        let s_104_1: u8 = {
            let value = state.read_register::<u8>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // C s_104_2: const #2u : u8
        let s_104_2: u8 = 2;
        // D s_104_3: cmp-lt s_104_1 s_104_2
        let s_104_3: bool = ((s_104_1) < (s_104_2));
        // N s_104_4: branch s_104_3 b139 b105
        if s_104_3 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#49918 <= s_105_0
        fn_state.gs_49918 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#49918:u8
        let s_106_0: bool = fn_state.gs_49918;
        // N s_106_1: branch s_106_0 b138 b107
        if s_106_0 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#49919 <= s_107_0
        fn_state.gs_49919 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#49919:u8
        let s_108_0: bool = fn_state.gs_49919;
        // N s_108_1: branch s_108_0 b132 b109
        if s_108_0 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #424u : u32
        let s_109_0: u32 = 424;
        // D s_109_1: read-reg s_109_0:u8
        let s_109_1: u8 = {
            let value = state.read_register::<u8>(s_109_0 as isize);
            tracer.read_register(s_109_0 as isize, value);
            value
        };
        // C s_109_2: const #2u : u8
        let s_109_2: u8 = 2;
        // D s_109_3: cmp-lt s_109_1 s_109_2
        let s_109_3: bool = ((s_109_1) < (s_109_2));
        // N s_109_4: branch s_109_3 b128 b110
        if s_109_3 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #0u : u8
        let s_110_0: bool = false;
        // D s_110_1: write-var gs#49925 <= s_110_0
        fn_state.gs_49925 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#49925:u8
        let s_111_0: bool = fn_state.gs_49925;
        // N s_111_1: branch s_111_0 b127 b112
        if s_111_0 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // D s_112_1: write-var gs#49926 <= s_112_0
        fn_state.gs_49926 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#49926:u8
        let s_113_0: bool = fn_state.gs_49926;
        // N s_113_1: branch s_113_0 b121 b114
        if s_113_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #() : ()
        let s_114_0: () = ();
        // S s_114_1: call EL2Enabled(s_114_0)
        let s_114_1: bool = EL2Enabled(state, tracer, s_114_0);
        // N s_114_2: branch s_114_1 b120 b115
        if s_114_1 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #0u : u8
        let s_115_0: bool = false;
        // D s_115_1: write-var gs#49927 <= s_115_0
        fn_state.gs_49927 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#49927:u8
        let s_116_0: bool = fn_state.gs_49927;
        // N s_116_1: branch s_116_0 b118 b117
        if s_116_0 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #64s : i64
        let s_117_0: i64 = 64;
        // C s_117_1: const #90640u : u32
        let s_117_1: u32 = 90640;
        // D s_117_2: read-reg s_117_1:struct
        let s_117_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_117_1 as isize);
            tracer.read_register(s_117_1 as isize, value);
            value
        };
        // D s_117_3: call __get_BRBCR_EL1(s_117_2)
        let s_117_3: ProductType5c790c8ef59cc8b2 = u__get_BRBCR_EL1(
            state,
            tracer,
            s_117_2,
        );
        // D s_117_4: write-var ga#44759 <= s_117_3
        fn_state.ga_44759 = s_117_3;
        // D s_117_5: read-var ga#44759.0:struct
        let s_117_5: u64 = fn_state.ga_44759._0;
        // D s_117_6: cast zx s_117_5 -> bv
        let s_117_6: Bits = Bits::new(s_117_5 as u128, 64u16);
        // D s_117_7: read-var t:i
        let s_117_7: i128 = fn_state.t;
        // D s_117_8: call X_set(s_117_7, s_117_0, s_117_6)
        let s_117_8: () = X_set(state, tracer, s_117_7, s_117_0, s_117_6);
        // N s_117_9: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #2272u : u12
        let s_118_0: u16 = 2272;
        // C s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 12u16);
        // C s_118_2: cast zx s_118_1 -> i
        let s_118_2: i128 = (s_118_1.value() as i128);
        // C s_118_3: cast reint s_118_2 -> i64
        let s_118_3: i64 = (s_118_2 as i64);
        // C s_118_4: cast zx s_118_3 -> i
        let s_118_4: i128 = (i128::try_from(s_118_3).unwrap());
        // S s_118_5: call NVMem_read(s_118_4)
        let s_118_5: u64 = NVMem_read(state, tracer, s_118_4);
        // D s_118_6: write-var ga#44758 <= s_118_5
        fn_state.ga_44758 = s_118_5;
        // N s_118_7: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var ga#44758:u64
        let s_119_0: u64 = fn_state.ga_44758;
        // D s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 64u16);
        // D s_119_2: read-var t:i
        let s_119_2: i128 = fn_state.t;
        // C s_119_3: const #64s : i64
        let s_119_3: i64 = 64;
        // D s_119_4: call X_set(s_119_2, s_119_3, s_119_1)
        let s_119_4: () = X_set(state, tracer, s_119_2, s_119_3, s_119_1);
        // N s_119_5: return
        return;
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #102552u : u32
        let s_120_0: u32 = 102552;
        // D s_120_1: read-reg s_120_0:struct
        let s_120_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_120_0 as isize);
            tracer.read_register(s_120_0 as isize, value);
            value
        };
        // D s_120_2: call _get_HCR_EL2_Type_NV2(s_120_1)
        let s_120_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_120_1);
        // C s_120_3: const #102552u : u32
        let s_120_3: u32 = 102552;
        // D s_120_4: read-reg s_120_3:struct
        let s_120_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_120_3 as isize);
            tracer.read_register(s_120_3 as isize, value);
            value
        };
        // D s_120_5: call _get_HCR_EL2_Type_NV1(s_120_4)
        let s_120_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_120_4);
        // C s_120_6: const #102552u : u32
        let s_120_6: u32 = 102552;
        // D s_120_7: read-reg s_120_6:struct
        let s_120_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_120_6 as isize);
            tracer.read_register(s_120_6 as isize, value);
            value
        };
        // D s_120_8: call _get_HCR_EL2_Type_NV(s_120_7)
        let s_120_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_120_7);
        // D s_120_9: cast zx s_120_5 -> bv
        let s_120_9: Bits = Bits::new(s_120_5 as u128, 1u16);
        // D s_120_10: cast zx s_120_8 -> bv
        let s_120_10: Bits = Bits::new(s_120_8 as u128, 1u16);
        // D s_120_11: cast reint s_120_9 -> u128
        let s_120_11: u128 = (s_120_9.value() as u128);
        // D s_120_12: size-of s_120_9
        let s_120_12: u16 = s_120_9.length();
        // D s_120_13: cast reint s_120_10 -> u128
        let s_120_13: u128 = (s_120_10.value() as u128);
        // D s_120_14: size-of s_120_10
        let s_120_14: u16 = s_120_10.length();
        // D s_120_15: lsl s_120_11 s_120_14
        let s_120_15: u128 = s_120_11 << s_120_14;
        // D s_120_16: or s_120_15 s_120_13
        let s_120_16: u128 = ((s_120_15) | (s_120_13));
        // D s_120_17: add s_120_12 s_120_14
        let s_120_17: u16 = (s_120_12 + s_120_14);
        // D s_120_18: create-bits s_120_16 s_120_17
        let s_120_18: Bits = Bits::new(s_120_16, s_120_17);
        // D s_120_19: cast reint s_120_18 -> u8
        let s_120_19: u8 = (s_120_18.value() as u8);
        // D s_120_20: cast zx s_120_2 -> bv
        let s_120_20: Bits = Bits::new(s_120_2 as u128, 1u16);
        // D s_120_21: cast zx s_120_19 -> bv
        let s_120_21: Bits = Bits::new(s_120_19 as u128, 2u16);
        // D s_120_22: cast reint s_120_20 -> u128
        let s_120_22: u128 = (s_120_20.value() as u128);
        // D s_120_23: size-of s_120_20
        let s_120_23: u16 = s_120_20.length();
        // D s_120_24: cast reint s_120_21 -> u128
        let s_120_24: u128 = (s_120_21.value() as u128);
        // D s_120_25: size-of s_120_21
        let s_120_25: u16 = s_120_21.length();
        // D s_120_26: lsl s_120_22 s_120_25
        let s_120_26: u128 = s_120_22 << s_120_25;
        // D s_120_27: or s_120_26 s_120_24
        let s_120_27: u128 = ((s_120_26) | (s_120_24));
        // D s_120_28: add s_120_23 s_120_25
        let s_120_28: u16 = (s_120_23 + s_120_25);
        // D s_120_29: create-bits s_120_27 s_120_28
        let s_120_29: Bits = Bits::new(s_120_27, s_120_28);
        // D s_120_30: cast reint s_120_29 -> u8
        let s_120_30: u8 = (s_120_29.value() as u8);
        // D s_120_31: cast zx s_120_30 -> bv
        let s_120_31: Bits = Bits::new(s_120_30 as u128, 3u16);
        // C s_120_32: const #7u : u8
        let s_120_32: u8 = 7;
        // C s_120_33: cast zx s_120_32 -> bv
        let s_120_33: Bits = Bits::new(s_120_32 as u128, 3u16);
        // D s_120_34: cmp-eq s_120_31 s_120_33
        let s_120_34: bool = ((s_120_31) == (s_120_33));
        // D s_120_35: write-var gs#49927 <= s_120_34
        fn_state.gs_49927 = s_120_34;
        // N s_120_36: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #() : ()
        let s_121_0: () = ();
        // S s_121_1: call Halted(s_121_0)
        let s_121_1: bool = Halted(state, tracer, s_121_0);
        // N s_121_2: branch s_121_1 b126 b122
        if s_121_1 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #0u : u8
        let s_122_0: bool = false;
        // D s_122_1: write-var gs#49930 <= s_122_0
        fn_state.gs_49930 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#49930:u8
        let s_123_0: bool = fn_state.gs_49930;
        // N s_123_1: branch s_123_0 b125 b124
        if s_123_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #24u : u8
        let s_124_0: u8 = 24;
        // C s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 8u16);
        // C s_124_2: cast zx s_124_1 -> i
        let s_124_2: i128 = (s_124_1.value() as i128);
        // C s_124_3: cast reint s_124_2 -> i64
        let s_124_3: i64 = (s_124_2 as i64);
        // C s_124_4: cast zx s_124_3 -> i
        let s_124_4: i128 = (i128::try_from(s_124_3).unwrap());
        // C s_124_5: const #424u : u32
        let s_124_5: u32 = 424;
        // D s_124_6: read-reg s_124_5:u8
        let s_124_6: u8 = {
            let value = state.read_register::<u8>(s_124_5 as isize);
            tracer.read_register(s_124_5 as isize, value);
            value
        };
        // D s_124_7: call AArch64_SystemAccessTrap(s_124_6, s_124_4)
        let s_124_7: () = AArch64_SystemAccessTrap(state, tracer, s_124_6, s_124_4);
        // N s_124_8: return
        return;
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_125_0: panic
        panic!("{:?}", ());
        // N s_125_1: return
        return;
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var __EDSCR_SDD:u8
        let s_126_0: bool = fn_state.u__EDSCR_SDD;
        // D s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 1u16);
        // C s_126_2: const #1u : u8
        let s_126_2: bool = true;
        // C s_126_3: cast zx s_126_2 -> bv
        let s_126_3: Bits = Bits::new(s_126_2 as u128, 1u16);
        // D s_126_4: cmp-eq s_126_1 s_126_3
        let s_126_4: bool = ((s_126_1) == (s_126_3));
        // D s_126_5: write-var gs#49930 <= s_126_4
        fn_state.gs_49930 = s_126_4;
        // N s_126_6: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var __SCR_EL3_NS:u8
        let s_127_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_127_1: cast zx s_127_0 -> bv
        let s_127_1: Bits = Bits::new(s_127_0 as u128, 1u16);
        // C s_127_2: const #1u : u8
        let s_127_2: bool = true;
        // C s_127_3: cast zx s_127_2 -> bv
        let s_127_3: Bits = Bits::new(s_127_2 as u128, 1u16);
        // D s_127_4: cmp-eq s_127_1 s_127_3
        let s_127_4: bool = ((s_127_1) == (s_127_3));
        // D s_127_5: write-var gs#49926 <= s_127_4
        fn_state.gs_49926 = s_127_4;
        // N s_127_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var __MDCR_EL3_SBRBE:u8
        let s_128_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_128_1: const #0s : i
        let s_128_1: i128 = 0;
        // D s_128_2: cast zx s_128_0 -> bv
        let s_128_2: Bits = Bits::new(s_128_0 as u128, 2u16);
        // C s_128_3: const #1s : i64
        let s_128_3: i64 = 1;
        // C s_128_4: cast zx s_128_3 -> i
        let s_128_4: i128 = (i128::try_from(s_128_3).unwrap());
        // C s_128_5: const #0s : i
        let s_128_5: i128 = 0;
        // C s_128_6: add s_128_5 s_128_4
        let s_128_6: i128 = (s_128_5 + s_128_4);
        // D s_128_7: bit-extract s_128_2 s_128_1 s_128_6
        let s_128_7: Bits = (Bits::new(
            ((s_128_2) >> (s_128_1)).value(),
            u16::try_from(s_128_6).unwrap(),
        ));
        // D s_128_8: cast reint s_128_7 -> u8
        let s_128_8: bool = ((s_128_7.value()) != 0);
        // D s_128_9: cast zx s_128_8 -> bv
        let s_128_9: Bits = Bits::new(s_128_8 as u128, 1u16);
        // C s_128_10: const #0u : u8
        let s_128_10: bool = false;
        // C s_128_11: cast zx s_128_10 -> bv
        let s_128_11: Bits = Bits::new(s_128_10 as u128, 1u16);
        // D s_128_12: cmp-eq s_128_9 s_128_11
        let s_128_12: bool = ((s_128_9) == (s_128_11));
        // D s_128_13: not s_128_12
        let s_128_13: bool = !s_128_12;
        // N s_128_14: branch s_128_13 b131 b129
        if s_128_13 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #1u : u8
        let s_129_0: bool = true;
        // D s_129_1: write-var gs#49920 <= s_129_0
        fn_state.gs_49920 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#49920:u8
        let s_130_0: bool = fn_state.gs_49920;
        // D s_130_1: write-var gs#49925 <= s_130_0
        fn_state.gs_49925 = s_130_0;
        // N s_130_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #0u : u8
        let s_131_0: bool = false;
        // D s_131_1: write-var gs#49920 <= s_131_0
        fn_state.gs_49920 = s_131_0;
        // N s_131_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #() : ()
        let s_132_0: () = ();
        // S s_132_1: call Halted(s_132_0)
        let s_132_1: bool = Halted(state, tracer, s_132_0);
        // N s_132_2: branch s_132_1 b137 b133
        if s_132_1 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #0u : u8
        let s_133_0: bool = false;
        // D s_133_1: write-var gs#49931 <= s_133_0
        fn_state.gs_49931 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#49931:u8
        let s_134_0: bool = fn_state.gs_49931;
        // N s_134_1: branch s_134_0 b136 b135
        if s_134_0 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #24u : u8
        let s_135_0: u8 = 24;
        // C s_135_1: cast zx s_135_0 -> bv
        let s_135_1: Bits = Bits::new(s_135_0 as u128, 8u16);
        // C s_135_2: cast zx s_135_1 -> i
        let s_135_2: i128 = (s_135_1.value() as i128);
        // C s_135_3: cast reint s_135_2 -> i64
        let s_135_3: i64 = (s_135_2 as i64);
        // C s_135_4: cast zx s_135_3 -> i
        let s_135_4: i128 = (i128::try_from(s_135_3).unwrap());
        // C s_135_5: const #424u : u32
        let s_135_5: u32 = 424;
        // D s_135_6: read-reg s_135_5:u8
        let s_135_6: u8 = {
            let value = state.read_register::<u8>(s_135_5 as isize);
            tracer.read_register(s_135_5 as isize, value);
            value
        };
        // D s_135_7: call AArch64_SystemAccessTrap(s_135_6, s_135_4)
        let s_135_7: () = AArch64_SystemAccessTrap(state, tracer, s_135_6, s_135_4);
        // N s_135_8: return
        return;
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_136_0: panic
        panic!("{:?}", ());
        // N s_136_1: return
        return;
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var __EDSCR_SDD:u8
        let s_137_0: bool = fn_state.u__EDSCR_SDD;
        // D s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 1u16);
        // C s_137_2: const #1u : u8
        let s_137_2: bool = true;
        // C s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 1u16);
        // D s_137_4: cmp-eq s_137_1 s_137_3
        let s_137_4: bool = ((s_137_1) == (s_137_3));
        // D s_137_5: write-var gs#49931 <= s_137_4
        fn_state.gs_49931 = s_137_4;
        // N s_137_6: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var __SCR_EL3_NS:u8
        let s_138_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_138_1: cast zx s_138_0 -> bv
        let s_138_1: Bits = Bits::new(s_138_0 as u128, 1u16);
        // C s_138_2: const #0u : u8
        let s_138_2: bool = false;
        // C s_138_3: cast zx s_138_2 -> bv
        let s_138_3: Bits = Bits::new(s_138_2 as u128, 1u16);
        // D s_138_4: cmp-eq s_138_1 s_138_3
        let s_138_4: bool = ((s_138_1) == (s_138_3));
        // D s_138_5: write-var gs#49919 <= s_138_4
        fn_state.gs_49919 = s_138_4;
        // N s_138_6: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var __MDCR_EL3_SBRBE:u8
        let s_139_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_139_1: cast zx s_139_0 -> bv
        let s_139_1: Bits = Bits::new(s_139_0 as u128, 2u16);
        // C s_139_2: const #3u : u8
        let s_139_2: u8 = 3;
        // C s_139_3: cast zx s_139_2 -> bv
        let s_139_3: Bits = Bits::new(s_139_2 as u128, 2u16);
        // D s_139_4: cmp-ne s_139_1 s_139_3
        let s_139_4: bool = ((s_139_1) != (s_139_3));
        // D s_139_5: write-var gs#49918 <= s_139_4
        fn_state.gs_49918 = s_139_4;
        // N s_139_6: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #24u : u8
        let s_140_0: u8 = 24;
        // C s_140_1: cast zx s_140_0 -> bv
        let s_140_1: Bits = Bits::new(s_140_0 as u128, 8u16);
        // C s_140_2: cast zx s_140_1 -> i
        let s_140_2: i128 = (s_140_1.value() as i128);
        // C s_140_3: cast reint s_140_2 -> i64
        let s_140_3: i64 = (s_140_2 as i64);
        // C s_140_4: cast zx s_140_3 -> i
        let s_140_4: i128 = (i128::try_from(s_140_3).unwrap());
        // C s_140_5: const #432u : u32
        let s_140_5: u32 = 432;
        // D s_140_6: read-reg s_140_5:u8
        let s_140_6: u8 = {
            let value = state.read_register::<u8>(s_140_5 as isize);
            tracer.read_register(s_140_5 as isize, value);
            value
        };
        // D s_140_7: call AArch64_SystemAccessTrap(s_140_6, s_140_4)
        let s_140_7: () = AArch64_SystemAccessTrap(state, tracer, s_140_6, s_140_4);
        // N s_140_8: return
        return;
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var __HDFGRTR_EL2_nBRBCTL:u8
        let s_141_0: bool = fn_state.u__HDFGRTR_EL2_nBRBCTL;
        // D s_141_1: cast zx s_141_0 -> bv
        let s_141_1: Bits = Bits::new(s_141_0 as u128, 1u16);
        // C s_141_2: const #0u : u8
        let s_141_2: bool = false;
        // C s_141_3: cast zx s_141_2 -> bv
        let s_141_3: Bits = Bits::new(s_141_2 as u128, 1u16);
        // D s_141_4: cmp-eq s_141_1 s_141_3
        let s_141_4: bool = ((s_141_1) == (s_141_3));
        // D s_141_5: write-var gs#49917 <= s_141_4
        fn_state.gs_49917 = s_141_4;
        // N s_141_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #424u : u32
        let s_142_0: u32 = 424;
        // D s_142_1: read-reg s_142_0:u8
        let s_142_1: u8 = {
            let value = state.read_register::<u8>(s_142_0 as isize);
            tracer.read_register(s_142_0 as isize, value);
            value
        };
        // C s_142_2: const #2u : u8
        let s_142_2: u8 = 2;
        // D s_142_3: cmp-lt s_142_1 s_142_2
        let s_142_3: bool = ((s_142_1) < (s_142_2));
        // D s_142_4: not s_142_3
        let s_142_4: bool = !s_142_3;
        // N s_142_5: branch s_142_4 b145 b143
        if s_142_4 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var __SCR_EL3_FGTEn:u8
        let s_143_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_143_1: cast zx s_143_0 -> bv
        let s_143_1: Bits = Bits::new(s_143_0 as u128, 1u16);
        // C s_143_2: const #1u : u8
        let s_143_2: bool = true;
        // C s_143_3: cast zx s_143_2 -> bv
        let s_143_3: Bits = Bits::new(s_143_2 as u128, 1u16);
        // D s_143_4: cmp-eq s_143_1 s_143_3
        let s_143_4: bool = ((s_143_1) == (s_143_3));
        // D s_143_5: write-var gs#49915 <= s_143_4
        fn_state.gs_49915 = s_143_4;
        // N s_143_6: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#49915:u8
        let s_144_0: bool = fn_state.gs_49915;
        // D s_144_1: write-var gs#49916 <= s_144_0
        fn_state.gs_49916 = s_144_0;
        // N s_144_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #1u : u8
        let s_145_0: bool = true;
        // D s_145_1: write-var gs#49915 <= s_145_0
        fn_state.gs_49915 = s_145_0;
        // N s_145_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #146u : u32
        let s_146_0: u32 = 146;
        // S s_146_1: call IsFeatureImplemented(s_146_0)
        let s_146_1: bool = IsFeatureImplemented(state, tracer, s_146_0);
        // D s_146_2: write-var gs#49914 <= s_146_1
        fn_state.gs_49914 = s_146_1;
        // N s_146_3: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_147_0: panic
        panic!("{:?}", ());
        // N s_147_1: return
        return;
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var __SCR_EL3_NS:u8
        let s_148_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_148_1: cast zx s_148_0 -> bv
        let s_148_1: Bits = Bits::new(s_148_0 as u128, 1u16);
        // C s_148_2: const #1u : u8
        let s_148_2: bool = true;
        // C s_148_3: cast zx s_148_2 -> bv
        let s_148_3: Bits = Bits::new(s_148_2 as u128, 1u16);
        // D s_148_4: cmp-eq s_148_1 s_148_3
        let s_148_4: bool = ((s_148_1) == (s_148_3));
        // D s_148_5: write-var gs#49913 <= s_148_4
        fn_state.gs_49913 = s_148_4;
        // N s_148_6: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var __MDCR_EL3_SBRBE:u8
        let s_149_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_149_1: const #0s : i
        let s_149_1: i128 = 0;
        // D s_149_2: cast zx s_149_0 -> bv
        let s_149_2: Bits = Bits::new(s_149_0 as u128, 2u16);
        // C s_149_3: const #1s : i64
        let s_149_3: i64 = 1;
        // C s_149_4: cast zx s_149_3 -> i
        let s_149_4: i128 = (i128::try_from(s_149_3).unwrap());
        // C s_149_5: const #0s : i
        let s_149_5: i128 = 0;
        // C s_149_6: add s_149_5 s_149_4
        let s_149_6: i128 = (s_149_5 + s_149_4);
        // D s_149_7: bit-extract s_149_2 s_149_1 s_149_6
        let s_149_7: Bits = (Bits::new(
            ((s_149_2) >> (s_149_1)).value(),
            u16::try_from(s_149_6).unwrap(),
        ));
        // D s_149_8: cast reint s_149_7 -> u8
        let s_149_8: bool = ((s_149_7.value()) != 0);
        // D s_149_9: cast zx s_149_8 -> bv
        let s_149_9: Bits = Bits::new(s_149_8 as u128, 1u16);
        // C s_149_10: const #0u : u8
        let s_149_10: bool = false;
        // C s_149_11: cast zx s_149_10 -> bv
        let s_149_11: Bits = Bits::new(s_149_10 as u128, 1u16);
        // D s_149_12: cmp-eq s_149_9 s_149_11
        let s_149_12: bool = ((s_149_9) == (s_149_11));
        // D s_149_13: not s_149_12
        let s_149_13: bool = !s_149_12;
        // N s_149_14: branch s_149_13 b152 b150
        if s_149_13 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #1u : u8
        let s_150_0: bool = true;
        // D s_150_1: write-var gs#49907 <= s_150_0
        fn_state.gs_49907 = s_150_0;
        // N s_150_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var gs#49907:u8
        let s_151_0: bool = fn_state.gs_49907;
        // D s_151_1: write-var gs#49912 <= s_151_0
        fn_state.gs_49912 = s_151_0;
        // N s_151_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #0u : u8
        let s_152_0: bool = false;
        // D s_152_1: write-var gs#49907 <= s_152_0
        fn_state.gs_49907 = s_152_0;
        // N s_152_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_153_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_153_1: call __IMPDEF_boolean(s_153_0)
        let s_153_1: bool = u__IMPDEF_boolean(state, tracer, s_153_0);
        // D s_153_2: write-var gs#49906 <= s_153_1
        fn_state.gs_49906 = s_153_1;
        // N s_153_3: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var __EDSCR_SDD:u8
        let s_154_0: bool = fn_state.u__EDSCR_SDD;
        // D s_154_1: cast zx s_154_0 -> bv
        let s_154_1: Bits = Bits::new(s_154_0 as u128, 1u16);
        // C s_154_2: const #1u : u8
        let s_154_2: bool = true;
        // C s_154_3: cast zx s_154_2 -> bv
        let s_154_3: Bits = Bits::new(s_154_2 as u128, 1u16);
        // D s_154_4: cmp-eq s_154_1 s_154_3
        let s_154_4: bool = ((s_154_1) == (s_154_3));
        // D s_154_5: write-var gs#49905 <= s_154_4
        fn_state.gs_49905 = s_154_4;
        // N s_154_6: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #424u : u32
        let s_155_0: u32 = 424;
        // D s_155_1: read-reg s_155_0:u8
        let s_155_1: u8 = {
            let value = state.read_register::<u8>(s_155_0 as isize);
            tracer.read_register(s_155_0 as isize, value);
            value
        };
        // C s_155_2: const #2u : u8
        let s_155_2: u8 = 2;
        // D s_155_3: cmp-lt s_155_1 s_155_2
        let s_155_3: bool = ((s_155_1) < (s_155_2));
        // D s_155_4: write-var gs#49904 <= s_155_3
        fn_state.gs_49904 = s_155_3;
        // N s_155_5: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_156_0: panic
        panic!("{:?}", ());
        // N s_156_1: return
        return;
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var __SCR_EL3_NS:u8
        let s_157_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_157_1: cast zx s_157_0 -> bv
        let s_157_1: Bits = Bits::new(s_157_0 as u128, 1u16);
        // C s_157_2: const #0u : u8
        let s_157_2: bool = false;
        // C s_157_3: cast zx s_157_2 -> bv
        let s_157_3: Bits = Bits::new(s_157_2 as u128, 1u16);
        // D s_157_4: cmp-eq s_157_1 s_157_3
        let s_157_4: bool = ((s_157_1) == (s_157_3));
        // D s_157_5: write-var gs#49903 <= s_157_4
        fn_state.gs_49903 = s_157_4;
        // N s_157_6: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var __MDCR_EL3_SBRBE:u8
        let s_158_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_158_1: cast zx s_158_0 -> bv
        let s_158_1: Bits = Bits::new(s_158_0 as u128, 2u16);
        // C s_158_2: const #3u : u8
        let s_158_2: u8 = 3;
        // C s_158_3: cast zx s_158_2 -> bv
        let s_158_3: Bits = Bits::new(s_158_2 as u128, 2u16);
        // D s_158_4: cmp-ne s_158_1 s_158_3
        let s_158_4: bool = ((s_158_1) != (s_158_3));
        // D s_158_5: write-var gs#49902 <= s_158_4
        fn_state.gs_49902 = s_158_4;
        // N s_158_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_159_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_159_1: call __IMPDEF_boolean(s_159_0)
        let s_159_1: bool = u__IMPDEF_boolean(state, tracer, s_159_0);
        // D s_159_2: write-var gs#49901 <= s_159_1
        fn_state.gs_49901 = s_159_1;
        // N s_159_3: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var __EDSCR_SDD:u8
        let s_160_0: bool = fn_state.u__EDSCR_SDD;
        // D s_160_1: cast zx s_160_0 -> bv
        let s_160_1: Bits = Bits::new(s_160_0 as u128, 1u16);
        // C s_160_2: const #1u : u8
        let s_160_2: bool = true;
        // C s_160_3: cast zx s_160_2 -> bv
        let s_160_3: Bits = Bits::new(s_160_2 as u128, 1u16);
        // D s_160_4: cmp-eq s_160_1 s_160_3
        let s_160_4: bool = ((s_160_1) == (s_160_3));
        // D s_160_5: write-var gs#49900 <= s_160_4
        fn_state.gs_49900 = s_160_4;
        // N s_160_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #424u : u32
        let s_161_0: u32 = 424;
        // D s_161_1: read-reg s_161_0:u8
        let s_161_1: u8 = {
            let value = state.read_register::<u8>(s_161_0 as isize);
            tracer.read_register(s_161_0 as isize, value);
            value
        };
        // C s_161_2: const #2u : u8
        let s_161_2: u8 = 2;
        // D s_161_3: cmp-lt s_161_1 s_161_2
        let s_161_3: bool = ((s_161_1) < (s_161_2));
        // D s_161_4: write-var gs#49899 <= s_161_3
        fn_state.gs_49899 = s_161_3;
        // N s_161_5: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_162_0: panic
        panic!("{:?}", ());
        // N s_162_1: return
        return;
    }
}
