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
use X_read::*;
use common::*;
pub fn UAO_SysRegWrite_3265e241e210a049<T: Tracer>(
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
        u__PSTATE_EL: u8,
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
        // D s_0_3: read-var __PSTATE_EL:u8
        let s_0_3: u8 = fn_state.u__PSTATE_EL;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // C s_0_5: const #448u : u32
        let s_0_5: u32 = 448;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: u8 = {
            let value = state.read_register::<u8>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // D s_0_8: cmp-eq s_0_4 s_0_7
        let s_0_8: bool = ((s_0_4) == (s_0_7));
        // N s_0_9: branch s_0_8 b8 b1
        if s_0_8 {
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
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // C s_5_4: const #23s : i
        let s_5_4: i128 = 23;
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
        // C s_5_21: const #16995u : u32
        let s_5_21: u32 = 16995;
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
        // C s_6_4: const #23s : i
        let s_6_4: i128 = 23;
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
        // C s_6_21: const #16995u : u32
        let s_6_21: u32 = 16995;
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
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // D s_7_1: read-var t:i
        let s_7_1: i128 = fn_state.t;
        // D s_7_2: call X_read(s_7_1, s_7_0)
        let s_7_2: Bits = X_read(state, tracer, s_7_1, s_7_0);
        // D s_7_3: cast reint s_7_2 -> u64
        let s_7_3: u64 = (s_7_2.value() as u64);
        // C s_7_4: const #23s : i
        let s_7_4: i128 = 23;
        // D s_7_5: cast zx s_7_3 -> bv
        let s_7_5: Bits = Bits::new(s_7_3 as u128, 64u16);
        // C s_7_6: const #1u : u64
        let s_7_6: u64 = 1;
        // D s_7_7: bit-extract s_7_5 s_7_4 s_7_6
        let s_7_7: Bits = (Bits::new(
            ((s_7_5) >> (s_7_4)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: bool = ((s_7_7.value()) != 0);
        // C s_7_9: const #0s : i
        let s_7_9: i128 = 0;
        // C s_7_10: const #0u : u64
        let s_7_10: u64 = 0;
        // D s_7_11: cast zx s_7_8 -> u64
        let s_7_11: u64 = (s_7_8 as u64);
        // C s_7_12: const #1u : u64
        let s_7_12: u64 = 1;
        // D s_7_13: and s_7_11 s_7_12
        let s_7_13: u64 = ((s_7_11) & (s_7_12));
        // D s_7_14: cmp-eq s_7_13 s_7_12
        let s_7_14: bool = ((s_7_13) == (s_7_12));
        // D s_7_15: lsl s_7_11 s_7_9
        let s_7_15: u64 = s_7_11 << s_7_9;
        // D s_7_16: or s_7_10 s_7_15
        let s_7_16: u64 = ((s_7_10) | (s_7_15));
        // D s_7_17: cmpl s_7_15
        let s_7_17: u64 = !s_7_15;
        // D s_7_18: and s_7_10 s_7_17
        let s_7_18: u64 = ((s_7_10) & (s_7_17));
        // D s_7_19: select s_7_14 s_7_16 s_7_18
        let s_7_19: u64 = if s_7_14 { s_7_16 } else { s_7_18 };
        // D s_7_20: cast trunc s_7_19 -> u8
        let s_7_20: bool = ((s_7_19) != 0);
        // C s_7_21: const #16995u : u32
        let s_7_21: u32 = 16995;
        // N s_7_22: write-reg s_7_21 <= s_7_20
        let s_7_22: () = {
            state.write_register::<bool>(s_7_21 as isize, s_7_20);
            tracer.write_register(s_7_21 as isize, s_7_20);
        };
        // N s_7_23: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
}
