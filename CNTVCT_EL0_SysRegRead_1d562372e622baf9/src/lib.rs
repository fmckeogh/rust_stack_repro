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
use u_get_CNTHCTL_EL2_Type_EL1TVCT::*;
use u_get_HCR_EL2_Type_E2H::*;
use ELUsingAArch32::*;
use u_get_CNTKCTL_EL1_Type_EL0VCTEN::*;
use u_get_CNTHCTL_EL2_Type_EL0VCTEN::*;
use CNTVOFF_read::*;
use u_get_HCR_EL2_Type_TGE::*;
use PhysicalCountInt::*;
use EL2Enabled::*;
use AArch64_SystemAccessTrap::*;
use common::*;
pub fn CNTVCT_EL0_SysRegRead_1d562372e622baf9<T: Tracer>(
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
        u__CNTHCTL_EL2_EL0VCTEN: bool,
        gs_58133: bool,
        u__HCR_EL2_E2H: bool,
        gs_58130: bool,
        u__HCR_EL2_TGE: bool,
        gs_58117: bool,
        gs_58132: bool,
        gs_58137: bool,
        gs_58134: bool,
        gs_58127: bool,
        gs_58128: bool,
        gs_58124: bool,
        gs_58118: bool,
        gs_58129: bool,
        u__PSTATE_EL: u8,
        u__CNTHCTL_EL2_EL1TVCT: bool,
        u__CNTKCTL_EL1_EL0VCTEN: bool,
        gs_58131: bool,
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
        // C s_0_3: const #22056u : u32
        let s_0_3: u32 = 22056;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_CNTKCTL_EL1_Type_EL0VCTEN(s_0_4)
        let s_0_5: bool = u_get_CNTKCTL_EL1_Type_EL0VCTEN(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTKCTL_EL1_EL0VCTEN <= s_0_5
        fn_state.u__CNTKCTL_EL1_EL0VCTEN = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_TGE(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_TGE <= s_0_9
        fn_state.u__HCR_EL2_TGE = s_0_9;
        // C s_0_11: const #12808u : u32
        let s_0_11: u32 = 12808;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_CNTHCTL_EL2_Type_EL0VCTEN(s_0_12)
        let s_0_13: bool = u_get_CNTHCTL_EL2_Type_EL0VCTEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTHCTL_EL2_EL0VCTEN <= s_0_13
        fn_state.u__CNTHCTL_EL2_EL0VCTEN = s_0_13;
        // C s_0_15: const #12808u : u32
        let s_0_15: u32 = 12808;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_CNTHCTL_EL2_Type_EL1TVCT(s_0_16)
        let s_0_17: bool = u_get_CNTHCTL_EL2_Type_EL1TVCT(state, tracer, s_0_16);
        // D s_0_18: write-var __CNTHCTL_EL2_EL1TVCT <= s_0_17
        fn_state.u__CNTHCTL_EL2_EL1TVCT = s_0_17;
        // C s_0_19: const #102552u : u32
        let s_0_19: u32 = 102552;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCR_EL2_Type_E2H(s_0_20)
        let s_0_21: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_EL2_E2H <= s_0_21
        fn_state.u__HCR_EL2_E2H = s_0_21;
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
        // N s_0_29: branch s_0_28 b27 b1
        if s_0_28 {
            return block_27(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b16 b3
        if s_2_5 {
            return block_16(state, tracer, fn_state);
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
        // C s_5_0: const #432u : u32
        let s_5_0: u32 = 432;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
        // D s_5_3: cmp-lt s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) < (s_5_2));
        // N s_5_4: branch s_5_3 b15 b6
        if s_5_3 {
            return block_15(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#58117 <= s_6_0
        fn_state.gs_58117 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#58117:u8
        let s_7_0: bool = fn_state.gs_58117;
        // N s_7_1: branch s_7_0 b14 b8
        if s_7_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #432u : u32
        let s_8_0: u32 = 432;
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
        // D s_9_1: write-var gs#58118 <= s_9_0
        fn_state.gs_58118 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#58118:u8
        let s_10_0: bool = fn_state.gs_58118;
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
        // C s_11_0: const #64s : i64
        let s_11_0: i64 = 64;
        // C s_11_1: const #() : ()
        let s_11_1: () = ();
        // S s_11_2: call PhysicalCountInt(s_11_1)
        let s_11_2: u64 = PhysicalCountInt(state, tracer, s_11_1);
        // S s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 64u16);
        // D s_11_4: read-var t:i
        let s_11_4: i128 = fn_state.t;
        // D s_11_5: call X_set(s_11_4, s_11_0, s_11_3)
        let s_11_5: () = X_set(state, tracer, s_11_4, s_11_0, s_11_3);
        // N s_11_6: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #64s : i64
        let s_12_0: i64 = 64;
        // C s_12_1: const #() : ()
        let s_12_1: () = ();
        // S s_12_2: call PhysicalCountInt(s_12_1)
        let s_12_2: u64 = PhysicalCountInt(state, tracer, s_12_1);
        // C s_12_3: const #() : ()
        let s_12_3: () = ();
        // S s_12_4: call CNTVOFF_read(s_12_3)
        let s_12_4: u64 = CNTVOFF_read(state, tracer, s_12_3);
        // S s_12_5: cast zx s_12_2 -> bv
        let s_12_5: Bits = Bits::new(s_12_2 as u128, 64u16);
        // S s_12_6: cast zx s_12_4 -> bv
        let s_12_6: Bits = Bits::new(s_12_4 as u128, 64u16);
        // S s_12_7: sub s_12_5 s_12_6
        let s_12_7: Bits = ((s_12_5) - (s_12_6));
        // S s_12_8: cast reint s_12_7 -> u64
        let s_12_8: u64 = (s_12_7.value() as u64);
        // S s_12_9: cast zx s_12_8 -> bv
        let s_12_9: Bits = Bits::new(s_12_8 as u128, 64u16);
        // D s_12_10: read-var t:i
        let s_12_10: i128 = fn_state.t;
        // D s_12_11: call X_set(s_12_10, s_12_0, s_12_9)
        let s_12_11: () = X_set(state, tracer, s_12_10, s_12_0, s_12_9);
        // N s_12_12: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #432u : u32
        let s_13_0: u32 = 432;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call ELUsingAArch32(s_13_1)
        let s_13_2: bool = ELUsingAArch32(state, tracer, s_13_1);
        // D s_13_3: write-var gs#58118 <= s_13_2
        fn_state.gs_58118 = s_13_2;
        // N s_13_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #64s : i64
        let s_14_0: i64 = 64;
        // C s_14_1: const #() : ()
        let s_14_1: () = ();
        // S s_14_2: call PhysicalCountInt(s_14_1)
        let s_14_2: u64 = PhysicalCountInt(state, tracer, s_14_1);
        // S s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 64u16);
        // C s_14_4: const #22400u : u32
        let s_14_4: u32 = 22400;
        // D s_14_5: read-reg s_14_4:u64
        let s_14_5: u64 = {
            let value = state.read_register::<u64>(s_14_4 as isize);
            tracer.read_register(s_14_4 as isize, value);
            value
        };
        // D s_14_6: cast zx s_14_5 -> bv
        let s_14_6: Bits = Bits::new(s_14_5 as u128, 64u16);
        // D s_14_7: sub s_14_3 s_14_6
        let s_14_7: Bits = ((s_14_3) - (s_14_6));
        // D s_14_8: cast reint s_14_7 -> u64
        let s_14_8: u64 = (s_14_7.value() as u64);
        // D s_14_9: cast zx s_14_8 -> bv
        let s_14_9: Bits = Bits::new(s_14_8 as u128, 64u16);
        // D s_14_10: read-var t:i
        let s_14_10: i128 = fn_state.t;
        // D s_14_11: call X_set(s_14_10, s_14_0, s_14_9)
        let s_14_11: () = X_set(state, tracer, s_14_10, s_14_0, s_14_9);
        // N s_14_12: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #432u : u32
        let s_15_0: u32 = 432;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call ELUsingAArch32(s_15_1)
        let s_15_2: bool = ELUsingAArch32(state, tracer, s_15_1);
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // D s_15_4: write-var gs#58117 <= s_15_3
        fn_state.gs_58117 = s_15_3;
        // N s_15_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var __HCR_EL2_E2H:u8
        let s_16_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #0u : u8
        let s_16_2: bool = false;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // N s_16_5: branch s_16_4 b18 b17
        if s_16_4 {
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
        // C s_17_0: const #64s : i64
        let s_17_0: i64 = 64;
        // C s_17_1: const #() : ()
        let s_17_1: () = ();
        // S s_17_2: call PhysicalCountInt(s_17_1)
        let s_17_2: u64 = PhysicalCountInt(state, tracer, s_17_1);
        // S s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 64u16);
        // D s_17_4: read-var t:i
        let s_17_4: i128 = fn_state.t;
        // D s_17_5: call X_set(s_17_4, s_17_0, s_17_3)
        let s_17_5: () = X_set(state, tracer, s_17_4, s_17_0, s_17_3);
        // N s_17_6: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #64s : i64
        let s_18_0: i64 = 64;
        // C s_18_1: const #() : ()
        let s_18_1: () = ();
        // S s_18_2: call PhysicalCountInt(s_18_1)
        let s_18_2: u64 = PhysicalCountInt(state, tracer, s_18_1);
        // S s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 64u16);
        // C s_18_4: const #22400u : u32
        let s_18_4: u32 = 22400;
        // D s_18_5: read-reg s_18_4:u64
        let s_18_5: u64 = {
            let value = state.read_register::<u64>(s_18_4 as isize);
            tracer.read_register(s_18_4 as isize, value);
            value
        };
        // D s_18_6: cast zx s_18_5 -> bv
        let s_18_6: Bits = Bits::new(s_18_5 as u128, 64u16);
        // D s_18_7: sub s_18_3 s_18_6
        let s_18_7: Bits = ((s_18_3) - (s_18_6));
        // D s_18_8: cast reint s_18_7 -> u64
        let s_18_8: u64 = (s_18_7.value() as u64);
        // D s_18_9: cast zx s_18_8 -> bv
        let s_18_9: Bits = Bits::new(s_18_8 as u128, 64u16);
        // D s_18_10: read-var t:i
        let s_18_10: i128 = fn_state.t;
        // D s_18_11: call X_set(s_18_10, s_18_0, s_18_9)
        let s_18_11: () = X_set(state, tracer, s_18_10, s_18_0, s_18_9);
        // N s_18_12: return
        return;
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
        // N s_19_2: branch s_19_1 b26 b20
        if s_19_1 {
            return block_26(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#58124 <= s_20_0
        fn_state.gs_58124 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#58124:u8
        let s_21_0: bool = fn_state.gs_58124;
        // N s_21_1: branch s_21_0 b25 b22
        if s_21_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #432u : u32
        let s_22_0: u32 = 432;
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
        // N s_22_4: branch s_22_3 b24 b23
        if s_22_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #64s : i64
        let s_23_0: i64 = 64;
        // C s_23_1: const #() : ()
        let s_23_1: () = ();
        // S s_23_2: call PhysicalCountInt(s_23_1)
        let s_23_2: u64 = PhysicalCountInt(state, tracer, s_23_1);
        // S s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 64u16);
        // D s_23_4: read-var t:i
        let s_23_4: i128 = fn_state.t;
        // D s_23_5: call X_set(s_23_4, s_23_0, s_23_3)
        let s_23_5: () = X_set(state, tracer, s_23_4, s_23_0, s_23_3);
        // N s_23_6: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #64s : i64
        let s_24_0: i64 = 64;
        // C s_24_1: const #() : ()
        let s_24_1: () = ();
        // S s_24_2: call PhysicalCountInt(s_24_1)
        let s_24_2: u64 = PhysicalCountInt(state, tracer, s_24_1);
        // S s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 64u16);
        // C s_24_4: const #22400u : u32
        let s_24_4: u32 = 22400;
        // D s_24_5: read-reg s_24_4:u64
        let s_24_5: u64 = {
            let value = state.read_register::<u64>(s_24_4 as isize);
            tracer.read_register(s_24_4 as isize, value);
            value
        };
        // D s_24_6: cast zx s_24_5 -> bv
        let s_24_6: Bits = Bits::new(s_24_5 as u128, 64u16);
        // D s_24_7: sub s_24_3 s_24_6
        let s_24_7: Bits = ((s_24_3) - (s_24_6));
        // D s_24_8: cast reint s_24_7 -> u64
        let s_24_8: u64 = (s_24_7.value() as u64);
        // D s_24_9: cast zx s_24_8 -> bv
        let s_24_9: Bits = Bits::new(s_24_8 as u128, 64u16);
        // D s_24_10: read-var t:i
        let s_24_10: i128 = fn_state.t;
        // D s_24_11: call X_set(s_24_10, s_24_0, s_24_9)
        let s_24_11: () = X_set(state, tracer, s_24_10, s_24_0, s_24_9);
        // N s_24_12: return
        return;
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
        // C s_25_5: const #432u : u32
        let s_25_5: u32 = 432;
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
        // D s_26_0: read-var __CNTHCTL_EL2_EL1TVCT:u8
        let s_26_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVCT;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#58124 <= s_26_4
        fn_state.gs_58124 = s_26_4;
        // N s_26_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call EL2Enabled(s_27_0)
        let s_27_1: bool = EL2Enabled(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b64 b28
        if s_27_1 {
            return block_64(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#58127 <= s_28_0
        fn_state.gs_58127 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#58127:u8
        let s_29_0: bool = fn_state.gs_58127;
        // D s_29_1: not s_29_0
        let s_29_1: bool = !s_29_0;
        // N s_29_2: branch s_29_1 b63 b30
        if s_29_1 {
            return block_63(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#58128 <= s_30_0
        fn_state.gs_58128 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#58128:u8
        let s_31_0: bool = fn_state.gs_58128;
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
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call EL2Enabled(s_32_0)
        let s_32_1: bool = EL2Enabled(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b56 b33
        if s_32_1 {
            return block_56(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#58129 <= s_33_0
        fn_state.gs_58129 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#58129:u8
        let s_34_0: bool = fn_state.gs_58129;
        // N s_34_1: branch s_34_0 b55 b35
        if s_34_0 {
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
        // D s_35_1: write-var gs#58130 <= s_35_0
        fn_state.gs_58130 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#58130:u8
        let s_36_0: bool = fn_state.gs_58130;
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
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call EL2Enabled(s_37_0)
        let s_37_1: bool = EL2Enabled(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b53 b38
        if s_37_1 {
            return block_53(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#58131 <= s_38_0
        fn_state.gs_58131 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#58131:u8
        let s_39_0: bool = fn_state.gs_58131;
        // N s_39_1: branch s_39_0 b52 b40
        if s_39_0 {
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
        // D s_40_1: write-var gs#58132 <= s_40_0
        fn_state.gs_58132 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#58132:u8
        let s_41_0: bool = fn_state.gs_58132;
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
        // C s_42_0: const #432u : u32
        let s_42_0: u32 = 432;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: u8 = {
            let value = state.read_register::<u8>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // C s_42_2: const #2u : u8
        let s_42_2: u8 = 2;
        // D s_42_3: cmp-lt s_42_1 s_42_2
        let s_42_3: bool = ((s_42_1) < (s_42_2));
        // N s_42_4: branch s_42_3 b47 b43
        if s_42_3 {
            return block_47(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#58134 <= s_43_0
        fn_state.gs_58134 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#58134:u8
        let s_44_0: bool = fn_state.gs_58134;
        // N s_44_1: branch s_44_0 b46 b45
        if s_44_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #64s : i64
        let s_45_0: i64 = 64;
        // C s_45_1: const #() : ()
        let s_45_1: () = ();
        // S s_45_2: call PhysicalCountInt(s_45_1)
        let s_45_2: u64 = PhysicalCountInt(state, tracer, s_45_1);
        // S s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 64u16);
        // D s_45_4: read-var t:i
        let s_45_4: i128 = fn_state.t;
        // D s_45_5: call X_set(s_45_4, s_45_0, s_45_3)
        let s_45_5: () = X_set(state, tracer, s_45_4, s_45_0, s_45_3);
        // N s_45_6: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #64s : i64
        let s_46_0: i64 = 64;
        // C s_46_1: const #() : ()
        let s_46_1: () = ();
        // S s_46_2: call PhysicalCountInt(s_46_1)
        let s_46_2: u64 = PhysicalCountInt(state, tracer, s_46_1);
        // S s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 64u16);
        // C s_46_4: const #22400u : u32
        let s_46_4: u32 = 22400;
        // D s_46_5: read-reg s_46_4:u64
        let s_46_5: u64 = {
            let value = state.read_register::<u64>(s_46_4 as isize);
            tracer.read_register(s_46_4 as isize, value);
            value
        };
        // D s_46_6: cast zx s_46_5 -> bv
        let s_46_6: Bits = Bits::new(s_46_5 as u128, 64u16);
        // D s_46_7: sub s_46_3 s_46_6
        let s_46_7: Bits = ((s_46_3) - (s_46_6));
        // D s_46_8: cast reint s_46_7 -> u64
        let s_46_8: u64 = (s_46_7.value() as u64);
        // D s_46_9: cast zx s_46_8 -> bv
        let s_46_9: Bits = Bits::new(s_46_8 as u128, 64u16);
        // D s_46_10: read-var t:i
        let s_46_10: i128 = fn_state.t;
        // D s_46_11: call X_set(s_46_10, s_46_0, s_46_9)
        let s_46_11: () = X_set(state, tracer, s_46_10, s_46_0, s_46_9);
        // N s_46_12: return
        return;
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
        // S s_47_2: not s_47_1
        let s_47_2: bool = !s_47_1;
        // N s_47_3: branch s_47_2 b50 b48
        if s_47_2 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #102552u : u32
        let s_48_0: u32 = 102552;
        // D s_48_1: read-reg s_48_0:struct
        let s_48_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: call _get_HCR_EL2_Type_E2H(s_48_1)
        let s_48_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_48_1);
        // C s_48_3: const #102552u : u32
        let s_48_3: u32 = 102552;
        // D s_48_4: read-reg s_48_3:struct
        let s_48_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_48_3 as isize);
            tracer.read_register(s_48_3 as isize, value);
            value
        };
        // D s_48_5: call _get_HCR_EL2_Type_TGE(s_48_4)
        let s_48_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_48_4);
        // D s_48_6: cast zx s_48_2 -> bv
        let s_48_6: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_7: cast zx s_48_5 -> bv
        let s_48_7: Bits = Bits::new(s_48_5 as u128, 1u16);
        // D s_48_8: cast reint s_48_6 -> u128
        let s_48_8: u128 = (s_48_6.value() as u128);
        // D s_48_9: size-of s_48_6
        let s_48_9: u16 = s_48_6.length();
        // D s_48_10: cast reint s_48_7 -> u128
        let s_48_10: u128 = (s_48_7.value() as u128);
        // D s_48_11: size-of s_48_7
        let s_48_11: u16 = s_48_7.length();
        // D s_48_12: lsl s_48_8 s_48_11
        let s_48_12: u128 = s_48_8 << s_48_11;
        // D s_48_13: or s_48_12 s_48_10
        let s_48_13: u128 = ((s_48_12) | (s_48_10));
        // D s_48_14: add s_48_9 s_48_11
        let s_48_14: u16 = (s_48_9 + s_48_11);
        // D s_48_15: create-bits s_48_13 s_48_14
        let s_48_15: Bits = Bits::new(s_48_13, s_48_14);
        // D s_48_16: cast reint s_48_15 -> u8
        let s_48_16: u8 = (s_48_15.value() as u8);
        // D s_48_17: cast zx s_48_16 -> bv
        let s_48_17: Bits = Bits::new(s_48_16 as u128, 2u16);
        // C s_48_18: const #3u : u8
        let s_48_18: u8 = 3;
        // C s_48_19: cast zx s_48_18 -> bv
        let s_48_19: Bits = Bits::new(s_48_18 as u128, 2u16);
        // D s_48_20: cmp-ne s_48_17 s_48_19
        let s_48_20: bool = ((s_48_17) != (s_48_19));
        // D s_48_21: write-var gs#58133 <= s_48_20
        fn_state.gs_58133 = s_48_20;
        // N s_48_22: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#58133:u8
        let s_49_0: bool = fn_state.gs_58133;
        // D s_49_1: write-var gs#58134 <= s_49_0
        fn_state.gs_58134 = s_49_0;
        // N s_49_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #1u : u8
        let s_50_0: bool = true;
        // D s_50_1: write-var gs#58133 <= s_50_0
        fn_state.gs_58133 = s_50_0;
        // N s_50_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #24u : u8
        let s_51_0: u8 = 24;
        // C s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 8u16);
        // C s_51_2: cast zx s_51_1 -> i
        let s_51_2: i128 = (s_51_1.value() as i128);
        // C s_51_3: cast reint s_51_2 -> i64
        let s_51_3: i64 = (s_51_2 as i64);
        // C s_51_4: cast zx s_51_3 -> i
        let s_51_4: i128 = (i128::try_from(s_51_3).unwrap());
        // C s_51_5: const #432u : u32
        let s_51_5: u32 = 432;
        // D s_51_6: read-reg s_51_5:u8
        let s_51_6: u8 = {
            let value = state.read_register::<u8>(s_51_5 as isize);
            tracer.read_register(s_51_5 as isize, value);
            value
        };
        // D s_51_7: call AArch64_SystemAccessTrap(s_51_6, s_51_4)
        let s_51_7: () = AArch64_SystemAccessTrap(state, tracer, s_51_6, s_51_4);
        // N s_51_8: return
        return;
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var __CNTHCTL_EL2_EL1TVCT:u8
        let s_52_0: bool = fn_state.u__CNTHCTL_EL2_EL1TVCT;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 1u16);
        // C s_52_2: const #1u : u8
        let s_52_2: bool = true;
        // C s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // D s_52_5: write-var gs#58132 <= s_52_4
        fn_state.gs_58132 = s_52_4;
        // N s_52_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #102552u : u32
        let s_53_0: u32 = 102552;
        // D s_53_1: read-reg s_53_0:struct
        let s_53_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_53_0 as isize);
            tracer.read_register(s_53_0 as isize, value);
            value
        };
        // D s_53_2: call _get_HCR_EL2_Type_E2H(s_53_1)
        let s_53_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_53_1);
        // C s_53_3: const #102552u : u32
        let s_53_3: u32 = 102552;
        // D s_53_4: read-reg s_53_3:struct
        let s_53_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_53_3 as isize);
            tracer.read_register(s_53_3 as isize, value);
            value
        };
        // D s_53_5: call _get_HCR_EL2_Type_TGE(s_53_4)
        let s_53_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_53_4);
        // D s_53_6: cast zx s_53_2 -> bv
        let s_53_6: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_7: cast zx s_53_5 -> bv
        let s_53_7: Bits = Bits::new(s_53_5 as u128, 1u16);
        // D s_53_8: cast reint s_53_6 -> u128
        let s_53_8: u128 = (s_53_6.value() as u128);
        // D s_53_9: size-of s_53_6
        let s_53_9: u16 = s_53_6.length();
        // D s_53_10: cast reint s_53_7 -> u128
        let s_53_10: u128 = (s_53_7.value() as u128);
        // D s_53_11: size-of s_53_7
        let s_53_11: u16 = s_53_7.length();
        // D s_53_12: lsl s_53_8 s_53_11
        let s_53_12: u128 = s_53_8 << s_53_11;
        // D s_53_13: or s_53_12 s_53_10
        let s_53_13: u128 = ((s_53_12) | (s_53_10));
        // D s_53_14: add s_53_9 s_53_11
        let s_53_14: u16 = (s_53_9 + s_53_11);
        // D s_53_15: create-bits s_53_13 s_53_14
        let s_53_15: Bits = Bits::new(s_53_13, s_53_14);
        // D s_53_16: cast reint s_53_15 -> u8
        let s_53_16: u8 = (s_53_15.value() as u8);
        // D s_53_17: cast zx s_53_16 -> bv
        let s_53_17: Bits = Bits::new(s_53_16 as u128, 2u16);
        // C s_53_18: const #3u : u8
        let s_53_18: u8 = 3;
        // C s_53_19: cast zx s_53_18 -> bv
        let s_53_19: Bits = Bits::new(s_53_18 as u128, 2u16);
        // D s_53_20: cmp-ne s_53_17 s_53_19
        let s_53_20: bool = ((s_53_17) != (s_53_19));
        // D s_53_21: write-var gs#58131 <= s_53_20
        fn_state.gs_58131 = s_53_20;
        // N s_53_22: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #24u : u8
        let s_54_0: u8 = 24;
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
        // D s_54_7: call AArch64_SystemAccessTrap(s_54_6, s_54_4)
        let s_54_7: () = AArch64_SystemAccessTrap(state, tracer, s_54_6, s_54_4);
        // N s_54_8: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var __CNTHCTL_EL2_EL0VCTEN:u8
        let s_55_0: bool = fn_state.u__CNTHCTL_EL2_EL0VCTEN;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 1u16);
        // C s_55_2: const #0u : u8
        let s_55_2: bool = false;
        // C s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // D s_55_4: cmp-eq s_55_1 s_55_3
        let s_55_4: bool = ((s_55_1) == (s_55_3));
        // D s_55_5: write-var gs#58130 <= s_55_4
        fn_state.gs_58130 = s_55_4;
        // N s_55_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #102552u : u32
        let s_56_0: u32 = 102552;
        // D s_56_1: read-reg s_56_0:struct
        let s_56_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: call _get_HCR_EL2_Type_E2H(s_56_1)
        let s_56_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_56_1);
        // C s_56_3: const #102552u : u32
        let s_56_3: u32 = 102552;
        // D s_56_4: read-reg s_56_3:struct
        let s_56_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_3 as isize);
            tracer.read_register(s_56_3 as isize, value);
            value
        };
        // D s_56_5: call _get_HCR_EL2_Type_TGE(s_56_4)
        let s_56_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_56_4);
        // D s_56_6: cast zx s_56_2 -> bv
        let s_56_6: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_7: cast zx s_56_5 -> bv
        let s_56_7: Bits = Bits::new(s_56_5 as u128, 1u16);
        // D s_56_8: cast reint s_56_6 -> u128
        let s_56_8: u128 = (s_56_6.value() as u128);
        // D s_56_9: size-of s_56_6
        let s_56_9: u16 = s_56_6.length();
        // D s_56_10: cast reint s_56_7 -> u128
        let s_56_10: u128 = (s_56_7.value() as u128);
        // D s_56_11: size-of s_56_7
        let s_56_11: u16 = s_56_7.length();
        // D s_56_12: lsl s_56_8 s_56_11
        let s_56_12: u128 = s_56_8 << s_56_11;
        // D s_56_13: or s_56_12 s_56_10
        let s_56_13: u128 = ((s_56_12) | (s_56_10));
        // D s_56_14: add s_56_9 s_56_11
        let s_56_14: u16 = (s_56_9 + s_56_11);
        // D s_56_15: create-bits s_56_13 s_56_14
        let s_56_15: Bits = Bits::new(s_56_13, s_56_14);
        // D s_56_16: cast reint s_56_15 -> u8
        let s_56_16: u8 = (s_56_15.value() as u8);
        // D s_56_17: cast zx s_56_16 -> bv
        let s_56_17: Bits = Bits::new(s_56_16 as u128, 2u16);
        // C s_56_18: const #3u : u8
        let s_56_18: u8 = 3;
        // C s_56_19: cast zx s_56_18 -> bv
        let s_56_19: Bits = Bits::new(s_56_18 as u128, 2u16);
        // D s_56_20: cmp-eq s_56_17 s_56_19
        let s_56_20: bool = ((s_56_17) == (s_56_19));
        // D s_56_21: write-var gs#58129 <= s_56_20
        fn_state.gs_58129 = s_56_20;
        // N s_56_22: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call EL2Enabled(s_57_0)
        let s_57_1: bool = EL2Enabled(state, tracer, s_57_0);
        // N s_57_2: branch s_57_1 b62 b58
        if s_57_1 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#58137 <= s_58_0
        fn_state.gs_58137 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#58137:u8
        let s_59_0: bool = fn_state.gs_58137;
        // N s_59_1: branch s_59_0 b61 b60
        if s_59_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #24u : u8
        let s_60_0: u8 = 24;
        // C s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 8u16);
        // C s_60_2: cast zx s_60_1 -> i
        let s_60_2: i128 = (s_60_1.value() as i128);
        // C s_60_3: cast reint s_60_2 -> i64
        let s_60_3: i64 = (s_60_2 as i64);
        // C s_60_4: cast zx s_60_3 -> i
        let s_60_4: i128 = (i128::try_from(s_60_3).unwrap());
        // C s_60_5: const #440u : u32
        let s_60_5: u32 = 440;
        // D s_60_6: read-reg s_60_5:u8
        let s_60_6: u8 = {
            let value = state.read_register::<u8>(s_60_5 as isize);
            tracer.read_register(s_60_5 as isize, value);
            value
        };
        // D s_60_7: call AArch64_SystemAccessTrap(s_60_6, s_60_4)
        let s_60_7: () = AArch64_SystemAccessTrap(state, tracer, s_60_6, s_60_4);
        // N s_60_8: return
        return;
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #24u : u8
        let s_61_0: u8 = 24;
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
        // D s_61_7: call AArch64_SystemAccessTrap(s_61_6, s_61_4)
        let s_61_7: () = AArch64_SystemAccessTrap(state, tracer, s_61_6, s_61_4);
        // N s_61_8: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var __HCR_EL2_TGE:u8
        let s_62_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#58137 <= s_62_4
        fn_state.gs_58137 = s_62_4;
        // N s_62_6: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var __CNTKCTL_EL1_EL0VCTEN:u8
        let s_63_0: bool = fn_state.u__CNTKCTL_EL1_EL0VCTEN;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #0u : u8
        let s_63_2: bool = false;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#58128 <= s_63_4
        fn_state.gs_58128 = s_63_4;
        // N s_63_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #102552u : u32
        let s_64_0: u32 = 102552;
        // D s_64_1: read-reg s_64_0:struct
        let s_64_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call _get_HCR_EL2_Type_E2H(s_64_1)
        let s_64_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_64_1);
        // C s_64_3: const #102552u : u32
        let s_64_3: u32 = 102552;
        // D s_64_4: read-reg s_64_3:struct
        let s_64_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_64_3 as isize);
            tracer.read_register(s_64_3 as isize, value);
            value
        };
        // D s_64_5: call _get_HCR_EL2_Type_TGE(s_64_4)
        let s_64_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_64_4);
        // D s_64_6: cast zx s_64_2 -> bv
        let s_64_6: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_7: cast zx s_64_5 -> bv
        let s_64_7: Bits = Bits::new(s_64_5 as u128, 1u16);
        // D s_64_8: cast reint s_64_6 -> u128
        let s_64_8: u128 = (s_64_6.value() as u128);
        // D s_64_9: size-of s_64_6
        let s_64_9: u16 = s_64_6.length();
        // D s_64_10: cast reint s_64_7 -> u128
        let s_64_10: u128 = (s_64_7.value() as u128);
        // D s_64_11: size-of s_64_7
        let s_64_11: u16 = s_64_7.length();
        // D s_64_12: lsl s_64_8 s_64_11
        let s_64_12: u128 = s_64_8 << s_64_11;
        // D s_64_13: or s_64_12 s_64_10
        let s_64_13: u128 = ((s_64_12) | (s_64_10));
        // D s_64_14: add s_64_9 s_64_11
        let s_64_14: u16 = (s_64_9 + s_64_11);
        // D s_64_15: create-bits s_64_13 s_64_14
        let s_64_15: Bits = Bits::new(s_64_13, s_64_14);
        // D s_64_16: cast reint s_64_15 -> u8
        let s_64_16: u8 = (s_64_15.value() as u8);
        // D s_64_17: cast zx s_64_16 -> bv
        let s_64_17: Bits = Bits::new(s_64_16 as u128, 2u16);
        // C s_64_18: const #3u : u8
        let s_64_18: u8 = 3;
        // C s_64_19: cast zx s_64_18 -> bv
        let s_64_19: Bits = Bits::new(s_64_18 as u128, 2u16);
        // D s_64_20: cmp-eq s_64_17 s_64_19
        let s_64_20: bool = ((s_64_17) == (s_64_19));
        // D s_64_21: write-var gs#58127 <= s_64_20
        fn_state.gs_58127 = s_64_20;
        // N s_64_22: jump b29
        return block_29(state, tracer, fn_state);
    }
}
