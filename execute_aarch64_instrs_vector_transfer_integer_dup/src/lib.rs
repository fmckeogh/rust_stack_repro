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
use u__id::*;
use Elem_set::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_transfer_integer_dup<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_148029: bool,
        e: i64,
        esizeshadow_1185: i64,
        gs_148031: bool,
        datasizeshadow_1186: i64,
        gs_148037: i64,
        element: Bits,
        result: Bits,
        gs_148027: bool,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#1185 <= s_0_2
        fn_state.esizeshadow_1185 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1186 <= s_0_6
        fn_state.datasizeshadow_1186 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPAdvSIMDEnabled64(s_0_8)
        let s_0_9: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var esizeshadow#1185:i64
        let s_1_0: i64 = fn_state.esizeshadow_1185;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call __id(s_1_1)
        let s_1_2: i128 = u__id(state, tracer, s_1_1);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #8s : i
        let s_1_4: i128 = 8;
        // D s_1_5: cast zx s_1_3 -> i
        let s_1_5: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_6: cmp-eq s_1_5 s_1_4
        let s_1_6: bool = ((s_1_5) == (s_1_4));
        // N s_1_7: branch s_1_6 b13 b2
        if s_1_6 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var esizeshadow#1185:i64
        let s_2_0: i64 = fn_state.esizeshadow_1185;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call __id(s_2_1)
        let s_2_2: i128 = u__id(state, tracer, s_2_1);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #16s : i
        let s_2_4: i128 = 16;
        // D s_2_5: cast zx s_2_3 -> i
        let s_2_5: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_6: cmp-eq s_2_5 s_2_4
        let s_2_6: bool = ((s_2_5) == (s_2_4));
        // D s_2_7: write-var gs#148027 <= s_2_6
        fn_state.gs_148027 = s_2_6;
        // N s_2_8: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#148027:u8
        let s_3_0: bool = fn_state.gs_148027;
        // N s_3_1: branch s_3_0 b12 b4
        if s_3_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esizeshadow#1185:i64
        let s_4_0: i64 = fn_state.esizeshadow_1185;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call __id(s_4_1)
        let s_4_2: i128 = u__id(state, tracer, s_4_1);
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #32s : i
        let s_4_4: i128 = 32;
        // D s_4_5: cast zx s_4_3 -> i
        let s_4_5: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_6: cmp-eq s_4_5 s_4_4
        let s_4_6: bool = ((s_4_5) == (s_4_4));
        // D s_4_7: write-var gs#148029 <= s_4_6
        fn_state.gs_148029 = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#148029:u8
        let s_5_0: bool = fn_state.gs_148029;
        // N s_5_1: branch s_5_0 b11 b6
        if s_5_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var esizeshadow#1185:i64
        let s_6_0: i64 = fn_state.esizeshadow_1185;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call __id(s_6_1)
        let s_6_2: i128 = u__id(state, tracer, s_6_1);
        // D s_6_3: cast reint s_6_2 -> i64
        let s_6_3: i64 = (s_6_2 as i64);
        // C s_6_4: const #64s : i
        let s_6_4: i128 = 64;
        // D s_6_5: cast zx s_6_3 -> i
        let s_6_5: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_6: cmp-eq s_6_5 s_6_4
        let s_6_6: bool = ((s_6_5) == (s_6_4));
        // D s_6_7: write-var gs#148031 <= s_6_6
        fn_state.gs_148031 = s_6_6;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#148031:u8
        let s_7_0: bool = fn_state.gs_148031;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // D s_7_2: read-var esizeshadow#1185:i64
        let s_7_2: i64 = fn_state.esizeshadow_1185;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: read-var n:i64
        let s_7_5: i64 = fn_state.n;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: call X_read(s_7_6, s_7_4)
        let s_7_7: Bits = X_read(state, tracer, s_7_6, s_7_4);
        // D s_7_8: write-var element <= s_7_7
        fn_state.element = s_7_7;
        // C s_7_9: const #0s : i64
        let s_7_9: i64 = 0;
        // C s_7_10: const #1s : i
        let s_7_10: i128 = 1;
        // D s_7_11: read-var elements:i
        let s_7_11: i128 = fn_state.elements;
        // D s_7_12: sub s_7_11 s_7_10
        let s_7_12: i128 = ((s_7_11) - (s_7_10));
        // D s_7_13: cast reint s_7_12 -> i64
        let s_7_13: i64 = (s_7_12 as i64);
        // D s_7_14: write-var gs#148037 <= s_7_13
        fn_state.gs_148037 = s_7_13;
        // D s_7_15: write-var e <= s_7_9
        fn_state.e = s_7_9;
        // N s_7_16: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var e:i64
        let s_8_0: i64 = fn_state.e;
        // D s_8_1: read-var gs#148037:i64
        let s_8_1: i64 = fn_state.gs_148037;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b10 b9
        if s_8_2 {
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
        // D s_9_0: read-var esizeshadow#1185:i64
        let s_9_0: i64 = fn_state.esizeshadow_1185;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var e:i64
        let s_9_3: i64 = fn_state.e;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: cast zx s_9_2 -> i
        let s_9_5: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_6: read-var result:bv
        let s_9_6: Bits = fn_state.result;
        // D s_9_7: read-var element:bv
        let s_9_7: Bits = fn_state.element;
        // D s_9_8: call Elem_set(s_9_6, s_9_4, s_9_5, s_9_7)
        let s_9_8: Bits = Elem_set(state, tracer, s_9_6, s_9_4, s_9_5, s_9_7);
        // D s_9_9: write-var result <= s_9_8
        fn_state.result = s_9_8;
        // D s_9_10: read-var e:i64
        let s_9_10: i64 = fn_state.e;
        // C s_9_11: const #1s : i64
        let s_9_11: i64 = 1;
        // D s_9_12: add s_9_10 s_9_11
        let s_9_12: i64 = (s_9_10 + s_9_11);
        // D s_9_13: write-var e <= s_9_12
        fn_state.e = s_9_12;
        // N s_9_14: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var datasizeshadow#1186:i64
        let s_10_0: i64 = fn_state.datasizeshadow_1186;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var d:i64
        let s_10_3: i64 = fn_state.d;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: read-var result:bv
        let s_10_5: Bits = fn_state.result;
        // D s_10_6: call V_set(s_10_4, s_10_2, s_10_5)
        let s_10_6: () = V_set(state, tracer, s_10_4, s_10_2, s_10_5);
        // N s_10_7: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#148031 <= s_11_0
        fn_state.gs_148031 = s_11_0;
        // N s_11_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#148029 <= s_12_0
        fn_state.gs_148029 = s_12_0;
        // N s_12_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#148027 <= s_13_0
        fn_state.gs_148027 = s_13_0;
        // N s_13_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
