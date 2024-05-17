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
use CNTFRQ_read::*;
use u_get_CNTHCTL_EL2_Type_EL0PCTEN::*;
use u_get_CNTKCTL_Type_PL0PCTEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use HCR_read::*;
use u_get_CNTKCTL_EL1_Type_EL0VCTEN::*;
use u_get_CNTKCTL_Type_PL0VCTEN::*;
use u_get_CNTHCTL_EL2_Type_EL0VCTEN::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HCR_Type_TGE::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_CNTKCTL_EL1_Type_EL0PCTEN::*;
use CNTKCTL_read__1::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn CNTFRQ_SysRegRead32_4d7a0faa14f3d1f9<T: Tracer>(
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
        gs_106925: bool,
        u__HCR_EL2_TGE: bool,
        gs_106926: bool,
        gs_106924: bool,
        gs_106923: bool,
        gs_106921: bool,
        gs_106922: bool,
        u__HCR_TGE: bool,
        u__CNTKCTL_PL0PCTEN: bool,
        gs_106929: bool,
        gs_106919: bool,
        u__CNTKCTL_PL0VCTEN: bool,
        gs_106930: bool,
        gs_106927: bool,
        gs_106920: bool,
        u__PSTATE_EL: u8,
        gs_106918: bool,
        gs_106931: bool,
        gs_106928: bool,
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
        // D s_0_5: call _get_HCR_EL2_Type_TGE(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_TGE <= s_0_5
        fn_state.u__HCR_EL2_TGE = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CNTKCTL_read__1(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = CNTKCTL_read__1(state, tracer, s_0_7);
        // S s_0_9: call _get_CNTKCTL_Type_PL0PCTEN(s_0_8)
        let s_0_9: bool = u_get_CNTKCTL_Type_PL0PCTEN(state, tracer, s_0_8);
        // D s_0_10: write-var __CNTKCTL_PL0PCTEN <= s_0_9
        fn_state.u__CNTKCTL_PL0PCTEN = s_0_9;
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
        // D s_0_19: read-var __PSTATE_EL:u8
        let s_0_19: u8 = fn_state.u__PSTATE_EL;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 2u16);
        // C s_0_21: const #448u : u32
        let s_0_21: u32 = 448;
        // D s_0_22: read-reg s_0_21:u8
        let s_0_22: u8 = {
            let value = state.read_register::<u8>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 2u16);
        // D s_0_24: cmp-eq s_0_20 s_0_23
        let s_0_24: bool = ((s_0_20) == (s_0_23));
        // N s_0_25: branch s_0_24 b8 b1
        if s_0_24 {
            return block_8(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call CNTFRQ_read(s_5_0)
        let s_5_1: u32 = CNTFRQ_read(state, tracer, s_5_0);
        // D s_5_2: read-var t:i
        let s_5_2: i128 = fn_state.t;
        // D s_5_3: call R_set(s_5_2, s_5_1)
        let s_5_3: () = R_set(state, tracer, s_5_2, s_5_1);
        // N s_5_4: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call CNTFRQ_read(s_6_0)
        let s_6_1: u32 = CNTFRQ_read(state, tracer, s_6_0);
        // D s_6_2: read-var t:i
        let s_6_2: i128 = fn_state.t;
        // D s_6_3: call R_set(s_6_2, s_6_1)
        let s_6_3: () = R_set(state, tracer, s_6_2, s_6_1);
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call CNTFRQ_read(s_7_0)
        let s_7_1: u32 = CNTFRQ_read(state, tracer, s_7_0);
        // D s_7_2: read-var t:i
        let s_7_2: i128 = fn_state.t;
        // D s_7_3: call R_set(s_7_2, s_7_1)
        let s_7_3: () = R_set(state, tracer, s_7_2, s_7_1);
        // N s_7_4: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #440u : u32
        let s_8_0: u32 = 440;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call ELUsingAArch32(s_8_1)
        let s_8_2: bool = ELUsingAArch32(state, tracer, s_8_1);
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b59 b9
        if s_8_3 {
            return block_59(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#106919 <= s_9_0
        fn_state.gs_106919 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#106919:u8
        let s_10_0: bool = fn_state.gs_106919;
        // N s_10_1: branch s_10_0 b58 b11
        if s_10_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#106920 <= s_11_0
        fn_state.gs_106920 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#106920:u8
        let s_12_0: bool = fn_state.gs_106920;
        // N s_12_1: branch s_12_0 b49 b13
        if s_12_0 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #440u : u32
        let s_13_0: u32 = 440;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call ELUsingAArch32(s_13_1)
        let s_13_2: bool = ELUsingAArch32(state, tracer, s_13_1);
        // N s_13_3: branch s_13_2 b48 b14
        if s_13_2 {
            return block_48(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#106921 <= s_14_0
        fn_state.gs_106921 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#106921:u8
        let s_15_0: bool = fn_state.gs_106921;
        // N s_15_1: branch s_15_0 b47 b16
        if s_15_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#106922 <= s_16_0
        fn_state.gs_106922 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#106922:u8
        let s_17_0: bool = fn_state.gs_106922;
        // N s_17_1: branch s_17_0 b30 b18
        if s_17_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call EL2Enabled(s_18_0)
        let s_18_1: bool = EL2Enabled(state, tracer, s_18_0);
        // N s_18_2: branch s_18_1 b29 b19
        if s_18_1 {
            return block_29(state, tracer, fn_state);
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
        // D s_19_1: write-var gs#106923 <= s_19_0
        fn_state.gs_106923 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#106923:u8
        let s_20_0: bool = fn_state.gs_106923;
        // N s_20_1: branch s_20_0 b28 b21
        if s_20_0 {
            return block_28(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#106924 <= s_21_0
        fn_state.gs_106924 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#106924:u8
        let s_22_0: bool = fn_state.gs_106924;
        // N s_22_1: branch s_22_0 b27 b23
        if s_22_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#106925 <= s_23_0
        fn_state.gs_106925 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#106925:u8
        let s_24_0: bool = fn_state.gs_106925;
        // N s_24_1: branch s_24_0 b26 b25
        if s_24_0 {
            return block_26(state, tracer, fn_state);
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
        // S s_25_1: call CNTFRQ_read(s_25_0)
        let s_25_1: u32 = CNTFRQ_read(state, tracer, s_25_0);
        // D s_25_2: read-var t:i
        let s_25_2: i128 = fn_state.t;
        // D s_25_3: call R_set(s_25_2, s_25_1)
        let s_25_3: () = R_set(state, tracer, s_25_2, s_25_1);
        // N s_25_4: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #3u : u8
        let s_26_0: u8 = 3;
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
        // D s_26_7: call AArch64_AArch32SystemAccessTrap(s_26_6, s_26_4)
        let s_26_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_26_6, s_26_4);
        // N s_26_8: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #12808u : u32
        let s_27_0: u32 = 12808;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_CNTHCTL_EL2_Type_EL0PCTEN(s_27_1)
        let s_27_2: bool = u_get_CNTHCTL_EL2_Type_EL0PCTEN(state, tracer, s_27_1);
        // C s_27_3: const #12808u : u32
        let s_27_3: u32 = 12808;
        // D s_27_4: read-reg s_27_3:struct
        let s_27_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_3 as isize);
            tracer.read_register(s_27_3 as isize, value);
            value
        };
        // D s_27_5: call _get_CNTHCTL_EL2_Type_EL0VCTEN(s_27_4)
        let s_27_5: bool = u_get_CNTHCTL_EL2_Type_EL0VCTEN(state, tracer, s_27_4);
        // D s_27_6: cast zx s_27_2 -> bv
        let s_27_6: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_7: cast zx s_27_5 -> bv
        let s_27_7: Bits = Bits::new(s_27_5 as u128, 1u16);
        // D s_27_8: cast reint s_27_6 -> u128
        let s_27_8: u128 = (s_27_6.value() as u128);
        // D s_27_9: size-of s_27_6
        let s_27_9: u16 = s_27_6.length();
        // D s_27_10: cast reint s_27_7 -> u128
        let s_27_10: u128 = (s_27_7.value() as u128);
        // D s_27_11: size-of s_27_7
        let s_27_11: u16 = s_27_7.length();
        // D s_27_12: lsl s_27_8 s_27_11
        let s_27_12: u128 = s_27_8 << s_27_11;
        // D s_27_13: or s_27_12 s_27_10
        let s_27_13: u128 = ((s_27_12) | (s_27_10));
        // D s_27_14: add s_27_9 s_27_11
        let s_27_14: u16 = (s_27_9 + s_27_11);
        // D s_27_15: create-bits s_27_13 s_27_14
        let s_27_15: Bits = Bits::new(s_27_13, s_27_14);
        // D s_27_16: cast reint s_27_15 -> u8
        let s_27_16: u8 = (s_27_15.value() as u8);
        // D s_27_17: cast zx s_27_16 -> bv
        let s_27_17: Bits = Bits::new(s_27_16 as u128, 2u16);
        // C s_27_18: const #0u : u8
        let s_27_18: u8 = 0;
        // C s_27_19: cast zx s_27_18 -> bv
        let s_27_19: Bits = Bits::new(s_27_18 as u128, 2u16);
        // D s_27_20: cmp-eq s_27_17 s_27_19
        let s_27_20: bool = ((s_27_17) == (s_27_19));
        // D s_27_21: write-var gs#106925 <= s_27_20
        fn_state.gs_106925 = s_27_20;
        // N s_27_22: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #102552u : u32
        let s_28_0: u32 = 102552;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_HCR_EL2_Type_E2H(s_28_1)
        let s_28_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_28_1);
        // C s_28_3: const #102552u : u32
        let s_28_3: u32 = 102552;
        // D s_28_4: read-reg s_28_3:struct
        let s_28_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_3 as isize);
            tracer.read_register(s_28_3 as isize, value);
            value
        };
        // D s_28_5: call _get_HCR_EL2_Type_TGE(s_28_4)
        let s_28_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_28_4);
        // D s_28_6: cast zx s_28_2 -> bv
        let s_28_6: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_7: cast zx s_28_5 -> bv
        let s_28_7: Bits = Bits::new(s_28_5 as u128, 1u16);
        // D s_28_8: cast reint s_28_6 -> u128
        let s_28_8: u128 = (s_28_6.value() as u128);
        // D s_28_9: size-of s_28_6
        let s_28_9: u16 = s_28_6.length();
        // D s_28_10: cast reint s_28_7 -> u128
        let s_28_10: u128 = (s_28_7.value() as u128);
        // D s_28_11: size-of s_28_7
        let s_28_11: u16 = s_28_7.length();
        // D s_28_12: lsl s_28_8 s_28_11
        let s_28_12: u128 = s_28_8 << s_28_11;
        // D s_28_13: or s_28_12 s_28_10
        let s_28_13: u128 = ((s_28_12) | (s_28_10));
        // D s_28_14: add s_28_9 s_28_11
        let s_28_14: u16 = (s_28_9 + s_28_11);
        // D s_28_15: create-bits s_28_13 s_28_14
        let s_28_15: Bits = Bits::new(s_28_13, s_28_14);
        // D s_28_16: cast reint s_28_15 -> u8
        let s_28_16: u8 = (s_28_15.value() as u8);
        // D s_28_17: cast zx s_28_16 -> bv
        let s_28_17: Bits = Bits::new(s_28_16 as u128, 2u16);
        // C s_28_18: const #3u : u8
        let s_28_18: u8 = 3;
        // C s_28_19: cast zx s_28_18 -> bv
        let s_28_19: Bits = Bits::new(s_28_18 as u128, 2u16);
        // D s_28_20: cmp-eq s_28_17 s_28_19
        let s_28_20: bool = ((s_28_17) == (s_28_19));
        // D s_28_21: write-var gs#106924 <= s_28_20
        fn_state.gs_106924 = s_28_20;
        // N s_28_22: jump b22
        return block_22(state, tracer, fn_state);
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
        // D s_29_4: write-var gs#106923 <= s_29_3
        fn_state.gs_106923 = s_29_3;
        // N s_29_5: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call EL2Enabled(s_30_0)
        let s_30_1: bool = EL2Enabled(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b46 b31
        if s_30_1 {
            return block_46(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#106926 <= s_31_0
        fn_state.gs_106926 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#106926:u8
        let s_32_0: bool = fn_state.gs_106926;
        // N s_32_1: branch s_32_0 b45 b33
        if s_32_0 {
            return block_45(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#106927 <= s_33_0
        fn_state.gs_106927 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#106927:u8
        let s_34_0: bool = fn_state.gs_106927;
        // N s_34_1: branch s_34_0 b44 b35
        if s_34_0 {
            return block_44(state, tracer, fn_state);
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
        // N s_35_2: branch s_35_1 b43 b36
        if s_35_1 {
            return block_43(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#106928 <= s_36_0
        fn_state.gs_106928 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#106928:u8
        let s_37_0: bool = fn_state.gs_106928;
        // N s_37_1: branch s_37_0 b42 b38
        if s_37_0 {
            return block_42(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#106929 <= s_38_0
        fn_state.gs_106929 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#106929:u8
        let s_39_0: bool = fn_state.gs_106929;
        // N s_39_1: branch s_39_0 b41 b40
        if s_39_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_40_0: panic
        panic!("{:?}", ());
        // N s_40_1: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: u8 = 0;
        // C s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 8u16);
        // C s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (s_41_1.value() as i128);
        // C s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: cast zx s_41_3 -> i
        let s_41_4: i128 = (i128::try_from(s_41_3).unwrap());
        // S s_41_5: call AArch32_TakeHypTrapException(s_41_4)
        let s_41_5: () = AArch32_TakeHypTrapException(state, tracer, s_41_4);
        // N s_41_6: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var __HCR_TGE:u8
        let s_42_0: bool = fn_state.u__HCR_TGE;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 1u16);
        // C s_42_2: const #1u : u8
        let s_42_2: bool = true;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: write-var gs#106929 <= s_42_4
        fn_state.gs_106929 = s_42_4;
        // N s_42_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #432u : u32
        let s_43_0: u32 = 432;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call ELUsingAArch32(s_43_1)
        let s_43_2: bool = ELUsingAArch32(state, tracer, s_43_1);
        // D s_43_3: write-var gs#106928 <= s_43_2
        fn_state.gs_106928 = s_43_2;
        // N s_43_4: jump b37
        return block_37(state, tracer, fn_state);
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
        // C s_44_5: const #432u : u32
        let s_44_5: u32 = 432;
        // D s_44_6: read-reg s_44_5:u8
        let s_44_6: u8 = {
            let value = state.read_register::<u8>(s_44_5 as isize);
            tracer.read_register(s_44_5 as isize, value);
            value
        };
        // D s_44_7: call AArch64_AArch32SystemAccessTrap(s_44_6, s_44_4)
        let s_44_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_44_6, s_44_4);
        // N s_44_8: return
        return;
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var __HCR_EL2_TGE:u8
        let s_45_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 1u16);
        // C s_45_2: const #1u : u8
        let s_45_2: bool = true;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: write-var gs#106927 <= s_45_4
        fn_state.gs_106927 = s_45_4;
        // N s_45_6: jump b34
        return block_34(state, tracer, fn_state);
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
        // D s_46_3: not s_46_2
        let s_46_3: bool = !s_46_2;
        // D s_46_4: write-var gs#106926 <= s_46_3
        fn_state.gs_106926 = s_46_3;
        // N s_46_5: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var __CNTKCTL_PL0VCTEN:u8
        let s_47_0: bool = fn_state.u__CNTKCTL_PL0VCTEN;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 1u16);
        // C s_47_2: const #0u : u8
        let s_47_2: bool = false;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: write-var gs#106922 <= s_47_4
        fn_state.gs_106922 = s_47_4;
        // N s_47_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __CNTKCTL_PL0PCTEN:u8
        let s_48_0: bool = fn_state.u__CNTKCTL_PL0PCTEN;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #0u : u8
        let s_48_2: bool = false;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#106921 <= s_48_4
        fn_state.gs_106921 = s_48_4;
        // N s_48_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #() : ()
        let s_49_0: () = ();
        // S s_49_1: call EL2Enabled(s_49_0)
        let s_49_1: bool = EL2Enabled(state, tracer, s_49_0);
        // N s_49_2: branch s_49_1 b57 b50
        if s_49_1 {
            return block_57(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#106930 <= s_50_0
        fn_state.gs_106930 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#106930:u8
        let s_51_0: bool = fn_state.gs_106930;
        // N s_51_1: branch s_51_0 b56 b52
        if s_51_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#106931 <= s_52_0
        fn_state.gs_106931 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#106931:u8
        let s_53_0: bool = fn_state.gs_106931;
        // N s_53_1: branch s_53_0 b55 b54
        if s_53_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #3u : u8
        let s_54_0: u8 = 3;
        // C s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 8u16);
        // C s_54_2: cast zx s_54_1 -> i
        let s_54_2: i128 = (s_54_1.value() as i128);
        // C s_54_3: cast reint s_54_2 -> i64
        let s_54_3: i64 = (s_54_2 as i64);
        // C s_54_4: cast zx s_54_3 -> i
        let s_54_4: i128 = (i128::try_from(s_54_3).unwrap());
        // C s_54_5: const #440u : u32
        let s_54_5: u32 = 440;
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
        // C s_55_0: const #3u : u8
        let s_55_0: u8 = 3;
        // C s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 8u16);
        // C s_55_2: cast zx s_55_1 -> i
        let s_55_2: i128 = (s_55_1.value() as i128);
        // C s_55_3: cast reint s_55_2 -> i64
        let s_55_3: i64 = (s_55_2 as i64);
        // C s_55_4: cast zx s_55_3 -> i
        let s_55_4: i128 = (i128::try_from(s_55_3).unwrap());
        // C s_55_5: const #432u : u32
        let s_55_5: u32 = 432;
        // D s_55_6: read-reg s_55_5:u8
        let s_55_6: u8 = {
            let value = state.read_register::<u8>(s_55_5 as isize);
            tracer.read_register(s_55_5 as isize, value);
            value
        };
        // D s_55_7: call AArch64_AArch32SystemAccessTrap(s_55_6, s_55_4)
        let s_55_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_55_6, s_55_4);
        // N s_55_8: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __HCR_EL2_TGE:u8
        let s_56_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #1u : u8
        let s_56_2: bool = true;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#106931 <= s_56_4
        fn_state.gs_106931 = s_56_4;
        // N s_56_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #432u : u32
        let s_57_0: u32 = 432;
        // D s_57_1: read-reg s_57_0:u8
        let s_57_1: u8 = {
            let value = state.read_register::<u8>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // D s_57_2: call ELUsingAArch32(s_57_1)
        let s_57_2: bool = ELUsingAArch32(state, tracer, s_57_1);
        // D s_57_3: not s_57_2
        let s_57_3: bool = !s_57_2;
        // D s_57_4: write-var gs#106930 <= s_57_3
        fn_state.gs_106930 = s_57_3;
        // N s_57_5: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #22056u : u32
        let s_58_0: u32 = 22056;
        // D s_58_1: read-reg s_58_0:struct
        let s_58_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call _get_CNTKCTL_EL1_Type_EL0PCTEN(s_58_1)
        let s_58_2: bool = u_get_CNTKCTL_EL1_Type_EL0PCTEN(state, tracer, s_58_1);
        // C s_58_3: const #22056u : u32
        let s_58_3: u32 = 22056;
        // D s_58_4: read-reg s_58_3:struct
        let s_58_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_3 as isize);
            tracer.read_register(s_58_3 as isize, value);
            value
        };
        // D s_58_5: call _get_CNTKCTL_EL1_Type_EL0VCTEN(s_58_4)
        let s_58_5: bool = u_get_CNTKCTL_EL1_Type_EL0VCTEN(state, tracer, s_58_4);
        // D s_58_6: cast zx s_58_2 -> bv
        let s_58_6: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_7: cast zx s_58_5 -> bv
        let s_58_7: Bits = Bits::new(s_58_5 as u128, 1u16);
        // D s_58_8: cast reint s_58_6 -> u128
        let s_58_8: u128 = (s_58_6.value() as u128);
        // D s_58_9: size-of s_58_6
        let s_58_9: u16 = s_58_6.length();
        // D s_58_10: cast reint s_58_7 -> u128
        let s_58_10: u128 = (s_58_7.value() as u128);
        // D s_58_11: size-of s_58_7
        let s_58_11: u16 = s_58_7.length();
        // D s_58_12: lsl s_58_8 s_58_11
        let s_58_12: u128 = s_58_8 << s_58_11;
        // D s_58_13: or s_58_12 s_58_10
        let s_58_13: u128 = ((s_58_12) | (s_58_10));
        // D s_58_14: add s_58_9 s_58_11
        let s_58_14: u16 = (s_58_9 + s_58_11);
        // D s_58_15: create-bits s_58_13 s_58_14
        let s_58_15: Bits = Bits::new(s_58_13, s_58_14);
        // D s_58_16: cast reint s_58_15 -> u8
        let s_58_16: u8 = (s_58_15.value() as u8);
        // D s_58_17: cast zx s_58_16 -> bv
        let s_58_17: Bits = Bits::new(s_58_16 as u128, 2u16);
        // C s_58_18: const #0u : u8
        let s_58_18: u8 = 0;
        // C s_58_19: cast zx s_58_18 -> bv
        let s_58_19: Bits = Bits::new(s_58_18 as u128, 2u16);
        // D s_58_20: cmp-eq s_58_17 s_58_19
        let s_58_20: bool = ((s_58_17) == (s_58_19));
        // D s_58_21: write-var gs#106920 <= s_58_20
        fn_state.gs_106920 = s_58_20;
        // N s_58_22: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call EL2Enabled(s_59_0)
        let s_59_1: bool = EL2Enabled(state, tracer, s_59_0);
        // N s_59_2: branch s_59_1 b62 b60
        if s_59_1 {
            return block_62(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#106918 <= s_60_0
        fn_state.gs_106918 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#106918:u8
        let s_61_0: bool = fn_state.gs_106918;
        // D s_61_1: not s_61_0
        let s_61_1: bool = !s_61_0;
        // D s_61_2: write-var gs#106919 <= s_61_1
        fn_state.gs_106919 = s_61_1;
        // N s_61_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #102552u : u32
        let s_62_0: u32 = 102552;
        // D s_62_1: read-reg s_62_0:struct
        let s_62_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: call _get_HCR_EL2_Type_E2H(s_62_1)
        let s_62_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_62_1);
        // C s_62_3: const #102552u : u32
        let s_62_3: u32 = 102552;
        // D s_62_4: read-reg s_62_3:struct
        let s_62_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_62_3 as isize);
            tracer.read_register(s_62_3 as isize, value);
            value
        };
        // D s_62_5: call _get_HCR_EL2_Type_TGE(s_62_4)
        let s_62_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_62_4);
        // D s_62_6: cast zx s_62_2 -> bv
        let s_62_6: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_7: cast zx s_62_5 -> bv
        let s_62_7: Bits = Bits::new(s_62_5 as u128, 1u16);
        // D s_62_8: cast reint s_62_6 -> u128
        let s_62_8: u128 = (s_62_6.value() as u128);
        // D s_62_9: size-of s_62_6
        let s_62_9: u16 = s_62_6.length();
        // D s_62_10: cast reint s_62_7 -> u128
        let s_62_10: u128 = (s_62_7.value() as u128);
        // D s_62_11: size-of s_62_7
        let s_62_11: u16 = s_62_7.length();
        // D s_62_12: lsl s_62_8 s_62_11
        let s_62_12: u128 = s_62_8 << s_62_11;
        // D s_62_13: or s_62_12 s_62_10
        let s_62_13: u128 = ((s_62_12) | (s_62_10));
        // D s_62_14: add s_62_9 s_62_11
        let s_62_14: u16 = (s_62_9 + s_62_11);
        // D s_62_15: create-bits s_62_13 s_62_14
        let s_62_15: Bits = Bits::new(s_62_13, s_62_14);
        // D s_62_16: cast reint s_62_15 -> u8
        let s_62_16: u8 = (s_62_15.value() as u8);
        // D s_62_17: cast zx s_62_16 -> bv
        let s_62_17: Bits = Bits::new(s_62_16 as u128, 2u16);
        // C s_62_18: const #3u : u8
        let s_62_18: u8 = 3;
        // C s_62_19: cast zx s_62_18 -> bv
        let s_62_19: Bits = Bits::new(s_62_18 as u128, 2u16);
        // D s_62_20: cmp-eq s_62_17 s_62_19
        let s_62_20: bool = ((s_62_17) == (s_62_19));
        // D s_62_21: write-var gs#106918 <= s_62_20
        fn_state.gs_106918 = s_62_20;
        // N s_62_22: jump b61
        return block_61(state, tracer, fn_state);
    }
}
