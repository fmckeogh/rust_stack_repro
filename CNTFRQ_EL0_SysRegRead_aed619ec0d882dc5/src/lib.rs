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
use X_set::*;
use u_get_CNTHCTL_EL2_Type_EL0PCTEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_CNTKCTL_EL1_Type_EL0PCTEN::*;
use u_get_CNTKCTL_EL1_Type_EL0VCTEN::*;
use AArch64_SystemAccessTrap::*;
use u_get_CNTHCTL_EL2_Type_EL0VCTEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u__get_CNTFRQ_EL0::*;
use common::*;
pub fn CNTFRQ_EL0_SysRegRead_aed619ec0d882dc5<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_57739: bool,
        u__HCR_EL2_TGE: bool,
        gs_57736: bool,
        gs_57737: bool,
        gs_57738: bool,
        u__PSTATE_EL: u8,
        gs_57741: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
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
        // D s_0_7: read-var __PSTATE_EL:u8
        let s_0_7: u8 = fn_state.u__PSTATE_EL;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 2u16);
        // C s_0_9: const #448u : u32
        let s_0_9: u32 = 448;
        // D s_0_10: read-reg s_0_9:u8
        let s_0_10: u8 = {
            let value = state.read_register::<u8>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // D s_0_12: cmp-eq s_0_8 s_0_11
        let s_0_12: bool = ((s_0_8) == (s_0_11));
        // N s_0_13: branch s_0_12 b8 b1
        if s_0_12 {
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
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // C s_5_1: const #22912u : u32
        let s_5_1: u32 = 22912;
        // D s_5_2: read-reg s_5_1:u64
        let s_5_2: u64 = {
            let value = state.read_register::<u64>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_CNTFRQ_EL0(s_5_2)
        let s_5_3: u64 = u__get_CNTFRQ_EL0(state, tracer, s_5_2);
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 64u16);
        // D s_5_5: read-var t:i
        let s_5_5: i128 = fn_state.t;
        // D s_5_6: call X_set(s_5_5, s_5_0, s_5_4)
        let s_5_6: () = X_set(state, tracer, s_5_5, s_5_0, s_5_4);
        // N s_5_7: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #22912u : u32
        let s_6_1: u32 = 22912;
        // D s_6_2: read-reg s_6_1:u64
        let s_6_2: u64 = {
            let value = state.read_register::<u64>(s_6_1 as isize);
            tracer.read_register(s_6_1 as isize, value);
            value
        };
        // D s_6_3: call __get_CNTFRQ_EL0(s_6_2)
        let s_6_3: u64 = u__get_CNTFRQ_EL0(state, tracer, s_6_2);
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 64u16);
        // D s_6_5: read-var t:i
        let s_6_5: i128 = fn_state.t;
        // D s_6_6: call X_set(s_6_5, s_6_0, s_6_4)
        let s_6_6: () = X_set(state, tracer, s_6_5, s_6_0, s_6_4);
        // N s_6_7: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #22912u : u32
        let s_7_1: u32 = 22912;
        // D s_7_2: read-reg s_7_1:u64
        let s_7_2: u64 = {
            let value = state.read_register::<u64>(s_7_1 as isize);
            tracer.read_register(s_7_1 as isize, value);
            value
        };
        // D s_7_3: call __get_CNTFRQ_EL0(s_7_2)
        let s_7_3: u64 = u__get_CNTFRQ_EL0(state, tracer, s_7_2);
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 64u16);
        // D s_7_5: read-var t:i
        let s_7_5: i128 = fn_state.t;
        // D s_7_6: call X_set(s_7_5, s_7_0, s_7_4)
        let s_7_6: () = X_set(state, tracer, s_7_5, s_7_0, s_7_4);
        // N s_7_7: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call EL2Enabled(s_8_0)
        let s_8_1: bool = EL2Enabled(state, tracer, s_8_0);
        // N s_8_2: branch s_8_1 b29 b9
        if s_8_1 {
            return block_29(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#57736 <= s_9_0
        fn_state.gs_57736 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#57736:u8
        let s_10_0: bool = fn_state.gs_57736;
        // D s_10_1: not s_10_0
        let s_10_1: bool = !s_10_0;
        // N s_10_2: branch s_10_1 b28 b11
        if s_10_1 {
            return block_28(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#57737 <= s_11_0
        fn_state.gs_57737 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#57737:u8
        let s_12_0: bool = fn_state.gs_57737;
        // N s_12_1: branch s_12_0 b22 b13
        if s_12_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call EL2Enabled(s_13_0)
        let s_13_1: bool = EL2Enabled(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b21 b14
        if s_13_1 {
            return block_21(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#57738 <= s_14_0
        fn_state.gs_57738 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#57738:u8
        let s_15_0: bool = fn_state.gs_57738;
        // N s_15_1: branch s_15_0 b20 b16
        if s_15_0 {
            return block_20(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#57739 <= s_16_0
        fn_state.gs_57739 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#57739:u8
        let s_17_0: bool = fn_state.gs_57739;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #64s : i64
        let s_18_0: i64 = 64;
        // C s_18_1: const #22912u : u32
        let s_18_1: u32 = 22912;
        // D s_18_2: read-reg s_18_1:u64
        let s_18_2: u64 = {
            let value = state.read_register::<u64>(s_18_1 as isize);
            tracer.read_register(s_18_1 as isize, value);
            value
        };
        // D s_18_3: call __get_CNTFRQ_EL0(s_18_2)
        let s_18_3: u64 = u__get_CNTFRQ_EL0(state, tracer, s_18_2);
        // D s_18_4: cast zx s_18_3 -> bv
        let s_18_4: Bits = Bits::new(s_18_3 as u128, 64u16);
        // D s_18_5: read-var t:i
        let s_18_5: i128 = fn_state.t;
        // D s_18_6: call X_set(s_18_5, s_18_0, s_18_4)
        let s_18_6: () = X_set(state, tracer, s_18_5, s_18_0, s_18_4);
        // N s_18_7: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #24u : u8
        let s_19_0: u8 = 24;
        // C s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 8u16);
        // C s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (s_19_1.value() as i128);
        // C s_19_3: cast reint s_19_2 -> i64
        let s_19_3: i64 = (s_19_2 as i64);
        // C s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (i128::try_from(s_19_3).unwrap());
        // C s_19_5: const #432u : u32
        let s_19_5: u32 = 432;
        // D s_19_6: read-reg s_19_5:u8
        let s_19_6: u8 = {
            let value = state.read_register::<u8>(s_19_5 as isize);
            tracer.read_register(s_19_5 as isize, value);
            value
        };
        // D s_19_7: call AArch64_SystemAccessTrap(s_19_6, s_19_4)
        let s_19_7: () = AArch64_SystemAccessTrap(state, tracer, s_19_6, s_19_4);
        // N s_19_8: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #12808u : u32
        let s_20_0: u32 = 12808;
        // D s_20_1: read-reg s_20_0:struct
        let s_20_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call _get_CNTHCTL_EL2_Type_EL0PCTEN(s_20_1)
        let s_20_2: bool = u_get_CNTHCTL_EL2_Type_EL0PCTEN(state, tracer, s_20_1);
        // C s_20_3: const #12808u : u32
        let s_20_3: u32 = 12808;
        // D s_20_4: read-reg s_20_3:struct
        let s_20_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_3 as isize);
            tracer.read_register(s_20_3 as isize, value);
            value
        };
        // D s_20_5: call _get_CNTHCTL_EL2_Type_EL0VCTEN(s_20_4)
        let s_20_5: bool = u_get_CNTHCTL_EL2_Type_EL0VCTEN(state, tracer, s_20_4);
        // D s_20_6: cast zx s_20_2 -> bv
        let s_20_6: Bits = Bits::new(s_20_2 as u128, 1u16);
        // D s_20_7: cast zx s_20_5 -> bv
        let s_20_7: Bits = Bits::new(s_20_5 as u128, 1u16);
        // D s_20_8: cast reint s_20_6 -> u128
        let s_20_8: u128 = (s_20_6.value() as u128);
        // D s_20_9: size-of s_20_6
        let s_20_9: u16 = s_20_6.length();
        // D s_20_10: cast reint s_20_7 -> u128
        let s_20_10: u128 = (s_20_7.value() as u128);
        // D s_20_11: size-of s_20_7
        let s_20_11: u16 = s_20_7.length();
        // D s_20_12: lsl s_20_8 s_20_11
        let s_20_12: u128 = s_20_8 << s_20_11;
        // D s_20_13: or s_20_12 s_20_10
        let s_20_13: u128 = ((s_20_12) | (s_20_10));
        // D s_20_14: add s_20_9 s_20_11
        let s_20_14: u16 = (s_20_9 + s_20_11);
        // D s_20_15: create-bits s_20_13 s_20_14
        let s_20_15: Bits = Bits::new(s_20_13, s_20_14);
        // D s_20_16: cast reint s_20_15 -> u8
        let s_20_16: u8 = (s_20_15.value() as u8);
        // D s_20_17: cast zx s_20_16 -> bv
        let s_20_17: Bits = Bits::new(s_20_16 as u128, 2u16);
        // C s_20_18: const #0u : u8
        let s_20_18: u8 = 0;
        // C s_20_19: cast zx s_20_18 -> bv
        let s_20_19: Bits = Bits::new(s_20_18 as u128, 2u16);
        // D s_20_20: cmp-eq s_20_17 s_20_19
        let s_20_20: bool = ((s_20_17) == (s_20_19));
        // D s_20_21: write-var gs#57739 <= s_20_20
        fn_state.gs_57739 = s_20_20;
        // N s_20_22: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #102552u : u32
        let s_21_0: u32 = 102552;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_HCR_EL2_Type_E2H(s_21_1)
        let s_21_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_21_1);
        // C s_21_3: const #102552u : u32
        let s_21_3: u32 = 102552;
        // D s_21_4: read-reg s_21_3:struct
        let s_21_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_3 as isize);
            tracer.read_register(s_21_3 as isize, value);
            value
        };
        // D s_21_5: call _get_HCR_EL2_Type_TGE(s_21_4)
        let s_21_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_21_4);
        // D s_21_6: cast zx s_21_2 -> bv
        let s_21_6: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_7: cast zx s_21_5 -> bv
        let s_21_7: Bits = Bits::new(s_21_5 as u128, 1u16);
        // D s_21_8: cast reint s_21_6 -> u128
        let s_21_8: u128 = (s_21_6.value() as u128);
        // D s_21_9: size-of s_21_6
        let s_21_9: u16 = s_21_6.length();
        // D s_21_10: cast reint s_21_7 -> u128
        let s_21_10: u128 = (s_21_7.value() as u128);
        // D s_21_11: size-of s_21_7
        let s_21_11: u16 = s_21_7.length();
        // D s_21_12: lsl s_21_8 s_21_11
        let s_21_12: u128 = s_21_8 << s_21_11;
        // D s_21_13: or s_21_12 s_21_10
        let s_21_13: u128 = ((s_21_12) | (s_21_10));
        // D s_21_14: add s_21_9 s_21_11
        let s_21_14: u16 = (s_21_9 + s_21_11);
        // D s_21_15: create-bits s_21_13 s_21_14
        let s_21_15: Bits = Bits::new(s_21_13, s_21_14);
        // D s_21_16: cast reint s_21_15 -> u8
        let s_21_16: u8 = (s_21_15.value() as u8);
        // D s_21_17: cast zx s_21_16 -> bv
        let s_21_17: Bits = Bits::new(s_21_16 as u128, 2u16);
        // C s_21_18: const #3u : u8
        let s_21_18: u8 = 3;
        // C s_21_19: cast zx s_21_18 -> bv
        let s_21_19: Bits = Bits::new(s_21_18 as u128, 2u16);
        // D s_21_20: cmp-eq s_21_17 s_21_19
        let s_21_20: bool = ((s_21_17) == (s_21_19));
        // D s_21_21: write-var gs#57738 <= s_21_20
        fn_state.gs_57738 = s_21_20;
        // N s_21_22: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call EL2Enabled(s_22_0)
        let s_22_1: bool = EL2Enabled(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b27 b23
        if s_22_1 {
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
        // D s_23_1: write-var gs#57741 <= s_23_0
        fn_state.gs_57741 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#57741:u8
        let s_24_0: bool = fn_state.gs_57741;
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
        // C s_25_0: const #24u : u8
        let s_25_0: u8 = 24;
        // C s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 8u16);
        // C s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (s_25_1.value() as i128);
        // C s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #440u : u32
        let s_25_5: u32 = 440;
        // D s_25_6: read-reg s_25_5:u8
        let s_25_6: u8 = {
            let value = state.read_register::<u8>(s_25_5 as isize);
            tracer.read_register(s_25_5 as isize, value);
            value
        };
        // D s_25_7: call AArch64_SystemAccessTrap(s_25_6, s_25_4)
        let s_25_7: () = AArch64_SystemAccessTrap(state, tracer, s_25_6, s_25_4);
        // N s_25_8: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #24u : u8
        let s_26_0: u8 = 24;
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
        // D s_26_7: call AArch64_SystemAccessTrap(s_26_6, s_26_4)
        let s_26_7: () = AArch64_SystemAccessTrap(state, tracer, s_26_6, s_26_4);
        // N s_26_8: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __HCR_EL2_TGE:u8
        let s_27_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #1u : u8
        let s_27_2: bool = true;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#57741 <= s_27_4
        fn_state.gs_57741 = s_27_4;
        // N s_27_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #22056u : u32
        let s_28_0: u32 = 22056;
        // D s_28_1: read-reg s_28_0:struct
        let s_28_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call _get_CNTKCTL_EL1_Type_EL0PCTEN(s_28_1)
        let s_28_2: bool = u_get_CNTKCTL_EL1_Type_EL0PCTEN(state, tracer, s_28_1);
        // C s_28_3: const #22056u : u32
        let s_28_3: u32 = 22056;
        // D s_28_4: read-reg s_28_3:struct
        let s_28_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_28_3 as isize);
            tracer.read_register(s_28_3 as isize, value);
            value
        };
        // D s_28_5: call _get_CNTKCTL_EL1_Type_EL0VCTEN(s_28_4)
        let s_28_5: bool = u_get_CNTKCTL_EL1_Type_EL0VCTEN(state, tracer, s_28_4);
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
        // C s_28_18: const #0u : u8
        let s_28_18: u8 = 0;
        // C s_28_19: cast zx s_28_18 -> bv
        let s_28_19: Bits = Bits::new(s_28_18 as u128, 2u16);
        // D s_28_20: cmp-eq s_28_17 s_28_19
        let s_28_20: bool = ((s_28_17) == (s_28_19));
        // D s_28_21: write-var gs#57737 <= s_28_20
        fn_state.gs_57737 = s_28_20;
        // N s_28_22: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #102552u : u32
        let s_29_0: u32 = 102552;
        // D s_29_1: read-reg s_29_0:struct
        let s_29_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_0 as isize);
            tracer.read_register(s_29_0 as isize, value);
            value
        };
        // D s_29_2: call _get_HCR_EL2_Type_E2H(s_29_1)
        let s_29_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_29_1);
        // C s_29_3: const #102552u : u32
        let s_29_3: u32 = 102552;
        // D s_29_4: read-reg s_29_3:struct
        let s_29_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_29_3 as isize);
            tracer.read_register(s_29_3 as isize, value);
            value
        };
        // D s_29_5: call _get_HCR_EL2_Type_TGE(s_29_4)
        let s_29_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_29_4);
        // D s_29_6: cast zx s_29_2 -> bv
        let s_29_6: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_7: cast zx s_29_5 -> bv
        let s_29_7: Bits = Bits::new(s_29_5 as u128, 1u16);
        // D s_29_8: cast reint s_29_6 -> u128
        let s_29_8: u128 = (s_29_6.value() as u128);
        // D s_29_9: size-of s_29_6
        let s_29_9: u16 = s_29_6.length();
        // D s_29_10: cast reint s_29_7 -> u128
        let s_29_10: u128 = (s_29_7.value() as u128);
        // D s_29_11: size-of s_29_7
        let s_29_11: u16 = s_29_7.length();
        // D s_29_12: lsl s_29_8 s_29_11
        let s_29_12: u128 = s_29_8 << s_29_11;
        // D s_29_13: or s_29_12 s_29_10
        let s_29_13: u128 = ((s_29_12) | (s_29_10));
        // D s_29_14: add s_29_9 s_29_11
        let s_29_14: u16 = (s_29_9 + s_29_11);
        // D s_29_15: create-bits s_29_13 s_29_14
        let s_29_15: Bits = Bits::new(s_29_13, s_29_14);
        // D s_29_16: cast reint s_29_15 -> u8
        let s_29_16: u8 = (s_29_15.value() as u8);
        // D s_29_17: cast zx s_29_16 -> bv
        let s_29_17: Bits = Bits::new(s_29_16 as u128, 2u16);
        // C s_29_18: const #3u : u8
        let s_29_18: u8 = 3;
        // C s_29_19: cast zx s_29_18 -> bv
        let s_29_19: Bits = Bits::new(s_29_18 as u128, 2u16);
        // D s_29_20: cmp-eq s_29_17 s_29_19
        let s_29_20: bool = ((s_29_17) == (s_29_19));
        // D s_29_21: write-var gs#57736 <= s_29_20
        fn_state.gs_57736 = s_29_20;
        // N s_29_22: jump b10
        return block_10(state, tracer, fn_state);
    }
}
