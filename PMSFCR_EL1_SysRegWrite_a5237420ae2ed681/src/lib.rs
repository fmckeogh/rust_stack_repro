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
use u_get_MDCR_EL3_Type_NSPB::*;
use IsFeatureImplemented::*;
use u_get_HDFGWTR_EL2_Type_PMSFCR_EL1::*;
use X_read::*;
use u_get_SCR_EL3_Type_NS::*;
use Mk_PMSFCR_EL1_Type::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_SCR_EL3_Type_NSE::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_MDCR_EL3_Type_NSPBE::*;
use u_get_MDCR_EL2_Type_TPMS::*;
use EL2Enabled::*;
use EDSCR_read::*;
use common::*;
pub fn PMSFCR_EL1_SysRegWrite_a5237420ae2ed681<T: Tracer>(
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
        gs_91052: bool,
        gs_91072: bool,
        gs_91076: bool,
        gs_91040: bool,
        gs_91037: bool,
        gs_91032: bool,
        gs_91063: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_91047: bool,
        gs_91066: bool,
        u__MDCR_EL3_NSPBE: bool,
        gs_91073: bool,
        gs_91045: bool,
        gs_91074: bool,
        gs_91059: bool,
        u__MDCR_EL2_TPMS: bool,
        gs_91064: bool,
        gs_91030: bool,
        gs_91071: bool,
        gs_91061: bool,
        gs_91058: bool,
        gs_91038: bool,
        gs_91031: bool,
        gs_91048: bool,
        gs_91039: bool,
        gs_91051: bool,
        gs_91060: bool,
        u__HDFGWTR_EL2_PMSFCR_EL1: bool,
        u__SCR_EL3_NSE: bool,
        u__PSTATE_EL: u8,
        gs_91046: bool,
        gs_91065: bool,
        gs_91062: bool,
        gs_91053: bool,
        gs_91050: bool,
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
        // C s_0_15: const #17360u : u32
        let s_0_15: u32 = 17360;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HDFGWTR_EL2_Type_PMSFCR_EL1(s_0_16)
        let s_0_17: bool = u_get_HDFGWTR_EL2_Type_PMSFCR_EL1(state, tracer, s_0_16);
        // D s_0_18: write-var __HDFGWTR_EL2_PMSFCR_EL1 <= s_0_17
        fn_state.u__HDFGWTR_EL2_PMSFCR_EL1 = s_0_17;
        // C s_0_19: const #104880u : u32
        let s_0_19: u32 = 104880;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_MDCR_EL2_Type_TPMS(s_0_20)
        let s_0_21: bool = u_get_MDCR_EL2_Type_TPMS(state, tracer, s_0_20);
        // D s_0_22: write-var __MDCR_EL2_TPMS <= s_0_21
        fn_state.u__MDCR_EL2_TPMS = s_0_21;
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
        // N s_0_29: branch s_0_28 b111 b1
        if s_0_28 {
            return block_111(state, tracer, fn_state);
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
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // D s_5_4: call Mk_PMSFCR_EL1_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_PMSFCR_EL1_Type(
            state,
            tracer,
            s_5_3,
        );
        // C s_5_5: const #101208u : u32
        let s_5_5: u32 = 101208;
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
        // D s_7_1: write-var gs#91030 <= s_7_0
        fn_state.gs_91030 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#91030:u8
        let s_8_0: bool = fn_state.gs_91030;
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
        // D s_9_1: write-var gs#91031 <= s_9_0
        fn_state.gs_91031 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#91031:u8
        let s_10_0: bool = fn_state.gs_91031;
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
        // D s_11_1: write-var gs#91032 <= s_11_0
        fn_state.gs_91032 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#91032:u8
        let s_12_0: bool = fn_state.gs_91032;
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
        // D s_13_1: write-var gs#91040 <= s_13_0
        fn_state.gs_91040 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#91040:u8
        let s_14_0: bool = fn_state.gs_91040;
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
        // D s_16_1: write-var gs#91048 <= s_16_0
        fn_state.gs_91048 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#91048:u8
        let s_17_0: bool = fn_state.gs_91048;
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
        // D s_18_1: read-var t:i
        let s_18_1: i128 = fn_state.t;
        // D s_18_2: call X_read(s_18_1, s_18_0)
        let s_18_2: Bits = X_read(state, tracer, s_18_1, s_18_0);
        // D s_18_3: cast reint s_18_2 -> u64
        let s_18_3: u64 = (s_18_2.value() as u64);
        // D s_18_4: call Mk_PMSFCR_EL1_Type(s_18_3)
        let s_18_4: ProductType5c790c8ef59cc8b2 = Mk_PMSFCR_EL1_Type(
            state,
            tracer,
            s_18_3,
        );
        // C s_18_5: const #101208u : u32
        let s_18_5: u32 = 101208;
        // N s_18_6: write-reg s_18_5 <= s_18_4
        let s_18_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_18_5 as isize, s_18_4);
            tracer.write_register(s_18_5 as isize, s_18_4);
        };
        // N s_18_7: return
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
        // D s_20_1: write-var gs#91050 <= s_20_0
        fn_state.gs_91050 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#91050:u8
        let s_21_0: bool = fn_state.gs_91050;
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
        // D s_24_7: write-var gs#91050 <= s_24_6
        fn_state.gs_91050 = s_24_6;
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
        // D s_26_26: write-var gs#91045 <= s_26_25
        fn_state.gs_91045 = s_26_25;
        // N s_26_27: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#91045:u8
        let s_27_0: bool = fn_state.gs_91045;
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
        // D s_29_1: write-var gs#91046 <= s_29_0
        fn_state.gs_91046 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#91046:u8
        let s_30_0: bool = fn_state.gs_91046;
        // D s_30_1: write-var gs#91047 <= s_30_0
        fn_state.gs_91047 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#91047:u8
        let s_31_0: bool = fn_state.gs_91047;
        // D s_31_1: write-var gs#91048 <= s_31_0
        fn_state.gs_91048 = s_31_0;
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
        // D s_32_5: write-var gs#91046 <= s_32_4
        fn_state.gs_91046 = s_32_4;
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
        // D s_33_1: write-var gs#91047 <= s_33_0
        fn_state.gs_91047 = s_33_0;
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
        // D s_34_1: write-var gs#91045 <= s_34_0
        fn_state.gs_91045 = s_34_0;
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
        // D s_37_26: write-var gs#91037 <= s_37_25
        fn_state.gs_91037 = s_37_25;
        // N s_37_27: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#91037:u8
        let s_38_0: bool = fn_state.gs_91037;
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
        // D s_40_1: write-var gs#91038 <= s_40_0
        fn_state.gs_91038 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#91038:u8
        let s_41_0: bool = fn_state.gs_91038;
        // D s_41_1: write-var gs#91039 <= s_41_0
        fn_state.gs_91039 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#91039:u8
        let s_42_0: bool = fn_state.gs_91039;
        // D s_42_1: write-var gs#91040 <= s_42_0
        fn_state.gs_91040 = s_42_0;
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
        // D s_43_5: write-var gs#91038 <= s_43_4
        fn_state.gs_91038 = s_43_4;
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
        // D s_44_1: write-var gs#91039 <= s_44_0
        fn_state.gs_91039 = s_44_0;
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
        // D s_45_1: write-var gs#91037 <= s_45_0
        fn_state.gs_91037 = s_45_0;
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
        // D s_46_2: write-var gs#91032 <= s_46_1
        fn_state.gs_91032 = s_46_1;
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
        // D s_47_7: write-var gs#91031 <= s_47_6
        fn_state.gs_91031 = s_47_6;
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
        // D s_48_4: write-var gs#91030 <= s_48_3
        fn_state.gs_91030 = s_48_3;
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
        // N s_49_2: branch s_49_1 b110 b50
        if s_49_1 {
            return block_110(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#91051 <= s_50_0
        fn_state.gs_91051 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#91051:u8
        let s_51_0: bool = fn_state.gs_91051;
        // N s_51_1: branch s_51_0 b109 b52
        if s_51_0 {
            return block_109(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#91052 <= s_52_0
        fn_state.gs_91052 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#91052:u8
        let s_53_0: bool = fn_state.gs_91052;
        // N s_53_1: branch s_53_0 b108 b54
        if s_53_0 {
            return block_108(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#91053 <= s_54_0
        fn_state.gs_91053 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#91053:u8
        let s_55_0: bool = fn_state.gs_91053;
        // N s_55_1: branch s_55_0 b98 b56
        if s_55_0 {
            return block_98(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#91061 <= s_56_0
        fn_state.gs_91061 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#91061:u8
        let s_57_0: bool = fn_state.gs_91061;
        // N s_57_1: branch s_57_0 b97 b58
        if s_57_0 {
            return block_97(state, tracer, fn_state);
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
        // N s_58_2: branch s_58_1 b96 b59
        if s_58_1 {
            return block_96(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#91062 <= s_59_0
        fn_state.gs_91062 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#91062:u8
        let s_60_0: bool = fn_state.gs_91062;
        // N s_60_1: branch s_60_0 b92 b61
        if s_60_0 {
            return block_92(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#91064 <= s_61_0
        fn_state.gs_91064 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#91064:u8
        let s_62_0: bool = fn_state.gs_91064;
        // N s_62_1: branch s_62_0 b91 b63
        if s_62_0 {
            return block_91(state, tracer, fn_state);
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
        // D s_63_1: write-var gs#91065 <= s_63_0
        fn_state.gs_91065 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#91065:u8
        let s_64_0: bool = fn_state.gs_91065;
        // N s_64_1: branch s_64_0 b90 b65
        if s_64_0 {
            return block_90(state, tracer, fn_state);
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
        // N s_65_2: branch s_65_1 b89 b66
        if s_65_1 {
            return block_89(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#91066 <= s_66_0
        fn_state.gs_91066 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#91066:u8
        let s_67_0: bool = fn_state.gs_91066;
        // N s_67_1: branch s_67_0 b88 b68
        if s_67_0 {
            return block_88(state, tracer, fn_state);
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
        // N s_68_4: branch s_68_3 b78 b69
        if s_68_3 {
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
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#91074 <= s_69_0
        fn_state.gs_91074 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#91074:u8
        let s_70_0: bool = fn_state.gs_91074;
        // N s_70_1: branch s_70_0 b72 b71
        if s_70_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #64s : i64
        let s_71_0: i64 = 64;
        // D s_71_1: read-var t:i
        let s_71_1: i128 = fn_state.t;
        // D s_71_2: call X_read(s_71_1, s_71_0)
        let s_71_2: Bits = X_read(state, tracer, s_71_1, s_71_0);
        // D s_71_3: cast reint s_71_2 -> u64
        let s_71_3: u64 = (s_71_2.value() as u64);
        // D s_71_4: call Mk_PMSFCR_EL1_Type(s_71_3)
        let s_71_4: ProductType5c790c8ef59cc8b2 = Mk_PMSFCR_EL1_Type(
            state,
            tracer,
            s_71_3,
        );
        // C s_71_5: const #101208u : u32
        let s_71_5: u32 = 101208;
        // N s_71_6: write-reg s_71_5 <= s_71_4
        let s_71_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_71_5 as isize, s_71_4);
            tracer.write_register(s_71_5 as isize, s_71_4);
        };
        // N s_71_7: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call Halted(s_72_0)
        let s_72_1: bool = Halted(state, tracer, s_72_0);
        // N s_72_2: branch s_72_1 b77 b73
        if s_72_1 {
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
        // D s_73_1: write-var gs#91076 <= s_73_0
        fn_state.gs_91076 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#91076:u8
        let s_74_0: bool = fn_state.gs_91076;
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
        // C s_75_0: const #24u : u8
        let s_75_0: u8 = 24;
        // C s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 8u16);
        // C s_75_2: cast zx s_75_1 -> i
        let s_75_2: i128 = (s_75_1.value() as i128);
        // C s_75_3: cast reint s_75_2 -> i64
        let s_75_3: i64 = (s_75_2 as i64);
        // C s_75_4: cast zx s_75_3 -> i
        let s_75_4: i128 = (i128::try_from(s_75_3).unwrap());
        // C s_75_5: const #424u : u32
        let s_75_5: u32 = 424;
        // D s_75_6: read-reg s_75_5:u8
        let s_75_6: u8 = {
            let value = state.read_register::<u8>(s_75_5 as isize);
            tracer.read_register(s_75_5 as isize, value);
            value
        };
        // D s_75_7: call AArch64_SystemAccessTrap(s_75_6, s_75_4)
        let s_75_7: () = AArch64_SystemAccessTrap(state, tracer, s_75_6, s_75_4);
        // N s_75_8: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_76_0: panic
        panic!("{:?}", ());
        // N s_76_1: return
        return;
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #() : ()
        let s_77_0: () = ();
        // S s_77_1: call EDSCR_read(s_77_0)
        let s_77_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_77_0);
        // S s_77_2: call _get_EDSCR_Type_SDD(s_77_1)
        let s_77_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_77_1);
        // S s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // C s_77_4: const #1u : u8
        let s_77_4: bool = true;
        // C s_77_5: cast zx s_77_4 -> bv
        let s_77_5: Bits = Bits::new(s_77_4 as u128, 1u16);
        // S s_77_6: cmp-eq s_77_3 s_77_5
        let s_77_6: bool = ((s_77_3) == (s_77_5));
        // D s_77_7: write-var gs#91076 <= s_77_6
        fn_state.gs_91076 = s_77_6;
        // N s_77_8: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #22712u : u32
        let s_78_0: u32 = 22712;
        // D s_78_1: read-reg s_78_0:struct
        let s_78_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // D s_78_2: call _get_MDCR_EL3_Type_NSPB(s_78_1)
        let s_78_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_78_1);
        // C s_78_3: const #0s : i
        let s_78_3: i128 = 0;
        // D s_78_4: cast zx s_78_2 -> bv
        let s_78_4: Bits = Bits::new(s_78_2 as u128, 2u16);
        // C s_78_5: const #1u : u64
        let s_78_5: u64 = 1;
        // D s_78_6: bit-extract s_78_4 s_78_3 s_78_5
        let s_78_6: Bits = (Bits::new(
            ((s_78_4) >> (s_78_3)).value(),
            u16::try_from(s_78_5).unwrap(),
        ));
        // D s_78_7: cast reint s_78_6 -> u8
        let s_78_7: bool = ((s_78_6.value()) != 0);
        // C s_78_8: const #0s : i
        let s_78_8: i128 = 0;
        // C s_78_9: const #0u : u64
        let s_78_9: u64 = 0;
        // D s_78_10: cast zx s_78_7 -> u64
        let s_78_10: u64 = (s_78_7 as u64);
        // C s_78_11: const #1u : u64
        let s_78_11: u64 = 1;
        // D s_78_12: and s_78_10 s_78_11
        let s_78_12: u64 = ((s_78_10) & (s_78_11));
        // D s_78_13: cmp-eq s_78_12 s_78_11
        let s_78_13: bool = ((s_78_12) == (s_78_11));
        // D s_78_14: lsl s_78_10 s_78_8
        let s_78_14: u64 = s_78_10 << s_78_8;
        // D s_78_15: or s_78_9 s_78_14
        let s_78_15: u64 = ((s_78_9) | (s_78_14));
        // D s_78_16: cmpl s_78_14
        let s_78_16: u64 = !s_78_14;
        // D s_78_17: and s_78_9 s_78_16
        let s_78_17: u64 = ((s_78_9) & (s_78_16));
        // D s_78_18: select s_78_13 s_78_15 s_78_17
        let s_78_18: u64 = if s_78_13 { s_78_15 } else { s_78_17 };
        // D s_78_19: cast trunc s_78_18 -> u8
        let s_78_19: bool = ((s_78_18) != 0);
        // D s_78_20: cast zx s_78_19 -> bv
        let s_78_20: Bits = Bits::new(s_78_19 as u128, 1u16);
        // C s_78_21: const #0u : u8
        let s_78_21: bool = false;
        // C s_78_22: cast zx s_78_21 -> bv
        let s_78_22: Bits = Bits::new(s_78_21 as u128, 1u16);
        // D s_78_23: cmp-eq s_78_20 s_78_22
        let s_78_23: bool = ((s_78_20) == (s_78_22));
        // N s_78_24: branch s_78_23 b87 b79
        if s_78_23 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #22712u : u32
        let s_79_0: u32 = 22712;
        // D s_79_1: read-reg s_79_0:struct
        let s_79_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // D s_79_2: call _get_MDCR_EL3_Type_NSPB(s_79_1)
        let s_79_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_79_1);
        // C s_79_3: const #1s : i
        let s_79_3: i128 = 1;
        // D s_79_4: cast zx s_79_2 -> bv
        let s_79_4: Bits = Bits::new(s_79_2 as u128, 2u16);
        // C s_79_5: const #1u : u64
        let s_79_5: u64 = 1;
        // D s_79_6: bit-extract s_79_4 s_79_3 s_79_5
        let s_79_6: Bits = (Bits::new(
            ((s_79_4) >> (s_79_3)).value(),
            u16::try_from(s_79_5).unwrap(),
        ));
        // D s_79_7: cast reint s_79_6 -> u8
        let s_79_7: bool = ((s_79_6.value()) != 0);
        // C s_79_8: const #0s : i
        let s_79_8: i128 = 0;
        // C s_79_9: const #0u : u64
        let s_79_9: u64 = 0;
        // D s_79_10: cast zx s_79_7 -> u64
        let s_79_10: u64 = (s_79_7 as u64);
        // C s_79_11: const #1u : u64
        let s_79_11: u64 = 1;
        // D s_79_12: and s_79_10 s_79_11
        let s_79_12: u64 = ((s_79_10) & (s_79_11));
        // D s_79_13: cmp-eq s_79_12 s_79_11
        let s_79_13: bool = ((s_79_12) == (s_79_11));
        // D s_79_14: lsl s_79_10 s_79_8
        let s_79_14: u64 = s_79_10 << s_79_8;
        // D s_79_15: or s_79_9 s_79_14
        let s_79_15: u64 = ((s_79_9) | (s_79_14));
        // D s_79_16: cmpl s_79_14
        let s_79_16: u64 = !s_79_14;
        // D s_79_17: and s_79_9 s_79_16
        let s_79_17: u64 = ((s_79_9) & (s_79_16));
        // D s_79_18: select s_79_13 s_79_15 s_79_17
        let s_79_18: u64 = if s_79_13 { s_79_15 } else { s_79_17 };
        // D s_79_19: cast trunc s_79_18 -> u8
        let s_79_19: bool = ((s_79_18) != 0);
        // C s_79_20: const #90704u : u32
        let s_79_20: u32 = 90704;
        // D s_79_21: read-reg s_79_20:struct
        let s_79_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_20 as isize);
            tracer.read_register(s_79_20 as isize, value);
            value
        };
        // D s_79_22: call _get_SCR_EL3_Type_NS(s_79_21)
        let s_79_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_79_21);
        // D s_79_23: cast zx s_79_19 -> bv
        let s_79_23: Bits = Bits::new(s_79_19 as u128, 1u16);
        // D s_79_24: cast zx s_79_22 -> bv
        let s_79_24: Bits = Bits::new(s_79_22 as u128, 1u16);
        // D s_79_25: cmp-ne s_79_23 s_79_24
        let s_79_25: bool = ((s_79_23) != (s_79_24));
        // D s_79_26: write-var gs#91071 <= s_79_25
        fn_state.gs_91071 = s_79_25;
        // N s_79_27: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#91071:u8
        let s_80_0: bool = fn_state.gs_91071;
        // N s_80_1: branch s_80_0 b86 b81
        if s_80_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #232u : u32
        let s_81_0: u32 = 232;
        // S s_81_1: call IsFeatureImplemented(s_81_0)
        let s_81_1: bool = IsFeatureImplemented(state, tracer, s_81_0);
        // N s_81_2: branch s_81_1 b85 b82
        if s_81_1 {
            return block_85(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#91072 <= s_82_0
        fn_state.gs_91072 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#91072:u8
        let s_83_0: bool = fn_state.gs_91072;
        // D s_83_1: write-var gs#91073 <= s_83_0
        fn_state.gs_91073 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#91073:u8
        let s_84_0: bool = fn_state.gs_91073;
        // D s_84_1: write-var gs#91074 <= s_84_0
        fn_state.gs_91074 = s_84_0;
        // N s_84_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var __MDCR_EL3_NSPBE:u8
        let s_85_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // D s_85_2: read-var __SCR_EL3_NSE:u8
        let s_85_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-ne s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) != (s_85_3));
        // D s_85_5: write-var gs#91072 <= s_85_4
        fn_state.gs_91072 = s_85_4;
        // N s_85_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #1u : u8
        let s_86_0: bool = true;
        // D s_86_1: write-var gs#91073 <= s_86_0
        fn_state.gs_91073 = s_86_0;
        // N s_86_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #1u : u8
        let s_87_0: bool = true;
        // D s_87_1: write-var gs#91071 <= s_87_0
        fn_state.gs_91071 = s_87_0;
        // N s_87_2: jump b80
        return block_80(state, tracer, fn_state);
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
        // D s_89_0: read-var __MDCR_EL2_TPMS:u8
        let s_89_0: bool = fn_state.u__MDCR_EL2_TPMS;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #1u : u8
        let s_89_2: bool = true;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // D s_89_5: write-var gs#91066 <= s_89_4
        fn_state.gs_91066 = s_89_4;
        // N s_89_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #24u : u8
        let s_90_0: u8 = 24;
        // C s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 8u16);
        // C s_90_2: cast zx s_90_1 -> i
        let s_90_2: i128 = (s_90_1.value() as i128);
        // C s_90_3: cast reint s_90_2 -> i64
        let s_90_3: i64 = (s_90_2 as i64);
        // C s_90_4: cast zx s_90_3 -> i
        let s_90_4: i128 = (i128::try_from(s_90_3).unwrap());
        // C s_90_5: const #432u : u32
        let s_90_5: u32 = 432;
        // D s_90_6: read-reg s_90_5:u8
        let s_90_6: u8 = {
            let value = state.read_register::<u8>(s_90_5 as isize);
            tracer.read_register(s_90_5 as isize, value);
            value
        };
        // D s_90_7: call AArch64_SystemAccessTrap(s_90_6, s_90_4)
        let s_90_7: () = AArch64_SystemAccessTrap(state, tracer, s_90_6, s_90_4);
        // N s_90_8: return
        return;
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var __HDFGWTR_EL2_PMSFCR_EL1:u8
        let s_91_0: bool = fn_state.u__HDFGWTR_EL2_PMSFCR_EL1;
        // D s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 1u16);
        // C s_91_2: const #1u : u8
        let s_91_2: bool = true;
        // C s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // D s_91_4: cmp-eq s_91_1 s_91_3
        let s_91_4: bool = ((s_91_1) == (s_91_3));
        // D s_91_5: write-var gs#91065 <= s_91_4
        fn_state.gs_91065 = s_91_4;
        // N s_91_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #424u : u32
        let s_92_0: u32 = 424;
        // D s_92_1: read-reg s_92_0:u8
        let s_92_1: u8 = {
            let value = state.read_register::<u8>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // C s_92_2: const #2u : u8
        let s_92_2: u8 = 2;
        // D s_92_3: cmp-lt s_92_1 s_92_2
        let s_92_3: bool = ((s_92_1) < (s_92_2));
        // D s_92_4: not s_92_3
        let s_92_4: bool = !s_92_3;
        // N s_92_5: branch s_92_4 b95 b93
        if s_92_4 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var __SCR_EL3_FGTEn:u8
        let s_93_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 1u16);
        // C s_93_2: const #1u : u8
        let s_93_2: bool = true;
        // C s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // D s_93_4: cmp-eq s_93_1 s_93_3
        let s_93_4: bool = ((s_93_1) == (s_93_3));
        // D s_93_5: write-var gs#91063 <= s_93_4
        fn_state.gs_91063 = s_93_4;
        // N s_93_6: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#91063:u8
        let s_94_0: bool = fn_state.gs_91063;
        // D s_94_1: write-var gs#91064 <= s_94_0
        fn_state.gs_91064 = s_94_0;
        // N s_94_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #1u : u8
        let s_95_0: bool = true;
        // D s_95_1: write-var gs#91063 <= s_95_0
        fn_state.gs_91063 = s_95_0;
        // N s_95_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #146u : u32
        let s_96_0: u32 = 146;
        // S s_96_1: call IsFeatureImplemented(s_96_0)
        let s_96_1: bool = IsFeatureImplemented(state, tracer, s_96_0);
        // D s_96_2: write-var gs#91062 <= s_96_1
        fn_state.gs_91062 = s_96_1;
        // N s_96_3: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_97_0: panic
        panic!("{:?}", ());
        // N s_97_1: return
        return;
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #22712u : u32
        let s_98_0: u32 = 22712;
        // D s_98_1: read-reg s_98_0:struct
        let s_98_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // D s_98_2: call _get_MDCR_EL3_Type_NSPB(s_98_1)
        let s_98_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_98_1);
        // C s_98_3: const #0s : i
        let s_98_3: i128 = 0;
        // D s_98_4: cast zx s_98_2 -> bv
        let s_98_4: Bits = Bits::new(s_98_2 as u128, 2u16);
        // C s_98_5: const #1u : u64
        let s_98_5: u64 = 1;
        // D s_98_6: bit-extract s_98_4 s_98_3 s_98_5
        let s_98_6: Bits = (Bits::new(
            ((s_98_4) >> (s_98_3)).value(),
            u16::try_from(s_98_5).unwrap(),
        ));
        // D s_98_7: cast reint s_98_6 -> u8
        let s_98_7: bool = ((s_98_6.value()) != 0);
        // C s_98_8: const #0s : i
        let s_98_8: i128 = 0;
        // C s_98_9: const #0u : u64
        let s_98_9: u64 = 0;
        // D s_98_10: cast zx s_98_7 -> u64
        let s_98_10: u64 = (s_98_7 as u64);
        // C s_98_11: const #1u : u64
        let s_98_11: u64 = 1;
        // D s_98_12: and s_98_10 s_98_11
        let s_98_12: u64 = ((s_98_10) & (s_98_11));
        // D s_98_13: cmp-eq s_98_12 s_98_11
        let s_98_13: bool = ((s_98_12) == (s_98_11));
        // D s_98_14: lsl s_98_10 s_98_8
        let s_98_14: u64 = s_98_10 << s_98_8;
        // D s_98_15: or s_98_9 s_98_14
        let s_98_15: u64 = ((s_98_9) | (s_98_14));
        // D s_98_16: cmpl s_98_14
        let s_98_16: u64 = !s_98_14;
        // D s_98_17: and s_98_9 s_98_16
        let s_98_17: u64 = ((s_98_9) & (s_98_16));
        // D s_98_18: select s_98_13 s_98_15 s_98_17
        let s_98_18: u64 = if s_98_13 { s_98_15 } else { s_98_17 };
        // D s_98_19: cast trunc s_98_18 -> u8
        let s_98_19: bool = ((s_98_18) != 0);
        // D s_98_20: cast zx s_98_19 -> bv
        let s_98_20: Bits = Bits::new(s_98_19 as u128, 1u16);
        // C s_98_21: const #0u : u8
        let s_98_21: bool = false;
        // C s_98_22: cast zx s_98_21 -> bv
        let s_98_22: Bits = Bits::new(s_98_21 as u128, 1u16);
        // D s_98_23: cmp-eq s_98_20 s_98_22
        let s_98_23: bool = ((s_98_20) == (s_98_22));
        // N s_98_24: branch s_98_23 b107 b99
        if s_98_23 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #22712u : u32
        let s_99_0: u32 = 22712;
        // D s_99_1: read-reg s_99_0:struct
        let s_99_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // D s_99_2: call _get_MDCR_EL3_Type_NSPB(s_99_1)
        let s_99_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_99_1);
        // C s_99_3: const #1s : i
        let s_99_3: i128 = 1;
        // D s_99_4: cast zx s_99_2 -> bv
        let s_99_4: Bits = Bits::new(s_99_2 as u128, 2u16);
        // C s_99_5: const #1u : u64
        let s_99_5: u64 = 1;
        // D s_99_6: bit-extract s_99_4 s_99_3 s_99_5
        let s_99_6: Bits = (Bits::new(
            ((s_99_4) >> (s_99_3)).value(),
            u16::try_from(s_99_5).unwrap(),
        ));
        // D s_99_7: cast reint s_99_6 -> u8
        let s_99_7: bool = ((s_99_6.value()) != 0);
        // C s_99_8: const #0s : i
        let s_99_8: i128 = 0;
        // C s_99_9: const #0u : u64
        let s_99_9: u64 = 0;
        // D s_99_10: cast zx s_99_7 -> u64
        let s_99_10: u64 = (s_99_7 as u64);
        // C s_99_11: const #1u : u64
        let s_99_11: u64 = 1;
        // D s_99_12: and s_99_10 s_99_11
        let s_99_12: u64 = ((s_99_10) & (s_99_11));
        // D s_99_13: cmp-eq s_99_12 s_99_11
        let s_99_13: bool = ((s_99_12) == (s_99_11));
        // D s_99_14: lsl s_99_10 s_99_8
        let s_99_14: u64 = s_99_10 << s_99_8;
        // D s_99_15: or s_99_9 s_99_14
        let s_99_15: u64 = ((s_99_9) | (s_99_14));
        // D s_99_16: cmpl s_99_14
        let s_99_16: u64 = !s_99_14;
        // D s_99_17: and s_99_9 s_99_16
        let s_99_17: u64 = ((s_99_9) & (s_99_16));
        // D s_99_18: select s_99_13 s_99_15 s_99_17
        let s_99_18: u64 = if s_99_13 { s_99_15 } else { s_99_17 };
        // D s_99_19: cast trunc s_99_18 -> u8
        let s_99_19: bool = ((s_99_18) != 0);
        // C s_99_20: const #90704u : u32
        let s_99_20: u32 = 90704;
        // D s_99_21: read-reg s_99_20:struct
        let s_99_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_99_20 as isize);
            tracer.read_register(s_99_20 as isize, value);
            value
        };
        // D s_99_22: call _get_SCR_EL3_Type_NS(s_99_21)
        let s_99_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_99_21);
        // D s_99_23: cast zx s_99_19 -> bv
        let s_99_23: Bits = Bits::new(s_99_19 as u128, 1u16);
        // D s_99_24: cast zx s_99_22 -> bv
        let s_99_24: Bits = Bits::new(s_99_22 as u128, 1u16);
        // D s_99_25: cmp-ne s_99_23 s_99_24
        let s_99_25: bool = ((s_99_23) != (s_99_24));
        // D s_99_26: write-var gs#91058 <= s_99_25
        fn_state.gs_91058 = s_99_25;
        // N s_99_27: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#91058:u8
        let s_100_0: bool = fn_state.gs_91058;
        // N s_100_1: branch s_100_0 b106 b101
        if s_100_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #232u : u32
        let s_101_0: u32 = 232;
        // S s_101_1: call IsFeatureImplemented(s_101_0)
        let s_101_1: bool = IsFeatureImplemented(state, tracer, s_101_0);
        // N s_101_2: branch s_101_1 b105 b102
        if s_101_1 {
            return block_105(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#91059 <= s_102_0
        fn_state.gs_91059 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#91059:u8
        let s_103_0: bool = fn_state.gs_91059;
        // D s_103_1: write-var gs#91060 <= s_103_0
        fn_state.gs_91060 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#91060:u8
        let s_104_0: bool = fn_state.gs_91060;
        // D s_104_1: write-var gs#91061 <= s_104_0
        fn_state.gs_91061 = s_104_0;
        // N s_104_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var __MDCR_EL3_NSPBE:u8
        let s_105_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 1u16);
        // D s_105_2: read-var __SCR_EL3_NSE:u8
        let s_105_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_105_3: cast zx s_105_2 -> bv
        let s_105_3: Bits = Bits::new(s_105_2 as u128, 1u16);
        // D s_105_4: cmp-ne s_105_1 s_105_3
        let s_105_4: bool = ((s_105_1) != (s_105_3));
        // D s_105_5: write-var gs#91059 <= s_105_4
        fn_state.gs_91059 = s_105_4;
        // N s_105_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #1u : u8
        let s_106_0: bool = true;
        // D s_106_1: write-var gs#91060 <= s_106_0
        fn_state.gs_91060 = s_106_0;
        // N s_106_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #1u : u8
        let s_107_0: bool = true;
        // D s_107_1: write-var gs#91058 <= s_107_0
        fn_state.gs_91058 = s_107_0;
        // N s_107_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_108_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_108_1: call __IMPDEF_boolean(s_108_0)
        let s_108_1: bool = u__IMPDEF_boolean(state, tracer, s_108_0);
        // D s_108_2: write-var gs#91053 <= s_108_1
        fn_state.gs_91053 = s_108_1;
        // N s_108_3: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call EDSCR_read(s_109_0)
        let s_109_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_109_0);
        // S s_109_2: call _get_EDSCR_Type_SDD(s_109_1)
        let s_109_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_109_1);
        // S s_109_3: cast zx s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 1u16);
        // C s_109_4: const #1u : u8
        let s_109_4: bool = true;
        // C s_109_5: cast zx s_109_4 -> bv
        let s_109_5: Bits = Bits::new(s_109_4 as u128, 1u16);
        // S s_109_6: cmp-eq s_109_3 s_109_5
        let s_109_6: bool = ((s_109_3) == (s_109_5));
        // D s_109_7: write-var gs#91052 <= s_109_6
        fn_state.gs_91052 = s_109_6;
        // N s_109_8: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #424u : u32
        let s_110_0: u32 = 424;
        // D s_110_1: read-reg s_110_0:u8
        let s_110_1: u8 = {
            let value = state.read_register::<u8>(s_110_0 as isize);
            tracer.read_register(s_110_0 as isize, value);
            value
        };
        // C s_110_2: const #2u : u8
        let s_110_2: u8 = 2;
        // D s_110_3: cmp-lt s_110_1 s_110_2
        let s_110_3: bool = ((s_110_1) < (s_110_2));
        // D s_110_4: write-var gs#91051 <= s_110_3
        fn_state.gs_91051 = s_110_3;
        // N s_110_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_111_0: panic
        panic!("{:?}", ());
        // N s_111_1: return
        return;
    }
}
