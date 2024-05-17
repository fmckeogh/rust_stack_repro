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
use u_get_HAFGRTR_EL2_Type_AMCNTEN0::*;
use Halted::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use AArch64_SystemAccessTrap::*;
use u__get_AMCNTENSET0_EL0::*;
use X_set::*;
use u_get_CPTR_EL3_Type_TAM::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_AMUSERENR_EL0_Type_EN::*;
use u_get_CPTR_EL2_Type_TAM::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use EDSCR_read::*;
use common::*;
pub fn AMCNTENSET0_EL0_SysRegRead_df359e5215fe4ea4<T: Tracer>(
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
        gs_47288: bool,
        gs_47310: bool,
        gs_47289: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        gs_47305: bool,
        gs_47318: bool,
        u__SCR_EL3_FGTEn: bool,
        u__CPTR_EL3_TAM: bool,
        u__CPTR_EL2_TAM: bool,
        u__AMUSERENR_EL0_EN: bool,
        ga_38527: ProductType5c790c8ef59cc8b2,
        gs_47287: bool,
        gs_47317: bool,
        gs_47293: bool,
        gs_47295: bool,
        gs_47308: bool,
        gs_47301: bool,
        gs_47307: bool,
        gs_47311: bool,
        gs_47312: bool,
        gs_47290: bool,
        gs_47309: bool,
        gs_47315: bool,
        gs_47296: bool,
        gs_47292: bool,
        gs_47286: bool,
        gs_47294: bool,
        ga_38507: ProductType5c790c8ef59cc8b2,
        gs_47306: bool,
        gs_47299: bool,
        u__HAFGRTR_EL2_AMCNTEN0: bool,
        gs_47297: bool,
        u__PSTATE_EL: u8,
        gs_47314: bool,
        ga_38523: ProductType5c790c8ef59cc8b2,
        gs_47302: bool,
        gs_47298: bool,
        gs_47304: bool,
        ga_38481: ProductType5c790c8ef59cc8b2,
        gs_47300: bool,
        gs_47313: bool,
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
        // C s_0_7: const #16840u : u32
        let s_0_7: u32 = 16840;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_CPTR_EL3_Type_TAM(s_0_8)
        let s_0_9: bool = u_get_CPTR_EL3_Type_TAM(state, tracer, s_0_8);
        // D s_0_10: write-var __CPTR_EL3_TAM <= s_0_9
        fn_state.u__CPTR_EL3_TAM = s_0_9;
        // C s_0_11: const #90496u : u32
        let s_0_11: u32 = 90496;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_AMUSERENR_EL0_Type_EN(s_0_12)
        let s_0_13: bool = u_get_AMUSERENR_EL0_Type_EN(state, tracer, s_0_12);
        // D s_0_14: write-var __AMUSERENR_EL0_EN <= s_0_13
        fn_state.u__AMUSERENR_EL0_EN = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_TGE(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_TGE <= s_0_17
        fn_state.u__HCR_EL2_TGE = s_0_17;
        // C s_0_19: const #11088u : u32
        let s_0_19: u32 = 11088;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_CPTR_EL2_Type_TAM(s_0_20)
        let s_0_21: bool = u_get_CPTR_EL2_Type_TAM(state, tracer, s_0_20);
        // D s_0_22: write-var __CPTR_EL2_TAM <= s_0_21
        fn_state.u__CPTR_EL2_TAM = s_0_21;
        // C s_0_23: const #90704u : u32
        let s_0_23: u32 = 90704;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_SCR_EL3_Type_FGTEn(s_0_24)
        let s_0_25: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_24);
        // D s_0_26: write-var __SCR_EL3_FGTEn <= s_0_25
        fn_state.u__SCR_EL3_FGTEn = s_0_25;
        // C s_0_27: const #21760u : u32
        let s_0_27: u32 = 21760;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_HAFGRTR_EL2_Type_AMCNTEN0(s_0_28)
        let s_0_29: bool = u_get_HAFGRTR_EL2_Type_AMCNTEN0(state, tracer, s_0_28);
        // D s_0_30: write-var __HAFGRTR_EL2_AMCNTEN0 <= s_0_29
        fn_state.u__HAFGRTR_EL2_AMCNTEN0 = s_0_29;
        // D s_0_31: read-var __PSTATE_EL:u8
        let s_0_31: u8 = fn_state.u__PSTATE_EL;
        // D s_0_32: cast zx s_0_31 -> bv
        let s_0_32: Bits = Bits::new(s_0_31 as u128, 2u16);
        // C s_0_33: const #448u : u32
        let s_0_33: u32 = 448;
        // D s_0_34: read-reg s_0_33:u8
        let s_0_34: u8 = {
            let value = state.read_register::<u8>(s_0_33 as isize);
            tracer.read_register(s_0_33 as isize, value);
            value
        };
        // D s_0_35: cast zx s_0_34 -> bv
        let s_0_35: Bits = Bits::new(s_0_34 as u128, 2u16);
        // D s_0_36: cmp-eq s_0_32 s_0_35
        let s_0_36: bool = ((s_0_32) == (s_0_35));
        // N s_0_37: branch s_0_36 b75 b1
        if s_0_36 {
            return block_75(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b31 b2
        if s_1_5 {
            return block_31(state, tracer, fn_state);
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
        // C s_5_1: const #104808u : u32
        let s_5_1: u32 = 104808;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_AMCNTENSET0_EL0(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_AMCNTENSET0_EL0(
            state,
            tracer,
            s_5_2,
        );
        // D s_5_4: write-var ga#38527 <= s_5_3
        fn_state.ga_38527 = s_5_3;
        // D s_5_5: read-var ga#38527.0:struct
        let s_5_5: u64 = fn_state.ga_38527._0;
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
        // N s_6_2: branch s_6_1 b30 b7
        if s_6_1 {
            return block_30(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#47286 <= s_7_0
        fn_state.gs_47286 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#47286:u8
        let s_8_0: bool = fn_state.gs_47286;
        // N s_8_1: branch s_8_0 b29 b9
        if s_8_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#47287 <= s_9_0
        fn_state.gs_47287 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#47287:u8
        let s_10_0: bool = fn_state.gs_47287;
        // N s_10_1: branch s_10_0 b28 b11
        if s_10_0 {
            return block_28(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#47288 <= s_11_0
        fn_state.gs_47288 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#47288:u8
        let s_12_0: bool = fn_state.gs_47288;
        // N s_12_1: branch s_12_0 b27 b13
        if s_12_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#47289 <= s_13_0
        fn_state.gs_47289 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#47289:u8
        let s_14_0: bool = fn_state.gs_47289;
        // N s_14_1: branch s_14_0 b26 b15
        if s_14_0 {
            return block_26(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#47290 <= s_16_0
        fn_state.gs_47290 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#47290:u8
        let s_17_0: bool = fn_state.gs_47290;
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
        // C s_18_1: const #104808u : u32
        let s_18_1: u32 = 104808;
        // D s_18_2: read-reg s_18_1:struct
        let s_18_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_1 as isize);
            tracer.read_register(s_18_1 as isize, value);
            value
        };
        // D s_18_3: call __get_AMCNTENSET0_EL0(s_18_2)
        let s_18_3: ProductType5c790c8ef59cc8b2 = u__get_AMCNTENSET0_EL0(
            state,
            tracer,
            s_18_2,
        );
        // D s_18_4: write-var ga#38523 <= s_18_3
        fn_state.ga_38523 = s_18_3;
        // D s_18_5: read-var ga#38523.0:struct
        let s_18_5: u64 = fn_state.ga_38523._0;
        // D s_18_6: cast zx s_18_5 -> bv
        let s_18_6: Bits = Bits::new(s_18_5 as u128, 64u16);
        // D s_18_7: read-var t:i
        let s_18_7: i128 = fn_state.t;
        // D s_18_8: call X_set(s_18_7, s_18_0, s_18_6)
        let s_18_8: () = X_set(state, tracer, s_18_7, s_18_0, s_18_6);
        // N s_18_9: return
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
        // D s_20_1: write-var gs#47292 <= s_20_0
        fn_state.gs_47292 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#47292:u8
        let s_21_0: bool = fn_state.gs_47292;
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
        // D s_24_0: read-var __EDSCR_SDD:u8
        let s_24_0: bool = fn_state.u__EDSCR_SDD;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#47292 <= s_24_4
        fn_state.gs_47292 = s_24_4;
        // N s_24_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __CPTR_EL3_TAM:u8
        let s_25_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#47290 <= s_25_4
        fn_state.gs_47290 = s_25_4;
        // N s_25_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __CPTR_EL3_TAM:u8
        let s_27_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #1u : u8
        let s_27_2: bool = true;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#47289 <= s_27_4
        fn_state.gs_47289 = s_27_4;
        // N s_27_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_28_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_28_1: call __IMPDEF_boolean(s_28_0)
        let s_28_1: bool = u__IMPDEF_boolean(state, tracer, s_28_0);
        // D s_28_2: write-var gs#47288 <= s_28_1
        fn_state.gs_47288 = s_28_1;
        // N s_28_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __EDSCR_SDD:u8
        let s_29_0: bool = fn_state.u__EDSCR_SDD;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#47287 <= s_29_4
        fn_state.gs_47287 = s_29_4;
        // N s_29_6: jump b10
        return block_10(state, tracer, fn_state);
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
        // D s_30_4: write-var gs#47286 <= s_30_3
        fn_state.gs_47286 = s_30_3;
        // N s_30_5: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_31_2: branch s_31_1 b74 b32
        if s_31_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#47293 <= s_32_0
        fn_state.gs_47293 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#47293:u8
        let s_33_0: bool = fn_state.gs_47293;
        // N s_33_1: branch s_33_0 b73 b34
        if s_33_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#47294 <= s_34_0
        fn_state.gs_47294 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#47294:u8
        let s_35_0: bool = fn_state.gs_47294;
        // N s_35_1: branch s_35_0 b72 b36
        if s_35_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#47295 <= s_36_0
        fn_state.gs_47295 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#47295:u8
        let s_37_0: bool = fn_state.gs_47295;
        // N s_37_1: branch s_37_0 b71 b38
        if s_37_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#47296 <= s_38_0
        fn_state.gs_47296 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#47296:u8
        let s_39_0: bool = fn_state.gs_47296;
        // N s_39_1: branch s_39_0 b70 b40
        if s_39_0 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EL2Enabled(s_40_0)
        let s_40_1: bool = EL2Enabled(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b69 b41
        if s_40_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#47297 <= s_41_0
        fn_state.gs_47297 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#47297:u8
        let s_42_0: bool = fn_state.gs_47297;
        // N s_42_1: branch s_42_0 b68 b43
        if s_42_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
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
        // N s_43_2: branch s_43_1 b67 b44
        if s_43_1 {
            return block_67(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#47298 <= s_44_0
        fn_state.gs_47298 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#47298:u8
        let s_45_0: bool = fn_state.gs_47298;
        // N s_45_1: branch s_45_0 b63 b46
        if s_45_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#47300 <= s_46_0
        fn_state.gs_47300 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#47300:u8
        let s_47_0: bool = fn_state.gs_47300;
        // N s_47_1: branch s_47_0 b62 b48
        if s_47_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#47301 <= s_48_0
        fn_state.gs_47301 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#47301:u8
        let s_49_0: bool = fn_state.gs_47301;
        // N s_49_1: branch s_49_0 b61 b50
        if s_49_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #424u : u32
        let s_50_0: u32 = 424;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: u8 = {
            let value = state.read_register::<u8>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // C s_50_2: const #2u : u8
        let s_50_2: u8 = 2;
        // D s_50_3: cmp-lt s_50_1 s_50_2
        let s_50_3: bool = ((s_50_1) < (s_50_2));
        // N s_50_4: branch s_50_3 b60 b51
        if s_50_3 {
            return block_60(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#47302 <= s_51_0
        fn_state.gs_47302 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#47302:u8
        let s_52_0: bool = fn_state.gs_47302;
        // N s_52_1: branch s_52_0 b54 b53
        if s_52_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #64s : i64
        let s_53_0: i64 = 64;
        // C s_53_1: const #104808u : u32
        let s_53_1: u32 = 104808;
        // D s_53_2: read-reg s_53_1:struct
        let s_53_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_53_1 as isize);
            tracer.read_register(s_53_1 as isize, value);
            value
        };
        // D s_53_3: call __get_AMCNTENSET0_EL0(s_53_2)
        let s_53_3: ProductType5c790c8ef59cc8b2 = u__get_AMCNTENSET0_EL0(
            state,
            tracer,
            s_53_2,
        );
        // D s_53_4: write-var ga#38507 <= s_53_3
        fn_state.ga_38507 = s_53_3;
        // D s_53_5: read-var ga#38507.0:struct
        let s_53_5: u64 = fn_state.ga_38507._0;
        // D s_53_6: cast zx s_53_5 -> bv
        let s_53_6: Bits = Bits::new(s_53_5 as u128, 64u16);
        // D s_53_7: read-var t:i
        let s_53_7: i128 = fn_state.t;
        // D s_53_8: call X_set(s_53_7, s_53_0, s_53_6)
        let s_53_8: () = X_set(state, tracer, s_53_7, s_53_0, s_53_6);
        // N s_53_9: return
        return;
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
        // D s_55_1: write-var gs#47304 <= s_55_0
        fn_state.gs_47304 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#47304:u8
        let s_56_0: bool = fn_state.gs_47304;
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
        // D s_59_5: write-var gs#47304 <= s_59_4
        fn_state.gs_47304 = s_59_4;
        // N s_59_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __CPTR_EL3_TAM:u8
        let s_60_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #1u : u8
        let s_60_2: bool = true;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#47302 <= s_60_4
        fn_state.gs_47302 = s_60_4;
        // N s_60_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #24u : u8
        let s_61_0: u8 = 24;
        // C s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 8u16);
        // C s_61_2: cast zx s_61_1 -> i
        let s_61_2: i128 = (s_61_1.value() as i128);
        // C s_61_3: cast reint s_61_2 -> i64
        let s_61_3: i64 = (s_61_2 as i64);
        // C s_61_4: cast zx s_61_3 -> i
        let s_61_4: i128 = (i128::try_from(s_61_3).unwrap());
        // C s_61_5: const #432u : u32
        let s_61_5: u32 = 432;
        // D s_61_6: read-reg s_61_5:u8
        let s_61_6: u8 = {
            let value = state.read_register::<u8>(s_61_5 as isize);
            tracer.read_register(s_61_5 as isize, value);
            value
        };
        // D s_61_7: call AArch64_SystemAccessTrap(s_61_6, s_61_4)
        let s_61_7: () = AArch64_SystemAccessTrap(state, tracer, s_61_6, s_61_4);
        // N s_61_8: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var __HAFGRTR_EL2_AMCNTEN0:u8
        let s_62_0: bool = fn_state.u__HAFGRTR_EL2_AMCNTEN0;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#47301 <= s_62_4
        fn_state.gs_47301 = s_62_4;
        // N s_62_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #424u : u32
        let s_63_0: u32 = 424;
        // D s_63_1: read-reg s_63_0:u8
        let s_63_1: u8 = {
            let value = state.read_register::<u8>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // C s_63_2: const #2u : u8
        let s_63_2: u8 = 2;
        // D s_63_3: cmp-lt s_63_1 s_63_2
        let s_63_3: bool = ((s_63_1) < (s_63_2));
        // D s_63_4: not s_63_3
        let s_63_4: bool = !s_63_3;
        // N s_63_5: branch s_63_4 b66 b64
        if s_63_4 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __SCR_EL3_FGTEn:u8
        let s_64_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#47299 <= s_64_4
        fn_state.gs_47299 = s_64_4;
        // N s_64_6: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#47299:u8
        let s_65_0: bool = fn_state.gs_47299;
        // D s_65_1: write-var gs#47300 <= s_65_0
        fn_state.gs_47300 = s_65_0;
        // N s_65_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: write-var gs#47299 <= s_66_0
        fn_state.gs_47299 = s_66_0;
        // N s_66_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #146u : u32
        let s_67_0: u32 = 146;
        // S s_67_1: call IsFeatureImplemented(s_67_0)
        let s_67_1: bool = IsFeatureImplemented(state, tracer, s_67_0);
        // D s_67_2: write-var gs#47298 <= s_67_1
        fn_state.gs_47298 = s_67_1;
        // N s_67_3: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #24u : u8
        let s_68_0: u8 = 24;
        // C s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 8u16);
        // C s_68_2: cast zx s_68_1 -> i
        let s_68_2: i128 = (s_68_1.value() as i128);
        // C s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: cast zx s_68_3 -> i
        let s_68_4: i128 = (i128::try_from(s_68_3).unwrap());
        // C s_68_5: const #432u : u32
        let s_68_5: u32 = 432;
        // D s_68_6: read-reg s_68_5:u8
        let s_68_6: u8 = {
            let value = state.read_register::<u8>(s_68_5 as isize);
            tracer.read_register(s_68_5 as isize, value);
            value
        };
        // D s_68_7: call AArch64_SystemAccessTrap(s_68_6, s_68_4)
        let s_68_7: () = AArch64_SystemAccessTrap(state, tracer, s_68_6, s_68_4);
        // N s_68_8: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __CPTR_EL2_TAM:u8
        let s_69_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#47297 <= s_69_4
        fn_state.gs_47297 = s_69_4;
        // N s_69_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_70_0: panic
        panic!("{:?}", ());
        // N s_70_1: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __CPTR_EL3_TAM:u8
        let s_71_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #1u : u8
        let s_71_2: bool = true;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#47296 <= s_71_4
        fn_state.gs_47296 = s_71_4;
        // N s_71_6: jump b39
        return block_39(state, tracer, fn_state);
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
        // D s_72_2: write-var gs#47295 <= s_72_1
        fn_state.gs_47295 = s_72_1;
        // N s_72_3: jump b37
        return block_37(state, tracer, fn_state);
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
        // D s_73_5: write-var gs#47294 <= s_73_4
        fn_state.gs_47294 = s_73_4;
        // N s_73_6: jump b35
        return block_35(state, tracer, fn_state);
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
        // D s_74_4: write-var gs#47293 <= s_74_3
        fn_state.gs_47293 = s_74_3;
        // N s_74_5: jump b33
        return block_33(state, tracer, fn_state);
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
        // N s_75_2: branch s_75_1 b128 b76
        if s_75_1 {
            return block_128(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#47305 <= s_76_0
        fn_state.gs_47305 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#47305:u8
        let s_77_0: bool = fn_state.gs_47305;
        // N s_77_1: branch s_77_0 b127 b78
        if s_77_0 {
            return block_127(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#47306 <= s_78_0
        fn_state.gs_47306 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#47306:u8
        let s_79_0: bool = fn_state.gs_47306;
        // N s_79_1: branch s_79_0 b126 b80
        if s_79_0 {
            return block_126(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#47307 <= s_80_0
        fn_state.gs_47307 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#47307:u8
        let s_81_0: bool = fn_state.gs_47307;
        // N s_81_1: branch s_81_0 b125 b82
        if s_81_0 {
            return block_125(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#47308 <= s_82_0
        fn_state.gs_47308 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#47308:u8
        let s_83_0: bool = fn_state.gs_47308;
        // N s_83_1: branch s_83_0 b124 b84
        if s_83_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var __AMUSERENR_EL0_EN:u8
        let s_84_0: bool = fn_state.u__AMUSERENR_EL0_EN;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #0u : u8
        let s_84_2: bool = false;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // N s_84_5: branch s_84_4 b118 b85
        if s_84_4 {
            return block_118(state, tracer, fn_state);
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
        // N s_85_2: branch s_85_1 b117 b86
        if s_85_1 {
            return block_117(state, tracer, fn_state);
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
        // D s_86_1: write-var gs#47309 <= s_86_0
        fn_state.gs_47309 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#47309:u8
        let s_87_0: bool = fn_state.gs_47309;
        // N s_87_1: branch s_87_0 b116 b88
        if s_87_0 {
            return block_116(state, tracer, fn_state);
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
        // N s_88_2: branch s_88_1 b115 b89
        if s_88_1 {
            return block_115(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#47310 <= s_89_0
        fn_state.gs_47310 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#47310:u8
        let s_90_0: bool = fn_state.gs_47310;
        // N s_90_1: branch s_90_0 b114 b91
        if s_90_0 {
            return block_114(state, tracer, fn_state);
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
        // D s_91_1: write-var gs#47311 <= s_91_0
        fn_state.gs_47311 = s_91_0;
        // N s_91_2: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var gs#47311:u8
        let s_92_0: bool = fn_state.gs_47311;
        // N s_92_1: branch s_92_0 b110 b93
        if s_92_0 {
            return block_110(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#47313 <= s_93_0
        fn_state.gs_47313 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#47313:u8
        let s_94_0: bool = fn_state.gs_47313;
        // N s_94_1: branch s_94_0 b109 b95
        if s_94_0 {
            return block_109(state, tracer, fn_state);
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
        // D s_95_1: write-var gs#47314 <= s_95_0
        fn_state.gs_47314 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#47314:u8
        let s_96_0: bool = fn_state.gs_47314;
        // N s_96_1: branch s_96_0 b108 b97
        if s_96_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #424u : u32
        let s_97_0: u32 = 424;
        // D s_97_1: read-reg s_97_0:u8
        let s_97_1: u8 = {
            let value = state.read_register::<u8>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // C s_97_2: const #2u : u8
        let s_97_2: u8 = 2;
        // D s_97_3: cmp-lt s_97_1 s_97_2
        let s_97_3: bool = ((s_97_1) < (s_97_2));
        // N s_97_4: branch s_97_3 b107 b98
        if s_97_3 {
            return block_107(state, tracer, fn_state);
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
        // D s_98_1: write-var gs#47315 <= s_98_0
        fn_state.gs_47315 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#47315:u8
        let s_99_0: bool = fn_state.gs_47315;
        // N s_99_1: branch s_99_0 b101 b100
        if s_99_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #64s : i64
        let s_100_0: i64 = 64;
        // C s_100_1: const #104808u : u32
        let s_100_1: u32 = 104808;
        // D s_100_2: read-reg s_100_1:struct
        let s_100_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_100_1 as isize);
            tracer.read_register(s_100_1 as isize, value);
            value
        };
        // D s_100_3: call __get_AMCNTENSET0_EL0(s_100_2)
        let s_100_3: ProductType5c790c8ef59cc8b2 = u__get_AMCNTENSET0_EL0(
            state,
            tracer,
            s_100_2,
        );
        // D s_100_4: write-var ga#38481 <= s_100_3
        fn_state.ga_38481 = s_100_3;
        // D s_100_5: read-var ga#38481.0:struct
        let s_100_5: u64 = fn_state.ga_38481._0;
        // D s_100_6: cast zx s_100_5 -> bv
        let s_100_6: Bits = Bits::new(s_100_5 as u128, 64u16);
        // D s_100_7: read-var t:i
        let s_100_7: i128 = fn_state.t;
        // D s_100_8: call X_set(s_100_7, s_100_0, s_100_6)
        let s_100_8: () = X_set(state, tracer, s_100_7, s_100_0, s_100_6);
        // N s_100_9: return
        return;
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #() : ()
        let s_101_0: () = ();
        // S s_101_1: call Halted(s_101_0)
        let s_101_1: bool = Halted(state, tracer, s_101_0);
        // N s_101_2: branch s_101_1 b106 b102
        if s_101_1 {
            return block_106(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#47317 <= s_102_0
        fn_state.gs_47317 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#47317:u8
        let s_103_0: bool = fn_state.gs_47317;
        // N s_103_1: branch s_103_0 b105 b104
        if s_103_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #24u : u8
        let s_104_0: u8 = 24;
        // C s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 8u16);
        // C s_104_2: cast zx s_104_1 -> i
        let s_104_2: i128 = (s_104_1.value() as i128);
        // C s_104_3: cast reint s_104_2 -> i64
        let s_104_3: i64 = (s_104_2 as i64);
        // C s_104_4: cast zx s_104_3 -> i
        let s_104_4: i128 = (i128::try_from(s_104_3).unwrap());
        // C s_104_5: const #424u : u32
        let s_104_5: u32 = 424;
        // D s_104_6: read-reg s_104_5:u8
        let s_104_6: u8 = {
            let value = state.read_register::<u8>(s_104_5 as isize);
            tracer.read_register(s_104_5 as isize, value);
            value
        };
        // D s_104_7: call AArch64_SystemAccessTrap(s_104_6, s_104_4)
        let s_104_7: () = AArch64_SystemAccessTrap(state, tracer, s_104_6, s_104_4);
        // N s_104_8: return
        return;
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_105_0: panic
        panic!("{:?}", ());
        // N s_105_1: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var __EDSCR_SDD:u8
        let s_106_0: bool = fn_state.u__EDSCR_SDD;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 1u16);
        // C s_106_2: const #1u : u8
        let s_106_2: bool = true;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: write-var gs#47317 <= s_106_4
        fn_state.gs_47317 = s_106_4;
        // N s_106_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var __CPTR_EL3_TAM:u8
        let s_107_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 1u16);
        // C s_107_2: const #1u : u8
        let s_107_2: bool = true;
        // C s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 1u16);
        // D s_107_4: cmp-eq s_107_1 s_107_3
        let s_107_4: bool = ((s_107_1) == (s_107_3));
        // D s_107_5: write-var gs#47315 <= s_107_4
        fn_state.gs_47315 = s_107_4;
        // N s_107_6: jump b99
        return block_99(state, tracer, fn_state);
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
        // C s_108_5: const #432u : u32
        let s_108_5: u32 = 432;
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
        // D s_109_0: read-var __HAFGRTR_EL2_AMCNTEN0:u8
        let s_109_0: bool = fn_state.u__HAFGRTR_EL2_AMCNTEN0;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 1u16);
        // C s_109_2: const #1u : u8
        let s_109_2: bool = true;
        // C s_109_3: cast zx s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 1u16);
        // D s_109_4: cmp-eq s_109_1 s_109_3
        let s_109_4: bool = ((s_109_1) == (s_109_3));
        // D s_109_5: write-var gs#47314 <= s_109_4
        fn_state.gs_47314 = s_109_4;
        // N s_109_6: jump b96
        return block_96(state, tracer, fn_state);
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
        // D s_110_4: not s_110_3
        let s_110_4: bool = !s_110_3;
        // N s_110_5: branch s_110_4 b113 b111
        if s_110_4 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var __SCR_EL3_FGTEn:u8
        let s_111_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 1u16);
        // C s_111_2: const #1u : u8
        let s_111_2: bool = true;
        // C s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 1u16);
        // D s_111_4: cmp-eq s_111_1 s_111_3
        let s_111_4: bool = ((s_111_1) == (s_111_3));
        // D s_111_5: write-var gs#47312 <= s_111_4
        fn_state.gs_47312 = s_111_4;
        // N s_111_6: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#47312:u8
        let s_112_0: bool = fn_state.gs_47312;
        // D s_112_1: write-var gs#47313 <= s_112_0
        fn_state.gs_47313 = s_112_0;
        // N s_112_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #1u : u8
        let s_113_0: bool = true;
        // D s_113_1: write-var gs#47312 <= s_113_0
        fn_state.gs_47312 = s_113_0;
        // N s_113_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #146u : u32
        let s_114_0: u32 = 146;
        // S s_114_1: call IsFeatureImplemented(s_114_0)
        let s_114_1: bool = IsFeatureImplemented(state, tracer, s_114_0);
        // D s_114_2: write-var gs#47311 <= s_114_1
        fn_state.gs_47311 = s_114_1;
        // N s_114_3: jump b92
        return block_92(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #102552u : u32
        let s_115_0: u32 = 102552;
        // D s_115_1: read-reg s_115_0:struct
        let s_115_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_115_0 as isize);
            tracer.read_register(s_115_0 as isize, value);
            value
        };
        // D s_115_2: call _get_HCR_EL2_Type_E2H(s_115_1)
        let s_115_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_115_1);
        // C s_115_3: const #102552u : u32
        let s_115_3: u32 = 102552;
        // D s_115_4: read-reg s_115_3:struct
        let s_115_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_115_3 as isize);
            tracer.read_register(s_115_3 as isize, value);
            value
        };
        // D s_115_5: call _get_HCR_EL2_Type_TGE(s_115_4)
        let s_115_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_115_4);
        // D s_115_6: cast zx s_115_2 -> bv
        let s_115_6: Bits = Bits::new(s_115_2 as u128, 1u16);
        // D s_115_7: cast zx s_115_5 -> bv
        let s_115_7: Bits = Bits::new(s_115_5 as u128, 1u16);
        // D s_115_8: cast reint s_115_6 -> u128
        let s_115_8: u128 = (s_115_6.value() as u128);
        // D s_115_9: size-of s_115_6
        let s_115_9: u16 = s_115_6.length();
        // D s_115_10: cast reint s_115_7 -> u128
        let s_115_10: u128 = (s_115_7.value() as u128);
        // D s_115_11: size-of s_115_7
        let s_115_11: u16 = s_115_7.length();
        // D s_115_12: lsl s_115_8 s_115_11
        let s_115_12: u128 = s_115_8 << s_115_11;
        // D s_115_13: or s_115_12 s_115_10
        let s_115_13: u128 = ((s_115_12) | (s_115_10));
        // D s_115_14: add s_115_9 s_115_11
        let s_115_14: u16 = (s_115_9 + s_115_11);
        // D s_115_15: create-bits s_115_13 s_115_14
        let s_115_15: Bits = Bits::new(s_115_13, s_115_14);
        // D s_115_16: cast reint s_115_15 -> u8
        let s_115_16: u8 = (s_115_15.value() as u8);
        // D s_115_17: cast zx s_115_16 -> bv
        let s_115_17: Bits = Bits::new(s_115_16 as u128, 2u16);
        // C s_115_18: const #3u : u8
        let s_115_18: u8 = 3;
        // C s_115_19: cast zx s_115_18 -> bv
        let s_115_19: Bits = Bits::new(s_115_18 as u128, 2u16);
        // D s_115_20: cmp-ne s_115_17 s_115_19
        let s_115_20: bool = ((s_115_17) != (s_115_19));
        // D s_115_21: write-var gs#47310 <= s_115_20
        fn_state.gs_47310 = s_115_20;
        // N s_115_22: jump b90
        return block_90(state, tracer, fn_state);
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
        // D s_117_0: read-var __CPTR_EL2_TAM:u8
        let s_117_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 1u16);
        // C s_117_2: const #1u : u8
        let s_117_2: bool = true;
        // C s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 1u16);
        // D s_117_4: cmp-eq s_117_1 s_117_3
        let s_117_4: bool = ((s_117_1) == (s_117_3));
        // D s_117_5: write-var gs#47309 <= s_117_4
        fn_state.gs_47309 = s_117_4;
        // N s_117_6: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call EL2Enabled(s_118_0)
        let s_118_1: bool = EL2Enabled(state, tracer, s_118_0);
        // N s_118_2: branch s_118_1 b123 b119
        if s_118_1 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#47318 <= s_119_0
        fn_state.gs_47318 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#47318:u8
        let s_120_0: bool = fn_state.gs_47318;
        // N s_120_1: branch s_120_0 b122 b121
        if s_120_0 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #24u : u8
        let s_121_0: u8 = 24;
        // C s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 8u16);
        // C s_121_2: cast zx s_121_1 -> i
        let s_121_2: i128 = (s_121_1.value() as i128);
        // C s_121_3: cast reint s_121_2 -> i64
        let s_121_3: i64 = (s_121_2 as i64);
        // C s_121_4: cast zx s_121_3 -> i
        let s_121_4: i128 = (i128::try_from(s_121_3).unwrap());
        // C s_121_5: const #440u : u32
        let s_121_5: u32 = 440;
        // D s_121_6: read-reg s_121_5:u8
        let s_121_6: u8 = {
            let value = state.read_register::<u8>(s_121_5 as isize);
            tracer.read_register(s_121_5 as isize, value);
            value
        };
        // D s_121_7: call AArch64_SystemAccessTrap(s_121_6, s_121_4)
        let s_121_7: () = AArch64_SystemAccessTrap(state, tracer, s_121_6, s_121_4);
        // N s_121_8: return
        return;
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
        // C s_122_5: const #432u : u32
        let s_122_5: u32 = 432;
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
        // D s_123_0: read-var __HCR_EL2_TGE:u8
        let s_123_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 1u16);
        // C s_123_2: const #1u : u8
        let s_123_2: bool = true;
        // C s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 1u16);
        // D s_123_4: cmp-eq s_123_1 s_123_3
        let s_123_4: bool = ((s_123_1) == (s_123_3));
        // D s_123_5: write-var gs#47318 <= s_123_4
        fn_state.gs_47318 = s_123_4;
        // N s_123_6: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_124_0: panic
        panic!("{:?}", ());
        // N s_124_1: return
        return;
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var __CPTR_EL3_TAM:u8
        let s_125_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_125_1: cast zx s_125_0 -> bv
        let s_125_1: Bits = Bits::new(s_125_0 as u128, 1u16);
        // C s_125_2: const #1u : u8
        let s_125_2: bool = true;
        // C s_125_3: cast zx s_125_2 -> bv
        let s_125_3: Bits = Bits::new(s_125_2 as u128, 1u16);
        // D s_125_4: cmp-eq s_125_1 s_125_3
        let s_125_4: bool = ((s_125_1) == (s_125_3));
        // D s_125_5: write-var gs#47308 <= s_125_4
        fn_state.gs_47308 = s_125_4;
        // N s_125_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_126_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_126_1: call __IMPDEF_boolean(s_126_0)
        let s_126_1: bool = u__IMPDEF_boolean(state, tracer, s_126_0);
        // D s_126_2: write-var gs#47307 <= s_126_1
        fn_state.gs_47307 = s_126_1;
        // N s_126_3: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_127_0: read-var __EDSCR_SDD:u8
        let s_127_0: bool = fn_state.u__EDSCR_SDD;
        // D s_127_1: cast zx s_127_0 -> bv
        let s_127_1: Bits = Bits::new(s_127_0 as u128, 1u16);
        // C s_127_2: const #1u : u8
        let s_127_2: bool = true;
        // C s_127_3: cast zx s_127_2 -> bv
        let s_127_3: Bits = Bits::new(s_127_2 as u128, 1u16);
        // D s_127_4: cmp-eq s_127_1 s_127_3
        let s_127_4: bool = ((s_127_1) == (s_127_3));
        // D s_127_5: write-var gs#47306 <= s_127_4
        fn_state.gs_47306 = s_127_4;
        // N s_127_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #424u : u32
        let s_128_0: u32 = 424;
        // D s_128_1: read-reg s_128_0:u8
        let s_128_1: u8 = {
            let value = state.read_register::<u8>(s_128_0 as isize);
            tracer.read_register(s_128_0 as isize, value);
            value
        };
        // C s_128_2: const #2u : u8
        let s_128_2: u8 = 2;
        // D s_128_3: cmp-lt s_128_1 s_128_2
        let s_128_3: bool = ((s_128_1) < (s_128_2));
        // D s_128_4: write-var gs#47305 <= s_128_3
        fn_state.gs_47305 = s_128_3;
        // N s_128_5: jump b77
        return block_77(state, tracer, fn_state);
    }
}
