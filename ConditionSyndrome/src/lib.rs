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
use AArch32_CurrentCond::*;
use UsingAArch32::*;
use Bit::*;
use u__IMPDEF_boolean::*;
use u__UNKNOWN_bits::*;
use ConditionHolds::*;
use ConstrainUnpredictableBool::*;
use common::*;
pub fn ConditionSyndrome<T: Tracer>(state: &mut State, tracer: &T, gs_6542: ()) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        condshadow_82: u8,
        gs_6559: bool,
        syndrome: u8,
        gs_6542: (),
    }
    let fn_state = FunctionState {
        gs_6542,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call UsingAArch32(s_0_0)
        let s_0_1: bool = UsingAArch32(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b3 b1
        if s_0_1 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #1u : u8
        let s_1_0: bool = true;
        // S s_1_1: call Bit(s_1_0)
        let s_1_1: bool = Bit(state, tracer, s_1_0);
        // C s_1_2: const #4s : i
        let s_1_2: i128 = 4;
        // D s_1_3: read-var syndrome:u8
        let s_1_3: u8 = fn_state.syndrome;
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 5u16);
        // C s_1_5: const #1u : u64
        let s_1_5: u64 = 1;
        // D s_1_6: bit-insert s_1_4 s_1_4 s_1_2 s_1_5
        let s_1_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_1_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_1_4.length(),
            );
            (s_1_4 & mask) | (s_1_4 << s_1_2)
        };
        // D s_1_7: cast reint s_1_6 -> u8
        let s_1_7: u8 = (s_1_6.value() as u8);
        // D s_1_8: write-var syndrome <= s_1_7
        fn_state.syndrome = s_1_7;
        // C s_1_9: const #0s : i
        let s_1_9: i128 = 0;
        // D s_1_10: read-var syndrome:u8
        let s_1_10: u8 = fn_state.syndrome;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 5u16);
        // C s_1_12: const #14u : u8
        let s_1_12: u8 = 14;
        // C s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 4u16);
        // C s_1_14: const #3s : i
        let s_1_14: i128 = 3;
        // C s_1_15: const #1u : u64
        let s_1_15: u64 = 1;
        // C s_1_16: cast zx s_1_15 -> bv
        let s_1_16: Bits = Bits::new(s_1_15 as u128, 64u16);
        // C s_1_17: lsl s_1_16 s_1_14
        let s_1_17: Bits = s_1_16 << s_1_14;
        // C s_1_18: sub s_1_17 s_1_16
        let s_1_18: Bits = ((s_1_17) - (s_1_16));
        // C s_1_19: and s_1_13 s_1_18
        let s_1_19: Bits = ((s_1_13) & (s_1_18));
        // C s_1_20: lsl s_1_19 s_1_9
        let s_1_20: Bits = s_1_19 << s_1_9;
        // C s_1_21: lsl s_1_18 s_1_9
        let s_1_21: Bits = s_1_18 << s_1_9;
        // C s_1_22: cmpl s_1_21
        let s_1_22: Bits = !s_1_21;
        // D s_1_23: and s_1_11 s_1_22
        let s_1_23: Bits = ((s_1_11) & (s_1_22));
        // D s_1_24: or s_1_23 s_1_20
        let s_1_24: Bits = ((s_1_23) | (s_1_20));
        // D s_1_25: cast reint s_1_24 -> u8
        let s_1_25: u8 = (s_1_24.value() as u8);
        // D s_1_26: write-var syndrome <= s_1_25
        fn_state.syndrome = s_1_25;
        // N s_1_27: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var syndrome:u8
        let s_2_0: u8 = fn_state.syndrome;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call AArch32_CurrentCond(s_3_0)
        let s_3_1: u8 = AArch32_CurrentCond(state, tracer, s_3_0);
        // D s_3_2: write-var condshadow#82 <= s_3_1
        fn_state.condshadow_82 = s_3_1;
        // C s_3_3: const #16993u : u32
        let s_3_3: u32 = 16993;
        // D s_3_4: read-reg s_3_3:u8
        let s_3_4: bool = {
            let value = state.read_register::<bool>(s_3_3 as isize);
            tracer.read_register(s_3_3 as isize, value);
            value
        };
        // D s_3_5: cast zx s_3_4 -> bv
        let s_3_5: Bits = Bits::new(s_3_4 as u128, 1u16);
        // C s_3_6: const #0u : u8
        let s_3_6: bool = false;
        // C s_3_7: cast zx s_3_6 -> bv
        let s_3_7: Bits = Bits::new(s_3_6 as u128, 1u16);
        // D s_3_8: cmp-eq s_3_5 s_3_7
        let s_3_8: bool = ((s_3_5) == (s_3_7));
        // N s_3_9: branch s_3_8 b7 b4
        if s_3_8 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_4_0: const #"Condition valid for trapped T32" : str
        let s_4_0: &'static str = "Condition valid for trapped T32";
        // S s_4_1: call __IMPDEF_boolean(s_4_0)
        let s_4_1: bool = u__IMPDEF_boolean(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b6 b5
        if s_4_1 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // S s_5_1: call Bit(s_5_0)
        let s_5_1: bool = Bit(state, tracer, s_5_0);
        // C s_5_2: const #4s : i
        let s_5_2: i128 = 4;
        // D s_5_3: read-var syndrome:u8
        let s_5_3: u8 = fn_state.syndrome;
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 5u16);
        // C s_5_5: const #1u : u64
        let s_5_5: u64 = 1;
        // D s_5_6: bit-insert s_5_4 s_5_4 s_5_2 s_5_5
        let s_5_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_5_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_5_4.length(),
            );
            (s_5_4 & mask) | (s_5_4 << s_5_2)
        };
        // D s_5_7: cast reint s_5_6 -> u8
        let s_5_7: u8 = (s_5_6.value() as u8);
        // D s_5_8: write-var syndrome <= s_5_7
        fn_state.syndrome = s_5_7;
        // C s_5_9: const #4s : i64
        let s_5_9: i64 = 4;
        // C s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // S s_5_11: call __UNKNOWN_bits(s_5_10)
        let s_5_11: Bits = u__UNKNOWN_bits(state, tracer, s_5_10);
        // S s_5_12: cast reint s_5_11 -> u8
        let s_5_12: u8 = (s_5_11.value() as u8);
        // C s_5_13: const #0s : i
        let s_5_13: i128 = 0;
        // D s_5_14: read-var syndrome:u8
        let s_5_14: u8 = fn_state.syndrome;
        // D s_5_15: cast zx s_5_14 -> bv
        let s_5_15: Bits = Bits::new(s_5_14 as u128, 5u16);
        // S s_5_16: cast zx s_5_12 -> bv
        let s_5_16: Bits = Bits::new(s_5_12 as u128, 4u16);
        // C s_5_17: const #3s : i
        let s_5_17: i128 = 3;
        // C s_5_18: const #1u : u64
        let s_5_18: u64 = 1;
        // C s_5_19: cast zx s_5_18 -> bv
        let s_5_19: Bits = Bits::new(s_5_18 as u128, 64u16);
        // C s_5_20: lsl s_5_19 s_5_17
        let s_5_20: Bits = s_5_19 << s_5_17;
        // C s_5_21: sub s_5_20 s_5_19
        let s_5_21: Bits = ((s_5_20) - (s_5_19));
        // S s_5_22: and s_5_16 s_5_21
        let s_5_22: Bits = ((s_5_16) & (s_5_21));
        // S s_5_23: lsl s_5_22 s_5_13
        let s_5_23: Bits = s_5_22 << s_5_13;
        // C s_5_24: lsl s_5_21 s_5_13
        let s_5_24: Bits = s_5_21 << s_5_13;
        // C s_5_25: cmpl s_5_24
        let s_5_25: Bits = !s_5_24;
        // D s_5_26: and s_5_15 s_5_25
        let s_5_26: Bits = ((s_5_15) & (s_5_25));
        // D s_5_27: or s_5_26 s_5_23
        let s_5_27: Bits = ((s_5_26) | (s_5_23));
        // D s_5_28: cast reint s_5_27 -> u8
        let s_5_28: u8 = (s_5_27.value() as u8);
        // D s_5_29: write-var syndrome <= s_5_28
        fn_state.syndrome = s_5_28;
        // N s_5_30: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // S s_6_1: call Bit(s_6_0)
        let s_6_1: bool = Bit(state, tracer, s_6_0);
        // C s_6_2: const #4s : i
        let s_6_2: i128 = 4;
        // D s_6_3: read-var syndrome:u8
        let s_6_3: u8 = fn_state.syndrome;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 5u16);
        // C s_6_5: const #1u : u64
        let s_6_5: u64 = 1;
        // D s_6_6: bit-insert s_6_4 s_6_4 s_6_2 s_6_5
        let s_6_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_6_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_6_4.length(),
            );
            (s_6_4 & mask) | (s_6_4 << s_6_2)
        };
        // D s_6_7: cast reint s_6_6 -> u8
        let s_6_7: u8 = (s_6_6.value() as u8);
        // D s_6_8: write-var syndrome <= s_6_7
        fn_state.syndrome = s_6_7;
        // C s_6_9: const #0s : i
        let s_6_9: i128 = 0;
        // D s_6_10: read-var syndrome:u8
        let s_6_10: u8 = fn_state.syndrome;
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 5u16);
        // D s_6_12: read-var condshadow#82:u8
        let s_6_12: u8 = fn_state.condshadow_82;
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 4u16);
        // C s_6_14: const #3s : i
        let s_6_14: i128 = 3;
        // C s_6_15: const #1u : u64
        let s_6_15: u64 = 1;
        // C s_6_16: cast zx s_6_15 -> bv
        let s_6_16: Bits = Bits::new(s_6_15 as u128, 64u16);
        // C s_6_17: lsl s_6_16 s_6_14
        let s_6_17: Bits = s_6_16 << s_6_14;
        // C s_6_18: sub s_6_17 s_6_16
        let s_6_18: Bits = ((s_6_17) - (s_6_16));
        // D s_6_19: and s_6_13 s_6_18
        let s_6_19: Bits = ((s_6_13) & (s_6_18));
        // D s_6_20: lsl s_6_19 s_6_9
        let s_6_20: Bits = s_6_19 << s_6_9;
        // C s_6_21: lsl s_6_18 s_6_9
        let s_6_21: Bits = s_6_18 << s_6_9;
        // C s_6_22: cmpl s_6_21
        let s_6_22: Bits = !s_6_21;
        // D s_6_23: and s_6_11 s_6_22
        let s_6_23: Bits = ((s_6_11) & (s_6_22));
        // D s_6_24: or s_6_23 s_6_20
        let s_6_24: Bits = ((s_6_23) | (s_6_20));
        // D s_6_25: cast reint s_6_24 -> u8
        let s_6_25: u8 = (s_6_24.value() as u8);
        // D s_6_26: write-var syndrome <= s_6_25
        fn_state.syndrome = s_6_25;
        // N s_6_27: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // S s_7_1: call Bit(s_7_0)
        let s_7_1: bool = Bit(state, tracer, s_7_0);
        // C s_7_2: const #4s : i
        let s_7_2: i128 = 4;
        // D s_7_3: read-var syndrome:u8
        let s_7_3: u8 = fn_state.syndrome;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 5u16);
        // C s_7_5: const #1u : u64
        let s_7_5: u64 = 1;
        // D s_7_6: bit-insert s_7_4 s_7_4 s_7_2 s_7_5
        let s_7_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_7_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_7_4.length(),
            );
            (s_7_4 & mask) | (s_7_4 << s_7_2)
        };
        // D s_7_7: cast reint s_7_6 -> u8
        let s_7_7: u8 = (s_7_6.value() as u8);
        // D s_7_8: write-var syndrome <= s_7_7
        fn_state.syndrome = s_7_7;
        // D s_7_9: read-var condshadow#82:u8
        let s_7_9: u8 = fn_state.condshadow_82;
        // D s_7_10: call ConditionHolds(s_7_9)
        let s_7_10: bool = ConditionHolds(state, tracer, s_7_9);
        // N s_7_11: branch s_7_10 b12 b8
        if s_7_10 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#6559 <= s_8_0
        fn_state.gs_6559 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_9_0: read-var gs#6559:u8
        let s_9_0: bool = fn_state.gs_6559;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_10_0: const #0s : i
        let s_10_0: i128 = 0;
        // D s_10_1: read-var syndrome:u8
        let s_10_1: u8 = fn_state.syndrome;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 5u16);
        // D s_10_3: read-var condshadow#82:u8
        let s_10_3: u8 = fn_state.condshadow_82;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 4u16);
        // C s_10_5: const #3s : i
        let s_10_5: i128 = 3;
        // C s_10_6: const #1u : u64
        let s_10_6: u64 = 1;
        // C s_10_7: cast zx s_10_6 -> bv
        let s_10_7: Bits = Bits::new(s_10_6 as u128, 64u16);
        // C s_10_8: lsl s_10_7 s_10_5
        let s_10_8: Bits = s_10_7 << s_10_5;
        // C s_10_9: sub s_10_8 s_10_7
        let s_10_9: Bits = ((s_10_8) - (s_10_7));
        // D s_10_10: and s_10_4 s_10_9
        let s_10_10: Bits = ((s_10_4) & (s_10_9));
        // D s_10_11: lsl s_10_10 s_10_0
        let s_10_11: Bits = s_10_10 << s_10_0;
        // C s_10_12: lsl s_10_9 s_10_0
        let s_10_12: Bits = s_10_9 << s_10_0;
        // C s_10_13: cmpl s_10_12
        let s_10_13: Bits = !s_10_12;
        // D s_10_14: and s_10_2 s_10_13
        let s_10_14: Bits = ((s_10_2) & (s_10_13));
        // D s_10_15: or s_10_14 s_10_11
        let s_10_15: Bits = ((s_10_14) | (s_10_11));
        // D s_10_16: cast reint s_10_15 -> u8
        let s_10_16: u8 = (s_10_15.value() as u8);
        // D s_10_17: write-var syndrome <= s_10_16
        fn_state.syndrome = s_10_16;
        // N s_10_18: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var syndrome:u8
        let s_11_1: u8 = fn_state.syndrome;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 5u16);
        // C s_11_3: const #14u : u8
        let s_11_3: u8 = 14;
        // C s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 4u16);
        // C s_11_5: const #3s : i
        let s_11_5: i128 = 3;
        // C s_11_6: const #1u : u64
        let s_11_6: u64 = 1;
        // C s_11_7: cast zx s_11_6 -> bv
        let s_11_7: Bits = Bits::new(s_11_6 as u128, 64u16);
        // C s_11_8: lsl s_11_7 s_11_5
        let s_11_8: Bits = s_11_7 << s_11_5;
        // C s_11_9: sub s_11_8 s_11_7
        let s_11_9: Bits = ((s_11_8) - (s_11_7));
        // C s_11_10: and s_11_4 s_11_9
        let s_11_10: Bits = ((s_11_4) & (s_11_9));
        // C s_11_11: lsl s_11_10 s_11_0
        let s_11_11: Bits = s_11_10 << s_11_0;
        // C s_11_12: lsl s_11_9 s_11_0
        let s_11_12: Bits = s_11_9 << s_11_0;
        // C s_11_13: cmpl s_11_12
        let s_11_13: Bits = !s_11_12;
        // D s_11_14: and s_11_2 s_11_13
        let s_11_14: Bits = ((s_11_2) & (s_11_13));
        // D s_11_15: or s_11_14 s_11_11
        let s_11_15: Bits = ((s_11_14) | (s_11_11));
        // D s_11_16: cast reint s_11_15 -> u8
        let s_11_16: u8 = (s_11_15.value() as u8);
        // D s_11_17: write-var syndrome <= s_11_16
        fn_state.syndrome = s_11_16;
        // N s_11_18: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_12_0: const #21u : u32
        let s_12_0: u32 = 21;
        // S s_12_1: call ConstrainUnpredictableBool(s_12_0)
        let s_12_1: bool = ConstrainUnpredictableBool(state, tracer, s_12_0);
        // D s_12_2: write-var gs#6559 <= s_12_1
        fn_state.gs_6559 = s_12_1;
        // N s_12_3: jump b9
        return block_9(state, tracer, fn_state);
    }
}
