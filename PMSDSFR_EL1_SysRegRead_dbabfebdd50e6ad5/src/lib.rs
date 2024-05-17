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
use u_get_HDFGRTR2_EL2_Type_nPMSDSFR_EL1::*;
use Halted::*;
use u_get_MDCR_EL3_Type_NSPB::*;
use AArch64_SystemAccessTrap::*;
use u__get_PMSDSFR_EL1::*;
use IsFeatureImplemented::*;
use u_get_SCR_EL3_Type_NS::*;
use u__IMPDEF_boolean::*;
use NVMem_read::*;
use X_set::*;
use u_get_MDCR_EL3_Type_EnPMS3::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_SCR_EL3_Type_NSE::*;
use u_get_MDCR_EL3_Type_NSPBE::*;
use u_get_SCR_EL3_Type_FGTEn2::*;
use u_get_MDCR_EL2_Type_TPMS::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn PMSDSFR_EL1_SysRegRead_dbabfebdd50e6ad5<T: Tracer>(
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
        gs_68312: bool,
        u__MDCR_EL3_EnPMS3: bool,
        gs_68284: bool,
        gs_68280: bool,
        gs_68252: bool,
        ga_82608: u64,
        gs_68290: bool,
        gs_68272: bool,
        u__MDCR_EL3_NSPBE: bool,
        gs_68279: bool,
        gs_68283: bool,
        gs_68292: bool,
        u__SCR_EL3_FGTEn2: bool,
        gs_68275: bool,
        gs_68295: bool,
        gs_68299: bool,
        gs_68271: bool,
        gs_68262: bool,
        gs_68300: bool,
        u__PSTATE_EL: u8,
        gs_68308: bool,
        gs_68291: bool,
        gs_68254: bool,
        ga_82609: ProductType5c790c8ef59cc8b2,
        gs_68313: bool,
        gs_68306: bool,
        gs_68307: bool,
        gs_68265: bool,
        ga_82665: ProductType5c790c8ef59cc8b2,
        u__HDFGRTR2_EL2_nPMSDSFR_EL1: bool,
        gs_68285: bool,
        gs_68305: bool,
        gs_68281: bool,
        gs_68294: bool,
        gs_68260: bool,
        gs_68264: bool,
        u__MDCR_EL2_TPMS: bool,
        gs_68297: bool,
        gs_68309: bool,
        gs_68293: bool,
        gs_68263: bool,
        gs_68282: bool,
        gs_68253: bool,
        gs_68261: bool,
        gs_68259: bool,
        gs_68278: bool,
        gs_68298: bool,
        gs_68277: bool,
        gs_68296: bool,
        gs_68274: bool,
        u__SCR_EL3_NSE: bool,
        ga_82669: ProductType5c790c8ef59cc8b2,
        gs_68266: bool,
        gs_68273: bool,
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
        // D s_0_5: call _get_MDCR_EL3_Type_EnPMS3(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_EnPMS3(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_EnPMS3 <= s_0_5
        fn_state.u__MDCR_EL3_EnPMS3 = s_0_5;
        // C s_0_7: const #22712u : u32
        let s_0_7: u32 = 22712;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDCR_EL3_Type_NSPBE(s_0_8)
        let s_0_9: bool = u_get_MDCR_EL3_Type_NSPBE(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL3_NSPBE <= s_0_9
        fn_state.u__MDCR_EL3_NSPBE = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_NSE(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_NSE(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_NSE <= s_0_13
        fn_state.u__SCR_EL3_NSE = s_0_13;
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
        // C s_0_19: const #101224u : u32
        let s_0_19: u32 = 101224;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HDFGRTR2_EL2_Type_nPMSDSFR_EL1(s_0_20)
        let s_0_21: bool = u_get_HDFGRTR2_EL2_Type_nPMSDSFR_EL1(state, tracer, s_0_20);
        // D s_0_22: write-var __HDFGRTR2_EL2_nPMSDSFR_EL1 <= s_0_21
        fn_state.u__HDFGRTR2_EL2_nPMSDSFR_EL1 = s_0_21;
        // C s_0_23: const #104880u : u32
        let s_0_23: u32 = 104880;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_MDCR_EL2_Type_TPMS(s_0_24)
        let s_0_25: bool = u_get_MDCR_EL2_Type_TPMS(state, tracer, s_0_24);
        // D s_0_26: write-var __MDCR_EL2_TPMS <= s_0_25
        fn_state.u__MDCR_EL2_TPMS = s_0_25;
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
        // N s_0_33: branch s_0_32 b170 b1
        if s_0_32 {
            return block_170(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b73 b2
        if s_1_5 {
            return block_73(state, tracer, fn_state);
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
        // C s_5_1: const #1520u : u32
        let s_5_1: u32 = 1520;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_PMSDSFR_EL1(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_PMSDSFR_EL1(
            state,
            tracer,
            s_5_2,
        );
        // D s_5_4: write-var ga#82669 <= s_5_3
        fn_state.ga_82669 = s_5_3;
        // D s_5_5: read-var ga#82669.0:struct
        let s_5_5: u64 = fn_state.ga_82669._0;
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
        // N s_6_2: branch s_6_1 b72 b7
        if s_6_1 {
            return block_72(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#68252 <= s_7_0
        fn_state.gs_68252 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#68252:u8
        let s_8_0: bool = fn_state.gs_68252;
        // N s_8_1: branch s_8_0 b71 b9
        if s_8_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#68253 <= s_9_0
        fn_state.gs_68253 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#68253:u8
        let s_10_0: bool = fn_state.gs_68253;
        // N s_10_1: branch s_10_0 b70 b11
        if s_10_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#68254 <= s_11_0
        fn_state.gs_68254 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#68254:u8
        let s_12_0: bool = fn_state.gs_68254;
        // N s_12_1: branch s_12_0 b60 b13
        if s_12_0 {
            return block_60(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#68262 <= s_13_0
        fn_state.gs_68262 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#68262:u8
        let s_14_0: bool = fn_state.gs_68262;
        // N s_14_1: branch s_14_0 b59 b15
        if s_14_0 {
            return block_59(state, tracer, fn_state);
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
        // N s_15_2: branch s_15_1 b58 b16
        if s_15_1 {
            return block_58(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#68263 <= s_16_0
        fn_state.gs_68263 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#68263:u8
        let s_17_0: bool = fn_state.gs_68263;
        // N s_17_1: branch s_17_0 b57 b18
        if s_17_0 {
            return block_57(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#68264 <= s_18_0
        fn_state.gs_68264 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#68264:u8
        let s_19_0: bool = fn_state.gs_68264;
        // N s_19_1: branch s_19_0 b56 b20
        if s_19_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#68265 <= s_20_0
        fn_state.gs_68265 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#68265:u8
        let s_21_0: bool = fn_state.gs_68265;
        // N s_21_1: branch s_21_0 b55 b22
        if s_21_0 {
            return block_55(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#68266 <= s_22_0
        fn_state.gs_68266 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#68266:u8
        let s_23_0: bool = fn_state.gs_68266;
        // N s_23_1: branch s_23_0 b54 b24
        if s_23_0 {
            return block_54(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#68274 <= s_25_0
        fn_state.gs_68274 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#68274:u8
        let s_26_0: bool = fn_state.gs_68274;
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
        // D s_28_1: write-var gs#68275 <= s_28_0
        fn_state.gs_68275 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#68275:u8
        let s_29_0: bool = fn_state.gs_68275;
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
        // C s_30_0: const #64s : i64
        let s_30_0: i64 = 64;
        // C s_30_1: const #1520u : u32
        let s_30_1: u32 = 1520;
        // D s_30_2: read-reg s_30_1:struct
        let s_30_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_30_1 as isize);
            tracer.read_register(s_30_1 as isize, value);
            value
        };
        // D s_30_3: call __get_PMSDSFR_EL1(s_30_2)
        let s_30_3: ProductType5c790c8ef59cc8b2 = u__get_PMSDSFR_EL1(
            state,
            tracer,
            s_30_2,
        );
        // D s_30_4: write-var ga#82665 <= s_30_3
        fn_state.ga_82665 = s_30_3;
        // D s_30_5: read-var ga#82665.0:struct
        let s_30_5: u64 = fn_state.ga_82665._0;
        // D s_30_6: cast zx s_30_5 -> bv
        let s_30_6: Bits = Bits::new(s_30_5 as u128, 64u16);
        // D s_30_7: read-var t:i
        let s_30_7: i128 = fn_state.t;
        // D s_30_8: call X_set(s_30_7, s_30_0, s_30_6)
        let s_30_8: () = X_set(state, tracer, s_30_7, s_30_0, s_30_6);
        // N s_30_9: return
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
        // D s_32_1: write-var gs#68277 <= s_32_0
        fn_state.gs_68277 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#68277:u8
        let s_33_0: bool = fn_state.gs_68277;
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
        // D s_36_7: write-var gs#68277 <= s_36_6
        fn_state.gs_68277 = s_36_6;
        // N s_36_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var __MDCR_EL3_EnPMS3:u8
        let s_37_0: bool = fn_state.u__MDCR_EL3_EnPMS3;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #0u : u8
        let s_37_2: bool = false;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#68275 <= s_37_4
        fn_state.gs_68275 = s_37_4;
        // N s_37_6: jump b29
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
        // D s_39_1: write-var gs#68278 <= s_39_0
        fn_state.gs_68278 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#68278:u8
        let s_40_0: bool = fn_state.gs_68278;
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
        // D s_43_7: write-var gs#68278 <= s_43_6
        fn_state.gs_68278 = s_43_6;
        // N s_43_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #22712u : u32
        let s_44_0: u32 = 22712;
        // D s_44_1: read-reg s_44_0:struct
        let s_44_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call _get_MDCR_EL3_Type_NSPB(s_44_1)
        let s_44_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_44_1);
        // C s_44_3: const #0s : i
        let s_44_3: i128 = 0;
        // D s_44_4: cast zx s_44_2 -> bv
        let s_44_4: Bits = Bits::new(s_44_2 as u128, 2u16);
        // C s_44_5: const #1u : u64
        let s_44_5: u64 = 1;
        // D s_44_6: bit-extract s_44_4 s_44_3 s_44_5
        let s_44_6: Bits = (Bits::new(
            ((s_44_4) >> (s_44_3)).value(),
            u16::try_from(s_44_5).unwrap(),
        ));
        // D s_44_7: cast reint s_44_6 -> u8
        let s_44_7: bool = ((s_44_6.value()) != 0);
        // C s_44_8: const #0s : i
        let s_44_8: i128 = 0;
        // C s_44_9: const #0u : u64
        let s_44_9: u64 = 0;
        // D s_44_10: cast zx s_44_7 -> u64
        let s_44_10: u64 = (s_44_7 as u64);
        // C s_44_11: const #1u : u64
        let s_44_11: u64 = 1;
        // D s_44_12: and s_44_10 s_44_11
        let s_44_12: u64 = ((s_44_10) & (s_44_11));
        // D s_44_13: cmp-eq s_44_12 s_44_11
        let s_44_13: bool = ((s_44_12) == (s_44_11));
        // D s_44_14: lsl s_44_10 s_44_8
        let s_44_14: u64 = s_44_10 << s_44_8;
        // D s_44_15: or s_44_9 s_44_14
        let s_44_15: u64 = ((s_44_9) | (s_44_14));
        // D s_44_16: cmpl s_44_14
        let s_44_16: u64 = !s_44_14;
        // D s_44_17: and s_44_9 s_44_16
        let s_44_17: u64 = ((s_44_9) & (s_44_16));
        // D s_44_18: select s_44_13 s_44_15 s_44_17
        let s_44_18: u64 = if s_44_13 { s_44_15 } else { s_44_17 };
        // D s_44_19: cast trunc s_44_18 -> u8
        let s_44_19: bool = ((s_44_18) != 0);
        // D s_44_20: cast zx s_44_19 -> bv
        let s_44_20: Bits = Bits::new(s_44_19 as u128, 1u16);
        // C s_44_21: const #0u : u8
        let s_44_21: bool = false;
        // C s_44_22: cast zx s_44_21 -> bv
        let s_44_22: Bits = Bits::new(s_44_21 as u128, 1u16);
        // D s_44_23: cmp-eq s_44_20 s_44_22
        let s_44_23: bool = ((s_44_20) == (s_44_22));
        // N s_44_24: branch s_44_23 b53 b45
        if s_44_23 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #22712u : u32
        let s_45_0: u32 = 22712;
        // D s_45_1: read-reg s_45_0:struct
        let s_45_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call _get_MDCR_EL3_Type_NSPB(s_45_1)
        let s_45_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_45_1);
        // C s_45_3: const #1s : i
        let s_45_3: i128 = 1;
        // D s_45_4: cast zx s_45_2 -> bv
        let s_45_4: Bits = Bits::new(s_45_2 as u128, 2u16);
        // C s_45_5: const #1u : u64
        let s_45_5: u64 = 1;
        // D s_45_6: bit-extract s_45_4 s_45_3 s_45_5
        let s_45_6: Bits = (Bits::new(
            ((s_45_4) >> (s_45_3)).value(),
            u16::try_from(s_45_5).unwrap(),
        ));
        // D s_45_7: cast reint s_45_6 -> u8
        let s_45_7: bool = ((s_45_6.value()) != 0);
        // C s_45_8: const #0s : i
        let s_45_8: i128 = 0;
        // C s_45_9: const #0u : u64
        let s_45_9: u64 = 0;
        // D s_45_10: cast zx s_45_7 -> u64
        let s_45_10: u64 = (s_45_7 as u64);
        // C s_45_11: const #1u : u64
        let s_45_11: u64 = 1;
        // D s_45_12: and s_45_10 s_45_11
        let s_45_12: u64 = ((s_45_10) & (s_45_11));
        // D s_45_13: cmp-eq s_45_12 s_45_11
        let s_45_13: bool = ((s_45_12) == (s_45_11));
        // D s_45_14: lsl s_45_10 s_45_8
        let s_45_14: u64 = s_45_10 << s_45_8;
        // D s_45_15: or s_45_9 s_45_14
        let s_45_15: u64 = ((s_45_9) | (s_45_14));
        // D s_45_16: cmpl s_45_14
        let s_45_16: u64 = !s_45_14;
        // D s_45_17: and s_45_9 s_45_16
        let s_45_17: u64 = ((s_45_9) & (s_45_16));
        // D s_45_18: select s_45_13 s_45_15 s_45_17
        let s_45_18: u64 = if s_45_13 { s_45_15 } else { s_45_17 };
        // D s_45_19: cast trunc s_45_18 -> u8
        let s_45_19: bool = ((s_45_18) != 0);
        // C s_45_20: const #90704u : u32
        let s_45_20: u32 = 90704;
        // D s_45_21: read-reg s_45_20:struct
        let s_45_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_45_20 as isize);
            tracer.read_register(s_45_20 as isize, value);
            value
        };
        // D s_45_22: call _get_SCR_EL3_Type_NS(s_45_21)
        let s_45_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_45_21);
        // D s_45_23: cast zx s_45_19 -> bv
        let s_45_23: Bits = Bits::new(s_45_19 as u128, 1u16);
        // D s_45_24: cast zx s_45_22 -> bv
        let s_45_24: Bits = Bits::new(s_45_22 as u128, 1u16);
        // D s_45_25: cmp-ne s_45_23 s_45_24
        let s_45_25: bool = ((s_45_23) != (s_45_24));
        // D s_45_26: write-var gs#68271 <= s_45_25
        fn_state.gs_68271 = s_45_25;
        // N s_45_27: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#68271:u8
        let s_46_0: bool = fn_state.gs_68271;
        // N s_46_1: branch s_46_0 b52 b47
        if s_46_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #232u : u32
        let s_47_0: u32 = 232;
        // S s_47_1: call IsFeatureImplemented(s_47_0)
        let s_47_1: bool = IsFeatureImplemented(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b51 b48
        if s_47_1 {
            return block_51(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#68272 <= s_48_0
        fn_state.gs_68272 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#68272:u8
        let s_49_0: bool = fn_state.gs_68272;
        // D s_49_1: write-var gs#68273 <= s_49_0
        fn_state.gs_68273 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#68273:u8
        let s_50_0: bool = fn_state.gs_68273;
        // D s_50_1: write-var gs#68274 <= s_50_0
        fn_state.gs_68274 = s_50_0;
        // N s_50_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var __MDCR_EL3_NSPBE:u8
        let s_51_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // D s_51_2: read-var __SCR_EL3_NSE:u8
        let s_51_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-ne s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) != (s_51_3));
        // D s_51_5: write-var gs#68272 <= s_51_4
        fn_state.gs_68272 = s_51_4;
        // N s_51_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: write-var gs#68273 <= s_52_0
        fn_state.gs_68273 = s_52_0;
        // N s_52_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#68271 <= s_53_0
        fn_state.gs_68271 = s_53_0;
        // N s_53_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_54_0: panic
        panic!("{:?}", ());
        // N s_54_1: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var __MDCR_EL3_EnPMS3:u8
        let s_55_0: bool = fn_state.u__MDCR_EL3_EnPMS3;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 1u16);
        // C s_55_2: const #0u : u8
        let s_55_2: bool = false;
        // C s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // D s_55_4: cmp-eq s_55_1 s_55_3
        let s_55_4: bool = ((s_55_1) == (s_55_3));
        // D s_55_5: write-var gs#68266 <= s_55_4
        fn_state.gs_68266 = s_55_4;
        // N s_55_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_56_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_56_1: call __IMPDEF_boolean(s_56_0)
        let s_56_1: bool = u__IMPDEF_boolean(state, tracer, s_56_0);
        // D s_56_2: write-var gs#68265 <= s_56_1
        fn_state.gs_68265 = s_56_1;
        // N s_56_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call EDSCR_read(s_57_0)
        let s_57_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_57_0);
        // S s_57_2: call _get_EDSCR_Type_SDD(s_57_1)
        let s_57_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_57_1);
        // S s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // C s_57_4: const #1u : u8
        let s_57_4: bool = true;
        // C s_57_5: cast zx s_57_4 -> bv
        let s_57_5: Bits = Bits::new(s_57_4 as u128, 1u16);
        // S s_57_6: cmp-eq s_57_3 s_57_5
        let s_57_6: bool = ((s_57_3) == (s_57_5));
        // D s_57_7: write-var gs#68264 <= s_57_6
        fn_state.gs_68264 = s_57_6;
        // N s_57_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #424u : u32
        let s_58_0: u32 = 424;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: u8 = {
            let value = state.read_register::<u8>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // C s_58_2: const #2u : u8
        let s_58_2: u8 = 2;
        // D s_58_3: cmp-lt s_58_1 s_58_2
        let s_58_3: bool = ((s_58_1) < (s_58_2));
        // D s_58_4: write-var gs#68263 <= s_58_3
        fn_state.gs_68263 = s_58_3;
        // N s_58_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: panic
        panic!("{:?}", ());
        // N s_59_1: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #22712u : u32
        let s_60_0: u32 = 22712;
        // D s_60_1: read-reg s_60_0:struct
        let s_60_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call _get_MDCR_EL3_Type_NSPB(s_60_1)
        let s_60_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_60_1);
        // C s_60_3: const #0s : i
        let s_60_3: i128 = 0;
        // D s_60_4: cast zx s_60_2 -> bv
        let s_60_4: Bits = Bits::new(s_60_2 as u128, 2u16);
        // C s_60_5: const #1u : u64
        let s_60_5: u64 = 1;
        // D s_60_6: bit-extract s_60_4 s_60_3 s_60_5
        let s_60_6: Bits = (Bits::new(
            ((s_60_4) >> (s_60_3)).value(),
            u16::try_from(s_60_5).unwrap(),
        ));
        // D s_60_7: cast reint s_60_6 -> u8
        let s_60_7: bool = ((s_60_6.value()) != 0);
        // C s_60_8: const #0s : i
        let s_60_8: i128 = 0;
        // C s_60_9: const #0u : u64
        let s_60_9: u64 = 0;
        // D s_60_10: cast zx s_60_7 -> u64
        let s_60_10: u64 = (s_60_7 as u64);
        // C s_60_11: const #1u : u64
        let s_60_11: u64 = 1;
        // D s_60_12: and s_60_10 s_60_11
        let s_60_12: u64 = ((s_60_10) & (s_60_11));
        // D s_60_13: cmp-eq s_60_12 s_60_11
        let s_60_13: bool = ((s_60_12) == (s_60_11));
        // D s_60_14: lsl s_60_10 s_60_8
        let s_60_14: u64 = s_60_10 << s_60_8;
        // D s_60_15: or s_60_9 s_60_14
        let s_60_15: u64 = ((s_60_9) | (s_60_14));
        // D s_60_16: cmpl s_60_14
        let s_60_16: u64 = !s_60_14;
        // D s_60_17: and s_60_9 s_60_16
        let s_60_17: u64 = ((s_60_9) & (s_60_16));
        // D s_60_18: select s_60_13 s_60_15 s_60_17
        let s_60_18: u64 = if s_60_13 { s_60_15 } else { s_60_17 };
        // D s_60_19: cast trunc s_60_18 -> u8
        let s_60_19: bool = ((s_60_18) != 0);
        // D s_60_20: cast zx s_60_19 -> bv
        let s_60_20: Bits = Bits::new(s_60_19 as u128, 1u16);
        // C s_60_21: const #0u : u8
        let s_60_21: bool = false;
        // C s_60_22: cast zx s_60_21 -> bv
        let s_60_22: Bits = Bits::new(s_60_21 as u128, 1u16);
        // D s_60_23: cmp-eq s_60_20 s_60_22
        let s_60_23: bool = ((s_60_20) == (s_60_22));
        // N s_60_24: branch s_60_23 b69 b61
        if s_60_23 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #22712u : u32
        let s_61_0: u32 = 22712;
        // D s_61_1: read-reg s_61_0:struct
        let s_61_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call _get_MDCR_EL3_Type_NSPB(s_61_1)
        let s_61_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_61_1);
        // C s_61_3: const #1s : i
        let s_61_3: i128 = 1;
        // D s_61_4: cast zx s_61_2 -> bv
        let s_61_4: Bits = Bits::new(s_61_2 as u128, 2u16);
        // C s_61_5: const #1u : u64
        let s_61_5: u64 = 1;
        // D s_61_6: bit-extract s_61_4 s_61_3 s_61_5
        let s_61_6: Bits = (Bits::new(
            ((s_61_4) >> (s_61_3)).value(),
            u16::try_from(s_61_5).unwrap(),
        ));
        // D s_61_7: cast reint s_61_6 -> u8
        let s_61_7: bool = ((s_61_6.value()) != 0);
        // C s_61_8: const #0s : i
        let s_61_8: i128 = 0;
        // C s_61_9: const #0u : u64
        let s_61_9: u64 = 0;
        // D s_61_10: cast zx s_61_7 -> u64
        let s_61_10: u64 = (s_61_7 as u64);
        // C s_61_11: const #1u : u64
        let s_61_11: u64 = 1;
        // D s_61_12: and s_61_10 s_61_11
        let s_61_12: u64 = ((s_61_10) & (s_61_11));
        // D s_61_13: cmp-eq s_61_12 s_61_11
        let s_61_13: bool = ((s_61_12) == (s_61_11));
        // D s_61_14: lsl s_61_10 s_61_8
        let s_61_14: u64 = s_61_10 << s_61_8;
        // D s_61_15: or s_61_9 s_61_14
        let s_61_15: u64 = ((s_61_9) | (s_61_14));
        // D s_61_16: cmpl s_61_14
        let s_61_16: u64 = !s_61_14;
        // D s_61_17: and s_61_9 s_61_16
        let s_61_17: u64 = ((s_61_9) & (s_61_16));
        // D s_61_18: select s_61_13 s_61_15 s_61_17
        let s_61_18: u64 = if s_61_13 { s_61_15 } else { s_61_17 };
        // D s_61_19: cast trunc s_61_18 -> u8
        let s_61_19: bool = ((s_61_18) != 0);
        // C s_61_20: const #90704u : u32
        let s_61_20: u32 = 90704;
        // D s_61_21: read-reg s_61_20:struct
        let s_61_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_20 as isize);
            tracer.read_register(s_61_20 as isize, value);
            value
        };
        // D s_61_22: call _get_SCR_EL3_Type_NS(s_61_21)
        let s_61_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_61_21);
        // D s_61_23: cast zx s_61_19 -> bv
        let s_61_23: Bits = Bits::new(s_61_19 as u128, 1u16);
        // D s_61_24: cast zx s_61_22 -> bv
        let s_61_24: Bits = Bits::new(s_61_22 as u128, 1u16);
        // D s_61_25: cmp-ne s_61_23 s_61_24
        let s_61_25: bool = ((s_61_23) != (s_61_24));
        // D s_61_26: write-var gs#68259 <= s_61_25
        fn_state.gs_68259 = s_61_25;
        // N s_61_27: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#68259:u8
        let s_62_0: bool = fn_state.gs_68259;
        // N s_62_1: branch s_62_0 b68 b63
        if s_62_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #232u : u32
        let s_63_0: u32 = 232;
        // S s_63_1: call IsFeatureImplemented(s_63_0)
        let s_63_1: bool = IsFeatureImplemented(state, tracer, s_63_0);
        // N s_63_2: branch s_63_1 b67 b64
        if s_63_1 {
            return block_67(state, tracer, fn_state);
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
        // D s_64_1: write-var gs#68260 <= s_64_0
        fn_state.gs_68260 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#68260:u8
        let s_65_0: bool = fn_state.gs_68260;
        // D s_65_1: write-var gs#68261 <= s_65_0
        fn_state.gs_68261 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#68261:u8
        let s_66_0: bool = fn_state.gs_68261;
        // D s_66_1: write-var gs#68262 <= s_66_0
        fn_state.gs_68262 = s_66_0;
        // N s_66_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __MDCR_EL3_NSPBE:u8
        let s_67_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // D s_67_2: read-var __SCR_EL3_NSE:u8
        let s_67_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-ne s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) != (s_67_3));
        // D s_67_5: write-var gs#68260 <= s_67_4
        fn_state.gs_68260 = s_67_4;
        // N s_67_6: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#68261 <= s_68_0
        fn_state.gs_68261 = s_68_0;
        // N s_68_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var gs#68259 <= s_69_0
        fn_state.gs_68259 = s_69_0;
        // N s_69_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_70_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_70_1: call __IMPDEF_boolean(s_70_0)
        let s_70_1: bool = u__IMPDEF_boolean(state, tracer, s_70_0);
        // D s_70_2: write-var gs#68254 <= s_70_1
        fn_state.gs_68254 = s_70_1;
        // N s_70_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call EDSCR_read(s_71_0)
        let s_71_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_71_0);
        // S s_71_2: call _get_EDSCR_Type_SDD(s_71_1)
        let s_71_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_71_1);
        // S s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // C s_71_4: const #1u : u8
        let s_71_4: bool = true;
        // C s_71_5: cast zx s_71_4 -> bv
        let s_71_5: Bits = Bits::new(s_71_4 as u128, 1u16);
        // S s_71_6: cmp-eq s_71_3 s_71_5
        let s_71_6: bool = ((s_71_3) == (s_71_5));
        // D s_71_7: write-var gs#68253 <= s_71_6
        fn_state.gs_68253 = s_71_6;
        // N s_71_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #424u : u32
        let s_72_0: u32 = 424;
        // D s_72_1: read-reg s_72_0:u8
        let s_72_1: u8 = {
            let value = state.read_register::<u8>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // C s_72_2: const #2u : u8
        let s_72_2: u8 = 2;
        // D s_72_3: cmp-lt s_72_1 s_72_2
        let s_72_3: bool = ((s_72_1) < (s_72_2));
        // D s_72_4: write-var gs#68252 <= s_72_3
        fn_state.gs_68252 = s_72_3;
        // N s_72_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call Halted(s_73_0)
        let s_73_1: bool = Halted(state, tracer, s_73_0);
        // N s_73_2: branch s_73_1 b169 b74
        if s_73_1 {
            return block_169(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#68279 <= s_74_0
        fn_state.gs_68279 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#68279:u8
        let s_75_0: bool = fn_state.gs_68279;
        // N s_75_1: branch s_75_0 b168 b76
        if s_75_0 {
            return block_168(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#68280 <= s_76_0
        fn_state.gs_68280 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#68280:u8
        let s_77_0: bool = fn_state.gs_68280;
        // N s_77_1: branch s_77_0 b167 b78
        if s_77_0 {
            return block_167(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#68281 <= s_78_0
        fn_state.gs_68281 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#68281:u8
        let s_79_0: bool = fn_state.gs_68281;
        // N s_79_1: branch s_79_0 b166 b80
        if s_79_0 {
            return block_166(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#68282 <= s_80_0
        fn_state.gs_68282 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#68282:u8
        let s_81_0: bool = fn_state.gs_68282;
        // N s_81_1: branch s_81_0 b165 b82
        if s_81_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call Halted(s_82_0)
        let s_82_1: bool = Halted(state, tracer, s_82_0);
        // N s_82_2: branch s_82_1 b164 b83
        if s_82_1 {
            return block_164(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#68283 <= s_83_0
        fn_state.gs_68283 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#68283:u8
        let s_84_0: bool = fn_state.gs_68283;
        // N s_84_1: branch s_84_0 b163 b85
        if s_84_0 {
            return block_163(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#68284 <= s_85_0
        fn_state.gs_68284 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#68284:u8
        let s_86_0: bool = fn_state.gs_68284;
        // N s_86_1: branch s_86_0 b162 b87
        if s_86_0 {
            return block_162(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#68285 <= s_87_0
        fn_state.gs_68285 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#68285:u8
        let s_88_0: bool = fn_state.gs_68285;
        // N s_88_1: branch s_88_0 b152 b89
        if s_88_0 {
            return block_152(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#68293 <= s_89_0
        fn_state.gs_68293 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#68293:u8
        let s_90_0: bool = fn_state.gs_68293;
        // N s_90_1: branch s_90_0 b151 b91
        if s_90_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
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
        // N s_91_2: branch s_91_1 b150 b92
        if s_91_1 {
            return block_150(state, tracer, fn_state);
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
        // D s_92_1: write-var gs#68294 <= s_92_0
        fn_state.gs_68294 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#68294:u8
        let s_93_0: bool = fn_state.gs_68294;
        // N s_93_1: branch s_93_0 b149 b94
        if s_93_0 {
            return block_149(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#68295 <= s_94_0
        fn_state.gs_68295 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#68295:u8
        let s_95_0: bool = fn_state.gs_68295;
        // N s_95_1: branch s_95_0 b148 b96
        if s_95_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var gs#68296 <= s_96_0
        fn_state.gs_68296 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#68296:u8
        let s_97_0: bool = fn_state.gs_68296;
        // N s_97_1: branch s_97_0 b147 b98
        if s_97_0 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call EL2Enabled(s_98_0)
        let s_98_1: bool = EL2Enabled(state, tracer, s_98_0);
        // N s_98_2: branch s_98_1 b146 b99
        if s_98_1 {
            return block_146(state, tracer, fn_state);
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
        // D s_99_1: write-var gs#68297 <= s_99_0
        fn_state.gs_68297 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#68297:u8
        let s_100_0: bool = fn_state.gs_68297;
        // N s_100_1: branch s_100_0 b145 b101
        if s_100_0 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #0u : u8
        let s_101_0: bool = false;
        // D s_101_1: write-var gs#68298 <= s_101_0
        fn_state.gs_68298 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#68298:u8
        let s_102_0: bool = fn_state.gs_68298;
        // N s_102_1: branch s_102_0 b144 b103
        if s_102_0 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #() : ()
        let s_103_0: () = ();
        // S s_103_1: call EL2Enabled(s_103_0)
        let s_103_1: bool = EL2Enabled(state, tracer, s_103_0);
        // N s_103_2: branch s_103_1 b143 b104
        if s_103_1 {
            return block_143(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#68299 <= s_104_0
        fn_state.gs_68299 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#68299:u8
        let s_105_0: bool = fn_state.gs_68299;
        // N s_105_1: branch s_105_0 b142 b106
        if s_105_0 {
            return block_142(state, tracer, fn_state);
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
        // N s_106_4: branch s_106_3 b141 b107
        if s_106_3 {
            return block_141(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#68300 <= s_107_0
        fn_state.gs_68300 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#68300:u8
        let s_108_0: bool = fn_state.gs_68300;
        // N s_108_1: branch s_108_0 b135 b109
        if s_108_0 {
            return block_135(state, tracer, fn_state);
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
        // N s_109_4: branch s_109_3 b125 b110
        if s_109_3 {
            return block_125(state, tracer, fn_state);
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
        // D s_110_1: write-var gs#68308 <= s_110_0
        fn_state.gs_68308 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#68308:u8
        let s_111_0: bool = fn_state.gs_68308;
        // N s_111_1: branch s_111_0 b119 b112
        if s_111_0 {
            return block_119(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #() : ()
        let s_112_0: () = ();
        // S s_112_1: call EL2Enabled(s_112_0)
        let s_112_1: bool = EL2Enabled(state, tracer, s_112_0);
        // N s_112_2: branch s_112_1 b118 b113
        if s_112_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #0u : u8
        let s_113_0: bool = false;
        // D s_113_1: write-var gs#68309 <= s_113_0
        fn_state.gs_68309 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#68309:u8
        let s_114_0: bool = fn_state.gs_68309;
        // N s_114_1: branch s_114_0 b116 b115
        if s_114_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #64s : i64
        let s_115_0: i64 = 64;
        // C s_115_1: const #1520u : u32
        let s_115_1: u32 = 1520;
        // D s_115_2: read-reg s_115_1:struct
        let s_115_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_115_1 as isize);
            tracer.read_register(s_115_1 as isize, value);
            value
        };
        // D s_115_3: call __get_PMSDSFR_EL1(s_115_2)
        let s_115_3: ProductType5c790c8ef59cc8b2 = u__get_PMSDSFR_EL1(
            state,
            tracer,
            s_115_2,
        );
        // D s_115_4: write-var ga#82609 <= s_115_3
        fn_state.ga_82609 = s_115_3;
        // D s_115_5: read-var ga#82609.0:struct
        let s_115_5: u64 = fn_state.ga_82609._0;
        // D s_115_6: cast zx s_115_5 -> bv
        let s_115_6: Bits = Bits::new(s_115_5 as u128, 64u16);
        // D s_115_7: read-var t:i
        let s_115_7: i128 = fn_state.t;
        // D s_115_8: call X_set(s_115_7, s_115_0, s_115_6)
        let s_115_8: () = X_set(state, tracer, s_115_7, s_115_0, s_115_6);
        // N s_115_9: return
        return;
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #2136u : u12
        let s_116_0: u16 = 2136;
        // C s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 12u16);
        // C s_116_2: cast zx s_116_1 -> i
        let s_116_2: i128 = (s_116_1.value() as i128);
        // C s_116_3: cast reint s_116_2 -> i64
        let s_116_3: i64 = (s_116_2 as i64);
        // C s_116_4: cast zx s_116_3 -> i
        let s_116_4: i128 = (i128::try_from(s_116_3).unwrap());
        // S s_116_5: call NVMem_read(s_116_4)
        let s_116_5: u64 = NVMem_read(state, tracer, s_116_4);
        // D s_116_6: write-var ga#82608 <= s_116_5
        fn_state.ga_82608 = s_116_5;
        // N s_116_7: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var ga#82608:u64
        let s_117_0: u64 = fn_state.ga_82608;
        // D s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 64u16);
        // D s_117_2: read-var t:i
        let s_117_2: i128 = fn_state.t;
        // C s_117_3: const #64s : i64
        let s_117_3: i64 = 64;
        // D s_117_4: call X_set(s_117_2, s_117_3, s_117_1)
        let s_117_4: () = X_set(state, tracer, s_117_2, s_117_3, s_117_1);
        // N s_117_5: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #102552u : u32
        let s_118_0: u32 = 102552;
        // D s_118_1: read-reg s_118_0:struct
        let s_118_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_118_0 as isize);
            tracer.read_register(s_118_0 as isize, value);
            value
        };
        // D s_118_2: call _get_HCR_EL2_Type_NV2(s_118_1)
        let s_118_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_118_1);
        // C s_118_3: const #102552u : u32
        let s_118_3: u32 = 102552;
        // D s_118_4: read-reg s_118_3:struct
        let s_118_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_118_3 as isize);
            tracer.read_register(s_118_3 as isize, value);
            value
        };
        // D s_118_5: call _get_HCR_EL2_Type_NV(s_118_4)
        let s_118_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_118_4);
        // D s_118_6: cast zx s_118_2 -> bv
        let s_118_6: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_7: cast zx s_118_5 -> bv
        let s_118_7: Bits = Bits::new(s_118_5 as u128, 1u16);
        // D s_118_8: cast reint s_118_6 -> u128
        let s_118_8: u128 = (s_118_6.value() as u128);
        // D s_118_9: size-of s_118_6
        let s_118_9: u16 = s_118_6.length();
        // D s_118_10: cast reint s_118_7 -> u128
        let s_118_10: u128 = (s_118_7.value() as u128);
        // D s_118_11: size-of s_118_7
        let s_118_11: u16 = s_118_7.length();
        // D s_118_12: lsl s_118_8 s_118_11
        let s_118_12: u128 = s_118_8 << s_118_11;
        // D s_118_13: or s_118_12 s_118_10
        let s_118_13: u128 = ((s_118_12) | (s_118_10));
        // D s_118_14: add s_118_9 s_118_11
        let s_118_14: u16 = (s_118_9 + s_118_11);
        // D s_118_15: create-bits s_118_13 s_118_14
        let s_118_15: Bits = Bits::new(s_118_13, s_118_14);
        // D s_118_16: cast reint s_118_15 -> u8
        let s_118_16: u8 = (s_118_15.value() as u8);
        // D s_118_17: cast zx s_118_16 -> bv
        let s_118_17: Bits = Bits::new(s_118_16 as u128, 2u16);
        // C s_118_18: const #3u : u8
        let s_118_18: u8 = 3;
        // C s_118_19: cast zx s_118_18 -> bv
        let s_118_19: Bits = Bits::new(s_118_18 as u128, 2u16);
        // D s_118_20: cmp-eq s_118_17 s_118_19
        let s_118_20: bool = ((s_118_17) == (s_118_19));
        // D s_118_21: write-var gs#68309 <= s_118_20
        fn_state.gs_68309 = s_118_20;
        // N s_118_22: jump b114
        return block_114(state, tracer, fn_state);
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
        // D s_120_1: write-var gs#68312 <= s_120_0
        fn_state.gs_68312 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#68312:u8
        let s_121_0: bool = fn_state.gs_68312;
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
        // C s_124_0: const #() : ()
        let s_124_0: () = ();
        // S s_124_1: call EDSCR_read(s_124_0)
        let s_124_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_124_0);
        // S s_124_2: call _get_EDSCR_Type_SDD(s_124_1)
        let s_124_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_124_1);
        // S s_124_3: cast zx s_124_2 -> bv
        let s_124_3: Bits = Bits::new(s_124_2 as u128, 1u16);
        // C s_124_4: const #1u : u8
        let s_124_4: bool = true;
        // C s_124_5: cast zx s_124_4 -> bv
        let s_124_5: Bits = Bits::new(s_124_4 as u128, 1u16);
        // S s_124_6: cmp-eq s_124_3 s_124_5
        let s_124_6: bool = ((s_124_3) == (s_124_5));
        // D s_124_7: write-var gs#68312 <= s_124_6
        fn_state.gs_68312 = s_124_6;
        // N s_124_8: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #22712u : u32
        let s_125_0: u32 = 22712;
        // D s_125_1: read-reg s_125_0:struct
        let s_125_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_125_0 as isize);
            tracer.read_register(s_125_0 as isize, value);
            value
        };
        // D s_125_2: call _get_MDCR_EL3_Type_NSPB(s_125_1)
        let s_125_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_125_1);
        // C s_125_3: const #0s : i
        let s_125_3: i128 = 0;
        // D s_125_4: cast zx s_125_2 -> bv
        let s_125_4: Bits = Bits::new(s_125_2 as u128, 2u16);
        // C s_125_5: const #1u : u64
        let s_125_5: u64 = 1;
        // D s_125_6: bit-extract s_125_4 s_125_3 s_125_5
        let s_125_6: Bits = (Bits::new(
            ((s_125_4) >> (s_125_3)).value(),
            u16::try_from(s_125_5).unwrap(),
        ));
        // D s_125_7: cast reint s_125_6 -> u8
        let s_125_7: bool = ((s_125_6.value()) != 0);
        // C s_125_8: const #0s : i
        let s_125_8: i128 = 0;
        // C s_125_9: const #0u : u64
        let s_125_9: u64 = 0;
        // D s_125_10: cast zx s_125_7 -> u64
        let s_125_10: u64 = (s_125_7 as u64);
        // C s_125_11: const #1u : u64
        let s_125_11: u64 = 1;
        // D s_125_12: and s_125_10 s_125_11
        let s_125_12: u64 = ((s_125_10) & (s_125_11));
        // D s_125_13: cmp-eq s_125_12 s_125_11
        let s_125_13: bool = ((s_125_12) == (s_125_11));
        // D s_125_14: lsl s_125_10 s_125_8
        let s_125_14: u64 = s_125_10 << s_125_8;
        // D s_125_15: or s_125_9 s_125_14
        let s_125_15: u64 = ((s_125_9) | (s_125_14));
        // D s_125_16: cmpl s_125_14
        let s_125_16: u64 = !s_125_14;
        // D s_125_17: and s_125_9 s_125_16
        let s_125_17: u64 = ((s_125_9) & (s_125_16));
        // D s_125_18: select s_125_13 s_125_15 s_125_17
        let s_125_18: u64 = if s_125_13 { s_125_15 } else { s_125_17 };
        // D s_125_19: cast trunc s_125_18 -> u8
        let s_125_19: bool = ((s_125_18) != 0);
        // D s_125_20: cast zx s_125_19 -> bv
        let s_125_20: Bits = Bits::new(s_125_19 as u128, 1u16);
        // C s_125_21: const #0u : u8
        let s_125_21: bool = false;
        // C s_125_22: cast zx s_125_21 -> bv
        let s_125_22: Bits = Bits::new(s_125_21 as u128, 1u16);
        // D s_125_23: cmp-eq s_125_20 s_125_22
        let s_125_23: bool = ((s_125_20) == (s_125_22));
        // N s_125_24: branch s_125_23 b134 b126
        if s_125_23 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #22712u : u32
        let s_126_0: u32 = 22712;
        // D s_126_1: read-reg s_126_0:struct
        let s_126_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_126_0 as isize);
            tracer.read_register(s_126_0 as isize, value);
            value
        };
        // D s_126_2: call _get_MDCR_EL3_Type_NSPB(s_126_1)
        let s_126_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_126_1);
        // C s_126_3: const #1s : i
        let s_126_3: i128 = 1;
        // D s_126_4: cast zx s_126_2 -> bv
        let s_126_4: Bits = Bits::new(s_126_2 as u128, 2u16);
        // C s_126_5: const #1u : u64
        let s_126_5: u64 = 1;
        // D s_126_6: bit-extract s_126_4 s_126_3 s_126_5
        let s_126_6: Bits = (Bits::new(
            ((s_126_4) >> (s_126_3)).value(),
            u16::try_from(s_126_5).unwrap(),
        ));
        // D s_126_7: cast reint s_126_6 -> u8
        let s_126_7: bool = ((s_126_6.value()) != 0);
        // C s_126_8: const #0s : i
        let s_126_8: i128 = 0;
        // C s_126_9: const #0u : u64
        let s_126_9: u64 = 0;
        // D s_126_10: cast zx s_126_7 -> u64
        let s_126_10: u64 = (s_126_7 as u64);
        // C s_126_11: const #1u : u64
        let s_126_11: u64 = 1;
        // D s_126_12: and s_126_10 s_126_11
        let s_126_12: u64 = ((s_126_10) & (s_126_11));
        // D s_126_13: cmp-eq s_126_12 s_126_11
        let s_126_13: bool = ((s_126_12) == (s_126_11));
        // D s_126_14: lsl s_126_10 s_126_8
        let s_126_14: u64 = s_126_10 << s_126_8;
        // D s_126_15: or s_126_9 s_126_14
        let s_126_15: u64 = ((s_126_9) | (s_126_14));
        // D s_126_16: cmpl s_126_14
        let s_126_16: u64 = !s_126_14;
        // D s_126_17: and s_126_9 s_126_16
        let s_126_17: u64 = ((s_126_9) & (s_126_16));
        // D s_126_18: select s_126_13 s_126_15 s_126_17
        let s_126_18: u64 = if s_126_13 { s_126_15 } else { s_126_17 };
        // D s_126_19: cast trunc s_126_18 -> u8
        let s_126_19: bool = ((s_126_18) != 0);
        // C s_126_20: const #90704u : u32
        let s_126_20: u32 = 90704;
        // D s_126_21: read-reg s_126_20:struct
        let s_126_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_126_20 as isize);
            tracer.read_register(s_126_20 as isize, value);
            value
        };
        // D s_126_22: call _get_SCR_EL3_Type_NS(s_126_21)
        let s_126_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_126_21);
        // D s_126_23: cast zx s_126_19 -> bv
        let s_126_23: Bits = Bits::new(s_126_19 as u128, 1u16);
        // D s_126_24: cast zx s_126_22 -> bv
        let s_126_24: Bits = Bits::new(s_126_22 as u128, 1u16);
        // D s_126_25: cmp-ne s_126_23 s_126_24
        let s_126_25: bool = ((s_126_23) != (s_126_24));
        // D s_126_26: write-var gs#68305 <= s_126_25
        fn_state.gs_68305 = s_126_25;
        // N s_126_27: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var gs#68305:u8
        let s_127_0: bool = fn_state.gs_68305;
        // N s_127_1: branch s_127_0 b133 b128
        if s_127_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #232u : u32
        let s_128_0: u32 = 232;
        // S s_128_1: call IsFeatureImplemented(s_128_0)
        let s_128_1: bool = IsFeatureImplemented(state, tracer, s_128_0);
        // N s_128_2: branch s_128_1 b132 b129
        if s_128_1 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #0u : u8
        let s_129_0: bool = false;
        // D s_129_1: write-var gs#68306 <= s_129_0
        fn_state.gs_68306 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#68306:u8
        let s_130_0: bool = fn_state.gs_68306;
        // D s_130_1: write-var gs#68307 <= s_130_0
        fn_state.gs_68307 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#68307:u8
        let s_131_0: bool = fn_state.gs_68307;
        // D s_131_1: write-var gs#68308 <= s_131_0
        fn_state.gs_68308 = s_131_0;
        // N s_131_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var __MDCR_EL3_NSPBE:u8
        let s_132_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_132_1: cast zx s_132_0 -> bv
        let s_132_1: Bits = Bits::new(s_132_0 as u128, 1u16);
        // D s_132_2: read-var __SCR_EL3_NSE:u8
        let s_132_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_132_3: cast zx s_132_2 -> bv
        let s_132_3: Bits = Bits::new(s_132_2 as u128, 1u16);
        // D s_132_4: cmp-ne s_132_1 s_132_3
        let s_132_4: bool = ((s_132_1) != (s_132_3));
        // D s_132_5: write-var gs#68306 <= s_132_4
        fn_state.gs_68306 = s_132_4;
        // N s_132_6: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #1u : u8
        let s_133_0: bool = true;
        // D s_133_1: write-var gs#68307 <= s_133_0
        fn_state.gs_68307 = s_133_0;
        // N s_133_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #1u : u8
        let s_134_0: bool = true;
        // D s_134_1: write-var gs#68305 <= s_134_0
        fn_state.gs_68305 = s_134_0;
        // N s_134_2: jump b127
        return block_127(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #() : ()
        let s_135_0: () = ();
        // S s_135_1: call Halted(s_135_0)
        let s_135_1: bool = Halted(state, tracer, s_135_0);
        // N s_135_2: branch s_135_1 b140 b136
        if s_135_1 {
            return block_140(state, tracer, fn_state);
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
        // D s_136_1: write-var gs#68313 <= s_136_0
        fn_state.gs_68313 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#68313:u8
        let s_137_0: bool = fn_state.gs_68313;
        // N s_137_1: branch s_137_0 b139 b138
        if s_137_0 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
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
        // C s_138_5: const #424u : u32
        let s_138_5: u32 = 424;
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
        // N s_139_0: panic
        panic!("{:?}", ());
        // N s_139_1: return
        return;
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #() : ()
        let s_140_0: () = ();
        // S s_140_1: call EDSCR_read(s_140_0)
        let s_140_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_140_0);
        // S s_140_2: call _get_EDSCR_Type_SDD(s_140_1)
        let s_140_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_140_1);
        // S s_140_3: cast zx s_140_2 -> bv
        let s_140_3: Bits = Bits::new(s_140_2 as u128, 1u16);
        // C s_140_4: const #1u : u8
        let s_140_4: bool = true;
        // C s_140_5: cast zx s_140_4 -> bv
        let s_140_5: Bits = Bits::new(s_140_4 as u128, 1u16);
        // S s_140_6: cmp-eq s_140_3 s_140_5
        let s_140_6: bool = ((s_140_3) == (s_140_5));
        // D s_140_7: write-var gs#68313 <= s_140_6
        fn_state.gs_68313 = s_140_6;
        // N s_140_8: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var __MDCR_EL3_EnPMS3:u8
        let s_141_0: bool = fn_state.u__MDCR_EL3_EnPMS3;
        // D s_141_1: cast zx s_141_0 -> bv
        let s_141_1: Bits = Bits::new(s_141_0 as u128, 1u16);
        // C s_141_2: const #0u : u8
        let s_141_2: bool = false;
        // C s_141_3: cast zx s_141_2 -> bv
        let s_141_3: Bits = Bits::new(s_141_2 as u128, 1u16);
        // D s_141_4: cmp-eq s_141_1 s_141_3
        let s_141_4: bool = ((s_141_1) == (s_141_3));
        // D s_141_5: write-var gs#68300 <= s_141_4
        fn_state.gs_68300 = s_141_4;
        // N s_141_6: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #24u : u8
        let s_142_0: u8 = 24;
        // C s_142_1: cast zx s_142_0 -> bv
        let s_142_1: Bits = Bits::new(s_142_0 as u128, 8u16);
        // C s_142_2: cast zx s_142_1 -> i
        let s_142_2: i128 = (s_142_1.value() as i128);
        // C s_142_3: cast reint s_142_2 -> i64
        let s_142_3: i64 = (s_142_2 as i64);
        // C s_142_4: cast zx s_142_3 -> i
        let s_142_4: i128 = (i128::try_from(s_142_3).unwrap());
        // C s_142_5: const #432u : u32
        let s_142_5: u32 = 432;
        // D s_142_6: read-reg s_142_5:u8
        let s_142_6: u8 = {
            let value = state.read_register::<u8>(s_142_5 as isize);
            tracer.read_register(s_142_5 as isize, value);
            value
        };
        // D s_142_7: call AArch64_SystemAccessTrap(s_142_6, s_142_4)
        let s_142_7: () = AArch64_SystemAccessTrap(state, tracer, s_142_6, s_142_4);
        // N s_142_8: return
        return;
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var __MDCR_EL2_TPMS:u8
        let s_143_0: bool = fn_state.u__MDCR_EL2_TPMS;
        // D s_143_1: cast zx s_143_0 -> bv
        let s_143_1: Bits = Bits::new(s_143_0 as u128, 1u16);
        // C s_143_2: const #1u : u8
        let s_143_2: bool = true;
        // C s_143_3: cast zx s_143_2 -> bv
        let s_143_3: Bits = Bits::new(s_143_2 as u128, 1u16);
        // D s_143_4: cmp-eq s_143_1 s_143_3
        let s_143_4: bool = ((s_143_1) == (s_143_3));
        // D s_143_5: write-var gs#68299 <= s_143_4
        fn_state.gs_68299 = s_143_4;
        // N s_143_6: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #24u : u8
        let s_144_0: u8 = 24;
        // C s_144_1: cast zx s_144_0 -> bv
        let s_144_1: Bits = Bits::new(s_144_0 as u128, 8u16);
        // C s_144_2: cast zx s_144_1 -> i
        let s_144_2: i128 = (s_144_1.value() as i128);
        // C s_144_3: cast reint s_144_2 -> i64
        let s_144_3: i64 = (s_144_2 as i64);
        // C s_144_4: cast zx s_144_3 -> i
        let s_144_4: i128 = (i128::try_from(s_144_3).unwrap());
        // C s_144_5: const #432u : u32
        let s_144_5: u32 = 432;
        // D s_144_6: read-reg s_144_5:u8
        let s_144_6: u8 = {
            let value = state.read_register::<u8>(s_144_5 as isize);
            tracer.read_register(s_144_5 as isize, value);
            value
        };
        // D s_144_7: call AArch64_SystemAccessTrap(s_144_6, s_144_4)
        let s_144_7: () = AArch64_SystemAccessTrap(state, tracer, s_144_6, s_144_4);
        // N s_144_8: return
        return;
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var __HDFGRTR2_EL2_nPMSDSFR_EL1:u8
        let s_145_0: bool = fn_state.u__HDFGRTR2_EL2_nPMSDSFR_EL1;
        // D s_145_1: cast zx s_145_0 -> bv
        let s_145_1: Bits = Bits::new(s_145_0 as u128, 1u16);
        // C s_145_2: const #0u : u8
        let s_145_2: bool = false;
        // C s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 1u16);
        // D s_145_4: cmp-eq s_145_1 s_145_3
        let s_145_4: bool = ((s_145_1) == (s_145_3));
        // D s_145_5: write-var gs#68298 <= s_145_4
        fn_state.gs_68298 = s_145_4;
        // N s_145_6: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #188u : u32
        let s_146_0: u32 = 188;
        // S s_146_1: call IsFeatureImplemented(s_146_0)
        let s_146_1: bool = IsFeatureImplemented(state, tracer, s_146_0);
        // D s_146_2: write-var gs#68297 <= s_146_1
        fn_state.gs_68297 = s_146_1;
        // N s_146_3: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #24u : u8
        let s_147_0: u8 = 24;
        // C s_147_1: cast zx s_147_0 -> bv
        let s_147_1: Bits = Bits::new(s_147_0 as u128, 8u16);
        // C s_147_2: cast zx s_147_1 -> i
        let s_147_2: i128 = (s_147_1.value() as i128);
        // C s_147_3: cast reint s_147_2 -> i64
        let s_147_3: i64 = (s_147_2 as i64);
        // C s_147_4: cast zx s_147_3 -> i
        let s_147_4: i128 = (i128::try_from(s_147_3).unwrap());
        // C s_147_5: const #432u : u32
        let s_147_5: u32 = 432;
        // D s_147_6: read-reg s_147_5:u8
        let s_147_6: u8 = {
            let value = state.read_register::<u8>(s_147_5 as isize);
            tracer.read_register(s_147_5 as isize, value);
            value
        };
        // D s_147_7: call AArch64_SystemAccessTrap(s_147_6, s_147_4)
        let s_147_7: () = AArch64_SystemAccessTrap(state, tracer, s_147_6, s_147_4);
        // N s_147_8: return
        return;
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_148_0: read-var __SCR_EL3_FGTEn2:u8
        let s_148_0: bool = fn_state.u__SCR_EL3_FGTEn2;
        // D s_148_1: cast zx s_148_0 -> bv
        let s_148_1: Bits = Bits::new(s_148_0 as u128, 1u16);
        // C s_148_2: const #0u : u8
        let s_148_2: bool = false;
        // C s_148_3: cast zx s_148_2 -> bv
        let s_148_3: Bits = Bits::new(s_148_2 as u128, 1u16);
        // D s_148_4: cmp-eq s_148_1 s_148_3
        let s_148_4: bool = ((s_148_1) == (s_148_3));
        // D s_148_5: write-var gs#68296 <= s_148_4
        fn_state.gs_68296 = s_148_4;
        // N s_148_6: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #424u : u32
        let s_149_0: u32 = 424;
        // D s_149_1: read-reg s_149_0:u8
        let s_149_1: u8 = {
            let value = state.read_register::<u8>(s_149_0 as isize);
            tracer.read_register(s_149_0 as isize, value);
            value
        };
        // C s_149_2: const #2u : u8
        let s_149_2: u8 = 2;
        // D s_149_3: cmp-lt s_149_1 s_149_2
        let s_149_3: bool = ((s_149_1) < (s_149_2));
        // D s_149_4: write-var gs#68295 <= s_149_3
        fn_state.gs_68295 = s_149_3;
        // N s_149_5: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #188u : u32
        let s_150_0: u32 = 188;
        // S s_150_1: call IsFeatureImplemented(s_150_0)
        let s_150_1: bool = IsFeatureImplemented(state, tracer, s_150_0);
        // D s_150_2: write-var gs#68294 <= s_150_1
        fn_state.gs_68294 = s_150_1;
        // N s_150_3: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_151_0: panic
        panic!("{:?}", ());
        // N s_151_1: return
        return;
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #22712u : u32
        let s_152_0: u32 = 22712;
        // D s_152_1: read-reg s_152_0:struct
        let s_152_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_152_0 as isize);
            tracer.read_register(s_152_0 as isize, value);
            value
        };
        // D s_152_2: call _get_MDCR_EL3_Type_NSPB(s_152_1)
        let s_152_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_152_1);
        // C s_152_3: const #0s : i
        let s_152_3: i128 = 0;
        // D s_152_4: cast zx s_152_2 -> bv
        let s_152_4: Bits = Bits::new(s_152_2 as u128, 2u16);
        // C s_152_5: const #1u : u64
        let s_152_5: u64 = 1;
        // D s_152_6: bit-extract s_152_4 s_152_3 s_152_5
        let s_152_6: Bits = (Bits::new(
            ((s_152_4) >> (s_152_3)).value(),
            u16::try_from(s_152_5).unwrap(),
        ));
        // D s_152_7: cast reint s_152_6 -> u8
        let s_152_7: bool = ((s_152_6.value()) != 0);
        // C s_152_8: const #0s : i
        let s_152_8: i128 = 0;
        // C s_152_9: const #0u : u64
        let s_152_9: u64 = 0;
        // D s_152_10: cast zx s_152_7 -> u64
        let s_152_10: u64 = (s_152_7 as u64);
        // C s_152_11: const #1u : u64
        let s_152_11: u64 = 1;
        // D s_152_12: and s_152_10 s_152_11
        let s_152_12: u64 = ((s_152_10) & (s_152_11));
        // D s_152_13: cmp-eq s_152_12 s_152_11
        let s_152_13: bool = ((s_152_12) == (s_152_11));
        // D s_152_14: lsl s_152_10 s_152_8
        let s_152_14: u64 = s_152_10 << s_152_8;
        // D s_152_15: or s_152_9 s_152_14
        let s_152_15: u64 = ((s_152_9) | (s_152_14));
        // D s_152_16: cmpl s_152_14
        let s_152_16: u64 = !s_152_14;
        // D s_152_17: and s_152_9 s_152_16
        let s_152_17: u64 = ((s_152_9) & (s_152_16));
        // D s_152_18: select s_152_13 s_152_15 s_152_17
        let s_152_18: u64 = if s_152_13 { s_152_15 } else { s_152_17 };
        // D s_152_19: cast trunc s_152_18 -> u8
        let s_152_19: bool = ((s_152_18) != 0);
        // D s_152_20: cast zx s_152_19 -> bv
        let s_152_20: Bits = Bits::new(s_152_19 as u128, 1u16);
        // C s_152_21: const #0u : u8
        let s_152_21: bool = false;
        // C s_152_22: cast zx s_152_21 -> bv
        let s_152_22: Bits = Bits::new(s_152_21 as u128, 1u16);
        // D s_152_23: cmp-eq s_152_20 s_152_22
        let s_152_23: bool = ((s_152_20) == (s_152_22));
        // N s_152_24: branch s_152_23 b161 b153
        if s_152_23 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #22712u : u32
        let s_153_0: u32 = 22712;
        // D s_153_1: read-reg s_153_0:struct
        let s_153_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_153_0 as isize);
            tracer.read_register(s_153_0 as isize, value);
            value
        };
        // D s_153_2: call _get_MDCR_EL3_Type_NSPB(s_153_1)
        let s_153_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_153_1);
        // C s_153_3: const #1s : i
        let s_153_3: i128 = 1;
        // D s_153_4: cast zx s_153_2 -> bv
        let s_153_4: Bits = Bits::new(s_153_2 as u128, 2u16);
        // C s_153_5: const #1u : u64
        let s_153_5: u64 = 1;
        // D s_153_6: bit-extract s_153_4 s_153_3 s_153_5
        let s_153_6: Bits = (Bits::new(
            ((s_153_4) >> (s_153_3)).value(),
            u16::try_from(s_153_5).unwrap(),
        ));
        // D s_153_7: cast reint s_153_6 -> u8
        let s_153_7: bool = ((s_153_6.value()) != 0);
        // C s_153_8: const #0s : i
        let s_153_8: i128 = 0;
        // C s_153_9: const #0u : u64
        let s_153_9: u64 = 0;
        // D s_153_10: cast zx s_153_7 -> u64
        let s_153_10: u64 = (s_153_7 as u64);
        // C s_153_11: const #1u : u64
        let s_153_11: u64 = 1;
        // D s_153_12: and s_153_10 s_153_11
        let s_153_12: u64 = ((s_153_10) & (s_153_11));
        // D s_153_13: cmp-eq s_153_12 s_153_11
        let s_153_13: bool = ((s_153_12) == (s_153_11));
        // D s_153_14: lsl s_153_10 s_153_8
        let s_153_14: u64 = s_153_10 << s_153_8;
        // D s_153_15: or s_153_9 s_153_14
        let s_153_15: u64 = ((s_153_9) | (s_153_14));
        // D s_153_16: cmpl s_153_14
        let s_153_16: u64 = !s_153_14;
        // D s_153_17: and s_153_9 s_153_16
        let s_153_17: u64 = ((s_153_9) & (s_153_16));
        // D s_153_18: select s_153_13 s_153_15 s_153_17
        let s_153_18: u64 = if s_153_13 { s_153_15 } else { s_153_17 };
        // D s_153_19: cast trunc s_153_18 -> u8
        let s_153_19: bool = ((s_153_18) != 0);
        // C s_153_20: const #90704u : u32
        let s_153_20: u32 = 90704;
        // D s_153_21: read-reg s_153_20:struct
        let s_153_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_153_20 as isize);
            tracer.read_register(s_153_20 as isize, value);
            value
        };
        // D s_153_22: call _get_SCR_EL3_Type_NS(s_153_21)
        let s_153_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_153_21);
        // D s_153_23: cast zx s_153_19 -> bv
        let s_153_23: Bits = Bits::new(s_153_19 as u128, 1u16);
        // D s_153_24: cast zx s_153_22 -> bv
        let s_153_24: Bits = Bits::new(s_153_22 as u128, 1u16);
        // D s_153_25: cmp-ne s_153_23 s_153_24
        let s_153_25: bool = ((s_153_23) != (s_153_24));
        // D s_153_26: write-var gs#68290 <= s_153_25
        fn_state.gs_68290 = s_153_25;
        // N s_153_27: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#68290:u8
        let s_154_0: bool = fn_state.gs_68290;
        // N s_154_1: branch s_154_0 b160 b155
        if s_154_0 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #232u : u32
        let s_155_0: u32 = 232;
        // S s_155_1: call IsFeatureImplemented(s_155_0)
        let s_155_1: bool = IsFeatureImplemented(state, tracer, s_155_0);
        // N s_155_2: branch s_155_1 b159 b156
        if s_155_1 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #0u : u8
        let s_156_0: bool = false;
        // D s_156_1: write-var gs#68291 <= s_156_0
        fn_state.gs_68291 = s_156_0;
        // N s_156_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var gs#68291:u8
        let s_157_0: bool = fn_state.gs_68291;
        // D s_157_1: write-var gs#68292 <= s_157_0
        fn_state.gs_68292 = s_157_0;
        // N s_157_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#68292:u8
        let s_158_0: bool = fn_state.gs_68292;
        // D s_158_1: write-var gs#68293 <= s_158_0
        fn_state.gs_68293 = s_158_0;
        // N s_158_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var __MDCR_EL3_NSPBE:u8
        let s_159_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_159_1: cast zx s_159_0 -> bv
        let s_159_1: Bits = Bits::new(s_159_0 as u128, 1u16);
        // D s_159_2: read-var __SCR_EL3_NSE:u8
        let s_159_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_159_3: cast zx s_159_2 -> bv
        let s_159_3: Bits = Bits::new(s_159_2 as u128, 1u16);
        // D s_159_4: cmp-ne s_159_1 s_159_3
        let s_159_4: bool = ((s_159_1) != (s_159_3));
        // D s_159_5: write-var gs#68291 <= s_159_4
        fn_state.gs_68291 = s_159_4;
        // N s_159_6: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #1u : u8
        let s_160_0: bool = true;
        // D s_160_1: write-var gs#68292 <= s_160_0
        fn_state.gs_68292 = s_160_0;
        // N s_160_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #1u : u8
        let s_161_0: bool = true;
        // D s_161_1: write-var gs#68290 <= s_161_0
        fn_state.gs_68290 = s_161_0;
        // N s_161_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_162_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_162_1: call __IMPDEF_boolean(s_162_0)
        let s_162_1: bool = u__IMPDEF_boolean(state, tracer, s_162_0);
        // D s_162_2: write-var gs#68285 <= s_162_1
        fn_state.gs_68285 = s_162_1;
        // N s_162_3: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #() : ()
        let s_163_0: () = ();
        // S s_163_1: call EDSCR_read(s_163_0)
        let s_163_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_163_0);
        // S s_163_2: call _get_EDSCR_Type_SDD(s_163_1)
        let s_163_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_163_1);
        // S s_163_3: cast zx s_163_2 -> bv
        let s_163_3: Bits = Bits::new(s_163_2 as u128, 1u16);
        // C s_163_4: const #1u : u8
        let s_163_4: bool = true;
        // C s_163_5: cast zx s_163_4 -> bv
        let s_163_5: Bits = Bits::new(s_163_4 as u128, 1u16);
        // S s_163_6: cmp-eq s_163_3 s_163_5
        let s_163_6: bool = ((s_163_3) == (s_163_5));
        // D s_163_7: write-var gs#68284 <= s_163_6
        fn_state.gs_68284 = s_163_6;
        // N s_163_8: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #424u : u32
        let s_164_0: u32 = 424;
        // D s_164_1: read-reg s_164_0:u8
        let s_164_1: u8 = {
            let value = state.read_register::<u8>(s_164_0 as isize);
            tracer.read_register(s_164_0 as isize, value);
            value
        };
        // C s_164_2: const #2u : u8
        let s_164_2: u8 = 2;
        // D s_164_3: cmp-lt s_164_1 s_164_2
        let s_164_3: bool = ((s_164_1) < (s_164_2));
        // D s_164_4: write-var gs#68283 <= s_164_3
        fn_state.gs_68283 = s_164_3;
        // N s_164_5: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_165_0: panic
        panic!("{:?}", ());
        // N s_165_1: return
        return;
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var __MDCR_EL3_EnPMS3:u8
        let s_166_0: bool = fn_state.u__MDCR_EL3_EnPMS3;
        // D s_166_1: cast zx s_166_0 -> bv
        let s_166_1: Bits = Bits::new(s_166_0 as u128, 1u16);
        // C s_166_2: const #0u : u8
        let s_166_2: bool = false;
        // C s_166_3: cast zx s_166_2 -> bv
        let s_166_3: Bits = Bits::new(s_166_2 as u128, 1u16);
        // D s_166_4: cmp-eq s_166_1 s_166_3
        let s_166_4: bool = ((s_166_1) == (s_166_3));
        // D s_166_5: write-var gs#68282 <= s_166_4
        fn_state.gs_68282 = s_166_4;
        // N s_166_6: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_167_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_167_1: call __IMPDEF_boolean(s_167_0)
        let s_167_1: bool = u__IMPDEF_boolean(state, tracer, s_167_0);
        // D s_167_2: write-var gs#68281 <= s_167_1
        fn_state.gs_68281 = s_167_1;
        // N s_167_3: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #() : ()
        let s_168_0: () = ();
        // S s_168_1: call EDSCR_read(s_168_0)
        let s_168_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_168_0);
        // S s_168_2: call _get_EDSCR_Type_SDD(s_168_1)
        let s_168_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_168_1);
        // S s_168_3: cast zx s_168_2 -> bv
        let s_168_3: Bits = Bits::new(s_168_2 as u128, 1u16);
        // C s_168_4: const #1u : u8
        let s_168_4: bool = true;
        // C s_168_5: cast zx s_168_4 -> bv
        let s_168_5: Bits = Bits::new(s_168_4 as u128, 1u16);
        // S s_168_6: cmp-eq s_168_3 s_168_5
        let s_168_6: bool = ((s_168_3) == (s_168_5));
        // D s_168_7: write-var gs#68280 <= s_168_6
        fn_state.gs_68280 = s_168_6;
        // N s_168_8: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #424u : u32
        let s_169_0: u32 = 424;
        // D s_169_1: read-reg s_169_0:u8
        let s_169_1: u8 = {
            let value = state.read_register::<u8>(s_169_0 as isize);
            tracer.read_register(s_169_0 as isize, value);
            value
        };
        // C s_169_2: const #2u : u8
        let s_169_2: u8 = 2;
        // D s_169_3: cmp-lt s_169_1 s_169_2
        let s_169_3: bool = ((s_169_1) < (s_169_2));
        // D s_169_4: write-var gs#68279 <= s_169_3
        fn_state.gs_68279 = s_169_3;
        // N s_169_5: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_170_0: panic
        panic!("{:?}", ());
        // N s_170_1: return
        return;
    }
}
