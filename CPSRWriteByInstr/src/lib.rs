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
use HaveSSBSExt::*;
use HaveDITExt::*;
use AArch32_WriteModeByInstr::*;
use common::*;
pub fn CPSRWriteByInstr<T: Tracer>(
    state: &mut State,
    tracer: &T,
    value_name: u32,
    bytemask: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        privileged: bool,
        value_name: u32,
        bytemask: u8,
    }
    let fn_state = FunctionState {
        value_name,
        bytemask,
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
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 2u16);
        // C s_0_3: const #448u : u32
        let s_0_3: u32 = 448;
        // D s_0_4: read-reg s_0_3:u8
        let s_0_4: u8 = {
            let value = state.read_register::<u8>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 2u16);
        // D s_0_6: cmp-ne s_0_2 s_0_5
        let s_0_6: bool = ((s_0_2) != (s_0_5));
        // D s_0_7: write-var privileged <= s_0_6
        fn_state.privileged = s_0_6;
        // C s_0_8: const #3s : i
        let s_0_8: i128 = 3;
        // D s_0_9: read-var bytemask:u8
        let s_0_9: u8 = fn_state.bytemask;
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 4u16);
        // C s_0_11: const #1u : u64
        let s_0_11: u64 = 1;
        // D s_0_12: bit-extract s_0_10 s_0_8 s_0_11
        let s_0_12: Bits = (Bits::new(
            ((s_0_10) >> (s_0_8)).value(),
            u16::try_from(s_0_11).unwrap(),
        ));
        // D s_0_13: cast reint s_0_12 -> u8
        let s_0_13: bool = ((s_0_12.value()) != 0);
        // C s_0_14: const #0s : i
        let s_0_14: i128 = 0;
        // C s_0_15: const #0u : u64
        let s_0_15: u64 = 0;
        // D s_0_16: cast zx s_0_13 -> u64
        let s_0_16: u64 = (s_0_13 as u64);
        // C s_0_17: const #1u : u64
        let s_0_17: u64 = 1;
        // D s_0_18: and s_0_16 s_0_17
        let s_0_18: u64 = ((s_0_16) & (s_0_17));
        // D s_0_19: cmp-eq s_0_18 s_0_17
        let s_0_19: bool = ((s_0_18) == (s_0_17));
        // D s_0_20: lsl s_0_16 s_0_14
        let s_0_20: u64 = s_0_16 << s_0_14;
        // D s_0_21: or s_0_15 s_0_20
        let s_0_21: u64 = ((s_0_15) | (s_0_20));
        // D s_0_22: cmpl s_0_20
        let s_0_22: u64 = !s_0_20;
        // D s_0_23: and s_0_15 s_0_22
        let s_0_23: u64 = ((s_0_15) & (s_0_22));
        // D s_0_24: select s_0_19 s_0_21 s_0_23
        let s_0_24: u64 = if s_0_19 { s_0_21 } else { s_0_23 };
        // D s_0_25: cast trunc s_0_24 -> u8
        let s_0_25: bool = ((s_0_24) != 0);
        // D s_0_26: cast zx s_0_25 -> bv
        let s_0_26: Bits = Bits::new(s_0_25 as u128, 1u16);
        // C s_0_27: const #1u : u8
        let s_0_27: bool = true;
        // C s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 1u16);
        // D s_0_29: cmp-eq s_0_26 s_0_28
        let s_0_29: bool = ((s_0_26) == (s_0_28));
        // N s_0_30: branch s_0_29 b25 b1
        if s_0_29 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #2s : i
        let s_2_0: i128 = 2;
        // D s_2_1: read-var bytemask:u8
        let s_2_1: u8 = fn_state.bytemask;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 4u16);
        // C s_2_3: const #1u : u64
        let s_2_3: u64 = 1;
        // D s_2_4: bit-extract s_2_2 s_2_0 s_2_3
        let s_2_4: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_3).unwrap(),
        ));
        // D s_2_5: cast reint s_2_4 -> u8
        let s_2_5: bool = ((s_2_4.value()) != 0);
        // C s_2_6: const #0s : i
        let s_2_6: i128 = 0;
        // C s_2_7: const #0u : u64
        let s_2_7: u64 = 0;
        // D s_2_8: cast zx s_2_5 -> u64
        let s_2_8: u64 = (s_2_5 as u64);
        // C s_2_9: const #1u : u64
        let s_2_9: u64 = 1;
        // D s_2_10: and s_2_8 s_2_9
        let s_2_10: u64 = ((s_2_8) & (s_2_9));
        // D s_2_11: cmp-eq s_2_10 s_2_9
        let s_2_11: bool = ((s_2_10) == (s_2_9));
        // D s_2_12: lsl s_2_8 s_2_6
        let s_2_12: u64 = s_2_8 << s_2_6;
        // D s_2_13: or s_2_7 s_2_12
        let s_2_13: u64 = ((s_2_7) | (s_2_12));
        // D s_2_14: cmpl s_2_12
        let s_2_14: u64 = !s_2_12;
        // D s_2_15: and s_2_7 s_2_14
        let s_2_15: u64 = ((s_2_7) & (s_2_14));
        // D s_2_16: select s_2_11 s_2_13 s_2_15
        let s_2_16: u64 = if s_2_11 { s_2_13 } else { s_2_15 };
        // D s_2_17: cast trunc s_2_16 -> u8
        let s_2_17: bool = ((s_2_16) != 0);
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 1u16);
        // C s_2_19: const #1u : u8
        let s_2_19: bool = true;
        // C s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // D s_2_21: cmp-eq s_2_18 s_2_20
        let s_2_21: bool = ((s_2_18) == (s_2_20));
        // N s_2_22: branch s_2_21 b15 b3
        if s_2_21 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1s : i
        let s_4_0: i128 = 1;
        // D s_4_1: read-var bytemask:u8
        let s_4_1: u8 = fn_state.bytemask;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1u : u64
        let s_4_3: u64 = 1;
        // D s_4_4: bit-extract s_4_2 s_4_0 s_4_3
        let s_4_4: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_3).unwrap(),
        ));
        // D s_4_5: cast reint s_4_4 -> u8
        let s_4_5: bool = ((s_4_4.value()) != 0);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // C s_4_7: const #0u : u64
        let s_4_7: u64 = 0;
        // D s_4_8: cast zx s_4_5 -> u64
        let s_4_8: u64 = (s_4_5 as u64);
        // C s_4_9: const #1u : u64
        let s_4_9: u64 = 1;
        // D s_4_10: and s_4_8 s_4_9
        let s_4_10: u64 = ((s_4_8) & (s_4_9));
        // D s_4_11: cmp-eq s_4_10 s_4_9
        let s_4_11: bool = ((s_4_10) == (s_4_9));
        // D s_4_12: lsl s_4_8 s_4_6
        let s_4_12: u64 = s_4_8 << s_4_6;
        // D s_4_13: or s_4_7 s_4_12
        let s_4_13: u64 = ((s_4_7) | (s_4_12));
        // D s_4_14: cmpl s_4_12
        let s_4_14: u64 = !s_4_12;
        // D s_4_15: and s_4_7 s_4_14
        let s_4_15: u64 = ((s_4_7) & (s_4_14));
        // D s_4_16: select s_4_11 s_4_13 s_4_15
        let s_4_16: u64 = if s_4_11 { s_4_13 } else { s_4_15 };
        // D s_4_17: cast trunc s_4_16 -> u8
        let s_4_17: bool = ((s_4_16) != 0);
        // D s_4_18: cast zx s_4_17 -> bv
        let s_4_18: Bits = Bits::new(s_4_17 as u128, 1u16);
        // C s_4_19: const #1u : u8
        let s_4_19: bool = true;
        // C s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 1u16);
        // D s_4_21: cmp-eq s_4_18 s_4_20
        let s_4_21: bool = ((s_4_18) == (s_4_20));
        // N s_4_22: branch s_4_21 b11 b5
        if s_4_21 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i
        let s_6_0: i128 = 0;
        // D s_6_1: read-var bytemask:u8
        let s_6_1: u8 = fn_state.bytemask;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 4u16);
        // C s_6_3: const #1u : u64
        let s_6_3: u64 = 1;
        // D s_6_4: bit-extract s_6_2 s_6_0 s_6_3
        let s_6_4: Bits = (Bits::new(
            ((s_6_2) >> (s_6_0)).value(),
            u16::try_from(s_6_3).unwrap(),
        ));
        // D s_6_5: cast reint s_6_4 -> u8
        let s_6_5: bool = ((s_6_4.value()) != 0);
        // C s_6_6: const #0s : i
        let s_6_6: i128 = 0;
        // C s_6_7: const #0u : u64
        let s_6_7: u64 = 0;
        // D s_6_8: cast zx s_6_5 -> u64
        let s_6_8: u64 = (s_6_5 as u64);
        // C s_6_9: const #1u : u64
        let s_6_9: u64 = 1;
        // D s_6_10: and s_6_8 s_6_9
        let s_6_10: u64 = ((s_6_8) & (s_6_9));
        // D s_6_11: cmp-eq s_6_10 s_6_9
        let s_6_11: bool = ((s_6_10) == (s_6_9));
        // D s_6_12: lsl s_6_8 s_6_6
        let s_6_12: u64 = s_6_8 << s_6_6;
        // D s_6_13: or s_6_7 s_6_12
        let s_6_13: u64 = ((s_6_7) | (s_6_12));
        // D s_6_14: cmpl s_6_12
        let s_6_14: u64 = !s_6_12;
        // D s_6_15: and s_6_7 s_6_14
        let s_6_15: u64 = ((s_6_7) & (s_6_14));
        // D s_6_16: select s_6_11 s_6_13 s_6_15
        let s_6_16: u64 = if s_6_11 { s_6_13 } else { s_6_15 };
        // D s_6_17: cast trunc s_6_16 -> u8
        let s_6_17: bool = ((s_6_16) != 0);
        // D s_6_18: cast zx s_6_17 -> bv
        let s_6_18: Bits = Bits::new(s_6_17 as u128, 1u16);
        // C s_6_19: const #1u : u8
        let s_6_19: bool = true;
        // C s_6_20: cast zx s_6_19 -> bv
        let s_6_20: Bits = Bits::new(s_6_19 as u128, 1u16);
        // D s_6_21: cmp-eq s_6_18 s_6_20
        let s_6_21: bool = ((s_6_18) == (s_6_20));
        // N s_6_22: branch s_6_21 b8 b7
        if s_6_21 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var privileged:u8
        let s_8_0: bool = fn_state.privileged;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
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
        // N s_9_0: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #6s : i
        let s_10_0: i128 = 6;
        // D s_10_1: read-var value_name:u32
        let s_10_1: u32 = fn_state.value_name;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 32u16);
        // C s_10_3: const #1s : i64
        let s_10_3: i64 = 1;
        // C s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // C s_10_5: const #1s : i
        let s_10_5: i128 = 1;
        // C s_10_6: add s_10_5 s_10_4
        let s_10_6: i128 = (s_10_5 + s_10_4);
        // D s_10_7: bit-extract s_10_2 s_10_0 s_10_6
        let s_10_7: Bits = (Bits::new(
            ((s_10_2) >> (s_10_0)).value(),
            u16::try_from(s_10_6).unwrap(),
        ));
        // D s_10_8: cast reint s_10_7 -> u8
        let s_10_8: u8 = (s_10_7.value() as u8);
        // C s_10_9: const #1s : i
        let s_10_9: i128 = 1;
        // D s_10_10: cast zx s_10_8 -> bv
        let s_10_10: Bits = Bits::new(s_10_8 as u128, 2u16);
        // C s_10_11: const #1s : i64
        let s_10_11: i64 = 1;
        // C s_10_12: cast zx s_10_11 -> i
        let s_10_12: i128 = (i128::try_from(s_10_11).unwrap());
        // C s_10_13: const #0s : i
        let s_10_13: i128 = 0;
        // C s_10_14: add s_10_13 s_10_12
        let s_10_14: i128 = (s_10_13 + s_10_12);
        // D s_10_15: bit-extract s_10_10 s_10_9 s_10_14
        let s_10_15: Bits = (Bits::new(
            ((s_10_10) >> (s_10_9)).value(),
            u16::try_from(s_10_14).unwrap(),
        ));
        // D s_10_16: cast reint s_10_15 -> u8
        let s_10_16: bool = ((s_10_15.value()) != 0);
        // C s_10_17: const #16979u : u32
        let s_10_17: u32 = 16979;
        // N s_10_18: write-reg s_10_17 <= s_10_16
        let s_10_18: () = {
            state.write_register::<bool>(s_10_17 as isize, s_10_16);
            tracer.write_register(s_10_17 as isize, s_10_16);
        };
        // C s_10_19: const #0s : i
        let s_10_19: i128 = 0;
        // D s_10_20: cast zx s_10_8 -> bv
        let s_10_20: Bits = Bits::new(s_10_8 as u128, 2u16);
        // C s_10_21: const #1s : i64
        let s_10_21: i64 = 1;
        // C s_10_22: cast zx s_10_21 -> i
        let s_10_22: i128 = (i128::try_from(s_10_21).unwrap());
        // C s_10_23: const #0s : i
        let s_10_23: i128 = 0;
        // C s_10_24: add s_10_23 s_10_22
        let s_10_24: i128 = (s_10_23 + s_10_22);
        // D s_10_25: bit-extract s_10_20 s_10_19 s_10_24
        let s_10_25: Bits = (Bits::new(
            ((s_10_20) >> (s_10_19)).value(),
            u16::try_from(s_10_24).unwrap(),
        ));
        // D s_10_26: cast reint s_10_25 -> u8
        let s_10_26: bool = ((s_10_25.value()) != 0);
        // C s_10_27: const #16977u : u32
        let s_10_27: u32 = 16977;
        // N s_10_28: write-reg s_10_27 <= s_10_26
        let s_10_28: () = {
            state.write_register::<bool>(s_10_27 as isize, s_10_26);
            tracer.write_register(s_10_27 as isize, s_10_26);
        };
        // C s_10_29: const #0s : i
        let s_10_29: i128 = 0;
        // D s_10_30: read-var value_name:u32
        let s_10_30: u32 = fn_state.value_name;
        // D s_10_31: cast zx s_10_30 -> bv
        let s_10_31: Bits = Bits::new(s_10_30 as u128, 32u16);
        // C s_10_32: const #1s : i64
        let s_10_32: i64 = 1;
        // C s_10_33: cast zx s_10_32 -> i
        let s_10_33: i128 = (i128::try_from(s_10_32).unwrap());
        // C s_10_34: const #4s : i
        let s_10_34: i128 = 4;
        // C s_10_35: add s_10_34 s_10_33
        let s_10_35: i128 = (s_10_34 + s_10_33);
        // D s_10_36: bit-extract s_10_31 s_10_29 s_10_35
        let s_10_36: Bits = (Bits::new(
            ((s_10_31) >> (s_10_29)).value(),
            u16::try_from(s_10_35).unwrap(),
        ));
        // D s_10_37: cast reint s_10_36 -> u8
        let s_10_37: u8 = (s_10_36.value() as u8);
        // D s_10_38: call AArch32_WriteModeByInstr(s_10_37)
        let s_10_38: () = AArch32_WriteModeByInstr(state, tracer, s_10_37);
        // N s_10_39: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #9s : i
        let s_11_0: i128 = 9;
        // D s_11_1: read-var value_name:u32
        let s_11_1: u32 = fn_state.value_name;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 32u16);
        // C s_11_3: const #1u : u64
        let s_11_3: u64 = 1;
        // D s_11_4: bit-extract s_11_2 s_11_0 s_11_3
        let s_11_4: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_3).unwrap(),
        ));
        // D s_11_5: cast reint s_11_4 -> u8
        let s_11_5: bool = ((s_11_4.value()) != 0);
        // C s_11_6: const #0s : i
        let s_11_6: i128 = 0;
        // C s_11_7: const #0u : u64
        let s_11_7: u64 = 0;
        // D s_11_8: cast zx s_11_5 -> u64
        let s_11_8: u64 = (s_11_5 as u64);
        // C s_11_9: const #1u : u64
        let s_11_9: u64 = 1;
        // D s_11_10: and s_11_8 s_11_9
        let s_11_10: u64 = ((s_11_8) & (s_11_9));
        // D s_11_11: cmp-eq s_11_10 s_11_9
        let s_11_11: bool = ((s_11_10) == (s_11_9));
        // D s_11_12: lsl s_11_8 s_11_6
        let s_11_12: u64 = s_11_8 << s_11_6;
        // D s_11_13: or s_11_7 s_11_12
        let s_11_13: u64 = ((s_11_7) | (s_11_12));
        // D s_11_14: cmpl s_11_12
        let s_11_14: u64 = !s_11_12;
        // D s_11_15: and s_11_7 s_11_14
        let s_11_15: u64 = ((s_11_7) & (s_11_14));
        // D s_11_16: select s_11_11 s_11_13 s_11_15
        let s_11_16: u64 = if s_11_11 { s_11_13 } else { s_11_15 };
        // D s_11_17: cast trunc s_11_16 -> u8
        let s_11_17: bool = ((s_11_16) != 0);
        // C s_11_18: const #16974u : u32
        let s_11_18: u32 = 16974;
        // N s_11_19: write-reg s_11_18 <= s_11_17
        let s_11_19: () = {
            state.write_register::<bool>(s_11_18 as isize, s_11_17);
            tracer.write_register(s_11_18 as isize, s_11_17);
        };
        // D s_11_20: read-var privileged:u8
        let s_11_20: bool = fn_state.privileged;
        // N s_11_21: branch s_11_20 b14 b12
        if s_11_20 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #8s : i
        let s_14_0: i128 = 8;
        // D s_14_1: read-var value_name:u32
        let s_14_1: u32 = fn_state.value_name;
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 32u16);
        // C s_14_3: const #1u : u64
        let s_14_3: u64 = 1;
        // D s_14_4: bit-extract s_14_2 s_14_0 s_14_3
        let s_14_4: Bits = (Bits::new(
            ((s_14_2) >> (s_14_0)).value(),
            u16::try_from(s_14_3).unwrap(),
        ));
        // D s_14_5: cast reint s_14_4 -> u8
        let s_14_5: bool = ((s_14_4.value()) != 0);
        // C s_14_6: const #0s : i
        let s_14_6: i128 = 0;
        // C s_14_7: const #0u : u64
        let s_14_7: u64 = 0;
        // D s_14_8: cast zx s_14_5 -> u64
        let s_14_8: u64 = (s_14_5 as u64);
        // C s_14_9: const #1u : u64
        let s_14_9: u64 = 1;
        // D s_14_10: and s_14_8 s_14_9
        let s_14_10: u64 = ((s_14_8) & (s_14_9));
        // D s_14_11: cmp-eq s_14_10 s_14_9
        let s_14_11: bool = ((s_14_10) == (s_14_9));
        // D s_14_12: lsl s_14_8 s_14_6
        let s_14_12: u64 = s_14_8 << s_14_6;
        // D s_14_13: or s_14_7 s_14_12
        let s_14_13: u64 = ((s_14_7) | (s_14_12));
        // D s_14_14: cmpl s_14_12
        let s_14_14: u64 = !s_14_12;
        // D s_14_15: and s_14_7 s_14_14
        let s_14_15: u64 = ((s_14_7) & (s_14_14));
        // D s_14_16: select s_14_11 s_14_13 s_14_15
        let s_14_16: u64 = if s_14_11 { s_14_13 } else { s_14_15 };
        // D s_14_17: cast trunc s_14_16 -> u8
        let s_14_17: bool = ((s_14_16) != 0);
        // C s_14_18: const #16968u : u32
        let s_14_18: u32 = 16968;
        // N s_14_19: write-reg s_14_18 <= s_14_17
        let s_14_19: () = {
            state.write_register::<bool>(s_14_18 as isize, s_14_17);
            tracer.write_register(s_14_18 as isize, s_14_17);
        };
        // N s_14_20: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call HaveSSBSExt(s_15_0)
        let s_15_1: bool = HaveSSBSExt(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b24 b16
        if s_15_1 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var privileged:u8
        let s_17_0: bool = fn_state.privileged;
        // N s_17_1: branch s_17_0 b23 b18
        if s_17_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call HaveDITExt(s_19_0)
        let s_19_1: bool = HaveDITExt(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b22 b20
        if s_19_1 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #16s : i
        let s_21_0: i128 = 16;
        // D s_21_1: read-var value_name:u32
        let s_21_1: u32 = fn_state.value_name;
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 32u16);
        // C s_21_3: const #1s : i64
        let s_21_3: i64 = 1;
        // C s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // C s_21_5: const #3s : i
        let s_21_5: i128 = 3;
        // C s_21_6: add s_21_5 s_21_4
        let s_21_6: i128 = (s_21_5 + s_21_4);
        // D s_21_7: bit-extract s_21_2 s_21_0 s_21_6
        let s_21_7: Bits = (Bits::new(
            ((s_21_2) >> (s_21_0)).value(),
            u16::try_from(s_21_6).unwrap(),
        ));
        // D s_21_8: cast reint s_21_7 -> u8
        let s_21_8: u8 = (s_21_7.value() as u8);
        // C s_21_9: const #16978u : u32
        let s_21_9: u32 = 16978;
        // N s_21_10: write-reg s_21_9 <= s_21_8
        let s_21_10: () = {
            state.write_register::<u8>(s_21_9 as isize, s_21_8);
            tracer.write_register(s_21_9 as isize, s_21_8);
        };
        // N s_21_11: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #21s : i
        let s_22_0: i128 = 21;
        // D s_22_1: read-var value_name:u32
        let s_22_1: u32 = fn_state.value_name;
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 32u16);
        // C s_22_3: const #1u : u64
        let s_22_3: u64 = 1;
        // D s_22_4: bit-extract s_22_2 s_22_0 s_22_3
        let s_22_4: Bits = (Bits::new(
            ((s_22_2) >> (s_22_0)).value(),
            u16::try_from(s_22_3).unwrap(),
        ));
        // D s_22_5: cast reint s_22_4 -> u8
        let s_22_5: bool = ((s_22_4.value()) != 0);
        // C s_22_6: const #0s : i
        let s_22_6: i128 = 0;
        // C s_22_7: const #0u : u64
        let s_22_7: u64 = 0;
        // D s_22_8: cast zx s_22_5 -> u64
        let s_22_8: u64 = (s_22_5 as u64);
        // C s_22_9: const #1u : u64
        let s_22_9: u64 = 1;
        // D s_22_10: and s_22_8 s_22_9
        let s_22_10: u64 = ((s_22_8) & (s_22_9));
        // D s_22_11: cmp-eq s_22_10 s_22_9
        let s_22_11: bool = ((s_22_10) == (s_22_9));
        // D s_22_12: lsl s_22_8 s_22_6
        let s_22_12: u64 = s_22_8 << s_22_6;
        // D s_22_13: or s_22_7 s_22_12
        let s_22_13: u64 = ((s_22_7) | (s_22_12));
        // D s_22_14: cmpl s_22_12
        let s_22_14: u64 = !s_22_12;
        // D s_22_15: and s_22_7 s_22_14
        let s_22_15: u64 = ((s_22_7) & (s_22_14));
        // D s_22_16: select s_22_11 s_22_13 s_22_15
        let s_22_16: u64 = if s_22_11 { s_22_13 } else { s_22_15 };
        // D s_22_17: cast trunc s_22_16 -> u8
        let s_22_17: bool = ((s_22_16) != 0);
        // C s_22_18: const #16973u : u32
        let s_22_18: u32 = 16973;
        // N s_22_19: write-reg s_22_18 <= s_22_17
        let s_22_19: () = {
            state.write_register::<bool>(s_22_18 as isize, s_22_17);
            tracer.write_register(s_22_18 as isize, s_22_17);
        };
        // N s_22_20: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #22s : i
        let s_23_0: i128 = 22;
        // D s_23_1: read-var value_name:u32
        let s_23_1: u32 = fn_state.value_name;
        // D s_23_2: cast zx s_23_1 -> bv
        let s_23_2: Bits = Bits::new(s_23_1 as u128, 32u16);
        // C s_23_3: const #1u : u64
        let s_23_3: u64 = 1;
        // D s_23_4: bit-extract s_23_2 s_23_0 s_23_3
        let s_23_4: Bits = (Bits::new(
            ((s_23_2) >> (s_23_0)).value(),
            u16::try_from(s_23_3).unwrap(),
        ));
        // D s_23_5: cast reint s_23_4 -> u8
        let s_23_5: bool = ((s_23_4.value()) != 0);
        // C s_23_6: const #0s : i
        let s_23_6: i128 = 0;
        // C s_23_7: const #0u : u64
        let s_23_7: u64 = 0;
        // D s_23_8: cast zx s_23_5 -> u64
        let s_23_8: u64 = (s_23_5 as u64);
        // C s_23_9: const #1u : u64
        let s_23_9: u64 = 1;
        // D s_23_10: and s_23_8 s_23_9
        let s_23_10: u64 = ((s_23_8) & (s_23_9));
        // D s_23_11: cmp-eq s_23_10 s_23_9
        let s_23_11: bool = ((s_23_10) == (s_23_9));
        // D s_23_12: lsl s_23_8 s_23_6
        let s_23_12: u64 = s_23_8 << s_23_6;
        // D s_23_13: or s_23_7 s_23_12
        let s_23_13: u64 = ((s_23_7) | (s_23_12));
        // D s_23_14: cmpl s_23_12
        let s_23_14: u64 = !s_23_12;
        // D s_23_15: and s_23_7 s_23_14
        let s_23_15: u64 = ((s_23_7) & (s_23_14));
        // D s_23_16: select s_23_11 s_23_13 s_23_15
        let s_23_16: u64 = if s_23_11 { s_23_13 } else { s_23_15 };
        // D s_23_17: cast trunc s_23_16 -> u8
        let s_23_17: bool = ((s_23_16) != 0);
        // C s_23_18: const #16985u : u32
        let s_23_18: u32 = 16985;
        // N s_23_19: write-reg s_23_18 <= s_23_17
        let s_23_19: () = {
            state.write_register::<bool>(s_23_18 as isize, s_23_17);
            tracer.write_register(s_23_18 as isize, s_23_17);
        };
        // N s_23_20: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #23s : i
        let s_24_0: i128 = 23;
        // D s_24_1: read-var value_name:u32
        let s_24_1: u32 = fn_state.value_name;
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 32u16);
        // C s_24_3: const #1u : u64
        let s_24_3: u64 = 1;
        // D s_24_4: bit-extract s_24_2 s_24_0 s_24_3
        let s_24_4: Bits = (Bits::new(
            ((s_24_2) >> (s_24_0)).value(),
            u16::try_from(s_24_3).unwrap(),
        ));
        // D s_24_5: cast reint s_24_4 -> u8
        let s_24_5: bool = ((s_24_4.value()) != 0);
        // C s_24_6: const #0s : i
        let s_24_6: i128 = 0;
        // C s_24_7: const #0u : u64
        let s_24_7: u64 = 0;
        // D s_24_8: cast zx s_24_5 -> u64
        let s_24_8: u64 = (s_24_5 as u64);
        // C s_24_9: const #1u : u64
        let s_24_9: u64 = 1;
        // D s_24_10: and s_24_8 s_24_9
        let s_24_10: u64 = ((s_24_8) & (s_24_9));
        // D s_24_11: cmp-eq s_24_10 s_24_9
        let s_24_11: bool = ((s_24_10) == (s_24_9));
        // D s_24_12: lsl s_24_8 s_24_6
        let s_24_12: u64 = s_24_8 << s_24_6;
        // D s_24_13: or s_24_7 s_24_12
        let s_24_13: u64 = ((s_24_7) | (s_24_12));
        // D s_24_14: cmpl s_24_12
        let s_24_14: u64 = !s_24_12;
        // D s_24_15: and s_24_7 s_24_14
        let s_24_15: u64 = ((s_24_7) & (s_24_14));
        // D s_24_16: select s_24_11 s_24_13 s_24_15
        let s_24_16: u64 = if s_24_11 { s_24_13 } else { s_24_15 };
        // D s_24_17: cast trunc s_24_16 -> u8
        let s_24_17: bool = ((s_24_16) != 0);
        // C s_24_18: const #16992u : u32
        let s_24_18: u32 = 16992;
        // N s_24_19: write-reg s_24_18 <= s_24_17
        let s_24_19: () = {
            state.write_register::<bool>(s_24_18 as isize, s_24_17);
            tracer.write_register(s_24_18 as isize, s_24_17);
        };
        // N s_24_20: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #27s : i
        let s_25_0: i128 = 27;
        // D s_25_1: read-var value_name:u32
        let s_25_1: u32 = fn_state.value_name;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 32u16);
        // C s_25_3: const #1s : i64
        let s_25_3: i64 = 1;
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #4s : i
        let s_25_5: i128 = 4;
        // C s_25_6: add s_25_5 s_25_4
        let s_25_6: i128 = (s_25_5 + s_25_4);
        // D s_25_7: bit-extract s_25_2 s_25_0 s_25_6
        let s_25_7: Bits = (Bits::new(
            ((s_25_2) >> (s_25_0)).value(),
            u16::try_from(s_25_6).unwrap(),
        ));
        // D s_25_8: cast reint s_25_7 -> u8
        let s_25_8: u8 = (s_25_7.value() as u8);
        // C s_25_9: const #4s : i
        let s_25_9: i128 = 4;
        // D s_25_10: cast zx s_25_8 -> bv
        let s_25_10: Bits = Bits::new(s_25_8 as u128, 5u16);
        // C s_25_11: const #1s : i64
        let s_25_11: i64 = 1;
        // C s_25_12: cast zx s_25_11 -> i
        let s_25_12: i128 = (i128::try_from(s_25_11).unwrap());
        // C s_25_13: const #0s : i
        let s_25_13: i128 = 0;
        // C s_25_14: add s_25_13 s_25_12
        let s_25_14: i128 = (s_25_13 + s_25_12);
        // D s_25_15: bit-extract s_25_10 s_25_9 s_25_14
        let s_25_15: Bits = (Bits::new(
            ((s_25_10) >> (s_25_9)).value(),
            u16::try_from(s_25_14).unwrap(),
        ));
        // D s_25_16: cast reint s_25_15 -> u8
        let s_25_16: bool = ((s_25_15.value()) != 0);
        // C s_25_17: const #16984u : u32
        let s_25_17: u32 = 16984;
        // N s_25_18: write-reg s_25_17 <= s_25_16
        let s_25_18: () = {
            state.write_register::<bool>(s_25_17 as isize, s_25_16);
            tracer.write_register(s_25_17 as isize, s_25_16);
        };
        // C s_25_19: const #3s : i
        let s_25_19: i128 = 3;
        // D s_25_20: cast zx s_25_8 -> bv
        let s_25_20: Bits = Bits::new(s_25_8 as u128, 5u16);
        // C s_25_21: const #1s : i64
        let s_25_21: i64 = 1;
        // C s_25_22: cast zx s_25_21 -> i
        let s_25_22: i128 = (i128::try_from(s_25_21).unwrap());
        // C s_25_23: const #0s : i
        let s_25_23: i128 = 0;
        // C s_25_24: add s_25_23 s_25_22
        let s_25_24: i128 = (s_25_23 + s_25_22);
        // D s_25_25: bit-extract s_25_20 s_25_19 s_25_24
        let s_25_25: Bits = (Bits::new(
            ((s_25_20) >> (s_25_19)).value(),
            u16::try_from(s_25_24).unwrap(),
        ));
        // D s_25_26: cast reint s_25_25 -> u8
        let s_25_26: bool = ((s_25_25.value()) != 0);
        // C s_25_27: const #16997u : u32
        let s_25_27: u32 = 16997;
        // N s_25_28: write-reg s_25_27 <= s_25_26
        let s_25_28: () = {
            state.write_register::<bool>(s_25_27 as isize, s_25_26);
            tracer.write_register(s_25_27 as isize, s_25_26);
        };
        // C s_25_29: const #2s : i
        let s_25_29: i128 = 2;
        // D s_25_30: cast zx s_25_8 -> bv
        let s_25_30: Bits = Bits::new(s_25_8 as u128, 5u16);
        // C s_25_31: const #1s : i64
        let s_25_31: i64 = 1;
        // C s_25_32: cast zx s_25_31 -> i
        let s_25_32: i128 = (i128::try_from(s_25_31).unwrap());
        // C s_25_33: const #0s : i
        let s_25_33: i128 = 0;
        // C s_25_34: add s_25_33 s_25_32
        let s_25_34: i128 = (s_25_33 + s_25_32);
        // D s_25_35: bit-extract s_25_30 s_25_29 s_25_34
        let s_25_35: Bits = (Bits::new(
            ((s_25_30) >> (s_25_29)).value(),
            u16::try_from(s_25_34).unwrap(),
        ));
        // D s_25_36: cast reint s_25_35 -> u8
        let s_25_36: bool = ((s_25_35.value()) != 0);
        // C s_25_37: const #16971u : u32
        let s_25_37: u32 = 16971;
        // N s_25_38: write-reg s_25_37 <= s_25_36
        let s_25_38: () = {
            state.write_register::<bool>(s_25_37 as isize, s_25_36);
            tracer.write_register(s_25_37 as isize, s_25_36);
        };
        // C s_25_39: const #1s : i
        let s_25_39: i128 = 1;
        // D s_25_40: cast zx s_25_8 -> bv
        let s_25_40: Bits = Bits::new(s_25_8 as u128, 5u16);
        // C s_25_41: const #1s : i64
        let s_25_41: i64 = 1;
        // C s_25_42: cast zx s_25_41 -> i
        let s_25_42: i128 = (i128::try_from(s_25_41).unwrap());
        // C s_25_43: const #0s : i
        let s_25_43: i128 = 0;
        // C s_25_44: add s_25_43 s_25_42
        let s_25_44: i128 = (s_25_43 + s_25_42);
        // D s_25_45: bit-extract s_25_40 s_25_39 s_25_44
        let s_25_45: Bits = (Bits::new(
            ((s_25_40) >> (s_25_39)).value(),
            u16::try_from(s_25_44).unwrap(),
        ));
        // D s_25_46: cast reint s_25_45 -> u8
        let s_25_46: bool = ((s_25_45.value()) != 0);
        // C s_25_47: const #16996u : u32
        let s_25_47: u32 = 16996;
        // N s_25_48: write-reg s_25_47 <= s_25_46
        let s_25_48: () = {
            state.write_register::<bool>(s_25_47 as isize, s_25_46);
            tracer.write_register(s_25_47 as isize, s_25_46);
        };
        // C s_25_49: const #0s : i
        let s_25_49: i128 = 0;
        // D s_25_50: cast zx s_25_8 -> bv
        let s_25_50: Bits = Bits::new(s_25_8 as u128, 5u16);
        // C s_25_51: const #1s : i64
        let s_25_51: i64 = 1;
        // C s_25_52: cast zx s_25_51 -> i
        let s_25_52: i128 = (i128::try_from(s_25_51).unwrap());
        // C s_25_53: const #0s : i
        let s_25_53: i128 = 0;
        // C s_25_54: add s_25_53 s_25_52
        let s_25_54: i128 = (s_25_53 + s_25_52);
        // D s_25_55: bit-extract s_25_50 s_25_49 s_25_54
        let s_25_55: Bits = (Bits::new(
            ((s_25_50) >> (s_25_49)).value(),
            u16::try_from(s_25_54).unwrap(),
        ));
        // D s_25_56: cast reint s_25_55 -> u8
        let s_25_56: bool = ((s_25_55.value()) != 0);
        // C s_25_57: const #16988u : u32
        let s_25_57: u32 = 16988;
        // N s_25_58: write-reg s_25_57 <= s_25_56
        let s_25_58: () = {
            state.write_register::<bool>(s_25_57 as isize, s_25_56);
            tracer.write_register(s_25_57 as isize, s_25_56);
        };
        // N s_25_59: jump b2
        return block_2(state, tracer, fn_state);
    }
}
