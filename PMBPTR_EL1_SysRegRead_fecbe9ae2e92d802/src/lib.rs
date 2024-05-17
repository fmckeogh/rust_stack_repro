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
use Halted::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use u_get_HDFGRTR_EL2_Type_PMBPTR_EL1::*;
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_MDCR_EL3_Type_NSPB::*;
use NVMem_read::*;
use X_set::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_SCR_EL3_Type_NSE::*;
use u_get_MDCR_EL3_Type_NSPBE::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_MDCR_EL2_Type_E2PB::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn PMBPTR_EL1_SysRegRead_fecbe9ae2e92d802<T: Tracer>(
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
        gs_63840: bool,
        gs_63841: bool,
        gs_63844: bool,
        gs_63828: bool,
        u__MDCR_EL3_NSPBE: bool,
        u__HDFGRTR_EL2_PMBPTR_EL1: bool,
        gs_63842: bool,
        gs_63809: bool,
        gs_63826: bool,
        gs_63836: bool,
        u__PSTATE_EL: u8,
        gs_63856: bool,
        gs_63849: bool,
        gs_63815: bool,
        gs_63816: bool,
        gs_63823: bool,
        gs_63857: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_63854: bool,
        gs_63861: bool,
        gs_63837: bool,
        gs_63831: bool,
        gs_63839: bool,
        gs_63843: bool,
        gs_63817: bool,
        gs_63825: bool,
        ga_72638: u64,
        gs_63858: bool,
        gs_63818: bool,
        u__SCR_EL3_NSE: bool,
        gs_63808: bool,
        gs_63810: bool,
        gs_63829: bool,
        gs_63824: bool,
        gs_63830: bool,
        gs_63838: bool,
        u__MDCR_EL2_E2PB: u8,
        gs_63855: bool,
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
        // C s_0_3: const #22712u : u32
        let s_0_3: u32 = 22712;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDCR_EL3_Type_NSPBE(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_NSPBE(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_NSPBE <= s_0_5
        fn_state.u__MDCR_EL3_NSPBE = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_NSE(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_NSE(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_NSE <= s_0_9
        fn_state.u__SCR_EL3_NSE = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_FGTEn(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_FGTEn <= s_0_13
        fn_state.u__SCR_EL3_FGTEn = s_0_13;
        // C s_0_15: const #19144u : u32
        let s_0_15: u32 = 19144;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HDFGRTR_EL2_Type_PMBPTR_EL1(s_0_16)
        let s_0_17: bool = u_get_HDFGRTR_EL2_Type_PMBPTR_EL1(state, tracer, s_0_16);
        // D s_0_18: write-var __HDFGRTR_EL2_PMBPTR_EL1 <= s_0_17
        fn_state.u__HDFGRTR_EL2_PMBPTR_EL1 = s_0_17;
        // C s_0_19: const #104880u : u32
        let s_0_19: u32 = 104880;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_MDCR_EL2_Type_E2PB(s_0_20)
        let s_0_21: u8 = u_get_MDCR_EL2_Type_E2PB(state, tracer, s_0_20);
        // D s_0_22: write-var __MDCR_EL2_E2PB <= s_0_21
        fn_state.u__MDCR_EL2_E2PB = s_0_21;
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
        // N s_0_29: branch s_0_28 b120 b1
        if s_0_28 {
            return block_120(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b49 b2
        if s_1_5 {
            return block_49(state, tracer, fn_state);
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
        // C s_5_1: const #90512u : u32
        let s_5_1: u32 = 90512;
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
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call Halted(s_6_0)
        let s_6_1: bool = Halted(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b48 b7
        if s_6_1 {
            return block_48(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#63808 <= s_7_0
        fn_state.gs_63808 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#63808:u8
        let s_8_0: bool = fn_state.gs_63808;
        // N s_8_1: branch s_8_0 b47 b9
        if s_8_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#63809 <= s_9_0
        fn_state.gs_63809 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#63809:u8
        let s_10_0: bool = fn_state.gs_63809;
        // N s_10_1: branch s_10_0 b46 b11
        if s_10_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#63810 <= s_11_0
        fn_state.gs_63810 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#63810:u8
        let s_12_0: bool = fn_state.gs_63810;
        // N s_12_1: branch s_12_0 b36 b13
        if s_12_0 {
            return block_36(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#63818 <= s_13_0
        fn_state.gs_63818 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#63818:u8
        let s_14_0: bool = fn_state.gs_63818;
        // N s_14_1: branch s_14_0 b35 b15
        if s_14_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #424u : u32
        let s_15_0: u32 = 424;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // D s_15_3: cmp-lt s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) < (s_15_2));
        // N s_15_4: branch s_15_3 b25 b16
        if s_15_3 {
            return block_25(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#63826 <= s_16_0
        fn_state.gs_63826 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#63826:u8
        let s_17_0: bool = fn_state.gs_63826;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #64s : i64
        let s_18_0: i64 = 64;
        // C s_18_1: const #90512u : u32
        let s_18_1: u32 = 90512;
        // D s_18_2: read-reg s_18_1:u64
        let s_18_2: u64 = {
            let value = state.read_register::<u64>(s_18_1 as isize);
            tracer.read_register(s_18_1 as isize, value);
            value
        };
        // D s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 64u16);
        // D s_18_4: read-var t:i
        let s_18_4: i128 = fn_state.t;
        // D s_18_5: call X_set(s_18_4, s_18_0, s_18_3)
        let s_18_5: () = X_set(state, tracer, s_18_4, s_18_0, s_18_3);
        // N s_18_6: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call Halted(s_19_0)
        let s_19_1: bool = Halted(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b24 b20
        if s_19_1 {
            return block_24(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#63828 <= s_20_0
        fn_state.gs_63828 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#63828:u8
        let s_21_0: bool = fn_state.gs_63828;
        // N s_21_1: branch s_21_0 b23 b22
        if s_21_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #24u : u8
        let s_22_0: u8 = 24;
        // C s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 8u16);
        // C s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (s_22_1.value() as i128);
        // C s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #424u : u32
        let s_22_5: u32 = 424;
        // D s_22_6: read-reg s_22_5:u8
        let s_22_6: u8 = {
            let value = state.read_register::<u8>(s_22_5 as isize);
            tracer.read_register(s_22_5 as isize, value);
            value
        };
        // D s_22_7: call AArch64_SystemAccessTrap(s_22_6, s_22_4)
        let s_22_7: () = AArch64_SystemAccessTrap(state, tracer, s_22_6, s_22_4);
        // N s_22_8: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call EDSCR_read(s_24_0)
        let s_24_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_24_0);
        // S s_24_2: call _get_EDSCR_Type_SDD(s_24_1)
        let s_24_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_24_1);
        // S s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // C s_24_4: const #1u : u8
        let s_24_4: bool = true;
        // C s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 1u16);
        // S s_24_6: cmp-eq s_24_3 s_24_5
        let s_24_6: bool = ((s_24_3) == (s_24_5));
        // D s_24_7: write-var gs#63828 <= s_24_6
        fn_state.gs_63828 = s_24_6;
        // N s_24_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #22712u : u32
        let s_25_0: u32 = 22712;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_MDCR_EL3_Type_NSPB(s_25_1)
        let s_25_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_25_1);
        // C s_25_3: const #0s : i
        let s_25_3: i128 = 0;
        // D s_25_4: cast zx s_25_2 -> bv
        let s_25_4: Bits = Bits::new(s_25_2 as u128, 2u16);
        // C s_25_5: const #1u : u64
        let s_25_5: u64 = 1;
        // D s_25_6: bit-extract s_25_4 s_25_3 s_25_5
        let s_25_6: Bits = (Bits::new(
            ((s_25_4) >> (s_25_3)).value(),
            u16::try_from(s_25_5).unwrap(),
        ));
        // D s_25_7: cast reint s_25_6 -> u8
        let s_25_7: bool = ((s_25_6.value()) != 0);
        // C s_25_8: const #0s : i
        let s_25_8: i128 = 0;
        // C s_25_9: const #0u : u64
        let s_25_9: u64 = 0;
        // D s_25_10: cast zx s_25_7 -> u64
        let s_25_10: u64 = (s_25_7 as u64);
        // C s_25_11: const #1u : u64
        let s_25_11: u64 = 1;
        // D s_25_12: and s_25_10 s_25_11
        let s_25_12: u64 = ((s_25_10) & (s_25_11));
        // D s_25_13: cmp-eq s_25_12 s_25_11
        let s_25_13: bool = ((s_25_12) == (s_25_11));
        // D s_25_14: lsl s_25_10 s_25_8
        let s_25_14: u64 = s_25_10 << s_25_8;
        // D s_25_15: or s_25_9 s_25_14
        let s_25_15: u64 = ((s_25_9) | (s_25_14));
        // D s_25_16: cmpl s_25_14
        let s_25_16: u64 = !s_25_14;
        // D s_25_17: and s_25_9 s_25_16
        let s_25_17: u64 = ((s_25_9) & (s_25_16));
        // D s_25_18: select s_25_13 s_25_15 s_25_17
        let s_25_18: u64 = if s_25_13 { s_25_15 } else { s_25_17 };
        // D s_25_19: cast trunc s_25_18 -> u8
        let s_25_19: bool = ((s_25_18) != 0);
        // D s_25_20: cast zx s_25_19 -> bv
        let s_25_20: Bits = Bits::new(s_25_19 as u128, 1u16);
        // C s_25_21: const #0u : u8
        let s_25_21: bool = false;
        // C s_25_22: cast zx s_25_21 -> bv
        let s_25_22: Bits = Bits::new(s_25_21 as u128, 1u16);
        // D s_25_23: cmp-eq s_25_20 s_25_22
        let s_25_23: bool = ((s_25_20) == (s_25_22));
        // N s_25_24: branch s_25_23 b34 b26
        if s_25_23 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #22712u : u32
        let s_26_0: u32 = 22712;
        // D s_26_1: read-reg s_26_0:struct
        let s_26_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call _get_MDCR_EL3_Type_NSPB(s_26_1)
        let s_26_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_26_1);
        // C s_26_3: const #1s : i
        let s_26_3: i128 = 1;
        // D s_26_4: cast zx s_26_2 -> bv
        let s_26_4: Bits = Bits::new(s_26_2 as u128, 2u16);
        // C s_26_5: const #1u : u64
        let s_26_5: u64 = 1;
        // D s_26_6: bit-extract s_26_4 s_26_3 s_26_5
        let s_26_6: Bits = (Bits::new(
            ((s_26_4) >> (s_26_3)).value(),
            u16::try_from(s_26_5).unwrap(),
        ));
        // D s_26_7: cast reint s_26_6 -> u8
        let s_26_7: bool = ((s_26_6.value()) != 0);
        // C s_26_8: const #0s : i
        let s_26_8: i128 = 0;
        // C s_26_9: const #0u : u64
        let s_26_9: u64 = 0;
        // D s_26_10: cast zx s_26_7 -> u64
        let s_26_10: u64 = (s_26_7 as u64);
        // C s_26_11: const #1u : u64
        let s_26_11: u64 = 1;
        // D s_26_12: and s_26_10 s_26_11
        let s_26_12: u64 = ((s_26_10) & (s_26_11));
        // D s_26_13: cmp-eq s_26_12 s_26_11
        let s_26_13: bool = ((s_26_12) == (s_26_11));
        // D s_26_14: lsl s_26_10 s_26_8
        let s_26_14: u64 = s_26_10 << s_26_8;
        // D s_26_15: or s_26_9 s_26_14
        let s_26_15: u64 = ((s_26_9) | (s_26_14));
        // D s_26_16: cmpl s_26_14
        let s_26_16: u64 = !s_26_14;
        // D s_26_17: and s_26_9 s_26_16
        let s_26_17: u64 = ((s_26_9) & (s_26_16));
        // D s_26_18: select s_26_13 s_26_15 s_26_17
        let s_26_18: u64 = if s_26_13 { s_26_15 } else { s_26_17 };
        // D s_26_19: cast trunc s_26_18 -> u8
        let s_26_19: bool = ((s_26_18) != 0);
        // C s_26_20: const #90704u : u32
        let s_26_20: u32 = 90704;
        // D s_26_21: read-reg s_26_20:struct
        let s_26_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_26_20 as isize);
            tracer.read_register(s_26_20 as isize, value);
            value
        };
        // D s_26_22: call _get_SCR_EL3_Type_NS(s_26_21)
        let s_26_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_26_21);
        // D s_26_23: cast zx s_26_19 -> bv
        let s_26_23: Bits = Bits::new(s_26_19 as u128, 1u16);
        // D s_26_24: cast zx s_26_22 -> bv
        let s_26_24: Bits = Bits::new(s_26_22 as u128, 1u16);
        // D s_26_25: cmp-ne s_26_23 s_26_24
        let s_26_25: bool = ((s_26_23) != (s_26_24));
        // D s_26_26: write-var gs#63823 <= s_26_25
        fn_state.gs_63823 = s_26_25;
        // N s_26_27: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#63823:u8
        let s_27_0: bool = fn_state.gs_63823;
        // N s_27_1: branch s_27_0 b33 b28
        if s_27_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #232u : u32
        let s_28_0: u32 = 232;
        // S s_28_1: call IsFeatureImplemented(s_28_0)
        let s_28_1: bool = IsFeatureImplemented(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b32 b29
        if s_28_1 {
            return block_32(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#63824 <= s_29_0
        fn_state.gs_63824 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#63824:u8
        let s_30_0: bool = fn_state.gs_63824;
        // D s_30_1: write-var gs#63825 <= s_30_0
        fn_state.gs_63825 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#63825:u8
        let s_31_0: bool = fn_state.gs_63825;
        // D s_31_1: write-var gs#63826 <= s_31_0
        fn_state.gs_63826 = s_31_0;
        // N s_31_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __MDCR_EL3_NSPBE:u8
        let s_32_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // D s_32_2: read-var __SCR_EL3_NSE:u8
        let s_32_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-ne s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) != (s_32_3));
        // D s_32_5: write-var gs#63824 <= s_32_4
        fn_state.gs_63824 = s_32_4;
        // N s_32_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#63825 <= s_33_0
        fn_state.gs_63825 = s_33_0;
        // N s_33_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#63823 <= s_34_0
        fn_state.gs_63823 = s_34_0;
        // N s_34_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: panic
        panic!("{:?}", ());
        // N s_35_1: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #22712u : u32
        let s_36_0: u32 = 22712;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call _get_MDCR_EL3_Type_NSPB(s_36_1)
        let s_36_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_36_1);
        // C s_36_3: const #0s : i
        let s_36_3: i128 = 0;
        // D s_36_4: cast zx s_36_2 -> bv
        let s_36_4: Bits = Bits::new(s_36_2 as u128, 2u16);
        // C s_36_5: const #1u : u64
        let s_36_5: u64 = 1;
        // D s_36_6: bit-extract s_36_4 s_36_3 s_36_5
        let s_36_6: Bits = (Bits::new(
            ((s_36_4) >> (s_36_3)).value(),
            u16::try_from(s_36_5).unwrap(),
        ));
        // D s_36_7: cast reint s_36_6 -> u8
        let s_36_7: bool = ((s_36_6.value()) != 0);
        // C s_36_8: const #0s : i
        let s_36_8: i128 = 0;
        // C s_36_9: const #0u : u64
        let s_36_9: u64 = 0;
        // D s_36_10: cast zx s_36_7 -> u64
        let s_36_10: u64 = (s_36_7 as u64);
        // C s_36_11: const #1u : u64
        let s_36_11: u64 = 1;
        // D s_36_12: and s_36_10 s_36_11
        let s_36_12: u64 = ((s_36_10) & (s_36_11));
        // D s_36_13: cmp-eq s_36_12 s_36_11
        let s_36_13: bool = ((s_36_12) == (s_36_11));
        // D s_36_14: lsl s_36_10 s_36_8
        let s_36_14: u64 = s_36_10 << s_36_8;
        // D s_36_15: or s_36_9 s_36_14
        let s_36_15: u64 = ((s_36_9) | (s_36_14));
        // D s_36_16: cmpl s_36_14
        let s_36_16: u64 = !s_36_14;
        // D s_36_17: and s_36_9 s_36_16
        let s_36_17: u64 = ((s_36_9) & (s_36_16));
        // D s_36_18: select s_36_13 s_36_15 s_36_17
        let s_36_18: u64 = if s_36_13 { s_36_15 } else { s_36_17 };
        // D s_36_19: cast trunc s_36_18 -> u8
        let s_36_19: bool = ((s_36_18) != 0);
        // D s_36_20: cast zx s_36_19 -> bv
        let s_36_20: Bits = Bits::new(s_36_19 as u128, 1u16);
        // C s_36_21: const #0u : u8
        let s_36_21: bool = false;
        // C s_36_22: cast zx s_36_21 -> bv
        let s_36_22: Bits = Bits::new(s_36_21 as u128, 1u16);
        // D s_36_23: cmp-eq s_36_20 s_36_22
        let s_36_23: bool = ((s_36_20) == (s_36_22));
        // N s_36_24: branch s_36_23 b45 b37
        if s_36_23 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #22712u : u32
        let s_37_0: u32 = 22712;
        // D s_37_1: read-reg s_37_0:struct
        let s_37_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call _get_MDCR_EL3_Type_NSPB(s_37_1)
        let s_37_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_37_1);
        // C s_37_3: const #1s : i
        let s_37_3: i128 = 1;
        // D s_37_4: cast zx s_37_2 -> bv
        let s_37_4: Bits = Bits::new(s_37_2 as u128, 2u16);
        // C s_37_5: const #1u : u64
        let s_37_5: u64 = 1;
        // D s_37_6: bit-extract s_37_4 s_37_3 s_37_5
        let s_37_6: Bits = (Bits::new(
            ((s_37_4) >> (s_37_3)).value(),
            u16::try_from(s_37_5).unwrap(),
        ));
        // D s_37_7: cast reint s_37_6 -> u8
        let s_37_7: bool = ((s_37_6.value()) != 0);
        // C s_37_8: const #0s : i
        let s_37_8: i128 = 0;
        // C s_37_9: const #0u : u64
        let s_37_9: u64 = 0;
        // D s_37_10: cast zx s_37_7 -> u64
        let s_37_10: u64 = (s_37_7 as u64);
        // C s_37_11: const #1u : u64
        let s_37_11: u64 = 1;
        // D s_37_12: and s_37_10 s_37_11
        let s_37_12: u64 = ((s_37_10) & (s_37_11));
        // D s_37_13: cmp-eq s_37_12 s_37_11
        let s_37_13: bool = ((s_37_12) == (s_37_11));
        // D s_37_14: lsl s_37_10 s_37_8
        let s_37_14: u64 = s_37_10 << s_37_8;
        // D s_37_15: or s_37_9 s_37_14
        let s_37_15: u64 = ((s_37_9) | (s_37_14));
        // D s_37_16: cmpl s_37_14
        let s_37_16: u64 = !s_37_14;
        // D s_37_17: and s_37_9 s_37_16
        let s_37_17: u64 = ((s_37_9) & (s_37_16));
        // D s_37_18: select s_37_13 s_37_15 s_37_17
        let s_37_18: u64 = if s_37_13 { s_37_15 } else { s_37_17 };
        // D s_37_19: cast trunc s_37_18 -> u8
        let s_37_19: bool = ((s_37_18) != 0);
        // C s_37_20: const #90704u : u32
        let s_37_20: u32 = 90704;
        // D s_37_21: read-reg s_37_20:struct
        let s_37_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_37_20 as isize);
            tracer.read_register(s_37_20 as isize, value);
            value
        };
        // D s_37_22: call _get_SCR_EL3_Type_NS(s_37_21)
        let s_37_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_37_21);
        // D s_37_23: cast zx s_37_19 -> bv
        let s_37_23: Bits = Bits::new(s_37_19 as u128, 1u16);
        // D s_37_24: cast zx s_37_22 -> bv
        let s_37_24: Bits = Bits::new(s_37_22 as u128, 1u16);
        // D s_37_25: cmp-ne s_37_23 s_37_24
        let s_37_25: bool = ((s_37_23) != (s_37_24));
        // D s_37_26: write-var gs#63815 <= s_37_25
        fn_state.gs_63815 = s_37_25;
        // N s_37_27: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#63815:u8
        let s_38_0: bool = fn_state.gs_63815;
        // N s_38_1: branch s_38_0 b44 b39
        if s_38_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #232u : u32
        let s_39_0: u32 = 232;
        // S s_39_1: call IsFeatureImplemented(s_39_0)
        let s_39_1: bool = IsFeatureImplemented(state, tracer, s_39_0);
        // N s_39_2: branch s_39_1 b43 b40
        if s_39_1 {
            return block_43(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#63816 <= s_40_0
        fn_state.gs_63816 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#63816:u8
        let s_41_0: bool = fn_state.gs_63816;
        // D s_41_1: write-var gs#63817 <= s_41_0
        fn_state.gs_63817 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#63817:u8
        let s_42_0: bool = fn_state.gs_63817;
        // D s_42_1: write-var gs#63818 <= s_42_0
        fn_state.gs_63818 = s_42_0;
        // N s_42_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var __MDCR_EL3_NSPBE:u8
        let s_43_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 1u16);
        // D s_43_2: read-var __SCR_EL3_NSE:u8
        let s_43_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_4: cmp-ne s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) != (s_43_3));
        // D s_43_5: write-var gs#63816 <= s_43_4
        fn_state.gs_63816 = s_43_4;
        // N s_43_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#63817 <= s_44_0
        fn_state.gs_63817 = s_44_0;
        // N s_44_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#63815 <= s_45_0
        fn_state.gs_63815 = s_45_0;
        // N s_45_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_46_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_46_1: call __IMPDEF_boolean(s_46_0)
        let s_46_1: bool = u__IMPDEF_boolean(state, tracer, s_46_0);
        // D s_46_2: write-var gs#63810 <= s_46_1
        fn_state.gs_63810 = s_46_1;
        // N s_46_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EDSCR_read(s_47_0)
        let s_47_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_47_0);
        // S s_47_2: call _get_EDSCR_Type_SDD(s_47_1)
        let s_47_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_47_1);
        // S s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // C s_47_4: const #1u : u8
        let s_47_4: bool = true;
        // C s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 1u16);
        // S s_47_6: cmp-eq s_47_3 s_47_5
        let s_47_6: bool = ((s_47_3) == (s_47_5));
        // D s_47_7: write-var gs#63809 <= s_47_6
        fn_state.gs_63809 = s_47_6;
        // N s_47_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #424u : u32
        let s_48_0: u32 = 424;
        // D s_48_1: read-reg s_48_0:u8
        let s_48_1: u8 = {
            let value = state.read_register::<u8>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // C s_48_2: const #2u : u8
        let s_48_2: u8 = 2;
        // D s_48_3: cmp-lt s_48_1 s_48_2
        let s_48_3: bool = ((s_48_1) < (s_48_2));
        // D s_48_4: write-var gs#63808 <= s_48_3
        fn_state.gs_63808 = s_48_3;
        // N s_48_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call Halted(s_49_0)
        let s_49_1: bool = Halted(state, tracer, s_49_0);
        // N s_49_2: branch s_49_1 b119 b50
        if s_49_1 {
            return block_119(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#63829 <= s_50_0
        fn_state.gs_63829 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#63829:u8
        let s_51_0: bool = fn_state.gs_63829;
        // N s_51_1: branch s_51_0 b118 b52
        if s_51_0 {
            return block_118(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#63830 <= s_52_0
        fn_state.gs_63830 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#63830:u8
        let s_53_0: bool = fn_state.gs_63830;
        // N s_53_1: branch s_53_0 b117 b54
        if s_53_0 {
            return block_117(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#63831 <= s_54_0
        fn_state.gs_63831 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#63831:u8
        let s_55_0: bool = fn_state.gs_63831;
        // N s_55_1: branch s_55_0 b107 b56
        if s_55_0 {
            return block_107(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#63839 <= s_56_0
        fn_state.gs_63839 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#63839:u8
        let s_57_0: bool = fn_state.gs_63839;
        // N s_57_1: branch s_57_0 b106 b58
        if s_57_0 {
            return block_106(state, tracer, fn_state);
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
        // N s_58_2: branch s_58_1 b105 b59
        if s_58_1 {
            return block_105(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#63840 <= s_59_0
        fn_state.gs_63840 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#63840:u8
        let s_60_0: bool = fn_state.gs_63840;
        // N s_60_1: branch s_60_0 b101 b61
        if s_60_0 {
            return block_101(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#63842 <= s_61_0
        fn_state.gs_63842 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#63842:u8
        let s_62_0: bool = fn_state.gs_63842;
        // N s_62_1: branch s_62_0 b100 b63
        if s_62_0 {
            return block_100(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#63843 <= s_63_0
        fn_state.gs_63843 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#63843:u8
        let s_64_0: bool = fn_state.gs_63843;
        // N s_64_1: branch s_64_0 b99 b65
        if s_64_0 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call EL2Enabled(s_65_0)
        let s_65_1: bool = EL2Enabled(state, tracer, s_65_0);
        // N s_65_2: branch s_65_1 b95 b66
        if s_65_1 {
            return block_95(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#63849 <= s_66_0
        fn_state.gs_63849 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#63849:u8
        let s_67_0: bool = fn_state.gs_63849;
        // N s_67_1: branch s_67_0 b94 b68
        if s_67_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
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
        // N s_68_4: branch s_68_3 b84 b69
        if s_68_3 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#63857 <= s_69_0
        fn_state.gs_63857 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#63857:u8
        let s_70_0: bool = fn_state.gs_63857;
        // N s_70_1: branch s_70_0 b78 b71
        if s_70_0 {
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
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call EL2Enabled(s_71_0)
        let s_71_1: bool = EL2Enabled(state, tracer, s_71_0);
        // N s_71_2: branch s_71_1 b77 b72
        if s_71_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#63858 <= s_72_0
        fn_state.gs_63858 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#63858:u8
        let s_73_0: bool = fn_state.gs_63858;
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
        // C s_74_1: const #90512u : u32
        let s_74_1: u32 = 90512;
        // D s_74_2: read-reg s_74_1:u64
        let s_74_2: u64 = {
            let value = state.read_register::<u64>(s_74_1 as isize);
            tracer.read_register(s_74_1 as isize, value);
            value
        };
        // D s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 64u16);
        // D s_74_4: read-var t:i
        let s_74_4: i128 = fn_state.t;
        // D s_74_5: call X_set(s_74_4, s_74_0, s_74_3)
        let s_74_5: () = X_set(state, tracer, s_74_4, s_74_0, s_74_3);
        // N s_74_6: return
        return;
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #2064u : u12
        let s_75_0: u16 = 2064;
        // C s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 12u16);
        // C s_75_2: cast zx s_75_1 -> i
        let s_75_2: i128 = (s_75_1.value() as i128);
        // C s_75_3: cast reint s_75_2 -> i64
        let s_75_3: i64 = (s_75_2 as i64);
        // C s_75_4: cast zx s_75_3 -> i
        let s_75_4: i128 = (i128::try_from(s_75_3).unwrap());
        // S s_75_5: call NVMem_read(s_75_4)
        let s_75_5: u64 = NVMem_read(state, tracer, s_75_4);
        // D s_75_6: write-var ga#72638 <= s_75_5
        fn_state.ga_72638 = s_75_5;
        // N s_75_7: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var ga#72638:u64
        let s_76_0: u64 = fn_state.ga_72638;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 64u16);
        // D s_76_2: read-var t:i
        let s_76_2: i128 = fn_state.t;
        // C s_76_3: const #64s : i64
        let s_76_3: i64 = 64;
        // D s_76_4: call X_set(s_76_2, s_76_3, s_76_1)
        let s_76_4: () = X_set(state, tracer, s_76_2, s_76_3, s_76_1);
        // N s_76_5: return
        return;
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
        // D s_77_2: call _get_HCR_EL2_Type_NV2(s_77_1)
        let s_77_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_77_1);
        // C s_77_3: const #102552u : u32
        let s_77_3: u32 = 102552;
        // D s_77_4: read-reg s_77_3:struct
        let s_77_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_77_3 as isize);
            tracer.read_register(s_77_3 as isize, value);
            value
        };
        // D s_77_5: call _get_HCR_EL2_Type_NV(s_77_4)
        let s_77_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_77_4);
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
        // D s_77_21: write-var gs#63858 <= s_77_20
        fn_state.gs_63858 = s_77_20;
        // N s_77_22: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #() : ()
        let s_78_0: () = ();
        // S s_78_1: call Halted(s_78_0)
        let s_78_1: bool = Halted(state, tracer, s_78_0);
        // N s_78_2: branch s_78_1 b83 b79
        if s_78_1 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#63861 <= s_79_0
        fn_state.gs_63861 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#63861:u8
        let s_80_0: bool = fn_state.gs_63861;
        // N s_80_1: branch s_80_0 b82 b81
        if s_80_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #24u : u8
        let s_81_0: u8 = 24;
        // C s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 8u16);
        // C s_81_2: cast zx s_81_1 -> i
        let s_81_2: i128 = (s_81_1.value() as i128);
        // C s_81_3: cast reint s_81_2 -> i64
        let s_81_3: i64 = (s_81_2 as i64);
        // C s_81_4: cast zx s_81_3 -> i
        let s_81_4: i128 = (i128::try_from(s_81_3).unwrap());
        // C s_81_5: const #424u : u32
        let s_81_5: u32 = 424;
        // D s_81_6: read-reg s_81_5:u8
        let s_81_6: u8 = {
            let value = state.read_register::<u8>(s_81_5 as isize);
            tracer.read_register(s_81_5 as isize, value);
            value
        };
        // D s_81_7: call AArch64_SystemAccessTrap(s_81_6, s_81_4)
        let s_81_7: () = AArch64_SystemAccessTrap(state, tracer, s_81_6, s_81_4);
        // N s_81_8: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_82_0: panic
        panic!("{:?}", ());
        // N s_82_1: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call EDSCR_read(s_83_0)
        let s_83_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_83_0);
        // S s_83_2: call _get_EDSCR_Type_SDD(s_83_1)
        let s_83_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_83_1);
        // S s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 1u16);
        // C s_83_4: const #1u : u8
        let s_83_4: bool = true;
        // C s_83_5: cast zx s_83_4 -> bv
        let s_83_5: Bits = Bits::new(s_83_4 as u128, 1u16);
        // S s_83_6: cmp-eq s_83_3 s_83_5
        let s_83_6: bool = ((s_83_3) == (s_83_5));
        // D s_83_7: write-var gs#63861 <= s_83_6
        fn_state.gs_63861 = s_83_6;
        // N s_83_8: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #22712u : u32
        let s_84_0: u32 = 22712;
        // D s_84_1: read-reg s_84_0:struct
        let s_84_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call _get_MDCR_EL3_Type_NSPB(s_84_1)
        let s_84_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_84_1);
        // C s_84_3: const #0s : i
        let s_84_3: i128 = 0;
        // D s_84_4: cast zx s_84_2 -> bv
        let s_84_4: Bits = Bits::new(s_84_2 as u128, 2u16);
        // C s_84_5: const #1u : u64
        let s_84_5: u64 = 1;
        // D s_84_6: bit-extract s_84_4 s_84_3 s_84_5
        let s_84_6: Bits = (Bits::new(
            ((s_84_4) >> (s_84_3)).value(),
            u16::try_from(s_84_5).unwrap(),
        ));
        // D s_84_7: cast reint s_84_6 -> u8
        let s_84_7: bool = ((s_84_6.value()) != 0);
        // C s_84_8: const #0s : i
        let s_84_8: i128 = 0;
        // C s_84_9: const #0u : u64
        let s_84_9: u64 = 0;
        // D s_84_10: cast zx s_84_7 -> u64
        let s_84_10: u64 = (s_84_7 as u64);
        // C s_84_11: const #1u : u64
        let s_84_11: u64 = 1;
        // D s_84_12: and s_84_10 s_84_11
        let s_84_12: u64 = ((s_84_10) & (s_84_11));
        // D s_84_13: cmp-eq s_84_12 s_84_11
        let s_84_13: bool = ((s_84_12) == (s_84_11));
        // D s_84_14: lsl s_84_10 s_84_8
        let s_84_14: u64 = s_84_10 << s_84_8;
        // D s_84_15: or s_84_9 s_84_14
        let s_84_15: u64 = ((s_84_9) | (s_84_14));
        // D s_84_16: cmpl s_84_14
        let s_84_16: u64 = !s_84_14;
        // D s_84_17: and s_84_9 s_84_16
        let s_84_17: u64 = ((s_84_9) & (s_84_16));
        // D s_84_18: select s_84_13 s_84_15 s_84_17
        let s_84_18: u64 = if s_84_13 { s_84_15 } else { s_84_17 };
        // D s_84_19: cast trunc s_84_18 -> u8
        let s_84_19: bool = ((s_84_18) != 0);
        // D s_84_20: cast zx s_84_19 -> bv
        let s_84_20: Bits = Bits::new(s_84_19 as u128, 1u16);
        // C s_84_21: const #0u : u8
        let s_84_21: bool = false;
        // C s_84_22: cast zx s_84_21 -> bv
        let s_84_22: Bits = Bits::new(s_84_21 as u128, 1u16);
        // D s_84_23: cmp-eq s_84_20 s_84_22
        let s_84_23: bool = ((s_84_20) == (s_84_22));
        // N s_84_24: branch s_84_23 b93 b85
        if s_84_23 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #22712u : u32
        let s_85_0: u32 = 22712;
        // D s_85_1: read-reg s_85_0:struct
        let s_85_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_85_0 as isize);
            tracer.read_register(s_85_0 as isize, value);
            value
        };
        // D s_85_2: call _get_MDCR_EL3_Type_NSPB(s_85_1)
        let s_85_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_85_1);
        // C s_85_3: const #1s : i
        let s_85_3: i128 = 1;
        // D s_85_4: cast zx s_85_2 -> bv
        let s_85_4: Bits = Bits::new(s_85_2 as u128, 2u16);
        // C s_85_5: const #1u : u64
        let s_85_5: u64 = 1;
        // D s_85_6: bit-extract s_85_4 s_85_3 s_85_5
        let s_85_6: Bits = (Bits::new(
            ((s_85_4) >> (s_85_3)).value(),
            u16::try_from(s_85_5).unwrap(),
        ));
        // D s_85_7: cast reint s_85_6 -> u8
        let s_85_7: bool = ((s_85_6.value()) != 0);
        // C s_85_8: const #0s : i
        let s_85_8: i128 = 0;
        // C s_85_9: const #0u : u64
        let s_85_9: u64 = 0;
        // D s_85_10: cast zx s_85_7 -> u64
        let s_85_10: u64 = (s_85_7 as u64);
        // C s_85_11: const #1u : u64
        let s_85_11: u64 = 1;
        // D s_85_12: and s_85_10 s_85_11
        let s_85_12: u64 = ((s_85_10) & (s_85_11));
        // D s_85_13: cmp-eq s_85_12 s_85_11
        let s_85_13: bool = ((s_85_12) == (s_85_11));
        // D s_85_14: lsl s_85_10 s_85_8
        let s_85_14: u64 = s_85_10 << s_85_8;
        // D s_85_15: or s_85_9 s_85_14
        let s_85_15: u64 = ((s_85_9) | (s_85_14));
        // D s_85_16: cmpl s_85_14
        let s_85_16: u64 = !s_85_14;
        // D s_85_17: and s_85_9 s_85_16
        let s_85_17: u64 = ((s_85_9) & (s_85_16));
        // D s_85_18: select s_85_13 s_85_15 s_85_17
        let s_85_18: u64 = if s_85_13 { s_85_15 } else { s_85_17 };
        // D s_85_19: cast trunc s_85_18 -> u8
        let s_85_19: bool = ((s_85_18) != 0);
        // C s_85_20: const #90704u : u32
        let s_85_20: u32 = 90704;
        // D s_85_21: read-reg s_85_20:struct
        let s_85_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_85_20 as isize);
            tracer.read_register(s_85_20 as isize, value);
            value
        };
        // D s_85_22: call _get_SCR_EL3_Type_NS(s_85_21)
        let s_85_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_85_21);
        // D s_85_23: cast zx s_85_19 -> bv
        let s_85_23: Bits = Bits::new(s_85_19 as u128, 1u16);
        // D s_85_24: cast zx s_85_22 -> bv
        let s_85_24: Bits = Bits::new(s_85_22 as u128, 1u16);
        // D s_85_25: cmp-ne s_85_23 s_85_24
        let s_85_25: bool = ((s_85_23) != (s_85_24));
        // D s_85_26: write-var gs#63854 <= s_85_25
        fn_state.gs_63854 = s_85_25;
        // N s_85_27: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#63854:u8
        let s_86_0: bool = fn_state.gs_63854;
        // N s_86_1: branch s_86_0 b92 b87
        if s_86_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #232u : u32
        let s_87_0: u32 = 232;
        // S s_87_1: call IsFeatureImplemented(s_87_0)
        let s_87_1: bool = IsFeatureImplemented(state, tracer, s_87_0);
        // N s_87_2: branch s_87_1 b91 b88
        if s_87_1 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#63855 <= s_88_0
        fn_state.gs_63855 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#63855:u8
        let s_89_0: bool = fn_state.gs_63855;
        // D s_89_1: write-var gs#63856 <= s_89_0
        fn_state.gs_63856 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#63856:u8
        let s_90_0: bool = fn_state.gs_63856;
        // D s_90_1: write-var gs#63857 <= s_90_0
        fn_state.gs_63857 = s_90_0;
        // N s_90_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var __MDCR_EL3_NSPBE:u8
        let s_91_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 1u16);
        // D s_91_2: read-var __SCR_EL3_NSE:u8
        let s_91_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // D s_91_4: cmp-ne s_91_1 s_91_3
        let s_91_4: bool = ((s_91_1) != (s_91_3));
        // D s_91_5: write-var gs#63855 <= s_91_4
        fn_state.gs_63855 = s_91_4;
        // N s_91_6: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #1u : u8
        let s_92_0: bool = true;
        // D s_92_1: write-var gs#63856 <= s_92_0
        fn_state.gs_63856 = s_92_0;
        // N s_92_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #1u : u8
        let s_93_0: bool = true;
        // D s_93_1: write-var gs#63854 <= s_93_0
        fn_state.gs_63854 = s_93_0;
        // N s_93_2: jump b86
        return block_86(state, tracer, fn_state);
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
        // C s_94_5: const #432u : u32
        let s_94_5: u32 = 432;
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
        // D s_95_0: read-var __MDCR_EL2_E2PB:u8
        let s_95_0: u8 = fn_state.u__MDCR_EL2_E2PB;
        // C s_95_1: const #0s : i
        let s_95_1: i128 = 0;
        // D s_95_2: cast zx s_95_0 -> bv
        let s_95_2: Bits = Bits::new(s_95_0 as u128, 2u16);
        // C s_95_3: const #1s : i64
        let s_95_3: i64 = 1;
        // C s_95_4: cast zx s_95_3 -> i
        let s_95_4: i128 = (i128::try_from(s_95_3).unwrap());
        // C s_95_5: const #0s : i
        let s_95_5: i128 = 0;
        // C s_95_6: add s_95_5 s_95_4
        let s_95_6: i128 = (s_95_5 + s_95_4);
        // D s_95_7: bit-extract s_95_2 s_95_1 s_95_6
        let s_95_7: Bits = (Bits::new(
            ((s_95_2) >> (s_95_1)).value(),
            u16::try_from(s_95_6).unwrap(),
        ));
        // D s_95_8: cast reint s_95_7 -> u8
        let s_95_8: bool = ((s_95_7.value()) != 0);
        // D s_95_9: cast zx s_95_8 -> bv
        let s_95_9: Bits = Bits::new(s_95_8 as u128, 1u16);
        // C s_95_10: const #0u : u8
        let s_95_10: bool = false;
        // C s_95_11: cast zx s_95_10 -> bv
        let s_95_11: Bits = Bits::new(s_95_10 as u128, 1u16);
        // D s_95_12: cmp-eq s_95_9 s_95_11
        let s_95_12: bool = ((s_95_9) == (s_95_11));
        // D s_95_13: not s_95_12
        let s_95_13: bool = !s_95_12;
        // N s_95_14: branch s_95_13 b98 b96
        if s_95_13 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #1u : u8
        let s_96_0: bool = true;
        // D s_96_1: write-var gs#63844 <= s_96_0
        fn_state.gs_63844 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#63844:u8
        let s_97_0: bool = fn_state.gs_63844;
        // D s_97_1: write-var gs#63849 <= s_97_0
        fn_state.gs_63849 = s_97_0;
        // N s_97_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#63844 <= s_98_0
        fn_state.gs_63844 = s_98_0;
        // N s_98_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #24u : u8
        let s_99_0: u8 = 24;
        // C s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 8u16);
        // C s_99_2: cast zx s_99_1 -> i
        let s_99_2: i128 = (s_99_1.value() as i128);
        // C s_99_3: cast reint s_99_2 -> i64
        let s_99_3: i64 = (s_99_2 as i64);
        // C s_99_4: cast zx s_99_3 -> i
        let s_99_4: i128 = (i128::try_from(s_99_3).unwrap());
        // C s_99_5: const #432u : u32
        let s_99_5: u32 = 432;
        // D s_99_6: read-reg s_99_5:u8
        let s_99_6: u8 = {
            let value = state.read_register::<u8>(s_99_5 as isize);
            tracer.read_register(s_99_5 as isize, value);
            value
        };
        // D s_99_7: call AArch64_SystemAccessTrap(s_99_6, s_99_4)
        let s_99_7: () = AArch64_SystemAccessTrap(state, tracer, s_99_6, s_99_4);
        // N s_99_8: return
        return;
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var __HDFGRTR_EL2_PMBPTR_EL1:u8
        let s_100_0: bool = fn_state.u__HDFGRTR_EL2_PMBPTR_EL1;
        // D s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 1u16);
        // C s_100_2: const #1u : u8
        let s_100_2: bool = true;
        // C s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 1u16);
        // D s_100_4: cmp-eq s_100_1 s_100_3
        let s_100_4: bool = ((s_100_1) == (s_100_3));
        // D s_100_5: write-var gs#63843 <= s_100_4
        fn_state.gs_63843 = s_100_4;
        // N s_100_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #424u : u32
        let s_101_0: u32 = 424;
        // D s_101_1: read-reg s_101_0:u8
        let s_101_1: u8 = {
            let value = state.read_register::<u8>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // C s_101_2: const #2u : u8
        let s_101_2: u8 = 2;
        // D s_101_3: cmp-lt s_101_1 s_101_2
        let s_101_3: bool = ((s_101_1) < (s_101_2));
        // D s_101_4: not s_101_3
        let s_101_4: bool = !s_101_3;
        // N s_101_5: branch s_101_4 b104 b102
        if s_101_4 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var __SCR_EL3_FGTEn:u8
        let s_102_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 1u16);
        // C s_102_2: const #1u : u8
        let s_102_2: bool = true;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: write-var gs#63841 <= s_102_4
        fn_state.gs_63841 = s_102_4;
        // N s_102_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#63841:u8
        let s_103_0: bool = fn_state.gs_63841;
        // D s_103_1: write-var gs#63842 <= s_103_0
        fn_state.gs_63842 = s_103_0;
        // N s_103_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #1u : u8
        let s_104_0: bool = true;
        // D s_104_1: write-var gs#63841 <= s_104_0
        fn_state.gs_63841 = s_104_0;
        // N s_104_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #146u : u32
        let s_105_0: u32 = 146;
        // S s_105_1: call IsFeatureImplemented(s_105_0)
        let s_105_1: bool = IsFeatureImplemented(state, tracer, s_105_0);
        // D s_105_2: write-var gs#63840 <= s_105_1
        fn_state.gs_63840 = s_105_1;
        // N s_105_3: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_106_0: panic
        panic!("{:?}", ());
        // N s_106_1: return
        return;
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #22712u : u32
        let s_107_0: u32 = 22712;
        // D s_107_1: read-reg s_107_0:struct
        let s_107_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: call _get_MDCR_EL3_Type_NSPB(s_107_1)
        let s_107_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_107_1);
        // C s_107_3: const #0s : i
        let s_107_3: i128 = 0;
        // D s_107_4: cast zx s_107_2 -> bv
        let s_107_4: Bits = Bits::new(s_107_2 as u128, 2u16);
        // C s_107_5: const #1u : u64
        let s_107_5: u64 = 1;
        // D s_107_6: bit-extract s_107_4 s_107_3 s_107_5
        let s_107_6: Bits = (Bits::new(
            ((s_107_4) >> (s_107_3)).value(),
            u16::try_from(s_107_5).unwrap(),
        ));
        // D s_107_7: cast reint s_107_6 -> u8
        let s_107_7: bool = ((s_107_6.value()) != 0);
        // C s_107_8: const #0s : i
        let s_107_8: i128 = 0;
        // C s_107_9: const #0u : u64
        let s_107_9: u64 = 0;
        // D s_107_10: cast zx s_107_7 -> u64
        let s_107_10: u64 = (s_107_7 as u64);
        // C s_107_11: const #1u : u64
        let s_107_11: u64 = 1;
        // D s_107_12: and s_107_10 s_107_11
        let s_107_12: u64 = ((s_107_10) & (s_107_11));
        // D s_107_13: cmp-eq s_107_12 s_107_11
        let s_107_13: bool = ((s_107_12) == (s_107_11));
        // D s_107_14: lsl s_107_10 s_107_8
        let s_107_14: u64 = s_107_10 << s_107_8;
        // D s_107_15: or s_107_9 s_107_14
        let s_107_15: u64 = ((s_107_9) | (s_107_14));
        // D s_107_16: cmpl s_107_14
        let s_107_16: u64 = !s_107_14;
        // D s_107_17: and s_107_9 s_107_16
        let s_107_17: u64 = ((s_107_9) & (s_107_16));
        // D s_107_18: select s_107_13 s_107_15 s_107_17
        let s_107_18: u64 = if s_107_13 { s_107_15 } else { s_107_17 };
        // D s_107_19: cast trunc s_107_18 -> u8
        let s_107_19: bool = ((s_107_18) != 0);
        // D s_107_20: cast zx s_107_19 -> bv
        let s_107_20: Bits = Bits::new(s_107_19 as u128, 1u16);
        // C s_107_21: const #0u : u8
        let s_107_21: bool = false;
        // C s_107_22: cast zx s_107_21 -> bv
        let s_107_22: Bits = Bits::new(s_107_21 as u128, 1u16);
        // D s_107_23: cmp-eq s_107_20 s_107_22
        let s_107_23: bool = ((s_107_20) == (s_107_22));
        // N s_107_24: branch s_107_23 b116 b108
        if s_107_23 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #22712u : u32
        let s_108_0: u32 = 22712;
        // D s_108_1: read-reg s_108_0:struct
        let s_108_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_108_0 as isize);
            tracer.read_register(s_108_0 as isize, value);
            value
        };
        // D s_108_2: call _get_MDCR_EL3_Type_NSPB(s_108_1)
        let s_108_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_108_1);
        // C s_108_3: const #1s : i
        let s_108_3: i128 = 1;
        // D s_108_4: cast zx s_108_2 -> bv
        let s_108_4: Bits = Bits::new(s_108_2 as u128, 2u16);
        // C s_108_5: const #1u : u64
        let s_108_5: u64 = 1;
        // D s_108_6: bit-extract s_108_4 s_108_3 s_108_5
        let s_108_6: Bits = (Bits::new(
            ((s_108_4) >> (s_108_3)).value(),
            u16::try_from(s_108_5).unwrap(),
        ));
        // D s_108_7: cast reint s_108_6 -> u8
        let s_108_7: bool = ((s_108_6.value()) != 0);
        // C s_108_8: const #0s : i
        let s_108_8: i128 = 0;
        // C s_108_9: const #0u : u64
        let s_108_9: u64 = 0;
        // D s_108_10: cast zx s_108_7 -> u64
        let s_108_10: u64 = (s_108_7 as u64);
        // C s_108_11: const #1u : u64
        let s_108_11: u64 = 1;
        // D s_108_12: and s_108_10 s_108_11
        let s_108_12: u64 = ((s_108_10) & (s_108_11));
        // D s_108_13: cmp-eq s_108_12 s_108_11
        let s_108_13: bool = ((s_108_12) == (s_108_11));
        // D s_108_14: lsl s_108_10 s_108_8
        let s_108_14: u64 = s_108_10 << s_108_8;
        // D s_108_15: or s_108_9 s_108_14
        let s_108_15: u64 = ((s_108_9) | (s_108_14));
        // D s_108_16: cmpl s_108_14
        let s_108_16: u64 = !s_108_14;
        // D s_108_17: and s_108_9 s_108_16
        let s_108_17: u64 = ((s_108_9) & (s_108_16));
        // D s_108_18: select s_108_13 s_108_15 s_108_17
        let s_108_18: u64 = if s_108_13 { s_108_15 } else { s_108_17 };
        // D s_108_19: cast trunc s_108_18 -> u8
        let s_108_19: bool = ((s_108_18) != 0);
        // C s_108_20: const #90704u : u32
        let s_108_20: u32 = 90704;
        // D s_108_21: read-reg s_108_20:struct
        let s_108_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_108_20 as isize);
            tracer.read_register(s_108_20 as isize, value);
            value
        };
        // D s_108_22: call _get_SCR_EL3_Type_NS(s_108_21)
        let s_108_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_108_21);
        // D s_108_23: cast zx s_108_19 -> bv
        let s_108_23: Bits = Bits::new(s_108_19 as u128, 1u16);
        // D s_108_24: cast zx s_108_22 -> bv
        let s_108_24: Bits = Bits::new(s_108_22 as u128, 1u16);
        // D s_108_25: cmp-ne s_108_23 s_108_24
        let s_108_25: bool = ((s_108_23) != (s_108_24));
        // D s_108_26: write-var gs#63836 <= s_108_25
        fn_state.gs_63836 = s_108_25;
        // N s_108_27: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#63836:u8
        let s_109_0: bool = fn_state.gs_63836;
        // N s_109_1: branch s_109_0 b115 b110
        if s_109_0 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #232u : u32
        let s_110_0: u32 = 232;
        // S s_110_1: call IsFeatureImplemented(s_110_0)
        let s_110_1: bool = IsFeatureImplemented(state, tracer, s_110_0);
        // N s_110_2: branch s_110_1 b114 b111
        if s_110_1 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #0u : u8
        let s_111_0: bool = false;
        // D s_111_1: write-var gs#63837 <= s_111_0
        fn_state.gs_63837 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#63837:u8
        let s_112_0: bool = fn_state.gs_63837;
        // D s_112_1: write-var gs#63838 <= s_112_0
        fn_state.gs_63838 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#63838:u8
        let s_113_0: bool = fn_state.gs_63838;
        // D s_113_1: write-var gs#63839 <= s_113_0
        fn_state.gs_63839 = s_113_0;
        // N s_113_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var __MDCR_EL3_NSPBE:u8
        let s_114_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_114_1: cast zx s_114_0 -> bv
        let s_114_1: Bits = Bits::new(s_114_0 as u128, 1u16);
        // D s_114_2: read-var __SCR_EL3_NSE:u8
        let s_114_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_114_3: cast zx s_114_2 -> bv
        let s_114_3: Bits = Bits::new(s_114_2 as u128, 1u16);
        // D s_114_4: cmp-ne s_114_1 s_114_3
        let s_114_4: bool = ((s_114_1) != (s_114_3));
        // D s_114_5: write-var gs#63837 <= s_114_4
        fn_state.gs_63837 = s_114_4;
        // N s_114_6: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #1u : u8
        let s_115_0: bool = true;
        // D s_115_1: write-var gs#63838 <= s_115_0
        fn_state.gs_63838 = s_115_0;
        // N s_115_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #1u : u8
        let s_116_0: bool = true;
        // D s_116_1: write-var gs#63836 <= s_116_0
        fn_state.gs_63836 = s_116_0;
        // N s_116_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_117_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_117_1: call __IMPDEF_boolean(s_117_0)
        let s_117_1: bool = u__IMPDEF_boolean(state, tracer, s_117_0);
        // D s_117_2: write-var gs#63831 <= s_117_1
        fn_state.gs_63831 = s_117_1;
        // N s_117_3: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call EDSCR_read(s_118_0)
        let s_118_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_118_0);
        // S s_118_2: call _get_EDSCR_Type_SDD(s_118_1)
        let s_118_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_118_1);
        // S s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // C s_118_4: const #1u : u8
        let s_118_4: bool = true;
        // C s_118_5: cast zx s_118_4 -> bv
        let s_118_5: Bits = Bits::new(s_118_4 as u128, 1u16);
        // S s_118_6: cmp-eq s_118_3 s_118_5
        let s_118_6: bool = ((s_118_3) == (s_118_5));
        // D s_118_7: write-var gs#63830 <= s_118_6
        fn_state.gs_63830 = s_118_6;
        // N s_118_8: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #424u : u32
        let s_119_0: u32 = 424;
        // D s_119_1: read-reg s_119_0:u8
        let s_119_1: u8 = {
            let value = state.read_register::<u8>(s_119_0 as isize);
            tracer.read_register(s_119_0 as isize, value);
            value
        };
        // C s_119_2: const #2u : u8
        let s_119_2: u8 = 2;
        // D s_119_3: cmp-lt s_119_1 s_119_2
        let s_119_3: bool = ((s_119_1) < (s_119_2));
        // D s_119_4: write-var gs#63829 <= s_119_3
        fn_state.gs_63829 = s_119_3;
        // N s_119_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_120_0: panic
        panic!("{:?}", ());
        // N s_120_1: return
        return;
    }
}