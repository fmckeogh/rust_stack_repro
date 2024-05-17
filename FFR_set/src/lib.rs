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
use ConstrainUnpredictableBool::*;
use CurrentVL_read::*;
use common::*;
pub fn FFR_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    width: i128,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        widthshadow_43: i128,
        width: i128,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        width,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var width:i
        let s_0_0: i128 = fn_state.width;
        // D s_0_1: write-var widthshadow#43 <= s_0_0
        fn_state.widthshadow_43 = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call CurrentVL_read(s_0_2)
        let s_0_3: i64 = CurrentVL_read(state, tracer, s_0_2);
        // C s_0_4: const #8s : i
        let s_0_4: i128 = 8;
        // S s_0_5: cast zx s_0_3 -> i
        let s_0_5: i128 = (i128::try_from(s_0_3).unwrap());
        // S s_0_6: div s_0_5 s_0_4
        let s_0_6: i128 = ((s_0_5) / (s_0_4));
        // S s_0_7: cast reint s_0_6 -> i64
        let s_0_7: i64 = (s_0_6 as i64);
        // S s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: read-var widthshadow#43:i
        let s_0_9: i128 = fn_state.widthshadow_43;
        // D s_0_10: cmp-eq s_0_9 s_0_8
        let s_0_10: bool = ((s_0_9) == (s_0_8));
        // N s_0_11: assert s_0_10
        let s_0_11: () = assert!(s_0_10);
        // C s_0_12: const #50u : u32
        let s_0_12: u32 = 50;
        // S s_0_13: call ConstrainUnpredictableBool(s_0_12)
        let s_0_13: bool = ConstrainUnpredictableBool(state, tracer, s_0_12);
        // N s_0_14: branch s_0_13 b2 b1
        if s_0_13 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #1s : i
        let s_1_0: i128 = 1;
        // D s_1_1: read-var widthshadow#43:i
        let s_1_1: i128 = fn_state.widthshadow_43;
        // D s_1_2: sub s_1_1 s_1_0
        let s_1_2: i128 = ((s_1_1) - (s_1_0));
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #0s : i
        let s_1_4: i128 = 0;
        // C s_1_5: const #14552u : u32
        let s_1_5: u32 = 14552;
        // D s_1_6: read-reg s_1_5:u256
        let s_1_6: u64 = {
            let value = state.read_register::<u64>(s_1_5 as isize);
            tracer.read_register(s_1_5 as isize, value);
            value
        };
        // D s_1_7: cast zx s_1_6 -> bv
        let s_1_7: Bits = Bits::new(s_1_6 as u128, 256u16);
        // D s_1_8: cast zx s_1_3 -> i
        let s_1_8: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_9: read-var value_name:bv
        let s_1_9: Bits = fn_state.value_name;
        // D s_1_10: sub s_1_8 s_1_4
        let s_1_10: i128 = ((s_1_8) - (s_1_4));
        // C s_1_11: const #1u : u64
        let s_1_11: u64 = 1;
        // C s_1_12: cast zx s_1_11 -> bv
        let s_1_12: Bits = Bits::new(s_1_11 as u128, 64u16);
        // D s_1_13: lsl s_1_12 s_1_10
        let s_1_13: Bits = s_1_12 << s_1_10;
        // D s_1_14: sub s_1_13 s_1_12
        let s_1_14: Bits = ((s_1_13) - (s_1_12));
        // D s_1_15: and s_1_9 s_1_14
        let s_1_15: Bits = ((s_1_9) & (s_1_14));
        // D s_1_16: lsl s_1_15 s_1_4
        let s_1_16: Bits = s_1_15 << s_1_4;
        // D s_1_17: lsl s_1_14 s_1_4
        let s_1_17: Bits = s_1_14 << s_1_4;
        // D s_1_18: cmpl s_1_17
        let s_1_18: Bits = !s_1_17;
        // D s_1_19: and s_1_7 s_1_18
        let s_1_19: Bits = ((s_1_7) & (s_1_18));
        // D s_1_20: or s_1_19 s_1_16
        let s_1_20: Bits = ((s_1_19) | (s_1_16));
        // D s_1_21: cast reint s_1_20 -> u256
        let s_1_21: u64 = (s_1_20.value() as u64);
        // C s_1_22: const #14552u : u32
        let s_1_22: u32 = 14552;
        // N s_1_23: write-reg s_1_22 <= s_1_21
        let s_1_23: () = {
            state.write_register::<u64>(s_1_22 as isize, s_1_21);
            tracer.write_register(s_1_22 as isize, s_1_21);
        };
        // N s_1_24: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #816u : u32
        let s_2_0: u32 = 816;
        // D s_2_1: read-reg s_2_0:i64
        let s_2_1: i64 = {
            let value = state.read_register::<i64>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: read-var value_name:bv
        let s_2_3: Bits = fn_state.value_name;
        // D s_2_4: bits-cast zx s_2_3 -> bv length s_2_2
        let s_2_4: Bits = s_2_3.zero_extend(s_2_2);
        // D s_2_5: cast reint s_2_4 -> u256
        let s_2_5: u64 = (s_2_4.value() as u64);
        // C s_2_6: const #14552u : u32
        let s_2_6: u32 = 14552;
        // N s_2_7: write-reg s_2_6 <= s_2_5
        let s_2_7: () = {
            state.write_register::<u64>(s_2_6 as isize, s_2_5);
            tracer.write_register(s_2_6 as isize, s_2_5);
        };
        // N s_2_8: return
        return;
    }
}
