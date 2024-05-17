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
use AArch32_TakeHypTrapException::*;
use u_get_HCR_EL2_Type_E2H::*;
use HCR_read::*;
use u_get_CNTKCTL_EL1_Type_EL0VCTEN::*;
use u_get_CNTKCTL_Type_PL0VCTEN::*;
use u_get_CNTHCTL_EL2_Type_EL0VCTEN::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HCR_Type_TGE::*;
use u_get_CNTHCTL_EL2_Type_EL1TVCT::*;
use R_set::*;
use ELUsingAArch32::*;
use CNTKCTL_read__1::*;
use CNTVOFF_read::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use PhysicalCountInt::*;
use Split::*;
use common::*;
pub fn CNTVCT_SysRegRead64_6e373436952b9ca9<T: Tracer>(
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
        gs_642580: ProductTypebc91b195b0b2a883,
        gs_642542: ProductTypebc91b195b0b2a883,
        gs_123230: bool,
        gs_123209: bool,
        gs_123208: bool,
        gs_123228: bool,
        gs_642573: ProductTypebc91b195b0b2a883,
        gs_123205: bool,
        u__HCR_TGE: bool,
        gs_123207: bool,
        gs_123184: bool,
        gs_123199: bool,
        gs_123227: bool,
        gs_642558: ProductTypebc91b195b0b2a883,
        gs_642624: ProductTypebc91b195b0b2a883,
        gs_123183: bool,
        gs_123211: bool,
        u__PSTATE_EL: u8,
        u__CNTHCTL_EL2_EL0VCTEN: bool,
        u__HCR_EL2_TGE: bool,
        gs_123210: bool,
        gs_123225: bool,
        gs_123229: bool,
        gs_123185: bool,
        gs_123203: bool,
        gs_123226: bool,
        gs_642631: ProductTypebc91b195b0b2a883,
        gs_123186: bool,
        gs_123204: bool,
        u__CNTKCTL_PL0VCTEN: bool,
        gs_123200: bool,
        gs_642617: ProductTypebc91b195b0b2a883,
        gs_123202: bool,
        gs_123212: bool,
        gs_123201: bool,
        u__CNTHCTL_EL2_EL1TVCT: bool,
        gs_642549: ProductTypebc91b195b0b2a883,
        u__CNTKCTL_EL1_EL0VCTEN: bool,
        gs_642566: ProductTypebc91b195b0b2a883,
        gs_123206: bool,
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
        // D s_0_5: call _get_CNTKCTL_EL1_Type_EL0VCTEN(s_0_4)
        let s_0_5: bool = u_get_CNTKCTL_EL1_Type_EL0VCTEN(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTKCTL_EL1_EL0VCTEN <= s_0_5
        fn_state.u__CNTKCTL_EL1_EL0VCTEN = s_0_5;
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
        // S s_0_13: call _get_CNTKCTL_Type_PL0VCTEN(s_0_12)
        let s_0_13: bool = u_get_CNTKCTL_Type_PL0VCTEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTKCTL_PL0VCTEN <= s_0_13
        fn_state.u__CNTKCTL_PL0VCTEN = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HCR_Type_TGE(s_0_16)
        let s_0_17: bool = u_get_HCR_Type_TGE(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_TGE <= s_0_17
        fn_state.u__HCR_TGE = s_0_17;
        // C s_0_19: const #12808u : u32
        let s_0_19: u32 = 12808;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_CNTHCTL_EL2_Type_EL0VCTEN(s_0_20)
        let s_0_21: bool = u_get_CNTHCTL_EL2_Type_EL0VCTEN(state, tracer, s_0_20);
        // D s_0_22: write-var __CNTHCTL_EL2_EL0VCTEN <= s_0_21
        fn_state.u__CNTHCTL_EL2_EL0VCTEN = s_0_21;
        // C s_0_23: const #12808u : u32
        let s_0_23: u32 = 12808;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_CNTHCTL_EL2_Type_EL1TVCT(s_0_24)
        let s_0_25: bool = u_get_CNTHCTL_EL2_Type_EL1TVCT(state, tracer, s_0_24);
        // D s_0_26: write-var __CNTHCTL_EL2_EL1TVCT <= s_0_25
        fn_state.u__CNTHCTL_EL2_EL1TVCT = s_0_25;
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
        // N s_0_33: branch s_0_32 b28 b1
        if s_0_32 {
            return block_28(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b9 b2
        if s_1_5 {
            return block_9(state, tracer, fn_state);
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
        // C s_5_0: const #432u : u32
        let s_5_0: u32 = 432;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
        // D s_5_3: cmp-lt s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) < (s_5_2));
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
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
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call PhysicalCountInt(s_6_0)
        let s_6_1: u64 = PhysicalCountInt(state, tracer, s_6_0);
        // C s_6_2: const #32s : i64
        let s_6_2: i64 = 32;
        // S s_6_3: cast zx s_6_1 -> bv
        let s_6_3: Bits = Bits::new(s_6_1 as u128, 64u16);
        // C s_6_4: cast zx s_6_2 -> i
        let s_6_4: i128 = (i128::try_from(s_6_2).unwrap());
        // S s_6_5: call Split(s_6_3, s_6_4)
        let s_6_5: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_6_3, s_6_4);
        // D s_6_6: write-var gs#642542 <= s_6_5
        fn_state.gs_642542 = s_6_5;
        // D s_6_7: read-var gs#642542.0:struct
        let s_6_7: Bits = fn_state.gs_642542._0;
        // D s_6_8: cast reint s_6_7 -> u32
        let s_6_8: u32 = (s_6_7.value() as u32);
        // D s_6_9: read-var gs#642542.1:struct
        let s_6_9: Bits = fn_state.gs_642542._1;
        // D s_6_10: cast reint s_6_9 -> u32
        let s_6_10: u32 = (s_6_9.value() as u32);
        // D s_6_11: read-var t2:i
        let s_6_11: i128 = fn_state.t2;
        // D s_6_12: call R_set(s_6_11, s_6_8)
        let s_6_12: () = R_set(state, tracer, s_6_11, s_6_8);
        // D s_6_13: read-var t:i
        let s_6_13: i128 = fn_state.t;
        // D s_6_14: call R_set(s_6_13, s_6_10)
        let s_6_14: () = R_set(state, tracer, s_6_13, s_6_10);
        // N s_6_15: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call PhysicalCountInt(s_7_0)
        let s_7_1: u64 = PhysicalCountInt(state, tracer, s_7_0);
        // C s_7_2: const #() : ()
        let s_7_2: () = ();
        // S s_7_3: call CNTVOFF_read(s_7_2)
        let s_7_3: u64 = CNTVOFF_read(state, tracer, s_7_2);
        // S s_7_4: cast zx s_7_1 -> bv
        let s_7_4: Bits = Bits::new(s_7_1 as u128, 64u16);
        // S s_7_5: cast zx s_7_3 -> bv
        let s_7_5: Bits = Bits::new(s_7_3 as u128, 64u16);
        // S s_7_6: sub s_7_4 s_7_5
        let s_7_6: Bits = ((s_7_4) - (s_7_5));
        // S s_7_7: cast reint s_7_6 -> u64
        let s_7_7: u64 = (s_7_6.value() as u64);
        // C s_7_8: const #32s : i64
        let s_7_8: i64 = 32;
        // S s_7_9: cast zx s_7_7 -> bv
        let s_7_9: Bits = Bits::new(s_7_7 as u128, 64u16);
        // C s_7_10: cast zx s_7_8 -> i
        let s_7_10: i128 = (i128::try_from(s_7_8).unwrap());
        // S s_7_11: call Split(s_7_9, s_7_10)
        let s_7_11: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_7_9, s_7_10);
        // D s_7_12: write-var gs#642549 <= s_7_11
        fn_state.gs_642549 = s_7_11;
        // D s_7_13: read-var gs#642549.0:struct
        let s_7_13: Bits = fn_state.gs_642549._0;
        // D s_7_14: cast reint s_7_13 -> u32
        let s_7_14: u32 = (s_7_13.value() as u32);
        // D s_7_15: read-var gs#642549.1:struct
        let s_7_15: Bits = fn_state.gs_642549._1;
        // D s_7_16: cast reint s_7_15 -> u32
        let s_7_16: u32 = (s_7_15.value() as u32);
        // D s_7_17: read-var t2:i
        let s_7_17: i128 = fn_state.t2;
        // D s_7_18: call R_set(s_7_17, s_7_14)
        let s_7_18: () = R_set(state, tracer, s_7_17, s_7_14);
        // D s_7_19: read-var t:i
        let s_7_19: i128 = fn_state.t;
        // D s_7_20: call R_set(s_7_19, s_7_16)
        let s_7_20: () = R_set(state, tracer, s_7_19, s_7_16);
        // N s_7_21: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call PhysicalCountInt(s_8_0)
        let s_8_1: u64 = PhysicalCountInt(state, tracer, s_8_0);
        // C s_8_2: const #() : ()
        let s_8_2: () = ();
        // S s_8_3: call CNTVOFF_read(s_8_2)
        let s_8_3: u64 = CNTVOFF_read(state, tracer, s_8_2);
        // S s_8_4: cast zx s_8_1 -> bv
        let s_8_4: Bits = Bits::new(s_8_1 as u128, 64u16);
        // S s_8_5: cast zx s_8_3 -> bv
        let s_8_5: Bits = Bits::new(s_8_3 as u128, 64u16);
        // S s_8_6: sub s_8_4 s_8_5
        let s_8_6: Bits = ((s_8_4) - (s_8_5));
        // S s_8_7: cast reint s_8_6 -> u64
        let s_8_7: u64 = (s_8_6.value() as u64);
        // C s_8_8: const #32s : i64
        let s_8_8: i64 = 32;
        // S s_8_9: cast zx s_8_7 -> bv
        let s_8_9: Bits = Bits::new(s_8_7 as u128, 64u16);
        // C s_8_10: cast zx s_8_8 -> i
        let s_8_10: i128 = (i128::try_from(s_8_8).unwrap());
        // S s_8_11: call Split(s_8_9, s_8_10)
        let s_8_11: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_8_9, s_8_10);
        // D s_8_12: write-var gs#642558 <= s_8_11
        fn_state.gs_642558 = s_8_11;
        // D s_8_13: read-var gs#642558.0:struct
        let s_8_13: Bits = fn_state.gs_642558._0;
        // D s_8_14: cast reint s_8_13 -> u32
        let s_8_14: u32 = (s_8_13.value() as u32);
        // D s_8_15: read-var gs#642558.1:struct
        let s_8_15: Bits = fn_state.gs_642558._1;
        // D s_8_16: cast reint s_8_15 -> u32
        let s_8_16: u32 = (s_8_15.value() as u32);
        // D s_8_17: read-var t2:i
        let s_8_17: i128 = fn_state.t2;
        // D s_8_18: call R_set(s_8_17, s_8_14)
        let s_8_18: () = R_set(state, tracer, s_8_17, s_8_14);
        // D s_8_19: read-var t:i
        let s_8_19: i128 = fn_state.t;
        // D s_8_20: call R_set(s_8_19, s_8_16)
        let s_8_20: () = R_set(state, tracer, s_8_19, s_8_16);
        // N s_8_21: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call EL2Enabled(s_9_0)
        let s_9_1: bool = EL2Enabled(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b27 b10
        if s_9_1 {
            return block_27(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#123183 <= s_10_0
        fn_state.gs_123183 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#123183:u8
        let s_11_0: bool = fn_state.gs_123183;
        // N s_11_1: branch s_11_0 b26 b12
        if s_11_0 {
            return block_26(state, tracer, fn_state);
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
        // D s_12_1: write-var gs#123184 <= s_12_0
        fn_state.gs_123184 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#123184:u8
        let s_13_0: bool = fn_state.gs_123184;
        // N s_13_1: branch s_13_0 b25 b14
        if s_13_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #432u : u32
        let s_14_0: u32 = 432;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: u8 = {
            let value = state.read_register::<u8>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // C s_14_2: const #2u : u8
        let s_14_2: u8 = 2;
        // D s_14_3: cmp-lt s_14_1 s_14_2
        let s_14_3: bool = ((s_14_1) < (s_14_2));
        // N s_14_4: branch s_14_3 b24 b15
        if s_14_3 {
            return block_24(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#123185 <= s_15_0
        fn_state.gs_123185 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#123185:u8
        let s_16_0: bool = fn_state.gs_123185;
        // N s_16_1: branch s_16_0 b23 b17
        if s_16_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #432u : u32
        let s_17_0: u32 = 432;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // D s_17_3: cmp-lt s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) < (s_17_2));
        // N s_17_4: branch s_17_3 b22 b18
        if s_17_3 {
            return block_22(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#123186 <= s_18_0
        fn_state.gs_123186 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#123186:u8
        let s_19_0: bool = fn_state.gs_123186;
        // N s_19_1: branch s_19_0 b21 b20
        if s_19_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call PhysicalCountInt(s_20_0)
        let s_20_1: u64 = PhysicalCountInt(state, tracer, s_20_0);
        // C s_20_2: const #32s : i64
        let s_20_2: i64 = 32;
        // S s_20_3: cast zx s_20_1 -> bv
        let s_20_3: Bits = Bits::new(s_20_1 as u128, 64u16);
        // C s_20_4: cast zx s_20_2 -> i
        let s_20_4: i128 = (i128::try_from(s_20_2).unwrap());
        // S s_20_5: call Split(s_20_3, s_20_4)
        let s_20_5: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_20_3, s_20_4);
        // D s_20_6: write-var gs#642566 <= s_20_5
        fn_state.gs_642566 = s_20_5;
        // D s_20_7: read-var gs#642566.0:struct
        let s_20_7: Bits = fn_state.gs_642566._0;
        // D s_20_8: cast reint s_20_7 -> u32
        let s_20_8: u32 = (s_20_7.value() as u32);
        // D s_20_9: read-var gs#642566.1:struct
        let s_20_9: Bits = fn_state.gs_642566._1;
        // D s_20_10: cast reint s_20_9 -> u32
        let s_20_10: u32 = (s_20_9.value() as u32);
        // D s_20_11: read-var t2:i
        let s_20_11: i128 = fn_state.t2;
        // D s_20_12: call R_set(s_20_11, s_20_8)
        let s_20_12: () = R_set(state, tracer, s_20_11, s_20_8);
        // D s_20_13: read-var t:i
        let s_20_13: i128 = fn_state.t;
        // D s_20_14: call R_set(s_20_13, s_20_10)
        let s_20_14: () = R_set(state, tracer, s_20_13, s_20_10);
        // N s_20_15: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call PhysicalCountInt(s_21_0)
        let s_21_1: u64 = PhysicalCountInt(state, tracer, s_21_0);
        // C s_21_2: const #() : ()
        let s_21_2: () = ();
        // S s_21_3: call CNTVOFF_read(s_21_2)
        let s_21_3: u64 = CNTVOFF_read(state, tracer, s_21_2);
        // S s_21_4: cast zx s_21_1 -> bv
        let s_21_4: Bits = Bits::new(s_21_1 as u128, 64u16);
        // S s_21_5: cast zx s_21_3 -> bv
        let s_21_5: Bits = Bits::new(s_21_3 as u128, 64u16);
        // S s_21_6: sub s_21_4 s_21_5
        let s_21_6: Bits = ((s_21_4) - (s_21_5));
        // S s_21_7: cast reint s_21_6 -> u64
        let s_21_7: u64 = (s_21_6.value() as u64);
        // C s_21_8: const #32s : i64
        let s_21_8: i64 = 32;
        // S s_21_9: cast zx s_21_7 -> bv
        let s_21_9: Bits = Bits::new(s_21_7 as u128, 64u16);
        // C s_21_10: cast zx s_21_8 -> i
        let s_21_10: i128 = (i128::try_from(s_21_8).unwrap());
        // S s_21_11: call Split(s_21_9, s_21_10)
        let s_21_11: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_21_9, s_21_10);
        // D s_21_12: write-var gs#642573 <= s_21_11
        fn_state.gs_642573 = s_21_11;
        // D s_21_13: read-var gs#642573.0:struct
        let s_21_13: Bits = fn_state.gs_642573._0;
        // D s_21_14: cast reint s_21_13 -> u32
        let s_21_14: u32 = (s_21_13.value() as u32);
        // D s_21_15: read-var gs#642573.1:struct
        let s_21_15: Bits = fn_state.gs_642573._1;
        // D s_21_16: cast reint s_21_15 -> u32
        let s_21_16: u32 = (s_21_15.value() as u32);
        // D s_21_17: read-var t2:i
        let s_21_17: i128 = fn_state.t2;
        // D s_21_18: call R_set(s_21_17, s_21_14)
        let s_21_18: () = R_set(state, tracer, s_21_17, s_21_14);
        // D s_21_19: read-var t:i
        let s_21_19: i128 = fn_state.t;
        // D s_21_20: call R_set(s_21_19, s_21_16)
        let s_21_20: () = R_set(state, tracer, s_21_19, s_21_16);
        // N s_21_21: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #432u : u32
        let s_22_0: u32 = 432;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call ELUsingAArch32(s_22_1)
        let s_22_2: bool = ELUsingAArch32(state, tracer, s_22_1);
        // D s_22_3: write-var gs#123186 <= s_22_2
        fn_state.gs_123186 = s_22_2;
        // N s_22_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call PhysicalCountInt(s_23_0)
        let s_23_1: u64 = PhysicalCountInt(state, tracer, s_23_0);
        // S s_23_2: cast zx s_23_1 -> bv
        let s_23_2: Bits = Bits::new(s_23_1 as u128, 64u16);
        // C s_23_3: const #22400u : u32
        let s_23_3: u32 = 22400;
        // D s_23_4: read-reg s_23_3:u64
        let s_23_4: u64 = {
            let value = state.read_register::<u64>(s_23_3 as isize);
            tracer.read_register(s_23_3 as isize, value);
            value
        };
        // D s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 64u16);
        // D s_23_6: sub s_23_2 s_23_5
        let s_23_6: Bits = ((s_23_2) - (s_23_5));
        // D s_23_7: cast reint s_23_6 -> u64
        let s_23_7: u64 = (s_23_6.value() as u64);
        // C s_23_8: const #32s : i64
        let s_23_8: i64 = 32;
        // D s_23_9: cast zx s_23_7 -> bv
        let s_23_9: Bits = Bits::new(s_23_7 as u128, 64u16);
        // C s_23_10: cast zx s_23_8 -> i
        let s_23_10: i128 = (i128::try_from(s_23_8).unwrap());
        // D s_23_11: call Split(s_23_9, s_23_10)
        let s_23_11: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_23_9, s_23_10);
        // D s_23_12: write-var gs#642580 <= s_23_11
        fn_state.gs_642580 = s_23_11;
        // D s_23_13: read-var gs#642580.0:struct
        let s_23_13: Bits = fn_state.gs_642580._0;
        // D s_23_14: cast reint s_23_13 -> u32
        let s_23_14: u32 = (s_23_13.value() as u32);
        // D s_23_15: read-var gs#642580.1:struct
        let s_23_15: Bits = fn_state.gs_642580._1;
        // D s_23_16: cast reint s_23_15 -> u32
        let s_23_16: u32 = (s_23_15.value() as u32);
        // D s_23_17: read-var t2:i
        let s_23_17: i128 = fn_state.t2;
        // D s_23_18: call R_set(s_23_17, s_23_14)
        let s_23_18: () = R_set(state, tracer, s_23_17, s_23_14);
        // D s_23_19: read-var t:i
        let s_23_19: i128 = fn_state.t;
        // D s_23_20: call R_set(s_23_19, s_23_16)
        let s_23_20: () = R_set(state, tracer, s_23_19, s_23_16);
        // N s_23_21: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #432u : u32
        let s_24_0: u32 = 432;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // D s_24_2: call ELUsingAArch32(s_24_1)
        let s_24_2: bool = ELUsingAArch32(state, tracer, s_24_1);
        // D s_24_3: not s_24_2
        let s_24_3: bool = !s_24_2;
        // D s_24_4: write-var gs#123185 <= s_24_3
        fn_state.gs_123185 = s_24_3;
        // N s_24_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #4u : u8
        let s_25_0: u8 = 4;
        // C s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 8u16);
        // C s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (s_25_1.value() as i128);
        // C s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #432u : u32
        let s_25_5: u32 = 432;
        // D s_25_6: read-reg s_25_5:u8
        let s_25_6: u8 = {
            let value = state.read_register::<u8>(s_25_5 as isize);
            tracer.read_register(s_25_5 as isize, value);
            value
        };
        // D s_25_7: call AArch64_AArch32SystemAccessTrap(s_25_6, s_25_4)
        let s_25_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_25_6, s_25_4);
        // N s_25_8: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __CNTHCTL_EL2_EL1TVCT:u8
        let s_26_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVCT;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#123184 <= s_26_4
        fn_state.gs_123184 = s_26_4;
        // N s_26_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #432u : u32
        let s_27_0: u32 = 432;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call ELUsingAArch32(s_27_1)
        let s_27_2: bool = ELUsingAArch32(state, tracer, s_27_1);
        // D s_27_3: not s_27_2
        let s_27_3: bool = !s_27_2;
        // D s_27_4: write-var gs#123183 <= s_27_3
        fn_state.gs_123183 = s_27_3;
        // N s_27_5: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #440u : u32
        let s_28_0: u32 = 440;
        // D s_28_1: read-reg s_28_0:u8
        let s_28_1: u8 = {
            let value = state.read_register::<u8>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call ELUsingAArch32(s_28_1)
        let s_28_2: bool = ELUsingAArch32(state, tracer, s_28_1);
        // D s_28_3: not s_28_2
        let s_28_3: bool = !s_28_2;
        // N s_28_4: branch s_28_3 b103 b29
        if s_28_3 {
            return block_103(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#123200 <= s_29_0
        fn_state.gs_123200 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#123200:u8
        let s_30_0: bool = fn_state.gs_123200;
        // N s_30_1: branch s_30_0 b102 b31
        if s_30_0 {
            return block_102(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#123201 <= s_31_0
        fn_state.gs_123201 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#123201:u8
        let s_32_0: bool = fn_state.gs_123201;
        // N s_32_1: branch s_32_0 b93 b33
        if s_32_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #440u : u32
        let s_33_0: u32 = 440;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call ELUsingAArch32(s_33_1)
        let s_33_2: bool = ELUsingAArch32(state, tracer, s_33_1);
        // N s_33_3: branch s_33_2 b92 b34
        if s_33_2 {
            return block_92(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#123202 <= s_34_0
        fn_state.gs_123202 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#123202:u8
        let s_35_0: bool = fn_state.gs_123202;
        // N s_35_1: branch s_35_0 b75 b36
        if s_35_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call EL2Enabled(s_36_0)
        let s_36_1: bool = EL2Enabled(state, tracer, s_36_0);
        // N s_36_2: branch s_36_1 b74 b37
        if s_36_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#123203 <= s_37_0
        fn_state.gs_123203 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#123203:u8
        let s_38_0: bool = fn_state.gs_123203;
        // N s_38_1: branch s_38_0 b73 b39
        if s_38_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#123204 <= s_39_0
        fn_state.gs_123204 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#123204:u8
        let s_40_0: bool = fn_state.gs_123204;
        // N s_40_1: branch s_40_0 b72 b41
        if s_40_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#123205 <= s_41_0
        fn_state.gs_123205 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#123205:u8
        let s_42_0: bool = fn_state.gs_123205;
        // N s_42_1: branch s_42_0 b71 b43
        if s_42_0 {
            return block_71(state, tracer, fn_state);
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
        // N s_43_2: branch s_43_1 b70 b44
        if s_43_1 {
            return block_70(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#123206 <= s_44_0
        fn_state.gs_123206 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#123206:u8
        let s_45_0: bool = fn_state.gs_123206;
        // N s_45_1: branch s_45_0 b69 b46
        if s_45_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#123207 <= s_46_0
        fn_state.gs_123207 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#123207:u8
        let s_47_0: bool = fn_state.gs_123207;
        // N s_47_1: branch s_47_0 b68 b48
        if s_47_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#123208 <= s_48_0
        fn_state.gs_123208 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#123208:u8
        let s_49_0: bool = fn_state.gs_123208;
        // N s_49_1: branch s_49_0 b67 b50
        if s_49_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #432u : u32
        let s_50_0: u32 = 432;
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
        // N s_50_4: branch s_50_3 b66 b51
        if s_50_3 {
            return block_66(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#123209 <= s_51_0
        fn_state.gs_123209 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#123209:u8
        let s_52_0: bool = fn_state.gs_123209;
        // N s_52_1: branch s_52_0 b62 b53
        if s_52_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#123211 <= s_53_0
        fn_state.gs_123211 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#123211:u8
        let s_54_0: bool = fn_state.gs_123211;
        // N s_54_1: branch s_54_0 b61 b55
        if s_54_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #432u : u32
        let s_55_0: u32 = 432;
        // D s_55_1: read-reg s_55_0:u8
        let s_55_1: u8 = {
            let value = state.read_register::<u8>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // C s_55_2: const #2u : u8
        let s_55_2: u8 = 2;
        // D s_55_3: cmp-lt s_55_1 s_55_2
        let s_55_3: bool = ((s_55_1) < (s_55_2));
        // N s_55_4: branch s_55_3 b60 b56
        if s_55_3 {
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
        // D s_56_1: write-var gs#123212 <= s_56_0
        fn_state.gs_123212 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#123212:u8
        let s_57_0: bool = fn_state.gs_123212;
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
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call PhysicalCountInt(s_58_0)
        let s_58_1: u64 = PhysicalCountInt(state, tracer, s_58_0);
        // C s_58_2: const #32s : i64
        let s_58_2: i64 = 32;
        // S s_58_3: cast zx s_58_1 -> bv
        let s_58_3: Bits = Bits::new(s_58_1 as u128, 64u16);
        // C s_58_4: cast zx s_58_2 -> i
        let s_58_4: i128 = (i128::try_from(s_58_2).unwrap());
        // S s_58_5: call Split(s_58_3, s_58_4)
        let s_58_5: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_58_3, s_58_4);
        // D s_58_6: write-var gs#642617 <= s_58_5
        fn_state.gs_642617 = s_58_5;
        // D s_58_7: read-var gs#642617.0:struct
        let s_58_7: Bits = fn_state.gs_642617._0;
        // D s_58_8: cast reint s_58_7 -> u32
        let s_58_8: u32 = (s_58_7.value() as u32);
        // D s_58_9: read-var gs#642617.1:struct
        let s_58_9: Bits = fn_state.gs_642617._1;
        // D s_58_10: cast reint s_58_9 -> u32
        let s_58_10: u32 = (s_58_9.value() as u32);
        // D s_58_11: read-var t2:i
        let s_58_11: i128 = fn_state.t2;
        // D s_58_12: call R_set(s_58_11, s_58_8)
        let s_58_12: () = R_set(state, tracer, s_58_11, s_58_8);
        // D s_58_13: read-var t:i
        let s_58_13: i128 = fn_state.t;
        // D s_58_14: call R_set(s_58_13, s_58_10)
        let s_58_14: () = R_set(state, tracer, s_58_13, s_58_10);
        // N s_58_15: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call PhysicalCountInt(s_59_0)
        let s_59_1: u64 = PhysicalCountInt(state, tracer, s_59_0);
        // C s_59_2: const #() : ()
        let s_59_2: () = ();
        // S s_59_3: call CNTVOFF_read(s_59_2)
        let s_59_3: u64 = CNTVOFF_read(state, tracer, s_59_2);
        // S s_59_4: cast zx s_59_1 -> bv
        let s_59_4: Bits = Bits::new(s_59_1 as u128, 64u16);
        // S s_59_5: cast zx s_59_3 -> bv
        let s_59_5: Bits = Bits::new(s_59_3 as u128, 64u16);
        // S s_59_6: sub s_59_4 s_59_5
        let s_59_6: Bits = ((s_59_4) - (s_59_5));
        // S s_59_7: cast reint s_59_6 -> u64
        let s_59_7: u64 = (s_59_6.value() as u64);
        // C s_59_8: const #32s : i64
        let s_59_8: i64 = 32;
        // S s_59_9: cast zx s_59_7 -> bv
        let s_59_9: Bits = Bits::new(s_59_7 as u128, 64u16);
        // C s_59_10: cast zx s_59_8 -> i
        let s_59_10: i128 = (i128::try_from(s_59_8).unwrap());
        // S s_59_11: call Split(s_59_9, s_59_10)
        let s_59_11: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_59_9, s_59_10);
        // D s_59_12: write-var gs#642624 <= s_59_11
        fn_state.gs_642624 = s_59_11;
        // D s_59_13: read-var gs#642624.0:struct
        let s_59_13: Bits = fn_state.gs_642624._0;
        // D s_59_14: cast reint s_59_13 -> u32
        let s_59_14: u32 = (s_59_13.value() as u32);
        // D s_59_15: read-var gs#642624.1:struct
        let s_59_15: Bits = fn_state.gs_642624._1;
        // D s_59_16: cast reint s_59_15 -> u32
        let s_59_16: u32 = (s_59_15.value() as u32);
        // D s_59_17: read-var t2:i
        let s_59_17: i128 = fn_state.t2;
        // D s_59_18: call R_set(s_59_17, s_59_14)
        let s_59_18: () = R_set(state, tracer, s_59_17, s_59_14);
        // D s_59_19: read-var t:i
        let s_59_19: i128 = fn_state.t;
        // D s_59_20: call R_set(s_59_19, s_59_16)
        let s_59_20: () = R_set(state, tracer, s_59_19, s_59_16);
        // N s_59_21: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #432u : u32
        let s_60_0: u32 = 432;
        // D s_60_1: read-reg s_60_0:u8
        let s_60_1: u8 = {
            let value = state.read_register::<u8>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call ELUsingAArch32(s_60_1)
        let s_60_2: bool = ELUsingAArch32(state, tracer, s_60_1);
        // D s_60_3: write-var gs#123212 <= s_60_2
        fn_state.gs_123212 = s_60_2;
        // N s_60_4: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #() : ()
        let s_61_0: () = ();
        // S s_61_1: call PhysicalCountInt(s_61_0)
        let s_61_1: u64 = PhysicalCountInt(state, tracer, s_61_0);
        // S s_61_2: cast zx s_61_1 -> bv
        let s_61_2: Bits = Bits::new(s_61_1 as u128, 64u16);
        // C s_61_3: const #22400u : u32
        let s_61_3: u32 = 22400;
        // D s_61_4: read-reg s_61_3:u64
        let s_61_4: u64 = {
            let value = state.read_register::<u64>(s_61_3 as isize);
            tracer.read_register(s_61_3 as isize, value);
            value
        };
        // D s_61_5: cast zx s_61_4 -> bv
        let s_61_5: Bits = Bits::new(s_61_4 as u128, 64u16);
        // D s_61_6: sub s_61_2 s_61_5
        let s_61_6: Bits = ((s_61_2) - (s_61_5));
        // D s_61_7: cast reint s_61_6 -> u64
        let s_61_7: u64 = (s_61_6.value() as u64);
        // C s_61_8: const #32s : i64
        let s_61_8: i64 = 32;
        // D s_61_9: cast zx s_61_7 -> bv
        let s_61_9: Bits = Bits::new(s_61_7 as u128, 64u16);
        // C s_61_10: cast zx s_61_8 -> i
        let s_61_10: i128 = (i128::try_from(s_61_8).unwrap());
        // D s_61_11: call Split(s_61_9, s_61_10)
        let s_61_11: ProductTypebc91b195b0b2a883 = Split(state, tracer, s_61_9, s_61_10);
        // D s_61_12: write-var gs#642631 <= s_61_11
        fn_state.gs_642631 = s_61_11;
        // D s_61_13: read-var gs#642631.0:struct
        let s_61_13: Bits = fn_state.gs_642631._0;
        // D s_61_14: cast reint s_61_13 -> u32
        let s_61_14: u32 = (s_61_13.value() as u32);
        // D s_61_15: read-var gs#642631.1:struct
        let s_61_15: Bits = fn_state.gs_642631._1;
        // D s_61_16: cast reint s_61_15 -> u32
        let s_61_16: u32 = (s_61_15.value() as u32);
        // D s_61_17: read-var t2:i
        let s_61_17: i128 = fn_state.t2;
        // D s_61_18: call R_set(s_61_17, s_61_14)
        let s_61_18: () = R_set(state, tracer, s_61_17, s_61_14);
        // D s_61_19: read-var t:i
        let s_61_19: i128 = fn_state.t;
        // D s_61_20: call R_set(s_61_19, s_61_16)
        let s_61_20: () = R_set(state, tracer, s_61_19, s_61_16);
        // N s_61_21: return
        return;
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
        // S s_62_2: not s_62_1
        let s_62_2: bool = !s_62_1;
        // N s_62_3: branch s_62_2 b65 b63
        if s_62_2 {
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
        // C s_63_0: const #102552u : u32
        let s_63_0: u32 = 102552;
        // D s_63_1: read-reg s_63_0:struct
        let s_63_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_0 as isize);
            tracer.read_register(s_63_0 as isize, value);
            value
        };
        // D s_63_2: call _get_HCR_EL2_Type_E2H(s_63_1)
        let s_63_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_63_1);
        // C s_63_3: const #102552u : u32
        let s_63_3: u32 = 102552;
        // D s_63_4: read-reg s_63_3:struct
        let s_63_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_63_3 as isize);
            tracer.read_register(s_63_3 as isize, value);
            value
        };
        // D s_63_5: call _get_HCR_EL2_Type_TGE(s_63_4)
        let s_63_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_63_4);
        // D s_63_6: cast zx s_63_2 -> bv
        let s_63_6: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_7: cast zx s_63_5 -> bv
        let s_63_7: Bits = Bits::new(s_63_5 as u128, 1u16);
        // D s_63_8: cast reint s_63_6 -> u128
        let s_63_8: u128 = (s_63_6.value() as u128);
        // D s_63_9: size-of s_63_6
        let s_63_9: u16 = s_63_6.length();
        // D s_63_10: cast reint s_63_7 -> u128
        let s_63_10: u128 = (s_63_7.value() as u128);
        // D s_63_11: size-of s_63_7
        let s_63_11: u16 = s_63_7.length();
        // D s_63_12: lsl s_63_8 s_63_11
        let s_63_12: u128 = s_63_8 << s_63_11;
        // D s_63_13: or s_63_12 s_63_10
        let s_63_13: u128 = ((s_63_12) | (s_63_10));
        // D s_63_14: add s_63_9 s_63_11
        let s_63_14: u16 = (s_63_9 + s_63_11);
        // D s_63_15: create-bits s_63_13 s_63_14
        let s_63_15: Bits = Bits::new(s_63_13, s_63_14);
        // D s_63_16: cast reint s_63_15 -> u8
        let s_63_16: u8 = (s_63_15.value() as u8);
        // D s_63_17: cast zx s_63_16 -> bv
        let s_63_17: Bits = Bits::new(s_63_16 as u128, 2u16);
        // C s_63_18: const #3u : u8
        let s_63_18: u8 = 3;
        // C s_63_19: cast zx s_63_18 -> bv
        let s_63_19: Bits = Bits::new(s_63_18 as u128, 2u16);
        // D s_63_20: cmp-ne s_63_17 s_63_19
        let s_63_20: bool = ((s_63_17) != (s_63_19));
        // D s_63_21: write-var gs#123210 <= s_63_20
        fn_state.gs_123210 = s_63_20;
        // N s_63_22: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#123210:u8
        let s_64_0: bool = fn_state.gs_123210;
        // D s_64_1: write-var gs#123211 <= s_64_0
        fn_state.gs_123211 = s_64_0;
        // N s_64_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#123210 <= s_65_0
        fn_state.gs_123210 = s_65_0;
        // N s_65_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #432u : u32
        let s_66_0: u32 = 432;
        // D s_66_1: read-reg s_66_0:u8
        let s_66_1: u8 = {
            let value = state.read_register::<u8>(s_66_0 as isize);
            tracer.read_register(s_66_0 as isize, value);
            value
        };
        // D s_66_2: call ELUsingAArch32(s_66_1)
        let s_66_2: bool = ELUsingAArch32(state, tracer, s_66_1);
        // D s_66_3: not s_66_2
        let s_66_3: bool = !s_66_2;
        // D s_66_4: write-var gs#123209 <= s_66_3
        fn_state.gs_123209 = s_66_3;
        // N s_66_5: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #4u : u8
        let s_67_0: u8 = 4;
        // C s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 8u16);
        // C s_67_2: cast zx s_67_1 -> i
        let s_67_2: i128 = (s_67_1.value() as i128);
        // C s_67_3: cast reint s_67_2 -> i64
        let s_67_3: i64 = (s_67_2 as i64);
        // C s_67_4: cast zx s_67_3 -> i
        let s_67_4: i128 = (i128::try_from(s_67_3).unwrap());
        // C s_67_5: const #432u : u32
        let s_67_5: u32 = 432;
        // D s_67_6: read-reg s_67_5:u8
        let s_67_6: u8 = {
            let value = state.read_register::<u8>(s_67_5 as isize);
            tracer.read_register(s_67_5 as isize, value);
            value
        };
        // D s_67_7: call AArch64_AArch32SystemAccessTrap(s_67_6, s_67_4)
        let s_67_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_67_6, s_67_4);
        // N s_67_8: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var __CNTHCTL_EL2_EL1TVCT:u8
        let s_68_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVCT;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 1u16);
        // C s_68_2: const #1u : u8
        let s_68_2: bool = true;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // D s_68_5: write-var gs#123208 <= s_68_4
        fn_state.gs_123208 = s_68_4;
        // N s_68_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #102552u : u32
        let s_69_0: u32 = 102552;
        // D s_69_1: read-reg s_69_0:struct
        let s_69_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // D s_69_2: call _get_HCR_EL2_Type_E2H(s_69_1)
        let s_69_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_69_1);
        // C s_69_3: const #102552u : u32
        let s_69_3: u32 = 102552;
        // D s_69_4: read-reg s_69_3:struct
        let s_69_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_69_3 as isize);
            tracer.read_register(s_69_3 as isize, value);
            value
        };
        // D s_69_5: call _get_HCR_EL2_Type_TGE(s_69_4)
        let s_69_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_69_4);
        // D s_69_6: cast zx s_69_2 -> bv
        let s_69_6: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_7: cast zx s_69_5 -> bv
        let s_69_7: Bits = Bits::new(s_69_5 as u128, 1u16);
        // D s_69_8: cast reint s_69_6 -> u128
        let s_69_8: u128 = (s_69_6.value() as u128);
        // D s_69_9: size-of s_69_6
        let s_69_9: u16 = s_69_6.length();
        // D s_69_10: cast reint s_69_7 -> u128
        let s_69_10: u128 = (s_69_7.value() as u128);
        // D s_69_11: size-of s_69_7
        let s_69_11: u16 = s_69_7.length();
        // D s_69_12: lsl s_69_8 s_69_11
        let s_69_12: u128 = s_69_8 << s_69_11;
        // D s_69_13: or s_69_12 s_69_10
        let s_69_13: u128 = ((s_69_12) | (s_69_10));
        // D s_69_14: add s_69_9 s_69_11
        let s_69_14: u16 = (s_69_9 + s_69_11);
        // D s_69_15: create-bits s_69_13 s_69_14
        let s_69_15: Bits = Bits::new(s_69_13, s_69_14);
        // D s_69_16: cast reint s_69_15 -> u8
        let s_69_16: u8 = (s_69_15.value() as u8);
        // D s_69_17: cast zx s_69_16 -> bv
        let s_69_17: Bits = Bits::new(s_69_16 as u128, 2u16);
        // C s_69_18: const #3u : u8
        let s_69_18: u8 = 3;
        // C s_69_19: cast zx s_69_18 -> bv
        let s_69_19: Bits = Bits::new(s_69_18 as u128, 2u16);
        // D s_69_20: cmp-ne s_69_17 s_69_19
        let s_69_20: bool = ((s_69_17) != (s_69_19));
        // D s_69_21: write-var gs#123207 <= s_69_20
        fn_state.gs_123207 = s_69_20;
        // N s_69_22: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #432u : u32
        let s_70_0: u32 = 432;
        // D s_70_1: read-reg s_70_0:u8
        let s_70_1: u8 = {
            let value = state.read_register::<u8>(s_70_0 as isize);
            tracer.read_register(s_70_0 as isize, value);
            value
        };
        // D s_70_2: call ELUsingAArch32(s_70_1)
        let s_70_2: bool = ELUsingAArch32(state, tracer, s_70_1);
        // D s_70_3: not s_70_2
        let s_70_3: bool = !s_70_2;
        // D s_70_4: write-var gs#123206 <= s_70_3
        fn_state.gs_123206 = s_70_3;
        // N s_70_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #4u : u8
        let s_71_0: u8 = 4;
        // C s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 8u16);
        // C s_71_2: cast zx s_71_1 -> i
        let s_71_2: i128 = (s_71_1.value() as i128);
        // C s_71_3: cast reint s_71_2 -> i64
        let s_71_3: i64 = (s_71_2 as i64);
        // C s_71_4: cast zx s_71_3 -> i
        let s_71_4: i128 = (i128::try_from(s_71_3).unwrap());
        // C s_71_5: const #432u : u32
        let s_71_5: u32 = 432;
        // D s_71_6: read-reg s_71_5:u8
        let s_71_6: u8 = {
            let value = state.read_register::<u8>(s_71_5 as isize);
            tracer.read_register(s_71_5 as isize, value);
            value
        };
        // D s_71_7: call AArch64_AArch32SystemAccessTrap(s_71_6, s_71_4)
        let s_71_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_71_6, s_71_4);
        // N s_71_8: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var __CNTHCTL_EL2_EL0VCTEN:u8
        let s_72_0: bool = fn_state.u__CNTHCTL_EL2_EL0VCTEN;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #0u : u8
        let s_72_2: bool = false;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#123205 <= s_72_4
        fn_state.gs_123205 = s_72_4;
        // N s_72_6: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #102552u : u32
        let s_73_0: u32 = 102552;
        // D s_73_1: read-reg s_73_0:struct
        let s_73_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // D s_73_2: call _get_HCR_EL2_Type_E2H(s_73_1)
        let s_73_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_73_1);
        // C s_73_3: const #102552u : u32
        let s_73_3: u32 = 102552;
        // D s_73_4: read-reg s_73_3:struct
        let s_73_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_3 as isize);
            tracer.read_register(s_73_3 as isize, value);
            value
        };
        // D s_73_5: call _get_HCR_EL2_Type_TGE(s_73_4)
        let s_73_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_73_4);
        // D s_73_6: cast zx s_73_2 -> bv
        let s_73_6: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_7: cast zx s_73_5 -> bv
        let s_73_7: Bits = Bits::new(s_73_5 as u128, 1u16);
        // D s_73_8: cast reint s_73_6 -> u128
        let s_73_8: u128 = (s_73_6.value() as u128);
        // D s_73_9: size-of s_73_6
        let s_73_9: u16 = s_73_6.length();
        // D s_73_10: cast reint s_73_7 -> u128
        let s_73_10: u128 = (s_73_7.value() as u128);
        // D s_73_11: size-of s_73_7
        let s_73_11: u16 = s_73_7.length();
        // D s_73_12: lsl s_73_8 s_73_11
        let s_73_12: u128 = s_73_8 << s_73_11;
        // D s_73_13: or s_73_12 s_73_10
        let s_73_13: u128 = ((s_73_12) | (s_73_10));
        // D s_73_14: add s_73_9 s_73_11
        let s_73_14: u16 = (s_73_9 + s_73_11);
        // D s_73_15: create-bits s_73_13 s_73_14
        let s_73_15: Bits = Bits::new(s_73_13, s_73_14);
        // D s_73_16: cast reint s_73_15 -> u8
        let s_73_16: u8 = (s_73_15.value() as u8);
        // D s_73_17: cast zx s_73_16 -> bv
        let s_73_17: Bits = Bits::new(s_73_16 as u128, 2u16);
        // C s_73_18: const #3u : u8
        let s_73_18: u8 = 3;
        // C s_73_19: cast zx s_73_18 -> bv
        let s_73_19: Bits = Bits::new(s_73_18 as u128, 2u16);
        // D s_73_20: cmp-eq s_73_17 s_73_19
        let s_73_20: bool = ((s_73_17) == (s_73_19));
        // D s_73_21: write-var gs#123204 <= s_73_20
        fn_state.gs_123204 = s_73_20;
        // N s_73_22: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #432u : u32
        let s_74_0: u32 = 432;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: call ELUsingAArch32(s_74_1)
        let s_74_2: bool = ELUsingAArch32(state, tracer, s_74_1);
        // D s_74_3: not s_74_2
        let s_74_3: bool = !s_74_2;
        // D s_74_4: write-var gs#123203 <= s_74_3
        fn_state.gs_123203 = s_74_3;
        // N s_74_5: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #() : ()
        let s_75_0: () = ();
        // S s_75_1: call EL2Enabled(s_75_0)
        let s_75_1: bool = EL2Enabled(state, tracer, s_75_0);
        // N s_75_2: branch s_75_1 b91 b76
        if s_75_1 {
            return block_91(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#123225 <= s_76_0
        fn_state.gs_123225 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#123225:u8
        let s_77_0: bool = fn_state.gs_123225;
        // N s_77_1: branch s_77_0 b90 b78
        if s_77_0 {
            return block_90(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#123226 <= s_78_0
        fn_state.gs_123226 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#123226:u8
        let s_79_0: bool = fn_state.gs_123226;
        // N s_79_1: branch s_79_0 b89 b80
        if s_79_0 {
            return block_89(state, tracer, fn_state);
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
        // N s_80_2: branch s_80_1 b88 b81
        if s_80_1 {
            return block_88(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#123227 <= s_81_0
        fn_state.gs_123227 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#123227:u8
        let s_82_0: bool = fn_state.gs_123227;
        // N s_82_1: branch s_82_0 b87 b83
        if s_82_0 {
            return block_87(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#123228 <= s_83_0
        fn_state.gs_123228 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#123228:u8
        let s_84_0: bool = fn_state.gs_123228;
        // N s_84_1: branch s_84_0 b86 b85
        if s_84_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_85_0: panic
        panic!("{:?}", ());
        // N s_85_1: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: u8 = 0;
        // C s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 8u16);
        // C s_86_2: cast zx s_86_1 -> i
        let s_86_2: i128 = (s_86_1.value() as i128);
        // C s_86_3: cast reint s_86_2 -> i64
        let s_86_3: i64 = (s_86_2 as i64);
        // C s_86_4: cast zx s_86_3 -> i
        let s_86_4: i128 = (i128::try_from(s_86_3).unwrap());
        // S s_86_5: call AArch32_TakeHypTrapException(s_86_4)
        let s_86_5: () = AArch32_TakeHypTrapException(state, tracer, s_86_4);
        // N s_86_6: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var __HCR_TGE:u8
        let s_87_0: bool = fn_state.u__HCR_TGE;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 1u16);
        // C s_87_2: const #1u : u8
        let s_87_2: bool = true;
        // C s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 1u16);
        // D s_87_4: cmp-eq s_87_1 s_87_3
        let s_87_4: bool = ((s_87_1) == (s_87_3));
        // D s_87_5: write-var gs#123228 <= s_87_4
        fn_state.gs_123228 = s_87_4;
        // N s_87_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #432u : u32
        let s_88_0: u32 = 432;
        // D s_88_1: read-reg s_88_0:u8
        let s_88_1: u8 = {
            let value = state.read_register::<u8>(s_88_0 as isize);
            tracer.read_register(s_88_0 as isize, value);
            value
        };
        // D s_88_2: call ELUsingAArch32(s_88_1)
        let s_88_2: bool = ELUsingAArch32(state, tracer, s_88_1);
        // D s_88_3: write-var gs#123227 <= s_88_2
        fn_state.gs_123227 = s_88_2;
        // N s_88_4: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #4u : u8
        let s_89_0: u8 = 4;
        // C s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 8u16);
        // C s_89_2: cast zx s_89_1 -> i
        let s_89_2: i128 = (s_89_1.value() as i128);
        // C s_89_3: cast reint s_89_2 -> i64
        let s_89_3: i64 = (s_89_2 as i64);
        // C s_89_4: cast zx s_89_3 -> i
        let s_89_4: i128 = (i128::try_from(s_89_3).unwrap());
        // C s_89_5: const #432u : u32
        let s_89_5: u32 = 432;
        // D s_89_6: read-reg s_89_5:u8
        let s_89_6: u8 = {
            let value = state.read_register::<u8>(s_89_5 as isize);
            tracer.read_register(s_89_5 as isize, value);
            value
        };
        // D s_89_7: call AArch64_AArch32SystemAccessTrap(s_89_6, s_89_4)
        let s_89_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_89_6, s_89_4);
        // N s_89_8: return
        return;
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var __HCR_EL2_TGE:u8
        let s_90_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 1u16);
        // C s_90_2: const #1u : u8
        let s_90_2: bool = true;
        // C s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // D s_90_5: write-var gs#123226 <= s_90_4
        fn_state.gs_123226 = s_90_4;
        // N s_90_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #432u : u32
        let s_91_0: u32 = 432;
        // D s_91_1: read-reg s_91_0:u8
        let s_91_1: u8 = {
            let value = state.read_register::<u8>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // D s_91_2: call ELUsingAArch32(s_91_1)
        let s_91_2: bool = ELUsingAArch32(state, tracer, s_91_1);
        // D s_91_3: not s_91_2
        let s_91_3: bool = !s_91_2;
        // D s_91_4: write-var gs#123225 <= s_91_3
        fn_state.gs_123225 = s_91_3;
        // N s_91_5: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var __CNTKCTL_PL0VCTEN:u8
        let s_92_0: bool = fn_state.u__CNTKCTL_PL0VCTEN;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #0u : u8
        let s_92_2: bool = false;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#123202 <= s_92_4
        fn_state.gs_123202 = s_92_4;
        // N s_92_6: jump b35
        return block_35(state, tracer, fn_state);
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
        // N s_93_2: branch s_93_1 b101 b94
        if s_93_1 {
            return block_101(state, tracer, fn_state);
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
        // D s_94_1: write-var gs#123229 <= s_94_0
        fn_state.gs_123229 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#123229:u8
        let s_95_0: bool = fn_state.gs_123229;
        // N s_95_1: branch s_95_0 b100 b96
        if s_95_0 {
            return block_100(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#123230 <= s_96_0
        fn_state.gs_123230 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#123230:u8
        let s_97_0: bool = fn_state.gs_123230;
        // N s_97_1: branch s_97_0 b99 b98
        if s_97_0 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #4u : u8
        let s_98_0: u8 = 4;
        // C s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 8u16);
        // C s_98_2: cast zx s_98_1 -> i
        let s_98_2: i128 = (s_98_1.value() as i128);
        // C s_98_3: cast reint s_98_2 -> i64
        let s_98_3: i64 = (s_98_2 as i64);
        // C s_98_4: cast zx s_98_3 -> i
        let s_98_4: i128 = (i128::try_from(s_98_3).unwrap());
        // C s_98_5: const #440u : u32
        let s_98_5: u32 = 440;
        // D s_98_6: read-reg s_98_5:u8
        let s_98_6: u8 = {
            let value = state.read_register::<u8>(s_98_5 as isize);
            tracer.read_register(s_98_5 as isize, value);
            value
        };
        // D s_98_7: call AArch64_AArch32SystemAccessTrap(s_98_6, s_98_4)
        let s_98_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_98_6, s_98_4);
        // N s_98_8: return
        return;
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #4u : u8
        let s_99_0: u8 = 4;
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
        // D s_99_7: call AArch64_AArch32SystemAccessTrap(s_99_6, s_99_4)
        let s_99_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_99_6, s_99_4);
        // N s_99_8: return
        return;
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var __HCR_EL2_TGE:u8
        let s_100_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 1u16);
        // C s_100_2: const #1u : u8
        let s_100_2: bool = true;
        // C s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 1u16);
        // D s_100_4: cmp-eq s_100_1 s_100_3
        let s_100_4: bool = ((s_100_1) == (s_100_3));
        // D s_100_5: write-var gs#123230 <= s_100_4
        fn_state.gs_123230 = s_100_4;
        // N s_100_6: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #432u : u32
        let s_101_0: u32 = 432;
        // D s_101_1: read-reg s_101_0:u8
        let s_101_1: u8 = {
            let value = state.read_register::<u8>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: call ELUsingAArch32(s_101_1)
        let s_101_2: bool = ELUsingAArch32(state, tracer, s_101_1);
        // D s_101_3: not s_101_2
        let s_101_3: bool = !s_101_2;
        // D s_101_4: write-var gs#123229 <= s_101_3
        fn_state.gs_123229 = s_101_3;
        // N s_101_5: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var __CNTKCTL_EL1_EL0VCTEN:u8
        let s_102_0: bool = fn_state.u__CNTKCTL_EL1_EL0VCTEN;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 1u16);
        // C s_102_2: const #0u : u8
        let s_102_2: bool = false;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: write-var gs#123201 <= s_102_4
        fn_state.gs_123201 = s_102_4;
        // N s_102_6: jump b32
        return block_32(state, tracer, fn_state);
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
        // N s_103_2: branch s_103_1 b106 b104
        if s_103_1 {
            return block_106(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#123199 <= s_104_0
        fn_state.gs_123199 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#123199:u8
        let s_105_0: bool = fn_state.gs_123199;
        // D s_105_1: not s_105_0
        let s_105_1: bool = !s_105_0;
        // D s_105_2: write-var gs#123200 <= s_105_1
        fn_state.gs_123200 = s_105_1;
        // N s_105_3: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #102552u : u32
        let s_106_0: u32 = 102552;
        // D s_106_1: read-reg s_106_0:struct
        let s_106_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // D s_106_2: call _get_HCR_EL2_Type_E2H(s_106_1)
        let s_106_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_106_1);
        // C s_106_3: const #102552u : u32
        let s_106_3: u32 = 102552;
        // D s_106_4: read-reg s_106_3:struct
        let s_106_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_106_3 as isize);
            tracer.read_register(s_106_3 as isize, value);
            value
        };
        // D s_106_5: call _get_HCR_EL2_Type_TGE(s_106_4)
        let s_106_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_106_4);
        // D s_106_6: cast zx s_106_2 -> bv
        let s_106_6: Bits = Bits::new(s_106_2 as u128, 1u16);
        // D s_106_7: cast zx s_106_5 -> bv
        let s_106_7: Bits = Bits::new(s_106_5 as u128, 1u16);
        // D s_106_8: cast reint s_106_6 -> u128
        let s_106_8: u128 = (s_106_6.value() as u128);
        // D s_106_9: size-of s_106_6
        let s_106_9: u16 = s_106_6.length();
        // D s_106_10: cast reint s_106_7 -> u128
        let s_106_10: u128 = (s_106_7.value() as u128);
        // D s_106_11: size-of s_106_7
        let s_106_11: u16 = s_106_7.length();
        // D s_106_12: lsl s_106_8 s_106_11
        let s_106_12: u128 = s_106_8 << s_106_11;
        // D s_106_13: or s_106_12 s_106_10
        let s_106_13: u128 = ((s_106_12) | (s_106_10));
        // D s_106_14: add s_106_9 s_106_11
        let s_106_14: u16 = (s_106_9 + s_106_11);
        // D s_106_15: create-bits s_106_13 s_106_14
        let s_106_15: Bits = Bits::new(s_106_13, s_106_14);
        // D s_106_16: cast reint s_106_15 -> u8
        let s_106_16: u8 = (s_106_15.value() as u8);
        // D s_106_17: cast zx s_106_16 -> bv
        let s_106_17: Bits = Bits::new(s_106_16 as u128, 2u16);
        // C s_106_18: const #3u : u8
        let s_106_18: u8 = 3;
        // C s_106_19: cast zx s_106_18 -> bv
        let s_106_19: Bits = Bits::new(s_106_18 as u128, 2u16);
        // D s_106_20: cmp-eq s_106_17 s_106_19
        let s_106_20: bool = ((s_106_17) == (s_106_19));
        // D s_106_21: write-var gs#123199 <= s_106_20
        fn_state.gs_123199 = s_106_20;
        // N s_106_22: jump b105
        return block_105(state, tracer, fn_state);
    }
}
