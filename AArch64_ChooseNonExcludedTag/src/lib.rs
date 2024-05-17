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
use IsOnes::*;
use common::*;
pub fn AArch64_ChooseNonExcludedTag<T: Tracer>(
    state: &mut State,
    tracer: &T,
    tag_in: u8,
    offset_in: u8,
    exclude: u16,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        tag: u8,
        offset: u8,
        return_value: u8,
        tag_in: u8,
        offset_in: u8,
        exclude: u16,
    }
    let fn_state = FunctionState {
        tag_in,
        offset_in,
        exclude,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_0_0: read-var tag_in:u8
        let s_0_0: u8 = fn_state.tag_in;
        // D s_0_1: write-var tag <= s_0_0
        fn_state.tag = s_0_0;
        // D s_0_2: read-var offset_in:u8
        let s_0_2: u8 = fn_state.offset_in;
        // D s_0_3: write-var offset <= s_0_2
        fn_state.offset = s_0_2;
        // D s_0_4: read-var exclude:u16
        let s_0_4: u16 = fn_state.exclude;
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 16u16);
        // D s_0_6: call IsOnes(s_0_5)
        let s_0_6: bool = IsOnes(state, tracer, s_0_5);
        // N s_0_7: branch s_0_6 b15 b1
        if s_0_6 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_1_0: read-var offset:u8
        let s_1_0: u8 = fn_state.offset;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 4u16);
        // C s_1_2: const #0u : u8
        let s_1_2: u8 = 0;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 4u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b11 b2
        if s_1_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_4_0: read-var offset:u8
        let s_4_0: u8 = fn_state.offset;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 4u16);
        // C s_4_2: const #0u : u8
        let s_4_2: u8 = 0;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 4u16);
        // D s_4_4: cmp-ne s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) != (s_4_3));
        // D s_4_5: not s_4_4
        let s_4_5: bool = !s_4_4;
        // N s_4_6: branch s_4_5 b9 b5
        if s_4_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_5_0: read-var offset:u8
        let s_5_0: u8 = fn_state.offset;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 4u16);
        // C s_5_2: const #1u : u8
        let s_5_2: u8 = 1;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 4u16);
        // D s_5_4: sub s_5_1 s_5_3
        let s_5_4: Bits = ((s_5_1) - (s_5_3));
        // D s_5_5: cast reint s_5_4 -> u8
        let s_5_5: u8 = (s_5_4.value() as u8);
        // D s_5_6: write-var offset <= s_5_5
        fn_state.offset = s_5_5;
        // D s_5_7: read-var tag:u8
        let s_5_7: u8 = fn_state.tag;
        // D s_5_8: cast zx s_5_7 -> bv
        let s_5_8: Bits = Bits::new(s_5_7 as u128, 4u16);
        // C s_5_9: const #1u : u8
        let s_5_9: u8 = 1;
        // C s_5_10: cast zx s_5_9 -> bv
        let s_5_10: Bits = Bits::new(s_5_9 as u128, 4u16);
        // D s_5_11: add s_5_8 s_5_10
        let s_5_11: Bits = (s_5_8 + s_5_10);
        // D s_5_12: cast reint s_5_11 -> u8
        let s_5_12: u8 = (s_5_11.value() as u8);
        // D s_5_13: write-var tag <= s_5_12
        fn_state.tag = s_5_12;
        // N s_5_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_6_0: read-var tag:u8
        let s_6_0: u8 = fn_state.tag;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 4u16);
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (s_6_1.value() as i128);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // D s_6_4: read-var exclude:u16
        let s_6_4: u16 = fn_state.exclude;
        // D s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 16u16);
        // D s_6_6: cast zx s_6_3 -> i
        let s_6_6: i128 = (i128::try_from(s_6_3).unwrap());
        // C s_6_7: const #1u : u64
        let s_6_7: u64 = 1;
        // D s_6_8: bit-extract s_6_5 s_6_6 s_6_7
        let s_6_8: Bits = (Bits::new(
            ((s_6_5) >> (s_6_6)).value(),
            u16::try_from(s_6_7).unwrap(),
        ));
        // D s_6_9: cast reint s_6_8 -> u8
        let s_6_9: bool = ((s_6_8.value()) != 0);
        // C s_6_10: const #0s : i
        let s_6_10: i128 = 0;
        // C s_6_11: const #0u : u64
        let s_6_11: u64 = 0;
        // D s_6_12: cast zx s_6_9 -> u64
        let s_6_12: u64 = (s_6_9 as u64);
        // C s_6_13: const #1u : u64
        let s_6_13: u64 = 1;
        // D s_6_14: and s_6_12 s_6_13
        let s_6_14: u64 = ((s_6_12) & (s_6_13));
        // D s_6_15: cmp-eq s_6_14 s_6_13
        let s_6_15: bool = ((s_6_14) == (s_6_13));
        // D s_6_16: lsl s_6_12 s_6_10
        let s_6_16: u64 = s_6_12 << s_6_10;
        // D s_6_17: or s_6_11 s_6_16
        let s_6_17: u64 = ((s_6_11) | (s_6_16));
        // D s_6_18: cmpl s_6_16
        let s_6_18: u64 = !s_6_16;
        // D s_6_19: and s_6_11 s_6_18
        let s_6_19: u64 = ((s_6_11) & (s_6_18));
        // D s_6_20: select s_6_15 s_6_17 s_6_19
        let s_6_20: u64 = if s_6_15 { s_6_17 } else { s_6_19 };
        // D s_6_21: cast trunc s_6_20 -> u8
        let s_6_21: bool = ((s_6_20) != 0);
        // D s_6_22: cast zx s_6_21 -> bv
        let s_6_22: Bits = Bits::new(s_6_21 as u128, 1u16);
        // C s_6_23: const #1u : u8
        let s_6_23: bool = true;
        // C s_6_24: cast zx s_6_23 -> bv
        let s_6_24: Bits = Bits::new(s_6_23 as u128, 1u16);
        // D s_6_25: cmp-eq s_6_22 s_6_24
        let s_6_25: bool = ((s_6_22) == (s_6_24));
        // D s_6_26: not s_6_25
        let s_6_26: bool = !s_6_25;
        // N s_6_27: branch s_6_26 b8 b7
        if s_6_26 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_7_0: read-var tag:u8
        let s_7_0: u8 = fn_state.tag;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 4u16);
        // C s_7_2: const #1u : u8
        let s_7_2: u8 = 1;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 4u16);
        // D s_7_4: add s_7_1 s_7_3
        let s_7_4: Bits = (s_7_1 + s_7_3);
        // D s_7_5: cast reint s_7_4 -> u8
        let s_7_5: u8 = (s_7_4.value() as u8);
        // D s_7_6: write-var tag <= s_7_5
        fn_state.tag = s_7_5;
        // N s_7_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_8_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_9_0: read-var tag:u8
        let s_9_0: u8 = fn_state.tag;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_10_0: read-var return_value:u8
        let s_10_0: u8 = fn_state.return_value;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_12_0: read-var tag:u8
        let s_12_0: u8 = fn_state.tag;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 4u16);
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (s_12_1.value() as i128);
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // D s_12_4: read-var exclude:u16
        let s_12_4: u16 = fn_state.exclude;
        // D s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 16u16);
        // D s_12_6: cast zx s_12_3 -> i
        let s_12_6: i128 = (i128::try_from(s_12_3).unwrap());
        // C s_12_7: const #1u : u64
        let s_12_7: u64 = 1;
        // D s_12_8: bit-extract s_12_5 s_12_6 s_12_7
        let s_12_8: Bits = (Bits::new(
            ((s_12_5) >> (s_12_6)).value(),
            u16::try_from(s_12_7).unwrap(),
        ));
        // D s_12_9: cast reint s_12_8 -> u8
        let s_12_9: bool = ((s_12_8.value()) != 0);
        // C s_12_10: const #0s : i
        let s_12_10: i128 = 0;
        // C s_12_11: const #0u : u64
        let s_12_11: u64 = 0;
        // D s_12_12: cast zx s_12_9 -> u64
        let s_12_12: u64 = (s_12_9 as u64);
        // C s_12_13: const #1u : u64
        let s_12_13: u64 = 1;
        // D s_12_14: and s_12_12 s_12_13
        let s_12_14: u64 = ((s_12_12) & (s_12_13));
        // D s_12_15: cmp-eq s_12_14 s_12_13
        let s_12_15: bool = ((s_12_14) == (s_12_13));
        // D s_12_16: lsl s_12_12 s_12_10
        let s_12_16: u64 = s_12_12 << s_12_10;
        // D s_12_17: or s_12_11 s_12_16
        let s_12_17: u64 = ((s_12_11) | (s_12_16));
        // D s_12_18: cmpl s_12_16
        let s_12_18: u64 = !s_12_16;
        // D s_12_19: and s_12_11 s_12_18
        let s_12_19: u64 = ((s_12_11) & (s_12_18));
        // D s_12_20: select s_12_15 s_12_17 s_12_19
        let s_12_20: u64 = if s_12_15 { s_12_17 } else { s_12_19 };
        // D s_12_21: cast trunc s_12_20 -> u8
        let s_12_21: bool = ((s_12_20) != 0);
        // D s_12_22: cast zx s_12_21 -> bv
        let s_12_22: Bits = Bits::new(s_12_21 as u128, 1u16);
        // C s_12_23: const #1u : u8
        let s_12_23: bool = true;
        // C s_12_24: cast zx s_12_23 -> bv
        let s_12_24: Bits = Bits::new(s_12_23 as u128, 1u16);
        // D s_12_25: cmp-eq s_12_22 s_12_24
        let s_12_25: bool = ((s_12_22) == (s_12_24));
        // D s_12_26: not s_12_25
        let s_12_26: bool = !s_12_25;
        // N s_12_27: branch s_12_26 b14 b13
        if s_12_26 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_13_0: read-var tag:u8
        let s_13_0: u8 = fn_state.tag;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 4u16);
        // C s_13_2: const #1u : u8
        let s_13_2: u8 = 1;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 4u16);
        // D s_13_4: add s_13_1 s_13_3
        let s_13_4: Bits = (s_13_1 + s_13_3);
        // D s_13_5: cast reint s_13_4 -> u8
        let s_13_5: u8 = (s_13_4.value() as u8);
        // D s_13_6: write-var tag <= s_13_5
        fn_state.tag = s_13_5;
        // N s_13_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_14_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_15_0: const #0u : u8
        let s_15_0: u8 = 0;
        // D s_15_1: write-var return_value <= s_15_0
        fn_state.return_value = s_15_0;
        // N s_15_2: jump b10
        return block_10(state, tracer, fn_state);
    }
}
