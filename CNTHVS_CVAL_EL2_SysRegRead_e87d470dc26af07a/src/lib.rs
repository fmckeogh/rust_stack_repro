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
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_EEL2::*;
use IsCurrentSecurityState::*;
use u_get_HCR_EL2_Type_NV::*;
use EL2Enabled::*;
use common::*;
pub fn CNTHVS_CVAL_EL2_SysRegRead_e87d470dc26af07a<T: Tracer>(
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
        gs_57868: bool,
        u__PSTATE_EL: u8,
        u__HCR_EL2_NV: bool,
        u__SCR_EL3_EEL2: bool,
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
        // D s_0_5: call _get_HCR_EL2_Type_NV(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_NV <= s_0_5
        fn_state.u__HCR_EL2_NV = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_EEL2(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_EEL2(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_EEL2 <= s_0_9
        fn_state.u__SCR_EL3_EEL2 = s_0_9;
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
        // N s_0_17: branch s_0_16 b19 b1
        if s_0_16 {
            return block_19(state, tracer, fn_state);
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
        // D s_5_0: read-var __SCR_EL3_EEL2:u8
        let s_5_0: bool = fn_state.u__SCR_EL3_EEL2;
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
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #10064u : u32
        let s_6_1: u32 = 10064;
        // D s_6_2: read-reg s_6_1:u64
        let s_6_2: u64 = {
            let value = state.read_register::<u64>(s_6_1 as isize);
            tracer.read_register(s_6_1 as isize, value);
            value
        };
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 64u16);
        // D s_6_4: read-var t:i
        let s_6_4: i128 = fn_state.t;
        // D s_6_5: call X_set(s_6_4, s_6_0, s_6_3)
        let s_6_5: () = X_set(state, tracer, s_6_4, s_6_0, s_6_3);
        // N s_6_6: return
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
        // C s_8_0: const #3u : u32
        let s_8_0: u32 = 3;
        // S s_8_1: call IsCurrentSecurityState(s_8_0)
        let s_8_1: bool = IsCurrentSecurityState(state, tracer, s_8_0);
        // S s_8_2: not s_8_1
        let s_8_2: bool = !s_8_1;
        // N s_8_3: branch s_8_2 b10 b9
        if s_8_2 {
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
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // C s_9_1: const #10064u : u32
        let s_9_1: u32 = 10064;
        // D s_9_2: read-reg s_9_1:u64
        let s_9_2: u64 = {
            let value = state.read_register::<u64>(s_9_1 as isize);
            tracer.read_register(s_9_1 as isize, value);
            value
        };
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 64u16);
        // D s_9_4: read-var t:i
        let s_9_4: i128 = fn_state.t;
        // D s_9_5: call X_set(s_9_4, s_9_0, s_9_3)
        let s_9_5: () = X_set(state, tracer, s_9_4, s_9_0, s_9_3);
        // N s_9_6: return
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
        // C s_11_0: const #3u : u32
        let s_11_0: u32 = 3;
        // S s_11_1: call IsCurrentSecurityState(s_11_0)
        let s_11_1: bool = IsCurrentSecurityState(state, tracer, s_11_0);
        // S s_11_2: not s_11_1
        let s_11_2: bool = !s_11_1;
        // N s_11_3: branch s_11_2 b18 b12
        if s_11_2 {
            return block_18(state, tracer, fn_state);
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
        // N s_12_2: branch s_12_1 b17 b13
        if s_12_1 {
            return block_17(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#57868 <= s_13_0
        fn_state.gs_57868 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#57868:u8
        let s_14_0: bool = fn_state.gs_57868;
        // N s_14_1: branch s_14_0 b16 b15
        if s_14_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #24u : u8
        let s_16_0: u8 = 24;
        // C s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 8u16);
        // C s_16_2: cast zx s_16_1 -> i
        let s_16_2: i128 = (s_16_1.value() as i128);
        // C s_16_3: cast reint s_16_2 -> i64
        let s_16_3: i64 = (s_16_2 as i64);
        // C s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // C s_16_5: const #432u : u32
        let s_16_5: u32 = 432;
        // D s_16_6: read-reg s_16_5:u8
        let s_16_6: u8 = {
            let value = state.read_register::<u8>(s_16_5 as isize);
            tracer.read_register(s_16_5 as isize, value);
            value
        };
        // D s_16_7: call AArch64_SystemAccessTrap(s_16_6, s_16_4)
        let s_16_7: () = AArch64_SystemAccessTrap(state, tracer, s_16_6, s_16_4);
        // N s_16_8: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var __HCR_EL2_NV:u8
        let s_17_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 1u16);
        // C s_17_2: const #1u : u8
        let s_17_2: bool = true;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: write-var gs#57868 <= s_17_4
        fn_state.gs_57868 = s_17_4;
        // N s_17_6: jump b14
        return block_14(state, tracer, fn_state);
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
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
}
