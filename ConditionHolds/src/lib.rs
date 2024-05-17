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
use common::*;
pub fn ConditionHolds<T: Tracer>(state: &mut State, tracer: &T, cond: u8) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_5674: bool,
        gs_5671: bool,
        ga_3760: u8,
        result: bool,
        gs_5679: bool,
        cond: u8,
    }
    let fn_state = FunctionState {
        cond,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #1s : i
        let s_0_0: i128 = 1;
        // D s_0_1: read-var cond:u8
        let s_0_1: u8 = fn_state.cond;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 4u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #2s : i
        let s_0_5: i128 = 2;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_0 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: u8 = (s_0_7.value() as u8);
        // D s_0_9: write-var ga#3760 <= s_0_8
        fn_state.ga_3760 = s_0_8;
        // D s_0_10: read-var ga#3760:u8
        let s_0_10: u8 = fn_state.ga_3760;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 3u16);
        // C s_0_12: const #0u : u8
        let s_0_12: u8 = 0;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 3u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // D s_0_15: not s_0_14
        let s_0_15: bool = !s_0_14;
        // N s_0_16: branch s_0_15 b9 b1
        if s_0_15 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #16997u : u32
        let s_1_0: u32 = 16997;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: bool = {
            let value = state.read_register::<bool>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 1u16);
        // C s_1_3: const #1u : u8
        let s_1_3: bool = true;
        // C s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 1u16);
        // D s_1_5: cmp-eq s_1_2 s_1_4
        let s_1_5: bool = ((s_1_2) == (s_1_4));
        // D s_1_6: write-var result <= s_1_5
        fn_state.result = s_1_5;
        // N s_1_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0s : i
        let s_2_0: i128 = 0;
        // D s_2_1: read-var cond:u8
        let s_2_1: u8 = fn_state.cond;
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
        // N s_2_22: branch s_2_21 b8 b3
        if s_2_21 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#5679 <= s_3_0
        fn_state.gs_5679 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#5679:u8
        let s_4_0: bool = fn_state.gs_5679;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var result:u8
        let s_6_0: bool = fn_state.result;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var result:u8
        let s_7_0: bool = fn_state.result;
        // D s_7_1: not s_7_0
        let s_7_1: bool = !s_7_0;
        // D s_7_2: write-var result <= s_7_1
        fn_state.result = s_7_1;
        // N s_7_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var cond:u8
        let s_8_0: u8 = fn_state.cond;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 4u16);
        // C s_8_2: const #15u : u8
        let s_8_2: u8 = 15;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 4u16);
        // D s_8_4: cmp-ne s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) != (s_8_3));
        // D s_8_5: write-var gs#5679 <= s_8_4
        fn_state.gs_5679 = s_8_4;
        // N s_8_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var ga#3760:u8
        let s_9_0: u8 = fn_state.ga_3760;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 3u16);
        // C s_9_2: const #1u : u8
        let s_9_2: u8 = 1;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 3u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: not s_9_4
        let s_9_5: bool = !s_9_4;
        // N s_9_6: branch s_9_5 b11 b10
        if s_9_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #16971u : u32
        let s_10_0: u32 = 16971;
        // D s_10_1: read-reg s_10_0:u8
        let s_10_1: bool = {
            let value = state.read_register::<bool>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 1u16);
        // C s_10_3: const #1u : u8
        let s_10_3: bool = true;
        // C s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 1u16);
        // D s_10_5: cmp-eq s_10_2 s_10_4
        let s_10_5: bool = ((s_10_2) == (s_10_4));
        // D s_10_6: write-var result <= s_10_5
        fn_state.result = s_10_5;
        // N s_10_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var ga#3760:u8
        let s_11_0: u8 = fn_state.ga_3760;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 3u16);
        // C s_11_2: const #2u : u8
        let s_11_2: u8 = 2;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 3u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: not s_11_4
        let s_11_5: bool = !s_11_4;
        // N s_11_6: branch s_11_5 b13 b12
        if s_11_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #16984u : u32
        let s_12_0: u32 = 16984;
        // D s_12_1: read-reg s_12_0:u8
        let s_12_1: bool = {
            let value = state.read_register::<bool>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 1u16);
        // C s_12_3: const #1u : u8
        let s_12_3: bool = true;
        // C s_12_4: cast zx s_12_3 -> bv
        let s_12_4: Bits = Bits::new(s_12_3 as u128, 1u16);
        // D s_12_5: cmp-eq s_12_2 s_12_4
        let s_12_5: bool = ((s_12_2) == (s_12_4));
        // D s_12_6: write-var result <= s_12_5
        fn_state.result = s_12_5;
        // N s_12_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var ga#3760:u8
        let s_13_0: u8 = fn_state.ga_3760;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 3u16);
        // C s_13_2: const #3u : u8
        let s_13_2: u8 = 3;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 3u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b15 b14
        if s_13_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #16996u : u32
        let s_14_0: u32 = 16996;
        // D s_14_1: read-reg s_14_0:u8
        let s_14_1: bool = {
            let value = state.read_register::<bool>(s_14_0 as isize);
            tracer.read_register(s_14_0 as isize, value);
            value
        };
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 1u16);
        // C s_14_3: const #1u : u8
        let s_14_3: bool = true;
        // C s_14_4: cast zx s_14_3 -> bv
        let s_14_4: Bits = Bits::new(s_14_3 as u128, 1u16);
        // D s_14_5: cmp-eq s_14_2 s_14_4
        let s_14_5: bool = ((s_14_2) == (s_14_4));
        // D s_14_6: write-var result <= s_14_5
        fn_state.result = s_14_5;
        // N s_14_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var ga#3760:u8
        let s_15_0: u8 = fn_state.ga_3760;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 3u16);
        // C s_15_2: const #4u : u8
        let s_15_2: u8 = 4;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 3u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: not s_15_4
        let s_15_5: bool = !s_15_4;
        // N s_15_6: branch s_15_5 b20 b16
        if s_15_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_16_0: const #16971u : u32
        let s_16_0: u32 = 16971;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: bool = {
            let value = state.read_register::<bool>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 1u16);
        // C s_16_3: const #1u : u8
        let s_16_3: bool = true;
        // C s_16_4: cast zx s_16_3 -> bv
        let s_16_4: Bits = Bits::new(s_16_3 as u128, 1u16);
        // D s_16_5: cmp-eq s_16_2 s_16_4
        let s_16_5: bool = ((s_16_2) == (s_16_4));
        // N s_16_6: branch s_16_5 b19 b17
        if s_16_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#5671 <= s_17_0
        fn_state.gs_5671 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#5671:u8
        let s_18_0: bool = fn_state.gs_5671;
        // D s_18_1: write-var result <= s_18_0
        fn_state.result = s_18_0;
        // N s_18_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #16997u : u32
        let s_19_0: u32 = 16997;
        // D s_19_1: read-reg s_19_0:u8
        let s_19_1: bool = {
            let value = state.read_register::<bool>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: cast zx s_19_1 -> bv
        let s_19_2: Bits = Bits::new(s_19_1 as u128, 1u16);
        // C s_19_3: const #0u : u8
        let s_19_3: bool = false;
        // C s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 1u16);
        // D s_19_5: cmp-eq s_19_2 s_19_4
        let s_19_5: bool = ((s_19_2) == (s_19_4));
        // D s_19_6: write-var gs#5671 <= s_19_5
        fn_state.gs_5671 = s_19_5;
        // N s_19_7: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var ga#3760:u8
        let s_20_0: u8 = fn_state.ga_3760;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 3u16);
        // C s_20_2: const #5u : u8
        let s_20_2: u8 = 5;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 3u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: not s_20_4
        let s_20_5: bool = !s_20_4;
        // N s_20_6: branch s_20_5 b22 b21
        if s_20_5 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_21_0: const #16984u : u32
        let s_21_0: u32 = 16984;
        // D s_21_1: read-reg s_21_0:u8
        let s_21_1: bool = {
            let value = state.read_register::<bool>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // C s_21_2: const #16996u : u32
        let s_21_2: u32 = 16996;
        // D s_21_3: read-reg s_21_2:u8
        let s_21_3: bool = {
            let value = state.read_register::<bool>(s_21_2 as isize);
            tracer.read_register(s_21_2 as isize, value);
            value
        };
        // D s_21_4: cast zx s_21_1 -> bv
        let s_21_4: Bits = Bits::new(s_21_1 as u128, 1u16);
        // D s_21_5: cast zx s_21_3 -> bv
        let s_21_5: Bits = Bits::new(s_21_3 as u128, 1u16);
        // D s_21_6: cmp-eq s_21_4 s_21_5
        let s_21_6: bool = ((s_21_4) == (s_21_5));
        // D s_21_7: write-var result <= s_21_6
        fn_state.result = s_21_6;
        // N s_21_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var ga#3760:u8
        let s_22_0: u8 = fn_state.ga_3760;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 3u16);
        // C s_22_2: const #6u : u8
        let s_22_2: u8 = 6;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 3u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: not s_22_4
        let s_22_5: bool = !s_22_4;
        // N s_22_6: branch s_22_5 b27 b23
        if s_22_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_23_0: const #16984u : u32
        let s_23_0: u32 = 16984;
        // D s_23_1: read-reg s_23_0:u8
        let s_23_1: bool = {
            let value = state.read_register::<bool>(s_23_0 as isize);
            tracer.read_register(s_23_0 as isize, value);
            value
        };
        // C s_23_2: const #16996u : u32
        let s_23_2: u32 = 16996;
        // D s_23_3: read-reg s_23_2:u8
        let s_23_3: bool = {
            let value = state.read_register::<bool>(s_23_2 as isize);
            tracer.read_register(s_23_2 as isize, value);
            value
        };
        // D s_23_4: cast zx s_23_1 -> bv
        let s_23_4: Bits = Bits::new(s_23_1 as u128, 1u16);
        // D s_23_5: cast zx s_23_3 -> bv
        let s_23_5: Bits = Bits::new(s_23_3 as u128, 1u16);
        // D s_23_6: cmp-eq s_23_4 s_23_5
        let s_23_6: bool = ((s_23_4) == (s_23_5));
        // N s_23_7: branch s_23_6 b26 b24
        if s_23_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#5674 <= s_24_0
        fn_state.gs_5674 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var gs#5674:u8
        let s_25_0: bool = fn_state.gs_5674;
        // D s_25_1: write-var result <= s_25_0
        fn_state.result = s_25_0;
        // N s_25_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #16997u : u32
        let s_26_0: u32 = 16997;
        // D s_26_1: read-reg s_26_0:u8
        let s_26_1: bool = {
            let value = state.read_register::<bool>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 1u16);
        // C s_26_3: const #0u : u8
        let s_26_3: bool = false;
        // C s_26_4: cast zx s_26_3 -> bv
        let s_26_4: Bits = Bits::new(s_26_3 as u128, 1u16);
        // D s_26_5: cmp-eq s_26_2 s_26_4
        let s_26_5: bool = ((s_26_2) == (s_26_4));
        // D s_26_6: write-var gs#5674 <= s_26_5
        fn_state.gs_5674 = s_26_5;
        // N s_26_7: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var result <= s_27_0
        fn_state.result = s_27_0;
        // N s_27_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
