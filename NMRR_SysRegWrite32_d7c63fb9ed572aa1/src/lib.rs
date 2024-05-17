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
use MAIR1_write::*;
use Mk_MAIR1_Type::*;
use HCR_read::*;
use Mk_NMRR_Type::*;
use NMRR_write::*;
use u_get_SCR_Type_NS::*;
use HSTR_read::*;
use MAIR1_S_write::*;
use EAEisOne::*;
use u_get_HSTR_EL2_Type_T10::*;
use AArch64_AArch32SystemAccessTrap::*;
use R_read::*;
use u_get_HCR_EL2_Type_TVM::*;
use MAIR1_NS_write::*;
use ELUsingAArch32::*;
use u_get_HSTR_Type_T10::*;
use u_get_HCR_Type_TVM::*;
use EL2Enabled::*;
use NMRR_NS_write::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn NMRR_SysRegWrite32_d7c63fb9ed572aa1<T: Tracer>(
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
        gs_129569: bool,
        gs_129580: bool,
        gs_129576: bool,
        gs_129575: bool,
        u__HSTR_T10: bool,
        u__SCR_NS: bool,
        u__HCR_EL2_TVM: bool,
        u__HSTR_EL2_T10: bool,
        gs_129570: bool,
        gs_129579: bool,
        gs_129571: bool,
        gs_129573: bool,
        gs_129578: bool,
        gs_129574: bool,
        u__PSTATE_EL: u8,
        gs_129572: bool,
        gs_129577: bool,
        u__HCR_TVM: bool,
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
        // D s_0_5: call _get_HSTR_EL2_Type_T10(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T10(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T10 <= s_0_5
        fn_state.u__HSTR_EL2_T10 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T10(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T10(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T10 <= s_0_9
        fn_state.u__HSTR_T10 = s_0_9;
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
        // N s_0_29: branch s_0_28 b74 b1
        if s_0_28 {
            return block_74(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b32 b2
        if s_1_5 {
            return block_32(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b22 b3
        if s_2_5 {
            return block_22(state, tracer, fn_state);
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
        // N s_5_5: branch s_5_4 b21 b6
        if s_5_4 {
            return block_21(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#129569 <= s_6_0
        fn_state.gs_129569 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#129569:u8
        let s_7_0: bool = fn_state.gs_129569;
        // N s_7_1: branch s_7_0 b20 b8
        if s_7_0 {
            return block_20(state, tracer, fn_state);
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
        // N s_8_5: branch s_8_4 b19 b9
        if s_8_4 {
            return block_19(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#129570 <= s_9_0
        fn_state.gs_129570 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#129570:u8
        let s_10_0: bool = fn_state.gs_129570;
        // N s_10_1: branch s_10_0 b18 b11
        if s_10_0 {
            return block_18(state, tracer, fn_state);
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
        // S s_11_1: call EAEisOne(s_11_0)
        let s_11_1: bool = EAEisOne(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b15 b12
        if s_11_1 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var __SCR_NS:u8
        let s_12_0: bool = fn_state.u__SCR_NS;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #0u : u8
        let s_12_2: bool = false;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // N s_12_5: branch s_12_4 b14 b13
        if s_12_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var t:i
        let s_13_0: i128 = fn_state.t;
        // D s_13_1: call R_read(s_13_0)
        let s_13_1: u32 = R_read(state, tracer, s_13_0);
        // D s_13_2: call Mk_NMRR_Type(s_13_1)
        let s_13_2: ProductType700c18a878c5601b = Mk_NMRR_Type(state, tracer, s_13_1);
        // D s_13_3: call NMRR_NS_write(s_13_2)
        let s_13_3: () = NMRR_NS_write(state, tracer, s_13_2);
        // N s_13_4: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var t:i
        let s_14_0: i128 = fn_state.t;
        // D s_14_1: call R_read(s_14_0)
        let s_14_1: u32 = R_read(state, tracer, s_14_0);
        // D s_14_2: call Mk_NMRR_Type(s_14_1)
        let s_14_2: ProductType700c18a878c5601b = Mk_NMRR_Type(state, tracer, s_14_1);
        // C s_14_3: const #100840u : u32
        let s_14_3: u32 = 100840;
        // N s_14_4: write-reg s_14_3 <= s_14_2
        let s_14_4: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_14_3 as isize, s_14_2);
            tracer.write_register(s_14_3 as isize, s_14_2);
        };
        // N s_14_5: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var __SCR_NS:u8
        let s_15_0: bool = fn_state.u__SCR_NS;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #0u : u8
        let s_15_2: bool = false;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // N s_15_5: branch s_15_4 b17 b16
        if s_15_4 {
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
        // D s_16_0: read-var t:i
        let s_16_0: i128 = fn_state.t;
        // D s_16_1: call R_read(s_16_0)
        let s_16_1: u32 = R_read(state, tracer, s_16_0);
        // D s_16_2: call Mk_MAIR1_Type(s_16_1)
        let s_16_2: ProductType700c18a878c5601b = Mk_MAIR1_Type(state, tracer, s_16_1);
        // D s_16_3: call MAIR1_NS_write(s_16_2)
        let s_16_3: () = MAIR1_NS_write(state, tracer, s_16_2);
        // N s_16_4: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var t:i
        let s_17_0: i128 = fn_state.t;
        // D s_17_1: call R_read(s_17_0)
        let s_17_1: u32 = R_read(state, tracer, s_17_0);
        // D s_17_2: call Mk_MAIR1_Type(s_17_1)
        let s_17_2: ProductType700c18a878c5601b = Mk_MAIR1_Type(state, tracer, s_17_1);
        // D s_17_3: call MAIR1_S_write(s_17_2)
        let s_17_3: () = MAIR1_S_write(state, tracer, s_17_2);
        // N s_17_4: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: panic
        panic!("{:?}", ());
        // N s_18_1: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #20328u : u32
        let s_19_0: u32 = 20328;
        // D s_19_1: read-reg s_19_0:u32
        let s_19_1: u32 = {
            let value = state.read_register::<u32>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // C s_19_2: const #1u : u32
        let s_19_2: u32 = 1;
        // D s_19_3: cmp-eq s_19_1 s_19_2
        let s_19_3: bool = ((s_19_1) == (s_19_2));
        // D s_19_4: write-var gs#129570 <= s_19_3
        fn_state.gs_129570 = s_19_3;
        // N s_19_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: panic
        panic!("{:?}", ());
        // N s_20_1: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #10192u : u32
        let s_21_0: u32 = 10192;
        // D s_21_1: read-reg s_21_0:u32
        let s_21_1: u32 = {
            let value = state.read_register::<u32>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // C s_21_2: const #1u : u32
        let s_21_2: u32 = 1;
        // D s_21_3: cmp-eq s_21_1 s_21_2
        let s_21_3: bool = ((s_21_1) == (s_21_2));
        // D s_21_4: write-var gs#129569 <= s_21_3
        fn_state.gs_129569 = s_21_3;
        // N s_21_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #424u : u32
        let s_22_0: u32 = 424;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // C s_22_2: const #2u : u8
        let s_22_2: u8 = 2;
        // D s_22_3: cmp-lt s_22_1 s_22_2
        let s_22_3: bool = ((s_22_1) < (s_22_2));
        // N s_22_4: branch s_22_3 b31 b23
        if s_22_3 {
            return block_31(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#129571 <= s_23_0
        fn_state.gs_129571 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#129571:u8
        let s_24_0: bool = fn_state.gs_129571;
        // N s_24_1: branch s_24_0 b28 b25
        if s_24_0 {
            return block_28(state, tracer, fn_state);
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
        // S s_25_1: call EAEisOne(s_25_0)
        let s_25_1: bool = EAEisOne(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b27 b26
        if s_25_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var t:i
        let s_26_0: i128 = fn_state.t;
        // D s_26_1: call R_read(s_26_0)
        let s_26_1: u32 = R_read(state, tracer, s_26_0);
        // D s_26_2: call Mk_NMRR_Type(s_26_1)
        let s_26_2: ProductType700c18a878c5601b = Mk_NMRR_Type(state, tracer, s_26_1);
        // D s_26_3: call NMRR_write(s_26_2)
        let s_26_3: () = NMRR_write(state, tracer, s_26_2);
        // N s_26_4: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var t:i
        let s_27_0: i128 = fn_state.t;
        // D s_27_1: call R_read(s_27_0)
        let s_27_1: u32 = R_read(state, tracer, s_27_0);
        // D s_27_2: call Mk_MAIR1_Type(s_27_1)
        let s_27_2: ProductType700c18a878c5601b = Mk_MAIR1_Type(state, tracer, s_27_1);
        // D s_27_3: call MAIR1_write(s_27_2)
        let s_27_3: () = MAIR1_write(state, tracer, s_27_2);
        // N s_27_4: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EAEisOne(s_28_0)
        let s_28_1: bool = EAEisOne(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b30 b29
        if s_28_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var t:i
        let s_29_0: i128 = fn_state.t;
        // D s_29_1: call R_read(s_29_0)
        let s_29_1: u32 = R_read(state, tracer, s_29_0);
        // D s_29_2: call Mk_NMRR_Type(s_29_1)
        let s_29_2: ProductType700c18a878c5601b = Mk_NMRR_Type(state, tracer, s_29_1);
        // D s_29_3: call NMRR_NS_write(s_29_2)
        let s_29_3: () = NMRR_NS_write(state, tracer, s_29_2);
        // N s_29_4: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var t:i
        let s_30_0: i128 = fn_state.t;
        // D s_30_1: call R_read(s_30_0)
        let s_30_1: u32 = R_read(state, tracer, s_30_0);
        // D s_30_2: call Mk_MAIR1_Type(s_30_1)
        let s_30_2: ProductType700c18a878c5601b = Mk_MAIR1_Type(state, tracer, s_30_1);
        // D s_30_3: call MAIR1_NS_write(s_30_2)
        let s_30_3: () = MAIR1_NS_write(state, tracer, s_30_2);
        // N s_30_4: return
        return;
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
        // D s_31_2: call ELUsingAArch32(s_31_1)
        let s_31_2: bool = ELUsingAArch32(state, tracer, s_31_1);
        // D s_31_3: write-var gs#129571 <= s_31_2
        fn_state.gs_129571 = s_31_2;
        // N s_31_4: jump b24
        return block_24(state, tracer, fn_state);
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
        // N s_32_2: branch s_32_1 b73 b33
        if s_32_1 {
            return block_73(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#129572 <= s_33_0
        fn_state.gs_129572 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#129572:u8
        let s_34_0: bool = fn_state.gs_129572;
        // N s_34_1: branch s_34_0 b72 b35
        if s_34_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#129573 <= s_35_0
        fn_state.gs_129573 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#129573:u8
        let s_36_0: bool = fn_state.gs_129573;
        // N s_36_1: branch s_36_0 b71 b37
        if s_36_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call EL2Enabled(s_37_0)
        let s_37_1: bool = EL2Enabled(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b70 b38
        if s_37_1 {
            return block_70(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#129574 <= s_38_0
        fn_state.gs_129574 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#129574:u8
        let s_39_0: bool = fn_state.gs_129574;
        // N s_39_1: branch s_39_0 b69 b40
        if s_39_0 {
            return block_69(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#129575 <= s_40_0
        fn_state.gs_129575 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#129575:u8
        let s_41_0: bool = fn_state.gs_129575;
        // N s_41_1: branch s_41_0 b68 b42
        if s_41_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call EL2Enabled(s_42_0)
        let s_42_1: bool = EL2Enabled(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b67 b43
        if s_42_1 {
            return block_67(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#129576 <= s_43_0
        fn_state.gs_129576 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#129576:u8
        let s_44_0: bool = fn_state.gs_129576;
        // N s_44_1: branch s_44_0 b66 b45
        if s_44_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#129577 <= s_45_0
        fn_state.gs_129577 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#129577:u8
        let s_46_0: bool = fn_state.gs_129577;
        // N s_46_1: branch s_46_0 b65 b47
        if s_46_0 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EL2Enabled(s_47_0)
        let s_47_1: bool = EL2Enabled(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b64 b48
        if s_47_1 {
            return block_64(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#129578 <= s_48_0
        fn_state.gs_129578 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#129578:u8
        let s_49_0: bool = fn_state.gs_129578;
        // N s_49_1: branch s_49_0 b63 b50
        if s_49_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#129579 <= s_50_0
        fn_state.gs_129579 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#129579:u8
        let s_51_0: bool = fn_state.gs_129579;
        // N s_51_1: branch s_51_0 b62 b52
        if s_51_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #424u : u32
        let s_52_0: u32 = 424;
        // D s_52_1: read-reg s_52_0:u8
        let s_52_1: u8 = {
            let value = state.read_register::<u8>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // C s_52_2: const #2u : u8
        let s_52_2: u8 = 2;
        // D s_52_3: cmp-lt s_52_1 s_52_2
        let s_52_3: bool = ((s_52_1) < (s_52_2));
        // N s_52_4: branch s_52_3 b61 b53
        if s_52_3 {
            return block_61(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#129580 <= s_53_0
        fn_state.gs_129580 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#129580:u8
        let s_54_0: bool = fn_state.gs_129580;
        // N s_54_1: branch s_54_0 b58 b55
        if s_54_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call EAEisOne(s_55_0)
        let s_55_1: bool = EAEisOne(state, tracer, s_55_0);
        // N s_55_2: branch s_55_1 b57 b56
        if s_55_1 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var t:i
        let s_56_0: i128 = fn_state.t;
        // D s_56_1: call R_read(s_56_0)
        let s_56_1: u32 = R_read(state, tracer, s_56_0);
        // D s_56_2: call Mk_NMRR_Type(s_56_1)
        let s_56_2: ProductType700c18a878c5601b = Mk_NMRR_Type(state, tracer, s_56_1);
        // D s_56_3: call NMRR_write(s_56_2)
        let s_56_3: () = NMRR_write(state, tracer, s_56_2);
        // N s_56_4: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var t:i
        let s_57_0: i128 = fn_state.t;
        // D s_57_1: call R_read(s_57_0)
        let s_57_1: u32 = R_read(state, tracer, s_57_0);
        // D s_57_2: call Mk_MAIR1_Type(s_57_1)
        let s_57_2: ProductType700c18a878c5601b = Mk_MAIR1_Type(state, tracer, s_57_1);
        // D s_57_3: call MAIR1_write(s_57_2)
        let s_57_3: () = MAIR1_write(state, tracer, s_57_2);
        // N s_57_4: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call EAEisOne(s_58_0)
        let s_58_1: bool = EAEisOne(state, tracer, s_58_0);
        // N s_58_2: branch s_58_1 b60 b59
        if s_58_1 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var t:i
        let s_59_0: i128 = fn_state.t;
        // D s_59_1: call R_read(s_59_0)
        let s_59_1: u32 = R_read(state, tracer, s_59_0);
        // D s_59_2: call Mk_NMRR_Type(s_59_1)
        let s_59_2: ProductType700c18a878c5601b = Mk_NMRR_Type(state, tracer, s_59_1);
        // D s_59_3: call NMRR_NS_write(s_59_2)
        let s_59_3: () = NMRR_NS_write(state, tracer, s_59_2);
        // N s_59_4: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var t:i
        let s_60_0: i128 = fn_state.t;
        // D s_60_1: call R_read(s_60_0)
        let s_60_1: u32 = R_read(state, tracer, s_60_0);
        // D s_60_2: call Mk_MAIR1_Type(s_60_1)
        let s_60_2: ProductType700c18a878c5601b = Mk_MAIR1_Type(state, tracer, s_60_1);
        // D s_60_3: call MAIR1_NS_write(s_60_2)
        let s_60_3: () = MAIR1_NS_write(state, tracer, s_60_2);
        // N s_60_4: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #424u : u32
        let s_61_0: u32 = 424;
        // D s_61_1: read-reg s_61_0:u8
        let s_61_1: u8 = {
            let value = state.read_register::<u8>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call ELUsingAArch32(s_61_1)
        let s_61_2: bool = ELUsingAArch32(state, tracer, s_61_1);
        // D s_61_3: write-var gs#129580 <= s_61_2
        fn_state.gs_129580 = s_61_2;
        // N s_61_4: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #3u : u8
        let s_62_0: u8 = 3;
        // C s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 8u16);
        // C s_62_2: cast zx s_62_1 -> i
        let s_62_2: i128 = (s_62_1.value() as i128);
        // C s_62_3: cast reint s_62_2 -> i64
        let s_62_3: i64 = (s_62_2 as i64);
        // C s_62_4: cast zx s_62_3 -> i
        let s_62_4: i128 = (i128::try_from(s_62_3).unwrap());
        // S s_62_5: call AArch32_TakeHypTrapException(s_62_4)
        let s_62_5: () = AArch32_TakeHypTrapException(state, tracer, s_62_4);
        // N s_62_6: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var __HCR_TVM:u8
        let s_63_0: bool = fn_state.u__HCR_TVM;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #1u : u8
        let s_63_2: bool = true;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#129579 <= s_63_4
        fn_state.gs_129579 = s_63_4;
        // N s_63_6: jump b51
        return block_51(state, tracer, fn_state);
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
        // D s_64_3: write-var gs#129578 <= s_64_2
        fn_state.gs_129578 = s_64_2;
        // N s_64_4: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #3u : u8
        let s_65_0: u8 = 3;
        // C s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 8u16);
        // C s_65_2: cast zx s_65_1 -> i
        let s_65_2: i128 = (s_65_1.value() as i128);
        // C s_65_3: cast reint s_65_2 -> i64
        let s_65_3: i64 = (s_65_2 as i64);
        // C s_65_4: cast zx s_65_3 -> i
        let s_65_4: i128 = (i128::try_from(s_65_3).unwrap());
        // C s_65_5: const #432u : u32
        let s_65_5: u32 = 432;
        // D s_65_6: read-reg s_65_5:u8
        let s_65_6: u8 = {
            let value = state.read_register::<u8>(s_65_5 as isize);
            tracer.read_register(s_65_5 as isize, value);
            value
        };
        // D s_65_7: call AArch64_AArch32SystemAccessTrap(s_65_6, s_65_4)
        let s_65_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_65_6, s_65_4);
        // N s_65_8: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var __HCR_EL2_TVM:u8
        let s_66_0: bool = fn_state.u__HCR_EL2_TVM;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #1u : u8
        let s_66_2: bool = true;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#129577 <= s_66_4
        fn_state.gs_129577 = s_66_4;
        // N s_66_6: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #432u : u32
        let s_67_0: u32 = 432;
        // D s_67_1: read-reg s_67_0:u8
        let s_67_1: u8 = {
            let value = state.read_register::<u8>(s_67_0 as isize);
            tracer.read_register(s_67_0 as isize, value);
            value
        };
        // D s_67_2: call ELUsingAArch32(s_67_1)
        let s_67_2: bool = ELUsingAArch32(state, tracer, s_67_1);
        // D s_67_3: not s_67_2
        let s_67_3: bool = !s_67_2;
        // D s_67_4: write-var gs#129576 <= s_67_3
        fn_state.gs_129576 = s_67_3;
        // N s_67_5: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #3u : u8
        let s_68_0: u8 = 3;
        // C s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 8u16);
        // C s_68_2: cast zx s_68_1 -> i
        let s_68_2: i128 = (s_68_1.value() as i128);
        // C s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: cast zx s_68_3 -> i
        let s_68_4: i128 = (i128::try_from(s_68_3).unwrap());
        // S s_68_5: call AArch32_TakeHypTrapException(s_68_4)
        let s_68_5: () = AArch32_TakeHypTrapException(state, tracer, s_68_4);
        // N s_68_6: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __HSTR_T10:u8
        let s_69_0: bool = fn_state.u__HSTR_T10;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#129575 <= s_69_4
        fn_state.gs_129575 = s_69_4;
        // N s_69_6: jump b41
        return block_41(state, tracer, fn_state);
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
        // D s_70_3: write-var gs#129574 <= s_70_2
        fn_state.gs_129574 = s_70_2;
        // N s_70_4: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #3u : u8
        let s_71_0: u8 = 3;
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
        // D s_72_0: read-var __HSTR_EL2_T10:u8
        let s_72_0: bool = fn_state.u__HSTR_EL2_T10;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #1u : u8
        let s_72_2: bool = true;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#129573 <= s_72_4
        fn_state.gs_129573 = s_72_4;
        // N s_72_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #432u : u32
        let s_73_0: u32 = 432;
        // D s_73_1: read-reg s_73_0:u8
        let s_73_1: u8 = {
            let value = state.read_register::<u8>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // D s_73_2: call ELUsingAArch32(s_73_1)
        let s_73_2: bool = ELUsingAArch32(state, tracer, s_73_1);
        // D s_73_3: not s_73_2
        let s_73_3: bool = !s_73_2;
        // D s_73_4: write-var gs#129572 <= s_73_3
        fn_state.gs_129572 = s_73_3;
        // N s_73_5: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_74_0: panic
        panic!("{:?}", ());
        // N s_74_1: return
        return;
    }
}
