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
use X_read::*;
use u__id::*;
use common::*;
pub fn execute_aarch64_instrs_integer_arithmetic_rev<T: Tracer>(
    state: &mut State,
    tracer: &T,
    container_size: i64,
    d: i64,
    datasize: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        gs_166576: bool,
        e: i64,
        gs_166557: i64,
        rev_indexshadow_1787: i128,
        index: i128,
        gs_166573: bool,
        indexshadow_1788: i128,
        datasizeshadow_1786: i64,
        result: Bits,
        rev_index: i128,
        c: i64,
        gs_166566: i64,
        elements_per_container: i64,
        container_size: i64,
        d: i64,
        datasize: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        container_size,
        d,
        datasize,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#1786 <= s_0_2
        fn_state.datasizeshadow_1786 = s_0_2;
        // D s_0_4: read-var datasizeshadow#1786:i64
        let s_0_4: i64 = fn_state.datasizeshadow_1786;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: read-var n:i64
        let s_0_7: i64 = fn_state.n;
        // D s_0_8: cast zx s_0_7 -> i
        let s_0_8: i128 = (i128::try_from(s_0_7).unwrap());
        // D s_0_9: call X_read(s_0_8, s_0_6)
        let s_0_9: Bits = X_read(state, tracer, s_0_8, s_0_6);
        // D s_0_10: write-var operand <= s_0_9
        fn_state.operand = s_0_9;
        // D s_0_11: read-var datasizeshadow#1786:i64
        let s_0_11: i64 = fn_state.datasizeshadow_1786;
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_13: read-var container_size:i64
        let s_0_13: i64 = fn_state.container_size;
        // D s_0_14: cast zx s_0_13 -> i
        let s_0_14: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_15: div s_0_12 s_0_14
        let s_0_15: i128 = ((s_0_12) / (s_0_14));
        // D s_0_16: cast reint s_0_15 -> i64
        let s_0_16: i64 = (s_0_15 as i64);
        // C s_0_17: const #8s : i
        let s_0_17: i128 = 8;
        // D s_0_18: read-var container_size:i64
        let s_0_18: i64 = fn_state.container_size;
        // D s_0_19: cast zx s_0_18 -> i
        let s_0_19: i128 = (i128::try_from(s_0_18).unwrap());
        // D s_0_20: div s_0_19 s_0_17
        let s_0_20: i128 = ((s_0_19) / (s_0_17));
        // D s_0_21: cast reint s_0_20 -> i64
        let s_0_21: i64 = (s_0_20 as i64);
        // D s_0_22: write-var elements_per_container <= s_0_21
        fn_state.elements_per_container = s_0_21;
        // C s_0_23: const #0s : i
        let s_0_23: i128 = 0;
        // D s_0_24: write-var index <= s_0_23
        fn_state.index = s_0_23;
        // C s_0_25: const #0s : i64
        let s_0_25: i64 = 0;
        // C s_0_26: const #1s : i
        let s_0_26: i128 = 1;
        // D s_0_27: cast zx s_0_16 -> i
        let s_0_27: i128 = (i128::try_from(s_0_16).unwrap());
        // D s_0_28: sub s_0_27 s_0_26
        let s_0_28: i128 = ((s_0_27) - (s_0_26));
        // D s_0_29: cast reint s_0_28 -> i64
        let s_0_29: i64 = (s_0_28 as i64);
        // D s_0_30: write-var gs#166557 <= s_0_29
        fn_state.gs_166557 = s_0_29;
        // D s_0_31: write-var c <= s_0_25
        fn_state.c = s_0_25;
        // N s_0_32: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var c:i64
        let s_1_0: i64 = fn_state.c;
        // D s_1_1: read-var gs#166557:i64
        let s_1_1: i64 = fn_state.gs_166557;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b12 b2
        if s_1_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #1s : i
        let s_2_0: i128 = 1;
        // D s_2_1: read-var elements_per_container:i64
        let s_2_1: i64 = fn_state.elements_per_container;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: sub s_2_2 s_2_0
        let s_2_3: i128 = ((s_2_2) - (s_2_0));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // C s_2_5: const #8s : i
        let s_2_5: i128 = 8;
        // D s_2_6: cast zx s_2_4 -> i
        let s_2_6: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_7: mul s_2_6 s_2_5
        let s_2_7: i128 = ((s_2_6) * (s_2_5));
        // D s_2_8: cast reint s_2_7 -> i64
        let s_2_8: i64 = (s_2_7 as i64);
        // D s_2_9: cast zx s_2_8 -> i
        let s_2_9: i128 = (i128::try_from(s_2_8).unwrap());
        // D s_2_10: read-var index:i
        let s_2_10: i128 = fn_state.index;
        // D s_2_11: add s_2_10 s_2_9
        let s_2_11: i128 = (s_2_10 + s_2_9);
        // D s_2_12: write-var rev_index <= s_2_11
        fn_state.rev_index = s_2_11;
        // C s_2_13: const #0s : i64
        let s_2_13: i64 = 0;
        // C s_2_14: const #1s : i
        let s_2_14: i128 = 1;
        // D s_2_15: read-var elements_per_container:i64
        let s_2_15: i64 = fn_state.elements_per_container;
        // D s_2_16: cast zx s_2_15 -> i
        let s_2_16: i128 = (i128::try_from(s_2_15).unwrap());
        // D s_2_17: sub s_2_16 s_2_14
        let s_2_17: i128 = ((s_2_16) - (s_2_14));
        // D s_2_18: cast reint s_2_17 -> i64
        let s_2_18: i64 = (s_2_17 as i64);
        // D s_2_19: write-var gs#166566 <= s_2_18
        fn_state.gs_166566 = s_2_18;
        // D s_2_20: write-var e <= s_2_13
        fn_state.e = s_2_13;
        // N s_2_21: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: read-var gs#166566:i64
        let s_3_1: i64 = fn_state.gs_166566;
        // D s_3_2: cmp-gt s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) > (s_3_1));
        // N s_3_3: branch s_3_2 b11 b4
        if s_3_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var rev_index:i
        let s_4_0: i128 = fn_state.rev_index;
        // D s_4_1: write-var rev_indexshadow#1787 <= s_4_0
        fn_state.rev_indexshadow_1787 = s_4_0;
        // D s_4_2: read-var index:i
        let s_4_2: i128 = fn_state.index;
        // D s_4_3: write-var indexshadow#1788 <= s_4_2
        fn_state.indexshadow_1788 = s_4_2;
        // D s_4_4: read-var rev_indexshadow#1787:i
        let s_4_4: i128 = fn_state.rev_indexshadow_1787;
        // D s_4_5: call __id(s_4_4)
        let s_4_5: i128 = u__id(state, tracer, s_4_4);
        // C s_4_6: const #0s : i
        let s_4_6: i128 = 0;
        // D s_4_7: cmp-le s_4_6 s_4_5
        let s_4_7: bool = ((s_4_6) <= (s_4_5));
        // N s_4_8: branch s_4_7 b10 b5
        if s_4_7 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#166573 <= s_5_0
        fn_state.gs_166573 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#166573:u8
        let s_6_0: bool = fn_state.gs_166573;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // D s_6_2: read-var indexshadow#1788:i
        let s_6_2: i128 = fn_state.indexshadow_1788;
        // D s_6_3: call __id(s_6_2)
        let s_6_3: i128 = u__id(state, tracer, s_6_2);
        // C s_6_4: const #0s : i
        let s_6_4: i128 = 0;
        // D s_6_5: cmp-le s_6_4 s_6_3
        let s_6_5: bool = ((s_6_4) <= (s_6_3));
        // N s_6_6: branch s_6_5 b9 b7
        if s_6_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#166576 <= s_7_0
        fn_state.gs_166576 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#166576:u8
        let s_8_0: bool = fn_state.gs_166576;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // C s_8_2: const #7s : i
        let s_8_2: i128 = 7;
        // D s_8_3: read-var rev_indexshadow#1787:i
        let s_8_3: i128 = fn_state.rev_indexshadow_1787;
        // D s_8_4: add s_8_3 s_8_2
        let s_8_4: i128 = (s_8_3 + s_8_2);
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // C s_8_6: const #8s : i
        let s_8_6: i128 = 8;
        // D s_8_7: read-var operand:bv
        let s_8_7: Bits = fn_state.operand;
        // D s_8_8: read-var indexshadow#1788:i
        let s_8_8: i128 = fn_state.indexshadow_1788;
        // D s_8_9: bit-extract s_8_7 s_8_8 s_8_6
        let s_8_9: Bits = (Bits::new(
            ((s_8_7) >> (s_8_8)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_10: cast reint s_8_9 -> u8
        let s_8_10: u8 = (s_8_9.value() as u8);
        // D s_8_11: cast zx s_8_5 -> i
        let s_8_11: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_12: cast zx s_8_10 -> bv
        let s_8_12: Bits = Bits::new(s_8_10 as u128, 8u16);
        // D s_8_13: read-var result:bv
        let s_8_13: Bits = fn_state.result;
        // D s_8_14: read-var rev_indexshadow#1787:i
        let s_8_14: i128 = fn_state.rev_indexshadow_1787;
        // D s_8_15: sub s_8_11 s_8_14
        let s_8_15: i128 = ((s_8_11) - (s_8_14));
        // C s_8_16: const #1u : u64
        let s_8_16: u64 = 1;
        // C s_8_17: cast zx s_8_16 -> bv
        let s_8_17: Bits = Bits::new(s_8_16 as u128, 64u16);
        // D s_8_18: lsl s_8_17 s_8_15
        let s_8_18: Bits = s_8_17 << s_8_15;
        // D s_8_19: sub s_8_18 s_8_17
        let s_8_19: Bits = ((s_8_18) - (s_8_17));
        // D s_8_20: and s_8_12 s_8_19
        let s_8_20: Bits = ((s_8_12) & (s_8_19));
        // D s_8_21: lsl s_8_20 s_8_14
        let s_8_21: Bits = s_8_20 << s_8_14;
        // D s_8_22: lsl s_8_19 s_8_14
        let s_8_22: Bits = s_8_19 << s_8_14;
        // D s_8_23: cmpl s_8_22
        let s_8_23: Bits = !s_8_22;
        // D s_8_24: and s_8_13 s_8_23
        let s_8_24: Bits = ((s_8_13) & (s_8_23));
        // D s_8_25: or s_8_24 s_8_21
        let s_8_25: Bits = ((s_8_24) | (s_8_21));
        // D s_8_26: write-var result <= s_8_25
        fn_state.result = s_8_25;
        // C s_8_27: const #8s : i
        let s_8_27: i128 = 8;
        // D s_8_28: read-var index:i
        let s_8_28: i128 = fn_state.index;
        // D s_8_29: add s_8_28 s_8_27
        let s_8_29: i128 = (s_8_28 + s_8_27);
        // D s_8_30: write-var index <= s_8_29
        fn_state.index = s_8_29;
        // C s_8_31: const #8s : i
        let s_8_31: i128 = 8;
        // D s_8_32: read-var rev_index:i
        let s_8_32: i128 = fn_state.rev_index;
        // D s_8_33: sub s_8_32 s_8_31
        let s_8_33: i128 = ((s_8_32) - (s_8_31));
        // D s_8_34: write-var rev_index <= s_8_33
        fn_state.rev_index = s_8_33;
        // D s_8_35: read-var e:i64
        let s_8_35: i64 = fn_state.e;
        // C s_8_36: const #1s : i64
        let s_8_36: i64 = 1;
        // D s_8_37: add s_8_35 s_8_36
        let s_8_37: i64 = (s_8_35 + s_8_36);
        // D s_8_38: write-var e <= s_8_37
        fn_state.e = s_8_37;
        // N s_8_39: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var indexshadow#1788:i
        let s_9_0: i128 = fn_state.indexshadow_1788;
        // D s_9_1: call __id(s_9_0)
        let s_9_1: i128 = u__id(state, tracer, s_9_0);
        // C s_9_2: const #7s : i
        let s_9_2: i128 = 7;
        // D s_9_3: add s_9_1 s_9_2
        let s_9_3: i128 = (s_9_1 + s_9_2);
        // D s_9_4: read-var datasizeshadow#1786:i64
        let s_9_4: i64 = fn_state.datasizeshadow_1786;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: call __id(s_9_5)
        let s_9_6: i128 = u__id(state, tracer, s_9_5);
        // D s_9_7: cast reint s_9_6 -> i64
        let s_9_7: i64 = (s_9_6 as i64);
        // D s_9_8: cast zx s_9_7 -> i
        let s_9_8: i128 = (i128::try_from(s_9_7).unwrap());
        // D s_9_9: cmp-lt s_9_3 s_9_8
        let s_9_9: bool = ((s_9_3) < (s_9_8));
        // D s_9_10: write-var gs#166576 <= s_9_9
        fn_state.gs_166576 = s_9_9;
        // N s_9_11: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var rev_indexshadow#1787:i
        let s_10_0: i128 = fn_state.rev_indexshadow_1787;
        // D s_10_1: call __id(s_10_0)
        let s_10_1: i128 = u__id(state, tracer, s_10_0);
        // C s_10_2: const #7s : i
        let s_10_2: i128 = 7;
        // D s_10_3: add s_10_1 s_10_2
        let s_10_3: i128 = (s_10_1 + s_10_2);
        // D s_10_4: read-var datasizeshadow#1786:i64
        let s_10_4: i64 = fn_state.datasizeshadow_1786;
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: call __id(s_10_5)
        let s_10_6: i128 = u__id(state, tracer, s_10_5);
        // D s_10_7: cast reint s_10_6 -> i64
        let s_10_7: i64 = (s_10_6 as i64);
        // D s_10_8: cast zx s_10_7 -> i
        let s_10_8: i128 = (i128::try_from(s_10_7).unwrap());
        // D s_10_9: cmp-lt s_10_3 s_10_8
        let s_10_9: bool = ((s_10_3) < (s_10_8));
        // D s_10_10: write-var gs#166573 <= s_10_9
        fn_state.gs_166573 = s_10_9;
        // N s_10_11: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var c:i64
        let s_11_0: i64 = fn_state.c;
        // C s_11_1: const #1s : i64
        let s_11_1: i64 = 1;
        // D s_11_2: add s_11_0 s_11_1
        let s_11_2: i64 = (s_11_0 + s_11_1);
        // D s_11_3: write-var c <= s_11_2
        fn_state.c = s_11_2;
        // N s_11_4: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var datasizeshadow#1786:i64
        let s_12_0: i64 = fn_state.datasizeshadow_1786;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: read-var d:i64
        let s_12_3: i64 = fn_state.d;
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: read-var result:bv
        let s_12_5: Bits = fn_state.result;
        // D s_12_6: call X_set(s_12_4, s_12_2, s_12_5)
        let s_12_6: () = X_set(state, tracer, s_12_4, s_12_2, s_12_5);
        // N s_12_7: return
        return;
    }
}
