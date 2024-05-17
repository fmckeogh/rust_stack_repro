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
use u_get_SCTLR_EL2_Type_CP15BEN::*;
use u_get_HSTR_Type_T7::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_SCTLR_EL1_Type_CP15BEN::*;
use u_get_SCTLR_Type_CP15BEN::*;
use u_get_HSTR_EL2_Type_T7::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use SCTLR_read__2::*;
use ELUsingAArch32::*;
use u_get_HSCTLR_Type_CP15BEN::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use CP15ISB::*;
use HSCTLR_read::*;
use common::*;
pub fn CP15ISB_SysRegWrite32_b2e3d8ab360938e2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRn: u8,
    opc2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_124495: bool,
        u__SCTLR_EL2_CP15BEN: bool,
        gs_124506: bool,
        gs_124507: bool,
        gs_124508: bool,
        gs_124503: bool,
        gs_124493: bool,
        gs_124501: bool,
        u__HSTR_EL2_T7: bool,
        gs_124496: bool,
        u__HSTR_T7: bool,
        gs_124498: bool,
        gs_124494: bool,
        u__SCTLR_EL1_CP15BEN: bool,
        gs_124502: bool,
        u__HSCTLR_CP15BEN: bool,
        gs_124499: bool,
        gs_124497: bool,
        u__PSTATE_EL: u8,
        gs_124500: bool,
        u__SCTLR_CP15BEN: bool,
        gs_124505: bool,
        gs_124504: bool,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRn: u8,
        opc2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRn,
        opc2,
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
        // C s_0_3: const #90272u : u32
        let s_0_3: u32 = 90272;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCTLR_EL1_Type_CP15BEN(s_0_4)
        let s_0_5: bool = u_get_SCTLR_EL1_Type_CP15BEN(state, tracer, s_0_4);
        // D s_0_6: write-var __SCTLR_EL1_CP15BEN <= s_0_5
        fn_state.u__SCTLR_EL1_CP15BEN = s_0_5;
        // C s_0_7: const #20784u : u32
        let s_0_7: u32 = 20784;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCTLR_EL2_Type_CP15BEN(s_0_8)
        let s_0_9: bool = u_get_SCTLR_EL2_Type_CP15BEN(state, tracer, s_0_8);
        // D s_0_10: write-var __SCTLR_EL2_CP15BEN <= s_0_9
        fn_state.u__SCTLR_EL2_CP15BEN = s_0_9;
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call SCTLR_read__2(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_0_11);
        // S s_0_13: call _get_SCTLR_Type_CP15BEN(s_0_12)
        let s_0_13: bool = u_get_SCTLR_Type_CP15BEN(state, tracer, s_0_12);
        // D s_0_14: write-var __SCTLR_CP15BEN <= s_0_13
        fn_state.u__SCTLR_CP15BEN = s_0_13;
        // C s_0_15: const #104936u : u32
        let s_0_15: u32 = 104936;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HSTR_EL2_Type_T7(s_0_16)
        let s_0_17: bool = u_get_HSTR_EL2_Type_T7(state, tracer, s_0_16);
        // D s_0_18: write-var __HSTR_EL2_T7 <= s_0_17
        fn_state.u__HSTR_EL2_T7 = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call HSTR_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_19);
        // S s_0_21: call _get_HSTR_Type_T7(s_0_20)
        let s_0_21: bool = u_get_HSTR_Type_T7(state, tracer, s_0_20);
        // D s_0_22: write-var __HSTR_T7 <= s_0_21
        fn_state.u__HSTR_T7 = s_0_21;
        // C s_0_23: const #() : ()
        let s_0_23: () = ();
        // S s_0_24: call HSCTLR_read(s_0_23)
        let s_0_24: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_0_23);
        // S s_0_25: call _get_HSCTLR_Type_CP15BEN(s_0_24)
        let s_0_25: bool = u_get_HSCTLR_Type_CP15BEN(state, tracer, s_0_24);
        // D s_0_26: write-var __HSCTLR_CP15BEN <= s_0_25
        fn_state.u__HSCTLR_CP15BEN = s_0_25;
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
        // N s_0_33: branch s_0_32 b30 b1
        if s_0_32 {
            return block_30(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b11 b2
        if s_1_5 {
            return block_11(state, tracer, fn_state);
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
        // D s_5_0: read-var __SCTLR_CP15BEN:u8
        let s_5_0: bool = fn_state.u__SCTLR_CP15BEN;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
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
        // S s_6_1: call CP15ISB(s_6_0)
        let s_6_1: () = CP15ISB(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var __HSCTLR_CP15BEN:u8
        let s_8_0: bool = fn_state.u__HSCTLR_CP15BEN;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #0u : u8
        let s_8_2: bool = false;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b10 b9
        if s_8_4 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call CP15ISB(s_9_0)
        let s_9_1: () = CP15ISB(state, tracer, s_9_0);
        // N s_9_2: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
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
        // N s_11_2: branch s_11_1 b29 b12
        if s_11_1 {
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
        // D s_12_1: write-var gs#124493 <= s_12_0
        fn_state.gs_124493 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#124493:u8
        let s_13_0: bool = fn_state.gs_124493;
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
        // D s_14_1: write-var gs#124494 <= s_14_0
        fn_state.gs_124494 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#124494:u8
        let s_15_0: bool = fn_state.gs_124494;
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
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call EL2Enabled(s_16_0)
        let s_16_1: bool = EL2Enabled(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b26 b17
        if s_16_1 {
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
        // D s_17_1: write-var gs#124495 <= s_17_0
        fn_state.gs_124495 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#124495:u8
        let s_18_0: bool = fn_state.gs_124495;
        // N s_18_1: branch s_18_0 b25 b19
        if s_18_0 {
            return block_25(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#124496 <= s_19_0
        fn_state.gs_124496 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#124496:u8
        let s_20_0: bool = fn_state.gs_124496;
        // N s_20_1: branch s_20_0 b24 b21
        if s_20_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var __SCTLR_CP15BEN:u8
        let s_21_0: bool = fn_state.u__SCTLR_CP15BEN;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 1u16);
        // C s_21_2: const #0u : u8
        let s_21_2: bool = false;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // N s_21_5: branch s_21_4 b23 b22
        if s_21_4 {
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
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call CP15ISB(s_22_0)
        let s_22_1: () = CP15ISB(state, tracer, s_22_0);
        // N s_22_2: return
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
        // C s_24_0: const #3u : u8
        let s_24_0: u8 = 3;
        // C s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 8u16);
        // C s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (s_24_1.value() as i128);
        // C s_24_3: cast reint s_24_2 -> i64
        let s_24_3: i64 = (s_24_2 as i64);
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // S s_24_5: call AArch32_TakeHypTrapException(s_24_4)
        let s_24_5: () = AArch32_TakeHypTrapException(state, tracer, s_24_4);
        // N s_24_6: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __HSTR_T7:u8
        let s_25_0: bool = fn_state.u__HSTR_T7;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#124496 <= s_25_4
        fn_state.gs_124496 = s_25_4;
        // N s_25_6: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #432u : u32
        let s_26_0: u32 = 432;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: u8 = {
            let value = state.read_register::<u8>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: call ELUsingAArch32(s_26_1)
        let s_26_2: bool = ELUsingAArch32(state, tracer, s_26_1);
        // D s_26_3: write-var gs#124495 <= s_26_2
        fn_state.gs_124495 = s_26_2;
        // N s_26_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #3u : u8
        let s_27_0: u8 = 3;
        // C s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 8u16);
        // C s_27_2: cast zx s_27_1 -> i
        let s_27_2: i128 = (s_27_1.value() as i128);
        // C s_27_3: cast reint s_27_2 -> i64
        let s_27_3: i64 = (s_27_2 as i64);
        // C s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // C s_27_5: const #432u : u32
        let s_27_5: u32 = 432;
        // D s_27_6: read-reg s_27_5:u8
        let s_27_6: u8 = {
            let value = state.read_register::<u8>(s_27_5 as isize);
            tracer.read_register(s_27_5 as isize, value);
            value
        };
        // D s_27_7: call AArch64_AArch32SystemAccessTrap(s_27_6, s_27_4)
        let s_27_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_27_6, s_27_4);
        // N s_27_8: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var __HSTR_EL2_T7:u8
        let s_28_0: bool = fn_state.u__HSTR_EL2_T7;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 1u16);
        // C s_28_2: const #1u : u8
        let s_28_2: bool = true;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: write-var gs#124494 <= s_28_4
        fn_state.gs_124494 = s_28_4;
        // N s_28_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #432u : u32
        let s_29_0: u32 = 432;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call ELUsingAArch32(s_29_1)
        let s_29_2: bool = ELUsingAArch32(state, tracer, s_29_1);
        // D s_29_3: not s_29_2
        let s_29_3: bool = !s_29_2;
        // D s_29_4: write-var gs#124493 <= s_29_3
        fn_state.gs_124493 = s_29_3;
        // N s_29_5: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #440u : u32
        let s_30_0: u32 = 440;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call ELUsingAArch32(s_30_1)
        let s_30_2: bool = ELUsingAArch32(state, tracer, s_30_1);
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // N s_30_4: branch s_30_3 b73 b31
        if s_30_3 {
            return block_73(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#124498 <= s_31_0
        fn_state.gs_124498 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#124498:u8
        let s_32_0: bool = fn_state.gs_124498;
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
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#124499 <= s_33_0
        fn_state.gs_124499 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#124499:u8
        let s_34_0: bool = fn_state.gs_124499;
        // N s_34_1: branch s_34_0 b71 b35
        if s_34_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
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
        // N s_35_2: branch s_35_1 b70 b36
        if s_35_1 {
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
        // D s_36_1: write-var gs#124500 <= s_36_0
        fn_state.gs_124500 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#124500:u8
        let s_37_0: bool = fn_state.gs_124500;
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
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#124501 <= s_38_0
        fn_state.gs_124501 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#124501:u8
        let s_39_0: bool = fn_state.gs_124501;
        // N s_39_1: branch s_39_0 b68 b40
        if s_39_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#124502 <= s_40_0
        fn_state.gs_124502 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#124502:u8
        let s_41_0: bool = fn_state.gs_124502;
        // N s_41_1: branch s_41_0 b67 b42
        if s_41_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
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
        // N s_42_3: branch s_42_2 b66 b43
        if s_42_2 {
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
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#124503 <= s_43_0
        fn_state.gs_124503 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#124503:u8
        let s_44_0: bool = fn_state.gs_124503;
        // N s_44_1: branch s_44_0 b65 b45
        if s_44_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call EL2Enabled(s_45_0)
        let s_45_1: bool = EL2Enabled(state, tracer, s_45_0);
        // N s_45_2: branch s_45_1 b64 b46
        if s_45_1 {
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
        // D s_46_1: write-var gs#124504 <= s_46_0
        fn_state.gs_124504 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#124504:u8
        let s_47_0: bool = fn_state.gs_124504;
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
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#124505 <= s_48_0
        fn_state.gs_124505 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#124505:u8
        let s_49_0: bool = fn_state.gs_124505;
        // N s_49_1: branch s_49_0 b62 b50
        if s_49_0 {
            return block_62(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#124506 <= s_50_0
        fn_state.gs_124506 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#124506:u8
        let s_51_0: bool = fn_state.gs_124506;
        // N s_51_1: branch s_51_0 b61 b52
        if s_51_0 {
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
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EL2Enabled(s_52_0)
        let s_52_1: bool = EL2Enabled(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b60 b53
        if s_52_1 {
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
        // D s_53_1: write-var gs#124507 <= s_53_0
        fn_state.gs_124507 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#124507:u8
        let s_54_0: bool = fn_state.gs_124507;
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
        // D s_55_1: write-var gs#124508 <= s_55_0
        fn_state.gs_124508 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#124508:u8
        let s_56_0: bool = fn_state.gs_124508;
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
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call CP15ISB(s_57_0)
        let s_57_1: () = CP15ISB(state, tracer, s_57_0);
        // N s_57_2: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #3u : u8
        let s_58_0: u8 = 3;
        // C s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 8u16);
        // C s_58_2: cast zx s_58_1 -> i
        let s_58_2: i128 = (s_58_1.value() as i128);
        // C s_58_3: cast reint s_58_2 -> i64
        let s_58_3: i64 = (s_58_2 as i64);
        // C s_58_4: cast zx s_58_3 -> i
        let s_58_4: i128 = (i128::try_from(s_58_3).unwrap());
        // S s_58_5: call AArch32_TakeHypTrapException(s_58_4)
        let s_58_5: () = AArch32_TakeHypTrapException(state, tracer, s_58_4);
        // N s_58_6: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var __HSTR_T7:u8
        let s_59_0: bool = fn_state.u__HSTR_T7;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 1u16);
        // C s_59_2: const #1u : u8
        let s_59_2: bool = true;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: write-var gs#124508 <= s_59_4
        fn_state.gs_124508 = s_59_4;
        // N s_59_6: jump b56
        return block_56(state, tracer, fn_state);
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
        // D s_60_3: write-var gs#124507 <= s_60_2
        fn_state.gs_124507 = s_60_2;
        // N s_60_4: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #3u : u8
        let s_61_0: u8 = 3;
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
        // D s_61_7: call AArch64_AArch32SystemAccessTrap(s_61_6, s_61_4)
        let s_61_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_61_6, s_61_4);
        // N s_61_8: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var __HSTR_EL2_T7:u8
        let s_62_0: bool = fn_state.u__HSTR_EL2_T7;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#124506 <= s_62_4
        fn_state.gs_124506 = s_62_4;
        // N s_62_6: jump b51
        return block_51(state, tracer, fn_state);
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
        // D s_63_21: write-var gs#124505 <= s_63_20
        fn_state.gs_124505 = s_63_20;
        // N s_63_22: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #432u : u32
        let s_64_0: u32 = 432;
        // D s_64_1: read-reg s_64_0:u8
        let s_64_1: u8 = {
            let value = state.read_register::<u8>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call ELUsingAArch32(s_64_1)
        let s_64_2: bool = ELUsingAArch32(state, tracer, s_64_1);
        // D s_64_3: not s_64_2
        let s_64_3: bool = !s_64_2;
        // D s_64_4: write-var gs#124504 <= s_64_3
        fn_state.gs_124504 = s_64_3;
        // N s_64_5: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_65_0: panic
        panic!("{:?}", ());
        // N s_65_1: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var __SCTLR_CP15BEN:u8
        let s_66_0: bool = fn_state.u__SCTLR_CP15BEN;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #0u : u8
        let s_66_2: bool = false;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#124503 <= s_66_4
        fn_state.gs_124503 = s_66_4;
        // N s_66_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_67_0: panic
        panic!("{:?}", ());
        // N s_67_1: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var __SCTLR_EL2_CP15BEN:u8
        let s_68_0: bool = fn_state.u__SCTLR_EL2_CP15BEN;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 1u16);
        // C s_68_2: const #0u : u8
        let s_68_2: bool = false;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // D s_68_5: write-var gs#124502 <= s_68_4
        fn_state.gs_124502 = s_68_4;
        // N s_68_6: jump b41
        return block_41(state, tracer, fn_state);
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
        // D s_69_20: cmp-eq s_69_17 s_69_19
        let s_69_20: bool = ((s_69_17) == (s_69_19));
        // D s_69_21: write-var gs#124501 <= s_69_20
        fn_state.gs_124501 = s_69_20;
        // N s_69_22: jump b39
        return block_39(state, tracer, fn_state);
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
        // D s_70_4: write-var gs#124500 <= s_70_3
        fn_state.gs_124500 = s_70_3;
        // N s_70_5: jump b37
        return block_37(state, tracer, fn_state);
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
        // D s_72_0: read-var __SCTLR_EL1_CP15BEN:u8
        let s_72_0: bool = fn_state.u__SCTLR_EL1_CP15BEN;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #0u : u8
        let s_72_2: bool = false;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#124499 <= s_72_4
        fn_state.gs_124499 = s_72_4;
        // N s_72_6: jump b34
        return block_34(state, tracer, fn_state);
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
        // N s_73_2: branch s_73_1 b76 b74
        if s_73_1 {
            return block_76(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#124497 <= s_74_0
        fn_state.gs_124497 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#124497:u8
        let s_75_0: bool = fn_state.gs_124497;
        // D s_75_1: not s_75_0
        let s_75_1: bool = !s_75_0;
        // D s_75_2: write-var gs#124498 <= s_75_1
        fn_state.gs_124498 = s_75_1;
        // N s_75_3: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #102552u : u32
        let s_76_0: u32 = 102552;
        // D s_76_1: read-reg s_76_0:struct
        let s_76_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // D s_76_2: call _get_HCR_EL2_Type_E2H(s_76_1)
        let s_76_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_76_1);
        // C s_76_3: const #102552u : u32
        let s_76_3: u32 = 102552;
        // D s_76_4: read-reg s_76_3:struct
        let s_76_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_76_3 as isize);
            tracer.read_register(s_76_3 as isize, value);
            value
        };
        // D s_76_5: call _get_HCR_EL2_Type_TGE(s_76_4)
        let s_76_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_76_4);
        // D s_76_6: cast zx s_76_2 -> bv
        let s_76_6: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_7: cast zx s_76_5 -> bv
        let s_76_7: Bits = Bits::new(s_76_5 as u128, 1u16);
        // D s_76_8: cast reint s_76_6 -> u128
        let s_76_8: u128 = (s_76_6.value() as u128);
        // D s_76_9: size-of s_76_6
        let s_76_9: u16 = s_76_6.length();
        // D s_76_10: cast reint s_76_7 -> u128
        let s_76_10: u128 = (s_76_7.value() as u128);
        // D s_76_11: size-of s_76_7
        let s_76_11: u16 = s_76_7.length();
        // D s_76_12: lsl s_76_8 s_76_11
        let s_76_12: u128 = s_76_8 << s_76_11;
        // D s_76_13: or s_76_12 s_76_10
        let s_76_13: u128 = ((s_76_12) | (s_76_10));
        // D s_76_14: add s_76_9 s_76_11
        let s_76_14: u16 = (s_76_9 + s_76_11);
        // D s_76_15: create-bits s_76_13 s_76_14
        let s_76_15: Bits = Bits::new(s_76_13, s_76_14);
        // D s_76_16: cast reint s_76_15 -> u8
        let s_76_16: u8 = (s_76_15.value() as u8);
        // D s_76_17: cast zx s_76_16 -> bv
        let s_76_17: Bits = Bits::new(s_76_16 as u128, 2u16);
        // C s_76_18: const #3u : u8
        let s_76_18: u8 = 3;
        // C s_76_19: cast zx s_76_18 -> bv
        let s_76_19: Bits = Bits::new(s_76_18 as u128, 2u16);
        // D s_76_20: cmp-eq s_76_17 s_76_19
        let s_76_20: bool = ((s_76_17) == (s_76_19));
        // D s_76_21: write-var gs#124497 <= s_76_20
        fn_state.gs_124497 = s_76_20;
        // N s_76_22: jump b75
        return block_75(state, tracer, fn_state);
    }
}
