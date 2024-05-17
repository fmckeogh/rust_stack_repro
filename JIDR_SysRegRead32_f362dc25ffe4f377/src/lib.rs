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
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HCR_EL2_Type_TID0::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HCR_EL2_Type_TGE::*;
use R_set::*;
use HCR_read::*;
use ELUsingAArch32::*;
use u__IMPDEF_boolean::*;
use EL2Enabled::*;
use u_get_HCR_Type_TID0::*;
use u__get_JIDR::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn JIDR_SysRegRead32_f362dc25ffe4f377<T: Tracer>(
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
        gs_112592: bool,
        gs_112597: bool,
        u__HCR_EL2_TID0: bool,
        gs_112598: bool,
        gs_112596: bool,
        gs_112591: bool,
        u__HCR_TID0: bool,
        gs_112594: bool,
        gs_112593: bool,
        u__PSTATE_EL: u8,
        gs_112595: bool,
        gs_112590: bool,
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
        // C s_0_3: const #102552u : u32
        let s_0_3: u32 = 102552;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HCR_EL2_Type_TID0(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_TID0(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_TID0 <= s_0_5
        fn_state.u__HCR_EL2_TID0 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HCR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HCR_Type_TID0(s_0_8)
        let s_0_9: bool = u_get_HCR_Type_TID0(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_TID0 <= s_0_9
        fn_state.u__HCR_TID0 = s_0_9;
        // D s_0_11: read-var __PSTATE_EL:u8
        let s_0_11: u8 = fn_state.u__PSTATE_EL;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_13: const #448u : u32
        let s_0_13: u32 = 448;
        // D s_0_14: read-reg s_0_13:u8
        let s_0_14: u8 = {
            let value = state.read_register::<u8>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cmp-eq s_0_12 s_0_15
        let s_0_16: bool = ((s_0_12) == (s_0_15));
        // N s_0_17: branch s_0_16 b24 b1
        if s_0_16 {
            return block_24(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b7 b2
        if s_1_5 {
            return block_7(state, tracer, fn_state);
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
        // C s_5_0: const #101872u : u32
        let s_5_0: u32 = 101872;
        // D s_5_1: read-reg s_5_0:u32
        let s_5_1: u32 = {
            let value = state.read_register::<u32>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call __get_JIDR(s_5_1)
        let s_5_2: u32 = u__get_JIDR(state, tracer, s_5_1);
        // D s_5_3: read-var t:i
        let s_5_3: i128 = fn_state.t;
        // D s_5_4: call R_set(s_5_3, s_5_2)
        let s_5_4: () = R_set(state, tracer, s_5_3, s_5_2);
        // N s_5_5: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #101872u : u32
        let s_6_0: u32 = 101872;
        // D s_6_1: read-reg s_6_0:u32
        let s_6_1: u32 = {
            let value = state.read_register::<u32>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call __get_JIDR(s_6_1)
        let s_6_2: u32 = u__get_JIDR(state, tracer, s_6_1);
        // D s_6_3: read-var t:i
        let s_6_3: i128 = fn_state.t;
        // D s_6_4: call R_set(s_6_3, s_6_2)
        let s_6_4: () = R_set(state, tracer, s_6_3, s_6_2);
        // N s_6_5: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call EL2Enabled(s_7_0)
        let s_7_1: bool = EL2Enabled(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b23 b8
        if s_7_1 {
            return block_23(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#112590 <= s_8_0
        fn_state.gs_112590 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#112590:u8
        let s_9_0: bool = fn_state.gs_112590;
        // N s_9_1: branch s_9_0 b22 b10
        if s_9_0 {
            return block_22(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#112591 <= s_10_0
        fn_state.gs_112591 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#112591:u8
        let s_11_0: bool = fn_state.gs_112591;
        // N s_11_1: branch s_11_0 b21 b12
        if s_11_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call EL2Enabled(s_12_0)
        let s_12_1: bool = EL2Enabled(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b20 b13
        if s_12_1 {
            return block_20(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#112592 <= s_13_0
        fn_state.gs_112592 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#112592:u8
        let s_14_0: bool = fn_state.gs_112592;
        // N s_14_1: branch s_14_0 b19 b15
        if s_14_0 {
            return block_19(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#112593 <= s_15_0
        fn_state.gs_112593 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#112593:u8
        let s_16_0: bool = fn_state.gs_112593;
        // N s_16_1: branch s_16_0 b18 b17
        if s_16_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #101872u : u32
        let s_17_0: u32 = 101872;
        // D s_17_1: read-reg s_17_0:u32
        let s_17_1: u32 = {
            let value = state.read_register::<u32>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call __get_JIDR(s_17_1)
        let s_17_2: u32 = u__get_JIDR(state, tracer, s_17_1);
        // D s_17_3: read-var t:i
        let s_17_3: i128 = fn_state.t;
        // D s_17_4: call R_set(s_17_3, s_17_2)
        let s_17_4: () = R_set(state, tracer, s_17_3, s_17_2);
        // N s_17_5: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #5u : u8
        let s_18_0: u8 = 5;
        // C s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 8u16);
        // C s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (s_18_1.value() as i128);
        // C s_18_3: cast reint s_18_2 -> i64
        let s_18_3: i64 = (s_18_2 as i64);
        // C s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // S s_18_5: call AArch32_TakeHypTrapException(s_18_4)
        let s_18_5: () = AArch32_TakeHypTrapException(state, tracer, s_18_4);
        // N s_18_6: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var __HCR_TID0:u8
        let s_19_0: bool = fn_state.u__HCR_TID0;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #1u : u8
        let s_19_2: bool = true;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: write-var gs#112593 <= s_19_4
        fn_state.gs_112593 = s_19_4;
        // N s_19_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #432u : u32
        let s_20_0: u32 = 432;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: u8 = {
            let value = state.read_register::<u8>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call ELUsingAArch32(s_20_1)
        let s_20_2: bool = ELUsingAArch32(state, tracer, s_20_1);
        // D s_20_3: write-var gs#112592 <= s_20_2
        fn_state.gs_112592 = s_20_2;
        // N s_20_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #5u : u8
        let s_21_0: u8 = 5;
        // C s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 8u16);
        // C s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (s_21_1.value() as i128);
        // C s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // C s_21_5: const #432u : u32
        let s_21_5: u32 = 432;
        // D s_21_6: read-reg s_21_5:u8
        let s_21_6: u8 = {
            let value = state.read_register::<u8>(s_21_5 as isize);
            tracer.read_register(s_21_5 as isize, value);
            value
        };
        // D s_21_7: call AArch64_AArch32SystemAccessTrap(s_21_6, s_21_4)
        let s_21_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_21_6, s_21_4);
        // N s_21_8: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var __HCR_EL2_TID0:u8
        let s_22_0: bool = fn_state.u__HCR_EL2_TID0;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #1u : u8
        let s_22_2: bool = true;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: write-var gs#112591 <= s_22_4
        fn_state.gs_112591 = s_22_4;
        // N s_22_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #432u : u32
        let s_23_0: u32 = 432;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: u8 = {
            let value = state.read_register::<u8>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // D s_23_2: call ELUsingAArch32(s_23_1)
        let s_23_2: bool = ELUsingAArch32(state, tracer, s_23_1);
        // D s_23_3: not s_23_2
        let s_23_3: bool = !s_23_2;
        // D s_23_4: write-var gs#112590 <= s_23_3
        fn_state.gs_112590 = s_23_3;
        // N s_23_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #"JIDR UNDEFINED at EL0" : str
        let s_24_0: &'static str = "JIDR UNDEFINED at EL0";
        // S s_24_1: call __IMPDEF_boolean(s_24_0)
        let s_24_1: bool = u__IMPDEF_boolean(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b45 b25
        if s_24_1 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
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
        // N s_25_2: branch s_25_1 b44 b26
        if s_25_1 {
            return block_44(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#112594 <= s_26_0
        fn_state.gs_112594 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#112594:u8
        let s_27_0: bool = fn_state.gs_112594;
        // N s_27_1: branch s_27_0 b43 b28
        if s_27_0 {
            return block_43(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#112595 <= s_28_0
        fn_state.gs_112595 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#112595:u8
        let s_29_0: bool = fn_state.gs_112595;
        // N s_29_1: branch s_29_0 b42 b30
        if s_29_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#112596 <= s_30_0
        fn_state.gs_112596 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#112596:u8
        let s_31_0: bool = fn_state.gs_112596;
        // N s_31_1: branch s_31_0 b41 b32
        if s_31_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call EL2Enabled(s_32_0)
        let s_32_1: bool = EL2Enabled(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b40 b33
        if s_32_1 {
            return block_40(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#112597 <= s_33_0
        fn_state.gs_112597 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#112597:u8
        let s_34_0: bool = fn_state.gs_112597;
        // N s_34_1: branch s_34_0 b39 b35
        if s_34_0 {
            return block_39(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#112598 <= s_35_0
        fn_state.gs_112598 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#112598:u8
        let s_36_0: bool = fn_state.gs_112598;
        // N s_36_1: branch s_36_0 b38 b37
        if s_36_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #101872u : u32
        let s_37_0: u32 = 101872;
        // D s_37_1: read-reg s_37_0:u32
        let s_37_1: u32 = {
            let value = state.read_register::<u32>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call __get_JIDR(s_37_1)
        let s_37_2: u32 = u__get_JIDR(state, tracer, s_37_1);
        // D s_37_3: read-var t:i
        let s_37_3: i128 = fn_state.t;
        // D s_37_4: call R_set(s_37_3, s_37_2)
        let s_37_4: () = R_set(state, tracer, s_37_3, s_37_2);
        // N s_37_5: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #5u : u8
        let s_38_0: u8 = 5;
        // C s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 8u16);
        // C s_38_2: cast zx s_38_1 -> i
        let s_38_2: i128 = (s_38_1.value() as i128);
        // C s_38_3: cast reint s_38_2 -> i64
        let s_38_3: i64 = (s_38_2 as i64);
        // C s_38_4: cast zx s_38_3 -> i
        let s_38_4: i128 = (i128::try_from(s_38_3).unwrap());
        // S s_38_5: call AArch32_TakeHypTrapException(s_38_4)
        let s_38_5: () = AArch32_TakeHypTrapException(state, tracer, s_38_4);
        // N s_38_6: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var __HCR_TID0:u8
        let s_39_0: bool = fn_state.u__HCR_TID0;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#112598 <= s_39_4
        fn_state.gs_112598 = s_39_4;
        // N s_39_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #432u : u32
        let s_40_0: u32 = 432;
        // D s_40_1: read-reg s_40_0:u8
        let s_40_1: u8 = {
            let value = state.read_register::<u8>(s_40_0 as isize);
            tracer.read_register(s_40_0 as isize, value);
            value
        };
        // D s_40_2: call ELUsingAArch32(s_40_1)
        let s_40_2: bool = ELUsingAArch32(state, tracer, s_40_1);
        // D s_40_3: write-var gs#112597 <= s_40_2
        fn_state.gs_112597 = s_40_2;
        // N s_40_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #5u : u8
        let s_41_0: u8 = 5;
        // C s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 8u16);
        // C s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (s_41_1.value() as i128);
        // C s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: cast zx s_41_3 -> i
        let s_41_4: i128 = (i128::try_from(s_41_3).unwrap());
        // C s_41_5: const #432u : u32
        let s_41_5: u32 = 432;
        // D s_41_6: read-reg s_41_5:u8
        let s_41_6: u8 = {
            let value = state.read_register::<u8>(s_41_5 as isize);
            tracer.read_register(s_41_5 as isize, value);
            value
        };
        // D s_41_7: call AArch64_AArch32SystemAccessTrap(s_41_6, s_41_4)
        let s_41_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_41_6, s_41_4);
        // N s_41_8: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var __HCR_EL2_TID0:u8
        let s_42_0: bool = fn_state.u__HCR_EL2_TID0;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 1u16);
        // C s_42_2: const #1u : u8
        let s_42_2: bool = true;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: write-var gs#112596 <= s_42_4
        fn_state.gs_112596 = s_42_4;
        // N s_42_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #102552u : u32
        let s_43_0: u32 = 102552;
        // D s_43_1: read-reg s_43_0:struct
        let s_43_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call _get_HCR_EL2_Type_E2H(s_43_1)
        let s_43_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_43_1);
        // C s_43_3: const #102552u : u32
        let s_43_3: u32 = 102552;
        // D s_43_4: read-reg s_43_3:struct
        let s_43_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_43_3 as isize);
            tracer.read_register(s_43_3 as isize, value);
            value
        };
        // D s_43_5: call _get_HCR_EL2_Type_TGE(s_43_4)
        let s_43_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_43_4);
        // D s_43_6: cast zx s_43_2 -> bv
        let s_43_6: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_7: cast zx s_43_5 -> bv
        let s_43_7: Bits = Bits::new(s_43_5 as u128, 1u16);
        // D s_43_8: cast reint s_43_6 -> u128
        let s_43_8: u128 = (s_43_6.value() as u128);
        // D s_43_9: size-of s_43_6
        let s_43_9: u16 = s_43_6.length();
        // D s_43_10: cast reint s_43_7 -> u128
        let s_43_10: u128 = (s_43_7.value() as u128);
        // D s_43_11: size-of s_43_7
        let s_43_11: u16 = s_43_7.length();
        // D s_43_12: lsl s_43_8 s_43_11
        let s_43_12: u128 = s_43_8 << s_43_11;
        // D s_43_13: or s_43_12 s_43_10
        let s_43_13: u128 = ((s_43_12) | (s_43_10));
        // D s_43_14: add s_43_9 s_43_11
        let s_43_14: u16 = (s_43_9 + s_43_11);
        // D s_43_15: create-bits s_43_13 s_43_14
        let s_43_15: Bits = Bits::new(s_43_13, s_43_14);
        // D s_43_16: cast reint s_43_15 -> u8
        let s_43_16: u8 = (s_43_15.value() as u8);
        // D s_43_17: cast zx s_43_16 -> bv
        let s_43_17: Bits = Bits::new(s_43_16 as u128, 2u16);
        // C s_43_18: const #3u : u8
        let s_43_18: u8 = 3;
        // C s_43_19: cast zx s_43_18 -> bv
        let s_43_19: Bits = Bits::new(s_43_18 as u128, 2u16);
        // D s_43_20: cmp-ne s_43_17 s_43_19
        let s_43_20: bool = ((s_43_17) != (s_43_19));
        // D s_43_21: write-var gs#112595 <= s_43_20
        fn_state.gs_112595 = s_43_20;
        // N s_43_22: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #432u : u32
        let s_44_0: u32 = 432;
        // D s_44_1: read-reg s_44_0:u8
        let s_44_1: u8 = {
            let value = state.read_register::<u8>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call ELUsingAArch32(s_44_1)
        let s_44_2: bool = ELUsingAArch32(state, tracer, s_44_1);
        // D s_44_3: not s_44_2
        let s_44_3: bool = !s_44_2;
        // D s_44_4: write-var gs#112594 <= s_44_3
        fn_state.gs_112594 = s_44_3;
        // N s_44_5: jump b27
        return block_27(state, tracer, fn_state);
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
}
