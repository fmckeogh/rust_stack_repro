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
use AArch64_AArch32SystemAccessTrap::*;
use PAR_read::*;
use ELUsingAArch32::*;
use R_set::*;
use u_get_HSTR_EL2_Type_T7::*;
use u_get_SCR_Type_NS::*;
use HSTR_read::*;
use EL2Enabled::*;
use u_get_HSTR_Type_T7::*;
use common::*;
pub fn PAR_SysRegRead32_6e09f827d9ae2cf7<T: Tracer>(
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
        gs_112649: bool,
        gs_112648: bool,
        gs_112646: bool,
        u__HSTR_T7: bool,
        ga_184315: ProductType5c790c8ef59cc8b2,
        gs_112650: bool,
        gs_112641: bool,
        u__PSTATE_EL: u8,
        u__SCR_NS: bool,
        ga_184307: ProductType5c790c8ef59cc8b2,
        u__HSTR_EL2_T7: bool,
        gs_112647: bool,
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
        // C s_0_3: const #104936u : u32
        let s_0_3: u32 = 104936;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HSTR_EL2_Type_T7(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T7(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T7 <= s_0_5
        fn_state.u__HSTR_EL2_T7 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T7(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T7(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T7 <= s_0_9
        fn_state.u__HSTR_T7 = s_0_9;
        // C s_0_11: const #20920u : u32
        let s_0_11: u32 = 20920;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_Type_NS(s_0_12)
        let s_0_13: bool = u_get_SCR_Type_NS(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_NS <= s_0_13
        fn_state.u__SCR_NS = s_0_13;
        // D s_0_15: read-var __PSTATE_EL:u8
        let s_0_15: u8 = fn_state.u__PSTATE_EL;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 2u16);
        // C s_0_17: const #448u : u32
        let s_0_17: u32 = 448;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: u8 = {
            let value = state.read_register::<u8>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 2u16);
        // D s_0_20: cmp-eq s_0_16 s_0_19
        let s_0_20: bool = ((s_0_16) == (s_0_19));
        // N s_0_21: branch s_0_20 b36 b1
        if s_0_20 {
            return block_36(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b14 b2
        if s_1_5 {
            return block_14(state, tracer, fn_state);
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
        // D s_5_0: read-var __SCR_NS:u8
        let s_5_0: bool = fn_state.u__SCR_NS;
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
        // C s_6_0: const #102824u : u32
        let s_6_0: u32 = 102824;
        // D s_6_1: read-reg s_6_0:u64
        let s_6_1: u64 = {
            let value = state.read_register::<u64>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #0s : i
        let s_6_2: i128 = 0;
        // D s_6_3: cast zx s_6_1 -> bv
        let s_6_3: Bits = Bits::new(s_6_1 as u128, 64u16);
        // C s_6_4: const #1s : i64
        let s_6_4: i64 = 1;
        // C s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // C s_6_6: const #31s : i
        let s_6_6: i128 = 31;
        // C s_6_7: add s_6_6 s_6_5
        let s_6_7: i128 = (s_6_6 + s_6_5);
        // D s_6_8: bit-extract s_6_3 s_6_2 s_6_7
        let s_6_8: Bits = (Bits::new(
            ((s_6_3) >> (s_6_2)).value(),
            u16::try_from(s_6_7).unwrap(),
        ));
        // D s_6_9: cast reint s_6_8 -> u32
        let s_6_9: u32 = (s_6_8.value() as u32);
        // D s_6_10: read-var t:i
        let s_6_10: i128 = fn_state.t;
        // D s_6_11: call R_set(s_6_10, s_6_9)
        let s_6_11: () = R_set(state, tracer, s_6_10, s_6_9);
        // N s_6_12: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #90712u : u32
        let s_7_0: u32 = 90712;
        // D s_7_1: read-reg s_7_0:u64
        let s_7_1: u64 = {
            let value = state.read_register::<u64>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #0s : i
        let s_7_2: i128 = 0;
        // D s_7_3: cast zx s_7_1 -> bv
        let s_7_3: Bits = Bits::new(s_7_1 as u128, 64u16);
        // C s_7_4: const #1s : i64
        let s_7_4: i64 = 1;
        // C s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // C s_7_6: const #31s : i
        let s_7_6: i128 = 31;
        // C s_7_7: add s_7_6 s_7_5
        let s_7_7: i128 = (s_7_6 + s_7_5);
        // D s_7_8: bit-extract s_7_3 s_7_2 s_7_7
        let s_7_8: Bits = (Bits::new(
            ((s_7_3) >> (s_7_2)).value(),
            u16::try_from(s_7_7).unwrap(),
        ));
        // D s_7_9: cast reint s_7_8 -> u32
        let s_7_9: u32 = (s_7_8.value() as u32);
        // D s_7_10: read-var t:i
        let s_7_10: i128 = fn_state.t;
        // D s_7_11: call R_set(s_7_10, s_7_9)
        let s_7_11: () = R_set(state, tracer, s_7_10, s_7_9);
        // N s_7_12: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #424u : u32
        let s_8_0: u32 = 424;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // D s_8_3: cmp-lt s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) < (s_8_2));
        // N s_8_4: branch s_8_3 b13 b9
        if s_8_3 {
            return block_13(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#112641 <= s_9_0
        fn_state.gs_112641 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#112641:u8
        let s_10_0: bool = fn_state.gs_112641;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call PAR_read(s_11_0)
        let s_11_1: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_11_0);
        // D s_11_2: write-var ga#184315 <= s_11_1
        fn_state.ga_184315 = s_11_1;
        // D s_11_3: read-var ga#184315.0:struct
        let s_11_3: u64 = fn_state.ga_184315._0;
        // C s_11_4: const #0s : i
        let s_11_4: i128 = 0;
        // D s_11_5: cast zx s_11_3 -> bv
        let s_11_5: Bits = Bits::new(s_11_3 as u128, 64u16);
        // C s_11_6: const #1s : i64
        let s_11_6: i64 = 1;
        // C s_11_7: cast zx s_11_6 -> i
        let s_11_7: i128 = (i128::try_from(s_11_6).unwrap());
        // C s_11_8: const #31s : i
        let s_11_8: i128 = 31;
        // C s_11_9: add s_11_8 s_11_7
        let s_11_9: i128 = (s_11_8 + s_11_7);
        // D s_11_10: bit-extract s_11_5 s_11_4 s_11_9
        let s_11_10: Bits = (Bits::new(
            ((s_11_5) >> (s_11_4)).value(),
            u16::try_from(s_11_9).unwrap(),
        ));
        // D s_11_11: cast reint s_11_10 -> u32
        let s_11_11: u32 = (s_11_10.value() as u32);
        // D s_11_12: read-var t:i
        let s_11_12: i128 = fn_state.t;
        // D s_11_13: call R_set(s_11_12, s_11_11)
        let s_11_13: () = R_set(state, tracer, s_11_12, s_11_11);
        // N s_11_14: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #102824u : u32
        let s_12_0: u32 = 102824;
        // D s_12_1: read-reg s_12_0:u64
        let s_12_1: u64 = {
            let value = state.read_register::<u64>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #0s : i
        let s_12_2: i128 = 0;
        // D s_12_3: cast zx s_12_1 -> bv
        let s_12_3: Bits = Bits::new(s_12_1 as u128, 64u16);
        // C s_12_4: const #1s : i64
        let s_12_4: i64 = 1;
        // C s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // C s_12_6: const #31s : i
        let s_12_6: i128 = 31;
        // C s_12_7: add s_12_6 s_12_5
        let s_12_7: i128 = (s_12_6 + s_12_5);
        // D s_12_8: bit-extract s_12_3 s_12_2 s_12_7
        let s_12_8: Bits = (Bits::new(
            ((s_12_3) >> (s_12_2)).value(),
            u16::try_from(s_12_7).unwrap(),
        ));
        // D s_12_9: cast reint s_12_8 -> u32
        let s_12_9: u32 = (s_12_8.value() as u32);
        // D s_12_10: read-var t:i
        let s_12_10: i128 = fn_state.t;
        // D s_12_11: call R_set(s_12_10, s_12_9)
        let s_12_11: () = R_set(state, tracer, s_12_10, s_12_9);
        // N s_12_12: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #424u : u32
        let s_13_0: u32 = 424;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call ELUsingAArch32(s_13_1)
        let s_13_2: bool = ELUsingAArch32(state, tracer, s_13_1);
        // D s_13_3: write-var gs#112641 <= s_13_2
        fn_state.gs_112641 = s_13_2;
        // N s_13_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call EL2Enabled(s_14_0)
        let s_14_1: bool = EL2Enabled(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b35 b15
        if s_14_1 {
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
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#112646 <= s_15_0
        fn_state.gs_112646 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#112646:u8
        let s_16_0: bool = fn_state.gs_112646;
        // N s_16_1: branch s_16_0 b34 b17
        if s_16_0 {
            return block_34(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#112647 <= s_17_0
        fn_state.gs_112647 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#112647:u8
        let s_18_0: bool = fn_state.gs_112647;
        // N s_18_1: branch s_18_0 b33 b19
        if s_18_0 {
            return block_33(state, tracer, fn_state);
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
        // S s_19_1: call EL2Enabled(s_19_0)
        let s_19_1: bool = EL2Enabled(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b32 b20
        if s_19_1 {
            return block_32(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#112648 <= s_20_0
        fn_state.gs_112648 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#112648:u8
        let s_21_0: bool = fn_state.gs_112648;
        // N s_21_1: branch s_21_0 b31 b22
        if s_21_0 {
            return block_31(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#112649 <= s_22_0
        fn_state.gs_112649 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#112649:u8
        let s_23_0: bool = fn_state.gs_112649;
        // N s_23_1: branch s_23_0 b30 b24
        if s_23_0 {
            return block_30(state, tracer, fn_state);
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
        // N s_24_4: branch s_24_3 b29 b25
        if s_24_3 {
            return block_29(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#112650 <= s_25_0
        fn_state.gs_112650 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#112650:u8
        let s_26_0: bool = fn_state.gs_112650;
        // N s_26_1: branch s_26_0 b28 b27
        if s_26_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call PAR_read(s_27_0)
        let s_27_1: ProductType5c790c8ef59cc8b2 = PAR_read(state, tracer, s_27_0);
        // D s_27_2: write-var ga#184307 <= s_27_1
        fn_state.ga_184307 = s_27_1;
        // D s_27_3: read-var ga#184307.0:struct
        let s_27_3: u64 = fn_state.ga_184307._0;
        // C s_27_4: const #0s : i
        let s_27_4: i128 = 0;
        // D s_27_5: cast zx s_27_3 -> bv
        let s_27_5: Bits = Bits::new(s_27_3 as u128, 64u16);
        // C s_27_6: const #1s : i64
        let s_27_6: i64 = 1;
        // C s_27_7: cast zx s_27_6 -> i
        let s_27_7: i128 = (i128::try_from(s_27_6).unwrap());
        // C s_27_8: const #31s : i
        let s_27_8: i128 = 31;
        // C s_27_9: add s_27_8 s_27_7
        let s_27_9: i128 = (s_27_8 + s_27_7);
        // D s_27_10: bit-extract s_27_5 s_27_4 s_27_9
        let s_27_10: Bits = (Bits::new(
            ((s_27_5) >> (s_27_4)).value(),
            u16::try_from(s_27_9).unwrap(),
        ));
        // D s_27_11: cast reint s_27_10 -> u32
        let s_27_11: u32 = (s_27_10.value() as u32);
        // D s_27_12: read-var t:i
        let s_27_12: i128 = fn_state.t;
        // D s_27_13: call R_set(s_27_12, s_27_11)
        let s_27_13: () = R_set(state, tracer, s_27_12, s_27_11);
        // N s_27_14: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #102824u : u32
        let s_28_0: u32 = 102824;
        // D s_28_1: read-reg s_28_0:u64
        let s_28_1: u64 = {
            let value = state.read_register::<u64>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // C s_28_2: const #0s : i
        let s_28_2: i128 = 0;
        // D s_28_3: cast zx s_28_1 -> bv
        let s_28_3: Bits = Bits::new(s_28_1 as u128, 64u16);
        // C s_28_4: const #1s : i64
        let s_28_4: i64 = 1;
        // C s_28_5: cast zx s_28_4 -> i
        let s_28_5: i128 = (i128::try_from(s_28_4).unwrap());
        // C s_28_6: const #31s : i
        let s_28_6: i128 = 31;
        // C s_28_7: add s_28_6 s_28_5
        let s_28_7: i128 = (s_28_6 + s_28_5);
        // D s_28_8: bit-extract s_28_3 s_28_2 s_28_7
        let s_28_8: Bits = (Bits::new(
            ((s_28_3) >> (s_28_2)).value(),
            u16::try_from(s_28_7).unwrap(),
        ));
        // D s_28_9: cast reint s_28_8 -> u32
        let s_28_9: u32 = (s_28_8.value() as u32);
        // D s_28_10: read-var t:i
        let s_28_10: i128 = fn_state.t;
        // D s_28_11: call R_set(s_28_10, s_28_9)
        let s_28_11: () = R_set(state, tracer, s_28_10, s_28_9);
        // N s_28_12: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #424u : u32
        let s_29_0: u32 = 424;
        // D s_29_1: read-reg s_29_0:u8
        let s_29_1: u8 = {
            let value = state.read_register::<u8>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call ELUsingAArch32(s_29_1)
        let s_29_2: bool = ELUsingAArch32(state, tracer, s_29_1);
        // D s_29_3: write-var gs#112650 <= s_29_2
        fn_state.gs_112650 = s_29_2;
        // N s_29_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #3u : u8
        let s_30_0: u8 = 3;
        // C s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 8u16);
        // C s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (s_30_1.value() as i128);
        // C s_30_3: cast reint s_30_2 -> i64
        let s_30_3: i64 = (s_30_2 as i64);
        // C s_30_4: cast zx s_30_3 -> i
        let s_30_4: i128 = (i128::try_from(s_30_3).unwrap());
        // S s_30_5: call AArch32_TakeHypTrapException(s_30_4)
        let s_30_5: () = AArch32_TakeHypTrapException(state, tracer, s_30_4);
        // N s_30_6: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var __HSTR_T7:u8
        let s_31_0: bool = fn_state.u__HSTR_T7;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#112649 <= s_31_4
        fn_state.gs_112649 = s_31_4;
        // N s_31_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #432u : u32
        let s_32_0: u32 = 432;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: call ELUsingAArch32(s_32_1)
        let s_32_2: bool = ELUsingAArch32(state, tracer, s_32_1);
        // D s_32_3: write-var gs#112648 <= s_32_2
        fn_state.gs_112648 = s_32_2;
        // N s_32_4: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #3u : u8
        let s_33_0: u8 = 3;
        // C s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 8u16);
        // C s_33_2: cast zx s_33_1 -> i
        let s_33_2: i128 = (s_33_1.value() as i128);
        // C s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: cast zx s_33_3 -> i
        let s_33_4: i128 = (i128::try_from(s_33_3).unwrap());
        // C s_33_5: const #432u : u32
        let s_33_5: u32 = 432;
        // D s_33_6: read-reg s_33_5:u8
        let s_33_6: u8 = {
            let value = state.read_register::<u8>(s_33_5 as isize);
            tracer.read_register(s_33_5 as isize, value);
            value
        };
        // D s_33_7: call AArch64_AArch32SystemAccessTrap(s_33_6, s_33_4)
        let s_33_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_33_6, s_33_4);
        // N s_33_8: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var __HSTR_EL2_T7:u8
        let s_34_0: bool = fn_state.u__HSTR_EL2_T7;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: write-var gs#112647 <= s_34_4
        fn_state.gs_112647 = s_34_4;
        // N s_34_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #432u : u32
        let s_35_0: u32 = 432;
        // D s_35_1: read-reg s_35_0:u8
        let s_35_1: u8 = {
            let value = state.read_register::<u8>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call ELUsingAArch32(s_35_1)
        let s_35_2: bool = ELUsingAArch32(state, tracer, s_35_1);
        // D s_35_3: not s_35_2
        let s_35_3: bool = !s_35_2;
        // D s_35_4: write-var gs#112646 <= s_35_3
        fn_state.gs_112646 = s_35_3;
        // N s_35_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_36_0: panic
        panic!("{:?}", ());
        // N s_36_1: return
        return;
    }
}
