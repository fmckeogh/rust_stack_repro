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
use u_get_HSTR_Type_T2::*;
use HCR_read::*;
use Mk_TTBR1_Type::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_SCR_Type_NS::*;
use HSTR_read::*;
use TTBR1_write::*;
use R_read::*;
use u_get_HCR_EL2_Type_TVM::*;
use ELUsingAArch32::*;
use u_get_HSTR_EL2_Type_T2::*;
use u_get_HCR_Type_TVM::*;
use EL2Enabled::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn TTBR1_SysRegWrite64_7d88e184b6073f76<T: Tracer>(
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
        gs_105053: bool,
        gs_105056: bool,
        gs_105049: bool,
        gs_105051: bool,
        u__SCR_NS: bool,
        u__HCR_EL2_TVM: bool,
        gs_105057: bool,
        u__HSTR_T2: bool,
        gs_105048: bool,
        u__HSTR_EL2_T2: bool,
        gs_105047: bool,
        gs_105050: bool,
        u__PSTATE_EL: u8,
        gs_105054: bool,
        gs_105052: bool,
        gs_105055: bool,
        u__HCR_TVM: bool,
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
        // C s_0_3: const #104936u : u32
        let s_0_3: u32 = 104936;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HSTR_EL2_Type_T2(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T2(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T2 <= s_0_5
        fn_state.u__HSTR_EL2_T2 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T2(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T2(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T2 <= s_0_9
        fn_state.u__HSTR_T2 = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TVM(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TVM(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TVM <= s_0_13
        fn_state.u__HCR_EL2_TVM = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HCR_Type_TVM(s_0_16)
        let s_0_17: bool = u_get_HCR_Type_TVM(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_TVM <= s_0_17
        fn_state.u__HCR_TVM = s_0_17;
        // C s_0_19: const #20920u : u32
        let s_0_19: u32 = 20920;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_SCR_Type_NS(s_0_20)
        let s_0_21: bool = u_get_SCR_Type_NS(state, tracer, s_0_20);
        // D s_0_22: write-var __SCR_NS <= s_0_21
        fn_state.u__SCR_NS = s_0_21;
        // D s_0_23: read-var __PSTATE_EL:u8
        let s_0_23: u8 = fn_state.u__PSTATE_EL;
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 2u16);
        // C s_0_25: const #448u : u32
        let s_0_25: u32 = 448;
        // D s_0_26: read-reg s_0_25:u8
        let s_0_26: u8 = {
            let value = state.read_register::<u8>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 2u16);
        // D s_0_28: cmp-eq s_0_24 s_0_27
        let s_0_28: bool = ((s_0_24) == (s_0_27));
        // N s_0_29: branch s_0_28 b57 b1
        if s_0_28 {
            return block_57(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b19 b2
        if s_1_5 {
            return block_19(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b13 b3
        if s_2_5 {
            return block_13(state, tracer, fn_state);
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
        // N s_5_5: branch s_5_4 b12 b6
        if s_5_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#105047 <= s_6_0
        fn_state.gs_105047 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#105047:u8
        let s_7_0: bool = fn_state.gs_105047;
        // N s_7_1: branch s_7_0 b11 b8
        if s_7_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var __SCR_NS:u8
        let s_8_0: bool = fn_state.u__SCR_NS;
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
        // D s_9_0: read-var t2:i
        let s_9_0: i128 = fn_state.t2;
        // D s_9_1: call R_read(s_9_0)
        let s_9_1: u32 = R_read(state, tracer, s_9_0);
        // D s_9_2: read-var t:i
        let s_9_2: i128 = fn_state.t;
        // D s_9_3: call R_read(s_9_2)
        let s_9_3: u32 = R_read(state, tracer, s_9_2);
        // D s_9_4: cast zx s_9_1 -> bv
        let s_9_4: Bits = Bits::new(s_9_1 as u128, 32u16);
        // D s_9_5: cast zx s_9_3 -> bv
        let s_9_5: Bits = Bits::new(s_9_3 as u128, 32u16);
        // D s_9_6: cast reint s_9_4 -> u128
        let s_9_6: u128 = (s_9_4.value() as u128);
        // D s_9_7: size-of s_9_4
        let s_9_7: u16 = s_9_4.length();
        // D s_9_8: cast reint s_9_5 -> u128
        let s_9_8: u128 = (s_9_5.value() as u128);
        // D s_9_9: size-of s_9_5
        let s_9_9: u16 = s_9_5.length();
        // D s_9_10: lsl s_9_6 s_9_9
        let s_9_10: u128 = s_9_6 << s_9_9;
        // D s_9_11: or s_9_10 s_9_8
        let s_9_11: u128 = ((s_9_10) | (s_9_8));
        // D s_9_12: add s_9_7 s_9_9
        let s_9_12: u16 = (s_9_7 + s_9_9);
        // D s_9_13: create-bits s_9_11 s_9_12
        let s_9_13: Bits = Bits::new(s_9_11, s_9_12);
        // D s_9_14: cast reint s_9_13 -> u64
        let s_9_14: u64 = (s_9_13.value() as u64);
        // D s_9_15: call Mk_TTBR1_Type(s_9_14)
        let s_9_15: ProductType5c790c8ef59cc8b2 = Mk_TTBR1_Type(state, tracer, s_9_14);
        // C s_9_16: const #20024u : u32
        let s_9_16: u32 = 20024;
        // N s_9_17: write-reg s_9_16 <= s_9_15
        let s_9_17: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_9_16 as isize, s_9_15);
            tracer.write_register(s_9_16 as isize, s_9_15);
        };
        // N s_9_18: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var t2:i
        let s_10_0: i128 = fn_state.t2;
        // D s_10_1: call R_read(s_10_0)
        let s_10_1: u32 = R_read(state, tracer, s_10_0);
        // D s_10_2: read-var t:i
        let s_10_2: i128 = fn_state.t;
        // D s_10_3: call R_read(s_10_2)
        let s_10_3: u32 = R_read(state, tracer, s_10_2);
        // D s_10_4: cast zx s_10_1 -> bv
        let s_10_4: Bits = Bits::new(s_10_1 as u128, 32u16);
        // D s_10_5: cast zx s_10_3 -> bv
        let s_10_5: Bits = Bits::new(s_10_3 as u128, 32u16);
        // D s_10_6: cast reint s_10_4 -> u128
        let s_10_6: u128 = (s_10_4.value() as u128);
        // D s_10_7: size-of s_10_4
        let s_10_7: u16 = s_10_4.length();
        // D s_10_8: cast reint s_10_5 -> u128
        let s_10_8: u128 = (s_10_5.value() as u128);
        // D s_10_9: size-of s_10_5
        let s_10_9: u16 = s_10_5.length();
        // D s_10_10: lsl s_10_6 s_10_9
        let s_10_10: u128 = s_10_6 << s_10_9;
        // D s_10_11: or s_10_10 s_10_8
        let s_10_11: u128 = ((s_10_10) | (s_10_8));
        // D s_10_12: add s_10_7 s_10_9
        let s_10_12: u16 = (s_10_7 + s_10_9);
        // D s_10_13: create-bits s_10_11 s_10_12
        let s_10_13: Bits = Bits::new(s_10_11, s_10_12);
        // D s_10_14: cast reint s_10_13 -> u64
        let s_10_14: u64 = (s_10_13.value() as u64);
        // D s_10_15: call Mk_TTBR1_Type(s_10_14)
        let s_10_15: ProductType5c790c8ef59cc8b2 = Mk_TTBR1_Type(state, tracer, s_10_14);
        // C s_10_16: const #19120u : u32
        let s_10_16: u32 = 19120;
        // N s_10_17: write-reg s_10_16 <= s_10_15
        let s_10_17: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_10_16 as isize, s_10_15);
            tracer.write_register(s_10_16 as isize, s_10_15);
        };
        // N s_10_18: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: panic
        panic!("{:?}", ());
        // N s_11_1: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #20328u : u32
        let s_12_0: u32 = 20328;
        // D s_12_1: read-reg s_12_0:u32
        let s_12_1: u32 = {
            let value = state.read_register::<u32>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #1u : u32
        let s_12_2: u32 = 1;
        // D s_12_3: cmp-eq s_12_1 s_12_2
        let s_12_3: bool = ((s_12_1) == (s_12_2));
        // D s_12_4: write-var gs#105047 <= s_12_3
        fn_state.gs_105047 = s_12_3;
        // N s_12_5: jump b7
        return block_7(state, tracer, fn_state);
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
        // C s_13_2: const #2u : u8
        let s_13_2: u8 = 2;
        // D s_13_3: cmp-lt s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) < (s_13_2));
        // N s_13_4: branch s_13_3 b18 b14
        if s_13_3 {
            return block_18(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#105048 <= s_14_0
        fn_state.gs_105048 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#105048:u8
        let s_15_0: bool = fn_state.gs_105048;
        // N s_15_1: branch s_15_0 b17 b16
        if s_15_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var t2:i
        let s_16_0: i128 = fn_state.t2;
        // D s_16_1: call R_read(s_16_0)
        let s_16_1: u32 = R_read(state, tracer, s_16_0);
        // D s_16_2: read-var t:i
        let s_16_2: i128 = fn_state.t;
        // D s_16_3: call R_read(s_16_2)
        let s_16_3: u32 = R_read(state, tracer, s_16_2);
        // D s_16_4: cast zx s_16_1 -> bv
        let s_16_4: Bits = Bits::new(s_16_1 as u128, 32u16);
        // D s_16_5: cast zx s_16_3 -> bv
        let s_16_5: Bits = Bits::new(s_16_3 as u128, 32u16);
        // D s_16_6: cast reint s_16_4 -> u128
        let s_16_6: u128 = (s_16_4.value() as u128);
        // D s_16_7: size-of s_16_4
        let s_16_7: u16 = s_16_4.length();
        // D s_16_8: cast reint s_16_5 -> u128
        let s_16_8: u128 = (s_16_5.value() as u128);
        // D s_16_9: size-of s_16_5
        let s_16_9: u16 = s_16_5.length();
        // D s_16_10: lsl s_16_6 s_16_9
        let s_16_10: u128 = s_16_6 << s_16_9;
        // D s_16_11: or s_16_10 s_16_8
        let s_16_11: u128 = ((s_16_10) | (s_16_8));
        // D s_16_12: add s_16_7 s_16_9
        let s_16_12: u16 = (s_16_7 + s_16_9);
        // D s_16_13: create-bits s_16_11 s_16_12
        let s_16_13: Bits = Bits::new(s_16_11, s_16_12);
        // D s_16_14: cast reint s_16_13 -> u64
        let s_16_14: u64 = (s_16_13.value() as u64);
        // D s_16_15: call Mk_TTBR1_Type(s_16_14)
        let s_16_15: ProductType5c790c8ef59cc8b2 = Mk_TTBR1_Type(state, tracer, s_16_14);
        // D s_16_16: call TTBR1_write(s_16_15)
        let s_16_16: () = TTBR1_write(state, tracer, s_16_15);
        // N s_16_17: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var t2:i
        let s_17_0: i128 = fn_state.t2;
        // D s_17_1: call R_read(s_17_0)
        let s_17_1: u32 = R_read(state, tracer, s_17_0);
        // D s_17_2: read-var t:i
        let s_17_2: i128 = fn_state.t;
        // D s_17_3: call R_read(s_17_2)
        let s_17_3: u32 = R_read(state, tracer, s_17_2);
        // D s_17_4: cast zx s_17_1 -> bv
        let s_17_4: Bits = Bits::new(s_17_1 as u128, 32u16);
        // D s_17_5: cast zx s_17_3 -> bv
        let s_17_5: Bits = Bits::new(s_17_3 as u128, 32u16);
        // D s_17_6: cast reint s_17_4 -> u128
        let s_17_6: u128 = (s_17_4.value() as u128);
        // D s_17_7: size-of s_17_4
        let s_17_7: u16 = s_17_4.length();
        // D s_17_8: cast reint s_17_5 -> u128
        let s_17_8: u128 = (s_17_5.value() as u128);
        // D s_17_9: size-of s_17_5
        let s_17_9: u16 = s_17_5.length();
        // D s_17_10: lsl s_17_6 s_17_9
        let s_17_10: u128 = s_17_6 << s_17_9;
        // D s_17_11: or s_17_10 s_17_8
        let s_17_11: u128 = ((s_17_10) | (s_17_8));
        // D s_17_12: add s_17_7 s_17_9
        let s_17_12: u16 = (s_17_7 + s_17_9);
        // D s_17_13: create-bits s_17_11 s_17_12
        let s_17_13: Bits = Bits::new(s_17_11, s_17_12);
        // D s_17_14: cast reint s_17_13 -> u64
        let s_17_14: u64 = (s_17_13.value() as u64);
        // D s_17_15: call Mk_TTBR1_Type(s_17_14)
        let s_17_15: ProductType5c790c8ef59cc8b2 = Mk_TTBR1_Type(state, tracer, s_17_14);
        // C s_17_16: const #20024u : u32
        let s_17_16: u32 = 20024;
        // N s_17_17: write-reg s_17_16 <= s_17_15
        let s_17_17: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_17_16 as isize, s_17_15);
            tracer.write_register(s_17_16 as isize, s_17_15);
        };
        // N s_17_18: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #424u : u32
        let s_18_0: u32 = 424;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: call ELUsingAArch32(s_18_1)
        let s_18_2: bool = ELUsingAArch32(state, tracer, s_18_1);
        // D s_18_3: write-var gs#105048 <= s_18_2
        fn_state.gs_105048 = s_18_2;
        // N s_18_4: jump b15
        return block_15(state, tracer, fn_state);
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
        // N s_19_2: branch s_19_1 b56 b20
        if s_19_1 {
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
        // D s_20_1: write-var gs#105049 <= s_20_0
        fn_state.gs_105049 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#105049:u8
        let s_21_0: bool = fn_state.gs_105049;
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
        // D s_22_1: write-var gs#105050 <= s_22_0
        fn_state.gs_105050 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#105050:u8
        let s_23_0: bool = fn_state.gs_105050;
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
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call EL2Enabled(s_24_0)
        let s_24_1: bool = EL2Enabled(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b53 b25
        if s_24_1 {
            return block_53(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#105051 <= s_25_0
        fn_state.gs_105051 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#105051:u8
        let s_26_0: bool = fn_state.gs_105051;
        // N s_26_1: branch s_26_0 b52 b27
        if s_26_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#105052 <= s_27_0
        fn_state.gs_105052 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#105052:u8
        let s_28_0: bool = fn_state.gs_105052;
        // N s_28_1: branch s_28_0 b51 b29
        if s_28_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call EL2Enabled(s_29_0)
        let s_29_1: bool = EL2Enabled(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b50 b30
        if s_29_1 {
            return block_50(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#105053 <= s_30_0
        fn_state.gs_105053 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#105053:u8
        let s_31_0: bool = fn_state.gs_105053;
        // N s_31_1: branch s_31_0 b49 b32
        if s_31_0 {
            return block_49(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#105054 <= s_32_0
        fn_state.gs_105054 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#105054:u8
        let s_33_0: bool = fn_state.gs_105054;
        // N s_33_1: branch s_33_0 b48 b34
        if s_33_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #() : ()
        let s_34_0: () = ();
        // S s_34_1: call EL2Enabled(s_34_0)
        let s_34_1: bool = EL2Enabled(state, tracer, s_34_0);
        // N s_34_2: branch s_34_1 b47 b35
        if s_34_1 {
            return block_47(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#105055 <= s_35_0
        fn_state.gs_105055 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#105055:u8
        let s_36_0: bool = fn_state.gs_105055;
        // N s_36_1: branch s_36_0 b46 b37
        if s_36_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#105056 <= s_37_0
        fn_state.gs_105056 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#105056:u8
        let s_38_0: bool = fn_state.gs_105056;
        // N s_38_1: branch s_38_0 b45 b39
        if s_38_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #424u : u32
        let s_39_0: u32 = 424;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: u8 = {
            let value = state.read_register::<u8>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // C s_39_2: const #2u : u8
        let s_39_2: u8 = 2;
        // D s_39_3: cmp-lt s_39_1 s_39_2
        let s_39_3: bool = ((s_39_1) < (s_39_2));
        // N s_39_4: branch s_39_3 b44 b40
        if s_39_3 {
            return block_44(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#105057 <= s_40_0
        fn_state.gs_105057 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#105057:u8
        let s_41_0: bool = fn_state.gs_105057;
        // N s_41_1: branch s_41_0 b43 b42
        if s_41_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var t2:i
        let s_42_0: i128 = fn_state.t2;
        // D s_42_1: call R_read(s_42_0)
        let s_42_1: u32 = R_read(state, tracer, s_42_0);
        // D s_42_2: read-var t:i
        let s_42_2: i128 = fn_state.t;
        // D s_42_3: call R_read(s_42_2)
        let s_42_3: u32 = R_read(state, tracer, s_42_2);
        // D s_42_4: cast zx s_42_1 -> bv
        let s_42_4: Bits = Bits::new(s_42_1 as u128, 32u16);
        // D s_42_5: cast zx s_42_3 -> bv
        let s_42_5: Bits = Bits::new(s_42_3 as u128, 32u16);
        // D s_42_6: cast reint s_42_4 -> u128
        let s_42_6: u128 = (s_42_4.value() as u128);
        // D s_42_7: size-of s_42_4
        let s_42_7: u16 = s_42_4.length();
        // D s_42_8: cast reint s_42_5 -> u128
        let s_42_8: u128 = (s_42_5.value() as u128);
        // D s_42_9: size-of s_42_5
        let s_42_9: u16 = s_42_5.length();
        // D s_42_10: lsl s_42_6 s_42_9
        let s_42_10: u128 = s_42_6 << s_42_9;
        // D s_42_11: or s_42_10 s_42_8
        let s_42_11: u128 = ((s_42_10) | (s_42_8));
        // D s_42_12: add s_42_7 s_42_9
        let s_42_12: u16 = (s_42_7 + s_42_9);
        // D s_42_13: create-bits s_42_11 s_42_12
        let s_42_13: Bits = Bits::new(s_42_11, s_42_12);
        // D s_42_14: cast reint s_42_13 -> u64
        let s_42_14: u64 = (s_42_13.value() as u64);
        // D s_42_15: call Mk_TTBR1_Type(s_42_14)
        let s_42_15: ProductType5c790c8ef59cc8b2 = Mk_TTBR1_Type(state, tracer, s_42_14);
        // D s_42_16: call TTBR1_write(s_42_15)
        let s_42_16: () = TTBR1_write(state, tracer, s_42_15);
        // N s_42_17: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var t2:i
        let s_43_0: i128 = fn_state.t2;
        // D s_43_1: call R_read(s_43_0)
        let s_43_1: u32 = R_read(state, tracer, s_43_0);
        // D s_43_2: read-var t:i
        let s_43_2: i128 = fn_state.t;
        // D s_43_3: call R_read(s_43_2)
        let s_43_3: u32 = R_read(state, tracer, s_43_2);
        // D s_43_4: cast zx s_43_1 -> bv
        let s_43_4: Bits = Bits::new(s_43_1 as u128, 32u16);
        // D s_43_5: cast zx s_43_3 -> bv
        let s_43_5: Bits = Bits::new(s_43_3 as u128, 32u16);
        // D s_43_6: cast reint s_43_4 -> u128
        let s_43_6: u128 = (s_43_4.value() as u128);
        // D s_43_7: size-of s_43_4
        let s_43_7: u16 = s_43_4.length();
        // D s_43_8: cast reint s_43_5 -> u128
        let s_43_8: u128 = (s_43_5.value() as u128);
        // D s_43_9: size-of s_43_5
        let s_43_9: u16 = s_43_5.length();
        // D s_43_10: lsl s_43_6 s_43_9
        let s_43_10: u128 = s_43_6 << s_43_9;
        // D s_43_11: or s_43_10 s_43_8
        let s_43_11: u128 = ((s_43_10) | (s_43_8));
        // D s_43_12: add s_43_7 s_43_9
        let s_43_12: u16 = (s_43_7 + s_43_9);
        // D s_43_13: create-bits s_43_11 s_43_12
        let s_43_13: Bits = Bits::new(s_43_11, s_43_12);
        // D s_43_14: cast reint s_43_13 -> u64
        let s_43_14: u64 = (s_43_13.value() as u64);
        // D s_43_15: call Mk_TTBR1_Type(s_43_14)
        let s_43_15: ProductType5c790c8ef59cc8b2 = Mk_TTBR1_Type(state, tracer, s_43_14);
        // C s_43_16: const #20024u : u32
        let s_43_16: u32 = 20024;
        // N s_43_17: write-reg s_43_16 <= s_43_15
        let s_43_17: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_43_16 as isize, s_43_15);
            tracer.write_register(s_43_16 as isize, s_43_15);
        };
        // N s_43_18: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #424u : u32
        let s_44_0: u32 = 424;
        // D s_44_1: read-reg s_44_0:u8
        let s_44_1: u8 = {
            let value = state.read_register::<u8>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call ELUsingAArch32(s_44_1)
        let s_44_2: bool = ELUsingAArch32(state, tracer, s_44_1);
        // D s_44_3: write-var gs#105057 <= s_44_2
        fn_state.gs_105057 = s_44_2;
        // N s_44_4: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #4u : u8
        let s_45_0: u8 = 4;
        // C s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 8u16);
        // C s_45_2: cast zx s_45_1 -> i
        let s_45_2: i128 = (s_45_1.value() as i128);
        // C s_45_3: cast reint s_45_2 -> i64
        let s_45_3: i64 = (s_45_2 as i64);
        // C s_45_4: cast zx s_45_3 -> i
        let s_45_4: i128 = (i128::try_from(s_45_3).unwrap());
        // S s_45_5: call AArch32_TakeHypTrapException(s_45_4)
        let s_45_5: () = AArch32_TakeHypTrapException(state, tracer, s_45_4);
        // N s_45_6: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var __HCR_TVM:u8
        let s_46_0: bool = fn_state.u__HCR_TVM;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#105056 <= s_46_4
        fn_state.gs_105056 = s_46_4;
        // N s_46_6: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #432u : u32
        let s_47_0: u32 = 432;
        // D s_47_1: read-reg s_47_0:u8
        let s_47_1: u8 = {
            let value = state.read_register::<u8>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call ELUsingAArch32(s_47_1)
        let s_47_2: bool = ELUsingAArch32(state, tracer, s_47_1);
        // D s_47_3: write-var gs#105055 <= s_47_2
        fn_state.gs_105055 = s_47_2;
        // N s_47_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #4u : u8
        let s_48_0: u8 = 4;
        // C s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 8u16);
        // C s_48_2: cast zx s_48_1 -> i
        let s_48_2: i128 = (s_48_1.value() as i128);
        // C s_48_3: cast reint s_48_2 -> i64
        let s_48_3: i64 = (s_48_2 as i64);
        // C s_48_4: cast zx s_48_3 -> i
        let s_48_4: i128 = (i128::try_from(s_48_3).unwrap());
        // C s_48_5: const #432u : u32
        let s_48_5: u32 = 432;
        // D s_48_6: read-reg s_48_5:u8
        let s_48_6: u8 = {
            let value = state.read_register::<u8>(s_48_5 as isize);
            tracer.read_register(s_48_5 as isize, value);
            value
        };
        // D s_48_7: call AArch64_AArch32SystemAccessTrap(s_48_6, s_48_4)
        let s_48_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_48_6, s_48_4);
        // N s_48_8: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var __HCR_EL2_TVM:u8
        let s_49_0: bool = fn_state.u__HCR_EL2_TVM;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 1u16);
        // C s_49_2: const #1u : u8
        let s_49_2: bool = true;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 1u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // D s_49_5: write-var gs#105054 <= s_49_4
        fn_state.gs_105054 = s_49_4;
        // N s_49_6: jump b33
        return block_33(state, tracer, fn_state);
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
        // D s_50_2: call ELUsingAArch32(s_50_1)
        let s_50_2: bool = ELUsingAArch32(state, tracer, s_50_1);
        // D s_50_3: not s_50_2
        let s_50_3: bool = !s_50_2;
        // D s_50_4: write-var gs#105053 <= s_50_3
        fn_state.gs_105053 = s_50_3;
        // N s_50_5: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #4u : u8
        let s_51_0: u8 = 4;
        // C s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 8u16);
        // C s_51_2: cast zx s_51_1 -> i
        let s_51_2: i128 = (s_51_1.value() as i128);
        // C s_51_3: cast reint s_51_2 -> i64
        let s_51_3: i64 = (s_51_2 as i64);
        // C s_51_4: cast zx s_51_3 -> i
        let s_51_4: i128 = (i128::try_from(s_51_3).unwrap());
        // S s_51_5: call AArch32_TakeHypTrapException(s_51_4)
        let s_51_5: () = AArch32_TakeHypTrapException(state, tracer, s_51_4);
        // N s_51_6: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var __HSTR_T2:u8
        let s_52_0: bool = fn_state.u__HSTR_T2;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 1u16);
        // C s_52_2: const #1u : u8
        let s_52_2: bool = true;
        // C s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // D s_52_5: write-var gs#105052 <= s_52_4
        fn_state.gs_105052 = s_52_4;
        // N s_52_6: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #432u : u32
        let s_53_0: u32 = 432;
        // D s_53_1: read-reg s_53_0:u8
        let s_53_1: u8 = {
            let value = state.read_register::<u8>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // D s_53_2: call ELUsingAArch32(s_53_1)
        let s_53_2: bool = ELUsingAArch32(state, tracer, s_53_1);
        // D s_53_3: write-var gs#105051 <= s_53_2
        fn_state.gs_105051 = s_53_2;
        // N s_53_4: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #4u : u8
        let s_54_0: u8 = 4;
        // C s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 8u16);
        // C s_54_2: cast zx s_54_1 -> i
        let s_54_2: i128 = (s_54_1.value() as i128);
        // C s_54_3: cast reint s_54_2 -> i64
        let s_54_3: i64 = (s_54_2 as i64);
        // C s_54_4: cast zx s_54_3 -> i
        let s_54_4: i128 = (i128::try_from(s_54_3).unwrap());
        // C s_54_5: const #432u : u32
        let s_54_5: u32 = 432;
        // D s_54_6: read-reg s_54_5:u8
        let s_54_6: u8 = {
            let value = state.read_register::<u8>(s_54_5 as isize);
            tracer.read_register(s_54_5 as isize, value);
            value
        };
        // D s_54_7: call AArch64_AArch32SystemAccessTrap(s_54_6, s_54_4)
        let s_54_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_54_6, s_54_4);
        // N s_54_8: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var __HSTR_EL2_T2:u8
        let s_55_0: bool = fn_state.u__HSTR_EL2_T2;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 1u16);
        // C s_55_2: const #1u : u8
        let s_55_2: bool = true;
        // C s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // D s_55_4: cmp-eq s_55_1 s_55_3
        let s_55_4: bool = ((s_55_1) == (s_55_3));
        // D s_55_5: write-var gs#105050 <= s_55_4
        fn_state.gs_105050 = s_55_4;
        // N s_55_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #432u : u32
        let s_56_0: u32 = 432;
        // D s_56_1: read-reg s_56_0:u8
        let s_56_1: u8 = {
            let value = state.read_register::<u8>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: call ELUsingAArch32(s_56_1)
        let s_56_2: bool = ELUsingAArch32(state, tracer, s_56_1);
        // D s_56_3: not s_56_2
        let s_56_3: bool = !s_56_2;
        // D s_56_4: write-var gs#105049 <= s_56_3
        fn_state.gs_105049 = s_56_3;
        // N s_56_5: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_57_0: panic
        panic!("{:?}", ());
        // N s_57_1: return
        return;
    }
}
