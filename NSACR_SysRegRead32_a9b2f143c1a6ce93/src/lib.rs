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
use u_get_SCR_EL3_Type_EEL2::*;
use AArch32_TakeHypTrapException::*;
use u__get_NSACR::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_HSTR_EL2_Type_T1::*;
use u_get_HSTR_Type_T1::*;
use Zeros::*;
use EL2Enabled::*;
use u_get_SCR_EL3_Type_NS::*;
use HSTR_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use common::*;
pub fn NSACR_SysRegRead32_a9b2f143c1a6ce93<T: Tracer>(
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
        gs_112624: bool,
        u__SCR_EL3_NS: bool,
        ga_184285: ProductType700c18a878c5601b,
        gs_112631: bool,
        gs_112623: bool,
        gs_112628: bool,
        u__HSTR_EL2_T1: bool,
        ga_184273: ProductType700c18a878c5601b,
        gs_112632: bool,
        gs_112633: bool,
        u__HSTR_T1: bool,
        gs_112627: bool,
        ga_184288: ProductType700c18a878c5601b,
        gs_112630: bool,
        gs_112629: bool,
        gs_112634: bool,
        u__PSTATE_EL: u8,
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
        // D s_0_5: call _get_HSTR_EL2_Type_T1(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T1(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T1 <= s_0_5
        fn_state.u__HSTR_EL2_T1 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T1(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T1(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T1 <= s_0_9
        fn_state.u__HSTR_T1 = s_0_9;
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
        // N s_0_21: branch s_0_20 b50 b1
        if s_0_20 {
            return block_50(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b15 b2
        if s_1_5 {
            return block_15(state, tracer, fn_state);
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
        // C s_5_0: const #102488u : u32
        let s_5_0: u32 = 102488;
        // D s_5_1: read-reg s_5_0:struct
        let s_5_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call __get_NSACR(s_5_1)
        let s_5_2: ProductType700c18a878c5601b = u__get_NSACR(state, tracer, s_5_1);
        // D s_5_3: write-var ga#184288 <= s_5_2
        fn_state.ga_184288 = s_5_2;
        // D s_5_4: read-var ga#184288.0:struct
        let s_5_4: u32 = fn_state.ga_184288._0;
        // D s_5_5: read-var t:i
        let s_5_5: i128 = fn_state.t;
        // D s_5_6: call R_set(s_5_5, s_5_4)
        let s_5_6: () = R_set(state, tracer, s_5_5, s_5_4);
        // N s_5_7: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #424u : u32
        let s_6_0: u32 = 424;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #2u : u8
        let s_6_2: u8 = 2;
        // D s_6_3: cmp-lt s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) < (s_6_2));
        // D s_6_4: not s_6_3
        let s_6_4: bool = !s_6_3;
        // N s_6_5: branch s_6_4 b14 b7
        if s_6_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #424u : u32
        let s_7_0: u32 = 424;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call ELUsingAArch32(s_7_1)
        let s_7_2: bool = ELUsingAArch32(state, tracer, s_7_1);
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b13 b8
        if s_7_3 {
            return block_13(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#112623 <= s_8_0
        fn_state.gs_112623 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#112623:u8
        let s_9_0: bool = fn_state.gs_112623;
        // D s_9_1: write-var gs#112624 <= s_9_0
        fn_state.gs_112624 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#112624:u8
        let s_10_0: bool = fn_state.gs_112624;
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
        // C s_11_0: const #102488u : u32
        let s_11_0: u32 = 102488;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call __get_NSACR(s_11_1)
        let s_11_2: ProductType700c18a878c5601b = u__get_NSACR(state, tracer, s_11_1);
        // D s_11_3: write-var ga#184285 <= s_11_2
        fn_state.ga_184285 = s_11_2;
        // D s_11_4: read-var ga#184285.0:struct
        let s_11_4: u32 = fn_state.ga_184285._0;
        // D s_11_5: read-var t:i
        let s_11_5: i128 = fn_state.t;
        // D s_11_6: call R_set(s_11_5, s_11_4)
        let s_11_6: () = R_set(state, tracer, s_11_5, s_11_4);
        // N s_11_7: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #20s : i
        let s_12_0: i128 = 20;
        // S s_12_1: call Zeros(s_12_0)
        let s_12_1: Bits = Zeros(state, tracer, s_12_0);
        // S s_12_2: cast reint s_12_1 -> u20
        let s_12_2: u32 = (s_12_1.value() as u32);
        // S s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 20u16);
        // C s_12_4: const #12u : u8
        let s_12_4: u8 = 12;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 4u16);
        // S s_12_6: cast reint s_12_3 -> u128
        let s_12_6: u128 = (s_12_3.value() as u128);
        // D s_12_7: size-of s_12_3
        let s_12_7: u16 = s_12_3.length();
        // C s_12_8: cast reint s_12_5 -> u128
        let s_12_8: u128 = (s_12_5.value() as u128);
        // D s_12_9: size-of s_12_5
        let s_12_9: u16 = s_12_5.length();
        // D s_12_10: lsl s_12_6 s_12_9
        let s_12_10: u128 = s_12_6 << s_12_9;
        // D s_12_11: or s_12_10 s_12_8
        let s_12_11: u128 = ((s_12_10) | (s_12_8));
        // D s_12_12: add s_12_7 s_12_9
        let s_12_12: u16 = (s_12_7 + s_12_9);
        // D s_12_13: create-bits s_12_11 s_12_12
        let s_12_13: Bits = Bits::new(s_12_11, s_12_12);
        // D s_12_14: cast reint s_12_13 -> u24
        let s_12_14: u32 = (s_12_13.value() as u32);
        // C s_12_15: const #8s : i
        let s_12_15: i128 = 8;
        // S s_12_16: call Zeros(s_12_15)
        let s_12_16: Bits = Zeros(state, tracer, s_12_15);
        // S s_12_17: cast reint s_12_16 -> u8
        let s_12_17: u8 = (s_12_16.value() as u8);
        // D s_12_18: cast zx s_12_14 -> bv
        let s_12_18: Bits = Bits::new(s_12_14 as u128, 24u16);
        // S s_12_19: cast zx s_12_17 -> bv
        let s_12_19: Bits = Bits::new(s_12_17 as u128, 8u16);
        // D s_12_20: cast reint s_12_18 -> u128
        let s_12_20: u128 = (s_12_18.value() as u128);
        // D s_12_21: size-of s_12_18
        let s_12_21: u16 = s_12_18.length();
        // S s_12_22: cast reint s_12_19 -> u128
        let s_12_22: u128 = (s_12_19.value() as u128);
        // D s_12_23: size-of s_12_19
        let s_12_23: u16 = s_12_19.length();
        // D s_12_24: lsl s_12_20 s_12_23
        let s_12_24: u128 = s_12_20 << s_12_23;
        // D s_12_25: or s_12_24 s_12_22
        let s_12_25: u128 = ((s_12_24) | (s_12_22));
        // D s_12_26: add s_12_21 s_12_23
        let s_12_26: u16 = (s_12_21 + s_12_23);
        // D s_12_27: create-bits s_12_25 s_12_26
        let s_12_27: Bits = Bits::new(s_12_25, s_12_26);
        // D s_12_28: cast reint s_12_27 -> u32
        let s_12_28: u32 = (s_12_27.value() as u32);
        // D s_12_29: read-var t:i
        let s_12_29: i128 = fn_state.t;
        // D s_12_30: call R_set(s_12_29, s_12_28)
        let s_12_30: () = R_set(state, tracer, s_12_29, s_12_28);
        // N s_12_31: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var __SCR_EL3_NS:u8
        let s_13_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #1u : u8
        let s_13_2: bool = true;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: write-var gs#112623 <= s_13_4
        fn_state.gs_112623 = s_13_4;
        // N s_13_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#112624 <= s_14_0
        fn_state.gs_112624 = s_14_0;
        // N s_14_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call EL2Enabled(s_15_0)
        let s_15_1: bool = EL2Enabled(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b49 b16
        if s_15_1 {
            return block_49(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#112627 <= s_16_0
        fn_state.gs_112627 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#112627:u8
        let s_17_0: bool = fn_state.gs_112627;
        // N s_17_1: branch s_17_0 b48 b18
        if s_17_0 {
            return block_48(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#112628 <= s_18_0
        fn_state.gs_112628 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#112628:u8
        let s_19_0: bool = fn_state.gs_112628;
        // N s_19_1: branch s_19_0 b47 b20
        if s_19_0 {
            return block_47(state, tracer, fn_state);
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
        // S s_20_1: call EL2Enabled(s_20_0)
        let s_20_1: bool = EL2Enabled(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b46 b21
        if s_20_1 {
            return block_46(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#112629 <= s_21_0
        fn_state.gs_112629 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#112629:u8
        let s_22_0: bool = fn_state.gs_112629;
        // N s_22_1: branch s_22_0 b45 b23
        if s_22_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#112630 <= s_23_0
        fn_state.gs_112630 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#112630:u8
        let s_24_0: bool = fn_state.gs_112630;
        // N s_24_1: branch s_24_0 b44 b25
        if s_24_0 {
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
        // C s_25_0: const #432u : u32
        let s_25_0: u32 = 432;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call ELUsingAArch32(s_25_1)
        let s_25_2: bool = ELUsingAArch32(state, tracer, s_25_1);
        // D s_25_3: not s_25_2
        let s_25_3: bool = !s_25_2;
        // N s_25_4: branch s_25_3 b43 b26
        if s_25_3 {
            return block_43(state, tracer, fn_state);
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
        // D s_26_1: write-var gs#112631 <= s_26_0
        fn_state.gs_112631 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#112631:u8
        let s_27_0: bool = fn_state.gs_112631;
        // N s_27_1: branch s_27_0 b42 b28
        if s_27_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #424u : u32
        let s_28_0: u32 = 424;
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
        // N s_28_4: branch s_28_3 b41 b29
        if s_28_3 {
            return block_41(state, tracer, fn_state);
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
        // D s_29_1: write-var gs#112632 <= s_29_0
        fn_state.gs_112632 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#112632:u8
        let s_30_0: bool = fn_state.gs_112632;
        // N s_30_1: branch s_30_0 b40 b31
        if s_30_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
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
        // D s_31_4: not s_31_3
        let s_31_4: bool = !s_31_3;
        // N s_31_5: branch s_31_4 b39 b32
        if s_31_4 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #424u : u32
        let s_32_0: u32 = 424;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: call ELUsingAArch32(s_32_1)
        let s_32_2: bool = ELUsingAArch32(state, tracer, s_32_1);
        // D s_32_3: not s_32_2
        let s_32_3: bool = !s_32_2;
        // N s_32_4: branch s_32_3 b38 b33
        if s_32_3 {
            return block_38(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#112633 <= s_33_0
        fn_state.gs_112633 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#112633:u8
        let s_34_0: bool = fn_state.gs_112633;
        // D s_34_1: write-var gs#112634 <= s_34_0
        fn_state.gs_112634 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#112634:u8
        let s_35_0: bool = fn_state.gs_112634;
        // N s_35_1: branch s_35_0 b37 b36
        if s_35_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #102488u : u32
        let s_36_0: u32 = 102488;
        // D s_36_1: read-reg s_36_0:struct
        let s_36_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // D s_36_2: call __get_NSACR(s_36_1)
        let s_36_2: ProductType700c18a878c5601b = u__get_NSACR(state, tracer, s_36_1);
        // D s_36_3: write-var ga#184273 <= s_36_2
        fn_state.ga_184273 = s_36_2;
        // D s_36_4: read-var ga#184273.0:struct
        let s_36_4: u32 = fn_state.ga_184273._0;
        // D s_36_5: read-var t:i
        let s_36_5: i128 = fn_state.t;
        // D s_36_6: call R_set(s_36_5, s_36_4)
        let s_36_6: () = R_set(state, tracer, s_36_5, s_36_4);
        // N s_36_7: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #20s : i
        let s_37_0: i128 = 20;
        // S s_37_1: call Zeros(s_37_0)
        let s_37_1: Bits = Zeros(state, tracer, s_37_0);
        // S s_37_2: cast reint s_37_1 -> u20
        let s_37_2: u32 = (s_37_1.value() as u32);
        // S s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 20u16);
        // C s_37_4: const #12u : u8
        let s_37_4: u8 = 12;
        // C s_37_5: cast zx s_37_4 -> bv
        let s_37_5: Bits = Bits::new(s_37_4 as u128, 4u16);
        // S s_37_6: cast reint s_37_3 -> u128
        let s_37_6: u128 = (s_37_3.value() as u128);
        // D s_37_7: size-of s_37_3
        let s_37_7: u16 = s_37_3.length();
        // C s_37_8: cast reint s_37_5 -> u128
        let s_37_8: u128 = (s_37_5.value() as u128);
        // D s_37_9: size-of s_37_5
        let s_37_9: u16 = s_37_5.length();
        // D s_37_10: lsl s_37_6 s_37_9
        let s_37_10: u128 = s_37_6 << s_37_9;
        // D s_37_11: or s_37_10 s_37_8
        let s_37_11: u128 = ((s_37_10) | (s_37_8));
        // D s_37_12: add s_37_7 s_37_9
        let s_37_12: u16 = (s_37_7 + s_37_9);
        // D s_37_13: create-bits s_37_11 s_37_12
        let s_37_13: Bits = Bits::new(s_37_11, s_37_12);
        // D s_37_14: cast reint s_37_13 -> u24
        let s_37_14: u32 = (s_37_13.value() as u32);
        // C s_37_15: const #8s : i
        let s_37_15: i128 = 8;
        // S s_37_16: call Zeros(s_37_15)
        let s_37_16: Bits = Zeros(state, tracer, s_37_15);
        // S s_37_17: cast reint s_37_16 -> u8
        let s_37_17: u8 = (s_37_16.value() as u8);
        // D s_37_18: cast zx s_37_14 -> bv
        let s_37_18: Bits = Bits::new(s_37_14 as u128, 24u16);
        // S s_37_19: cast zx s_37_17 -> bv
        let s_37_19: Bits = Bits::new(s_37_17 as u128, 8u16);
        // D s_37_20: cast reint s_37_18 -> u128
        let s_37_20: u128 = (s_37_18.value() as u128);
        // D s_37_21: size-of s_37_18
        let s_37_21: u16 = s_37_18.length();
        // S s_37_22: cast reint s_37_19 -> u128
        let s_37_22: u128 = (s_37_19.value() as u128);
        // D s_37_23: size-of s_37_19
        let s_37_23: u16 = s_37_19.length();
        // D s_37_24: lsl s_37_20 s_37_23
        let s_37_24: u128 = s_37_20 << s_37_23;
        // D s_37_25: or s_37_24 s_37_22
        let s_37_25: u128 = ((s_37_24) | (s_37_22));
        // D s_37_26: add s_37_21 s_37_23
        let s_37_26: u16 = (s_37_21 + s_37_23);
        // D s_37_27: create-bits s_37_25 s_37_26
        let s_37_27: Bits = Bits::new(s_37_25, s_37_26);
        // D s_37_28: cast reint s_37_27 -> u32
        let s_37_28: u32 = (s_37_27.value() as u32);
        // D s_37_29: read-var t:i
        let s_37_29: i128 = fn_state.t;
        // D s_37_30: call R_set(s_37_29, s_37_28)
        let s_37_30: () = R_set(state, tracer, s_37_29, s_37_28);
        // N s_37_31: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var __SCR_EL3_NS:u8
        let s_38_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 1u16);
        // C s_38_2: const #1u : u8
        let s_38_2: bool = true;
        // C s_38_3: cast zx s_38_2 -> bv
        let s_38_3: Bits = Bits::new(s_38_2 as u128, 1u16);
        // D s_38_4: cmp-eq s_38_1 s_38_3
        let s_38_4: bool = ((s_38_1) == (s_38_3));
        // D s_38_5: write-var gs#112633 <= s_38_4
        fn_state.gs_112633 = s_38_4;
        // N s_38_6: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#112634 <= s_39_0
        fn_state.gs_112634 = s_39_0;
        // N s_39_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #3u : u8
        let s_40_0: u8 = 3;
        // C s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 8u16);
        // C s_40_2: cast zx s_40_1 -> i
        let s_40_2: i128 = (s_40_1.value() as i128);
        // C s_40_3: cast reint s_40_2 -> i64
        let s_40_3: i64 = (s_40_2 as i64);
        // C s_40_4: cast zx s_40_3 -> i
        let s_40_4: i128 = (i128::try_from(s_40_3).unwrap());
        // C s_40_5: const #424u : u32
        let s_40_5: u32 = 424;
        // D s_40_6: read-reg s_40_5:u8
        let s_40_6: u8 = {
            let value = state.read_register::<u8>(s_40_5 as isize);
            tracer.read_register(s_40_5 as isize, value);
            value
        };
        // D s_40_7: call AArch64_AArch32SystemAccessTrap(s_40_6, s_40_4)
        let s_40_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_40_6, s_40_4);
        // N s_40_8: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var __SCR_EL3_NS:u8
        let s_41_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 1u16);
        // C s_41_2: const #0u : u8
        let s_41_2: bool = false;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: write-var gs#112632 <= s_41_4
        fn_state.gs_112632 = s_41_4;
        // N s_41_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #3u : u8
        let s_42_0: u8 = 3;
        // C s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 8u16);
        // C s_42_2: cast zx s_42_1 -> i
        let s_42_2: i128 = (s_42_1.value() as i128);
        // C s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // C s_42_4: cast zx s_42_3 -> i
        let s_42_4: i128 = (i128::try_from(s_42_3).unwrap());
        // C s_42_5: const #432u : u32
        let s_42_5: u32 = 432;
        // D s_42_6: read-reg s_42_5:u8
        let s_42_6: u8 = {
            let value = state.read_register::<u8>(s_42_5 as isize);
            tracer.read_register(s_42_5 as isize, value);
            value
        };
        // D s_42_7: call AArch64_AArch32SystemAccessTrap(s_42_6, s_42_4)
        let s_42_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_42_6, s_42_4);
        // N s_42_8: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #90704u : u32
        let s_43_0: u32 = 90704;
        // D s_43_1: read-reg s_43_0:struct
        let s_43_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call _get_SCR_EL3_Type_NS(s_43_1)
        let s_43_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_43_1);
        // C s_43_3: const #90704u : u32
        let s_43_3: u32 = 90704;
        // D s_43_4: read-reg s_43_3:struct
        let s_43_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_43_3 as isize);
            tracer.read_register(s_43_3 as isize, value);
            value
        };
        // D s_43_5: call _get_SCR_EL3_Type_EEL2(s_43_4)
        let s_43_5: bool = u_get_SCR_EL3_Type_EEL2(state, tracer, s_43_4);
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
        // C s_43_18: const #1u : u8
        let s_43_18: u8 = 1;
        // C s_43_19: cast zx s_43_18 -> bv
        let s_43_19: Bits = Bits::new(s_43_18 as u128, 2u16);
        // D s_43_20: cmp-eq s_43_17 s_43_19
        let s_43_20: bool = ((s_43_17) == (s_43_19));
        // D s_43_21: write-var gs#112631 <= s_43_20
        fn_state.gs_112631 = s_43_20;
        // N s_43_22: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #3u : u8
        let s_44_0: u8 = 3;
        // C s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 8u16);
        // C s_44_2: cast zx s_44_1 -> i
        let s_44_2: i128 = (s_44_1.value() as i128);
        // C s_44_3: cast reint s_44_2 -> i64
        let s_44_3: i64 = (s_44_2 as i64);
        // C s_44_4: cast zx s_44_3 -> i
        let s_44_4: i128 = (i128::try_from(s_44_3).unwrap());
        // S s_44_5: call AArch32_TakeHypTrapException(s_44_4)
        let s_44_5: () = AArch32_TakeHypTrapException(state, tracer, s_44_4);
        // N s_44_6: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var __HSTR_T1:u8
        let s_45_0: bool = fn_state.u__HSTR_T1;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 1u16);
        // C s_45_2: const #1u : u8
        let s_45_2: bool = true;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: write-var gs#112630 <= s_45_4
        fn_state.gs_112630 = s_45_4;
        // N s_45_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #432u : u32
        let s_46_0: u32 = 432;
        // D s_46_1: read-reg s_46_0:u8
        let s_46_1: u8 = {
            let value = state.read_register::<u8>(s_46_0 as isize);
            tracer.read_register(s_46_0 as isize, value);
            value
        };
        // D s_46_2: call ELUsingAArch32(s_46_1)
        let s_46_2: bool = ELUsingAArch32(state, tracer, s_46_1);
        // D s_46_3: write-var gs#112629 <= s_46_2
        fn_state.gs_112629 = s_46_2;
        // N s_46_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #3u : u8
        let s_47_0: u8 = 3;
        // C s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 8u16);
        // C s_47_2: cast zx s_47_1 -> i
        let s_47_2: i128 = (s_47_1.value() as i128);
        // C s_47_3: cast reint s_47_2 -> i64
        let s_47_3: i64 = (s_47_2 as i64);
        // C s_47_4: cast zx s_47_3 -> i
        let s_47_4: i128 = (i128::try_from(s_47_3).unwrap());
        // C s_47_5: const #432u : u32
        let s_47_5: u32 = 432;
        // D s_47_6: read-reg s_47_5:u8
        let s_47_6: u8 = {
            let value = state.read_register::<u8>(s_47_5 as isize);
            tracer.read_register(s_47_5 as isize, value);
            value
        };
        // D s_47_7: call AArch64_AArch32SystemAccessTrap(s_47_6, s_47_4)
        let s_47_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_47_6, s_47_4);
        // N s_47_8: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __HSTR_EL2_T1:u8
        let s_48_0: bool = fn_state.u__HSTR_EL2_T1;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #1u : u8
        let s_48_2: bool = true;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#112628 <= s_48_4
        fn_state.gs_112628 = s_48_4;
        // N s_48_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #432u : u32
        let s_49_0: u32 = 432;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: call ELUsingAArch32(s_49_1)
        let s_49_2: bool = ELUsingAArch32(state, tracer, s_49_1);
        // D s_49_3: not s_49_2
        let s_49_3: bool = !s_49_2;
        // D s_49_4: write-var gs#112627 <= s_49_3
        fn_state.gs_112627 = s_49_3;
        // N s_49_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: panic
        panic!("{:?}", ());
        // N s_50_1: return
        return;
    }
}
