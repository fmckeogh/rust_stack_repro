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
use X_set::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_CNTHCTL_EL2_Type_EL0PCTEN::*;
use u_get_SCR_EL3_Type_ECVEn::*;
use u_get_CNTKCTL_EL1_Type_EL0PCTEN::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use u_get_HCR_EL2_Type_TGE::*;
use PhysicalCountInt::*;
use EL2Enabled::*;
use u_get_CNTHCTL_EL2_Type_ECV::*;
use common::*;
pub fn CNTPCT_EL0_SysRegRead_30a0e264d583fe56<T: Tracer>(
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
        gs_57989: bool,
        gs_57991: bool,
        u__HCR_EL2_E2H: bool,
        gs_57992: bool,
        gs_57998: bool,
        gs_57982: bool,
        u__HCR_EL2_TGE: bool,
        gs_58001: bool,
        gs_57981: bool,
        gs_57995: bool,
        u__CNTHCTL_EL2_EL0PCTEN: bool,
        ga_56427: u64,
        gs_57983: bool,
        u__SCR_EL3_ECVEn: bool,
        gs_57994: bool,
        gs_57984: bool,
        u__CNTHCTL_EL2_EL1PCTEN: bool,
        ga_56428: i64,
        gs_57987: bool,
        gs_57993: bool,
        u__CNTHCTL_EL2_ECV: bool,
        u__PSTATE_EL: u8,
        u__CNTKCTL_EL1_EL0PCTEN: bool,
        gs_57997: bool,
        gs_57990: bool,
        gs_57988: bool,
        gs_57996: bool,
        gs_57975: bool,
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
        // D s_0_5: call _get_CNTKCTL_EL1_Type_EL0PCTEN(s_0_4)
        let s_0_5: bool = u_get_CNTKCTL_EL1_Type_EL0PCTEN(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTKCTL_EL1_EL0PCTEN <= s_0_5
        fn_state.u__CNTKCTL_EL1_EL0PCTEN = s_0_5;
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
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_E2H(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_E2H <= s_0_13
        fn_state.u__HCR_EL2_E2H = s_0_13;
        // C s_0_15: const #12808u : u32
        let s_0_15: u32 = 12808;
        // D s_0_16: read-reg s_0_15:u64
        let s_0_16: u64 = {
            let value = state.read_register::<u64>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: write-var ga#56427 <= s_0_16
        fn_state.ga_56427 = s_0_16;
        // C s_0_18: const #54u : u32
        let s_0_18: u32 = 54;
        // S s_0_19: call IsFeatureImplemented(s_0_18)
        let s_0_19: bool = IsFeatureImplemented(state, tracer, s_0_18);
        // N s_0_20: branch s_0_19 b81 b1
        if s_0_19 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#57975 <= s_1_0
        fn_state.gs_57975 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#57975:u8
        let s_2_0: bool = fn_state.gs_57975;
        // N s_2_1: branch s_2_0 b80 b3
        if s_2_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // D s_3_1: write-var ga#56428 <= s_3_0
        fn_state.ga_56428 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1s : i
        let s_4_0: i128 = 1;
        // D s_4_1: read-var ga#56427:u64
        let s_4_1: u64 = fn_state.ga_56427;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 64u16);
        // D s_4_3: read-var ga#56428:i64
        let s_4_3: i64 = fn_state.ga_56428;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: bit-extract s_4_2 s_4_4 s_4_0
        let s_4_5: Bits = (Bits::new(
            ((s_4_2) >> (s_4_4)).value(),
            u16::try_from(s_4_0).unwrap(),
        ));
        // D s_4_6: cast reint s_4_5 -> u8
        let s_4_6: bool = ((s_4_5.value()) != 0);
        // D s_4_7: write-var __CNTHCTL_EL2_EL1PCTEN <= s_4_6
        fn_state.u__CNTHCTL_EL2_EL1PCTEN = s_4_6;
        // C s_4_8: const #12808u : u32
        let s_4_8: u32 = 12808;
        // D s_4_9: read-reg s_4_8:struct
        let s_4_9: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_8 as isize);
            tracer.read_register(s_4_8 as isize, value);
            value
        };
        // D s_4_10: call _get_CNTHCTL_EL2_Type_EL0PCTEN(s_4_9)
        let s_4_10: bool = u_get_CNTHCTL_EL2_Type_EL0PCTEN(state, tracer, s_4_9);
        // D s_4_11: write-var __CNTHCTL_EL2_EL0PCTEN <= s_4_10
        fn_state.u__CNTHCTL_EL2_EL0PCTEN = s_4_10;
        // C s_4_12: const #90704u : u32
        let s_4_12: u32 = 90704;
        // D s_4_13: read-reg s_4_12:struct
        let s_4_13: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_12 as isize);
            tracer.read_register(s_4_12 as isize, value);
            value
        };
        // D s_4_14: call _get_SCR_EL3_Type_ECVEn(s_4_13)
        let s_4_14: bool = u_get_SCR_EL3_Type_ECVEn(state, tracer, s_4_13);
        // D s_4_15: write-var __SCR_EL3_ECVEn <= s_4_14
        fn_state.u__SCR_EL3_ECVEn = s_4_14;
        // C s_4_16: const #12808u : u32
        let s_4_16: u32 = 12808;
        // D s_4_17: read-reg s_4_16:struct
        let s_4_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_16 as isize);
            tracer.read_register(s_4_16 as isize, value);
            value
        };
        // D s_4_18: call _get_CNTHCTL_EL2_Type_ECV(s_4_17)
        let s_4_18: bool = u_get_CNTHCTL_EL2_Type_ECV(state, tracer, s_4_17);
        // D s_4_19: write-var __CNTHCTL_EL2_ECV <= s_4_18
        fn_state.u__CNTHCTL_EL2_ECV = s_4_18;
        // D s_4_20: read-var __PSTATE_EL:u8
        let s_4_20: u8 = fn_state.u__PSTATE_EL;
        // D s_4_21: cast zx s_4_20 -> bv
        let s_4_21: Bits = Bits::new(s_4_20 as u128, 2u16);
        // C s_4_22: const #448u : u32
        let s_4_22: u32 = 448;
        // D s_4_23: read-reg s_4_22:u8
        let s_4_23: u8 = {
            let value = state.read_register::<u8>(s_4_22 as isize);
            tracer.read_register(s_4_22 as isize, value);
            value
        };
        // D s_4_24: cast zx s_4_23 -> bv
        let s_4_24: Bits = Bits::new(s_4_23 as u128, 2u16);
        // D s_4_25: cmp-eq s_4_21 s_4_24
        let s_4_25: bool = ((s_4_21) == (s_4_24));
        // N s_4_26: branch s_4_25 b28 b5
        if s_4_25 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var __PSTATE_EL:u8
        let s_5_0: u8 = fn_state.u__PSTATE_EL;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #440u : u32
        let s_5_2: u32 = 440;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // N s_5_6: branch s_5_5 b11 b6
        if s_5_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __PSTATE_EL:u8
        let s_6_0: u8 = fn_state.u__PSTATE_EL;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #432u : u32
        let s_6_2: u32 = 432;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // N s_6_6: branch s_6_5 b10 b7
        if s_6_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var __PSTATE_EL:u8
        let s_7_0: u8 = fn_state.u__PSTATE_EL;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #424u : u32
        let s_7_2: u32 = 424;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // C s_9_1: const #() : ()
        let s_9_1: () = ();
        // S s_9_2: call PhysicalCountInt(s_9_1)
        let s_9_2: u64 = PhysicalCountInt(state, tracer, s_9_1);
        // S s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 64u16);
        // D s_9_4: read-var t:i
        let s_9_4: i128 = fn_state.t;
        // D s_9_5: call X_set(s_9_4, s_9_0, s_9_3)
        let s_9_5: () = X_set(state, tracer, s_9_4, s_9_0, s_9_3);
        // N s_9_6: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // C s_10_1: const #() : ()
        let s_10_1: () = ();
        // S s_10_2: call PhysicalCountInt(s_10_1)
        let s_10_2: u64 = PhysicalCountInt(state, tracer, s_10_1);
        // S s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 64u16);
        // D s_10_4: read-var t:i
        let s_10_4: i128 = fn_state.t;
        // D s_10_5: call X_set(s_10_4, s_10_0, s_10_3)
        let s_10_5: () = X_set(state, tracer, s_10_4, s_10_0, s_10_3);
        // N s_10_6: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call EL2Enabled(s_11_0)
        let s_11_1: bool = EL2Enabled(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b27 b12
        if s_11_1 {
            return block_27(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#57981 <= s_12_0
        fn_state.gs_57981 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#57981:u8
        let s_13_0: bool = fn_state.gs_57981;
        // N s_13_1: branch s_13_0 b26 b14
        if s_13_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #145u : u32
        let s_14_0: u32 = 145;
        // S s_14_1: call IsFeatureImplemented(s_14_0)
        let s_14_1: bool = IsFeatureImplemented(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b25 b15
        if s_14_1 {
            return block_25(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#57982 <= s_15_0
        fn_state.gs_57982 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#57982:u8
        let s_16_0: bool = fn_state.gs_57982;
        // N s_16_1: branch s_16_0 b24 b17
        if s_16_0 {
            return block_24(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#57983 <= s_17_0
        fn_state.gs_57983 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#57983:u8
        let s_18_0: bool = fn_state.gs_57983;
        // N s_18_1: branch s_18_0 b23 b19
        if s_18_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#57984 <= s_19_0
        fn_state.gs_57984 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#57984:u8
        let s_20_0: bool = fn_state.gs_57984;
        // N s_20_1: branch s_20_0 b22 b21
        if s_20_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #64s : i64
        let s_21_0: i64 = 64;
        // C s_21_1: const #() : ()
        let s_21_1: () = ();
        // S s_21_2: call PhysicalCountInt(s_21_1)
        let s_21_2: u64 = PhysicalCountInt(state, tracer, s_21_1);
        // S s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 64u16);
        // D s_21_4: read-var t:i
        let s_21_4: i128 = fn_state.t;
        // D s_21_5: call X_set(s_21_4, s_21_0, s_21_3)
        let s_21_5: () = X_set(state, tracer, s_21_4, s_21_0, s_21_3);
        // N s_21_6: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #64s : i64
        let s_22_0: i64 = 64;
        // C s_22_1: const #() : ()
        let s_22_1: () = ();
        // S s_22_2: call PhysicalCountInt(s_22_1)
        let s_22_2: u64 = PhysicalCountInt(state, tracer, s_22_1);
        // S s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 64u16);
        // C s_22_4: const #14584u : u32
        let s_22_4: u32 = 14584;
        // D s_22_5: read-reg s_22_4:u64
        let s_22_5: u64 = {
            let value = state.read_register::<u64>(s_22_4 as isize);
            tracer.read_register(s_22_4 as isize, value);
            value
        };
        // D s_22_6: cast zx s_22_5 -> bv
        let s_22_6: Bits = Bits::new(s_22_5 as u128, 64u16);
        // D s_22_7: sub s_22_3 s_22_6
        let s_22_7: Bits = ((s_22_3) - (s_22_6));
        // D s_22_8: cast reint s_22_7 -> u64
        let s_22_8: u64 = (s_22_7.value() as u64);
        // D s_22_9: cast zx s_22_8 -> bv
        let s_22_9: Bits = Bits::new(s_22_8 as u128, 64u16);
        // D s_22_10: read-var t:i
        let s_22_10: i128 = fn_state.t;
        // D s_22_11: call X_set(s_22_10, s_22_0, s_22_9)
        let s_22_11: () = X_set(state, tracer, s_22_10, s_22_0, s_22_9);
        // N s_22_12: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_23_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // C s_23_2: const #1u : u8
        let s_23_2: bool = true;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: write-var gs#57984 <= s_23_4
        fn_state.gs_57984 = s_23_4;
        // N s_23_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var __SCR_EL3_ECVEn:u8
        let s_24_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#57983 <= s_24_4
        fn_state.gs_57983 = s_24_4;
        // N s_24_6: jump b18
        return block_18(state, tracer, fn_state);
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
        // D s_25_2: write-var gs#57982 <= s_25_1
        fn_state.gs_57982 = s_25_1;
        // N s_25_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #24u : u8
        let s_26_0: u8 = 24;
        // C s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 8u16);
        // C s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #432u : u32
        let s_26_5: u32 = 432;
        // D s_26_6: read-reg s_26_5:u8
        let s_26_6: u8 = {
            let value = state.read_register::<u8>(s_26_5 as isize);
            tracer.read_register(s_26_5 as isize, value);
            value
        };
        // D s_26_7: call AArch64_SystemAccessTrap(s_26_6, s_26_4)
        let s_26_7: () = AArch64_SystemAccessTrap(state, tracer, s_26_6, s_26_4);
        // N s_26_8: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __CNTHCTL_EL2_EL1PCTEN:u8
        let s_27_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCTEN;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#57981 <= s_27_4
        fn_state.gs_57981 = s_27_4;
        // N s_27_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EL2Enabled(s_28_0)
        let s_28_1: bool = EL2Enabled(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b79 b29
        if s_28_1 {
            return block_79(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#57987 <= s_29_0
        fn_state.gs_57987 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#57987:u8
        let s_30_0: bool = fn_state.gs_57987;
        // D s_30_1: not s_30_0
        let s_30_1: bool = !s_30_0;
        // N s_30_2: branch s_30_1 b78 b31
        if s_30_1 {
            return block_78(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#57988 <= s_31_0
        fn_state.gs_57988 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#57988:u8
        let s_32_0: bool = fn_state.gs_57988;
        // N s_32_1: branch s_32_0 b72 b33
        if s_32_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call EL2Enabled(s_33_0)
        let s_33_1: bool = EL2Enabled(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b71 b34
        if s_33_1 {
            return block_71(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#57989 <= s_34_0
        fn_state.gs_57989 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#57989:u8
        let s_35_0: bool = fn_state.gs_57989;
        // N s_35_1: branch s_35_0 b70 b36
        if s_35_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#57990 <= s_36_0
        fn_state.gs_57990 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#57990:u8
        let s_37_0: bool = fn_state.gs_57990;
        // N s_37_1: branch s_37_0 b69 b38
        if s_37_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call EL2Enabled(s_38_0)
        let s_38_1: bool = EL2Enabled(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b68 b39
        if s_38_1 {
            return block_68(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#57991 <= s_39_0
        fn_state.gs_57991 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#57991:u8
        let s_40_0: bool = fn_state.gs_57991;
        // N s_40_1: branch s_40_0 b67 b41
        if s_40_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#57992 <= s_41_0
        fn_state.gs_57992 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#57992:u8
        let s_42_0: bool = fn_state.gs_57992;
        // N s_42_1: branch s_42_0 b66 b43
        if s_42_0 {
            return block_66(state, tracer, fn_state);
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
        // N s_43_2: branch s_43_1 b65 b44
        if s_43_1 {
            return block_65(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#57993 <= s_44_0
        fn_state.gs_57993 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#57993:u8
        let s_45_0: bool = fn_state.gs_57993;
        // N s_45_1: branch s_45_0 b64 b46
        if s_45_0 {
            return block_64(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#57994 <= s_46_0
        fn_state.gs_57994 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#57994:u8
        let s_47_0: bool = fn_state.gs_57994;
        // N s_47_1: branch s_47_0 b63 b48
        if s_47_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #145u : u32
        let s_48_0: u32 = 145;
        // S s_48_1: call IsFeatureImplemented(s_48_0)
        let s_48_1: bool = IsFeatureImplemented(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b62 b49
        if s_48_1 {
            return block_62(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#57995 <= s_49_0
        fn_state.gs_57995 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#57995:u8
        let s_50_0: bool = fn_state.gs_57995;
        // N s_50_1: branch s_50_0 b61 b51
        if s_50_0 {
            return block_61(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#57996 <= s_51_0
        fn_state.gs_57996 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#57996:u8
        let s_52_0: bool = fn_state.gs_57996;
        // N s_52_1: branch s_52_0 b60 b53
        if s_52_0 {
            return block_60(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#57997 <= s_53_0
        fn_state.gs_57997 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#57997:u8
        let s_54_0: bool = fn_state.gs_57997;
        // N s_54_1: branch s_54_0 b59 b55
        if s_54_0 {
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
        // D s_55_1: write-var gs#57998 <= s_55_0
        fn_state.gs_57998 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#57998:u8
        let s_56_0: bool = fn_state.gs_57998;
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
        // C s_57_0: const #64s : i64
        let s_57_0: i64 = 64;
        // C s_57_1: const #() : ()
        let s_57_1: () = ();
        // S s_57_2: call PhysicalCountInt(s_57_1)
        let s_57_2: u64 = PhysicalCountInt(state, tracer, s_57_1);
        // S s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 64u16);
        // D s_57_4: read-var t:i
        let s_57_4: i128 = fn_state.t;
        // D s_57_5: call X_set(s_57_4, s_57_0, s_57_3)
        let s_57_5: () = X_set(state, tracer, s_57_4, s_57_0, s_57_3);
        // N s_57_6: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #64s : i64
        let s_58_0: i64 = 64;
        // C s_58_1: const #() : ()
        let s_58_1: () = ();
        // S s_58_2: call PhysicalCountInt(s_58_1)
        let s_58_2: u64 = PhysicalCountInt(state, tracer, s_58_1);
        // S s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 64u16);
        // C s_58_4: const #14584u : u32
        let s_58_4: u32 = 14584;
        // D s_58_5: read-reg s_58_4:u64
        let s_58_5: u64 = {
            let value = state.read_register::<u64>(s_58_4 as isize);
            tracer.read_register(s_58_4 as isize, value);
            value
        };
        // D s_58_6: cast zx s_58_5 -> bv
        let s_58_6: Bits = Bits::new(s_58_5 as u128, 64u16);
        // D s_58_7: sub s_58_3 s_58_6
        let s_58_7: Bits = ((s_58_3) - (s_58_6));
        // D s_58_8: cast reint s_58_7 -> u64
        let s_58_8: u64 = (s_58_7.value() as u64);
        // D s_58_9: cast zx s_58_8 -> bv
        let s_58_9: Bits = Bits::new(s_58_8 as u128, 64u16);
        // D s_58_10: read-var t:i
        let s_58_10: i128 = fn_state.t;
        // D s_58_11: call X_set(s_58_10, s_58_0, s_58_9)
        let s_58_11: () = X_set(state, tracer, s_58_10, s_58_0, s_58_9);
        // N s_58_12: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #102552u : u32
        let s_59_0: u32 = 102552;
        // D s_59_1: read-reg s_59_0:struct
        let s_59_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call _get_HCR_EL2_Type_E2H(s_59_1)
        let s_59_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_59_1);
        // C s_59_3: const #102552u : u32
        let s_59_3: u32 = 102552;
        // D s_59_4: read-reg s_59_3:struct
        let s_59_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_3 as isize);
            tracer.read_register(s_59_3 as isize, value);
            value
        };
        // D s_59_5: call _get_HCR_EL2_Type_TGE(s_59_4)
        let s_59_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_59_4);
        // D s_59_6: cast zx s_59_2 -> bv
        let s_59_6: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_7: cast zx s_59_5 -> bv
        let s_59_7: Bits = Bits::new(s_59_5 as u128, 1u16);
        // D s_59_8: cast reint s_59_6 -> u128
        let s_59_8: u128 = (s_59_6.value() as u128);
        // D s_59_9: size-of s_59_6
        let s_59_9: u16 = s_59_6.length();
        // D s_59_10: cast reint s_59_7 -> u128
        let s_59_10: u128 = (s_59_7.value() as u128);
        // D s_59_11: size-of s_59_7
        let s_59_11: u16 = s_59_7.length();
        // D s_59_12: lsl s_59_8 s_59_11
        let s_59_12: u128 = s_59_8 << s_59_11;
        // D s_59_13: or s_59_12 s_59_10
        let s_59_13: u128 = ((s_59_12) | (s_59_10));
        // D s_59_14: add s_59_9 s_59_11
        let s_59_14: u16 = (s_59_9 + s_59_11);
        // D s_59_15: create-bits s_59_13 s_59_14
        let s_59_15: Bits = Bits::new(s_59_13, s_59_14);
        // D s_59_16: cast reint s_59_15 -> u8
        let s_59_16: u8 = (s_59_15.value() as u8);
        // D s_59_17: cast zx s_59_16 -> bv
        let s_59_17: Bits = Bits::new(s_59_16 as u128, 2u16);
        // C s_59_18: const #3u : u8
        let s_59_18: u8 = 3;
        // C s_59_19: cast zx s_59_18 -> bv
        let s_59_19: Bits = Bits::new(s_59_18 as u128, 2u16);
        // D s_59_20: cmp-ne s_59_17 s_59_19
        let s_59_20: bool = ((s_59_17) != (s_59_19));
        // D s_59_21: write-var gs#57998 <= s_59_20
        fn_state.gs_57998 = s_59_20;
        // N s_59_22: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_60_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #1u : u8
        let s_60_2: bool = true;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#57997 <= s_60_4
        fn_state.gs_57997 = s_60_4;
        // N s_60_6: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var __SCR_EL3_ECVEn:u8
        let s_61_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 1u16);
        // C s_61_2: const #1u : u8
        let s_61_2: bool = true;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 1u16);
        // D s_61_4: cmp-eq s_61_1 s_61_3
        let s_61_4: bool = ((s_61_1) == (s_61_3));
        // D s_61_5: write-var gs#57996 <= s_61_4
        fn_state.gs_57996 = s_61_4;
        // N s_61_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call EL2Enabled(s_62_0)
        let s_62_1: bool = EL2Enabled(state, tracer, s_62_0);
        // D s_62_2: write-var gs#57995 <= s_62_1
        fn_state.gs_57995 = s_62_1;
        // N s_62_3: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #24u : u8
        let s_63_0: u8 = 24;
        // C s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 8u16);
        // C s_63_2: cast zx s_63_1 -> i
        let s_63_2: i128 = (s_63_1.value() as i128);
        // C s_63_3: cast reint s_63_2 -> i64
        let s_63_3: i64 = (s_63_2 as i64);
        // C s_63_4: cast zx s_63_3 -> i
        let s_63_4: i128 = (i128::try_from(s_63_3).unwrap());
        // C s_63_5: const #432u : u32
        let s_63_5: u32 = 432;
        // D s_63_6: read-reg s_63_5:u8
        let s_63_6: u8 = {
            let value = state.read_register::<u8>(s_63_5 as isize);
            tracer.read_register(s_63_5 as isize, value);
            value
        };
        // D s_63_7: call AArch64_SystemAccessTrap(s_63_6, s_63_4)
        let s_63_7: () = AArch64_SystemAccessTrap(state, tracer, s_63_6, s_63_4);
        // N s_63_8: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __CNTHCTL_EL2_EL0PCTEN:u8
        let s_64_0: bool = fn_state.u__CNTHCTL_EL2_EL0PCTEN;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #0u : u8
        let s_64_2: bool = false;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#57994 <= s_64_4
        fn_state.gs_57994 = s_64_4;
        // N s_64_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #102552u : u32
        let s_65_0: u32 = 102552;
        // D s_65_1: read-reg s_65_0:struct
        let s_65_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // D s_65_2: call _get_HCR_EL2_Type_E2H(s_65_1)
        let s_65_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_65_1);
        // C s_65_3: const #102552u : u32
        let s_65_3: u32 = 102552;
        // D s_65_4: read-reg s_65_3:struct
        let s_65_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_65_3 as isize);
            tracer.read_register(s_65_3 as isize, value);
            value
        };
        // D s_65_5: call _get_HCR_EL2_Type_TGE(s_65_4)
        let s_65_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_65_4);
        // D s_65_6: cast zx s_65_2 -> bv
        let s_65_6: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_7: cast zx s_65_5 -> bv
        let s_65_7: Bits = Bits::new(s_65_5 as u128, 1u16);
        // D s_65_8: cast reint s_65_6 -> u128
        let s_65_8: u128 = (s_65_6.value() as u128);
        // D s_65_9: size-of s_65_6
        let s_65_9: u16 = s_65_6.length();
        // D s_65_10: cast reint s_65_7 -> u128
        let s_65_10: u128 = (s_65_7.value() as u128);
        // D s_65_11: size-of s_65_7
        let s_65_11: u16 = s_65_7.length();
        // D s_65_12: lsl s_65_8 s_65_11
        let s_65_12: u128 = s_65_8 << s_65_11;
        // D s_65_13: or s_65_12 s_65_10
        let s_65_13: u128 = ((s_65_12) | (s_65_10));
        // D s_65_14: add s_65_9 s_65_11
        let s_65_14: u16 = (s_65_9 + s_65_11);
        // D s_65_15: create-bits s_65_13 s_65_14
        let s_65_15: Bits = Bits::new(s_65_13, s_65_14);
        // D s_65_16: cast reint s_65_15 -> u8
        let s_65_16: u8 = (s_65_15.value() as u8);
        // D s_65_17: cast zx s_65_16 -> bv
        let s_65_17: Bits = Bits::new(s_65_16 as u128, 2u16);
        // C s_65_18: const #3u : u8
        let s_65_18: u8 = 3;
        // C s_65_19: cast zx s_65_18 -> bv
        let s_65_19: Bits = Bits::new(s_65_18 as u128, 2u16);
        // D s_65_20: cmp-eq s_65_17 s_65_19
        let s_65_20: bool = ((s_65_17) == (s_65_19));
        // D s_65_21: write-var gs#57993 <= s_65_20
        fn_state.gs_57993 = s_65_20;
        // N s_65_22: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #24u : u8
        let s_66_0: u8 = 24;
        // C s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 8u16);
        // C s_66_2: cast zx s_66_1 -> i
        let s_66_2: i128 = (s_66_1.value() as i128);
        // C s_66_3: cast reint s_66_2 -> i64
        let s_66_3: i64 = (s_66_2 as i64);
        // C s_66_4: cast zx s_66_3 -> i
        let s_66_4: i128 = (i128::try_from(s_66_3).unwrap());
        // C s_66_5: const #432u : u32
        let s_66_5: u32 = 432;
        // D s_66_6: read-reg s_66_5:u8
        let s_66_6: u8 = {
            let value = state.read_register::<u8>(s_66_5 as isize);
            tracer.read_register(s_66_5 as isize, value);
            value
        };
        // D s_66_7: call AArch64_SystemAccessTrap(s_66_6, s_66_4)
        let s_66_7: () = AArch64_SystemAccessTrap(state, tracer, s_66_6, s_66_4);
        // N s_66_8: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __CNTHCTL_EL2_EL1PCTEN:u8
        let s_67_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCTEN;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #0u : u8
        let s_67_2: bool = false;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#57992 <= s_67_4
        fn_state.gs_57992 = s_67_4;
        // N s_67_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #102552u : u32
        let s_68_0: u32 = 102552;
        // D s_68_1: read-reg s_68_0:struct
        let s_68_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: call _get_HCR_EL2_Type_E2H(s_68_1)
        let s_68_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_68_1);
        // C s_68_3: const #102552u : u32
        let s_68_3: u32 = 102552;
        // D s_68_4: read-reg s_68_3:struct
        let s_68_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_3 as isize);
            tracer.read_register(s_68_3 as isize, value);
            value
        };
        // D s_68_5: call _get_HCR_EL2_Type_TGE(s_68_4)
        let s_68_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_68_4);
        // D s_68_6: cast zx s_68_2 -> bv
        let s_68_6: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_7: cast zx s_68_5 -> bv
        let s_68_7: Bits = Bits::new(s_68_5 as u128, 1u16);
        // D s_68_8: cast reint s_68_6 -> u128
        let s_68_8: u128 = (s_68_6.value() as u128);
        // D s_68_9: size-of s_68_6
        let s_68_9: u16 = s_68_6.length();
        // D s_68_10: cast reint s_68_7 -> u128
        let s_68_10: u128 = (s_68_7.value() as u128);
        // D s_68_11: size-of s_68_7
        let s_68_11: u16 = s_68_7.length();
        // D s_68_12: lsl s_68_8 s_68_11
        let s_68_12: u128 = s_68_8 << s_68_11;
        // D s_68_13: or s_68_12 s_68_10
        let s_68_13: u128 = ((s_68_12) | (s_68_10));
        // D s_68_14: add s_68_9 s_68_11
        let s_68_14: u16 = (s_68_9 + s_68_11);
        // D s_68_15: create-bits s_68_13 s_68_14
        let s_68_15: Bits = Bits::new(s_68_13, s_68_14);
        // D s_68_16: cast reint s_68_15 -> u8
        let s_68_16: u8 = (s_68_15.value() as u8);
        // D s_68_17: cast zx s_68_16 -> bv
        let s_68_17: Bits = Bits::new(s_68_16 as u128, 2u16);
        // C s_68_18: const #2u : u8
        let s_68_18: u8 = 2;
        // C s_68_19: cast zx s_68_18 -> bv
        let s_68_19: Bits = Bits::new(s_68_18 as u128, 2u16);
        // D s_68_20: cmp-eq s_68_17 s_68_19
        let s_68_20: bool = ((s_68_17) == (s_68_19));
        // D s_68_21: write-var gs#57991 <= s_68_20
        fn_state.gs_57991 = s_68_20;
        // N s_68_22: jump b40
        return block_40(state, tracer, fn_state);
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
        // D s_70_0: read-var __CNTHCTL_EL2_EL1PCTEN:u8
        let s_70_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCTEN;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #0u : u8
        let s_70_2: bool = false;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#57990 <= s_70_4
        fn_state.gs_57990 = s_70_4;
        // N s_70_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __HCR_EL2_E2H:u8
        let s_71_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #0u : u8
        let s_71_2: bool = false;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#57989 <= s_71_4
        fn_state.gs_57989 = s_71_4;
        // N s_71_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call EL2Enabled(s_72_0)
        let s_72_1: bool = EL2Enabled(state, tracer, s_72_0);
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
        // D s_73_1: write-var gs#58001 <= s_73_0
        fn_state.gs_58001 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#58001:u8
        let s_74_0: bool = fn_state.gs_58001;
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
        // C s_75_5: const #440u : u32
        let s_75_5: u32 = 440;
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
        // C s_76_5: const #432u : u32
        let s_76_5: u32 = 432;
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
        // D s_77_0: read-var __HCR_EL2_TGE:u8
        let s_77_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 1u16);
        // C s_77_2: const #1u : u8
        let s_77_2: bool = true;
        // C s_77_3: cast zx s_77_2 -> bv
        let s_77_3: Bits = Bits::new(s_77_2 as u128, 1u16);
        // D s_77_4: cmp-eq s_77_1 s_77_3
        let s_77_4: bool = ((s_77_1) == (s_77_3));
        // D s_77_5: write-var gs#58001 <= s_77_4
        fn_state.gs_58001 = s_77_4;
        // N s_77_6: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var __CNTKCTL_EL1_EL0PCTEN:u8
        let s_78_0: bool = fn_state.u__CNTKCTL_EL1_EL0PCTEN;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #0u : u8
        let s_78_2: bool = false;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#57988 <= s_78_4
        fn_state.gs_57988 = s_78_4;
        // N s_78_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #102552u : u32
        let s_79_0: u32 = 102552;
        // D s_79_1: read-reg s_79_0:struct
        let s_79_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // D s_79_2: call _get_HCR_EL2_Type_E2H(s_79_1)
        let s_79_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_79_1);
        // C s_79_3: const #102552u : u32
        let s_79_3: u32 = 102552;
        // D s_79_4: read-reg s_79_3:struct
        let s_79_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_79_3 as isize);
            tracer.read_register(s_79_3 as isize, value);
            value
        };
        // D s_79_5: call _get_HCR_EL2_Type_TGE(s_79_4)
        let s_79_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_79_4);
        // D s_79_6: cast zx s_79_2 -> bv
        let s_79_6: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_7: cast zx s_79_5 -> bv
        let s_79_7: Bits = Bits::new(s_79_5 as u128, 1u16);
        // D s_79_8: cast reint s_79_6 -> u128
        let s_79_8: u128 = (s_79_6.value() as u128);
        // D s_79_9: size-of s_79_6
        let s_79_9: u16 = s_79_6.length();
        // D s_79_10: cast reint s_79_7 -> u128
        let s_79_10: u128 = (s_79_7.value() as u128);
        // D s_79_11: size-of s_79_7
        let s_79_11: u16 = s_79_7.length();
        // D s_79_12: lsl s_79_8 s_79_11
        let s_79_12: u128 = s_79_8 << s_79_11;
        // D s_79_13: or s_79_12 s_79_10
        let s_79_13: u128 = ((s_79_12) | (s_79_10));
        // D s_79_14: add s_79_9 s_79_11
        let s_79_14: u16 = (s_79_9 + s_79_11);
        // D s_79_15: create-bits s_79_13 s_79_14
        let s_79_15: Bits = Bits::new(s_79_13, s_79_14);
        // D s_79_16: cast reint s_79_15 -> u8
        let s_79_16: u8 = (s_79_15.value() as u8);
        // D s_79_17: cast zx s_79_16 -> bv
        let s_79_17: Bits = Bits::new(s_79_16 as u128, 2u16);
        // C s_79_18: const #3u : u8
        let s_79_18: u8 = 3;
        // C s_79_19: cast zx s_79_18 -> bv
        let s_79_19: Bits = Bits::new(s_79_18 as u128, 2u16);
        // D s_79_20: cmp-eq s_79_17 s_79_19
        let s_79_20: bool = ((s_79_17) == (s_79_19));
        // D s_79_21: write-var gs#57987 <= s_79_20
        fn_state.gs_57987 = s_79_20;
        // N s_79_22: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #10s : i64
        let s_80_0: i64 = 10;
        // D s_80_1: write-var ga#56428 <= s_80_0
        fn_state.ga_56428 = s_80_0;
        // N s_80_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #102552u : u32
        let s_81_0: u32 = 102552;
        // D s_81_1: read-reg s_81_0:struct
        let s_81_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call _get_HCR_EL2_Type_E2H(s_81_1)
        let s_81_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_81_1);
        // D s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // C s_81_4: const #1u : u8
        let s_81_4: bool = true;
        // C s_81_5: cast zx s_81_4 -> bv
        let s_81_5: Bits = Bits::new(s_81_4 as u128, 1u16);
        // D s_81_6: cmp-eq s_81_3 s_81_5
        let s_81_6: bool = ((s_81_3) == (s_81_5));
        // D s_81_7: write-var gs#57975 <= s_81_6
        fn_state.gs_57975 = s_81_6;
        // N s_81_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
