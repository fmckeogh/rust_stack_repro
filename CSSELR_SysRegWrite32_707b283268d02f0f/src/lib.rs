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
use u_get_HSTR_EL2_Type_T0::*;
use Mk_CSSELR_Type::*;
use CSSELR_write::*;
use HCR_read::*;
use u_get_HCR_Type_TID2::*;
use u_get_HCR_EL2_Type_TID4::*;
use CSSELR_NS_write::*;
use u_get_SCR_Type_NS::*;
use HSTR_read::*;
use AArch64_AArch32SystemAccessTrap::*;
use R_read::*;
use u_get_HSTR_Type_T0::*;
use ELUsingAArch32::*;
use u_get_HCR2_Type_TID4::*;
use u_get_HCR_EL2_Type_TID2::*;
use EL2Enabled::*;
use HCR2_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn CSSELR_SysRegWrite32_707b283268d02f0f<T: Tracer>(
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
        u__HCR_EL2_TID2: bool,
        gs_124564: bool,
        gs_124566: bool,
        gs_124574: bool,
        u__HCR_EL2_TID4: bool,
        gs_124567: bool,
        gs_124565: bool,
        gs_124571: bool,
        gs_124572: bool,
        gs_124575: bool,
        gs_124576: bool,
        u__HCR_TID2: bool,
        u__SCR_NS: bool,
        gs_124563: bool,
        gs_124573: bool,
        gs_124569: bool,
        gs_124570: bool,
        gs_124568: bool,
        u__PSTATE_EL: u8,
        u__HSTR_EL2_T0: bool,
        u__HSTR_T0: bool,
        u__HCR2_TID4: bool,
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
        // D s_0_5: call _get_HSTR_EL2_Type_T0(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T0(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T0 <= s_0_5
        fn_state.u__HSTR_EL2_T0 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T0(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T0(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T0 <= s_0_9
        fn_state.u__HSTR_T0 = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TID2(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TID2(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TID2 <= s_0_13
        fn_state.u__HCR_EL2_TID2 = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_TID4(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_TID4(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_TID4 <= s_0_17
        fn_state.u__HCR_EL2_TID4 = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call HCR_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_19);
        // S s_0_21: call _get_HCR_Type_TID2(s_0_20)
        let s_0_21: bool = u_get_HCR_Type_TID2(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_TID2 <= s_0_21
        fn_state.u__HCR_TID2 = s_0_21;
        // C s_0_23: const #() : ()
        let s_0_23: () = ();
        // S s_0_24: call HCR2_read(s_0_23)
        let s_0_24: ProductType700c18a878c5601b = HCR2_read(state, tracer, s_0_23);
        // S s_0_25: call _get_HCR2_Type_TID4(s_0_24)
        let s_0_25: bool = u_get_HCR2_Type_TID4(state, tracer, s_0_24);
        // D s_0_26: write-var __HCR2_TID4 <= s_0_25
        fn_state.u__HCR2_TID4 = s_0_25;
        // C s_0_27: const #20920u : u32
        let s_0_27: u32 = 20920;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_SCR_Type_NS(s_0_28)
        let s_0_29: bool = u_get_SCR_Type_NS(state, tracer, s_0_28);
        // D s_0_30: write-var __SCR_NS <= s_0_29
        fn_state.u__SCR_NS = s_0_29;
        // D s_0_31: read-var __PSTATE_EL:u8
        let s_0_31: u8 = fn_state.u__PSTATE_EL;
        // D s_0_32: cast zx s_0_31 -> bv
        let s_0_32: Bits = Bits::new(s_0_31 as u128, 2u16);
        // C s_0_33: const #448u : u32
        let s_0_33: u32 = 448;
        // D s_0_34: read-reg s_0_33:u8
        let s_0_34: u8 = {
            let value = state.read_register::<u8>(s_0_33 as isize);
            tracer.read_register(s_0_33 as isize, value);
            value
        };
        // D s_0_35: cast zx s_0_34 -> bv
        let s_0_35: Bits = Bits::new(s_0_34 as u128, 2u16);
        // D s_0_36: cmp-eq s_0_32 s_0_35
        let s_0_36: bool = ((s_0_32) == (s_0_35));
        // N s_0_37: branch s_0_36 b68 b1
        if s_0_36 {
            return block_68(state, tracer, fn_state);
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
        // D s_6_0: read-var t:i
        let s_6_0: i128 = fn_state.t;
        // D s_6_1: call R_read(s_6_0)
        let s_6_1: u32 = R_read(state, tracer, s_6_0);
        // D s_6_2: call Mk_CSSELR_Type(s_6_1)
        let s_6_2: ProductType700c18a878c5601b = Mk_CSSELR_Type(state, tracer, s_6_1);
        // D s_6_3: call CSSELR_NS_write(s_6_2)
        let s_6_3: () = CSSELR_NS_write(state, tracer, s_6_2);
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var t:i
        let s_7_0: i128 = fn_state.t;
        // D s_7_1: call R_read(s_7_0)
        let s_7_1: u32 = R_read(state, tracer, s_7_0);
        // D s_7_2: call Mk_CSSELR_Type(s_7_1)
        let s_7_2: ProductType700c18a878c5601b = Mk_CSSELR_Type(state, tracer, s_7_1);
        // C s_7_3: const #90736u : u32
        let s_7_3: u32 = 90736;
        // N s_7_4: write-reg s_7_3 <= s_7_2
        let s_7_4: () = {
            state.write_register::<ProductType700c18a878c5601b>(s_7_3 as isize, s_7_2);
            tracer.write_register(s_7_3 as isize, s_7_2);
        };
        // N s_7_5: return
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
        // D s_9_1: write-var gs#124563 <= s_9_0
        fn_state.gs_124563 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#124563:u8
        let s_10_0: bool = fn_state.gs_124563;
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
        // D s_11_0: read-var t:i
        let s_11_0: i128 = fn_state.t;
        // D s_11_1: call R_read(s_11_0)
        let s_11_1: u32 = R_read(state, tracer, s_11_0);
        // D s_11_2: call Mk_CSSELR_Type(s_11_1)
        let s_11_2: ProductType700c18a878c5601b = Mk_CSSELR_Type(state, tracer, s_11_1);
        // D s_11_3: call CSSELR_write(s_11_2)
        let s_11_3: () = CSSELR_write(state, tracer, s_11_2);
        // N s_11_4: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var t:i
        let s_12_0: i128 = fn_state.t;
        // D s_12_1: call R_read(s_12_0)
        let s_12_1: u32 = R_read(state, tracer, s_12_0);
        // D s_12_2: call Mk_CSSELR_Type(s_12_1)
        let s_12_2: ProductType700c18a878c5601b = Mk_CSSELR_Type(state, tracer, s_12_1);
        // D s_12_3: call CSSELR_NS_write(s_12_2)
        let s_12_3: () = CSSELR_NS_write(state, tracer, s_12_2);
        // N s_12_4: return
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
        // D s_13_3: write-var gs#124563 <= s_13_2
        fn_state.gs_124563 = s_13_2;
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
        // N s_14_2: branch s_14_1 b67 b15
        if s_14_1 {
            return block_67(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#124564 <= s_15_0
        fn_state.gs_124564 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#124564:u8
        let s_16_0: bool = fn_state.gs_124564;
        // N s_16_1: branch s_16_0 b66 b17
        if s_16_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#124565 <= s_17_0
        fn_state.gs_124565 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#124565:u8
        let s_18_0: bool = fn_state.gs_124565;
        // N s_18_1: branch s_18_0 b65 b19
        if s_18_0 {
            return block_65(state, tracer, fn_state);
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
        // N s_19_2: branch s_19_1 b64 b20
        if s_19_1 {
            return block_64(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#124566 <= s_20_0
        fn_state.gs_124566 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#124566:u8
        let s_21_0: bool = fn_state.gs_124566;
        // N s_21_1: branch s_21_0 b63 b22
        if s_21_0 {
            return block_63(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#124567 <= s_22_0
        fn_state.gs_124567 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#124567:u8
        let s_23_0: bool = fn_state.gs_124567;
        // N s_23_1: branch s_23_0 b62 b24
        if s_23_0 {
            return block_62(state, tracer, fn_state);
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
        // N s_24_2: branch s_24_1 b61 b25
        if s_24_1 {
            return block_61(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#124568 <= s_25_0
        fn_state.gs_124568 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#124568:u8
        let s_26_0: bool = fn_state.gs_124568;
        // N s_26_1: branch s_26_0 b60 b27
        if s_26_0 {
            return block_60(state, tracer, fn_state);
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
        // D s_27_1: write-var gs#124569 <= s_27_0
        fn_state.gs_124569 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#124569:u8
        let s_28_0: bool = fn_state.gs_124569;
        // N s_28_1: branch s_28_0 b59 b29
        if s_28_0 {
            return block_59(state, tracer, fn_state);
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
        // N s_29_2: branch s_29_1 b58 b30
        if s_29_1 {
            return block_58(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#124570 <= s_30_0
        fn_state.gs_124570 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#124570:u8
        let s_31_0: bool = fn_state.gs_124570;
        // N s_31_1: branch s_31_0 b57 b32
        if s_31_0 {
            return block_57(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#124571 <= s_32_0
        fn_state.gs_124571 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#124571:u8
        let s_33_0: bool = fn_state.gs_124571;
        // N s_33_1: branch s_33_0 b56 b34
        if s_33_0 {
            return block_56(state, tracer, fn_state);
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
        // N s_34_2: branch s_34_1 b55 b35
        if s_34_1 {
            return block_55(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#124572 <= s_35_0
        fn_state.gs_124572 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#124572:u8
        let s_36_0: bool = fn_state.gs_124572;
        // N s_36_1: branch s_36_0 b54 b37
        if s_36_0 {
            return block_54(state, tracer, fn_state);
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
        // D s_37_1: write-var gs#124573 <= s_37_0
        fn_state.gs_124573 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#124573:u8
        let s_38_0: bool = fn_state.gs_124573;
        // N s_38_1: branch s_38_0 b53 b39
        if s_38_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #() : ()
        let s_39_0: () = ();
        // S s_39_1: call EL2Enabled(s_39_0)
        let s_39_1: bool = EL2Enabled(state, tracer, s_39_0);
        // N s_39_2: branch s_39_1 b52 b40
        if s_39_1 {
            return block_52(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#124574 <= s_40_0
        fn_state.gs_124574 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#124574:u8
        let s_41_0: bool = fn_state.gs_124574;
        // N s_41_1: branch s_41_0 b51 b42
        if s_41_0 {
            return block_51(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#124575 <= s_42_0
        fn_state.gs_124575 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#124575:u8
        let s_43_0: bool = fn_state.gs_124575;
        // N s_43_1: branch s_43_0 b50 b44
        if s_43_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
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
        // C s_44_2: const #2u : u8
        let s_44_2: u8 = 2;
        // D s_44_3: cmp-lt s_44_1 s_44_2
        let s_44_3: bool = ((s_44_1) < (s_44_2));
        // N s_44_4: branch s_44_3 b49 b45
        if s_44_3 {
            return block_49(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#124576 <= s_45_0
        fn_state.gs_124576 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#124576:u8
        let s_46_0: bool = fn_state.gs_124576;
        // N s_46_1: branch s_46_0 b48 b47
        if s_46_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var t:i
        let s_47_0: i128 = fn_state.t;
        // D s_47_1: call R_read(s_47_0)
        let s_47_1: u32 = R_read(state, tracer, s_47_0);
        // D s_47_2: call Mk_CSSELR_Type(s_47_1)
        let s_47_2: ProductType700c18a878c5601b = Mk_CSSELR_Type(state, tracer, s_47_1);
        // D s_47_3: call CSSELR_write(s_47_2)
        let s_47_3: () = CSSELR_write(state, tracer, s_47_2);
        // N s_47_4: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var t:i
        let s_48_0: i128 = fn_state.t;
        // D s_48_1: call R_read(s_48_0)
        let s_48_1: u32 = R_read(state, tracer, s_48_0);
        // D s_48_2: call Mk_CSSELR_Type(s_48_1)
        let s_48_2: ProductType700c18a878c5601b = Mk_CSSELR_Type(state, tracer, s_48_1);
        // D s_48_3: call CSSELR_NS_write(s_48_2)
        let s_48_3: () = CSSELR_NS_write(state, tracer, s_48_2);
        // N s_48_4: return
        return;
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #424u : u32
        let s_49_0: u32 = 424;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: call ELUsingAArch32(s_49_1)
        let s_49_2: bool = ELUsingAArch32(state, tracer, s_49_1);
        // D s_49_3: write-var gs#124576 <= s_49_2
        fn_state.gs_124576 = s_49_2;
        // N s_49_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #3u : u8
        let s_50_0: u8 = 3;
        // C s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 8u16);
        // C s_50_2: cast zx s_50_1 -> i
        let s_50_2: i128 = (s_50_1.value() as i128);
        // C s_50_3: cast reint s_50_2 -> i64
        let s_50_3: i64 = (s_50_2 as i64);
        // C s_50_4: cast zx s_50_3 -> i
        let s_50_4: i128 = (i128::try_from(s_50_3).unwrap());
        // S s_50_5: call AArch32_TakeHypTrapException(s_50_4)
        let s_50_5: () = AArch32_TakeHypTrapException(state, tracer, s_50_4);
        // N s_50_6: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var __HCR2_TID4:u8
        let s_51_0: bool = fn_state.u__HCR2_TID4;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #1u : u8
        let s_51_2: bool = true;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#124575 <= s_51_4
        fn_state.gs_124575 = s_51_4;
        // N s_51_6: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #432u : u32
        let s_52_0: u32 = 432;
        // D s_52_1: read-reg s_52_0:u8
        let s_52_1: u8 = {
            let value = state.read_register::<u8>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: call ELUsingAArch32(s_52_1)
        let s_52_2: bool = ELUsingAArch32(state, tracer, s_52_1);
        // D s_52_3: write-var gs#124574 <= s_52_2
        fn_state.gs_124574 = s_52_2;
        // N s_52_4: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #3u : u8
        let s_53_0: u8 = 3;
        // C s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 8u16);
        // C s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (s_53_1.value() as i128);
        // C s_53_3: cast reint s_53_2 -> i64
        let s_53_3: i64 = (s_53_2 as i64);
        // C s_53_4: cast zx s_53_3 -> i
        let s_53_4: i128 = (i128::try_from(s_53_3).unwrap());
        // S s_53_5: call AArch32_TakeHypTrapException(s_53_4)
        let s_53_5: () = AArch32_TakeHypTrapException(state, tracer, s_53_4);
        // N s_53_6: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var __HCR_TID2:u8
        let s_54_0: bool = fn_state.u__HCR_TID2;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 1u16);
        // C s_54_2: const #1u : u8
        let s_54_2: bool = true;
        // C s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 1u16);
        // D s_54_4: cmp-eq s_54_1 s_54_3
        let s_54_4: bool = ((s_54_1) == (s_54_3));
        // D s_54_5: write-var gs#124573 <= s_54_4
        fn_state.gs_124573 = s_54_4;
        // N s_54_6: jump b38
        return block_38(state, tracer, fn_state);
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
        // D s_55_2: call ELUsingAArch32(s_55_1)
        let s_55_2: bool = ELUsingAArch32(state, tracer, s_55_1);
        // D s_55_3: write-var gs#124572 <= s_55_2
        fn_state.gs_124572 = s_55_2;
        // N s_55_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #3u : u8
        let s_56_0: u8 = 3;
        // C s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 8u16);
        // C s_56_2: cast zx s_56_1 -> i
        let s_56_2: i128 = (s_56_1.value() as i128);
        // C s_56_3: cast reint s_56_2 -> i64
        let s_56_3: i64 = (s_56_2 as i64);
        // C s_56_4: cast zx s_56_3 -> i
        let s_56_4: i128 = (i128::try_from(s_56_3).unwrap());
        // C s_56_5: const #432u : u32
        let s_56_5: u32 = 432;
        // D s_56_6: read-reg s_56_5:u8
        let s_56_6: u8 = {
            let value = state.read_register::<u8>(s_56_5 as isize);
            tracer.read_register(s_56_5 as isize, value);
            value
        };
        // D s_56_7: call AArch64_AArch32SystemAccessTrap(s_56_6, s_56_4)
        let s_56_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_56_6, s_56_4);
        // N s_56_8: return
        return;
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var __HCR_EL2_TID4:u8
        let s_57_0: bool = fn_state.u__HCR_EL2_TID4;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 1u16);
        // C s_57_2: const #1u : u8
        let s_57_2: bool = true;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // D s_57_4: cmp-eq s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) == (s_57_3));
        // D s_57_5: write-var gs#124571 <= s_57_4
        fn_state.gs_124571 = s_57_4;
        // N s_57_6: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #432u : u32
        let s_58_0: u32 = 432;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: u8 = {
            let value = state.read_register::<u8>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call ELUsingAArch32(s_58_1)
        let s_58_2: bool = ELUsingAArch32(state, tracer, s_58_1);
        // D s_58_3: not s_58_2
        let s_58_3: bool = !s_58_2;
        // D s_58_4: write-var gs#124570 <= s_58_3
        fn_state.gs_124570 = s_58_3;
        // N s_58_5: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #3u : u8
        let s_59_0: u8 = 3;
        // C s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 8u16);
        // C s_59_2: cast zx s_59_1 -> i
        let s_59_2: i128 = (s_59_1.value() as i128);
        // C s_59_3: cast reint s_59_2 -> i64
        let s_59_3: i64 = (s_59_2 as i64);
        // C s_59_4: cast zx s_59_3 -> i
        let s_59_4: i128 = (i128::try_from(s_59_3).unwrap());
        // C s_59_5: const #432u : u32
        let s_59_5: u32 = 432;
        // D s_59_6: read-reg s_59_5:u8
        let s_59_6: u8 = {
            let value = state.read_register::<u8>(s_59_5 as isize);
            tracer.read_register(s_59_5 as isize, value);
            value
        };
        // D s_59_7: call AArch64_AArch32SystemAccessTrap(s_59_6, s_59_4)
        let s_59_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_59_6, s_59_4);
        // N s_59_8: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __HCR_EL2_TID2:u8
        let s_60_0: bool = fn_state.u__HCR_EL2_TID2;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #1u : u8
        let s_60_2: bool = true;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#124569 <= s_60_4
        fn_state.gs_124569 = s_60_4;
        // N s_60_6: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #432u : u32
        let s_61_0: u32 = 432;
        // D s_61_1: read-reg s_61_0:u8
        let s_61_1: u8 = {
            let value = state.read_register::<u8>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call ELUsingAArch32(s_61_1)
        let s_61_2: bool = ELUsingAArch32(state, tracer, s_61_1);
        // D s_61_3: not s_61_2
        let s_61_3: bool = !s_61_2;
        // D s_61_4: write-var gs#124568 <= s_61_3
        fn_state.gs_124568 = s_61_3;
        // N s_61_5: jump b26
        return block_26(state, tracer, fn_state);
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
        // D s_63_0: read-var __HSTR_T0:u8
        let s_63_0: bool = fn_state.u__HSTR_T0;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #1u : u8
        let s_63_2: bool = true;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#124567 <= s_63_4
        fn_state.gs_124567 = s_63_4;
        // N s_63_6: jump b23
        return block_23(state, tracer, fn_state);
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
        // D s_64_3: write-var gs#124566 <= s_64_2
        fn_state.gs_124566 = s_64_2;
        // N s_64_4: jump b21
        return block_21(state, tracer, fn_state);
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
        // D s_66_0: read-var __HSTR_EL2_T0:u8
        let s_66_0: bool = fn_state.u__HSTR_EL2_T0;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #1u : u8
        let s_66_2: bool = true;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#124565 <= s_66_4
        fn_state.gs_124565 = s_66_4;
        // N s_66_6: jump b18
        return block_18(state, tracer, fn_state);
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
        // D s_67_4: write-var gs#124564 <= s_67_3
        fn_state.gs_124564 = s_67_3;
        // N s_67_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_68_0: panic
        panic!("{:?}", ());
        // N s_68_1: return
        return;
    }
}
