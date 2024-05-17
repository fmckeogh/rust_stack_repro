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
use EL2Enabled::*;
use AArch32_TakeHypTrapException::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_CNTKCTL_Type_PL0PCTEN::*;
use u_get_CNTHCTL_EL2_Type_EL0PCTEN::*;
use HCR_read::*;
use IsFeatureImplemented::*;
use CNTHCTL_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_CNTHCTL_Type_PL1PCTEN::*;
use u_get_HCR_Type_TGE::*;
use u_get_SCR_EL3_Type_ECVEn::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_CNTKCTL_EL1_Type_EL0PCTEN::*;
use CNTKCTL_read__1::*;
use PhysicalCountInt::*;
use u_get_HCR_EL2_Type_TGE::*;
use Split::*;
use u_get_CNTHCTL_EL2_Type_ECV::*;
use common::*;
pub fn CNTPCT_SysRegRead64_7dc4fea5bbad7fe9<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_642428: ProductTypebc91b195b0b2a883,
        u__HCR_EL2_E2H: bool,
        gs_123150: bool,
        gs_123166: bool,
        gs_123152: bool,
        gs_642442: ProductTypebc91b195b0b2a883,
        gs_123126: bool,
        gs_123151: bool,
        ga_204549: i64,
        gs_123109: bool,
        gs_123143: bool,
        gs_123167: bool,
        gs_642506: ProductTypebc91b195b0b2a883,
        gs_123148: bool,
        gs_123121: bool,
        u__SCR_EL3_ECVEn: bool,
        u__HCR_TGE: bool,
        gs_123169: bool,
        u__CNTKCTL_PL0PCTEN: bool,
        gs_123153: bool,
        gs_123155: bool,
        gs_642499: ProductTypebc91b195b0b2a883,
        gs_123123: bool,
        gs_123156: bool,
        gs_123142: bool,
        gs_123145: bool,
        gs_642449: ProductTypebc91b195b0b2a883,
        u__CNTHCTL_EL2_ECV: bool,
        gs_123170: bool,
        u__PSTATE_EL: u8,
        gs_123168: bool,
        gs_123127: bool,
        u__CNTKCTL_EL1_EL0PCTEN: bool,
        gs_123140: bool,
        gs_123138: bool,
        gs_123144: bool,
        u__HCR_EL2_TGE: bool,
        gs_123149: bool,
        gs_123124: bool,
        gs_123137: bool,
        u__CNTHCTL_EL2_EL0PCTEN: bool,
        gs_123125: bool,
        gs_123141: bool,
        u__CNTHCTL_PL1PCTEN: bool,
        ga_204548: u64,
        gs_123154: bool,
        gs_123139: bool,
        gs_123146: bool,
        u__CNTHCTL_EL2_EL1PCTEN: bool,
        gs_123128: bool,
        gs_642422: ProductTypebc91b195b0b2a883,
        gs_123122: bool,
        gs_123165: bool,
        gs_123147: bool,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRm,
        t,
        t2,
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
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call CNTKCTL_read__1(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = CNTKCTL_read__1(state, tracer, s_0_11);
        // S s_0_13: call _get_CNTKCTL_Type_PL0PCTEN(s_0_12)
        let s_0_13: bool = u_get_CNTKCTL_Type_PL0PCTEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTKCTL_PL0PCTEN <= s_0_13
        fn_state.u__CNTKCTL_PL0PCTEN = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HCR_Type_TGE(s_0_16)
        let s_0_17: bool = u_get_HCR_Type_TGE(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_TGE <= s_0_17
        fn_state.u__HCR_TGE = s_0_17;
        // C s_0_19: const #102552u : u32
        let s_0_19: u32 = 102552;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCR_EL2_Type_E2H(s_0_20)
        let s_0_21: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_EL2_E2H <= s_0_21
        fn_state.u__HCR_EL2_E2H = s_0_21;
        // C s_0_23: const #12808u : u32
        let s_0_23: u32 = 12808;
        // D s_0_24: read-reg s_0_23:u64
        let s_0_24: u64 = {
            let value = state.read_register::<u64>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: write-var ga#204548 <= s_0_24
        fn_state.ga_204548 = s_0_24;
        // C s_0_26: const #54u : u32
        let s_0_26: u32 = 54;
        // S s_0_27: call IsFeatureImplemented(s_0_26)
        let s_0_27: bool = IsFeatureImplemented(state, tracer, s_0_26);
        // N s_0_28: branch s_0_27 b142 b1
        if s_0_27 {
            return block_142(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#123109 <= s_1_0
        fn_state.gs_123109 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#123109:u8
        let s_2_0: bool = fn_state.gs_123109;
        // N s_2_1: branch s_2_0 b141 b3
        if s_2_0 {
            return block_141(state, tracer, fn_state);
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
        // D s_3_1: write-var ga#204549 <= s_3_0
        fn_state.ga_204549 = s_3_0;
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
        // D s_4_1: read-var ga#204548:u64
        let s_4_1: u64 = fn_state.ga_204548;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 64u16);
        // D s_4_3: read-var ga#204549:i64
        let s_4_3: i64 = fn_state.ga_204549;
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
        // C s_4_12: const #() : ()
        let s_4_12: () = ();
        // S s_4_13: call CNTHCTL_read(s_4_12)
        let s_4_13: ProductType700c18a878c5601b = CNTHCTL_read(state, tracer, s_4_12);
        // S s_4_14: call _get_CNTHCTL_Type_PL1PCTEN(s_4_13)
        let s_4_14: bool = u_get_CNTHCTL_Type_PL1PCTEN(state, tracer, s_4_13);
        // D s_4_15: write-var __CNTHCTL_PL1PCTEN <= s_4_14
        fn_state.u__CNTHCTL_PL1PCTEN = s_4_14;
        // C s_4_16: const #90704u : u32
        let s_4_16: u32 = 90704;
        // D s_4_17: read-reg s_4_16:struct
        let s_4_17: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_16 as isize);
            tracer.read_register(s_4_16 as isize, value);
            value
        };
        // D s_4_18: call _get_SCR_EL3_Type_ECVEn(s_4_17)
        let s_4_18: bool = u_get_SCR_EL3_Type_ECVEn(state, tracer, s_4_17);
        // D s_4_19: write-var __SCR_EL3_ECVEn <= s_4_18
        fn_state.u__SCR_EL3_ECVEn = s_4_18;
        // C s_4_20: const #12808u : u32
        let s_4_20: u32 = 12808;
        // D s_4_21: read-reg s_4_20:struct
        let s_4_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_4_20 as isize);
            tracer.read_register(s_4_20 as isize, value);
            value
        };
        // D s_4_22: call _get_CNTHCTL_EL2_Type_ECV(s_4_21)
        let s_4_22: bool = u_get_CNTHCTL_EL2_Type_ECV(state, tracer, s_4_21);
        // D s_4_23: write-var __CNTHCTL_EL2_ECV <= s_4_22
        fn_state.u__CNTHCTL_EL2_ECV = s_4_22;
        // D s_4_24: read-var __PSTATE_EL:u8
        let s_4_24: u8 = fn_state.u__PSTATE_EL;
        // D s_4_25: cast zx s_4_24 -> bv
        let s_4_25: Bits = Bits::new(s_4_24 as u128, 2u16);
        // C s_4_26: const #448u : u32
        let s_4_26: u32 = 448;
        // D s_4_27: read-reg s_4_26:u8
        let s_4_27: u8 = {
            let value = state.read_register::<u8>(s_4_26 as isize);
            tracer.read_register(s_4_26 as isize, value);
            value
        };
        // D s_4_28: cast zx s_4_27 -> bv
        let s_4_28: Bits = Bits::new(s_4_27 as u128, 2u16);
        // D s_4_29: cmp-eq s_4_25 s_4_28
        let s_4_29: bool = ((s_4_25) == (s_4_28));
        // N s_4_30: branch s_4_29 b42 b5
        if s_4_29 {
            return block_42(state, tracer, fn_state);
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
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call PhysicalCountInt(s_9_0)
        let s_9_1: u64 = PhysicalCountInt(state, tracer, s_9_0);
        // C s_9_2: const #32s : i64
        let s_9_2: i64 = 32;
        // S s_9_3: cast zx s_9_1 -> bv
        let s_9_3: Bits = Bits::new(s_9_1 as u128, 64u16);
        // C s_9_4: cast zx s_9_2 -> i
        let s_9_4: i128 = (i128::try_from(s_9_2).unwrap());
        // S s_9_5: call Split(s_9_3, s_9_4)
        let s_9_5: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_9_3, s_9_4);
        // D s_9_6: write-var gs#642422 <= s_9_5
        fn_state.gs_642422 = s_9_5;
        // D s_9_7: read-var gs#642422.0:struct
        let s_9_7: Bits = fn_state.gs_642422._0;
        // D s_9_8: cast reint s_9_7 -> u32
        let s_9_8: u32 = (s_9_7.value() as u32);
        // D s_9_9: read-var gs#642422.1:struct
        let s_9_9: Bits = fn_state.gs_642422._1;
        // D s_9_10: cast reint s_9_9 -> u32
        let s_9_10: u32 = (s_9_9.value() as u32);
        // D s_9_11: read-var t2:i
        let s_9_11: i128 = fn_state.t2;
        // D s_9_12: call R_set(s_9_11, s_9_8)
        let s_9_12: () = R_set(state, tracer, s_9_11, s_9_8);
        // D s_9_13: read-var t:i
        let s_9_13: i128 = fn_state.t;
        // D s_9_14: call R_set(s_9_13, s_9_10)
        let s_9_14: () = R_set(state, tracer, s_9_13, s_9_10);
        // N s_9_15: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call PhysicalCountInt(s_10_0)
        let s_10_1: u64 = PhysicalCountInt(state, tracer, s_10_0);
        // C s_10_2: const #32s : i64
        let s_10_2: i64 = 32;
        // S s_10_3: cast zx s_10_1 -> bv
        let s_10_3: Bits = Bits::new(s_10_1 as u128, 64u16);
        // C s_10_4: cast zx s_10_2 -> i
        let s_10_4: i128 = (i128::try_from(s_10_2).unwrap());
        // S s_10_5: call Split(s_10_3, s_10_4)
        let s_10_5: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_10_3, s_10_4);
        // D s_10_6: write-var gs#642428 <= s_10_5
        fn_state.gs_642428 = s_10_5;
        // D s_10_7: read-var gs#642428.0:struct
        let s_10_7: Bits = fn_state.gs_642428._0;
        // D s_10_8: cast reint s_10_7 -> u32
        let s_10_8: u32 = (s_10_7.value() as u32);
        // D s_10_9: read-var gs#642428.1:struct
        let s_10_9: Bits = fn_state.gs_642428._1;
        // D s_10_10: cast reint s_10_9 -> u32
        let s_10_10: u32 = (s_10_9.value() as u32);
        // D s_10_11: read-var t2:i
        let s_10_11: i128 = fn_state.t2;
        // D s_10_12: call R_set(s_10_11, s_10_8)
        let s_10_12: () = R_set(state, tracer, s_10_11, s_10_8);
        // D s_10_13: read-var t:i
        let s_10_13: i128 = fn_state.t;
        // D s_10_14: call R_set(s_10_13, s_10_10)
        let s_10_14: () = R_set(state, tracer, s_10_13, s_10_10);
        // N s_10_15: return
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
        // N s_11_2: branch s_11_1 b41 b12
        if s_11_1 {
            return block_41(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#123121 <= s_12_0
        fn_state.gs_123121 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#123121:u8
        let s_13_0: bool = fn_state.gs_123121;
        // N s_13_1: branch s_13_0 b40 b14
        if s_13_0 {
            return block_40(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#123122 <= s_14_0
        fn_state.gs_123122 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#123122:u8
        let s_15_0: bool = fn_state.gs_123122;
        // N s_15_1: branch s_15_0 b39 b16
        if s_15_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call EL2Enabled(s_16_0)
        let s_16_1: bool = EL2Enabled(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b38 b17
        if s_16_1 {
            return block_38(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#123123 <= s_17_0
        fn_state.gs_123123 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#123123:u8
        let s_18_0: bool = fn_state.gs_123123;
        // N s_18_1: branch s_18_0 b37 b19
        if s_18_0 {
            return block_37(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#123124 <= s_19_0
        fn_state.gs_123124 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#123124:u8
        let s_20_0: bool = fn_state.gs_123124;
        // N s_20_1: branch s_20_0 b36 b21
        if s_20_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #145u : u32
        let s_21_0: u32 = 145;
        // S s_21_1: call IsFeatureImplemented(s_21_0)
        let s_21_1: bool = IsFeatureImplemented(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b35 b22
        if s_21_1 {
            return block_35(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#123125 <= s_22_0
        fn_state.gs_123125 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#123125:u8
        let s_23_0: bool = fn_state.gs_123125;
        // N s_23_1: branch s_23_0 b34 b24
        if s_23_0 {
            return block_34(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#123126 <= s_24_0
        fn_state.gs_123126 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#123126:u8
        let s_25_0: bool = fn_state.gs_123126;
        // N s_25_1: branch s_25_0 b33 b26
        if s_25_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#123127 <= s_26_0
        fn_state.gs_123127 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#123127:u8
        let s_27_0: bool = fn_state.gs_123127;
        // N s_27_1: branch s_27_0 b32 b28
        if s_27_0 {
            return block_32(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#123128 <= s_28_0
        fn_state.gs_123128 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#123128:u8
        let s_29_0: bool = fn_state.gs_123128;
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
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call PhysicalCountInt(s_30_0)
        let s_30_1: u64 = PhysicalCountInt(state, tracer, s_30_0);
        // C s_30_2: const #32s : i64
        let s_30_2: i64 = 32;
        // S s_30_3: cast zx s_30_1 -> bv
        let s_30_3: Bits = Bits::new(s_30_1 as u128, 64u16);
        // C s_30_4: cast zx s_30_2 -> i
        let s_30_4: i128 = (i128::try_from(s_30_2).unwrap());
        // S s_30_5: call Split(s_30_3, s_30_4)
        let s_30_5: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_30_3, s_30_4);
        // D s_30_6: write-var gs#642442 <= s_30_5
        fn_state.gs_642442 = s_30_5;
        // D s_30_7: read-var gs#642442.0:struct
        let s_30_7: Bits = fn_state.gs_642442._0;
        // D s_30_8: cast reint s_30_7 -> u32
        let s_30_8: u32 = (s_30_7.value() as u32);
        // D s_30_9: read-var gs#642442.1:struct
        let s_30_9: Bits = fn_state.gs_642442._1;
        // D s_30_10: cast reint s_30_9 -> u32
        let s_30_10: u32 = (s_30_9.value() as u32);
        // D s_30_11: read-var t2:i
        let s_30_11: i128 = fn_state.t2;
        // D s_30_12: call R_set(s_30_11, s_30_8)
        let s_30_12: () = R_set(state, tracer, s_30_11, s_30_8);
        // D s_30_13: read-var t:i
        let s_30_13: i128 = fn_state.t;
        // D s_30_14: call R_set(s_30_13, s_30_10)
        let s_30_14: () = R_set(state, tracer, s_30_13, s_30_10);
        // N s_30_15: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call PhysicalCountInt(s_31_0)
        let s_31_1: u64 = PhysicalCountInt(state, tracer, s_31_0);
        // S s_31_2: cast zx s_31_1 -> bv
        let s_31_2: Bits = Bits::new(s_31_1 as u128, 64u16);
        // C s_31_3: const #14584u : u32
        let s_31_3: u32 = 14584;
        // D s_31_4: read-reg s_31_3:u64
        let s_31_4: u64 = {
            let value = state.read_register::<u64>(s_31_3 as isize);
            tracer.read_register(s_31_3 as isize, value);
            value
        };
        // D s_31_5: cast zx s_31_4 -> bv
        let s_31_5: Bits = Bits::new(s_31_4 as u128, 64u16);
        // D s_31_6: sub s_31_2 s_31_5
        let s_31_6: Bits = ((s_31_2) - (s_31_5));
        // D s_31_7: cast reint s_31_6 -> u64
        let s_31_7: u64 = (s_31_6.value() as u64);
        // C s_31_8: const #32s : i64
        let s_31_8: i64 = 32;
        // D s_31_9: cast zx s_31_7 -> bv
        let s_31_9: Bits = Bits::new(s_31_7 as u128, 64u16);
        // C s_31_10: cast zx s_31_8 -> i
        let s_31_10: i128 = (i128::try_from(s_31_8).unwrap());
        // D s_31_11: call Split(s_31_9, s_31_10)
        let s_31_11: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_31_9, s_31_10);
        // D s_31_12: write-var gs#642449 <= s_31_11
        fn_state.gs_642449 = s_31_11;
        // D s_31_13: read-var gs#642449.0:struct
        let s_31_13: Bits = fn_state.gs_642449._0;
        // D s_31_14: cast reint s_31_13 -> u32
        let s_31_14: u32 = (s_31_13.value() as u32);
        // D s_31_15: read-var gs#642449.1:struct
        let s_31_15: Bits = fn_state.gs_642449._1;
        // D s_31_16: cast reint s_31_15 -> u32
        let s_31_16: u32 = (s_31_15.value() as u32);
        // D s_31_17: read-var t2:i
        let s_31_17: i128 = fn_state.t2;
        // D s_31_18: call R_set(s_31_17, s_31_14)
        let s_31_18: () = R_set(state, tracer, s_31_17, s_31_14);
        // D s_31_19: read-var t:i
        let s_31_19: i128 = fn_state.t;
        // D s_31_20: call R_set(s_31_19, s_31_16)
        let s_31_20: () = R_set(state, tracer, s_31_19, s_31_16);
        // N s_31_21: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_32_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#123128 <= s_32_4
        fn_state.gs_123128 = s_32_4;
        // N s_32_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __SCR_EL3_ECVEn:u8
        let s_33_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #1u : u8
        let s_33_2: bool = true;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#123127 <= s_33_4
        fn_state.gs_123127 = s_33_4;
        // N s_33_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #432u : u32
        let s_34_0: u32 = 432;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // D s_34_2: call ELUsingAArch32(s_34_1)
        let s_34_2: bool = ELUsingAArch32(state, tracer, s_34_1);
        // D s_34_3: not s_34_2
        let s_34_3: bool = !s_34_2;
        // D s_34_4: write-var gs#123126 <= s_34_3
        fn_state.gs_123126 = s_34_3;
        // N s_34_5: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call EL2Enabled(s_35_0)
        let s_35_1: bool = EL2Enabled(state, tracer, s_35_0);
        // D s_35_2: write-var gs#123125 <= s_35_1
        fn_state.gs_123125 = s_35_1;
        // N s_35_3: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #4u : u8
        let s_36_0: u8 = 4;
        // C s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 8u16);
        // C s_36_2: cast zx s_36_1 -> i
        let s_36_2: i128 = (s_36_1.value() as i128);
        // C s_36_3: cast reint s_36_2 -> i64
        let s_36_3: i64 = (s_36_2 as i64);
        // C s_36_4: cast zx s_36_3 -> i
        let s_36_4: i128 = (i128::try_from(s_36_3).unwrap());
        // S s_36_5: call AArch32_TakeHypTrapException(s_36_4)
        let s_36_5: () = AArch32_TakeHypTrapException(state, tracer, s_36_4);
        // N s_36_6: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var __CNTHCTL_PL1PCTEN:u8
        let s_37_0: bool = fn_state.u__CNTHCTL_PL1PCTEN;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #0u : u8
        let s_37_2: bool = false;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#123124 <= s_37_4
        fn_state.gs_123124 = s_37_4;
        // N s_37_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #432u : u32
        let s_38_0: u32 = 432;
        // D s_38_1: read-reg s_38_0:u8
        let s_38_1: u8 = {
            let value = state.read_register::<u8>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // D s_38_2: call ELUsingAArch32(s_38_1)
        let s_38_2: bool = ELUsingAArch32(state, tracer, s_38_1);
        // D s_38_3: write-var gs#123123 <= s_38_2
        fn_state.gs_123123 = s_38_2;
        // N s_38_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #4u : u8
        let s_39_0: u8 = 4;
        // C s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 8u16);
        // C s_39_2: cast zx s_39_1 -> i
        let s_39_2: i128 = (s_39_1.value() as i128);
        // C s_39_3: cast reint s_39_2 -> i64
        let s_39_3: i64 = (s_39_2 as i64);
        // C s_39_4: cast zx s_39_3 -> i
        let s_39_4: i128 = (i128::try_from(s_39_3).unwrap());
        // C s_39_5: const #432u : u32
        let s_39_5: u32 = 432;
        // D s_39_6: read-reg s_39_5:u8
        let s_39_6: u8 = {
            let value = state.read_register::<u8>(s_39_5 as isize);
            tracer.read_register(s_39_5 as isize, value);
            value
        };
        // D s_39_7: call AArch64_AArch32SystemAccessTrap(s_39_6, s_39_4)
        let s_39_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_39_6, s_39_4);
        // N s_39_8: return
        return;
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var __CNTHCTL_EL2_EL1PCTEN:u8
        let s_40_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCTEN;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #0u : u8
        let s_40_2: bool = false;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#123122 <= s_40_4
        fn_state.gs_123122 = s_40_4;
        // N s_40_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #432u : u32
        let s_41_0: u32 = 432;
        // D s_41_1: read-reg s_41_0:u8
        let s_41_1: u8 = {
            let value = state.read_register::<u8>(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: call ELUsingAArch32(s_41_1)
        let s_41_2: bool = ELUsingAArch32(state, tracer, s_41_1);
        // D s_41_3: not s_41_2
        let s_41_3: bool = !s_41_2;
        // D s_41_4: write-var gs#123121 <= s_41_3
        fn_state.gs_123121 = s_41_3;
        // N s_41_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #440u : u32
        let s_42_0: u32 = 440;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: u8 = {
            let value = state.read_register::<u8>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call ELUsingAArch32(s_42_1)
        let s_42_2: bool = ELUsingAArch32(state, tracer, s_42_1);
        // D s_42_3: not s_42_2
        let s_42_3: bool = !s_42_2;
        // N s_42_4: branch s_42_3 b137 b43
        if s_42_3 {
            return block_137(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#123138 <= s_43_0
        fn_state.gs_123138 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#123138:u8
        let s_44_0: bool = fn_state.gs_123138;
        // N s_44_1: branch s_44_0 b136 b45
        if s_44_0 {
            return block_136(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#123139 <= s_45_0
        fn_state.gs_123139 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#123139:u8
        let s_46_0: bool = fn_state.gs_123139;
        // N s_46_1: branch s_46_0 b127 b47
        if s_46_0 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #440u : u32
        let s_47_0: u32 = 440;
        // D s_47_1: read-reg s_47_0:u8
        let s_47_1: u8 = {
            let value = state.read_register::<u8>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call ELUsingAArch32(s_47_1)
        let s_47_2: bool = ELUsingAArch32(state, tracer, s_47_1);
        // N s_47_3: branch s_47_2 b126 b48
        if s_47_2 {
            return block_126(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#123140 <= s_48_0
        fn_state.gs_123140 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#123140:u8
        let s_49_0: bool = fn_state.gs_123140;
        // N s_49_1: branch s_49_0 b109 b50
        if s_49_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #() : ()
        let s_50_0: () = ();
        // S s_50_1: call EL2Enabled(s_50_0)
        let s_50_1: bool = EL2Enabled(state, tracer, s_50_0);
        // N s_50_2: branch s_50_1 b108 b51
        if s_50_1 {
            return block_108(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#123141 <= s_51_0
        fn_state.gs_123141 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#123141:u8
        let s_52_0: bool = fn_state.gs_123141;
        // N s_52_1: branch s_52_0 b107 b53
        if s_52_0 {
            return block_107(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#123142 <= s_53_0
        fn_state.gs_123142 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#123142:u8
        let s_54_0: bool = fn_state.gs_123142;
        // N s_54_1: branch s_54_0 b106 b55
        if s_54_0 {
            return block_106(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#123143 <= s_55_0
        fn_state.gs_123143 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#123143:u8
        let s_56_0: bool = fn_state.gs_123143;
        // N s_56_1: branch s_56_0 b105 b57
        if s_56_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call EL2Enabled(s_57_0)
        let s_57_1: bool = EL2Enabled(state, tracer, s_57_0);
        // N s_57_2: branch s_57_1 b104 b58
        if s_57_1 {
            return block_104(state, tracer, fn_state);
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
        // D s_58_1: write-var gs#123144 <= s_58_0
        fn_state.gs_123144 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#123144:u8
        let s_59_0: bool = fn_state.gs_123144;
        // N s_59_1: branch s_59_0 b103 b60
        if s_59_0 {
            return block_103(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#123145 <= s_60_0
        fn_state.gs_123145 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#123145:u8
        let s_61_0: bool = fn_state.gs_123145;
        // N s_61_1: branch s_61_0 b102 b62
        if s_61_0 {
            return block_102(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#123146 <= s_62_0
        fn_state.gs_123146 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#123146:u8
        let s_63_0: bool = fn_state.gs_123146;
        // N s_63_1: branch s_63_0 b101 b64
        if s_63_0 {
            return block_101(state, tracer, fn_state);
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
        // S s_64_1: call EL2Enabled(s_64_0)
        let s_64_1: bool = EL2Enabled(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b100 b65
        if s_64_1 {
            return block_100(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#123147 <= s_65_0
        fn_state.gs_123147 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#123147:u8
        let s_66_0: bool = fn_state.gs_123147;
        // N s_66_1: branch s_66_0 b99 b67
        if s_66_0 {
            return block_99(state, tracer, fn_state);
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
        // D s_67_1: write-var gs#123148 <= s_67_0
        fn_state.gs_123148 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#123148:u8
        let s_68_0: bool = fn_state.gs_123148;
        // N s_68_1: branch s_68_0 b98 b69
        if s_68_0 {
            return block_98(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#123149 <= s_69_0
        fn_state.gs_123149 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#123149:u8
        let s_70_0: bool = fn_state.gs_123149;
        // N s_70_1: branch s_70_0 b97 b71
        if s_70_0 {
            return block_97(state, tracer, fn_state);
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
        // N s_71_2: branch s_71_1 b96 b72
        if s_71_1 {
            return block_96(state, tracer, fn_state);
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
        // D s_72_1: write-var gs#123150 <= s_72_0
        fn_state.gs_123150 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#123150:u8
        let s_73_0: bool = fn_state.gs_123150;
        // N s_73_1: branch s_73_0 b95 b74
        if s_73_0 {
            return block_95(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#123151 <= s_74_0
        fn_state.gs_123151 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#123151:u8
        let s_75_0: bool = fn_state.gs_123151;
        // N s_75_1: branch s_75_0 b94 b76
        if s_75_0 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #145u : u32
        let s_76_0: u32 = 145;
        // S s_76_1: call IsFeatureImplemented(s_76_0)
        let s_76_1: bool = IsFeatureImplemented(state, tracer, s_76_0);
        // N s_76_2: branch s_76_1 b93 b77
        if s_76_1 {
            return block_93(state, tracer, fn_state);
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
        // D s_77_1: write-var gs#123152 <= s_77_0
        fn_state.gs_123152 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#123152:u8
        let s_78_0: bool = fn_state.gs_123152;
        // N s_78_1: branch s_78_0 b92 b79
        if s_78_0 {
            return block_92(state, tracer, fn_state);
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
        // D s_79_1: write-var gs#123153 <= s_79_0
        fn_state.gs_123153 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#123153:u8
        let s_80_0: bool = fn_state.gs_123153;
        // N s_80_1: branch s_80_0 b91 b81
        if s_80_0 {
            return block_91(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#123154 <= s_81_0
        fn_state.gs_123154 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#123154:u8
        let s_82_0: bool = fn_state.gs_123154;
        // N s_82_1: branch s_82_0 b90 b83
        if s_82_0 {
            return block_90(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#123155 <= s_83_0
        fn_state.gs_123155 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#123155:u8
        let s_84_0: bool = fn_state.gs_123155;
        // N s_84_1: branch s_84_0 b89 b85
        if s_84_0 {
            return block_89(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#123156 <= s_85_0
        fn_state.gs_123156 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#123156:u8
        let s_86_0: bool = fn_state.gs_123156;
        // N s_86_1: branch s_86_0 b88 b87
        if s_86_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #() : ()
        let s_87_0: () = ();
        // S s_87_1: call PhysicalCountInt(s_87_0)
        let s_87_1: u64 = PhysicalCountInt(state, tracer, s_87_0);
        // C s_87_2: const #32s : i64
        let s_87_2: i64 = 32;
        // S s_87_3: cast zx s_87_1 -> bv
        let s_87_3: Bits = Bits::new(s_87_1 as u128, 64u16);
        // C s_87_4: cast zx s_87_2 -> i
        let s_87_4: i128 = (i128::try_from(s_87_2).unwrap());
        // S s_87_5: call Split(s_87_3, s_87_4)
        let s_87_5: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_87_3, s_87_4);
        // D s_87_6: write-var gs#642499 <= s_87_5
        fn_state.gs_642499 = s_87_5;
        // D s_87_7: read-var gs#642499.0:struct
        let s_87_7: Bits = fn_state.gs_642499._0;
        // D s_87_8: cast reint s_87_7 -> u32
        let s_87_8: u32 = (s_87_7.value() as u32);
        // D s_87_9: read-var gs#642499.1:struct
        let s_87_9: Bits = fn_state.gs_642499._1;
        // D s_87_10: cast reint s_87_9 -> u32
        let s_87_10: u32 = (s_87_9.value() as u32);
        // D s_87_11: read-var t2:i
        let s_87_11: i128 = fn_state.t2;
        // D s_87_12: call R_set(s_87_11, s_87_8)
        let s_87_12: () = R_set(state, tracer, s_87_11, s_87_8);
        // D s_87_13: read-var t:i
        let s_87_13: i128 = fn_state.t;
        // D s_87_14: call R_set(s_87_13, s_87_10)
        let s_87_14: () = R_set(state, tracer, s_87_13, s_87_10);
        // N s_87_15: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #() : ()
        let s_88_0: () = ();
        // S s_88_1: call PhysicalCountInt(s_88_0)
        let s_88_1: u64 = PhysicalCountInt(state, tracer, s_88_0);
        // S s_88_2: cast zx s_88_1 -> bv
        let s_88_2: Bits = Bits::new(s_88_1 as u128, 64u16);
        // C s_88_3: const #14584u : u32
        let s_88_3: u32 = 14584;
        // D s_88_4: read-reg s_88_3:u64
        let s_88_4: u64 = {
            let value = state.read_register::<u64>(s_88_3 as isize);
            tracer.read_register(s_88_3 as isize, value);
            value
        };
        // D s_88_5: cast zx s_88_4 -> bv
        let s_88_5: Bits = Bits::new(s_88_4 as u128, 64u16);
        // D s_88_6: sub s_88_2 s_88_5
        let s_88_6: Bits = ((s_88_2) - (s_88_5));
        // D s_88_7: cast reint s_88_6 -> u64
        let s_88_7: u64 = (s_88_6.value() as u64);
        // C s_88_8: const #32s : i64
        let s_88_8: i64 = 32;
        // D s_88_9: cast zx s_88_7 -> bv
        let s_88_9: Bits = Bits::new(s_88_7 as u128, 64u16);
        // C s_88_10: cast zx s_88_8 -> i
        let s_88_10: i128 = (i128::try_from(s_88_8).unwrap());
        // D s_88_11: call Split(s_88_9, s_88_10)
        let s_88_11: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_88_9, s_88_10);
        // D s_88_12: write-var gs#642506 <= s_88_11
        fn_state.gs_642506 = s_88_11;
        // D s_88_13: read-var gs#642506.0:struct
        let s_88_13: Bits = fn_state.gs_642506._0;
        // D s_88_14: cast reint s_88_13 -> u32
        let s_88_14: u32 = (s_88_13.value() as u32);
        // D s_88_15: read-var gs#642506.1:struct
        let s_88_15: Bits = fn_state.gs_642506._1;
        // D s_88_16: cast reint s_88_15 -> u32
        let s_88_16: u32 = (s_88_15.value() as u32);
        // D s_88_17: read-var t2:i
        let s_88_17: i128 = fn_state.t2;
        // D s_88_18: call R_set(s_88_17, s_88_14)
        let s_88_18: () = R_set(state, tracer, s_88_17, s_88_14);
        // D s_88_19: read-var t:i
        let s_88_19: i128 = fn_state.t;
        // D s_88_20: call R_set(s_88_19, s_88_16)
        let s_88_20: () = R_set(state, tracer, s_88_19, s_88_16);
        // N s_88_21: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #102552u : u32
        let s_89_0: u32 = 102552;
        // D s_89_1: read-reg s_89_0:struct
        let s_89_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // D s_89_2: call _get_HCR_EL2_Type_E2H(s_89_1)
        let s_89_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_89_1);
        // C s_89_3: const #102552u : u32
        let s_89_3: u32 = 102552;
        // D s_89_4: read-reg s_89_3:struct
        let s_89_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_89_3 as isize);
            tracer.read_register(s_89_3 as isize, value);
            value
        };
        // D s_89_5: call _get_HCR_EL2_Type_TGE(s_89_4)
        let s_89_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_89_4);
        // D s_89_6: cast zx s_89_2 -> bv
        let s_89_6: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_7: cast zx s_89_5 -> bv
        let s_89_7: Bits = Bits::new(s_89_5 as u128, 1u16);
        // D s_89_8: cast reint s_89_6 -> u128
        let s_89_8: u128 = (s_89_6.value() as u128);
        // D s_89_9: size-of s_89_6
        let s_89_9: u16 = s_89_6.length();
        // D s_89_10: cast reint s_89_7 -> u128
        let s_89_10: u128 = (s_89_7.value() as u128);
        // D s_89_11: size-of s_89_7
        let s_89_11: u16 = s_89_7.length();
        // D s_89_12: lsl s_89_8 s_89_11
        let s_89_12: u128 = s_89_8 << s_89_11;
        // D s_89_13: or s_89_12 s_89_10
        let s_89_13: u128 = ((s_89_12) | (s_89_10));
        // D s_89_14: add s_89_9 s_89_11
        let s_89_14: u16 = (s_89_9 + s_89_11);
        // D s_89_15: create-bits s_89_13 s_89_14
        let s_89_15: Bits = Bits::new(s_89_13, s_89_14);
        // D s_89_16: cast reint s_89_15 -> u8
        let s_89_16: u8 = (s_89_15.value() as u8);
        // D s_89_17: cast zx s_89_16 -> bv
        let s_89_17: Bits = Bits::new(s_89_16 as u128, 2u16);
        // C s_89_18: const #3u : u8
        let s_89_18: u8 = 3;
        // C s_89_19: cast zx s_89_18 -> bv
        let s_89_19: Bits = Bits::new(s_89_18 as u128, 2u16);
        // D s_89_20: cmp-ne s_89_17 s_89_19
        let s_89_20: bool = ((s_89_17) != (s_89_19));
        // D s_89_21: write-var gs#123156 <= s_89_20
        fn_state.gs_123156 = s_89_20;
        // N s_89_22: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_90_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 1u16);
        // C s_90_2: const #1u : u8
        let s_90_2: bool = true;
        // C s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // D s_90_5: write-var gs#123155 <= s_90_4
        fn_state.gs_123155 = s_90_4;
        // N s_90_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var __SCR_EL3_ECVEn:u8
        let s_91_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 1u16);
        // C s_91_2: const #1u : u8
        let s_91_2: bool = true;
        // C s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // D s_91_4: cmp-eq s_91_1 s_91_3
        let s_91_4: bool = ((s_91_1) == (s_91_3));
        // D s_91_5: write-var gs#123154 <= s_91_4
        fn_state.gs_123154 = s_91_4;
        // N s_91_6: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #432u : u32
        let s_92_0: u32 = 432;
        // D s_92_1: read-reg s_92_0:u8
        let s_92_1: u8 = {
            let value = state.read_register::<u8>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // D s_92_2: call ELUsingAArch32(s_92_1)
        let s_92_2: bool = ELUsingAArch32(state, tracer, s_92_1);
        // D s_92_3: not s_92_2
        let s_92_3: bool = !s_92_2;
        // D s_92_4: write-var gs#123153 <= s_92_3
        fn_state.gs_123153 = s_92_3;
        // N s_92_5: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #() : ()
        let s_93_0: () = ();
        // S s_93_1: call EL2Enabled(s_93_0)
        let s_93_1: bool = EL2Enabled(state, tracer, s_93_0);
        // D s_93_2: write-var gs#123152 <= s_93_1
        fn_state.gs_123152 = s_93_1;
        // N s_93_3: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #4u : u8
        let s_94_0: u8 = 4;
        // C s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 8u16);
        // C s_94_2: cast zx s_94_1 -> i
        let s_94_2: i128 = (s_94_1.value() as i128);
        // C s_94_3: cast reint s_94_2 -> i64
        let s_94_3: i64 = (s_94_2 as i64);
        // C s_94_4: cast zx s_94_3 -> i
        let s_94_4: i128 = (i128::try_from(s_94_3).unwrap());
        // S s_94_5: call AArch32_TakeHypTrapException(s_94_4)
        let s_94_5: () = AArch32_TakeHypTrapException(state, tracer, s_94_4);
        // N s_94_6: return
        return;
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var __CNTHCTL_PL1PCTEN:u8
        let s_95_0: bool = fn_state.u__CNTHCTL_PL1PCTEN;
        // D s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 1u16);
        // C s_95_2: const #0u : u8
        let s_95_2: bool = false;
        // C s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // D s_95_4: cmp-eq s_95_1 s_95_3
        let s_95_4: bool = ((s_95_1) == (s_95_3));
        // D s_95_5: write-var gs#123151 <= s_95_4
        fn_state.gs_123151 = s_95_4;
        // N s_95_6: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #432u : u32
        let s_96_0: u32 = 432;
        // D s_96_1: read-reg s_96_0:u8
        let s_96_1: u8 = {
            let value = state.read_register::<u8>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // D s_96_2: call ELUsingAArch32(s_96_1)
        let s_96_2: bool = ELUsingAArch32(state, tracer, s_96_1);
        // D s_96_3: write-var gs#123150 <= s_96_2
        fn_state.gs_123150 = s_96_2;
        // N s_96_4: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #4u : u8
        let s_97_0: u8 = 4;
        // C s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 8u16);
        // C s_97_2: cast zx s_97_1 -> i
        let s_97_2: i128 = (s_97_1.value() as i128);
        // C s_97_3: cast reint s_97_2 -> i64
        let s_97_3: i64 = (s_97_2 as i64);
        // C s_97_4: cast zx s_97_3 -> i
        let s_97_4: i128 = (i128::try_from(s_97_3).unwrap());
        // C s_97_5: const #432u : u32
        let s_97_5: u32 = 432;
        // D s_97_6: read-reg s_97_5:u8
        let s_97_6: u8 = {
            let value = state.read_register::<u8>(s_97_5 as isize);
            tracer.read_register(s_97_5 as isize, value);
            value
        };
        // D s_97_7: call AArch64_AArch32SystemAccessTrap(s_97_6, s_97_4)
        let s_97_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_97_6, s_97_4);
        // N s_97_8: return
        return;
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var __CNTHCTL_EL2_EL0PCTEN:u8
        let s_98_0: bool = fn_state.u__CNTHCTL_EL2_EL0PCTEN;
        // D s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 1u16);
        // C s_98_2: const #0u : u8
        let s_98_2: bool = false;
        // C s_98_3: cast zx s_98_2 -> bv
        let s_98_3: Bits = Bits::new(s_98_2 as u128, 1u16);
        // D s_98_4: cmp-eq s_98_1 s_98_3
        let s_98_4: bool = ((s_98_1) == (s_98_3));
        // D s_98_5: write-var gs#123149 <= s_98_4
        fn_state.gs_123149 = s_98_4;
        // N s_98_6: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #102552u : u32
        let s_99_0: u32 = 102552;
        // D s_99_1: read-reg s_99_0:struct
        let s_99_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // D s_99_2: call _get_HCR_EL2_Type_E2H(s_99_1)
        let s_99_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_99_1);
        // C s_99_3: const #102552u : u32
        let s_99_3: u32 = 102552;
        // D s_99_4: read-reg s_99_3:struct
        let s_99_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_99_3 as isize);
            tracer.read_register(s_99_3 as isize, value);
            value
        };
        // D s_99_5: call _get_HCR_EL2_Type_TGE(s_99_4)
        let s_99_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_99_4);
        // D s_99_6: cast zx s_99_2 -> bv
        let s_99_6: Bits = Bits::new(s_99_2 as u128, 1u16);
        // D s_99_7: cast zx s_99_5 -> bv
        let s_99_7: Bits = Bits::new(s_99_5 as u128, 1u16);
        // D s_99_8: cast reint s_99_6 -> u128
        let s_99_8: u128 = (s_99_6.value() as u128);
        // D s_99_9: size-of s_99_6
        let s_99_9: u16 = s_99_6.length();
        // D s_99_10: cast reint s_99_7 -> u128
        let s_99_10: u128 = (s_99_7.value() as u128);
        // D s_99_11: size-of s_99_7
        let s_99_11: u16 = s_99_7.length();
        // D s_99_12: lsl s_99_8 s_99_11
        let s_99_12: u128 = s_99_8 << s_99_11;
        // D s_99_13: or s_99_12 s_99_10
        let s_99_13: u128 = ((s_99_12) | (s_99_10));
        // D s_99_14: add s_99_9 s_99_11
        let s_99_14: u16 = (s_99_9 + s_99_11);
        // D s_99_15: create-bits s_99_13 s_99_14
        let s_99_15: Bits = Bits::new(s_99_13, s_99_14);
        // D s_99_16: cast reint s_99_15 -> u8
        let s_99_16: u8 = (s_99_15.value() as u8);
        // D s_99_17: cast zx s_99_16 -> bv
        let s_99_17: Bits = Bits::new(s_99_16 as u128, 2u16);
        // C s_99_18: const #3u : u8
        let s_99_18: u8 = 3;
        // C s_99_19: cast zx s_99_18 -> bv
        let s_99_19: Bits = Bits::new(s_99_18 as u128, 2u16);
        // D s_99_20: cmp-eq s_99_17 s_99_19
        let s_99_20: bool = ((s_99_17) == (s_99_19));
        // D s_99_21: write-var gs#123148 <= s_99_20
        fn_state.gs_123148 = s_99_20;
        // N s_99_22: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #432u : u32
        let s_100_0: u32 = 432;
        // D s_100_1: read-reg s_100_0:u8
        let s_100_1: u8 = {
            let value = state.read_register::<u8>(s_100_0 as isize);
            tracer.read_register(s_100_0 as isize, value);
            value
        };
        // D s_100_2: call ELUsingAArch32(s_100_1)
        let s_100_2: bool = ELUsingAArch32(state, tracer, s_100_1);
        // D s_100_3: not s_100_2
        let s_100_3: bool = !s_100_2;
        // D s_100_4: write-var gs#123147 <= s_100_3
        fn_state.gs_123147 = s_100_3;
        // N s_100_5: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #4u : u8
        let s_101_0: u8 = 4;
        // C s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 8u16);
        // C s_101_2: cast zx s_101_1 -> i
        let s_101_2: i128 = (s_101_1.value() as i128);
        // C s_101_3: cast reint s_101_2 -> i64
        let s_101_3: i64 = (s_101_2 as i64);
        // C s_101_4: cast zx s_101_3 -> i
        let s_101_4: i128 = (i128::try_from(s_101_3).unwrap());
        // C s_101_5: const #432u : u32
        let s_101_5: u32 = 432;
        // D s_101_6: read-reg s_101_5:u8
        let s_101_6: u8 = {
            let value = state.read_register::<u8>(s_101_5 as isize);
            tracer.read_register(s_101_5 as isize, value);
            value
        };
        // D s_101_7: call AArch64_AArch32SystemAccessTrap(s_101_6, s_101_4)
        let s_101_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_101_6,
            s_101_4,
        );
        // N s_101_8: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var __CNTHCTL_EL2_EL1PCTEN:u8
        let s_102_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCTEN;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 1u16);
        // C s_102_2: const #0u : u8
        let s_102_2: bool = false;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: write-var gs#123146 <= s_102_4
        fn_state.gs_123146 = s_102_4;
        // N s_102_6: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #102552u : u32
        let s_103_0: u32 = 102552;
        // D s_103_1: read-reg s_103_0:struct
        let s_103_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_103_0 as isize);
            tracer.read_register(s_103_0 as isize, value);
            value
        };
        // D s_103_2: call _get_HCR_EL2_Type_E2H(s_103_1)
        let s_103_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_103_1);
        // C s_103_3: const #102552u : u32
        let s_103_3: u32 = 102552;
        // D s_103_4: read-reg s_103_3:struct
        let s_103_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_103_3 as isize);
            tracer.read_register(s_103_3 as isize, value);
            value
        };
        // D s_103_5: call _get_HCR_EL2_Type_TGE(s_103_4)
        let s_103_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_103_4);
        // D s_103_6: cast zx s_103_2 -> bv
        let s_103_6: Bits = Bits::new(s_103_2 as u128, 1u16);
        // D s_103_7: cast zx s_103_5 -> bv
        let s_103_7: Bits = Bits::new(s_103_5 as u128, 1u16);
        // D s_103_8: cast reint s_103_6 -> u128
        let s_103_8: u128 = (s_103_6.value() as u128);
        // D s_103_9: size-of s_103_6
        let s_103_9: u16 = s_103_6.length();
        // D s_103_10: cast reint s_103_7 -> u128
        let s_103_10: u128 = (s_103_7.value() as u128);
        // D s_103_11: size-of s_103_7
        let s_103_11: u16 = s_103_7.length();
        // D s_103_12: lsl s_103_8 s_103_11
        let s_103_12: u128 = s_103_8 << s_103_11;
        // D s_103_13: or s_103_12 s_103_10
        let s_103_13: u128 = ((s_103_12) | (s_103_10));
        // D s_103_14: add s_103_9 s_103_11
        let s_103_14: u16 = (s_103_9 + s_103_11);
        // D s_103_15: create-bits s_103_13 s_103_14
        let s_103_15: Bits = Bits::new(s_103_13, s_103_14);
        // D s_103_16: cast reint s_103_15 -> u8
        let s_103_16: u8 = (s_103_15.value() as u8);
        // D s_103_17: cast zx s_103_16 -> bv
        let s_103_17: Bits = Bits::new(s_103_16 as u128, 2u16);
        // C s_103_18: const #2u : u8
        let s_103_18: u8 = 2;
        // C s_103_19: cast zx s_103_18 -> bv
        let s_103_19: Bits = Bits::new(s_103_18 as u128, 2u16);
        // D s_103_20: cmp-eq s_103_17 s_103_19
        let s_103_20: bool = ((s_103_17) == (s_103_19));
        // D s_103_21: write-var gs#123145 <= s_103_20
        fn_state.gs_123145 = s_103_20;
        // N s_103_22: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #432u : u32
        let s_104_0: u32 = 432;
        // D s_104_1: read-reg s_104_0:u8
        let s_104_1: u8 = {
            let value = state.read_register::<u8>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // D s_104_2: call ELUsingAArch32(s_104_1)
        let s_104_2: bool = ELUsingAArch32(state, tracer, s_104_1);
        // D s_104_3: not s_104_2
        let s_104_3: bool = !s_104_2;
        // D s_104_4: write-var gs#123144 <= s_104_3
        fn_state.gs_123144 = s_104_3;
        // N s_104_5: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #4u : u8
        let s_105_0: u8 = 4;
        // C s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 8u16);
        // C s_105_2: cast zx s_105_1 -> i
        let s_105_2: i128 = (s_105_1.value() as i128);
        // C s_105_3: cast reint s_105_2 -> i64
        let s_105_3: i64 = (s_105_2 as i64);
        // C s_105_4: cast zx s_105_3 -> i
        let s_105_4: i128 = (i128::try_from(s_105_3).unwrap());
        // C s_105_5: const #432u : u32
        let s_105_5: u32 = 432;
        // D s_105_6: read-reg s_105_5:u8
        let s_105_6: u8 = {
            let value = state.read_register::<u8>(s_105_5 as isize);
            tracer.read_register(s_105_5 as isize, value);
            value
        };
        // D s_105_7: call AArch64_AArch32SystemAccessTrap(s_105_6, s_105_4)
        let s_105_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_105_6,
            s_105_4,
        );
        // N s_105_8: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var __CNTHCTL_EL2_EL1PCTEN:u8
        let s_106_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCTEN;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 1u16);
        // C s_106_2: const #0u : u8
        let s_106_2: bool = false;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: write-var gs#123143 <= s_106_4
        fn_state.gs_123143 = s_106_4;
        // N s_106_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var __HCR_EL2_E2H:u8
        let s_107_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 1u16);
        // C s_107_2: const #0u : u8
        let s_107_2: bool = false;
        // C s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 1u16);
        // D s_107_4: cmp-eq s_107_1 s_107_3
        let s_107_4: bool = ((s_107_1) == (s_107_3));
        // D s_107_5: write-var gs#123142 <= s_107_4
        fn_state.gs_123142 = s_107_4;
        // N s_107_6: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #432u : u32
        let s_108_0: u32 = 432;
        // D s_108_1: read-reg s_108_0:u8
        let s_108_1: u8 = {
            let value = state.read_register::<u8>(s_108_0 as isize);
            tracer.read_register(s_108_0 as isize, value);
            value
        };
        // D s_108_2: call ELUsingAArch32(s_108_1)
        let s_108_2: bool = ELUsingAArch32(state, tracer, s_108_1);
        // D s_108_3: not s_108_2
        let s_108_3: bool = !s_108_2;
        // D s_108_4: write-var gs#123141 <= s_108_3
        fn_state.gs_123141 = s_108_3;
        // N s_108_5: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call EL2Enabled(s_109_0)
        let s_109_1: bool = EL2Enabled(state, tracer, s_109_0);
        // N s_109_2: branch s_109_1 b125 b110
        if s_109_1 {
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
        // D s_110_1: write-var gs#123165 <= s_110_0
        fn_state.gs_123165 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#123165:u8
        let s_111_0: bool = fn_state.gs_123165;
        // N s_111_1: branch s_111_0 b124 b112
        if s_111_0 {
            return block_124(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#123166 <= s_112_0
        fn_state.gs_123166 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#123166:u8
        let s_113_0: bool = fn_state.gs_123166;
        // N s_113_1: branch s_113_0 b123 b114
        if s_113_0 {
            return block_123(state, tracer, fn_state);
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
        // N s_114_2: branch s_114_1 b122 b115
        if s_114_1 {
            return block_122(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#123167 <= s_115_0
        fn_state.gs_123167 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#123167:u8
        let s_116_0: bool = fn_state.gs_123167;
        // N s_116_1: branch s_116_0 b121 b117
        if s_116_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #0u : u8
        let s_117_0: bool = false;
        // D s_117_1: write-var gs#123168 <= s_117_0
        fn_state.gs_123168 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#123168:u8
        let s_118_0: bool = fn_state.gs_123168;
        // N s_118_1: branch s_118_0 b120 b119
        if s_118_0 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_119_0: panic
        panic!("{:?}", ());
        // N s_119_1: return
        return;
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #0u : u8
        let s_120_0: u8 = 0;
        // C s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 8u16);
        // C s_120_2: cast zx s_120_1 -> i
        let s_120_2: i128 = (s_120_1.value() as i128);
        // C s_120_3: cast reint s_120_2 -> i64
        let s_120_3: i64 = (s_120_2 as i64);
        // C s_120_4: cast zx s_120_3 -> i
        let s_120_4: i128 = (i128::try_from(s_120_3).unwrap());
        // S s_120_5: call AArch32_TakeHypTrapException(s_120_4)
        let s_120_5: () = AArch32_TakeHypTrapException(state, tracer, s_120_4);
        // N s_120_6: return
        return;
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var __HCR_TGE:u8
        let s_121_0: bool = fn_state.u__HCR_TGE;
        // D s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 1u16);
        // C s_121_2: const #1u : u8
        let s_121_2: bool = true;
        // C s_121_3: cast zx s_121_2 -> bv
        let s_121_3: Bits = Bits::new(s_121_2 as u128, 1u16);
        // D s_121_4: cmp-eq s_121_1 s_121_3
        let s_121_4: bool = ((s_121_1) == (s_121_3));
        // D s_121_5: write-var gs#123168 <= s_121_4
        fn_state.gs_123168 = s_121_4;
        // N s_121_6: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #432u : u32
        let s_122_0: u32 = 432;
        // D s_122_1: read-reg s_122_0:u8
        let s_122_1: u8 = {
            let value = state.read_register::<u8>(s_122_0 as isize);
            tracer.read_register(s_122_0 as isize, value);
            value
        };
        // D s_122_2: call ELUsingAArch32(s_122_1)
        let s_122_2: bool = ELUsingAArch32(state, tracer, s_122_1);
        // D s_122_3: write-var gs#123167 <= s_122_2
        fn_state.gs_123167 = s_122_2;
        // N s_122_4: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #4u : u8
        let s_123_0: u8 = 4;
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
        // D s_123_7: call AArch64_AArch32SystemAccessTrap(s_123_6, s_123_4)
        let s_123_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_123_6,
            s_123_4,
        );
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
        // D s_124_5: write-var gs#123166 <= s_124_4
        fn_state.gs_123166 = s_124_4;
        // N s_124_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #432u : u32
        let s_125_0: u32 = 432;
        // D s_125_1: read-reg s_125_0:u8
        let s_125_1: u8 = {
            let value = state.read_register::<u8>(s_125_0 as isize);
            tracer.read_register(s_125_0 as isize, value);
            value
        };
        // D s_125_2: call ELUsingAArch32(s_125_1)
        let s_125_2: bool = ELUsingAArch32(state, tracer, s_125_1);
        // D s_125_3: not s_125_2
        let s_125_3: bool = !s_125_2;
        // D s_125_4: write-var gs#123165 <= s_125_3
        fn_state.gs_123165 = s_125_3;
        // N s_125_5: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var __CNTKCTL_PL0PCTEN:u8
        let s_126_0: bool = fn_state.u__CNTKCTL_PL0PCTEN;
        // D s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 1u16);
        // C s_126_2: const #0u : u8
        let s_126_2: bool = false;
        // C s_126_3: cast zx s_126_2 -> bv
        let s_126_3: Bits = Bits::new(s_126_2 as u128, 1u16);
        // D s_126_4: cmp-eq s_126_1 s_126_3
        let s_126_4: bool = ((s_126_1) == (s_126_3));
        // D s_126_5: write-var gs#123140 <= s_126_4
        fn_state.gs_123140 = s_126_4;
        // N s_126_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #() : ()
        let s_127_0: () = ();
        // S s_127_1: call EL2Enabled(s_127_0)
        let s_127_1: bool = EL2Enabled(state, tracer, s_127_0);
        // N s_127_2: branch s_127_1 b135 b128
        if s_127_1 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #0u : u8
        let s_128_0: bool = false;
        // D s_128_1: write-var gs#123169 <= s_128_0
        fn_state.gs_123169 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#123169:u8
        let s_129_0: bool = fn_state.gs_123169;
        // N s_129_1: branch s_129_0 b134 b130
        if s_129_0 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #0u : u8
        let s_130_0: bool = false;
        // D s_130_1: write-var gs#123170 <= s_130_0
        fn_state.gs_123170 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#123170:u8
        let s_131_0: bool = fn_state.gs_123170;
        // N s_131_1: branch s_131_0 b133 b132
        if s_131_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #4u : u8
        let s_132_0: u8 = 4;
        // C s_132_1: cast zx s_132_0 -> bv
        let s_132_1: Bits = Bits::new(s_132_0 as u128, 8u16);
        // C s_132_2: cast zx s_132_1 -> i
        let s_132_2: i128 = (s_132_1.value() as i128);
        // C s_132_3: cast reint s_132_2 -> i64
        let s_132_3: i64 = (s_132_2 as i64);
        // C s_132_4: cast zx s_132_3 -> i
        let s_132_4: i128 = (i128::try_from(s_132_3).unwrap());
        // C s_132_5: const #440u : u32
        let s_132_5: u32 = 440;
        // D s_132_6: read-reg s_132_5:u8
        let s_132_6: u8 = {
            let value = state.read_register::<u8>(s_132_5 as isize);
            tracer.read_register(s_132_5 as isize, value);
            value
        };
        // D s_132_7: call AArch64_AArch32SystemAccessTrap(s_132_6, s_132_4)
        let s_132_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_132_6,
            s_132_4,
        );
        // N s_132_8: return
        return;
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #4u : u8
        let s_133_0: u8 = 4;
        // C s_133_1: cast zx s_133_0 -> bv
        let s_133_1: Bits = Bits::new(s_133_0 as u128, 8u16);
        // C s_133_2: cast zx s_133_1 -> i
        let s_133_2: i128 = (s_133_1.value() as i128);
        // C s_133_3: cast reint s_133_2 -> i64
        let s_133_3: i64 = (s_133_2 as i64);
        // C s_133_4: cast zx s_133_3 -> i
        let s_133_4: i128 = (i128::try_from(s_133_3).unwrap());
        // C s_133_5: const #432u : u32
        let s_133_5: u32 = 432;
        // D s_133_6: read-reg s_133_5:u8
        let s_133_6: u8 = {
            let value = state.read_register::<u8>(s_133_5 as isize);
            tracer.read_register(s_133_5 as isize, value);
            value
        };
        // D s_133_7: call AArch64_AArch32SystemAccessTrap(s_133_6, s_133_4)
        let s_133_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_133_6,
            s_133_4,
        );
        // N s_133_8: return
        return;
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var __HCR_EL2_TGE:u8
        let s_134_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 1u16);
        // C s_134_2: const #1u : u8
        let s_134_2: bool = true;
        // C s_134_3: cast zx s_134_2 -> bv
        let s_134_3: Bits = Bits::new(s_134_2 as u128, 1u16);
        // D s_134_4: cmp-eq s_134_1 s_134_3
        let s_134_4: bool = ((s_134_1) == (s_134_3));
        // D s_134_5: write-var gs#123170 <= s_134_4
        fn_state.gs_123170 = s_134_4;
        // N s_134_6: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #432u : u32
        let s_135_0: u32 = 432;
        // D s_135_1: read-reg s_135_0:u8
        let s_135_1: u8 = {
            let value = state.read_register::<u8>(s_135_0 as isize);
            tracer.read_register(s_135_0 as isize, value);
            value
        };
        // D s_135_2: call ELUsingAArch32(s_135_1)
        let s_135_2: bool = ELUsingAArch32(state, tracer, s_135_1);
        // D s_135_3: not s_135_2
        let s_135_3: bool = !s_135_2;
        // D s_135_4: write-var gs#123169 <= s_135_3
        fn_state.gs_123169 = s_135_3;
        // N s_135_5: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var __CNTKCTL_EL1_EL0PCTEN:u8
        let s_136_0: bool = fn_state.u__CNTKCTL_EL1_EL0PCTEN;
        // D s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 1u16);
        // C s_136_2: const #0u : u8
        let s_136_2: bool = false;
        // C s_136_3: cast zx s_136_2 -> bv
        let s_136_3: Bits = Bits::new(s_136_2 as u128, 1u16);
        // D s_136_4: cmp-eq s_136_1 s_136_3
        let s_136_4: bool = ((s_136_1) == (s_136_3));
        // D s_136_5: write-var gs#123139 <= s_136_4
        fn_state.gs_123139 = s_136_4;
        // N s_136_6: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #() : ()
        let s_137_0: () = ();
        // S s_137_1: call EL2Enabled(s_137_0)
        let s_137_1: bool = EL2Enabled(state, tracer, s_137_0);
        // N s_137_2: branch s_137_1 b140 b138
        if s_137_1 {
            return block_140(state, tracer, fn_state);
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
        // D s_138_1: write-var gs#123137 <= s_138_0
        fn_state.gs_123137 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#123137:u8
        let s_139_0: bool = fn_state.gs_123137;
        // D s_139_1: not s_139_0
        let s_139_1: bool = !s_139_0;
        // D s_139_2: write-var gs#123138 <= s_139_1
        fn_state.gs_123138 = s_139_1;
        // N s_139_3: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #102552u : u32
        let s_140_0: u32 = 102552;
        // D s_140_1: read-reg s_140_0:struct
        let s_140_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_140_0 as isize);
            tracer.read_register(s_140_0 as isize, value);
            value
        };
        // D s_140_2: call _get_HCR_EL2_Type_E2H(s_140_1)
        let s_140_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_140_1);
        // C s_140_3: const #102552u : u32
        let s_140_3: u32 = 102552;
        // D s_140_4: read-reg s_140_3:struct
        let s_140_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_140_3 as isize);
            tracer.read_register(s_140_3 as isize, value);
            value
        };
        // D s_140_5: call _get_HCR_EL2_Type_TGE(s_140_4)
        let s_140_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_140_4);
        // D s_140_6: cast zx s_140_2 -> bv
        let s_140_6: Bits = Bits::new(s_140_2 as u128, 1u16);
        // D s_140_7: cast zx s_140_5 -> bv
        let s_140_7: Bits = Bits::new(s_140_5 as u128, 1u16);
        // D s_140_8: cast reint s_140_6 -> u128
        let s_140_8: u128 = (s_140_6.value() as u128);
        // D s_140_9: size-of s_140_6
        let s_140_9: u16 = s_140_6.length();
        // D s_140_10: cast reint s_140_7 -> u128
        let s_140_10: u128 = (s_140_7.value() as u128);
        // D s_140_11: size-of s_140_7
        let s_140_11: u16 = s_140_7.length();
        // D s_140_12: lsl s_140_8 s_140_11
        let s_140_12: u128 = s_140_8 << s_140_11;
        // D s_140_13: or s_140_12 s_140_10
        let s_140_13: u128 = ((s_140_12) | (s_140_10));
        // D s_140_14: add s_140_9 s_140_11
        let s_140_14: u16 = (s_140_9 + s_140_11);
        // D s_140_15: create-bits s_140_13 s_140_14
        let s_140_15: Bits = Bits::new(s_140_13, s_140_14);
        // D s_140_16: cast reint s_140_15 -> u8
        let s_140_16: u8 = (s_140_15.value() as u8);
        // D s_140_17: cast zx s_140_16 -> bv
        let s_140_17: Bits = Bits::new(s_140_16 as u128, 2u16);
        // C s_140_18: const #3u : u8
        let s_140_18: u8 = 3;
        // C s_140_19: cast zx s_140_18 -> bv
        let s_140_19: Bits = Bits::new(s_140_18 as u128, 2u16);
        // D s_140_20: cmp-eq s_140_17 s_140_19
        let s_140_20: bool = ((s_140_17) == (s_140_19));
        // D s_140_21: write-var gs#123137 <= s_140_20
        fn_state.gs_123137 = s_140_20;
        // N s_140_22: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #10s : i64
        let s_141_0: i64 = 10;
        // D s_141_1: write-var ga#204549 <= s_141_0
        fn_state.ga_204549 = s_141_0;
        // N s_141_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #102552u : u32
        let s_142_0: u32 = 102552;
        // D s_142_1: read-reg s_142_0:struct
        let s_142_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_142_0 as isize);
            tracer.read_register(s_142_0 as isize, value);
            value
        };
        // D s_142_2: call _get_HCR_EL2_Type_E2H(s_142_1)
        let s_142_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_142_1);
        // D s_142_3: cast zx s_142_2 -> bv
        let s_142_3: Bits = Bits::new(s_142_2 as u128, 1u16);
        // C s_142_4: const #1u : u8
        let s_142_4: bool = true;
        // C s_142_5: cast zx s_142_4 -> bv
        let s_142_5: Bits = Bits::new(s_142_4 as u128, 1u16);
        // D s_142_6: cmp-eq s_142_3 s_142_5
        let s_142_6: bool = ((s_142_3) == (s_142_5));
        // D s_142_7: write-var gs#123109 <= s_142_6
        fn_state.gs_123109 = s_142_6;
        // N s_142_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
