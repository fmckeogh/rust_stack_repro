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
use u_get_BRBFCR_EL1_Type_BANK::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use Halted::*;
use u_get_MDCR_EL3_Type_SBRBE::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use Zeros::*;
use u__IMPDEF_boolean::*;
use u_get_SCR_EL3_Type_NS::*;
use X_set::*;
use u__get_BRBINF_EL1::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_HDFGRTR_EL2_Type_nBRBDATA::*;
use EDSCR_read::*;
use common::*;
pub fn BRBINF_EL1_SysRegRead_ea333ab47e4a6f1e<T: Tracer>(
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
        u__SCR_EL3_NS: bool,
        gs_52227: bool,
        gs_52256: bool,
        gs_52262: bool,
        gs_52292: bool,
        gs_52226: bool,
        gs_52275: bool,
        ga_48049: ProductType5c790c8ef59cc8b2,
        gs_52231: bool,
        gs_52261: bool,
        gs_52232: bool,
        gs_52272: bool,
        gs_52240: bool,
        u__PSTATE_EL: u8,
        gs_52241: bool,
        u__HDFGRTR_EL2_nBRBDATA: bool,
        gs_52276: bool,
        gs_52277: bool,
        gs_52247: bool,
        gs_52224: bool,
        gs_52283: bool,
        gs_52238: bool,
        gs_52270: bool,
        u__EDSCR_SDD: bool,
        gs_52229: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_52246: bool,
        gs_52255: bool,
        gs_52237: bool,
        gs_52260: bool,
        gs_52230: bool,
        gs_52228: bool,
        gs_52274: bool,
        gs_52225: bool,
        ga_48061: ProductType5c790c8ef59cc8b2,
        gs_52259: bool,
        gs_52264: bool,
        u__MDCR_EL3_SBRBE: u8,
        gs_52273: bool,
        gs_52284: bool,
        gs_52258: bool,
        gs_52239: bool,
        gs_52265: bool,
        gs_52293: bool,
        gs_52257: bool,
        ga_48007: ProductType5c790c8ef59cc8b2,
        gs_52278: bool,
        gs_52263: bool,
        gs_52271: bool,
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
        // D s_0_21: call _get_HDFGRTR_EL2_Type_nBRBDATA(s_0_20)
        let s_0_21: bool = u_get_HDFGRTR_EL2_Type_nBRBDATA(state, tracer, s_0_20);
        // D s_0_22: write-var __HDFGRTR_EL2_nBRBDATA <= s_0_21
        fn_state.u__HDFGRTR_EL2_nBRBDATA = s_0_21;
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
        // N s_0_29: branch s_0_28 b160 b1
        if s_0_28 {
            return block_160(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b77 b2
        if s_1_5 {
            return block_77(state, tracer, fn_state);
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
        // C s_5_0: const #16536u : u32
        let s_5_0: u32 = 16536;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call _get_BRBFCR_EL1_Type_BANK(s_5_1)
        let s_5_2: u8 = u_get_BRBFCR_EL1_Type_BANK(state, tracer, s_5_1);
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (s_5_3.value() as i128);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // C s_5_6: const #32s : i
        let s_5_6: i128 = 32;
        // D s_5_7: cast zx s_5_5 -> i
        let s_5_7: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_8: mul s_5_7 s_5_6
        let s_5_8: i128 = ((s_5_7) * (s_5_6));
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // C s_5_10: const #28s : i
        let s_5_10: i128 = 28;
        // D s_5_11: cast zx s_5_9 -> i
        let s_5_11: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_12: add s_5_10 s_5_11
        let s_5_12: i128 = (s_5_10 + s_5_11);
        // D s_5_13: cast reint s_5_12 -> i64
        let s_5_13: i64 = (s_5_12 as i64);
        // C s_5_14: const #64s : i
        let s_5_14: i128 = 64;
        // D s_5_15: cast zx s_5_13 -> i
        let s_5_15: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_16: cmp-ge s_5_15 s_5_14
        let s_5_16: bool = ((s_5_15) >= (s_5_14));
        // N s_5_17: branch s_5_16 b7 b6
        if s_5_16 {
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
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #28s : i
        let s_6_1: i128 = 28;
        // C s_6_2: const #22120u : u32
        let s_6_2: u32 = 22120;
        // D s_6_3: read-reg s_6_2:[struct; 32]
        let s_6_3: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 32usize]>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: read-element s_6_3[s_6_1]
        let s_6_4: ProductType5c790c8ef59cc8b2 = s_6_3[(s_6_1) as usize];
        // D s_6_5: call __get_BRBINF_EL1(s_6_4)
        let s_6_5: ProductType5c790c8ef59cc8b2 = u__get_BRBINF_EL1(state, tracer, s_6_4);
        // D s_6_6: write-var ga#48061 <= s_6_5
        fn_state.ga_48061 = s_6_5;
        // D s_6_7: read-var ga#48061.0:struct
        let s_6_7: u64 = fn_state.ga_48061._0;
        // D s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 64u16);
        // D s_6_9: read-var t:i
        let s_6_9: i128 = fn_state.t;
        // D s_6_10: call X_set(s_6_9, s_6_0, s_6_8)
        let s_6_10: () = X_set(state, tracer, s_6_9, s_6_0, s_6_8);
        // N s_6_11: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #64s : i
        let s_7_1: i128 = 64;
        // S s_7_2: call Zeros(s_7_1)
        let s_7_2: Bits = Zeros(state, tracer, s_7_1);
        // S s_7_3: cast reint s_7_2 -> u64
        let s_7_3: u64 = (s_7_2.value() as u64);
        // S s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 64u16);
        // D s_7_5: read-var t:i
        let s_7_5: i128 = fn_state.t;
        // D s_7_6: call X_set(s_7_5, s_7_0, s_7_4)
        let s_7_6: () = X_set(state, tracer, s_7_5, s_7_0, s_7_4);
        // N s_7_7: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call Halted(s_8_0)
        let s_8_1: bool = Halted(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b76 b9
        if s_8_1 {
            return block_76(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#52224 <= s_9_0
        fn_state.gs_52224 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#52224:u8
        let s_10_0: bool = fn_state.gs_52224;
        // N s_10_1: branch s_10_0 b75 b11
        if s_10_0 {
            return block_75(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#52225 <= s_11_0
        fn_state.gs_52225 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#52225:u8
        let s_12_0: bool = fn_state.gs_52225;
        // N s_12_1: branch s_12_0 b74 b13
        if s_12_0 {
            return block_74(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#52226 <= s_13_0
        fn_state.gs_52226 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#52226:u8
        let s_14_0: bool = fn_state.gs_52226;
        // N s_14_1: branch s_14_0 b73 b15
        if s_14_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#52227 <= s_15_0
        fn_state.gs_52227 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#52227:u8
        let s_16_0: bool = fn_state.gs_52227;
        // N s_16_1: branch s_16_0 b72 b17
        if s_16_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#52228 <= s_17_0
        fn_state.gs_52228 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#52228:u8
        let s_18_0: bool = fn_state.gs_52228;
        // N s_18_1: branch s_18_0 b71 b19
        if s_18_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
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
        // N s_19_2: branch s_19_1 b70 b20
        if s_19_1 {
            return block_70(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#52229 <= s_20_0
        fn_state.gs_52229 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#52229:u8
        let s_21_0: bool = fn_state.gs_52229;
        // N s_21_1: branch s_21_0 b69 b22
        if s_21_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#52230 <= s_22_0
        fn_state.gs_52230 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#52230:u8
        let s_23_0: bool = fn_state.gs_52230;
        // N s_23_1: branch s_23_0 b68 b24
        if s_23_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#52231 <= s_24_0
        fn_state.gs_52231 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#52231:u8
        let s_25_0: bool = fn_state.gs_52231;
        // N s_25_1: branch s_25_0 b64 b26
        if s_25_0 {
            return block_64(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#52237 <= s_26_0
        fn_state.gs_52237 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#52237:u8
        let s_27_0: bool = fn_state.gs_52237;
        // N s_27_1: branch s_27_0 b63 b28
        if s_27_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#52238 <= s_28_0
        fn_state.gs_52238 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#52238:u8
        let s_29_0: bool = fn_state.gs_52238;
        // N s_29_1: branch s_29_0 b62 b30
        if s_29_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #424u : u32
        let s_30_0: u32 = 424;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // C s_30_2: const #2u : u8
        let s_30_2: u8 = 2;
        // D s_30_3: cmp-lt s_30_1 s_30_2
        let s_30_3: bool = ((s_30_1) < (s_30_2));
        // N s_30_4: branch s_30_3 b61 b31
        if s_30_3 {
            return block_61(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#52239 <= s_31_0
        fn_state.gs_52239 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#52239:u8
        let s_32_0: bool = fn_state.gs_52239;
        // N s_32_1: branch s_32_0 b60 b33
        if s_32_0 {
            return block_60(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#52240 <= s_33_0
        fn_state.gs_52240 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#52240:u8
        let s_34_0: bool = fn_state.gs_52240;
        // N s_34_1: branch s_34_0 b54 b35
        if s_34_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #424u : u32
        let s_35_0: u32 = 424;
        // D s_35_1: read-reg s_35_0:u8
        let s_35_1: u8 = {
            let value = state.read_register::<u8>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // C s_35_2: const #2u : u8
        let s_35_2: u8 = 2;
        // D s_35_3: cmp-lt s_35_1 s_35_2
        let s_35_3: bool = ((s_35_1) < (s_35_2));
        // N s_35_4: branch s_35_3 b50 b36
        if s_35_3 {
            return block_50(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#52246 <= s_36_0
        fn_state.gs_52246 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#52246:u8
        let s_37_0: bool = fn_state.gs_52246;
        // N s_37_1: branch s_37_0 b49 b38
        if s_37_0 {
            return block_49(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#52247 <= s_38_0
        fn_state.gs_52247 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#52247:u8
        let s_39_0: bool = fn_state.gs_52247;
        // N s_39_1: branch s_39_0 b43 b40
        if s_39_0 {
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
        // C s_40_0: const #16536u : u32
        let s_40_0: u32 = 16536;
        // D s_40_1: read-reg s_40_0:struct
        let s_40_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: call _get_BRBFCR_EL1_Type_BANK(s_40_1)
        let s_40_2: u8 = u_get_BRBFCR_EL1_Type_BANK(state, tracer, s_40_1);
        // D s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 2u16);
        // D s_40_4: cast zx s_40_3 -> i
        let s_40_4: i128 = (s_40_3.value() as i128);
        // D s_40_5: cast reint s_40_4 -> i64
        let s_40_5: i64 = (s_40_4 as i64);
        // C s_40_6: const #32s : i
        let s_40_6: i128 = 32;
        // D s_40_7: cast zx s_40_5 -> i
        let s_40_7: i128 = (i128::try_from(s_40_5).unwrap());
        // D s_40_8: mul s_40_7 s_40_6
        let s_40_8: i128 = ((s_40_7) * (s_40_6));
        // D s_40_9: cast reint s_40_8 -> i64
        let s_40_9: i64 = (s_40_8 as i64);
        // C s_40_10: const #28s : i
        let s_40_10: i128 = 28;
        // D s_40_11: cast zx s_40_9 -> i
        let s_40_11: i128 = (i128::try_from(s_40_9).unwrap());
        // D s_40_12: add s_40_10 s_40_11
        let s_40_12: i128 = (s_40_10 + s_40_11);
        // D s_40_13: cast reint s_40_12 -> i64
        let s_40_13: i64 = (s_40_12 as i64);
        // C s_40_14: const #64s : i
        let s_40_14: i128 = 64;
        // D s_40_15: cast zx s_40_13 -> i
        let s_40_15: i128 = (i128::try_from(s_40_13).unwrap());
        // D s_40_16: cmp-ge s_40_15 s_40_14
        let s_40_16: bool = ((s_40_15) >= (s_40_14));
        // N s_40_17: branch s_40_16 b42 b41
        if s_40_16 {
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
        // C s_41_0: const #64s : i64
        let s_41_0: i64 = 64;
        // C s_41_1: const #28s : i
        let s_41_1: i128 = 28;
        // C s_41_2: const #22120u : u32
        let s_41_2: u32 = 22120;
        // D s_41_3: read-reg s_41_2:[struct; 32]
        let s_41_3: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_41_2 as isize);
            tracer.read_register(s_41_2 as isize, value);
            value
        };
        // D s_41_4: read-element s_41_3[s_41_1]
        let s_41_4: ProductType5c790c8ef59cc8b2 = s_41_3[(s_41_1) as usize];
        // D s_41_5: call __get_BRBINF_EL1(s_41_4)
        let s_41_5: ProductType5c790c8ef59cc8b2 = u__get_BRBINF_EL1(
            state,
            tracer,
            s_41_4,
        );
        // D s_41_6: write-var ga#48049 <= s_41_5
        fn_state.ga_48049 = s_41_5;
        // D s_41_7: read-var ga#48049.0:struct
        let s_41_7: u64 = fn_state.ga_48049._0;
        // D s_41_8: cast zx s_41_7 -> bv
        let s_41_8: Bits = Bits::new(s_41_7 as u128, 64u16);
        // D s_41_9: read-var t:i
        let s_41_9: i128 = fn_state.t;
        // D s_41_10: call X_set(s_41_9, s_41_0, s_41_8)
        let s_41_10: () = X_set(state, tracer, s_41_9, s_41_0, s_41_8);
        // N s_41_11: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #64s : i64
        let s_42_0: i64 = 64;
        // C s_42_1: const #64s : i
        let s_42_1: i128 = 64;
        // S s_42_2: call Zeros(s_42_1)
        let s_42_2: Bits = Zeros(state, tracer, s_42_1);
        // S s_42_3: cast reint s_42_2 -> u64
        let s_42_3: u64 = (s_42_2.value() as u64);
        // S s_42_4: cast zx s_42_3 -> bv
        let s_42_4: Bits = Bits::new(s_42_3 as u128, 64u16);
        // D s_42_5: read-var t:i
        let s_42_5: i128 = fn_state.t;
        // D s_42_6: call X_set(s_42_5, s_42_0, s_42_4)
        let s_42_6: () = X_set(state, tracer, s_42_5, s_42_0, s_42_4);
        // N s_42_7: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call Halted(s_43_0)
        let s_43_1: bool = Halted(state, tracer, s_43_0);
        // N s_43_2: branch s_43_1 b48 b44
        if s_43_1 {
            return block_48(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#52255 <= s_44_0
        fn_state.gs_52255 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#52255:u8
        let s_45_0: bool = fn_state.gs_52255;
        // N s_45_1: branch s_45_0 b47 b46
        if s_45_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #24u : u8
        let s_46_0: u8 = 24;
        // C s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 8u16);
        // C s_46_2: cast zx s_46_1 -> i
        let s_46_2: i128 = (s_46_1.value() as i128);
        // C s_46_3: cast reint s_46_2 -> i64
        let s_46_3: i64 = (s_46_2 as i64);
        // C s_46_4: cast zx s_46_3 -> i
        let s_46_4: i128 = (i128::try_from(s_46_3).unwrap());
        // C s_46_5: const #424u : u32
        let s_46_5: u32 = 424;
        // D s_46_6: read-reg s_46_5:u8
        let s_46_6: u8 = {
            let value = state.read_register::<u8>(s_46_5 as isize);
            tracer.read_register(s_46_5 as isize, value);
            value
        };
        // D s_46_7: call AArch64_SystemAccessTrap(s_46_6, s_46_4)
        let s_46_7: () = AArch64_SystemAccessTrap(state, tracer, s_46_6, s_46_4);
        // N s_46_8: return
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
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __EDSCR_SDD:u8
        let s_48_0: bool = fn_state.u__EDSCR_SDD;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #1u : u8
        let s_48_2: bool = true;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#52255 <= s_48_4
        fn_state.gs_52255 = s_48_4;
        // N s_48_6: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var __SCR_EL3_NS:u8
        let s_49_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 1u16);
        // C s_49_2: const #1u : u8
        let s_49_2: bool = true;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // D s_49_5: write-var gs#52247 <= s_49_4
        fn_state.gs_52247 = s_49_4;
        // N s_49_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var __MDCR_EL3_SBRBE:u8
        let s_50_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_50_1: const #0s : i
        let s_50_1: i128 = 0;
        // D s_50_2: cast zx s_50_0 -> bv
        let s_50_2: Bits = Bits::new(s_50_0 as u128, 2u16);
        // C s_50_3: const #1s : i64
        let s_50_3: i64 = 1;
        // C s_50_4: cast zx s_50_3 -> i
        let s_50_4: i128 = (i128::try_from(s_50_3).unwrap());
        // C s_50_5: const #0s : i
        let s_50_5: i128 = 0;
        // C s_50_6: add s_50_5 s_50_4
        let s_50_6: i128 = (s_50_5 + s_50_4);
        // D s_50_7: bit-extract s_50_2 s_50_1 s_50_6
        let s_50_7: Bits = (Bits::new(
            ((s_50_2) >> (s_50_1)).value(),
            u16::try_from(s_50_6).unwrap(),
        ));
        // D s_50_8: cast reint s_50_7 -> u8
        let s_50_8: bool = ((s_50_7.value()) != 0);
        // D s_50_9: cast zx s_50_8 -> bv
        let s_50_9: Bits = Bits::new(s_50_8 as u128, 1u16);
        // C s_50_10: const #0u : u8
        let s_50_10: bool = false;
        // C s_50_11: cast zx s_50_10 -> bv
        let s_50_11: Bits = Bits::new(s_50_10 as u128, 1u16);
        // D s_50_12: cmp-eq s_50_9 s_50_11
        let s_50_12: bool = ((s_50_9) == (s_50_11));
        // D s_50_13: not s_50_12
        let s_50_13: bool = !s_50_12;
        // N s_50_14: branch s_50_13 b53 b51
        if s_50_13 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #1u : u8
        let s_51_0: bool = true;
        // D s_51_1: write-var gs#52241 <= s_51_0
        fn_state.gs_52241 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#52241:u8
        let s_52_0: bool = fn_state.gs_52241;
        // D s_52_1: write-var gs#52246 <= s_52_0
        fn_state.gs_52246 = s_52_0;
        // N s_52_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#52241 <= s_53_0
        fn_state.gs_52241 = s_53_0;
        // N s_53_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call Halted(s_54_0)
        let s_54_1: bool = Halted(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b59 b55
        if s_54_1 {
            return block_59(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#52256 <= s_55_0
        fn_state.gs_52256 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#52256:u8
        let s_56_0: bool = fn_state.gs_52256;
        // N s_56_1: branch s_56_0 b58 b57
        if s_56_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #24u : u8
        let s_57_0: u8 = 24;
        // C s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 8u16);
        // C s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (s_57_1.value() as i128);
        // C s_57_3: cast reint s_57_2 -> i64
        let s_57_3: i64 = (s_57_2 as i64);
        // C s_57_4: cast zx s_57_3 -> i
        let s_57_4: i128 = (i128::try_from(s_57_3).unwrap());
        // C s_57_5: const #424u : u32
        let s_57_5: u32 = 424;
        // D s_57_6: read-reg s_57_5:u8
        let s_57_6: u8 = {
            let value = state.read_register::<u8>(s_57_5 as isize);
            tracer.read_register(s_57_5 as isize, value);
            value
        };
        // D s_57_7: call AArch64_SystemAccessTrap(s_57_6, s_57_4)
        let s_57_7: () = AArch64_SystemAccessTrap(state, tracer, s_57_6, s_57_4);
        // N s_57_8: return
        return;
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
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var __EDSCR_SDD:u8
        let s_59_0: bool = fn_state.u__EDSCR_SDD;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 1u16);
        // C s_59_2: const #1u : u8
        let s_59_2: bool = true;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: write-var gs#52256 <= s_59_4
        fn_state.gs_52256 = s_59_4;
        // N s_59_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __SCR_EL3_NS:u8
        let s_60_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #0u : u8
        let s_60_2: bool = false;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#52240 <= s_60_4
        fn_state.gs_52240 = s_60_4;
        // N s_60_6: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var __MDCR_EL3_SBRBE:u8
        let s_61_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 2u16);
        // C s_61_2: const #3u : u8
        let s_61_2: u8 = 3;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 2u16);
        // D s_61_4: cmp-ne s_61_1 s_61_3
        let s_61_4: bool = ((s_61_1) != (s_61_3));
        // D s_61_5: write-var gs#52239 <= s_61_4
        fn_state.gs_52239 = s_61_4;
        // N s_61_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_62_0: panic
        panic!("{:?}", ());
        // N s_62_1: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var __SCR_EL3_NS:u8
        let s_63_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #1u : u8
        let s_63_2: bool = true;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#52238 <= s_63_4
        fn_state.gs_52238 = s_63_4;
        // N s_63_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __MDCR_EL3_SBRBE:u8
        let s_64_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_64_1: const #0s : i
        let s_64_1: i128 = 0;
        // D s_64_2: cast zx s_64_0 -> bv
        let s_64_2: Bits = Bits::new(s_64_0 as u128, 2u16);
        // C s_64_3: const #1s : i64
        let s_64_3: i64 = 1;
        // C s_64_4: cast zx s_64_3 -> i
        let s_64_4: i128 = (i128::try_from(s_64_3).unwrap());
        // C s_64_5: const #0s : i
        let s_64_5: i128 = 0;
        // C s_64_6: add s_64_5 s_64_4
        let s_64_6: i128 = (s_64_5 + s_64_4);
        // D s_64_7: bit-extract s_64_2 s_64_1 s_64_6
        let s_64_7: Bits = (Bits::new(
            ((s_64_2) >> (s_64_1)).value(),
            u16::try_from(s_64_6).unwrap(),
        ));
        // D s_64_8: cast reint s_64_7 -> u8
        let s_64_8: bool = ((s_64_7.value()) != 0);
        // D s_64_9: cast zx s_64_8 -> bv
        let s_64_9: Bits = Bits::new(s_64_8 as u128, 1u16);
        // C s_64_10: const #0u : u8
        let s_64_10: bool = false;
        // C s_64_11: cast zx s_64_10 -> bv
        let s_64_11: Bits = Bits::new(s_64_10 as u128, 1u16);
        // D s_64_12: cmp-eq s_64_9 s_64_11
        let s_64_12: bool = ((s_64_9) == (s_64_11));
        // D s_64_13: not s_64_12
        let s_64_13: bool = !s_64_12;
        // N s_64_14: branch s_64_13 b67 b65
        if s_64_13 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#52232 <= s_65_0
        fn_state.gs_52232 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#52232:u8
        let s_66_0: bool = fn_state.gs_52232;
        // D s_66_1: write-var gs#52237 <= s_66_0
        fn_state.gs_52237 = s_66_0;
        // N s_66_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#52232 <= s_67_0
        fn_state.gs_52232 = s_67_0;
        // N s_67_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_68_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_68_1: call __IMPDEF_boolean(s_68_0)
        let s_68_1: bool = u__IMPDEF_boolean(state, tracer, s_68_0);
        // D s_68_2: write-var gs#52231 <= s_68_1
        fn_state.gs_52231 = s_68_1;
        // N s_68_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __EDSCR_SDD:u8
        let s_69_0: bool = fn_state.u__EDSCR_SDD;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#52230 <= s_69_4
        fn_state.gs_52230 = s_69_4;
        // N s_69_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #424u : u32
        let s_70_0: u32 = 424;
        // D s_70_1: read-reg s_70_0:u8
        let s_70_1: u8 = {
            let value = state.read_register::<u8>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // C s_70_2: const #2u : u8
        let s_70_2: u8 = 2;
        // D s_70_3: cmp-lt s_70_1 s_70_2
        let s_70_3: bool = ((s_70_1) < (s_70_2));
        // D s_70_4: write-var gs#52229 <= s_70_3
        fn_state.gs_52229 = s_70_3;
        // N s_70_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_71_0: panic
        panic!("{:?}", ());
        // N s_71_1: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var __SCR_EL3_NS:u8
        let s_72_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #0u : u8
        let s_72_2: bool = false;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#52228 <= s_72_4
        fn_state.gs_52228 = s_72_4;
        // N s_72_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __MDCR_EL3_SBRBE:u8
        let s_73_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 2u16);
        // C s_73_2: const #3u : u8
        let s_73_2: u8 = 3;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 2u16);
        // D s_73_4: cmp-ne s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) != (s_73_3));
        // D s_73_5: write-var gs#52227 <= s_73_4
        fn_state.gs_52227 = s_73_4;
        // N s_73_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_74_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_74_1: call __IMPDEF_boolean(s_74_0)
        let s_74_1: bool = u__IMPDEF_boolean(state, tracer, s_74_0);
        // D s_74_2: write-var gs#52226 <= s_74_1
        fn_state.gs_52226 = s_74_1;
        // N s_74_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var __EDSCR_SDD:u8
        let s_75_0: bool = fn_state.u__EDSCR_SDD;
        // D s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 1u16);
        // C s_75_2: const #1u : u8
        let s_75_2: bool = true;
        // C s_75_3: cast zx s_75_2 -> bv
        let s_75_3: Bits = Bits::new(s_75_2 as u128, 1u16);
        // D s_75_4: cmp-eq s_75_1 s_75_3
        let s_75_4: bool = ((s_75_1) == (s_75_3));
        // D s_75_5: write-var gs#52225 <= s_75_4
        fn_state.gs_52225 = s_75_4;
        // N s_75_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #424u : u32
        let s_76_0: u32 = 424;
        // D s_76_1: read-reg s_76_0:u8
        let s_76_1: u8 = {
            let value = state.read_register::<u8>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // C s_76_2: const #2u : u8
        let s_76_2: u8 = 2;
        // D s_76_3: cmp-lt s_76_1 s_76_2
        let s_76_3: bool = ((s_76_1) < (s_76_2));
        // D s_76_4: write-var gs#52224 <= s_76_3
        fn_state.gs_52224 = s_76_3;
        // N s_76_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #() : ()
        let s_77_0: () = ();
        // S s_77_1: call Halted(s_77_0)
        let s_77_1: bool = Halted(state, tracer, s_77_0);
        // N s_77_2: branch s_77_1 b159 b78
        if s_77_1 {
            return block_159(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#52257 <= s_78_0
        fn_state.gs_52257 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#52257:u8
        let s_79_0: bool = fn_state.gs_52257;
        // N s_79_1: branch s_79_0 b158 b80
        if s_79_0 {
            return block_158(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#52258 <= s_80_0
        fn_state.gs_52258 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#52258:u8
        let s_81_0: bool = fn_state.gs_52258;
        // N s_81_1: branch s_81_0 b157 b82
        if s_81_0 {
            return block_157(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#52259 <= s_82_0
        fn_state.gs_52259 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#52259:u8
        let s_83_0: bool = fn_state.gs_52259;
        // N s_83_1: branch s_83_0 b156 b84
        if s_83_0 {
            return block_156(state, tracer, fn_state);
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
        // D s_84_1: write-var gs#52260 <= s_84_0
        fn_state.gs_52260 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#52260:u8
        let s_85_0: bool = fn_state.gs_52260;
        // N s_85_1: branch s_85_0 b155 b86
        if s_85_0 {
            return block_155(state, tracer, fn_state);
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
        // D s_86_1: write-var gs#52261 <= s_86_0
        fn_state.gs_52261 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#52261:u8
        let s_87_0: bool = fn_state.gs_52261;
        // N s_87_1: branch s_87_0 b154 b88
        if s_87_0 {
            return block_154(state, tracer, fn_state);
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
        // S s_88_1: call Halted(s_88_0)
        let s_88_1: bool = Halted(state, tracer, s_88_0);
        // N s_88_2: branch s_88_1 b153 b89
        if s_88_1 {
            return block_153(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#52262 <= s_89_0
        fn_state.gs_52262 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#52262:u8
        let s_90_0: bool = fn_state.gs_52262;
        // N s_90_1: branch s_90_0 b152 b91
        if s_90_0 {
            return block_152(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#52263 <= s_91_0
        fn_state.gs_52263 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#52263:u8
        let s_92_0: bool = fn_state.gs_52263;
        // N s_92_1: branch s_92_0 b151 b93
        if s_92_0 {
            return block_151(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#52264 <= s_93_0
        fn_state.gs_52264 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#52264:u8
        let s_94_0: bool = fn_state.gs_52264;
        // N s_94_1: branch s_94_0 b147 b95
        if s_94_0 {
            return block_147(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#52270 <= s_95_0
        fn_state.gs_52270 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#52270:u8
        let s_96_0: bool = fn_state.gs_52270;
        // N s_96_1: branch s_96_0 b146 b97
        if s_96_0 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#52271 <= s_97_0
        fn_state.gs_52271 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#52271:u8
        let s_98_0: bool = fn_state.gs_52271;
        // N s_98_1: branch s_98_0 b145 b99
        if s_98_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #() : ()
        let s_99_0: () = ();
        // S s_99_1: call EL2Enabled(s_99_0)
        let s_99_1: bool = EL2Enabled(state, tracer, s_99_0);
        // N s_99_2: branch s_99_1 b144 b100
        if s_99_1 {
            return block_144(state, tracer, fn_state);
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
        // D s_100_1: write-var gs#52272 <= s_100_0
        fn_state.gs_52272 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#52272:u8
        let s_101_0: bool = fn_state.gs_52272;
        // N s_101_1: branch s_101_0 b140 b102
        if s_101_0 {
            return block_140(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#52274 <= s_102_0
        fn_state.gs_52274 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#52274:u8
        let s_103_0: bool = fn_state.gs_52274;
        // N s_103_1: branch s_103_0 b139 b104
        if s_103_0 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #0u : u8
        let s_104_0: bool = false;
        // D s_104_1: write-var gs#52275 <= s_104_0
        fn_state.gs_52275 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#52275:u8
        let s_105_0: bool = fn_state.gs_52275;
        // N s_105_1: branch s_105_0 b138 b106
        if s_105_0 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #424u : u32
        let s_106_0: u32 = 424;
        // D s_106_1: read-reg s_106_0:u8
        let s_106_1: u8 = {
            let value = state.read_register::<u8>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // C s_106_2: const #2u : u8
        let s_106_2: u8 = 2;
        // D s_106_3: cmp-lt s_106_1 s_106_2
        let s_106_3: bool = ((s_106_1) < (s_106_2));
        // N s_106_4: branch s_106_3 b137 b107
        if s_106_3 {
            return block_137(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#52276 <= s_107_0
        fn_state.gs_52276 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#52276:u8
        let s_108_0: bool = fn_state.gs_52276;
        // N s_108_1: branch s_108_0 b136 b109
        if s_108_0 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#52277 <= s_109_0
        fn_state.gs_52277 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#52277:u8
        let s_110_0: bool = fn_state.gs_52277;
        // N s_110_1: branch s_110_0 b130 b111
        if s_110_0 {
            return block_130(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #424u : u32
        let s_111_0: u32 = 424;
        // D s_111_1: read-reg s_111_0:u8
        let s_111_1: u8 = {
            let value = state.read_register::<u8>(s_111_0 as isize);
            tracer.read_register(s_111_0 as isize, value);
            value
        };
        // C s_111_2: const #2u : u8
        let s_111_2: u8 = 2;
        // D s_111_3: cmp-lt s_111_1 s_111_2
        let s_111_3: bool = ((s_111_1) < (s_111_2));
        // N s_111_4: branch s_111_3 b126 b112
        if s_111_3 {
            return block_126(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#52283 <= s_112_0
        fn_state.gs_52283 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#52283:u8
        let s_113_0: bool = fn_state.gs_52283;
        // N s_113_1: branch s_113_0 b125 b114
        if s_113_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #0u : u8
        let s_114_0: bool = false;
        // D s_114_1: write-var gs#52284 <= s_114_0
        fn_state.gs_52284 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#52284:u8
        let s_115_0: bool = fn_state.gs_52284;
        // N s_115_1: branch s_115_0 b119 b116
        if s_115_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #16536u : u32
        let s_116_0: u32 = 16536;
        // D s_116_1: read-reg s_116_0:struct
        let s_116_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // D s_116_2: call _get_BRBFCR_EL1_Type_BANK(s_116_1)
        let s_116_2: u8 = u_get_BRBFCR_EL1_Type_BANK(state, tracer, s_116_1);
        // D s_116_3: cast zx s_116_2 -> bv
        let s_116_3: Bits = Bits::new(s_116_2 as u128, 2u16);
        // D s_116_4: cast zx s_116_3 -> i
        let s_116_4: i128 = (s_116_3.value() as i128);
        // D s_116_5: cast reint s_116_4 -> i64
        let s_116_5: i64 = (s_116_4 as i64);
        // C s_116_6: const #32s : i
        let s_116_6: i128 = 32;
        // D s_116_7: cast zx s_116_5 -> i
        let s_116_7: i128 = (i128::try_from(s_116_5).unwrap());
        // D s_116_8: mul s_116_7 s_116_6
        let s_116_8: i128 = ((s_116_7) * (s_116_6));
        // D s_116_9: cast reint s_116_8 -> i64
        let s_116_9: i64 = (s_116_8 as i64);
        // C s_116_10: const #28s : i
        let s_116_10: i128 = 28;
        // D s_116_11: cast zx s_116_9 -> i
        let s_116_11: i128 = (i128::try_from(s_116_9).unwrap());
        // D s_116_12: add s_116_10 s_116_11
        let s_116_12: i128 = (s_116_10 + s_116_11);
        // D s_116_13: cast reint s_116_12 -> i64
        let s_116_13: i64 = (s_116_12 as i64);
        // C s_116_14: const #64s : i
        let s_116_14: i128 = 64;
        // D s_116_15: cast zx s_116_13 -> i
        let s_116_15: i128 = (i128::try_from(s_116_13).unwrap());
        // D s_116_16: cmp-ge s_116_15 s_116_14
        let s_116_16: bool = ((s_116_15) >= (s_116_14));
        // N s_116_17: branch s_116_16 b118 b117
        if s_116_16 {
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
        // C s_117_1: const #28s : i
        let s_117_1: i128 = 28;
        // C s_117_2: const #22120u : u32
        let s_117_2: u32 = 22120;
        // D s_117_3: read-reg s_117_2:[struct; 32]
        let s_117_3: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_117_2 as isize);
            tracer.read_register(s_117_2 as isize, value);
            value
        };
        // D s_117_4: read-element s_117_3[s_117_1]
        let s_117_4: ProductType5c790c8ef59cc8b2 = s_117_3[(s_117_1) as usize];
        // D s_117_5: call __get_BRBINF_EL1(s_117_4)
        let s_117_5: ProductType5c790c8ef59cc8b2 = u__get_BRBINF_EL1(
            state,
            tracer,
            s_117_4,
        );
        // D s_117_6: write-var ga#48007 <= s_117_5
        fn_state.ga_48007 = s_117_5;
        // D s_117_7: read-var ga#48007.0:struct
        let s_117_7: u64 = fn_state.ga_48007._0;
        // D s_117_8: cast zx s_117_7 -> bv
        let s_117_8: Bits = Bits::new(s_117_7 as u128, 64u16);
        // D s_117_9: read-var t:i
        let s_117_9: i128 = fn_state.t;
        // D s_117_10: call X_set(s_117_9, s_117_0, s_117_8)
        let s_117_10: () = X_set(state, tracer, s_117_9, s_117_0, s_117_8);
        // N s_117_11: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #64s : i64
        let s_118_0: i64 = 64;
        // C s_118_1: const #64s : i
        let s_118_1: i128 = 64;
        // S s_118_2: call Zeros(s_118_1)
        let s_118_2: Bits = Zeros(state, tracer, s_118_1);
        // S s_118_3: cast reint s_118_2 -> u64
        let s_118_3: u64 = (s_118_2.value() as u64);
        // S s_118_4: cast zx s_118_3 -> bv
        let s_118_4: Bits = Bits::new(s_118_3 as u128, 64u16);
        // D s_118_5: read-var t:i
        let s_118_5: i128 = fn_state.t;
        // D s_118_6: call X_set(s_118_5, s_118_0, s_118_4)
        let s_118_6: () = X_set(state, tracer, s_118_5, s_118_0, s_118_4);
        // N s_118_7: return
        return;
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #() : ()
        let s_119_0: () = ();
        // S s_119_1: call Halted(s_119_0)
        let s_119_1: bool = Halted(state, tracer, s_119_0);
        // N s_119_2: branch s_119_1 b124 b120
        if s_119_1 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #0u : u8
        let s_120_0: bool = false;
        // D s_120_1: write-var gs#52292 <= s_120_0
        fn_state.gs_52292 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#52292:u8
        let s_121_0: bool = fn_state.gs_52292;
        // N s_121_1: branch s_121_0 b123 b122
        if s_121_0 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #24u : u8
        let s_122_0: u8 = 24;
        // C s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 8u16);
        // C s_122_2: cast zx s_122_1 -> i
        let s_122_2: i128 = (s_122_1.value() as i128);
        // C s_122_3: cast reint s_122_2 -> i64
        let s_122_3: i64 = (s_122_2 as i64);
        // C s_122_4: cast zx s_122_3 -> i
        let s_122_4: i128 = (i128::try_from(s_122_3).unwrap());
        // C s_122_5: const #424u : u32
        let s_122_5: u32 = 424;
        // D s_122_6: read-reg s_122_5:u8
        let s_122_6: u8 = {
            let value = state.read_register::<u8>(s_122_5 as isize);
            tracer.read_register(s_122_5 as isize, value);
            value
        };
        // D s_122_7: call AArch64_SystemAccessTrap(s_122_6, s_122_4)
        let s_122_7: () = AArch64_SystemAccessTrap(state, tracer, s_122_6, s_122_4);
        // N s_122_8: return
        return;
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
        // D s_124_0: read-var __EDSCR_SDD:u8
        let s_124_0: bool = fn_state.u__EDSCR_SDD;
        // D s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 1u16);
        // C s_124_2: const #1u : u8
        let s_124_2: bool = true;
        // C s_124_3: cast zx s_124_2 -> bv
        let s_124_3: Bits = Bits::new(s_124_2 as u128, 1u16);
        // D s_124_4: cmp-eq s_124_1 s_124_3
        let s_124_4: bool = ((s_124_1) == (s_124_3));
        // D s_124_5: write-var gs#52292 <= s_124_4
        fn_state.gs_52292 = s_124_4;
        // N s_124_6: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var __SCR_EL3_NS:u8
        let s_125_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_125_1: cast zx s_125_0 -> bv
        let s_125_1: Bits = Bits::new(s_125_0 as u128, 1u16);
        // C s_125_2: const #1u : u8
        let s_125_2: bool = true;
        // C s_125_3: cast zx s_125_2 -> bv
        let s_125_3: Bits = Bits::new(s_125_2 as u128, 1u16);
        // D s_125_4: cmp-eq s_125_1 s_125_3
        let s_125_4: bool = ((s_125_1) == (s_125_3));
        // D s_125_5: write-var gs#52284 <= s_125_4
        fn_state.gs_52284 = s_125_4;
        // N s_125_6: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var __MDCR_EL3_SBRBE:u8
        let s_126_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_126_1: const #0s : i
        let s_126_1: i128 = 0;
        // D s_126_2: cast zx s_126_0 -> bv
        let s_126_2: Bits = Bits::new(s_126_0 as u128, 2u16);
        // C s_126_3: const #1s : i64
        let s_126_3: i64 = 1;
        // C s_126_4: cast zx s_126_3 -> i
        let s_126_4: i128 = (i128::try_from(s_126_3).unwrap());
        // C s_126_5: const #0s : i
        let s_126_5: i128 = 0;
        // C s_126_6: add s_126_5 s_126_4
        let s_126_6: i128 = (s_126_5 + s_126_4);
        // D s_126_7: bit-extract s_126_2 s_126_1 s_126_6
        let s_126_7: Bits = (Bits::new(
            ((s_126_2) >> (s_126_1)).value(),
            u16::try_from(s_126_6).unwrap(),
        ));
        // D s_126_8: cast reint s_126_7 -> u8
        let s_126_8: bool = ((s_126_7.value()) != 0);
        // D s_126_9: cast zx s_126_8 -> bv
        let s_126_9: Bits = Bits::new(s_126_8 as u128, 1u16);
        // C s_126_10: const #0u : u8
        let s_126_10: bool = false;
        // C s_126_11: cast zx s_126_10 -> bv
        let s_126_11: Bits = Bits::new(s_126_10 as u128, 1u16);
        // D s_126_12: cmp-eq s_126_9 s_126_11
        let s_126_12: bool = ((s_126_9) == (s_126_11));
        // D s_126_13: not s_126_12
        let s_126_13: bool = !s_126_12;
        // N s_126_14: branch s_126_13 b129 b127
        if s_126_13 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #1u : u8
        let s_127_0: bool = true;
        // D s_127_1: write-var gs#52278 <= s_127_0
        fn_state.gs_52278 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#52278:u8
        let s_128_0: bool = fn_state.gs_52278;
        // D s_128_1: write-var gs#52283 <= s_128_0
        fn_state.gs_52283 = s_128_0;
        // N s_128_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #0u : u8
        let s_129_0: bool = false;
        // D s_129_1: write-var gs#52278 <= s_129_0
        fn_state.gs_52278 = s_129_0;
        // N s_129_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #() : ()
        let s_130_0: () = ();
        // S s_130_1: call Halted(s_130_0)
        let s_130_1: bool = Halted(state, tracer, s_130_0);
        // N s_130_2: branch s_130_1 b135 b131
        if s_130_1 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #0u : u8
        let s_131_0: bool = false;
        // D s_131_1: write-var gs#52293 <= s_131_0
        fn_state.gs_52293 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#52293:u8
        let s_132_0: bool = fn_state.gs_52293;
        // N s_132_1: branch s_132_0 b134 b133
        if s_132_0 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #24u : u8
        let s_133_0: u8 = 24;
        // C s_133_1: cast zx s_133_0 -> bv
        let s_133_1: Bits = Bits::new(s_133_0 as u128, 8u16);
        // C s_133_2: cast zx s_133_1 -> i
        let s_133_2: i128 = (s_133_1.value() as i128);
        // C s_133_3: cast reint s_133_2 -> i64
        let s_133_3: i64 = (s_133_2 as i64);
        // C s_133_4: cast zx s_133_3 -> i
        let s_133_4: i128 = (i128::try_from(s_133_3).unwrap());
        // C s_133_5: const #424u : u32
        let s_133_5: u32 = 424;
        // D s_133_6: read-reg s_133_5:u8
        let s_133_6: u8 = {
            let value = state.read_register::<u8>(s_133_5 as isize);
            tracer.read_register(s_133_5 as isize, value);
            value
        };
        // D s_133_7: call AArch64_SystemAccessTrap(s_133_6, s_133_4)
        let s_133_7: () = AArch64_SystemAccessTrap(state, tracer, s_133_6, s_133_4);
        // N s_133_8: return
        return;
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_134_0: panic
        panic!("{:?}", ());
        // N s_134_1: return
        return;
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var __EDSCR_SDD:u8
        let s_135_0: bool = fn_state.u__EDSCR_SDD;
        // D s_135_1: cast zx s_135_0 -> bv
        let s_135_1: Bits = Bits::new(s_135_0 as u128, 1u16);
        // C s_135_2: const #1u : u8
        let s_135_2: bool = true;
        // C s_135_3: cast zx s_135_2 -> bv
        let s_135_3: Bits = Bits::new(s_135_2 as u128, 1u16);
        // D s_135_4: cmp-eq s_135_1 s_135_3
        let s_135_4: bool = ((s_135_1) == (s_135_3));
        // D s_135_5: write-var gs#52293 <= s_135_4
        fn_state.gs_52293 = s_135_4;
        // N s_135_6: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var __SCR_EL3_NS:u8
        let s_136_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 1u16);
        // C s_136_2: const #0u : u8
        let s_136_2: bool = false;
        // C s_136_3: cast zx s_136_2 -> bv
        let s_136_3: Bits = Bits::new(s_136_2 as u128, 1u16);
        // D s_136_4: cmp-eq s_136_1 s_136_3
        let s_136_4: bool = ((s_136_1) == (s_136_3));
        // D s_136_5: write-var gs#52277 <= s_136_4
        fn_state.gs_52277 = s_136_4;
        // N s_136_6: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var __MDCR_EL3_SBRBE:u8
        let s_137_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 2u16);
        // C s_137_2: const #3u : u8
        let s_137_2: u8 = 3;
        // C s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 2u16);
        // D s_137_4: cmp-ne s_137_1 s_137_3
        let s_137_4: bool = ((s_137_1) != (s_137_3));
        // D s_137_5: write-var gs#52276 <= s_137_4
        fn_state.gs_52276 = s_137_4;
        // N s_137_6: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #24u : u8
        let s_138_0: u8 = 24;
        // C s_138_1: cast zx s_138_0 -> bv
        let s_138_1: Bits = Bits::new(s_138_0 as u128, 8u16);
        // C s_138_2: cast zx s_138_1 -> i
        let s_138_2: i128 = (s_138_1.value() as i128);
        // C s_138_3: cast reint s_138_2 -> i64
        let s_138_3: i64 = (s_138_2 as i64);
        // C s_138_4: cast zx s_138_3 -> i
        let s_138_4: i128 = (i128::try_from(s_138_3).unwrap());
        // C s_138_5: const #432u : u32
        let s_138_5: u32 = 432;
        // D s_138_6: read-reg s_138_5:u8
        let s_138_6: u8 = {
            let value = state.read_register::<u8>(s_138_5 as isize);
            tracer.read_register(s_138_5 as isize, value);
            value
        };
        // D s_138_7: call AArch64_SystemAccessTrap(s_138_6, s_138_4)
        let s_138_7: () = AArch64_SystemAccessTrap(state, tracer, s_138_6, s_138_4);
        // N s_138_8: return
        return;
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var __HDFGRTR_EL2_nBRBDATA:u8
        let s_139_0: bool = fn_state.u__HDFGRTR_EL2_nBRBDATA;
        // D s_139_1: cast zx s_139_0 -> bv
        let s_139_1: Bits = Bits::new(s_139_0 as u128, 1u16);
        // C s_139_2: const #0u : u8
        let s_139_2: bool = false;
        // C s_139_3: cast zx s_139_2 -> bv
        let s_139_3: Bits = Bits::new(s_139_2 as u128, 1u16);
        // D s_139_4: cmp-eq s_139_1 s_139_3
        let s_139_4: bool = ((s_139_1) == (s_139_3));
        // D s_139_5: write-var gs#52275 <= s_139_4
        fn_state.gs_52275 = s_139_4;
        // N s_139_6: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #424u : u32
        let s_140_0: u32 = 424;
        // D s_140_1: read-reg s_140_0:u8
        let s_140_1: u8 = {
            let value = state.read_register::<u8>(s_140_0 as isize);
            tracer.read_register(s_140_0 as isize, value);
            value
        };
        // C s_140_2: const #2u : u8
        let s_140_2: u8 = 2;
        // D s_140_3: cmp-lt s_140_1 s_140_2
        let s_140_3: bool = ((s_140_1) < (s_140_2));
        // D s_140_4: not s_140_3
        let s_140_4: bool = !s_140_3;
        // N s_140_5: branch s_140_4 b143 b141
        if s_140_4 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var __SCR_EL3_FGTEn:u8
        let s_141_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_141_1: cast zx s_141_0 -> bv
        let s_141_1: Bits = Bits::new(s_141_0 as u128, 1u16);
        // C s_141_2: const #1u : u8
        let s_141_2: bool = true;
        // C s_141_3: cast zx s_141_2 -> bv
        let s_141_3: Bits = Bits::new(s_141_2 as u128, 1u16);
        // D s_141_4: cmp-eq s_141_1 s_141_3
        let s_141_4: bool = ((s_141_1) == (s_141_3));
        // D s_141_5: write-var gs#52273 <= s_141_4
        fn_state.gs_52273 = s_141_4;
        // N s_141_6: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#52273:u8
        let s_142_0: bool = fn_state.gs_52273;
        // D s_142_1: write-var gs#52274 <= s_142_0
        fn_state.gs_52274 = s_142_0;
        // N s_142_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #1u : u8
        let s_143_0: bool = true;
        // D s_143_1: write-var gs#52273 <= s_143_0
        fn_state.gs_52273 = s_143_0;
        // N s_143_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #146u : u32
        let s_144_0: u32 = 146;
        // S s_144_1: call IsFeatureImplemented(s_144_0)
        let s_144_1: bool = IsFeatureImplemented(state, tracer, s_144_0);
        // D s_144_2: write-var gs#52272 <= s_144_1
        fn_state.gs_52272 = s_144_1;
        // N s_144_3: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_145_0: panic
        panic!("{:?}", ());
        // N s_145_1: return
        return;
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var __SCR_EL3_NS:u8
        let s_146_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 1u16);
        // C s_146_2: const #1u : u8
        let s_146_2: bool = true;
        // C s_146_3: cast zx s_146_2 -> bv
        let s_146_3: Bits = Bits::new(s_146_2 as u128, 1u16);
        // D s_146_4: cmp-eq s_146_1 s_146_3
        let s_146_4: bool = ((s_146_1) == (s_146_3));
        // D s_146_5: write-var gs#52271 <= s_146_4
        fn_state.gs_52271 = s_146_4;
        // N s_146_6: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var __MDCR_EL3_SBRBE:u8
        let s_147_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // C s_147_1: const #0s : i
        let s_147_1: i128 = 0;
        // D s_147_2: cast zx s_147_0 -> bv
        let s_147_2: Bits = Bits::new(s_147_0 as u128, 2u16);
        // C s_147_3: const #1s : i64
        let s_147_3: i64 = 1;
        // C s_147_4: cast zx s_147_3 -> i
        let s_147_4: i128 = (i128::try_from(s_147_3).unwrap());
        // C s_147_5: const #0s : i
        let s_147_5: i128 = 0;
        // C s_147_6: add s_147_5 s_147_4
        let s_147_6: i128 = (s_147_5 + s_147_4);
        // D s_147_7: bit-extract s_147_2 s_147_1 s_147_6
        let s_147_7: Bits = (Bits::new(
            ((s_147_2) >> (s_147_1)).value(),
            u16::try_from(s_147_6).unwrap(),
        ));
        // D s_147_8: cast reint s_147_7 -> u8
        let s_147_8: bool = ((s_147_7.value()) != 0);
        // D s_147_9: cast zx s_147_8 -> bv
        let s_147_9: Bits = Bits::new(s_147_8 as u128, 1u16);
        // C s_147_10: const #0u : u8
        let s_147_10: bool = false;
        // C s_147_11: cast zx s_147_10 -> bv
        let s_147_11: Bits = Bits::new(s_147_10 as u128, 1u16);
        // D s_147_12: cmp-eq s_147_9 s_147_11
        let s_147_12: bool = ((s_147_9) == (s_147_11));
        // D s_147_13: not s_147_12
        let s_147_13: bool = !s_147_12;
        // N s_147_14: branch s_147_13 b150 b148
        if s_147_13 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #1u : u8
        let s_148_0: bool = true;
        // D s_148_1: write-var gs#52265 <= s_148_0
        fn_state.gs_52265 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#52265:u8
        let s_149_0: bool = fn_state.gs_52265;
        // D s_149_1: write-var gs#52270 <= s_149_0
        fn_state.gs_52270 = s_149_0;
        // N s_149_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #0u : u8
        let s_150_0: bool = false;
        // D s_150_1: write-var gs#52265 <= s_150_0
        fn_state.gs_52265 = s_150_0;
        // N s_150_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_151_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_151_1: call __IMPDEF_boolean(s_151_0)
        let s_151_1: bool = u__IMPDEF_boolean(state, tracer, s_151_0);
        // D s_151_2: write-var gs#52264 <= s_151_1
        fn_state.gs_52264 = s_151_1;
        // N s_151_3: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var __EDSCR_SDD:u8
        let s_152_0: bool = fn_state.u__EDSCR_SDD;
        // D s_152_1: cast zx s_152_0 -> bv
        let s_152_1: Bits = Bits::new(s_152_0 as u128, 1u16);
        // C s_152_2: const #1u : u8
        let s_152_2: bool = true;
        // C s_152_3: cast zx s_152_2 -> bv
        let s_152_3: Bits = Bits::new(s_152_2 as u128, 1u16);
        // D s_152_4: cmp-eq s_152_1 s_152_3
        let s_152_4: bool = ((s_152_1) == (s_152_3));
        // D s_152_5: write-var gs#52263 <= s_152_4
        fn_state.gs_52263 = s_152_4;
        // N s_152_6: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #424u : u32
        let s_153_0: u32 = 424;
        // D s_153_1: read-reg s_153_0:u8
        let s_153_1: u8 = {
            let value = state.read_register::<u8>(s_153_0 as isize);
            tracer.read_register(s_153_0 as isize, value);
            value
        };
        // C s_153_2: const #2u : u8
        let s_153_2: u8 = 2;
        // D s_153_3: cmp-lt s_153_1 s_153_2
        let s_153_3: bool = ((s_153_1) < (s_153_2));
        // D s_153_4: write-var gs#52262 <= s_153_3
        fn_state.gs_52262 = s_153_3;
        // N s_153_5: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_154_0: panic
        panic!("{:?}", ());
        // N s_154_1: return
        return;
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var __SCR_EL3_NS:u8
        let s_155_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_155_1: cast zx s_155_0 -> bv
        let s_155_1: Bits = Bits::new(s_155_0 as u128, 1u16);
        // C s_155_2: const #0u : u8
        let s_155_2: bool = false;
        // C s_155_3: cast zx s_155_2 -> bv
        let s_155_3: Bits = Bits::new(s_155_2 as u128, 1u16);
        // D s_155_4: cmp-eq s_155_1 s_155_3
        let s_155_4: bool = ((s_155_1) == (s_155_3));
        // D s_155_5: write-var gs#52261 <= s_155_4
        fn_state.gs_52261 = s_155_4;
        // N s_155_6: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var __MDCR_EL3_SBRBE:u8
        let s_156_0: u8 = fn_state.u__MDCR_EL3_SBRBE;
        // D s_156_1: cast zx s_156_0 -> bv
        let s_156_1: Bits = Bits::new(s_156_0 as u128, 2u16);
        // C s_156_2: const #3u : u8
        let s_156_2: u8 = 3;
        // C s_156_3: cast zx s_156_2 -> bv
        let s_156_3: Bits = Bits::new(s_156_2 as u128, 2u16);
        // D s_156_4: cmp-ne s_156_1 s_156_3
        let s_156_4: bool = ((s_156_1) != (s_156_3));
        // D s_156_5: write-var gs#52260 <= s_156_4
        fn_state.gs_52260 = s_156_4;
        // N s_156_6: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_157_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_157_1: call __IMPDEF_boolean(s_157_0)
        let s_157_1: bool = u__IMPDEF_boolean(state, tracer, s_157_0);
        // D s_157_2: write-var gs#52259 <= s_157_1
        fn_state.gs_52259 = s_157_1;
        // N s_157_3: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var __EDSCR_SDD:u8
        let s_158_0: bool = fn_state.u__EDSCR_SDD;
        // D s_158_1: cast zx s_158_0 -> bv
        let s_158_1: Bits = Bits::new(s_158_0 as u128, 1u16);
        // C s_158_2: const #1u : u8
        let s_158_2: bool = true;
        // C s_158_3: cast zx s_158_2 -> bv
        let s_158_3: Bits = Bits::new(s_158_2 as u128, 1u16);
        // D s_158_4: cmp-eq s_158_1 s_158_3
        let s_158_4: bool = ((s_158_1) == (s_158_3));
        // D s_158_5: write-var gs#52258 <= s_158_4
        fn_state.gs_52258 = s_158_4;
        // N s_158_6: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #424u : u32
        let s_159_0: u32 = 424;
        // D s_159_1: read-reg s_159_0:u8
        let s_159_1: u8 = {
            let value = state.read_register::<u8>(s_159_0 as isize);
            tracer.read_register(s_159_0 as isize, value);
            value
        };
        // C s_159_2: const #2u : u8
        let s_159_2: u8 = 2;
        // D s_159_3: cmp-lt s_159_1 s_159_2
        let s_159_3: bool = ((s_159_1) < (s_159_2));
        // D s_159_4: write-var gs#52257 <= s_159_3
        fn_state.gs_52257 = s_159_3;
        // N s_159_5: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_160_0: panic
        panic!("{:?}", ());
        // N s_160_1: return
        return;
    }
}
