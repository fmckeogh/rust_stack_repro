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
use u_get_HCRX_EL2_Type_TALLINT::*;
use X_read::*;
use EL2Enabled::*;
use AArch64_SystemAccessTrap::*;
use IsHCRXEL2Enabled::*;
use common::*;
pub fn ALLINT_SysRegWrite_28e93c037c43b390<T: Tracer>(
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
        gs_80134: bool,
        u__PSTATE_EL: u8,
        u__HCRX_EL2_TALLINT: bool,
        gs_80133: bool,
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
        // C s_0_3: const #22528u : u32
        let s_0_3: u32 = 22528;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HCRX_EL2_Type_TALLINT(s_0_4)
        let s_0_5: bool = u_get_HCRX_EL2_Type_TALLINT(state, tracer, s_0_4);
        // D s_0_6: write-var __HCRX_EL2_TALLINT <= s_0_5
        fn_state.u__HCRX_EL2_TALLINT = s_0_5;
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
        // N s_0_13: branch s_0_12 b16 b1
        if s_0_12 {
            return block_16(state, tracer, fn_state);
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
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // C s_5_4: const #13s : i
        let s_5_4: i128 = 13;
        // D s_5_5: cast zx s_5_3 -> bv
        let s_5_5: Bits = Bits::new(s_5_3 as u128, 64u16);
        // C s_5_6: const #1u : u64
        let s_5_6: u64 = 1;
        // D s_5_7: bit-extract s_5_5 s_5_4 s_5_6
        let s_5_7: Bits = (Bits::new(
            ((s_5_5) >> (s_5_4)).value(),
            u16::try_from(s_5_6).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: bool = ((s_5_7.value()) != 0);
        // C s_5_9: const #0s : i
        let s_5_9: i128 = 0;
        // C s_5_10: const #0u : u64
        let s_5_10: u64 = 0;
        // D s_5_11: cast zx s_5_8 -> u64
        let s_5_11: u64 = (s_5_8 as u64);
        // C s_5_12: const #1u : u64
        let s_5_12: u64 = 1;
        // D s_5_13: and s_5_11 s_5_12
        let s_5_13: u64 = ((s_5_11) & (s_5_12));
        // D s_5_14: cmp-eq s_5_13 s_5_12
        let s_5_14: bool = ((s_5_13) == (s_5_12));
        // D s_5_15: lsl s_5_11 s_5_9
        let s_5_15: u64 = s_5_11 << s_5_9;
        // D s_5_16: or s_5_10 s_5_15
        let s_5_16: u64 = ((s_5_10) | (s_5_15));
        // D s_5_17: cmpl s_5_15
        let s_5_17: u64 = !s_5_15;
        // D s_5_18: and s_5_10 s_5_17
        let s_5_18: u64 = ((s_5_10) & (s_5_17));
        // D s_5_19: select s_5_14 s_5_16 s_5_18
        let s_5_19: u64 = if s_5_14 { s_5_16 } else { s_5_18 };
        // D s_5_20: cast trunc s_5_19 -> u8
        let s_5_20: bool = ((s_5_19) != 0);
        // C s_5_21: const #16969u : u32
        let s_5_21: u32 = 16969;
        // N s_5_22: write-reg s_5_21 <= s_5_20
        let s_5_22: () = {
            state.write_register::<bool>(s_5_21 as isize, s_5_20);
            tracer.write_register(s_5_21 as isize, s_5_20);
        };
        // N s_5_23: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: read-var t:i
        let s_6_1: i128 = fn_state.t;
        // D s_6_2: call X_read(s_6_1, s_6_0)
        let s_6_2: Bits = X_read(state, tracer, s_6_1, s_6_0);
        // D s_6_3: cast reint s_6_2 -> u64
        let s_6_3: u64 = (s_6_2.value() as u64);
        // C s_6_4: const #13s : i
        let s_6_4: i128 = 13;
        // D s_6_5: cast zx s_6_3 -> bv
        let s_6_5: Bits = Bits::new(s_6_3 as u128, 64u16);
        // C s_6_6: const #1u : u64
        let s_6_6: u64 = 1;
        // D s_6_7: bit-extract s_6_5 s_6_4 s_6_6
        let s_6_7: Bits = (Bits::new(
            ((s_6_5) >> (s_6_4)).value(),
            u16::try_from(s_6_6).unwrap(),
        ));
        // D s_6_8: cast reint s_6_7 -> u8
        let s_6_8: bool = ((s_6_7.value()) != 0);
        // C s_6_9: const #0s : i
        let s_6_9: i128 = 0;
        // C s_6_10: const #0u : u64
        let s_6_10: u64 = 0;
        // D s_6_11: cast zx s_6_8 -> u64
        let s_6_11: u64 = (s_6_8 as u64);
        // C s_6_12: const #1u : u64
        let s_6_12: u64 = 1;
        // D s_6_13: and s_6_11 s_6_12
        let s_6_13: u64 = ((s_6_11) & (s_6_12));
        // D s_6_14: cmp-eq s_6_13 s_6_12
        let s_6_14: bool = ((s_6_13) == (s_6_12));
        // D s_6_15: lsl s_6_11 s_6_9
        let s_6_15: u64 = s_6_11 << s_6_9;
        // D s_6_16: or s_6_10 s_6_15
        let s_6_16: u64 = ((s_6_10) | (s_6_15));
        // D s_6_17: cmpl s_6_15
        let s_6_17: u64 = !s_6_15;
        // D s_6_18: and s_6_10 s_6_17
        let s_6_18: u64 = ((s_6_10) & (s_6_17));
        // D s_6_19: select s_6_14 s_6_16 s_6_18
        let s_6_19: u64 = if s_6_14 { s_6_16 } else { s_6_18 };
        // D s_6_20: cast trunc s_6_19 -> u8
        let s_6_20: bool = ((s_6_19) != 0);
        // C s_6_21: const #16969u : u32
        let s_6_21: u32 = 16969;
        // N s_6_22: write-reg s_6_21 <= s_6_20
        let s_6_22: () = {
            state.write_register::<bool>(s_6_21 as isize, s_6_20);
            tracer.write_register(s_6_21 as isize, s_6_20);
        };
        // N s_6_23: return
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
        // N s_7_2: branch s_7_1 b15 b8
        if s_7_1 {
            return block_15(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#80133 <= s_8_0
        fn_state.gs_80133 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#80133:u8
        let s_9_0: bool = fn_state.gs_80133;
        // N s_9_1: branch s_9_0 b14 b10
        if s_9_0 {
            return block_14(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#80134 <= s_10_0
        fn_state.gs_80134 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#80134:u8
        let s_11_0: bool = fn_state.gs_80134;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #64s : i64
        let s_12_0: i64 = 64;
        // D s_12_1: read-var t:i
        let s_12_1: i128 = fn_state.t;
        // D s_12_2: call X_read(s_12_1, s_12_0)
        let s_12_2: Bits = X_read(state, tracer, s_12_1, s_12_0);
        // D s_12_3: cast reint s_12_2 -> u64
        let s_12_3: u64 = (s_12_2.value() as u64);
        // C s_12_4: const #13s : i
        let s_12_4: i128 = 13;
        // D s_12_5: cast zx s_12_3 -> bv
        let s_12_5: Bits = Bits::new(s_12_3 as u128, 64u16);
        // C s_12_6: const #1u : u64
        let s_12_6: u64 = 1;
        // D s_12_7: bit-extract s_12_5 s_12_4 s_12_6
        let s_12_7: Bits = (Bits::new(
            ((s_12_5) >> (s_12_4)).value(),
            u16::try_from(s_12_6).unwrap(),
        ));
        // D s_12_8: cast reint s_12_7 -> u8
        let s_12_8: bool = ((s_12_7.value()) != 0);
        // C s_12_9: const #0s : i
        let s_12_9: i128 = 0;
        // C s_12_10: const #0u : u64
        let s_12_10: u64 = 0;
        // D s_12_11: cast zx s_12_8 -> u64
        let s_12_11: u64 = (s_12_8 as u64);
        // C s_12_12: const #1u : u64
        let s_12_12: u64 = 1;
        // D s_12_13: and s_12_11 s_12_12
        let s_12_13: u64 = ((s_12_11) & (s_12_12));
        // D s_12_14: cmp-eq s_12_13 s_12_12
        let s_12_14: bool = ((s_12_13) == (s_12_12));
        // D s_12_15: lsl s_12_11 s_12_9
        let s_12_15: u64 = s_12_11 << s_12_9;
        // D s_12_16: or s_12_10 s_12_15
        let s_12_16: u64 = ((s_12_10) | (s_12_15));
        // D s_12_17: cmpl s_12_15
        let s_12_17: u64 = !s_12_15;
        // D s_12_18: and s_12_10 s_12_17
        let s_12_18: u64 = ((s_12_10) & (s_12_17));
        // D s_12_19: select s_12_14 s_12_16 s_12_18
        let s_12_19: u64 = if s_12_14 { s_12_16 } else { s_12_18 };
        // D s_12_20: cast trunc s_12_19 -> u8
        let s_12_20: bool = ((s_12_19) != 0);
        // C s_12_21: const #16969u : u32
        let s_12_21: u32 = 16969;
        // N s_12_22: write-reg s_12_21 <= s_12_20
        let s_12_22: () = {
            state.write_register::<bool>(s_12_21 as isize, s_12_20);
            tracer.write_register(s_12_21 as isize, s_12_20);
        };
        // N s_12_23: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #24u : u8
        let s_13_0: u8 = 24;
        // C s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 8u16);
        // C s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (s_13_1.value() as i128);
        // C s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #432u : u32
        let s_13_5: u32 = 432;
        // D s_13_6: read-reg s_13_5:u8
        let s_13_6: u8 = {
            let value = state.read_register::<u8>(s_13_5 as isize);
            tracer.read_register(s_13_5 as isize, value);
            value
        };
        // D s_13_7: call AArch64_SystemAccessTrap(s_13_6, s_13_4)
        let s_13_7: () = AArch64_SystemAccessTrap(state, tracer, s_13_6, s_13_4);
        // N s_13_8: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var __HCRX_EL2_TALLINT:u8
        let s_14_0: bool = fn_state.u__HCRX_EL2_TALLINT;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: write-var gs#80134 <= s_14_4
        fn_state.gs_80134 = s_14_4;
        // N s_14_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call IsHCRXEL2Enabled(s_15_0)
        let s_15_1: bool = IsHCRXEL2Enabled(state, tracer, s_15_0);
        // D s_15_2: write-var gs#80133 <= s_15_1
        fn_state.gs_80133 = s_15_1;
        // N s_15_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
}
