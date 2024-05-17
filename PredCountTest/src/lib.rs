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
use neq_int::*;
use common::*;
pub fn PredCountTest<T: Tracer>(
    state: &mut State,
    tracer: &T,
    elements: i128,
    count: i128,
    invert: bool,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        c: bool,
        z: bool,
        n: bool,
        elements: i128,
        count: i128,
        invert: bool,
    }
    let fn_state = FunctionState {
        elements,
        count,
        invert,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var count:i
        let s_0_1: i128 = fn_state.count;
        // D s_0_2: cmp-eq s_0_1 s_0_0
        let s_0_2: bool = ((s_0_1) == (s_0_0));
        // N s_0_3: branch s_0_2 b18 b1
        if s_0_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var z <= s_1_0
        fn_state.z = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var invert:u8
        let s_2_0: bool = fn_state.invert;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b11 b3
        if s_2_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_3_0: read-var count:i
        let s_3_0: i128 = fn_state.count;
        // D s_3_1: read-var elements:i
        let s_3_1: i128 = fn_state.elements;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b10 b4
        if s_3_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var n <= s_4_0
        fn_state.n = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_5_0: const #0s : i
        let s_5_0: i128 = 0;
        // D s_5_1: read-var count:i
        let s_5_1: i128 = fn_state.count;
        // D s_5_2: call neq_int(s_5_1, s_5_0)
        let s_5_2: bool = neq_int(state, tracer, s_5_1, s_5_0);
        // N s_5_3: branch s_5_2 b9 b6
        if s_5_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: write-var c <= s_6_0
        fn_state.c = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: read-var n:u8
        let s_8_1: bool = fn_state.n;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 1u16);
        // D s_8_3: read-var z:u8
        let s_8_3: bool = fn_state.z;
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 1u16);
        // D s_8_5: cast reint s_8_2 -> u128
        let s_8_5: u128 = (s_8_2.value() as u128);
        // D s_8_6: size-of s_8_2
        let s_8_6: u16 = s_8_2.length();
        // D s_8_7: cast reint s_8_4 -> u128
        let s_8_7: u128 = (s_8_4.value() as u128);
        // D s_8_8: size-of s_8_4
        let s_8_8: u16 = s_8_4.length();
        // D s_8_9: lsl s_8_5 s_8_8
        let s_8_9: u128 = s_8_5 << s_8_8;
        // D s_8_10: or s_8_9 s_8_7
        let s_8_10: u128 = ((s_8_9) | (s_8_7));
        // D s_8_11: add s_8_6 s_8_8
        let s_8_11: u16 = (s_8_6 + s_8_8);
        // D s_8_12: create-bits s_8_10 s_8_11
        let s_8_12: Bits = Bits::new(s_8_10, s_8_11);
        // D s_8_13: cast reint s_8_12 -> u8
        let s_8_13: u8 = (s_8_12.value() as u8);
        // D s_8_14: cast zx s_8_13 -> bv
        let s_8_14: Bits = Bits::new(s_8_13 as u128, 2u16);
        // D s_8_15: read-var c:u8
        let s_8_15: bool = fn_state.c;
        // D s_8_16: cast zx s_8_15 -> bv
        let s_8_16: Bits = Bits::new(s_8_15 as u128, 1u16);
        // D s_8_17: cast reint s_8_14 -> u128
        let s_8_17: u128 = (s_8_14.value() as u128);
        // D s_8_18: size-of s_8_14
        let s_8_18: u16 = s_8_14.length();
        // D s_8_19: cast reint s_8_16 -> u128
        let s_8_19: u128 = (s_8_16.value() as u128);
        // D s_8_20: size-of s_8_16
        let s_8_20: u16 = s_8_16.length();
        // D s_8_21: lsl s_8_17 s_8_20
        let s_8_21: u128 = s_8_17 << s_8_20;
        // D s_8_22: or s_8_21 s_8_19
        let s_8_22: u128 = ((s_8_21) | (s_8_19));
        // D s_8_23: add s_8_18 s_8_20
        let s_8_23: u16 = (s_8_18 + s_8_20);
        // D s_8_24: create-bits s_8_22 s_8_23
        let s_8_24: Bits = Bits::new(s_8_22, s_8_23);
        // D s_8_25: cast reint s_8_24 -> u8
        let s_8_25: u8 = (s_8_24.value() as u8);
        // D s_8_26: cast zx s_8_25 -> bv
        let s_8_26: Bits = Bits::new(s_8_25 as u128, 3u16);
        // C s_8_27: cast zx s_8_0 -> bv
        let s_8_27: Bits = Bits::new(s_8_0 as u128, 1u16);
        // D s_8_28: cast reint s_8_26 -> u128
        let s_8_28: u128 = (s_8_26.value() as u128);
        // D s_8_29: size-of s_8_26
        let s_8_29: u16 = s_8_26.length();
        // C s_8_30: cast reint s_8_27 -> u128
        let s_8_30: u128 = (s_8_27.value() as u128);
        // D s_8_31: size-of s_8_27
        let s_8_31: u16 = s_8_27.length();
        // D s_8_32: lsl s_8_28 s_8_31
        let s_8_32: u128 = s_8_28 << s_8_31;
        // D s_8_33: or s_8_32 s_8_30
        let s_8_33: u128 = ((s_8_32) | (s_8_30));
        // D s_8_34: add s_8_29 s_8_31
        let s_8_34: u16 = (s_8_29 + s_8_31);
        // D s_8_35: create-bits s_8_33 s_8_34
        let s_8_35: Bits = Bits::new(s_8_33, s_8_34);
        // D s_8_36: cast reint s_8_35 -> u8
        let s_8_36: u8 = (s_8_35.value() as u8);
        // N s_8_37: return s_8_36
        return s_8_36;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var c <= s_9_0
        fn_state.c = s_9_0;
        // N s_9_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var n <= s_10_0
        fn_state.n = s_10_0;
        // N s_10_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var count:i
        let s_11_1: i128 = fn_state.count;
        // D s_11_2: call neq_int(s_11_1, s_11_0)
        let s_11_2: bool = neq_int(state, tracer, s_11_1, s_11_0);
        // N s_11_3: branch s_11_2 b17 b12
        if s_11_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var n <= s_12_0
        fn_state.n = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_13_0: read-var count:i
        let s_13_0: i128 = fn_state.count;
        // D s_13_1: read-var elements:i
        let s_13_1: i128 = fn_state.elements;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b16 b14
        if s_13_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var c <= s_14_0
        fn_state.c = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_15_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var c <= s_16_0
        fn_state.c = s_16_0;
        // N s_16_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var n <= s_17_0
        fn_state.n = s_17_0;
        // N s_17_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var z <= s_18_0
        fn_state.z = s_18_0;
        // N s_18_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
