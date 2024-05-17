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
use u__get_AMEVTYPER1_EL0::*;
use Halted::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use AArch64_SystemAccessTrap::*;
use IsG1ActivityMonitorImplemented::*;
use X_set::*;
use u_get_CPTR_EL3_Type_TAM::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_AMUSERENR_EL0_Type_EN::*;
use u_get_CPTR_EL2_Type_TAM::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HAFGRTR_EL2_Type_AMEVTYPER113_EL0::*;
use common::*;
pub fn AMEVTYPER1_EL0_SysRegRead_ace2a16645ca5a01<T: Tracer>(
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
        gs_49335: bool,
        gs_49338: bool,
        gs_49351: bool,
        gs_49360: bool,
        ga_43602: ProductType5c790c8ef59cc8b2,
        u__CPTR_EL3_TAM: bool,
        u__CPTR_EL2_TAM: bool,
        gs_49343: bool,
        gs_49352: bool,
        gs_49347: bool,
        gs_49331: bool,
        gs_49365: bool,
        gs_49334: bool,
        gs_49342: bool,
        gs_49361: bool,
        gs_49355: bool,
        gs_49353: bool,
        u__PSTATE_EL: u8,
        gs_49333: bool,
        gs_49348: bool,
        gs_49344: bool,
        gs_49359: bool,
        gs_49356: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        gs_49345: bool,
        gs_49340: bool,
        ga_43585: ProductType5c790c8ef59cc8b2,
        u__SCR_EL3_FGTEn: bool,
        gs_49332: bool,
        u__AMUSERENR_EL0_EN: bool,
        gs_49362: bool,
        gs_49339: bool,
        gs_49366: bool,
        ga_43558: ProductType5c790c8ef59cc8b2,
        gs_49358: bool,
        gs_49354: bool,
        gs_49346: bool,
        u__HAFGRTR_EL2_AMEVTYPER113_EL0: bool,
        ga_43607: ProductType5c790c8ef59cc8b2,
        gs_49357: bool,
        gs_49341: bool,
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
        // D s_0_29: call _get_HAFGRTR_EL2_Type_AMEVTYPER113_EL0(s_0_28)
        let s_0_29: bool = u_get_HAFGRTR_EL2_Type_AMEVTYPER113_EL0(
            state,
            tracer,
            s_0_28,
        );
        // D s_0_30: write-var __HAFGRTR_EL2_AMEVTYPER113_EL0 <= s_0_29
        fn_state.u__HAFGRTR_EL2_AMEVTYPER113_EL0 = s_0_29;
        // C s_0_31: const #13s : i
        let s_0_31: i128 = 13;
        // S s_0_32: call IsG1ActivityMonitorImplemented(s_0_31)
        let s_0_32: bool = IsG1ActivityMonitorImplemented(state, tracer, s_0_31);
        // S s_0_33: not s_0_32
        let s_0_33: bool = !s_0_32;
        // N s_0_34: branch s_0_33 b130 b1
        if s_0_33 {
            return block_130(state, tracer, fn_state);
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
        // C s_1_2: const #448u : u32
        let s_1_2: u32 = 448;
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
        // N s_1_6: branch s_1_5 b76 b2
        if s_1_5 {
            return block_76(state, tracer, fn_state);
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
        // C s_2_2: const #440u : u32
        let s_2_2: u32 = 440;
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
        // N s_2_6: branch s_2_5 b32 b3
        if s_2_5 {
            return block_32(state, tracer, fn_state);
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
        // C s_3_2: const #432u : u32
        let s_3_2: u32 = 432;
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
        // N s_3_6: branch s_3_5 b7 b4
        if s_3_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var __PSTATE_EL:u8
        let s_4_0: u8 = fn_state.u__PSTATE_EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #424u : u32
        let s_4_2: u32 = 424;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #13s : i
        let s_6_1: i128 = 13;
        // C s_6_2: const #19152u : u32
        let s_6_2: u32 = 19152;
        // D s_6_3: read-reg s_6_2:[struct; 16]
        let s_6_3: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 16usize]>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: read-element s_6_3[s_6_1]
        let s_6_4: ProductType5c790c8ef59cc8b2 = s_6_3[(s_6_1) as usize];
        // D s_6_5: call __get_AMEVTYPER1_EL0(s_6_4)
        let s_6_5: ProductType5c790c8ef59cc8b2 = u__get_AMEVTYPER1_EL0(
            state,
            tracer,
            s_6_4,
        );
        // D s_6_6: write-var ga#43607 <= s_6_5
        fn_state.ga_43607 = s_6_5;
        // D s_6_7: read-var ga#43607.0:struct
        let s_6_7: u64 = fn_state.ga_43607._0;
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
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call Halted(s_7_0)
        let s_7_1: bool = Halted(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b31 b8
        if s_7_1 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#49331 <= s_8_0
        fn_state.gs_49331 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#49331:u8
        let s_9_0: bool = fn_state.gs_49331;
        // N s_9_1: branch s_9_0 b30 b10
        if s_9_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#49332 <= s_10_0
        fn_state.gs_49332 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#49332:u8
        let s_11_0: bool = fn_state.gs_49332;
        // N s_11_1: branch s_11_0 b29 b12
        if s_11_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#49333 <= s_12_0
        fn_state.gs_49333 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#49333:u8
        let s_13_0: bool = fn_state.gs_49333;
        // N s_13_1: branch s_13_0 b28 b14
        if s_13_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#49334 <= s_14_0
        fn_state.gs_49334 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#49334:u8
        let s_15_0: bool = fn_state.gs_49334;
        // N s_15_1: branch s_15_0 b27 b16
        if s_15_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #424u : u32
        let s_16_0: u32 = 424;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // C s_16_2: const #2u : u8
        let s_16_2: u8 = 2;
        // D s_16_3: cmp-lt s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) < (s_16_2));
        // N s_16_4: branch s_16_3 b26 b17
        if s_16_3 {
            return block_26(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#49335 <= s_17_0
        fn_state.gs_49335 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#49335:u8
        let s_18_0: bool = fn_state.gs_49335;
        // N s_18_1: branch s_18_0 b20 b19
        if s_18_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #64s : i64
        let s_19_0: i64 = 64;
        // C s_19_1: const #13s : i
        let s_19_1: i128 = 13;
        // C s_19_2: const #19152u : u32
        let s_19_2: u32 = 19152;
        // D s_19_3: read-reg s_19_2:[struct; 16]
        let s_19_3: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 16usize],
                >(s_19_2 as isize);
            tracer.read_register(s_19_2 as isize, value);
            value
        };
        // D s_19_4: read-element s_19_3[s_19_1]
        let s_19_4: ProductType5c790c8ef59cc8b2 = s_19_3[(s_19_1) as usize];
        // D s_19_5: call __get_AMEVTYPER1_EL0(s_19_4)
        let s_19_5: ProductType5c790c8ef59cc8b2 = u__get_AMEVTYPER1_EL0(
            state,
            tracer,
            s_19_4,
        );
        // D s_19_6: write-var ga#43602 <= s_19_5
        fn_state.ga_43602 = s_19_5;
        // D s_19_7: read-var ga#43602.0:struct
        let s_19_7: u64 = fn_state.ga_43602._0;
        // D s_19_8: cast zx s_19_7 -> bv
        let s_19_8: Bits = Bits::new(s_19_7 as u128, 64u16);
        // D s_19_9: read-var t:i
        let s_19_9: i128 = fn_state.t;
        // D s_19_10: call X_set(s_19_9, s_19_0, s_19_8)
        let s_19_10: () = X_set(state, tracer, s_19_9, s_19_0, s_19_8);
        // N s_19_11: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call Halted(s_20_0)
        let s_20_1: bool = Halted(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b25 b21
        if s_20_1 {
            return block_25(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#49338 <= s_21_0
        fn_state.gs_49338 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#49338:u8
        let s_22_0: bool = fn_state.gs_49338;
        // N s_22_1: branch s_22_0 b24 b23
        if s_22_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #24u : u8
        let s_23_0: u8 = 24;
        // C s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 8u16);
        // C s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (s_23_1.value() as i128);
        // C s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #424u : u32
        let s_23_5: u32 = 424;
        // D s_23_6: read-reg s_23_5:u8
        let s_23_6: u8 = {
            let value = state.read_register::<u8>(s_23_5 as isize);
            tracer.read_register(s_23_5 as isize, value);
            value
        };
        // D s_23_7: call AArch64_SystemAccessTrap(s_23_6, s_23_4)
        let s_23_7: () = AArch64_SystemAccessTrap(state, tracer, s_23_6, s_23_4);
        // N s_23_8: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __EDSCR_SDD:u8
        let s_25_0: bool = fn_state.u__EDSCR_SDD;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#49338 <= s_25_4
        fn_state.gs_49338 = s_25_4;
        // N s_25_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __CPTR_EL3_TAM:u8
        let s_26_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#49335 <= s_26_4
        fn_state.gs_49335 = s_26_4;
        // N s_26_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var __CPTR_EL3_TAM:u8
        let s_28_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 1u16);
        // C s_28_2: const #1u : u8
        let s_28_2: bool = true;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: write-var gs#49334 <= s_28_4
        fn_state.gs_49334 = s_28_4;
        // N s_28_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_29_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_29_1: call __IMPDEF_boolean(s_29_0)
        let s_29_1: bool = u__IMPDEF_boolean(state, tracer, s_29_0);
        // D s_29_2: write-var gs#49333 <= s_29_1
        fn_state.gs_49333 = s_29_1;
        // N s_29_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __EDSCR_SDD:u8
        let s_30_0: bool = fn_state.u__EDSCR_SDD;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#49332 <= s_30_4
        fn_state.gs_49332 = s_30_4;
        // N s_30_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #424u : u32
        let s_31_0: u32 = 424;
        // D s_31_1: read-reg s_31_0:u8
        let s_31_1: u8 = {
            let value = state.read_register::<u8>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // C s_31_2: const #2u : u8
        let s_31_2: u8 = 2;
        // D s_31_3: cmp-lt s_31_1 s_31_2
        let s_31_3: bool = ((s_31_1) < (s_31_2));
        // D s_31_4: write-var gs#49331 <= s_31_3
        fn_state.gs_49331 = s_31_3;
        // N s_31_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call Halted(s_32_0)
        let s_32_1: bool = Halted(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b75 b33
        if s_32_1 {
            return block_75(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#49339 <= s_33_0
        fn_state.gs_49339 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#49339:u8
        let s_34_0: bool = fn_state.gs_49339;
        // N s_34_1: branch s_34_0 b74 b35
        if s_34_0 {
            return block_74(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#49340 <= s_35_0
        fn_state.gs_49340 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#49340:u8
        let s_36_0: bool = fn_state.gs_49340;
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
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#49341 <= s_37_0
        fn_state.gs_49341 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#49341:u8
        let s_38_0: bool = fn_state.gs_49341;
        // N s_38_1: branch s_38_0 b72 b39
        if s_38_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#49342 <= s_39_0
        fn_state.gs_49342 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#49342:u8
        let s_40_0: bool = fn_state.gs_49342;
        // N s_40_1: branch s_40_0 b71 b41
        if s_40_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call EL2Enabled(s_41_0)
        let s_41_1: bool = EL2Enabled(state, tracer, s_41_0);
        // N s_41_2: branch s_41_1 b70 b42
        if s_41_1 {
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
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#49343 <= s_42_0
        fn_state.gs_49343 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#49343:u8
        let s_43_0: bool = fn_state.gs_49343;
        // N s_43_1: branch s_43_0 b69 b44
        if s_43_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #() : ()
        let s_44_0: () = ();
        // S s_44_1: call EL2Enabled(s_44_0)
        let s_44_1: bool = EL2Enabled(state, tracer, s_44_0);
        // N s_44_2: branch s_44_1 b68 b45
        if s_44_1 {
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
        // D s_45_1: write-var gs#49344 <= s_45_0
        fn_state.gs_49344 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#49344:u8
        let s_46_0: bool = fn_state.gs_49344;
        // N s_46_1: branch s_46_0 b64 b47
        if s_46_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#49346 <= s_47_0
        fn_state.gs_49346 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#49346:u8
        let s_48_0: bool = fn_state.gs_49346;
        // N s_48_1: branch s_48_0 b63 b49
        if s_48_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#49347 <= s_49_0
        fn_state.gs_49347 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#49347:u8
        let s_50_0: bool = fn_state.gs_49347;
        // N s_50_1: branch s_50_0 b62 b51
        if s_50_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #424u : u32
        let s_51_0: u32 = 424;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: u8 = {
            let value = state.read_register::<u8>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // C s_51_2: const #2u : u8
        let s_51_2: u8 = 2;
        // D s_51_3: cmp-lt s_51_1 s_51_2
        let s_51_3: bool = ((s_51_1) < (s_51_2));
        // N s_51_4: branch s_51_3 b61 b52
        if s_51_3 {
            return block_61(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#49348 <= s_52_0
        fn_state.gs_49348 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#49348:u8
        let s_53_0: bool = fn_state.gs_49348;
        // N s_53_1: branch s_53_0 b55 b54
        if s_53_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #64s : i64
        let s_54_0: i64 = 64;
        // C s_54_1: const #13s : i
        let s_54_1: i128 = 13;
        // C s_54_2: const #19152u : u32
        let s_54_2: u32 = 19152;
        // D s_54_3: read-reg s_54_2:[struct; 16]
        let s_54_3: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 16usize],
                >(s_54_2 as isize);
            tracer.read_register(s_54_2 as isize, value);
            value
        };
        // D s_54_4: read-element s_54_3[s_54_1]
        let s_54_4: ProductType5c790c8ef59cc8b2 = s_54_3[(s_54_1) as usize];
        // D s_54_5: call __get_AMEVTYPER1_EL0(s_54_4)
        let s_54_5: ProductType5c790c8ef59cc8b2 = u__get_AMEVTYPER1_EL0(
            state,
            tracer,
            s_54_4,
        );
        // D s_54_6: write-var ga#43585 <= s_54_5
        fn_state.ga_43585 = s_54_5;
        // D s_54_7: read-var ga#43585.0:struct
        let s_54_7: u64 = fn_state.ga_43585._0;
        // D s_54_8: cast zx s_54_7 -> bv
        let s_54_8: Bits = Bits::new(s_54_7 as u128, 64u16);
        // D s_54_9: read-var t:i
        let s_54_9: i128 = fn_state.t;
        // D s_54_10: call X_set(s_54_9, s_54_0, s_54_8)
        let s_54_10: () = X_set(state, tracer, s_54_9, s_54_0, s_54_8);
        // N s_54_11: return
        return;
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
        // N s_55_2: branch s_55_1 b60 b56
        if s_55_1 {
            return block_60(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#49351 <= s_56_0
        fn_state.gs_49351 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#49351:u8
        let s_57_0: bool = fn_state.gs_49351;
        // N s_57_1: branch s_57_0 b59 b58
        if s_57_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #24u : u8
        let s_58_0: u8 = 24;
        // C s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 8u16);
        // C s_58_2: cast zx s_58_1 -> i
        let s_58_2: i128 = (s_58_1.value() as i128);
        // C s_58_3: cast reint s_58_2 -> i64
        let s_58_3: i64 = (s_58_2 as i64);
        // C s_58_4: cast zx s_58_3 -> i
        let s_58_4: i128 = (i128::try_from(s_58_3).unwrap());
        // C s_58_5: const #424u : u32
        let s_58_5: u32 = 424;
        // D s_58_6: read-reg s_58_5:u8
        let s_58_6: u8 = {
            let value = state.read_register::<u8>(s_58_5 as isize);
            tracer.read_register(s_58_5 as isize, value);
            value
        };
        // D s_58_7: call AArch64_SystemAccessTrap(s_58_6, s_58_4)
        let s_58_7: () = AArch64_SystemAccessTrap(state, tracer, s_58_6, s_58_4);
        // N s_58_8: return
        return;
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
        // D s_60_0: read-var __EDSCR_SDD:u8
        let s_60_0: bool = fn_state.u__EDSCR_SDD;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #1u : u8
        let s_60_2: bool = true;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#49351 <= s_60_4
        fn_state.gs_49351 = s_60_4;
        // N s_60_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var __CPTR_EL3_TAM:u8
        let s_61_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 1u16);
        // C s_61_2: const #1u : u8
        let s_61_2: bool = true;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 1u16);
        // D s_61_4: cmp-eq s_61_1 s_61_3
        let s_61_4: bool = ((s_61_1) == (s_61_3));
        // D s_61_5: write-var gs#49348 <= s_61_4
        fn_state.gs_49348 = s_61_4;
        // N s_61_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #24u : u8
        let s_62_0: u8 = 24;
        // C s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 8u16);
        // C s_62_2: cast zx s_62_1 -> i
        let s_62_2: i128 = (s_62_1.value() as i128);
        // C s_62_3: cast reint s_62_2 -> i64
        let s_62_3: i64 = (s_62_2 as i64);
        // C s_62_4: cast zx s_62_3 -> i
        let s_62_4: i128 = (i128::try_from(s_62_3).unwrap());
        // C s_62_5: const #432u : u32
        let s_62_5: u32 = 432;
        // D s_62_6: read-reg s_62_5:u8
        let s_62_6: u8 = {
            let value = state.read_register::<u8>(s_62_5 as isize);
            tracer.read_register(s_62_5 as isize, value);
            value
        };
        // D s_62_7: call AArch64_SystemAccessTrap(s_62_6, s_62_4)
        let s_62_7: () = AArch64_SystemAccessTrap(state, tracer, s_62_6, s_62_4);
        // N s_62_8: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var __HAFGRTR_EL2_AMEVTYPER113_EL0:u8
        let s_63_0: bool = fn_state.u__HAFGRTR_EL2_AMEVTYPER113_EL0;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #1u : u8
        let s_63_2: bool = true;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#49347 <= s_63_4
        fn_state.gs_49347 = s_63_4;
        // N s_63_6: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #424u : u32
        let s_64_0: u32 = 424;
        // D s_64_1: read-reg s_64_0:u8
        let s_64_1: u8 = {
            let value = state.read_register::<u8>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // C s_64_2: const #2u : u8
        let s_64_2: u8 = 2;
        // D s_64_3: cmp-lt s_64_1 s_64_2
        let s_64_3: bool = ((s_64_1) < (s_64_2));
        // D s_64_4: not s_64_3
        let s_64_4: bool = !s_64_3;
        // N s_64_5: branch s_64_4 b67 b65
        if s_64_4 {
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
        // D s_65_0: read-var __SCR_EL3_FGTEn:u8
        let s_65_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #1u : u8
        let s_65_2: bool = true;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#49345 <= s_65_4
        fn_state.gs_49345 = s_65_4;
        // N s_65_6: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#49345:u8
        let s_66_0: bool = fn_state.gs_49345;
        // D s_66_1: write-var gs#49346 <= s_66_0
        fn_state.gs_49346 = s_66_0;
        // N s_66_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #1u : u8
        let s_67_0: bool = true;
        // D s_67_1: write-var gs#49345 <= s_67_0
        fn_state.gs_49345 = s_67_0;
        // N s_67_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #146u : u32
        let s_68_0: u32 = 146;
        // S s_68_1: call IsFeatureImplemented(s_68_0)
        let s_68_1: bool = IsFeatureImplemented(state, tracer, s_68_0);
        // D s_68_2: write-var gs#49344 <= s_68_1
        fn_state.gs_49344 = s_68_1;
        // N s_68_3: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #24u : u8
        let s_69_0: u8 = 24;
        // C s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 8u16);
        // C s_69_2: cast zx s_69_1 -> i
        let s_69_2: i128 = (s_69_1.value() as i128);
        // C s_69_3: cast reint s_69_2 -> i64
        let s_69_3: i64 = (s_69_2 as i64);
        // C s_69_4: cast zx s_69_3 -> i
        let s_69_4: i128 = (i128::try_from(s_69_3).unwrap());
        // C s_69_5: const #432u : u32
        let s_69_5: u32 = 432;
        // D s_69_6: read-reg s_69_5:u8
        let s_69_6: u8 = {
            let value = state.read_register::<u8>(s_69_5 as isize);
            tracer.read_register(s_69_5 as isize, value);
            value
        };
        // D s_69_7: call AArch64_SystemAccessTrap(s_69_6, s_69_4)
        let s_69_7: () = AArch64_SystemAccessTrap(state, tracer, s_69_6, s_69_4);
        // N s_69_8: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __CPTR_EL2_TAM:u8
        let s_70_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#49343 <= s_70_4
        fn_state.gs_49343 = s_70_4;
        // N s_70_6: jump b43
        return block_43(state, tracer, fn_state);
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
        // D s_72_0: read-var __CPTR_EL3_TAM:u8
        let s_72_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #1u : u8
        let s_72_2: bool = true;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#49342 <= s_72_4
        fn_state.gs_49342 = s_72_4;
        // N s_72_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_73_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_73_1: call __IMPDEF_boolean(s_73_0)
        let s_73_1: bool = u__IMPDEF_boolean(state, tracer, s_73_0);
        // D s_73_2: write-var gs#49341 <= s_73_1
        fn_state.gs_49341 = s_73_1;
        // N s_73_3: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var __EDSCR_SDD:u8
        let s_74_0: bool = fn_state.u__EDSCR_SDD;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // C s_74_2: const #1u : u8
        let s_74_2: bool = true;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: write-var gs#49340 <= s_74_4
        fn_state.gs_49340 = s_74_4;
        // N s_74_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #424u : u32
        let s_75_0: u32 = 424;
        // D s_75_1: read-reg s_75_0:u8
        let s_75_1: u8 = {
            let value = state.read_register::<u8>(s_75_0 as isize);
            tracer.read_register(s_75_0 as isize, value);
            value
        };
        // C s_75_2: const #2u : u8
        let s_75_2: u8 = 2;
        // D s_75_3: cmp-lt s_75_1 s_75_2
        let s_75_3: bool = ((s_75_1) < (s_75_2));
        // D s_75_4: write-var gs#49339 <= s_75_3
        fn_state.gs_49339 = s_75_3;
        // N s_75_5: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #() : ()
        let s_76_0: () = ();
        // S s_76_1: call Halted(s_76_0)
        let s_76_1: bool = Halted(state, tracer, s_76_0);
        // N s_76_2: branch s_76_1 b129 b77
        if s_76_1 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#49352 <= s_77_0
        fn_state.gs_49352 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#49352:u8
        let s_78_0: bool = fn_state.gs_49352;
        // N s_78_1: branch s_78_0 b128 b79
        if s_78_0 {
            return block_128(state, tracer, fn_state);
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
        // D s_79_1: write-var gs#49353 <= s_79_0
        fn_state.gs_49353 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#49353:u8
        let s_80_0: bool = fn_state.gs_49353;
        // N s_80_1: branch s_80_0 b127 b81
        if s_80_0 {
            return block_127(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#49354 <= s_81_0
        fn_state.gs_49354 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#49354:u8
        let s_82_0: bool = fn_state.gs_49354;
        // N s_82_1: branch s_82_0 b126 b83
        if s_82_0 {
            return block_126(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#49355 <= s_83_0
        fn_state.gs_49355 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#49355:u8
        let s_84_0: bool = fn_state.gs_49355;
        // N s_84_1: branch s_84_0 b125 b85
        if s_84_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var __AMUSERENR_EL0_EN:u8
        let s_85_0: bool = fn_state.u__AMUSERENR_EL0_EN;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // C s_85_2: const #0u : u8
        let s_85_2: bool = false;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // N s_85_5: branch s_85_4 b119 b86
        if s_85_4 {
            return block_119(state, tracer, fn_state);
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
        // S s_86_1: call EL2Enabled(s_86_0)
        let s_86_1: bool = EL2Enabled(state, tracer, s_86_0);
        // N s_86_2: branch s_86_1 b118 b87
        if s_86_1 {
            return block_118(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#49356 <= s_87_0
        fn_state.gs_49356 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#49356:u8
        let s_88_0: bool = fn_state.gs_49356;
        // N s_88_1: branch s_88_0 b117 b89
        if s_88_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #() : ()
        let s_89_0: () = ();
        // S s_89_1: call EL2Enabled(s_89_0)
        let s_89_1: bool = EL2Enabled(state, tracer, s_89_0);
        // N s_89_2: branch s_89_1 b116 b90
        if s_89_1 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#49357 <= s_90_0
        fn_state.gs_49357 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#49357:u8
        let s_91_0: bool = fn_state.gs_49357;
        // N s_91_1: branch s_91_0 b115 b92
        if s_91_0 {
            return block_115(state, tracer, fn_state);
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
        // D s_92_1: write-var gs#49358 <= s_92_0
        fn_state.gs_49358 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#49358:u8
        let s_93_0: bool = fn_state.gs_49358;
        // N s_93_1: branch s_93_0 b111 b94
        if s_93_0 {
            return block_111(state, tracer, fn_state);
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
        // D s_94_1: write-var gs#49360 <= s_94_0
        fn_state.gs_49360 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#49360:u8
        let s_95_0: bool = fn_state.gs_49360;
        // N s_95_1: branch s_95_0 b110 b96
        if s_95_0 {
            return block_110(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#49361 <= s_96_0
        fn_state.gs_49361 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#49361:u8
        let s_97_0: bool = fn_state.gs_49361;
        // N s_97_1: branch s_97_0 b109 b98
        if s_97_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #424u : u32
        let s_98_0: u32 = 424;
        // D s_98_1: read-reg s_98_0:u8
        let s_98_1: u8 = {
            let value = state.read_register::<u8>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // C s_98_2: const #2u : u8
        let s_98_2: u8 = 2;
        // D s_98_3: cmp-lt s_98_1 s_98_2
        let s_98_3: bool = ((s_98_1) < (s_98_2));
        // N s_98_4: branch s_98_3 b108 b99
        if s_98_3 {
            return block_108(state, tracer, fn_state);
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
        // D s_99_1: write-var gs#49362 <= s_99_0
        fn_state.gs_49362 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#49362:u8
        let s_100_0: bool = fn_state.gs_49362;
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
        // C s_101_0: const #64s : i64
        let s_101_0: i64 = 64;
        // C s_101_1: const #13s : i
        let s_101_1: i128 = 13;
        // C s_101_2: const #19152u : u32
        let s_101_2: u32 = 19152;
        // D s_101_3: read-reg s_101_2:[struct; 16]
        let s_101_3: [ProductType5c790c8ef59cc8b2; 16usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 16usize],
                >(s_101_2 as isize);
            tracer.read_register(s_101_2 as isize, value);
            value
        };
        // D s_101_4: read-element s_101_3[s_101_1]
        let s_101_4: ProductType5c790c8ef59cc8b2 = s_101_3[(s_101_1) as usize];
        // D s_101_5: call __get_AMEVTYPER1_EL0(s_101_4)
        let s_101_5: ProductType5c790c8ef59cc8b2 = u__get_AMEVTYPER1_EL0(
            state,
            tracer,
            s_101_4,
        );
        // D s_101_6: write-var ga#43558 <= s_101_5
        fn_state.ga_43558 = s_101_5;
        // D s_101_7: read-var ga#43558.0:struct
        let s_101_7: u64 = fn_state.ga_43558._0;
        // D s_101_8: cast zx s_101_7 -> bv
        let s_101_8: Bits = Bits::new(s_101_7 as u128, 64u16);
        // D s_101_9: read-var t:i
        let s_101_9: i128 = fn_state.t;
        // D s_101_10: call X_set(s_101_9, s_101_0, s_101_8)
        let s_101_10: () = X_set(state, tracer, s_101_9, s_101_0, s_101_8);
        // N s_101_11: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #() : ()
        let s_102_0: () = ();
        // S s_102_1: call Halted(s_102_0)
        let s_102_1: bool = Halted(state, tracer, s_102_0);
        // N s_102_2: branch s_102_1 b107 b103
        if s_102_1 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#49365 <= s_103_0
        fn_state.gs_49365 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#49365:u8
        let s_104_0: bool = fn_state.gs_49365;
        // N s_104_1: branch s_104_0 b106 b105
        if s_104_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #24u : u8
        let s_105_0: u8 = 24;
        // C s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 8u16);
        // C s_105_2: cast zx s_105_1 -> i
        let s_105_2: i128 = (s_105_1.value() as i128);
        // C s_105_3: cast reint s_105_2 -> i64
        let s_105_3: i64 = (s_105_2 as i64);
        // C s_105_4: cast zx s_105_3 -> i
        let s_105_4: i128 = (i128::try_from(s_105_3).unwrap());
        // C s_105_5: const #424u : u32
        let s_105_5: u32 = 424;
        // D s_105_6: read-reg s_105_5:u8
        let s_105_6: u8 = {
            let value = state.read_register::<u8>(s_105_5 as isize);
            tracer.read_register(s_105_5 as isize, value);
            value
        };
        // D s_105_7: call AArch64_SystemAccessTrap(s_105_6, s_105_4)
        let s_105_7: () = AArch64_SystemAccessTrap(state, tracer, s_105_6, s_105_4);
        // N s_105_8: return
        return;
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
        // D s_107_0: read-var __EDSCR_SDD:u8
        let s_107_0: bool = fn_state.u__EDSCR_SDD;
        // D s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 1u16);
        // C s_107_2: const #1u : u8
        let s_107_2: bool = true;
        // C s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 1u16);
        // D s_107_4: cmp-eq s_107_1 s_107_3
        let s_107_4: bool = ((s_107_1) == (s_107_3));
        // D s_107_5: write-var gs#49365 <= s_107_4
        fn_state.gs_49365 = s_107_4;
        // N s_107_6: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var __CPTR_EL3_TAM:u8
        let s_108_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 1u16);
        // C s_108_2: const #1u : u8
        let s_108_2: bool = true;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#49362 <= s_108_4
        fn_state.gs_49362 = s_108_4;
        // N s_108_6: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #24u : u8
        let s_109_0: u8 = 24;
        // C s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 8u16);
        // C s_109_2: cast zx s_109_1 -> i
        let s_109_2: i128 = (s_109_1.value() as i128);
        // C s_109_3: cast reint s_109_2 -> i64
        let s_109_3: i64 = (s_109_2 as i64);
        // C s_109_4: cast zx s_109_3 -> i
        let s_109_4: i128 = (i128::try_from(s_109_3).unwrap());
        // C s_109_5: const #432u : u32
        let s_109_5: u32 = 432;
        // D s_109_6: read-reg s_109_5:u8
        let s_109_6: u8 = {
            let value = state.read_register::<u8>(s_109_5 as isize);
            tracer.read_register(s_109_5 as isize, value);
            value
        };
        // D s_109_7: call AArch64_SystemAccessTrap(s_109_6, s_109_4)
        let s_109_7: () = AArch64_SystemAccessTrap(state, tracer, s_109_6, s_109_4);
        // N s_109_8: return
        return;
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var __HAFGRTR_EL2_AMEVTYPER113_EL0:u8
        let s_110_0: bool = fn_state.u__HAFGRTR_EL2_AMEVTYPER113_EL0;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 1u16);
        // C s_110_2: const #1u : u8
        let s_110_2: bool = true;
        // C s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // D s_110_5: write-var gs#49361 <= s_110_4
        fn_state.gs_49361 = s_110_4;
        // N s_110_6: jump b97
        return block_97(state, tracer, fn_state);
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
        // D s_111_4: not s_111_3
        let s_111_4: bool = !s_111_3;
        // N s_111_5: branch s_111_4 b114 b112
        if s_111_4 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var __SCR_EL3_FGTEn:u8
        let s_112_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 1u16);
        // C s_112_2: const #1u : u8
        let s_112_2: bool = true;
        // C s_112_3: cast zx s_112_2 -> bv
        let s_112_3: Bits = Bits::new(s_112_2 as u128, 1u16);
        // D s_112_4: cmp-eq s_112_1 s_112_3
        let s_112_4: bool = ((s_112_1) == (s_112_3));
        // D s_112_5: write-var gs#49359 <= s_112_4
        fn_state.gs_49359 = s_112_4;
        // N s_112_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#49359:u8
        let s_113_0: bool = fn_state.gs_49359;
        // D s_113_1: write-var gs#49360 <= s_113_0
        fn_state.gs_49360 = s_113_0;
        // N s_113_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #1u : u8
        let s_114_0: bool = true;
        // D s_114_1: write-var gs#49359 <= s_114_0
        fn_state.gs_49359 = s_114_0;
        // N s_114_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #146u : u32
        let s_115_0: u32 = 146;
        // S s_115_1: call IsFeatureImplemented(s_115_0)
        let s_115_1: bool = IsFeatureImplemented(state, tracer, s_115_0);
        // D s_115_2: write-var gs#49358 <= s_115_1
        fn_state.gs_49358 = s_115_1;
        // N s_115_3: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #102552u : u32
        let s_116_0: u32 = 102552;
        // D s_116_1: read-reg s_116_0:struct
        let s_116_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // D s_116_2: call _get_HCR_EL2_Type_E2H(s_116_1)
        let s_116_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_116_1);
        // C s_116_3: const #102552u : u32
        let s_116_3: u32 = 102552;
        // D s_116_4: read-reg s_116_3:struct
        let s_116_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_116_3 as isize);
            tracer.read_register(s_116_3 as isize, value);
            value
        };
        // D s_116_5: call _get_HCR_EL2_Type_TGE(s_116_4)
        let s_116_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_116_4);
        // D s_116_6: cast zx s_116_2 -> bv
        let s_116_6: Bits = Bits::new(s_116_2 as u128, 1u16);
        // D s_116_7: cast zx s_116_5 -> bv
        let s_116_7: Bits = Bits::new(s_116_5 as u128, 1u16);
        // D s_116_8: cast reint s_116_6 -> u128
        let s_116_8: u128 = (s_116_6.value() as u128);
        // D s_116_9: size-of s_116_6
        let s_116_9: u16 = s_116_6.length();
        // D s_116_10: cast reint s_116_7 -> u128
        let s_116_10: u128 = (s_116_7.value() as u128);
        // D s_116_11: size-of s_116_7
        let s_116_11: u16 = s_116_7.length();
        // D s_116_12: lsl s_116_8 s_116_11
        let s_116_12: u128 = s_116_8 << s_116_11;
        // D s_116_13: or s_116_12 s_116_10
        let s_116_13: u128 = ((s_116_12) | (s_116_10));
        // D s_116_14: add s_116_9 s_116_11
        let s_116_14: u16 = (s_116_9 + s_116_11);
        // D s_116_15: create-bits s_116_13 s_116_14
        let s_116_15: Bits = Bits::new(s_116_13, s_116_14);
        // D s_116_16: cast reint s_116_15 -> u8
        let s_116_16: u8 = (s_116_15.value() as u8);
        // D s_116_17: cast zx s_116_16 -> bv
        let s_116_17: Bits = Bits::new(s_116_16 as u128, 2u16);
        // C s_116_18: const #3u : u8
        let s_116_18: u8 = 3;
        // C s_116_19: cast zx s_116_18 -> bv
        let s_116_19: Bits = Bits::new(s_116_18 as u128, 2u16);
        // D s_116_20: cmp-ne s_116_17 s_116_19
        let s_116_20: bool = ((s_116_17) != (s_116_19));
        // D s_116_21: write-var gs#49357 <= s_116_20
        fn_state.gs_49357 = s_116_20;
        // N s_116_22: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #24u : u8
        let s_117_0: u8 = 24;
        // C s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 8u16);
        // C s_117_2: cast zx s_117_1 -> i
        let s_117_2: i128 = (s_117_1.value() as i128);
        // C s_117_3: cast reint s_117_2 -> i64
        let s_117_3: i64 = (s_117_2 as i64);
        // C s_117_4: cast zx s_117_3 -> i
        let s_117_4: i128 = (i128::try_from(s_117_3).unwrap());
        // C s_117_5: const #432u : u32
        let s_117_5: u32 = 432;
        // D s_117_6: read-reg s_117_5:u8
        let s_117_6: u8 = {
            let value = state.read_register::<u8>(s_117_5 as isize);
            tracer.read_register(s_117_5 as isize, value);
            value
        };
        // D s_117_7: call AArch64_SystemAccessTrap(s_117_6, s_117_4)
        let s_117_7: () = AArch64_SystemAccessTrap(state, tracer, s_117_6, s_117_4);
        // N s_117_8: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var __CPTR_EL2_TAM:u8
        let s_118_0: bool = fn_state.u__CPTR_EL2_TAM;
        // D s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 1u16);
        // C s_118_2: const #1u : u8
        let s_118_2: bool = true;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_4: cmp-eq s_118_1 s_118_3
        let s_118_4: bool = ((s_118_1) == (s_118_3));
        // D s_118_5: write-var gs#49356 <= s_118_4
        fn_state.gs_49356 = s_118_4;
        // N s_118_6: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #() : ()
        let s_119_0: () = ();
        // S s_119_1: call EL2Enabled(s_119_0)
        let s_119_1: bool = EL2Enabled(state, tracer, s_119_0);
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
        // D s_120_1: write-var gs#49366 <= s_120_0
        fn_state.gs_49366 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#49366:u8
        let s_121_0: bool = fn_state.gs_49366;
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
        // C s_122_5: const #440u : u32
        let s_122_5: u32 = 440;
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
        // C s_123_0: const #24u : u8
        let s_123_0: u8 = 24;
        // C s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 8u16);
        // C s_123_2: cast zx s_123_1 -> i
        let s_123_2: i128 = (s_123_1.value() as i128);
        // C s_123_3: cast reint s_123_2 -> i64
        let s_123_3: i64 = (s_123_2 as i64);
        // C s_123_4: cast zx s_123_3 -> i
        let s_123_4: i128 = (i128::try_from(s_123_3).unwrap());
        // C s_123_5: const #432u : u32
        let s_123_5: u32 = 432;
        // D s_123_6: read-reg s_123_5:u8
        let s_123_6: u8 = {
            let value = state.read_register::<u8>(s_123_5 as isize);
            tracer.read_register(s_123_5 as isize, value);
            value
        };
        // D s_123_7: call AArch64_SystemAccessTrap(s_123_6, s_123_4)
        let s_123_7: () = AArch64_SystemAccessTrap(state, tracer, s_123_6, s_123_4);
        // N s_123_8: return
        return;
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var __HCR_EL2_TGE:u8
        let s_124_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 1u16);
        // C s_124_2: const #1u : u8
        let s_124_2: bool = true;
        // C s_124_3: cast zx s_124_2 -> bv
        let s_124_3: Bits = Bits::new(s_124_2 as u128, 1u16);
        // D s_124_4: cmp-eq s_124_1 s_124_3
        let s_124_4: bool = ((s_124_1) == (s_124_3));
        // D s_124_5: write-var gs#49366 <= s_124_4
        fn_state.gs_49366 = s_124_4;
        // N s_124_6: jump b121
        return block_121(state, tracer, fn_state);
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
        // D s_126_0: read-var __CPTR_EL3_TAM:u8
        let s_126_0: bool = fn_state.u__CPTR_EL3_TAM;
        // D s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 1u16);
        // C s_126_2: const #1u : u8
        let s_126_2: bool = true;
        // C s_126_3: cast zx s_126_2 -> bv
        let s_126_3: Bits = Bits::new(s_126_2 as u128, 1u16);
        // D s_126_4: cmp-eq s_126_1 s_126_3
        let s_126_4: bool = ((s_126_1) == (s_126_3));
        // D s_126_5: write-var gs#49355 <= s_126_4
        fn_state.gs_49355 = s_126_4;
        // N s_126_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_127_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_127_1: call __IMPDEF_boolean(s_127_0)
        let s_127_1: bool = u__IMPDEF_boolean(state, tracer, s_127_0);
        // D s_127_2: write-var gs#49354 <= s_127_1
        fn_state.gs_49354 = s_127_1;
        // N s_127_3: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var __EDSCR_SDD:u8
        let s_128_0: bool = fn_state.u__EDSCR_SDD;
        // D s_128_1: cast zx s_128_0 -> bv
        let s_128_1: Bits = Bits::new(s_128_0 as u128, 1u16);
        // C s_128_2: const #1u : u8
        let s_128_2: bool = true;
        // C s_128_3: cast zx s_128_2 -> bv
        let s_128_3: Bits = Bits::new(s_128_2 as u128, 1u16);
        // D s_128_4: cmp-eq s_128_1 s_128_3
        let s_128_4: bool = ((s_128_1) == (s_128_3));
        // D s_128_5: write-var gs#49353 <= s_128_4
        fn_state.gs_49353 = s_128_4;
        // N s_128_6: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #424u : u32
        let s_129_0: u32 = 424;
        // D s_129_1: read-reg s_129_0:u8
        let s_129_1: u8 = {
            let value = state.read_register::<u8>(s_129_0 as isize);
            tracer.read_register(s_129_0 as isize, value);
            value
        };
        // C s_129_2: const #2u : u8
        let s_129_2: u8 = 2;
        // D s_129_3: cmp-lt s_129_1 s_129_2
        let s_129_3: bool = ((s_129_1) < (s_129_2));
        // D s_129_4: write-var gs#49352 <= s_129_3
        fn_state.gs_49352 = s_129_3;
        // N s_129_5: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_130_0: panic
        panic!("{:?}", ());
        // N s_130_1: return
        return;
    }
}
