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
use SPMEVFILT2R_EL0_set::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_E2H::*;
use Halted::*;
use IsFeatureImplemented::*;
use u_get_MDCR_EL2_Type_EnSPM::*;
use X_read::*;
use u_get_SPMSELR_EL0_Type_SYSPMUSEL::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u__get_selected_SPMACCESSR_EL2_field::*;
use u__get_selected_SPMACCESSR_EL1_field::*;
use u_get_MDSCR_EL1_Type_EnSPM::*;
use u_get_HDFGWTR2_EL2_Type_nSPMEVTYPERn_EL0::*;
use u_get_SCR_EL3_Type_FGTEn2::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_SPMSELR_EL0_Type_BANK::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_MDCR_EL3_Type_EnPM2::*;
use EDSCR_read::*;
use u__get_selected_SPMACCESSR_EL3_field::*;
use common::*;
pub fn SPMEVFILT2R_EL0_SysRegWrite_a4da6c90289f656a<T: Tracer>(
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
        u__MDCR_EL3_EnPM2: bool,
        gs_94050: bool,
        gs_94072: bool,
        gs_94022: bool,
        gs_94046: bool,
        gs_94065: bool,
        gs_94037: bool,
        gs_94039: bool,
        gs_94056: bool,
        gs_94044: bool,
        gs_94043: bool,
        u__SCR_EL3_FGTEn2: bool,
        gs_94060: bool,
        gs_94018: bool,
        gs_94042: bool,
        gs_94017: bool,
        u__HCR_EL2_TGE: bool,
        gs_94063: bool,
        gs_94029: bool,
        gs_94016: bool,
        gs_94015: bool,
        gs_94076: bool,
        gs_94066: bool,
        gs_94021: bool,
        gs_94040: bool,
        gs_94077: bool,
        gs_94033: bool,
        gs_94032: bool,
        gs_94028: bool,
        gs_94078: bool,
        gs_94062: bool,
        gs_94059: bool,
        gs_94051: bool,
        gs_94019: bool,
        gs_94041: bool,
        gs_94068: bool,
        gs_94057: bool,
        gs_94070: bool,
        gs_94020: bool,
        gs_94058: bool,
        u__HDFGWTR2_EL2_nSPMEVTYPERn_EL0: bool,
        gs_94052: bool,
        gs_94067: bool,
        gs_94035: bool,
        u__PSTATE_EL: u8,
        gs_94036: bool,
        gs_94061: bool,
        gs_94024: bool,
        gs_94054: bool,
        gs_94034: bool,
        gs_94069: bool,
        gs_94053: bool,
        gs_94055: bool,
        gs_94023: bool,
        gs_94079: bool,
        u__MDSCR_EL1_EnSPM: bool,
        u__MDCR_EL2_EnSPM: bool,
        gs_94071: bool,
        gs_94038: bool,
        gs_94064: bool,
        gs_94031: bool,
        gs_94030: bool,
        gs_94045: bool,
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
        // D s_0_5: call _get_MDCR_EL3_Type_EnPM2(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_EnPM2(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_EnPM2 <= s_0_5
        fn_state.u__MDCR_EL3_EnPM2 = s_0_5;
        // C s_0_7: const #104648u : u32
        let s_0_7: u32 = 104648;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDSCR_EL1_Type_EnSPM(s_0_8)
        let s_0_9: bool = u_get_MDSCR_EL1_Type_EnSPM(state, tracer, s_0_8);
        // D s_0_10: write-var __MDSCR_EL1_EnSPM <= s_0_9
        fn_state.u__MDSCR_EL1_EnSPM = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TGE(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TGE <= s_0_13
        fn_state.u__HCR_EL2_TGE = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_FGTEn2(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_FGTEn2(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_FGTEn2 <= s_0_17
        fn_state.u__SCR_EL3_FGTEn2 = s_0_17;
        // C s_0_19: const #17664u : u32
        let s_0_19: u32 = 17664;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HDFGWTR2_EL2_Type_nSPMEVTYPERn_EL0(s_0_20)
        let s_0_21: bool = u_get_HDFGWTR2_EL2_Type_nSPMEVTYPERn_EL0(
            state,
            tracer,
            s_0_20,
        );
        // D s_0_22: write-var __HDFGWTR2_EL2_nSPMEVTYPERn_EL0 <= s_0_21
        fn_state.u__HDFGWTR2_EL2_nSPMEVTYPERn_EL0 = s_0_21;
        // C s_0_23: const #104880u : u32
        let s_0_23: u32 = 104880;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_MDCR_EL2_Type_EnSPM(s_0_24)
        let s_0_25: bool = u_get_MDCR_EL2_Type_EnSPM(state, tracer, s_0_24);
        // D s_0_26: write-var __MDCR_EL2_EnSPM <= s_0_25
        fn_state.u__MDCR_EL2_EnSPM = s_0_25;
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
        // N s_0_33: branch s_0_32 b133 b1
        if s_0_32 {
            return block_133(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b55 b2
        if s_1_5 {
            return block_55(state, tracer, fn_state);
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
        // C s_5_0: const #16552u : u32
        let s_5_0: u32 = 16552;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_5_1)
        let s_5_2: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_5_1);
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 6u16);
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (s_5_3.value() as i128);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // C s_5_6: const #16552u : u32
        let s_5_6: u32 = 16552;
        // D s_5_7: read-reg s_5_6:struct
        let s_5_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_6 as isize);
            tracer.read_register(s_5_6 as isize, value);
            value
        };
        // D s_5_8: call _get_SPMSELR_EL0_Type_BANK(s_5_7)
        let s_5_8: u8 = u_get_SPMSELR_EL0_Type_BANK(state, tracer, s_5_7);
        // D s_5_9: cast zx s_5_8 -> bv
        let s_5_9: Bits = Bits::new(s_5_8 as u128, 2u16);
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (s_5_9.value() as i128);
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // C s_5_12: const #16s : i
        let s_5_12: i128 = 16;
        // D s_5_13: cast zx s_5_11 -> i
        let s_5_13: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_14: mul s_5_13 s_5_12
        let s_5_14: i128 = ((s_5_13) * (s_5_12));
        // D s_5_15: cast reint s_5_14 -> i64
        let s_5_15: i64 = (s_5_14 as i64);
        // C s_5_16: const #14s : i
        let s_5_16: i128 = 14;
        // D s_5_17: cast zx s_5_15 -> i
        let s_5_17: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_18: add s_5_17 s_5_16
        let s_5_18: i128 = (s_5_17 + s_5_16);
        // D s_5_19: cast reint s_5_18 -> i64
        let s_5_19: i64 = (s_5_18 as i64);
        // C s_5_20: const #64s : i64
        let s_5_20: i64 = 64;
        // D s_5_21: read-var t:i
        let s_5_21: i128 = fn_state.t;
        // D s_5_22: call X_read(s_5_21, s_5_20)
        let s_5_22: Bits = X_read(state, tracer, s_5_21, s_5_20);
        // D s_5_23: cast reint s_5_22 -> u64
        let s_5_23: u64 = (s_5_22.value() as u64);
        // D s_5_24: cast zx s_5_5 -> i
        let s_5_24: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_25: cast zx s_5_19 -> i
        let s_5_25: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_26: call SPMEVFILT2R_EL0_set(s_5_24, s_5_25, s_5_23)
        let s_5_26: () = SPMEVFILT2R_EL0_set(state, tracer, s_5_24, s_5_25, s_5_23);
        // N s_5_27: return
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
        // N s_6_2: branch s_6_1 b54 b7
        if s_6_1 {
            return block_54(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#94015 <= s_7_0
        fn_state.gs_94015 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#94015:u8
        let s_8_0: bool = fn_state.gs_94015;
        // N s_8_1: branch s_8_0 b53 b9
        if s_8_0 {
            return block_53(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#94016 <= s_9_0
        fn_state.gs_94016 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#94016:u8
        let s_10_0: bool = fn_state.gs_94016;
        // N s_10_1: branch s_10_0 b52 b11
        if s_10_0 {
            return block_52(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#94017 <= s_11_0
        fn_state.gs_94017 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#94017:u8
        let s_12_0: bool = fn_state.gs_94017;
        // N s_12_1: branch s_12_0 b51 b13
        if s_12_0 {
            return block_51(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#94018 <= s_13_0
        fn_state.gs_94018 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#94018:u8
        let s_14_0: bool = fn_state.gs_94018;
        // N s_14_1: branch s_14_0 b50 b15
        if s_14_0 {
            return block_50(state, tracer, fn_state);
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
        // S s_15_1: call Halted(s_15_0)
        let s_15_1: bool = Halted(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b49 b16
        if s_15_1 {
            return block_49(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#94019 <= s_16_0
        fn_state.gs_94019 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#94019:u8
        let s_17_0: bool = fn_state.gs_94019;
        // N s_17_1: branch s_17_0 b48 b18
        if s_17_0 {
            return block_48(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#94020 <= s_18_0
        fn_state.gs_94020 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#94020:u8
        let s_19_0: bool = fn_state.gs_94020;
        // N s_19_1: branch s_19_0 b47 b20
        if s_19_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#94021 <= s_20_0
        fn_state.gs_94021 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#94021:u8
        let s_21_0: bool = fn_state.gs_94021;
        // N s_21_1: branch s_21_0 b46 b22
        if s_21_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#94022 <= s_22_0
        fn_state.gs_94022 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#94022:u8
        let s_23_0: bool = fn_state.gs_94022;
        // N s_23_1: branch s_23_0 b45 b24
        if s_23_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #424u : u32
        let s_24_0: u32 = 424;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // C s_24_2: const #2u : u8
        let s_24_2: u8 = 2;
        // D s_24_3: cmp-lt s_24_1 s_24_2
        let s_24_3: bool = ((s_24_1) < (s_24_2));
        // N s_24_4: branch s_24_3 b44 b25
        if s_24_3 {
            return block_44(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#94023 <= s_25_0
        fn_state.gs_94023 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#94023:u8
        let s_26_0: bool = fn_state.gs_94023;
        // N s_26_1: branch s_26_0 b38 b27
        if s_26_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #424u : u32
        let s_27_0: u32 = 424;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // C s_27_2: const #2u : u8
        let s_27_2: u8 = 2;
        // D s_27_3: cmp-lt s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) < (s_27_2));
        // N s_27_4: branch s_27_3 b37 b28
        if s_27_3 {
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
        // D s_28_1: write-var gs#94024 <= s_28_0
        fn_state.gs_94024 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#94024:u8
        let s_29_0: bool = fn_state.gs_94024;
        // N s_29_1: branch s_29_0 b31 b30
        if s_29_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #16552u : u32
        let s_30_0: u32 = 16552;
        // D s_30_1: read-reg s_30_0:struct
        let s_30_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_30_1)
        let s_30_2: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_30_1);
        // D s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 6u16);
        // D s_30_4: cast zx s_30_3 -> i
        let s_30_4: i128 = (s_30_3.value() as i128);
        // D s_30_5: cast reint s_30_4 -> i64
        let s_30_5: i64 = (s_30_4 as i64);
        // C s_30_6: const #16552u : u32
        let s_30_6: u32 = 16552;
        // D s_30_7: read-reg s_30_6:struct
        let s_30_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_6 as isize);
            tracer.read_register(s_30_6 as isize, value);
            value
        };
        // D s_30_8: call _get_SPMSELR_EL0_Type_BANK(s_30_7)
        let s_30_8: u8 = u_get_SPMSELR_EL0_Type_BANK(state, tracer, s_30_7);
        // D s_30_9: cast zx s_30_8 -> bv
        let s_30_9: Bits = Bits::new(s_30_8 as u128, 2u16);
        // D s_30_10: cast zx s_30_9 -> i
        let s_30_10: i128 = (s_30_9.value() as i128);
        // D s_30_11: cast reint s_30_10 -> i64
        let s_30_11: i64 = (s_30_10 as i64);
        // C s_30_12: const #16s : i
        let s_30_12: i128 = 16;
        // D s_30_13: cast zx s_30_11 -> i
        let s_30_13: i128 = (i128::try_from(s_30_11).unwrap());
        // D s_30_14: mul s_30_13 s_30_12
        let s_30_14: i128 = ((s_30_13) * (s_30_12));
        // D s_30_15: cast reint s_30_14 -> i64
        let s_30_15: i64 = (s_30_14 as i64);
        // C s_30_16: const #14s : i
        let s_30_16: i128 = 14;
        // D s_30_17: cast zx s_30_15 -> i
        let s_30_17: i128 = (i128::try_from(s_30_15).unwrap());
        // D s_30_18: add s_30_17 s_30_16
        let s_30_18: i128 = (s_30_17 + s_30_16);
        // D s_30_19: cast reint s_30_18 -> i64
        let s_30_19: i64 = (s_30_18 as i64);
        // C s_30_20: const #64s : i64
        let s_30_20: i64 = 64;
        // D s_30_21: read-var t:i
        let s_30_21: i128 = fn_state.t;
        // D s_30_22: call X_read(s_30_21, s_30_20)
        let s_30_22: Bits = X_read(state, tracer, s_30_21, s_30_20);
        // D s_30_23: cast reint s_30_22 -> u64
        let s_30_23: u64 = (s_30_22.value() as u64);
        // D s_30_24: cast zx s_30_5 -> i
        let s_30_24: i128 = (i128::try_from(s_30_5).unwrap());
        // D s_30_25: cast zx s_30_19 -> i
        let s_30_25: i128 = (i128::try_from(s_30_19).unwrap());
        // D s_30_26: call SPMEVFILT2R_EL0_set(s_30_24, s_30_25, s_30_23)
        let s_30_26: () = SPMEVFILT2R_EL0_set(state, tracer, s_30_24, s_30_25, s_30_23);
        // N s_30_27: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call Halted(s_31_0)
        let s_31_1: bool = Halted(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b36 b32
        if s_31_1 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#94028 <= s_32_0
        fn_state.gs_94028 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#94028:u8
        let s_33_0: bool = fn_state.gs_94028;
        // N s_33_1: branch s_33_0 b35 b34
        if s_33_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #24u : u8
        let s_34_0: u8 = 24;
        // C s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 8u16);
        // C s_34_2: cast zx s_34_1 -> i
        let s_34_2: i128 = (s_34_1.value() as i128);
        // C s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: cast zx s_34_3 -> i
        let s_34_4: i128 = (i128::try_from(s_34_3).unwrap());
        // C s_34_5: const #424u : u32
        let s_34_5: u32 = 424;
        // D s_34_6: read-reg s_34_5:u8
        let s_34_6: u8 = {
            let value = state.read_register::<u8>(s_34_5 as isize);
            tracer.read_register(s_34_5 as isize, value);
            value
        };
        // D s_34_7: call AArch64_SystemAccessTrap(s_34_6, s_34_4)
        let s_34_7: () = AArch64_SystemAccessTrap(state, tracer, s_34_6, s_34_4);
        // N s_34_8: return
        return;
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
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call EDSCR_read(s_36_0)
        let s_36_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_36_0);
        // S s_36_2: call _get_EDSCR_Type_SDD(s_36_1)
        let s_36_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_36_1);
        // S s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // S s_36_6: cmp-eq s_36_3 s_36_5
        let s_36_6: bool = ((s_36_3) == (s_36_5));
        // D s_36_7: write-var gs#94028 <= s_36_6
        fn_state.gs_94028 = s_36_6;
        // N s_36_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call __get_selected_SPMACCESSR_EL3_field(s_37_0)
        let s_37_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_37_0);
        // S s_37_2: cast zx s_37_1 -> bv
        let s_37_2: Bits = Bits::new(s_37_1 as u128, 2u16);
        // C s_37_3: const #3u : u8
        let s_37_3: u8 = 3;
        // C s_37_4: cast zx s_37_3 -> bv
        let s_37_4: Bits = Bits::new(s_37_3 as u128, 2u16);
        // S s_37_5: cmp-ne s_37_2 s_37_4
        let s_37_5: bool = ((s_37_2) != (s_37_4));
        // D s_37_6: write-var gs#94024 <= s_37_5
        fn_state.gs_94024 = s_37_5;
        // N s_37_7: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call Halted(s_38_0)
        let s_38_1: bool = Halted(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b43 b39
        if s_38_1 {
            return block_43(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#94029 <= s_39_0
        fn_state.gs_94029 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#94029:u8
        let s_40_0: bool = fn_state.gs_94029;
        // N s_40_1: branch s_40_0 b42 b41
        if s_40_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #24u : u8
        let s_41_0: u8 = 24;
        // C s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 8u16);
        // C s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (s_41_1.value() as i128);
        // C s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: cast zx s_41_3 -> i
        let s_41_4: i128 = (i128::try_from(s_41_3).unwrap());
        // C s_41_5: const #424u : u32
        let s_41_5: u32 = 424;
        // D s_41_6: read-reg s_41_5:u8
        let s_41_6: u8 = {
            let value = state.read_register::<u8>(s_41_5 as isize);
            tracer.read_register(s_41_5 as isize, value);
            value
        };
        // D s_41_7: call AArch64_SystemAccessTrap(s_41_6, s_41_4)
        let s_41_7: () = AArch64_SystemAccessTrap(state, tracer, s_41_6, s_41_4);
        // N s_41_8: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: panic
        panic!("{:?}", ());
        // N s_42_1: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call EDSCR_read(s_43_0)
        let s_43_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_43_0);
        // S s_43_2: call _get_EDSCR_Type_SDD(s_43_1)
        let s_43_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_43_1);
        // S s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // C s_43_4: const #1u : u8
        let s_43_4: bool = true;
        // C s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 1u16);
        // S s_43_6: cmp-eq s_43_3 s_43_5
        let s_43_6: bool = ((s_43_3) == (s_43_5));
        // D s_43_7: write-var gs#94029 <= s_43_6
        fn_state.gs_94029 = s_43_6;
        // N s_43_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __MDCR_EL3_EnPM2:u8
        let s_44_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #0u : u8
        let s_44_2: bool = false;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#94023 <= s_44_4
        fn_state.gs_94023 = s_44_4;
        // N s_44_6: jump b26
        return block_26(state, tracer, fn_state);
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
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call __get_selected_SPMACCESSR_EL3_field(s_46_0)
        let s_46_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_46_0);
        // S s_46_2: cast zx s_46_1 -> bv
        let s_46_2: Bits = Bits::new(s_46_1 as u128, 2u16);
        // C s_46_3: const #3u : u8
        let s_46_3: u8 = 3;
        // C s_46_4: cast zx s_46_3 -> bv
        let s_46_4: Bits = Bits::new(s_46_3 as u128, 2u16);
        // S s_46_5: cmp-ne s_46_2 s_46_4
        let s_46_5: bool = ((s_46_2) != (s_46_4));
        // D s_46_6: write-var gs#94022 <= s_46_5
        fn_state.gs_94022 = s_46_5;
        // N s_46_7: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_47_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_47_1: call __IMPDEF_boolean(s_47_0)
        let s_47_1: bool = u__IMPDEF_boolean(state, tracer, s_47_0);
        // D s_47_2: write-var gs#94021 <= s_47_1
        fn_state.gs_94021 = s_47_1;
        // N s_47_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EDSCR_read(s_48_0)
        let s_48_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_48_0);
        // S s_48_2: call _get_EDSCR_Type_SDD(s_48_1)
        let s_48_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_48_1);
        // S s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // C s_48_4: const #1u : u8
        let s_48_4: bool = true;
        // C s_48_5: cast zx s_48_4 -> bv
        let s_48_5: Bits = Bits::new(s_48_4 as u128, 1u16);
        // S s_48_6: cmp-eq s_48_3 s_48_5
        let s_48_6: bool = ((s_48_3) == (s_48_5));
        // D s_48_7: write-var gs#94020 <= s_48_6
        fn_state.gs_94020 = s_48_6;
        // N s_48_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #424u : u32
        let s_49_0: u32 = 424;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // C s_49_2: const #2u : u8
        let s_49_2: u8 = 2;
        // D s_49_3: cmp-lt s_49_1 s_49_2
        let s_49_3: bool = ((s_49_1) < (s_49_2));
        // D s_49_4: write-var gs#94019 <= s_49_3
        fn_state.gs_94019 = s_49_3;
        // N s_49_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: panic
        panic!("{:?}", ());
        // N s_50_1: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var __MDCR_EL3_EnPM2:u8
        let s_51_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #0u : u8
        let s_51_2: bool = false;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#94018 <= s_51_4
        fn_state.gs_94018 = s_51_4;
        // N s_51_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_52_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_52_1: call __IMPDEF_boolean(s_52_0)
        let s_52_1: bool = u__IMPDEF_boolean(state, tracer, s_52_0);
        // D s_52_2: write-var gs#94017 <= s_52_1
        fn_state.gs_94017 = s_52_1;
        // N s_52_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EDSCR_read(s_53_0)
        let s_53_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_53_0);
        // S s_53_2: call _get_EDSCR_Type_SDD(s_53_1)
        let s_53_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_53_1);
        // S s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // C s_53_4: const #1u : u8
        let s_53_4: bool = true;
        // C s_53_5: cast zx s_53_4 -> bv
        let s_53_5: Bits = Bits::new(s_53_4 as u128, 1u16);
        // S s_53_6: cmp-eq s_53_3 s_53_5
        let s_53_6: bool = ((s_53_3) == (s_53_5));
        // D s_53_7: write-var gs#94016 <= s_53_6
        fn_state.gs_94016 = s_53_6;
        // N s_53_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #424u : u32
        let s_54_0: u32 = 424;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // C s_54_2: const #2u : u8
        let s_54_2: u8 = 2;
        // D s_54_3: cmp-lt s_54_1 s_54_2
        let s_54_3: bool = ((s_54_1) < (s_54_2));
        // D s_54_4: write-var gs#94015 <= s_54_3
        fn_state.gs_94015 = s_54_3;
        // N s_54_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call Halted(s_55_0)
        let s_55_1: bool = Halted(state, tracer, s_55_0);
        // N s_55_2: branch s_55_1 b132 b56
        if s_55_1 {
            return block_132(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#94030 <= s_56_0
        fn_state.gs_94030 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#94030:u8
        let s_57_0: bool = fn_state.gs_94030;
        // N s_57_1: branch s_57_0 b131 b58
        if s_57_0 {
            return block_131(state, tracer, fn_state);
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
        // D s_58_1: write-var gs#94031 <= s_58_0
        fn_state.gs_94031 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#94031:u8
        let s_59_0: bool = fn_state.gs_94031;
        // N s_59_1: branch s_59_0 b130 b60
        if s_59_0 {
            return block_130(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#94032 <= s_60_0
        fn_state.gs_94032 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#94032:u8
        let s_61_0: bool = fn_state.gs_94032;
        // N s_61_1: branch s_61_0 b129 b62
        if s_61_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#94033 <= s_62_0
        fn_state.gs_94033 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#94033:u8
        let s_63_0: bool = fn_state.gs_94033;
        // N s_63_1: branch s_63_0 b128 b64
        if s_63_0 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call Halted(s_64_0)
        let s_64_1: bool = Halted(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b127 b65
        if s_64_1 {
            return block_127(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#94034 <= s_65_0
        fn_state.gs_94034 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#94034:u8
        let s_66_0: bool = fn_state.gs_94034;
        // N s_66_1: branch s_66_0 b126 b67
        if s_66_0 {
            return block_126(state, tracer, fn_state);
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
        // D s_67_1: write-var gs#94035 <= s_67_0
        fn_state.gs_94035 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#94035:u8
        let s_68_0: bool = fn_state.gs_94035;
        // N s_68_1: branch s_68_0 b125 b69
        if s_68_0 {
            return block_125(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#94036 <= s_69_0
        fn_state.gs_94036 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#94036:u8
        let s_70_0: bool = fn_state.gs_94036;
        // N s_70_1: branch s_70_0 b124 b71
        if s_70_0 {
            return block_124(state, tracer, fn_state);
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
        // D s_71_1: write-var gs#94037 <= s_71_0
        fn_state.gs_94037 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#94037:u8
        let s_72_0: bool = fn_state.gs_94037;
        // N s_72_1: branch s_72_0 b123 b73
        if s_72_0 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
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
        // N s_73_2: branch s_73_1 b122 b74
        if s_73_1 {
            return block_122(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#94038 <= s_74_0
        fn_state.gs_94038 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#94038:u8
        let s_75_0: bool = fn_state.gs_94038;
        // N s_75_1: branch s_75_0 b121 b76
        if s_75_0 {
            return block_121(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#94039 <= s_76_0
        fn_state.gs_94039 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#94039:u8
        let s_77_0: bool = fn_state.gs_94039;
        // N s_77_1: branch s_77_0 b120 b78
        if s_77_0 {
            return block_120(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#94040 <= s_78_0
        fn_state.gs_94040 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#94040:u8
        let s_79_0: bool = fn_state.gs_94040;
        // N s_79_1: branch s_79_0 b119 b80
        if s_79_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call EL2Enabled(s_80_0)
        let s_80_1: bool = EL2Enabled(state, tracer, s_80_0);
        // N s_80_2: branch s_80_1 b118 b81
        if s_80_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#94041 <= s_81_0
        fn_state.gs_94041 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#94041:u8
        let s_82_0: bool = fn_state.gs_94041;
        // N s_82_1: branch s_82_0 b117 b83
        if s_82_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#94042 <= s_83_0
        fn_state.gs_94042 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#94042:u8
        let s_84_0: bool = fn_state.gs_94042;
        // N s_84_1: branch s_84_0 b116 b85
        if s_84_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #() : ()
        let s_85_0: () = ();
        // S s_85_1: call EL2Enabled(s_85_0)
        let s_85_1: bool = EL2Enabled(state, tracer, s_85_0);
        // N s_85_2: branch s_85_1 b115 b86
        if s_85_1 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#94043 <= s_86_0
        fn_state.gs_94043 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#94043:u8
        let s_87_0: bool = fn_state.gs_94043;
        // N s_87_1: branch s_87_0 b114 b88
        if s_87_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #() : ()
        let s_88_0: () = ();
        // S s_88_1: call EL2Enabled(s_88_0)
        let s_88_1: bool = EL2Enabled(state, tracer, s_88_0);
        // N s_88_2: branch s_88_1 b113 b89
        if s_88_1 {
            return block_113(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#94044 <= s_89_0
        fn_state.gs_94044 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#94044:u8
        let s_90_0: bool = fn_state.gs_94044;
        // N s_90_1: branch s_90_0 b112 b91
        if s_90_0 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #424u : u32
        let s_91_0: u32 = 424;
        // D s_91_1: read-reg s_91_0:u8
        let s_91_1: u8 = {
            let value = state.read_register::<u8>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // C s_91_2: const #2u : u8
        let s_91_2: u8 = 2;
        // D s_91_3: cmp-lt s_91_1 s_91_2
        let s_91_3: bool = ((s_91_1) < (s_91_2));
        // N s_91_4: branch s_91_3 b111 b92
        if s_91_3 {
            return block_111(state, tracer, fn_state);
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
        // D s_92_1: write-var gs#94045 <= s_92_0
        fn_state.gs_94045 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#94045:u8
        let s_93_0: bool = fn_state.gs_94045;
        // N s_93_1: branch s_93_0 b105 b94
        if s_93_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #424u : u32
        let s_94_0: u32 = 424;
        // D s_94_1: read-reg s_94_0:u8
        let s_94_1: u8 = {
            let value = state.read_register::<u8>(s_94_0 as isize);
            tracer.read_register(s_94_0 as isize, value);
            value
        };
        // C s_94_2: const #2u : u8
        let s_94_2: u8 = 2;
        // D s_94_3: cmp-lt s_94_1 s_94_2
        let s_94_3: bool = ((s_94_1) < (s_94_2));
        // N s_94_4: branch s_94_3 b104 b95
        if s_94_3 {
            return block_104(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#94046 <= s_95_0
        fn_state.gs_94046 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#94046:u8
        let s_96_0: bool = fn_state.gs_94046;
        // N s_96_1: branch s_96_0 b98 b97
        if s_96_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #16552u : u32
        let s_97_0: u32 = 16552;
        // D s_97_1: read-reg s_97_0:struct
        let s_97_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // D s_97_2: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_97_1)
        let s_97_2: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_97_1);
        // D s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 6u16);
        // D s_97_4: cast zx s_97_3 -> i
        let s_97_4: i128 = (s_97_3.value() as i128);
        // D s_97_5: cast reint s_97_4 -> i64
        let s_97_5: i64 = (s_97_4 as i64);
        // C s_97_6: const #16552u : u32
        let s_97_6: u32 = 16552;
        // D s_97_7: read-reg s_97_6:struct
        let s_97_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_97_6 as isize);
            tracer.read_register(s_97_6 as isize, value);
            value
        };
        // D s_97_8: call _get_SPMSELR_EL0_Type_BANK(s_97_7)
        let s_97_8: u8 = u_get_SPMSELR_EL0_Type_BANK(state, tracer, s_97_7);
        // D s_97_9: cast zx s_97_8 -> bv
        let s_97_9: Bits = Bits::new(s_97_8 as u128, 2u16);
        // D s_97_10: cast zx s_97_9 -> i
        let s_97_10: i128 = (s_97_9.value() as i128);
        // D s_97_11: cast reint s_97_10 -> i64
        let s_97_11: i64 = (s_97_10 as i64);
        // C s_97_12: const #16s : i
        let s_97_12: i128 = 16;
        // D s_97_13: cast zx s_97_11 -> i
        let s_97_13: i128 = (i128::try_from(s_97_11).unwrap());
        // D s_97_14: mul s_97_13 s_97_12
        let s_97_14: i128 = ((s_97_13) * (s_97_12));
        // D s_97_15: cast reint s_97_14 -> i64
        let s_97_15: i64 = (s_97_14 as i64);
        // C s_97_16: const #14s : i
        let s_97_16: i128 = 14;
        // D s_97_17: cast zx s_97_15 -> i
        let s_97_17: i128 = (i128::try_from(s_97_15).unwrap());
        // D s_97_18: add s_97_17 s_97_16
        let s_97_18: i128 = (s_97_17 + s_97_16);
        // D s_97_19: cast reint s_97_18 -> i64
        let s_97_19: i64 = (s_97_18 as i64);
        // C s_97_20: const #64s : i64
        let s_97_20: i64 = 64;
        // D s_97_21: read-var t:i
        let s_97_21: i128 = fn_state.t;
        // D s_97_22: call X_read(s_97_21, s_97_20)
        let s_97_22: Bits = X_read(state, tracer, s_97_21, s_97_20);
        // D s_97_23: cast reint s_97_22 -> u64
        let s_97_23: u64 = (s_97_22.value() as u64);
        // D s_97_24: cast zx s_97_5 -> i
        let s_97_24: i128 = (i128::try_from(s_97_5).unwrap());
        // D s_97_25: cast zx s_97_19 -> i
        let s_97_25: i128 = (i128::try_from(s_97_19).unwrap());
        // D s_97_26: call SPMEVFILT2R_EL0_set(s_97_24, s_97_25, s_97_23)
        let s_97_26: () = SPMEVFILT2R_EL0_set(state, tracer, s_97_24, s_97_25, s_97_23);
        // N s_97_27: return
        return;
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call Halted(s_98_0)
        let s_98_1: bool = Halted(state, tracer, s_98_0);
        // N s_98_2: branch s_98_1 b103 b99
        if s_98_1 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#94050 <= s_99_0
        fn_state.gs_94050 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#94050:u8
        let s_100_0: bool = fn_state.gs_94050;
        // N s_100_1: branch s_100_0 b102 b101
        if s_100_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #24u : u8
        let s_101_0: u8 = 24;
        // C s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 8u16);
        // C s_101_2: cast zx s_101_1 -> i
        let s_101_2: i128 = (s_101_1.value() as i128);
        // C s_101_3: cast reint s_101_2 -> i64
        let s_101_3: i64 = (s_101_2 as i64);
        // C s_101_4: cast zx s_101_3 -> i
        let s_101_4: i128 = (i128::try_from(s_101_3).unwrap());
        // C s_101_5: const #424u : u32
        let s_101_5: u32 = 424;
        // D s_101_6: read-reg s_101_5:u8
        let s_101_6: u8 = {
            let value = state.read_register::<u8>(s_101_5 as isize);
            tracer.read_register(s_101_5 as isize, value);
            value
        };
        // D s_101_7: call AArch64_SystemAccessTrap(s_101_6, s_101_4)
        let s_101_7: () = AArch64_SystemAccessTrap(state, tracer, s_101_6, s_101_4);
        // N s_101_8: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_102_0: panic
        panic!("{:?}", ());
        // N s_102_1: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #() : ()
        let s_103_0: () = ();
        // S s_103_1: call EDSCR_read(s_103_0)
        let s_103_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_103_0);
        // S s_103_2: call _get_EDSCR_Type_SDD(s_103_1)
        let s_103_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_103_1);
        // S s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 1u16);
        // C s_103_4: const #1u : u8
        let s_103_4: bool = true;
        // C s_103_5: cast zx s_103_4 -> bv
        let s_103_5: Bits = Bits::new(s_103_4 as u128, 1u16);
        // S s_103_6: cmp-eq s_103_3 s_103_5
        let s_103_6: bool = ((s_103_3) == (s_103_5));
        // D s_103_7: write-var gs#94050 <= s_103_6
        fn_state.gs_94050 = s_103_6;
        // N s_103_8: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #() : ()
        let s_104_0: () = ();
        // S s_104_1: call __get_selected_SPMACCESSR_EL3_field(s_104_0)
        let s_104_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_104_0);
        // S s_104_2: cast zx s_104_1 -> bv
        let s_104_2: Bits = Bits::new(s_104_1 as u128, 2u16);
        // C s_104_3: const #3u : u8
        let s_104_3: u8 = 3;
        // C s_104_4: cast zx s_104_3 -> bv
        let s_104_4: Bits = Bits::new(s_104_3 as u128, 2u16);
        // S s_104_5: cmp-ne s_104_2 s_104_4
        let s_104_5: bool = ((s_104_2) != (s_104_4));
        // D s_104_6: write-var gs#94046 <= s_104_5
        fn_state.gs_94046 = s_104_5;
        // N s_104_7: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #() : ()
        let s_105_0: () = ();
        // S s_105_1: call Halted(s_105_0)
        let s_105_1: bool = Halted(state, tracer, s_105_0);
        // N s_105_2: branch s_105_1 b110 b106
        if s_105_1 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #0u : u8
        let s_106_0: bool = false;
        // D s_106_1: write-var gs#94051 <= s_106_0
        fn_state.gs_94051 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#94051:u8
        let s_107_0: bool = fn_state.gs_94051;
        // N s_107_1: branch s_107_0 b109 b108
        if s_107_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #24u : u8
        let s_108_0: u8 = 24;
        // C s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 8u16);
        // C s_108_2: cast zx s_108_1 -> i
        let s_108_2: i128 = (s_108_1.value() as i128);
        // C s_108_3: cast reint s_108_2 -> i64
        let s_108_3: i64 = (s_108_2 as i64);
        // C s_108_4: cast zx s_108_3 -> i
        let s_108_4: i128 = (i128::try_from(s_108_3).unwrap());
        // C s_108_5: const #424u : u32
        let s_108_5: u32 = 424;
        // D s_108_6: read-reg s_108_5:u8
        let s_108_6: u8 = {
            let value = state.read_register::<u8>(s_108_5 as isize);
            tracer.read_register(s_108_5 as isize, value);
            value
        };
        // D s_108_7: call AArch64_SystemAccessTrap(s_108_6, s_108_4)
        let s_108_7: () = AArch64_SystemAccessTrap(state, tracer, s_108_6, s_108_4);
        // N s_108_8: return
        return;
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_109_0: panic
        panic!("{:?}", ());
        // N s_109_1: return
        return;
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #() : ()
        let s_110_0: () = ();
        // S s_110_1: call EDSCR_read(s_110_0)
        let s_110_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_110_0);
        // S s_110_2: call _get_EDSCR_Type_SDD(s_110_1)
        let s_110_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_110_1);
        // S s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // C s_110_4: const #1u : u8
        let s_110_4: bool = true;
        // C s_110_5: cast zx s_110_4 -> bv
        let s_110_5: Bits = Bits::new(s_110_4 as u128, 1u16);
        // S s_110_6: cmp-eq s_110_3 s_110_5
        let s_110_6: bool = ((s_110_3) == (s_110_5));
        // D s_110_7: write-var gs#94051 <= s_110_6
        fn_state.gs_94051 = s_110_6;
        // N s_110_8: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var __MDCR_EL3_EnPM2:u8
        let s_111_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 1u16);
        // C s_111_2: const #0u : u8
        let s_111_2: bool = false;
        // C s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 1u16);
        // D s_111_4: cmp-eq s_111_1 s_111_3
        let s_111_4: bool = ((s_111_1) == (s_111_3));
        // D s_111_5: write-var gs#94045 <= s_111_4
        fn_state.gs_94045 = s_111_4;
        // N s_111_6: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #24u : u8
        let s_112_0: u8 = 24;
        // C s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 8u16);
        // C s_112_2: cast zx s_112_1 -> i
        let s_112_2: i128 = (s_112_1.value() as i128);
        // C s_112_3: cast reint s_112_2 -> i64
        let s_112_3: i64 = (s_112_2 as i64);
        // C s_112_4: cast zx s_112_3 -> i
        let s_112_4: i128 = (i128::try_from(s_112_3).unwrap());
        // C s_112_5: const #432u : u32
        let s_112_5: u32 = 432;
        // D s_112_6: read-reg s_112_5:u8
        let s_112_6: u8 = {
            let value = state.read_register::<u8>(s_112_5 as isize);
            tracer.read_register(s_112_5 as isize, value);
            value
        };
        // D s_112_7: call AArch64_SystemAccessTrap(s_112_6, s_112_4)
        let s_112_7: () = AArch64_SystemAccessTrap(state, tracer, s_112_6, s_112_4);
        // N s_112_8: return
        return;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #() : ()
        let s_113_0: () = ();
        // S s_113_1: call __get_selected_SPMACCESSR_EL2_field(s_113_0)
        let s_113_1: u8 = u__get_selected_SPMACCESSR_EL2_field(state, tracer, s_113_0);
        // S s_113_2: cast zx s_113_1 -> bv
        let s_113_2: Bits = Bits::new(s_113_1 as u128, 2u16);
        // C s_113_3: const #3u : u8
        let s_113_3: u8 = 3;
        // C s_113_4: cast zx s_113_3 -> bv
        let s_113_4: Bits = Bits::new(s_113_3 as u128, 2u16);
        // S s_113_5: cmp-ne s_113_2 s_113_4
        let s_113_5: bool = ((s_113_2) != (s_113_4));
        // D s_113_6: write-var gs#94044 <= s_113_5
        fn_state.gs_94044 = s_113_5;
        // N s_113_7: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #24u : u8
        let s_114_0: u8 = 24;
        // C s_114_1: cast zx s_114_0 -> bv
        let s_114_1: Bits = Bits::new(s_114_0 as u128, 8u16);
        // C s_114_2: cast zx s_114_1 -> i
        let s_114_2: i128 = (s_114_1.value() as i128);
        // C s_114_3: cast reint s_114_2 -> i64
        let s_114_3: i64 = (s_114_2 as i64);
        // C s_114_4: cast zx s_114_3 -> i
        let s_114_4: i128 = (i128::try_from(s_114_3).unwrap());
        // C s_114_5: const #432u : u32
        let s_114_5: u32 = 432;
        // D s_114_6: read-reg s_114_5:u8
        let s_114_6: u8 = {
            let value = state.read_register::<u8>(s_114_5 as isize);
            tracer.read_register(s_114_5 as isize, value);
            value
        };
        // D s_114_7: call AArch64_SystemAccessTrap(s_114_6, s_114_4)
        let s_114_7: () = AArch64_SystemAccessTrap(state, tracer, s_114_6, s_114_4);
        // N s_114_8: return
        return;
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var __MDCR_EL2_EnSPM:u8
        let s_115_0: bool = fn_state.u__MDCR_EL2_EnSPM;
        // D s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 1u16);
        // C s_115_2: const #0u : u8
        let s_115_2: bool = false;
        // C s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 1u16);
        // D s_115_4: cmp-eq s_115_1 s_115_3
        let s_115_4: bool = ((s_115_1) == (s_115_3));
        // D s_115_5: write-var gs#94043 <= s_115_4
        fn_state.gs_94043 = s_115_4;
        // N s_115_6: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #24u : u8
        let s_116_0: u8 = 24;
        // C s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 8u16);
        // C s_116_2: cast zx s_116_1 -> i
        let s_116_2: i128 = (s_116_1.value() as i128);
        // C s_116_3: cast reint s_116_2 -> i64
        let s_116_3: i64 = (s_116_2 as i64);
        // C s_116_4: cast zx s_116_3 -> i
        let s_116_4: i128 = (i128::try_from(s_116_3).unwrap());
        // C s_116_5: const #432u : u32
        let s_116_5: u32 = 432;
        // D s_116_6: read-reg s_116_5:u8
        let s_116_6: u8 = {
            let value = state.read_register::<u8>(s_116_5 as isize);
            tracer.read_register(s_116_5 as isize, value);
            value
        };
        // D s_116_7: call AArch64_SystemAccessTrap(s_116_6, s_116_4)
        let s_116_7: () = AArch64_SystemAccessTrap(state, tracer, s_116_6, s_116_4);
        // N s_116_8: return
        return;
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var __HDFGWTR2_EL2_nSPMEVTYPERn_EL0:u8
        let s_117_0: bool = fn_state.u__HDFGWTR2_EL2_nSPMEVTYPERn_EL0;
        // D s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 1u16);
        // C s_117_2: const #0u : u8
        let s_117_2: bool = false;
        // C s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 1u16);
        // D s_117_4: cmp-eq s_117_1 s_117_3
        let s_117_4: bool = ((s_117_1) == (s_117_3));
        // D s_117_5: write-var gs#94042 <= s_117_4
        fn_state.gs_94042 = s_117_4;
        // N s_117_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #188u : u32
        let s_118_0: u32 = 188;
        // S s_118_1: call IsFeatureImplemented(s_118_0)
        let s_118_1: bool = IsFeatureImplemented(state, tracer, s_118_0);
        // D s_118_2: write-var gs#94041 <= s_118_1
        fn_state.gs_94041 = s_118_1;
        // N s_118_3: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #24u : u8
        let s_119_0: u8 = 24;
        // C s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 8u16);
        // C s_119_2: cast zx s_119_1 -> i
        let s_119_2: i128 = (s_119_1.value() as i128);
        // C s_119_3: cast reint s_119_2 -> i64
        let s_119_3: i64 = (s_119_2 as i64);
        // C s_119_4: cast zx s_119_3 -> i
        let s_119_4: i128 = (i128::try_from(s_119_3).unwrap());
        // C s_119_5: const #432u : u32
        let s_119_5: u32 = 432;
        // D s_119_6: read-reg s_119_5:u8
        let s_119_6: u8 = {
            let value = state.read_register::<u8>(s_119_5 as isize);
            tracer.read_register(s_119_5 as isize, value);
            value
        };
        // D s_119_7: call AArch64_SystemAccessTrap(s_119_6, s_119_4)
        let s_119_7: () = AArch64_SystemAccessTrap(state, tracer, s_119_6, s_119_4);
        // N s_119_8: return
        return;
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var __SCR_EL3_FGTEn2:u8
        let s_120_0: bool = fn_state.u__SCR_EL3_FGTEn2;
        // D s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 1u16);
        // C s_120_2: const #0u : u8
        let s_120_2: bool = false;
        // C s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 1u16);
        // D s_120_4: cmp-eq s_120_1 s_120_3
        let s_120_4: bool = ((s_120_1) == (s_120_3));
        // D s_120_5: write-var gs#94040 <= s_120_4
        fn_state.gs_94040 = s_120_4;
        // N s_120_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #424u : u32
        let s_121_0: u32 = 424;
        // D s_121_1: read-reg s_121_0:u8
        let s_121_1: u8 = {
            let value = state.read_register::<u8>(s_121_0 as isize);
            tracer.read_register(s_121_0 as isize, value);
            value
        };
        // C s_121_2: const #2u : u8
        let s_121_2: u8 = 2;
        // D s_121_3: cmp-lt s_121_1 s_121_2
        let s_121_3: bool = ((s_121_1) < (s_121_2));
        // D s_121_4: write-var gs#94039 <= s_121_3
        fn_state.gs_94039 = s_121_3;
        // N s_121_5: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #188u : u32
        let s_122_0: u32 = 188;
        // S s_122_1: call IsFeatureImplemented(s_122_0)
        let s_122_1: bool = IsFeatureImplemented(state, tracer, s_122_0);
        // D s_122_2: write-var gs#94038 <= s_122_1
        fn_state.gs_94038 = s_122_1;
        // N s_122_3: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_123_0: panic
        panic!("{:?}", ());
        // N s_123_1: return
        return;
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #() : ()
        let s_124_0: () = ();
        // S s_124_1: call __get_selected_SPMACCESSR_EL3_field(s_124_0)
        let s_124_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_124_0);
        // S s_124_2: cast zx s_124_1 -> bv
        let s_124_2: Bits = Bits::new(s_124_1 as u128, 2u16);
        // C s_124_3: const #3u : u8
        let s_124_3: u8 = 3;
        // C s_124_4: cast zx s_124_3 -> bv
        let s_124_4: Bits = Bits::new(s_124_3 as u128, 2u16);
        // S s_124_5: cmp-ne s_124_2 s_124_4
        let s_124_5: bool = ((s_124_2) != (s_124_4));
        // D s_124_6: write-var gs#94037 <= s_124_5
        fn_state.gs_94037 = s_124_5;
        // N s_124_7: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_125_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_125_1: call __IMPDEF_boolean(s_125_0)
        let s_125_1: bool = u__IMPDEF_boolean(state, tracer, s_125_0);
        // D s_125_2: write-var gs#94036 <= s_125_1
        fn_state.gs_94036 = s_125_1;
        // N s_125_3: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #() : ()
        let s_126_0: () = ();
        // S s_126_1: call EDSCR_read(s_126_0)
        let s_126_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_126_0);
        // S s_126_2: call _get_EDSCR_Type_SDD(s_126_1)
        let s_126_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_126_1);
        // S s_126_3: cast zx s_126_2 -> bv
        let s_126_3: Bits = Bits::new(s_126_2 as u128, 1u16);
        // C s_126_4: const #1u : u8
        let s_126_4: bool = true;
        // C s_126_5: cast zx s_126_4 -> bv
        let s_126_5: Bits = Bits::new(s_126_4 as u128, 1u16);
        // S s_126_6: cmp-eq s_126_3 s_126_5
        let s_126_6: bool = ((s_126_3) == (s_126_5));
        // D s_126_7: write-var gs#94035 <= s_126_6
        fn_state.gs_94035 = s_126_6;
        // N s_126_8: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #424u : u32
        let s_127_0: u32 = 424;
        // D s_127_1: read-reg s_127_0:u8
        let s_127_1: u8 = {
            let value = state.read_register::<u8>(s_127_0 as isize);
            tracer.read_register(s_127_0 as isize, value);
            value
        };
        // C s_127_2: const #2u : u8
        let s_127_2: u8 = 2;
        // D s_127_3: cmp-lt s_127_1 s_127_2
        let s_127_3: bool = ((s_127_1) < (s_127_2));
        // D s_127_4: write-var gs#94034 <= s_127_3
        fn_state.gs_94034 = s_127_3;
        // N s_127_5: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_128_0: panic
        panic!("{:?}", ());
        // N s_128_1: return
        return;
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var __MDCR_EL3_EnPM2:u8
        let s_129_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 1u16);
        // C s_129_2: const #0u : u8
        let s_129_2: bool = false;
        // C s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 1u16);
        // D s_129_4: cmp-eq s_129_1 s_129_3
        let s_129_4: bool = ((s_129_1) == (s_129_3));
        // D s_129_5: write-var gs#94033 <= s_129_4
        fn_state.gs_94033 = s_129_4;
        // N s_129_6: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_130_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_130_1: call __IMPDEF_boolean(s_130_0)
        let s_130_1: bool = u__IMPDEF_boolean(state, tracer, s_130_0);
        // D s_130_2: write-var gs#94032 <= s_130_1
        fn_state.gs_94032 = s_130_1;
        // N s_130_3: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #() : ()
        let s_131_0: () = ();
        // S s_131_1: call EDSCR_read(s_131_0)
        let s_131_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_131_0);
        // S s_131_2: call _get_EDSCR_Type_SDD(s_131_1)
        let s_131_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_131_1);
        // S s_131_3: cast zx s_131_2 -> bv
        let s_131_3: Bits = Bits::new(s_131_2 as u128, 1u16);
        // C s_131_4: const #1u : u8
        let s_131_4: bool = true;
        // C s_131_5: cast zx s_131_4 -> bv
        let s_131_5: Bits = Bits::new(s_131_4 as u128, 1u16);
        // S s_131_6: cmp-eq s_131_3 s_131_5
        let s_131_6: bool = ((s_131_3) == (s_131_5));
        // D s_131_7: write-var gs#94031 <= s_131_6
        fn_state.gs_94031 = s_131_6;
        // N s_131_8: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #424u : u32
        let s_132_0: u32 = 424;
        // D s_132_1: read-reg s_132_0:u8
        let s_132_1: u8 = {
            let value = state.read_register::<u8>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // C s_132_2: const #2u : u8
        let s_132_2: u8 = 2;
        // D s_132_3: cmp-lt s_132_1 s_132_2
        let s_132_3: bool = ((s_132_1) < (s_132_2));
        // D s_132_4: write-var gs#94030 <= s_132_3
        fn_state.gs_94030 = s_132_3;
        // N s_132_5: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #() : ()
        let s_133_0: () = ();
        // S s_133_1: call Halted(s_133_0)
        let s_133_1: bool = Halted(state, tracer, s_133_0);
        // N s_133_2: branch s_133_1 b236 b134
        if s_133_1 {
            return block_236(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #0u : u8
        let s_134_0: bool = false;
        // D s_134_1: write-var gs#94052 <= s_134_0
        fn_state.gs_94052 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#94052:u8
        let s_135_0: bool = fn_state.gs_94052;
        // N s_135_1: branch s_135_0 b235 b136
        if s_135_0 {
            return block_235(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var gs#94053 <= s_136_0
        fn_state.gs_94053 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#94053:u8
        let s_137_0: bool = fn_state.gs_94053;
        // N s_137_1: branch s_137_0 b234 b138
        if s_137_0 {
            return block_234(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #0u : u8
        let s_138_0: bool = false;
        // D s_138_1: write-var gs#94054 <= s_138_0
        fn_state.gs_94054 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#94054:u8
        let s_139_0: bool = fn_state.gs_94054;
        // N s_139_1: branch s_139_0 b233 b140
        if s_139_0 {
            return block_233(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #0u : u8
        let s_140_0: bool = false;
        // D s_140_1: write-var gs#94055 <= s_140_0
        fn_state.gs_94055 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#94055:u8
        let s_141_0: bool = fn_state.gs_94055;
        // N s_141_1: branch s_141_0 b232 b142
        if s_141_0 {
            return block_232(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #() : ()
        let s_142_0: () = ();
        // S s_142_1: call Halted(s_142_0)
        let s_142_1: bool = Halted(state, tracer, s_142_0);
        // N s_142_2: branch s_142_1 b231 b143
        if s_142_1 {
            return block_231(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #0u : u8
        let s_143_0: bool = false;
        // D s_143_1: write-var gs#94056 <= s_143_0
        fn_state.gs_94056 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#94056:u8
        let s_144_0: bool = fn_state.gs_94056;
        // N s_144_1: branch s_144_0 b230 b145
        if s_144_0 {
            return block_230(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #0u : u8
        let s_145_0: bool = false;
        // D s_145_1: write-var gs#94057 <= s_145_0
        fn_state.gs_94057 = s_145_0;
        // N s_145_2: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var gs#94057:u8
        let s_146_0: bool = fn_state.gs_94057;
        // N s_146_1: branch s_146_0 b229 b147
        if s_146_0 {
            return block_229(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #0u : u8
        let s_147_0: bool = false;
        // D s_147_1: write-var gs#94058 <= s_147_0
        fn_state.gs_94058 = s_147_0;
        // N s_147_2: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var gs#94058:u8
        let s_148_0: bool = fn_state.gs_94058;
        // N s_148_1: branch s_148_0 b228 b149
        if s_148_0 {
            return block_228(state, tracer, fn_state);
        } else {
            return block_149(state, tracer, fn_state);
        };
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #0u : u8
        let s_149_0: bool = false;
        // D s_149_1: write-var gs#94059 <= s_149_0
        fn_state.gs_94059 = s_149_0;
        // N s_149_2: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_150_0: read-var gs#94059:u8
        let s_150_0: bool = fn_state.gs_94059;
        // N s_150_1: branch s_150_0 b227 b151
        if s_150_0 {
            return block_227(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var __MDSCR_EL1_EnSPM:u8
        let s_151_0: bool = fn_state.u__MDSCR_EL1_EnSPM;
        // D s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 1u16);
        // C s_151_2: const #0u : u8
        let s_151_2: bool = false;
        // C s_151_3: cast zx s_151_2 -> bv
        let s_151_3: Bits = Bits::new(s_151_2 as u128, 1u16);
        // D s_151_4: cmp-eq s_151_1 s_151_3
        let s_151_4: bool = ((s_151_1) == (s_151_3));
        // N s_151_5: branch s_151_4 b221 b152
        if s_151_4 {
            return block_221(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #() : ()
        let s_152_0: () = ();
        // S s_152_1: call EL2Enabled(s_152_0)
        let s_152_1: bool = EL2Enabled(state, tracer, s_152_0);
        // N s_152_2: branch s_152_1 b220 b153
        if s_152_1 {
            return block_220(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #0u : u8
        let s_153_0: bool = false;
        // D s_153_1: write-var gs#94060 <= s_153_0
        fn_state.gs_94060 = s_153_0;
        // N s_153_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#94060:u8
        let s_154_0: bool = fn_state.gs_94060;
        // D s_154_1: not s_154_0
        let s_154_1: bool = !s_154_0;
        // N s_154_2: branch s_154_1 b219 b155
        if s_154_1 {
            return block_219(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #0u : u8
        let s_155_0: bool = false;
        // D s_155_1: write-var gs#94061 <= s_155_0
        fn_state.gs_94061 = s_155_0;
        // N s_155_2: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var gs#94061:u8
        let s_156_0: bool = fn_state.gs_94061;
        // N s_156_1: branch s_156_0 b213 b157
        if s_156_0 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #() : ()
        let s_157_0: () = ();
        // S s_157_1: call EL2Enabled(s_157_0)
        let s_157_1: bool = EL2Enabled(state, tracer, s_157_0);
        // N s_157_2: branch s_157_1 b212 b158
        if s_157_1 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #0u : u8
        let s_158_0: bool = false;
        // D s_158_1: write-var gs#94062 <= s_158_0
        fn_state.gs_94062 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#94062:u8
        let s_159_0: bool = fn_state.gs_94062;
        // N s_159_1: branch s_159_0 b211 b160
        if s_159_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #0u : u8
        let s_160_0: bool = false;
        // D s_160_1: write-var gs#94063 <= s_160_0
        fn_state.gs_94063 = s_160_0;
        // N s_160_2: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var gs#94063:u8
        let s_161_0: bool = fn_state.gs_94063;
        // N s_161_1: branch s_161_0 b210 b162
        if s_161_0 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_162(state, tracer, fn_state);
        };
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #0u : u8
        let s_162_0: bool = false;
        // D s_162_1: write-var gs#94064 <= s_162_0
        fn_state.gs_94064 = s_162_0;
        // N s_162_2: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_163_0: read-var gs#94064:u8
        let s_163_0: bool = fn_state.gs_94064;
        // N s_163_1: branch s_163_0 b209 b164
        if s_163_0 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #0u : u8
        let s_164_0: bool = false;
        // D s_164_1: write-var gs#94065 <= s_164_0
        fn_state.gs_94065 = s_164_0;
        // N s_164_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var gs#94065:u8
        let s_165_0: bool = fn_state.gs_94065;
        // N s_165_1: branch s_165_0 b208 b166
        if s_165_0 {
            return block_208(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #() : ()
        let s_166_0: () = ();
        // S s_166_1: call EL2Enabled(s_166_0)
        let s_166_1: bool = EL2Enabled(state, tracer, s_166_0);
        // N s_166_2: branch s_166_1 b207 b167
        if s_166_1 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #0u : u8
        let s_167_0: bool = false;
        // D s_167_1: write-var gs#94066 <= s_167_0
        fn_state.gs_94066 = s_167_0;
        // N s_167_2: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var gs#94066:u8
        let s_168_0: bool = fn_state.gs_94066;
        // N s_168_1: branch s_168_0 b206 b169
        if s_168_0 {
            return block_206(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #0u : u8
        let s_169_0: bool = false;
        // D s_169_1: write-var gs#94067 <= s_169_0
        fn_state.gs_94067 = s_169_0;
        // N s_169_2: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_170_0: read-var gs#94067:u8
        let s_170_0: bool = fn_state.gs_94067;
        // N s_170_1: branch s_170_0 b205 b171
        if s_170_0 {
            return block_205(state, tracer, fn_state);
        } else {
            return block_171(state, tracer, fn_state);
        };
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #0u : u8
        let s_171_0: bool = false;
        // D s_171_1: write-var gs#94068 <= s_171_0
        fn_state.gs_94068 = s_171_0;
        // N s_171_2: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_172_0: read-var gs#94068:u8
        let s_172_0: bool = fn_state.gs_94068;
        // N s_172_1: branch s_172_0 b204 b173
        if s_172_0 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #() : ()
        let s_173_0: () = ();
        // S s_173_1: call EL2Enabled(s_173_0)
        let s_173_1: bool = EL2Enabled(state, tracer, s_173_0);
        // N s_173_2: branch s_173_1 b203 b174
        if s_173_1 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_174(state, tracer, fn_state);
        };
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #0u : u8
        let s_174_0: bool = false;
        // D s_174_1: write-var gs#94069 <= s_174_0
        fn_state.gs_94069 = s_174_0;
        // N s_174_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var gs#94069:u8
        let s_175_0: bool = fn_state.gs_94069;
        // N s_175_1: branch s_175_0 b202 b176
        if s_175_0 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_176(state, tracer, fn_state);
        };
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #() : ()
        let s_176_0: () = ();
        // S s_176_1: call EL2Enabled(s_176_0)
        let s_176_1: bool = EL2Enabled(state, tracer, s_176_0);
        // N s_176_2: branch s_176_1 b201 b177
        if s_176_1 {
            return block_201(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #0u : u8
        let s_177_0: bool = false;
        // D s_177_1: write-var gs#94070 <= s_177_0
        fn_state.gs_94070 = s_177_0;
        // N s_177_2: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var gs#94070:u8
        let s_178_0: bool = fn_state.gs_94070;
        // N s_178_1: branch s_178_0 b200 b179
        if s_178_0 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_179(state, tracer, fn_state);
        };
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #424u : u32
        let s_179_0: u32 = 424;
        // D s_179_1: read-reg s_179_0:u8
        let s_179_1: u8 = {
            let value = state.read_register::<u8>(s_179_0 as isize);
            tracer.read_register(s_179_0 as isize, value);
            value
        };
        // C s_179_2: const #2u : u8
        let s_179_2: u8 = 2;
        // D s_179_3: cmp-lt s_179_1 s_179_2
        let s_179_3: bool = ((s_179_1) < (s_179_2));
        // N s_179_4: branch s_179_3 b199 b180
        if s_179_3 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_180(state, tracer, fn_state);
        };
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #0u : u8
        let s_180_0: bool = false;
        // D s_180_1: write-var gs#94071 <= s_180_0
        fn_state.gs_94071 = s_180_0;
        // N s_180_2: jump b181
        return block_181(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_181_0: read-var gs#94071:u8
        let s_181_0: bool = fn_state.gs_94071;
        // N s_181_1: branch s_181_0 b193 b182
        if s_181_0 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_182(state, tracer, fn_state);
        };
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #424u : u32
        let s_182_0: u32 = 424;
        // D s_182_1: read-reg s_182_0:u8
        let s_182_1: u8 = {
            let value = state.read_register::<u8>(s_182_0 as isize);
            tracer.read_register(s_182_0 as isize, value);
            value
        };
        // C s_182_2: const #2u : u8
        let s_182_2: u8 = 2;
        // D s_182_3: cmp-lt s_182_1 s_182_2
        let s_182_3: bool = ((s_182_1) < (s_182_2));
        // N s_182_4: branch s_182_3 b192 b183
        if s_182_3 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #0u : u8
        let s_183_0: bool = false;
        // D s_183_1: write-var gs#94072 <= s_183_0
        fn_state.gs_94072 = s_183_0;
        // N s_183_2: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var gs#94072:u8
        let s_184_0: bool = fn_state.gs_94072;
        // N s_184_1: branch s_184_0 b186 b185
        if s_184_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_185(state, tracer, fn_state);
        };
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #16552u : u32
        let s_185_0: u32 = 16552;
        // D s_185_1: read-reg s_185_0:struct
        let s_185_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_185_0 as isize);
            tracer.read_register(s_185_0 as isize, value);
            value
        };
        // D s_185_2: call _get_SPMSELR_EL0_Type_SYSPMUSEL(s_185_1)
        let s_185_2: u8 = u_get_SPMSELR_EL0_Type_SYSPMUSEL(state, tracer, s_185_1);
        // D s_185_3: cast zx s_185_2 -> bv
        let s_185_3: Bits = Bits::new(s_185_2 as u128, 6u16);
        // D s_185_4: cast zx s_185_3 -> i
        let s_185_4: i128 = (s_185_3.value() as i128);
        // D s_185_5: cast reint s_185_4 -> i64
        let s_185_5: i64 = (s_185_4 as i64);
        // C s_185_6: const #16552u : u32
        let s_185_6: u32 = 16552;
        // D s_185_7: read-reg s_185_6:struct
        let s_185_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_185_6 as isize);
            tracer.read_register(s_185_6 as isize, value);
            value
        };
        // D s_185_8: call _get_SPMSELR_EL0_Type_BANK(s_185_7)
        let s_185_8: u8 = u_get_SPMSELR_EL0_Type_BANK(state, tracer, s_185_7);
        // D s_185_9: cast zx s_185_8 -> bv
        let s_185_9: Bits = Bits::new(s_185_8 as u128, 2u16);
        // D s_185_10: cast zx s_185_9 -> i
        let s_185_10: i128 = (s_185_9.value() as i128);
        // D s_185_11: cast reint s_185_10 -> i64
        let s_185_11: i64 = (s_185_10 as i64);
        // C s_185_12: const #16s : i
        let s_185_12: i128 = 16;
        // D s_185_13: cast zx s_185_11 -> i
        let s_185_13: i128 = (i128::try_from(s_185_11).unwrap());
        // D s_185_14: mul s_185_13 s_185_12
        let s_185_14: i128 = ((s_185_13) * (s_185_12));
        // D s_185_15: cast reint s_185_14 -> i64
        let s_185_15: i64 = (s_185_14 as i64);
        // C s_185_16: const #14s : i
        let s_185_16: i128 = 14;
        // D s_185_17: cast zx s_185_15 -> i
        let s_185_17: i128 = (i128::try_from(s_185_15).unwrap());
        // D s_185_18: add s_185_17 s_185_16
        let s_185_18: i128 = (s_185_17 + s_185_16);
        // D s_185_19: cast reint s_185_18 -> i64
        let s_185_19: i64 = (s_185_18 as i64);
        // C s_185_20: const #64s : i64
        let s_185_20: i64 = 64;
        // D s_185_21: read-var t:i
        let s_185_21: i128 = fn_state.t;
        // D s_185_22: call X_read(s_185_21, s_185_20)
        let s_185_22: Bits = X_read(state, tracer, s_185_21, s_185_20);
        // D s_185_23: cast reint s_185_22 -> u64
        let s_185_23: u64 = (s_185_22.value() as u64);
        // D s_185_24: cast zx s_185_5 -> i
        let s_185_24: i128 = (i128::try_from(s_185_5).unwrap());
        // D s_185_25: cast zx s_185_19 -> i
        let s_185_25: i128 = (i128::try_from(s_185_19).unwrap());
        // D s_185_26: call SPMEVFILT2R_EL0_set(s_185_24, s_185_25, s_185_23)
        let s_185_26: () = SPMEVFILT2R_EL0_set(
            state,
            tracer,
            s_185_24,
            s_185_25,
            s_185_23,
        );
        // N s_185_27: return
        return;
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #() : ()
        let s_186_0: () = ();
        // S s_186_1: call Halted(s_186_0)
        let s_186_1: bool = Halted(state, tracer, s_186_0);
        // N s_186_2: branch s_186_1 b191 b187
        if s_186_1 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #0u : u8
        let s_187_0: bool = false;
        // D s_187_1: write-var gs#94076 <= s_187_0
        fn_state.gs_94076 = s_187_0;
        // N s_187_2: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var gs#94076:u8
        let s_188_0: bool = fn_state.gs_94076;
        // N s_188_1: branch s_188_0 b190 b189
        if s_188_0 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_189(state, tracer, fn_state);
        };
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #24u : u8
        let s_189_0: u8 = 24;
        // C s_189_1: cast zx s_189_0 -> bv
        let s_189_1: Bits = Bits::new(s_189_0 as u128, 8u16);
        // C s_189_2: cast zx s_189_1 -> i
        let s_189_2: i128 = (s_189_1.value() as i128);
        // C s_189_3: cast reint s_189_2 -> i64
        let s_189_3: i64 = (s_189_2 as i64);
        // C s_189_4: cast zx s_189_3 -> i
        let s_189_4: i128 = (i128::try_from(s_189_3).unwrap());
        // C s_189_5: const #424u : u32
        let s_189_5: u32 = 424;
        // D s_189_6: read-reg s_189_5:u8
        let s_189_6: u8 = {
            let value = state.read_register::<u8>(s_189_5 as isize);
            tracer.read_register(s_189_5 as isize, value);
            value
        };
        // D s_189_7: call AArch64_SystemAccessTrap(s_189_6, s_189_4)
        let s_189_7: () = AArch64_SystemAccessTrap(state, tracer, s_189_6, s_189_4);
        // N s_189_8: return
        return;
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_190_0: panic
        panic!("{:?}", ());
        // N s_190_1: return
        return;
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #() : ()
        let s_191_0: () = ();
        // S s_191_1: call EDSCR_read(s_191_0)
        let s_191_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_191_0);
        // S s_191_2: call _get_EDSCR_Type_SDD(s_191_1)
        let s_191_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_191_1);
        // S s_191_3: cast zx s_191_2 -> bv
        let s_191_3: Bits = Bits::new(s_191_2 as u128, 1u16);
        // C s_191_4: const #1u : u8
        let s_191_4: bool = true;
        // C s_191_5: cast zx s_191_4 -> bv
        let s_191_5: Bits = Bits::new(s_191_4 as u128, 1u16);
        // S s_191_6: cmp-eq s_191_3 s_191_5
        let s_191_6: bool = ((s_191_3) == (s_191_5));
        // D s_191_7: write-var gs#94076 <= s_191_6
        fn_state.gs_94076 = s_191_6;
        // N s_191_8: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #() : ()
        let s_192_0: () = ();
        // S s_192_1: call __get_selected_SPMACCESSR_EL3_field(s_192_0)
        let s_192_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_192_0);
        // S s_192_2: cast zx s_192_1 -> bv
        let s_192_2: Bits = Bits::new(s_192_1 as u128, 2u16);
        // C s_192_3: const #3u : u8
        let s_192_3: u8 = 3;
        // C s_192_4: cast zx s_192_3 -> bv
        let s_192_4: Bits = Bits::new(s_192_3 as u128, 2u16);
        // S s_192_5: cmp-ne s_192_2 s_192_4
        let s_192_5: bool = ((s_192_2) != (s_192_4));
        // D s_192_6: write-var gs#94072 <= s_192_5
        fn_state.gs_94072 = s_192_5;
        // N s_192_7: jump b184
        return block_184(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #() : ()
        let s_193_0: () = ();
        // S s_193_1: call Halted(s_193_0)
        let s_193_1: bool = Halted(state, tracer, s_193_0);
        // N s_193_2: branch s_193_1 b198 b194
        if s_193_1 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_194(state, tracer, fn_state);
        };
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #0u : u8
        let s_194_0: bool = false;
        // D s_194_1: write-var gs#94077 <= s_194_0
        fn_state.gs_94077 = s_194_0;
        // N s_194_2: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_195_0: read-var gs#94077:u8
        let s_195_0: bool = fn_state.gs_94077;
        // N s_195_1: branch s_195_0 b197 b196
        if s_195_0 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_196(state, tracer, fn_state);
        };
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #24u : u8
        let s_196_0: u8 = 24;
        // C s_196_1: cast zx s_196_0 -> bv
        let s_196_1: Bits = Bits::new(s_196_0 as u128, 8u16);
        // C s_196_2: cast zx s_196_1 -> i
        let s_196_2: i128 = (s_196_1.value() as i128);
        // C s_196_3: cast reint s_196_2 -> i64
        let s_196_3: i64 = (s_196_2 as i64);
        // C s_196_4: cast zx s_196_3 -> i
        let s_196_4: i128 = (i128::try_from(s_196_3).unwrap());
        // C s_196_5: const #424u : u32
        let s_196_5: u32 = 424;
        // D s_196_6: read-reg s_196_5:u8
        let s_196_6: u8 = {
            let value = state.read_register::<u8>(s_196_5 as isize);
            tracer.read_register(s_196_5 as isize, value);
            value
        };
        // D s_196_7: call AArch64_SystemAccessTrap(s_196_6, s_196_4)
        let s_196_7: () = AArch64_SystemAccessTrap(state, tracer, s_196_6, s_196_4);
        // N s_196_8: return
        return;
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_197_0: panic
        panic!("{:?}", ());
        // N s_197_1: return
        return;
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #() : ()
        let s_198_0: () = ();
        // S s_198_1: call EDSCR_read(s_198_0)
        let s_198_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_198_0);
        // S s_198_2: call _get_EDSCR_Type_SDD(s_198_1)
        let s_198_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_198_1);
        // S s_198_3: cast zx s_198_2 -> bv
        let s_198_3: Bits = Bits::new(s_198_2 as u128, 1u16);
        // C s_198_4: const #1u : u8
        let s_198_4: bool = true;
        // C s_198_5: cast zx s_198_4 -> bv
        let s_198_5: Bits = Bits::new(s_198_4 as u128, 1u16);
        // S s_198_6: cmp-eq s_198_3 s_198_5
        let s_198_6: bool = ((s_198_3) == (s_198_5));
        // D s_198_7: write-var gs#94077 <= s_198_6
        fn_state.gs_94077 = s_198_6;
        // N s_198_8: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_199_0: read-var __MDCR_EL3_EnPM2:u8
        let s_199_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_199_1: cast zx s_199_0 -> bv
        let s_199_1: Bits = Bits::new(s_199_0 as u128, 1u16);
        // C s_199_2: const #0u : u8
        let s_199_2: bool = false;
        // C s_199_3: cast zx s_199_2 -> bv
        let s_199_3: Bits = Bits::new(s_199_2 as u128, 1u16);
        // D s_199_4: cmp-eq s_199_1 s_199_3
        let s_199_4: bool = ((s_199_1) == (s_199_3));
        // D s_199_5: write-var gs#94071 <= s_199_4
        fn_state.gs_94071 = s_199_4;
        // N s_199_6: jump b181
        return block_181(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #24u : u8
        let s_200_0: u8 = 24;
        // C s_200_1: cast zx s_200_0 -> bv
        let s_200_1: Bits = Bits::new(s_200_0 as u128, 8u16);
        // C s_200_2: cast zx s_200_1 -> i
        let s_200_2: i128 = (s_200_1.value() as i128);
        // C s_200_3: cast reint s_200_2 -> i64
        let s_200_3: i64 = (s_200_2 as i64);
        // C s_200_4: cast zx s_200_3 -> i
        let s_200_4: i128 = (i128::try_from(s_200_3).unwrap());
        // C s_200_5: const #432u : u32
        let s_200_5: u32 = 432;
        // D s_200_6: read-reg s_200_5:u8
        let s_200_6: u8 = {
            let value = state.read_register::<u8>(s_200_5 as isize);
            tracer.read_register(s_200_5 as isize, value);
            value
        };
        // D s_200_7: call AArch64_SystemAccessTrap(s_200_6, s_200_4)
        let s_200_7: () = AArch64_SystemAccessTrap(state, tracer, s_200_6, s_200_4);
        // N s_200_8: return
        return;
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #() : ()
        let s_201_0: () = ();
        // S s_201_1: call __get_selected_SPMACCESSR_EL2_field(s_201_0)
        let s_201_1: u8 = u__get_selected_SPMACCESSR_EL2_field(state, tracer, s_201_0);
        // S s_201_2: cast zx s_201_1 -> bv
        let s_201_2: Bits = Bits::new(s_201_1 as u128, 2u16);
        // C s_201_3: const #3u : u8
        let s_201_3: u8 = 3;
        // C s_201_4: cast zx s_201_3 -> bv
        let s_201_4: Bits = Bits::new(s_201_3 as u128, 2u16);
        // S s_201_5: cmp-ne s_201_2 s_201_4
        let s_201_5: bool = ((s_201_2) != (s_201_4));
        // D s_201_6: write-var gs#94070 <= s_201_5
        fn_state.gs_94070 = s_201_5;
        // N s_201_7: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_202_0: const #24u : u8
        let s_202_0: u8 = 24;
        // C s_202_1: cast zx s_202_0 -> bv
        let s_202_1: Bits = Bits::new(s_202_0 as u128, 8u16);
        // C s_202_2: cast zx s_202_1 -> i
        let s_202_2: i128 = (s_202_1.value() as i128);
        // C s_202_3: cast reint s_202_2 -> i64
        let s_202_3: i64 = (s_202_2 as i64);
        // C s_202_4: cast zx s_202_3 -> i
        let s_202_4: i128 = (i128::try_from(s_202_3).unwrap());
        // C s_202_5: const #432u : u32
        let s_202_5: u32 = 432;
        // D s_202_6: read-reg s_202_5:u8
        let s_202_6: u8 = {
            let value = state.read_register::<u8>(s_202_5 as isize);
            tracer.read_register(s_202_5 as isize, value);
            value
        };
        // D s_202_7: call AArch64_SystemAccessTrap(s_202_6, s_202_4)
        let s_202_7: () = AArch64_SystemAccessTrap(state, tracer, s_202_6, s_202_4);
        // N s_202_8: return
        return;
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_203_0: read-var __MDCR_EL2_EnSPM:u8
        let s_203_0: bool = fn_state.u__MDCR_EL2_EnSPM;
        // D s_203_1: cast zx s_203_0 -> bv
        let s_203_1: Bits = Bits::new(s_203_0 as u128, 1u16);
        // C s_203_2: const #0u : u8
        let s_203_2: bool = false;
        // C s_203_3: cast zx s_203_2 -> bv
        let s_203_3: Bits = Bits::new(s_203_2 as u128, 1u16);
        // D s_203_4: cmp-eq s_203_1 s_203_3
        let s_203_4: bool = ((s_203_1) == (s_203_3));
        // D s_203_5: write-var gs#94069 <= s_203_4
        fn_state.gs_94069 = s_203_4;
        // N s_203_6: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #24u : u8
        let s_204_0: u8 = 24;
        // C s_204_1: cast zx s_204_0 -> bv
        let s_204_1: Bits = Bits::new(s_204_0 as u128, 8u16);
        // C s_204_2: cast zx s_204_1 -> i
        let s_204_2: i128 = (s_204_1.value() as i128);
        // C s_204_3: cast reint s_204_2 -> i64
        let s_204_3: i64 = (s_204_2 as i64);
        // C s_204_4: cast zx s_204_3 -> i
        let s_204_4: i128 = (i128::try_from(s_204_3).unwrap());
        // C s_204_5: const #432u : u32
        let s_204_5: u32 = 432;
        // D s_204_6: read-reg s_204_5:u8
        let s_204_6: u8 = {
            let value = state.read_register::<u8>(s_204_5 as isize);
            tracer.read_register(s_204_5 as isize, value);
            value
        };
        // D s_204_7: call AArch64_SystemAccessTrap(s_204_6, s_204_4)
        let s_204_7: () = AArch64_SystemAccessTrap(state, tracer, s_204_6, s_204_4);
        // N s_204_8: return
        return;
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_205_0: read-var __HDFGWTR2_EL2_nSPMEVTYPERn_EL0:u8
        let s_205_0: bool = fn_state.u__HDFGWTR2_EL2_nSPMEVTYPERn_EL0;
        // D s_205_1: cast zx s_205_0 -> bv
        let s_205_1: Bits = Bits::new(s_205_0 as u128, 1u16);
        // C s_205_2: const #0u : u8
        let s_205_2: bool = false;
        // C s_205_3: cast zx s_205_2 -> bv
        let s_205_3: Bits = Bits::new(s_205_2 as u128, 1u16);
        // D s_205_4: cmp-eq s_205_1 s_205_3
        let s_205_4: bool = ((s_205_1) == (s_205_3));
        // D s_205_5: write-var gs#94068 <= s_205_4
        fn_state.gs_94068 = s_205_4;
        // N s_205_6: jump b172
        return block_172(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_206_0: const #188u : u32
        let s_206_0: u32 = 188;
        // S s_206_1: call IsFeatureImplemented(s_206_0)
        let s_206_1: bool = IsFeatureImplemented(state, tracer, s_206_0);
        // D s_206_2: write-var gs#94067 <= s_206_1
        fn_state.gs_94067 = s_206_1;
        // N s_206_3: jump b170
        return block_170(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #102552u : u32
        let s_207_0: u32 = 102552;
        // D s_207_1: read-reg s_207_0:struct
        let s_207_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_207_0 as isize);
            tracer.read_register(s_207_0 as isize, value);
            value
        };
        // D s_207_2: call _get_HCR_EL2_Type_E2H(s_207_1)
        let s_207_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_207_1);
        // C s_207_3: const #102552u : u32
        let s_207_3: u32 = 102552;
        // D s_207_4: read-reg s_207_3:struct
        let s_207_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_207_3 as isize);
            tracer.read_register(s_207_3 as isize, value);
            value
        };
        // D s_207_5: call _get_HCR_EL2_Type_TGE(s_207_4)
        let s_207_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_207_4);
        // D s_207_6: cast zx s_207_2 -> bv
        let s_207_6: Bits = Bits::new(s_207_2 as u128, 1u16);
        // D s_207_7: cast zx s_207_5 -> bv
        let s_207_7: Bits = Bits::new(s_207_5 as u128, 1u16);
        // D s_207_8: cast reint s_207_6 -> u128
        let s_207_8: u128 = (s_207_6.value() as u128);
        // D s_207_9: size-of s_207_6
        let s_207_9: u16 = s_207_6.length();
        // D s_207_10: cast reint s_207_7 -> u128
        let s_207_10: u128 = (s_207_7.value() as u128);
        // D s_207_11: size-of s_207_7
        let s_207_11: u16 = s_207_7.length();
        // D s_207_12: lsl s_207_8 s_207_11
        let s_207_12: u128 = s_207_8 << s_207_11;
        // D s_207_13: or s_207_12 s_207_10
        let s_207_13: u128 = ((s_207_12) | (s_207_10));
        // D s_207_14: add s_207_9 s_207_11
        let s_207_14: u16 = (s_207_9 + s_207_11);
        // D s_207_15: create-bits s_207_13 s_207_14
        let s_207_15: Bits = Bits::new(s_207_13, s_207_14);
        // D s_207_16: cast reint s_207_15 -> u8
        let s_207_16: u8 = (s_207_15.value() as u8);
        // D s_207_17: cast zx s_207_16 -> bv
        let s_207_17: Bits = Bits::new(s_207_16 as u128, 2u16);
        // C s_207_18: const #3u : u8
        let s_207_18: u8 = 3;
        // C s_207_19: cast zx s_207_18 -> bv
        let s_207_19: Bits = Bits::new(s_207_18 as u128, 2u16);
        // D s_207_20: cmp-ne s_207_17 s_207_19
        let s_207_20: bool = ((s_207_17) != (s_207_19));
        // D s_207_21: write-var gs#94066 <= s_207_20
        fn_state.gs_94066 = s_207_20;
        // N s_207_22: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_208_0: const #24u : u8
        let s_208_0: u8 = 24;
        // C s_208_1: cast zx s_208_0 -> bv
        let s_208_1: Bits = Bits::new(s_208_0 as u128, 8u16);
        // C s_208_2: cast zx s_208_1 -> i
        let s_208_2: i128 = (s_208_1.value() as i128);
        // C s_208_3: cast reint s_208_2 -> i64
        let s_208_3: i64 = (s_208_2 as i64);
        // C s_208_4: cast zx s_208_3 -> i
        let s_208_4: i128 = (i128::try_from(s_208_3).unwrap());
        // C s_208_5: const #432u : u32
        let s_208_5: u32 = 432;
        // D s_208_6: read-reg s_208_5:u8
        let s_208_6: u8 = {
            let value = state.read_register::<u8>(s_208_5 as isize);
            tracer.read_register(s_208_5 as isize, value);
            value
        };
        // D s_208_7: call AArch64_SystemAccessTrap(s_208_6, s_208_4)
        let s_208_7: () = AArch64_SystemAccessTrap(state, tracer, s_208_6, s_208_4);
        // N s_208_8: return
        return;
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_209_0: read-var __SCR_EL3_FGTEn2:u8
        let s_209_0: bool = fn_state.u__SCR_EL3_FGTEn2;
        // D s_209_1: cast zx s_209_0 -> bv
        let s_209_1: Bits = Bits::new(s_209_0 as u128, 1u16);
        // C s_209_2: const #0u : u8
        let s_209_2: bool = false;
        // C s_209_3: cast zx s_209_2 -> bv
        let s_209_3: Bits = Bits::new(s_209_2 as u128, 1u16);
        // D s_209_4: cmp-eq s_209_1 s_209_3
        let s_209_4: bool = ((s_209_1) == (s_209_3));
        // D s_209_5: write-var gs#94065 <= s_209_4
        fn_state.gs_94065 = s_209_4;
        // N s_209_6: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #424u : u32
        let s_210_0: u32 = 424;
        // D s_210_1: read-reg s_210_0:u8
        let s_210_1: u8 = {
            let value = state.read_register::<u8>(s_210_0 as isize);
            tracer.read_register(s_210_0 as isize, value);
            value
        };
        // C s_210_2: const #2u : u8
        let s_210_2: u8 = 2;
        // D s_210_3: cmp-lt s_210_1 s_210_2
        let s_210_3: bool = ((s_210_1) < (s_210_2));
        // D s_210_4: write-var gs#94064 <= s_210_3
        fn_state.gs_94064 = s_210_3;
        // N s_210_5: jump b163
        return block_163(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #188u : u32
        let s_211_0: u32 = 188;
        // S s_211_1: call IsFeatureImplemented(s_211_0)
        let s_211_1: bool = IsFeatureImplemented(state, tracer, s_211_0);
        // D s_211_2: write-var gs#94063 <= s_211_1
        fn_state.gs_94063 = s_211_1;
        // N s_211_3: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_212_0: const #102552u : u32
        let s_212_0: u32 = 102552;
        // D s_212_1: read-reg s_212_0:struct
        let s_212_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_212_0 as isize);
            tracer.read_register(s_212_0 as isize, value);
            value
        };
        // D s_212_2: call _get_HCR_EL2_Type_E2H(s_212_1)
        let s_212_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_212_1);
        // C s_212_3: const #102552u : u32
        let s_212_3: u32 = 102552;
        // D s_212_4: read-reg s_212_3:struct
        let s_212_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_212_3 as isize);
            tracer.read_register(s_212_3 as isize, value);
            value
        };
        // D s_212_5: call _get_HCR_EL2_Type_TGE(s_212_4)
        let s_212_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_212_4);
        // D s_212_6: cast zx s_212_2 -> bv
        let s_212_6: Bits = Bits::new(s_212_2 as u128, 1u16);
        // D s_212_7: cast zx s_212_5 -> bv
        let s_212_7: Bits = Bits::new(s_212_5 as u128, 1u16);
        // D s_212_8: cast reint s_212_6 -> u128
        let s_212_8: u128 = (s_212_6.value() as u128);
        // D s_212_9: size-of s_212_6
        let s_212_9: u16 = s_212_6.length();
        // D s_212_10: cast reint s_212_7 -> u128
        let s_212_10: u128 = (s_212_7.value() as u128);
        // D s_212_11: size-of s_212_7
        let s_212_11: u16 = s_212_7.length();
        // D s_212_12: lsl s_212_8 s_212_11
        let s_212_12: u128 = s_212_8 << s_212_11;
        // D s_212_13: or s_212_12 s_212_10
        let s_212_13: u128 = ((s_212_12) | (s_212_10));
        // D s_212_14: add s_212_9 s_212_11
        let s_212_14: u16 = (s_212_9 + s_212_11);
        // D s_212_15: create-bits s_212_13 s_212_14
        let s_212_15: Bits = Bits::new(s_212_13, s_212_14);
        // D s_212_16: cast reint s_212_15 -> u8
        let s_212_16: u8 = (s_212_15.value() as u8);
        // D s_212_17: cast zx s_212_16 -> bv
        let s_212_17: Bits = Bits::new(s_212_16 as u128, 2u16);
        // C s_212_18: const #3u : u8
        let s_212_18: u8 = 3;
        // C s_212_19: cast zx s_212_18 -> bv
        let s_212_19: Bits = Bits::new(s_212_18 as u128, 2u16);
        // D s_212_20: cmp-ne s_212_17 s_212_19
        let s_212_20: bool = ((s_212_17) != (s_212_19));
        // D s_212_21: write-var gs#94062 <= s_212_20
        fn_state.gs_94062 = s_212_20;
        // N s_212_22: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #() : ()
        let s_213_0: () = ();
        // S s_213_1: call EL2Enabled(s_213_0)
        let s_213_1: bool = EL2Enabled(state, tracer, s_213_0);
        // N s_213_2: branch s_213_1 b218 b214
        if s_213_1 {
            return block_218(state, tracer, fn_state);
        } else {
            return block_214(state, tracer, fn_state);
        };
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #0u : u8
        let s_214_0: bool = false;
        // D s_214_1: write-var gs#94078 <= s_214_0
        fn_state.gs_94078 = s_214_0;
        // N s_214_2: jump b215
        return block_215(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_215_0: read-var gs#94078:u8
        let s_215_0: bool = fn_state.gs_94078;
        // N s_215_1: branch s_215_0 b217 b216
        if s_215_0 {
            return block_217(state, tracer, fn_state);
        } else {
            return block_216(state, tracer, fn_state);
        };
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #24u : u8
        let s_216_0: u8 = 24;
        // C s_216_1: cast zx s_216_0 -> bv
        let s_216_1: Bits = Bits::new(s_216_0 as u128, 8u16);
        // C s_216_2: cast zx s_216_1 -> i
        let s_216_2: i128 = (s_216_1.value() as i128);
        // C s_216_3: cast reint s_216_2 -> i64
        let s_216_3: i64 = (s_216_2 as i64);
        // C s_216_4: cast zx s_216_3 -> i
        let s_216_4: i128 = (i128::try_from(s_216_3).unwrap());
        // C s_216_5: const #440u : u32
        let s_216_5: u32 = 440;
        // D s_216_6: read-reg s_216_5:u8
        let s_216_6: u8 = {
            let value = state.read_register::<u8>(s_216_5 as isize);
            tracer.read_register(s_216_5 as isize, value);
            value
        };
        // D s_216_7: call AArch64_SystemAccessTrap(s_216_6, s_216_4)
        let s_216_7: () = AArch64_SystemAccessTrap(state, tracer, s_216_6, s_216_4);
        // N s_216_8: return
        return;
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #24u : u8
        let s_217_0: u8 = 24;
        // C s_217_1: cast zx s_217_0 -> bv
        let s_217_1: Bits = Bits::new(s_217_0 as u128, 8u16);
        // C s_217_2: cast zx s_217_1 -> i
        let s_217_2: i128 = (s_217_1.value() as i128);
        // C s_217_3: cast reint s_217_2 -> i64
        let s_217_3: i64 = (s_217_2 as i64);
        // C s_217_4: cast zx s_217_3 -> i
        let s_217_4: i128 = (i128::try_from(s_217_3).unwrap());
        // C s_217_5: const #432u : u32
        let s_217_5: u32 = 432;
        // D s_217_6: read-reg s_217_5:u8
        let s_217_6: u8 = {
            let value = state.read_register::<u8>(s_217_5 as isize);
            tracer.read_register(s_217_5 as isize, value);
            value
        };
        // D s_217_7: call AArch64_SystemAccessTrap(s_217_6, s_217_4)
        let s_217_7: () = AArch64_SystemAccessTrap(state, tracer, s_217_6, s_217_4);
        // N s_217_8: return
        return;
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_218_0: read-var __HCR_EL2_TGE:u8
        let s_218_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_218_1: cast zx s_218_0 -> bv
        let s_218_1: Bits = Bits::new(s_218_0 as u128, 1u16);
        // C s_218_2: const #1u : u8
        let s_218_2: bool = true;
        // C s_218_3: cast zx s_218_2 -> bv
        let s_218_3: Bits = Bits::new(s_218_2 as u128, 1u16);
        // D s_218_4: cmp-eq s_218_1 s_218_3
        let s_218_4: bool = ((s_218_1) == (s_218_3));
        // D s_218_5: write-var gs#94078 <= s_218_4
        fn_state.gs_94078 = s_218_4;
        // N s_218_6: jump b215
        return block_215(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #() : ()
        let s_219_0: () = ();
        // S s_219_1: call __get_selected_SPMACCESSR_EL1_field(s_219_0)
        let s_219_1: u8 = u__get_selected_SPMACCESSR_EL1_field(state, tracer, s_219_0);
        // S s_219_2: cast zx s_219_1 -> bv
        let s_219_2: Bits = Bits::new(s_219_1 as u128, 2u16);
        // C s_219_3: const #3u : u8
        let s_219_3: u8 = 3;
        // C s_219_4: cast zx s_219_3 -> bv
        let s_219_4: Bits = Bits::new(s_219_3 as u128, 2u16);
        // S s_219_5: cmp-ne s_219_2 s_219_4
        let s_219_5: bool = ((s_219_2) != (s_219_4));
        // D s_219_6: write-var gs#94061 <= s_219_5
        fn_state.gs_94061 = s_219_5;
        // N s_219_7: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #102552u : u32
        let s_220_0: u32 = 102552;
        // D s_220_1: read-reg s_220_0:struct
        let s_220_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_220_0 as isize);
            tracer.read_register(s_220_0 as isize, value);
            value
        };
        // D s_220_2: call _get_HCR_EL2_Type_E2H(s_220_1)
        let s_220_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_220_1);
        // C s_220_3: const #102552u : u32
        let s_220_3: u32 = 102552;
        // D s_220_4: read-reg s_220_3:struct
        let s_220_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_220_3 as isize);
            tracer.read_register(s_220_3 as isize, value);
            value
        };
        // D s_220_5: call _get_HCR_EL2_Type_TGE(s_220_4)
        let s_220_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_220_4);
        // D s_220_6: cast zx s_220_2 -> bv
        let s_220_6: Bits = Bits::new(s_220_2 as u128, 1u16);
        // D s_220_7: cast zx s_220_5 -> bv
        let s_220_7: Bits = Bits::new(s_220_5 as u128, 1u16);
        // D s_220_8: cast reint s_220_6 -> u128
        let s_220_8: u128 = (s_220_6.value() as u128);
        // D s_220_9: size-of s_220_6
        let s_220_9: u16 = s_220_6.length();
        // D s_220_10: cast reint s_220_7 -> u128
        let s_220_10: u128 = (s_220_7.value() as u128);
        // D s_220_11: size-of s_220_7
        let s_220_11: u16 = s_220_7.length();
        // D s_220_12: lsl s_220_8 s_220_11
        let s_220_12: u128 = s_220_8 << s_220_11;
        // D s_220_13: or s_220_12 s_220_10
        let s_220_13: u128 = ((s_220_12) | (s_220_10));
        // D s_220_14: add s_220_9 s_220_11
        let s_220_14: u16 = (s_220_9 + s_220_11);
        // D s_220_15: create-bits s_220_13 s_220_14
        let s_220_15: Bits = Bits::new(s_220_13, s_220_14);
        // D s_220_16: cast reint s_220_15 -> u8
        let s_220_16: u8 = (s_220_15.value() as u8);
        // D s_220_17: cast zx s_220_16 -> bv
        let s_220_17: Bits = Bits::new(s_220_16 as u128, 2u16);
        // C s_220_18: const #3u : u8
        let s_220_18: u8 = 3;
        // C s_220_19: cast zx s_220_18 -> bv
        let s_220_19: Bits = Bits::new(s_220_18 as u128, 2u16);
        // D s_220_20: cmp-eq s_220_17 s_220_19
        let s_220_20: bool = ((s_220_17) == (s_220_19));
        // D s_220_21: write-var gs#94060 <= s_220_20
        fn_state.gs_94060 = s_220_20;
        // N s_220_22: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #() : ()
        let s_221_0: () = ();
        // S s_221_1: call EL2Enabled(s_221_0)
        let s_221_1: bool = EL2Enabled(state, tracer, s_221_0);
        // N s_221_2: branch s_221_1 b226 b222
        if s_221_1 {
            return block_226(state, tracer, fn_state);
        } else {
            return block_222(state, tracer, fn_state);
        };
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_222_0: const #0u : u8
        let s_222_0: bool = false;
        // D s_222_1: write-var gs#94079 <= s_222_0
        fn_state.gs_94079 = s_222_0;
        // N s_222_2: jump b223
        return block_223(state, tracer, fn_state);
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_223_0: read-var gs#94079:u8
        let s_223_0: bool = fn_state.gs_94079;
        // N s_223_1: branch s_223_0 b225 b224
        if s_223_0 {
            return block_225(state, tracer, fn_state);
        } else {
            return block_224(state, tracer, fn_state);
        };
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_224_0: const #24u : u8
        let s_224_0: u8 = 24;
        // C s_224_1: cast zx s_224_0 -> bv
        let s_224_1: Bits = Bits::new(s_224_0 as u128, 8u16);
        // C s_224_2: cast zx s_224_1 -> i
        let s_224_2: i128 = (s_224_1.value() as i128);
        // C s_224_3: cast reint s_224_2 -> i64
        let s_224_3: i64 = (s_224_2 as i64);
        // C s_224_4: cast zx s_224_3 -> i
        let s_224_4: i128 = (i128::try_from(s_224_3).unwrap());
        // C s_224_5: const #440u : u32
        let s_224_5: u32 = 440;
        // D s_224_6: read-reg s_224_5:u8
        let s_224_6: u8 = {
            let value = state.read_register::<u8>(s_224_5 as isize);
            tracer.read_register(s_224_5 as isize, value);
            value
        };
        // D s_224_7: call AArch64_SystemAccessTrap(s_224_6, s_224_4)
        let s_224_7: () = AArch64_SystemAccessTrap(state, tracer, s_224_6, s_224_4);
        // N s_224_8: return
        return;
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #24u : u8
        let s_225_0: u8 = 24;
        // C s_225_1: cast zx s_225_0 -> bv
        let s_225_1: Bits = Bits::new(s_225_0 as u128, 8u16);
        // C s_225_2: cast zx s_225_1 -> i
        let s_225_2: i128 = (s_225_1.value() as i128);
        // C s_225_3: cast reint s_225_2 -> i64
        let s_225_3: i64 = (s_225_2 as i64);
        // C s_225_4: cast zx s_225_3 -> i
        let s_225_4: i128 = (i128::try_from(s_225_3).unwrap());
        // C s_225_5: const #432u : u32
        let s_225_5: u32 = 432;
        // D s_225_6: read-reg s_225_5:u8
        let s_225_6: u8 = {
            let value = state.read_register::<u8>(s_225_5 as isize);
            tracer.read_register(s_225_5 as isize, value);
            value
        };
        // D s_225_7: call AArch64_SystemAccessTrap(s_225_6, s_225_4)
        let s_225_7: () = AArch64_SystemAccessTrap(state, tracer, s_225_6, s_225_4);
        // N s_225_8: return
        return;
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_226_0: read-var __HCR_EL2_TGE:u8
        let s_226_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_226_1: cast zx s_226_0 -> bv
        let s_226_1: Bits = Bits::new(s_226_0 as u128, 1u16);
        // C s_226_2: const #1u : u8
        let s_226_2: bool = true;
        // C s_226_3: cast zx s_226_2 -> bv
        let s_226_3: Bits = Bits::new(s_226_2 as u128, 1u16);
        // D s_226_4: cmp-eq s_226_1 s_226_3
        let s_226_4: bool = ((s_226_1) == (s_226_3));
        // D s_226_5: write-var gs#94079 <= s_226_4
        fn_state.gs_94079 = s_226_4;
        // N s_226_6: jump b223
        return block_223(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_227_0: panic
        panic!("{:?}", ());
        // N s_227_1: return
        return;
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_228_0: const #() : ()
        let s_228_0: () = ();
        // S s_228_1: call __get_selected_SPMACCESSR_EL3_field(s_228_0)
        let s_228_1: u8 = u__get_selected_SPMACCESSR_EL3_field(state, tracer, s_228_0);
        // S s_228_2: cast zx s_228_1 -> bv
        let s_228_2: Bits = Bits::new(s_228_1 as u128, 2u16);
        // C s_228_3: const #3u : u8
        let s_228_3: u8 = 3;
        // C s_228_4: cast zx s_228_3 -> bv
        let s_228_4: Bits = Bits::new(s_228_3 as u128, 2u16);
        // S s_228_5: cmp-ne s_228_2 s_228_4
        let s_228_5: bool = ((s_228_2) != (s_228_4));
        // D s_228_6: write-var gs#94059 <= s_228_5
        fn_state.gs_94059 = s_228_5;
        // N s_228_7: jump b150
        return block_150(state, tracer, fn_state);
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_229_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_229_1: call __IMPDEF_boolean(s_229_0)
        let s_229_1: bool = u__IMPDEF_boolean(state, tracer, s_229_0);
        // D s_229_2: write-var gs#94058 <= s_229_1
        fn_state.gs_94058 = s_229_1;
        // N s_229_3: jump b148
        return block_148(state, tracer, fn_state);
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_230_0: const #() : ()
        let s_230_0: () = ();
        // S s_230_1: call EDSCR_read(s_230_0)
        let s_230_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_230_0);
        // S s_230_2: call _get_EDSCR_Type_SDD(s_230_1)
        let s_230_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_230_1);
        // S s_230_3: cast zx s_230_2 -> bv
        let s_230_3: Bits = Bits::new(s_230_2 as u128, 1u16);
        // C s_230_4: const #1u : u8
        let s_230_4: bool = true;
        // C s_230_5: cast zx s_230_4 -> bv
        let s_230_5: Bits = Bits::new(s_230_4 as u128, 1u16);
        // S s_230_6: cmp-eq s_230_3 s_230_5
        let s_230_6: bool = ((s_230_3) == (s_230_5));
        // D s_230_7: write-var gs#94057 <= s_230_6
        fn_state.gs_94057 = s_230_6;
        // N s_230_8: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_231_0: const #424u : u32
        let s_231_0: u32 = 424;
        // D s_231_1: read-reg s_231_0:u8
        let s_231_1: u8 = {
            let value = state.read_register::<u8>(s_231_0 as isize);
            tracer.read_register(s_231_0 as isize, value);
            value
        };
        // C s_231_2: const #2u : u8
        let s_231_2: u8 = 2;
        // D s_231_3: cmp-lt s_231_1 s_231_2
        let s_231_3: bool = ((s_231_1) < (s_231_2));
        // D s_231_4: write-var gs#94056 <= s_231_3
        fn_state.gs_94056 = s_231_3;
        // N s_231_5: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_232_0: panic
        panic!("{:?}", ());
        // N s_232_1: return
        return;
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_233_0: read-var __MDCR_EL3_EnPM2:u8
        let s_233_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_233_1: cast zx s_233_0 -> bv
        let s_233_1: Bits = Bits::new(s_233_0 as u128, 1u16);
        // C s_233_2: const #0u : u8
        let s_233_2: bool = false;
        // C s_233_3: cast zx s_233_2 -> bv
        let s_233_3: Bits = Bits::new(s_233_2 as u128, 1u16);
        // D s_233_4: cmp-eq s_233_1 s_233_3
        let s_233_4: bool = ((s_233_1) == (s_233_3));
        // D s_233_5: write-var gs#94055 <= s_233_4
        fn_state.gs_94055 = s_233_4;
        // N s_233_6: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_234_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_234_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_234_1: call __IMPDEF_boolean(s_234_0)
        let s_234_1: bool = u__IMPDEF_boolean(state, tracer, s_234_0);
        // D s_234_2: write-var gs#94054 <= s_234_1
        fn_state.gs_94054 = s_234_1;
        // N s_234_3: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_235_0: const #() : ()
        let s_235_0: () = ();
        // S s_235_1: call EDSCR_read(s_235_0)
        let s_235_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_235_0);
        // S s_235_2: call _get_EDSCR_Type_SDD(s_235_1)
        let s_235_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_235_1);
        // S s_235_3: cast zx s_235_2 -> bv
        let s_235_3: Bits = Bits::new(s_235_2 as u128, 1u16);
        // C s_235_4: const #1u : u8
        let s_235_4: bool = true;
        // C s_235_5: cast zx s_235_4 -> bv
        let s_235_5: Bits = Bits::new(s_235_4 as u128, 1u16);
        // S s_235_6: cmp-eq s_235_3 s_235_5
        let s_235_6: bool = ((s_235_3) == (s_235_5));
        // D s_235_7: write-var gs#94053 <= s_235_6
        fn_state.gs_94053 = s_235_6;
        // N s_235_8: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_236_0: const #424u : u32
        let s_236_0: u32 = 424;
        // D s_236_1: read-reg s_236_0:u8
        let s_236_1: u8 = {
            let value = state.read_register::<u8>(s_236_0 as isize);
            tracer.read_register(s_236_0 as isize, value);
            value
        };
        // C s_236_2: const #2u : u8
        let s_236_2: u8 = 2;
        // D s_236_3: cmp-lt s_236_1 s_236_2
        let s_236_3: bool = ((s_236_1) < (s_236_2));
        // D s_236_4: write-var gs#94052 <= s_236_3
        fn_state.gs_94052 = s_236_3;
        // N s_236_5: jump b135
        return block_135(state, tracer, fn_state);
    }
}
