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
use V_set::*;
use V_read::*;
use Zeros::*;
use Ones::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_logical_bsl_eor<
    T: Tracer,
>(state: &mut State, tracer: &T, d: i64, datasize: i64, m: i64, n: i64, op: u32) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand4: Bits,
        operand3: Bits,
        datasizeshadow_1090: i64,
        operand1: Bits,
        operand2: Bits,
        d: i64,
        datasize: i64,
        m: i64,
        n: i64,
        op: u32,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        m,
        n,
        op,
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
        // D s_0_3: write-var datasizeshadow#1090 <= s_0_2
        fn_state.datasizeshadow_1090 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPAdvSIMDEnabled64(s_0_4)
        let s_0_5: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1090:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1090;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var operand4 <= s_1_5
        fn_state.operand4 = s_1_5;
        // C s_1_7: const #3u : u32
        let s_1_7: u32 = 3;
        // D s_1_8: read-var op:u32
        let s_1_8: u32 = fn_state.op;
        // D s_1_9: cmp-eq s_1_7 s_1_8
        let s_1_9: bool = ((s_1_7) == (s_1_8));
        // D s_1_10: not s_1_9
        let s_1_10: bool = !s_1_9;
        // N s_1_11: branch s_1_10 b4 b2
        if s_1_10 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var datasizeshadow#1090:i64
        let s_2_0: i64 = fn_state.datasizeshadow_1090;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: cast reint s_2_1 -> i64
        let s_2_2: i64 = (s_2_1 as i64);
        // D s_2_3: read-var m:i64
        let s_2_3: i64 = fn_state.m;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: call V_read(s_2_4, s_2_2)
        let s_2_5: Bits = V_read(state, tracer, s_2_4, s_2_2);
        // D s_2_6: write-var operand1 <= s_2_5
        fn_state.operand1 = s_2_5;
        // D s_2_7: read-var datasizeshadow#1090:i64
        let s_2_7: i64 = fn_state.datasizeshadow_1090;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: call Zeros(s_2_8)
        let s_2_9: Bits = Zeros(state, tracer, s_2_8);
        // D s_2_10: write-var operand2 <= s_2_9
        fn_state.operand2 = s_2_9;
        // D s_2_11: read-var datasizeshadow#1090:i64
        let s_2_11: i64 = fn_state.datasizeshadow_1090;
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_13: call Ones(s_2_12)
        let s_2_13: Bits = Ones(state, tracer, s_2_12);
        // D s_2_14: write-var operand3 <= s_2_13
        fn_state.operand3 = s_2_13;
        // N s_2_15: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var operand3:bv
        let s_3_0: Bits = fn_state.operand3;
        // D s_3_1: read-var operand2:bv
        let s_3_1: Bits = fn_state.operand2;
        // D s_3_2: read-var operand1:bv
        let s_3_2: Bits = fn_state.operand1;
        // D s_3_3: read-var datasizeshadow#1090:i64
        let s_3_3: i64 = fn_state.datasizeshadow_1090;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var operand4:bv
        let s_3_6: Bits = fn_state.operand4;
        // D s_3_7: xor s_3_1 s_3_6
        let s_3_7: Bits = ((s_3_1) ^ (s_3_6));
        // D s_3_8: and s_3_7 s_3_0
        let s_3_8: Bits = ((s_3_7) & (s_3_0));
        // D s_3_9: xor s_3_2 s_3_8
        let s_3_9: Bits = ((s_3_2) ^ (s_3_8));
        // D s_3_10: read-var d:i64
        let s_3_10: i64 = fn_state.d;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: call V_set(s_3_11, s_3_5, s_3_9)
        let s_3_12: () = V_set(state, tracer, s_3_11, s_3_5, s_3_9);
        // N s_3_13: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2u : u32
        let s_4_0: u32 = 2;
        // D s_4_1: read-var op:u32
        let s_4_1: u32 = fn_state.op;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // D s_4_3: not s_4_2
        let s_4_3: bool = !s_4_2;
        // N s_4_4: branch s_4_3 b6 b5
        if s_4_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var datasizeshadow#1090:i64
        let s_5_0: i64 = fn_state.datasizeshadow_1090;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var m:i64
        let s_5_3: i64 = fn_state.m;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: call V_read(s_5_4, s_5_2)
        let s_5_5: Bits = V_read(state, tracer, s_5_4, s_5_2);
        // D s_5_6: write-var operand1 <= s_5_5
        fn_state.operand1 = s_5_5;
        // D s_5_7: read-var operand1:bv
        let s_5_7: Bits = fn_state.operand1;
        // D s_5_8: write-var operand2 <= s_5_7
        fn_state.operand2 = s_5_7;
        // D s_5_9: read-var datasizeshadow#1090:i64
        let s_5_9: i64 = fn_state.datasizeshadow_1090;
        // D s_5_10: cast zx s_5_9 -> i
        let s_5_10: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // D s_5_12: read-var d:i64
        let s_5_12: i64 = fn_state.d;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: call V_read(s_5_13, s_5_11)
        let s_5_14: Bits = V_read(state, tracer, s_5_13, s_5_11);
        // D s_5_15: write-var operand3 <= s_5_14
        fn_state.operand3 = s_5_14;
        // N s_5_16: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: read-var op:u32
        let s_6_1: u32 = fn_state.op;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // D s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // N s_6_4: branch s_6_3 b8 b7
        if s_6_3 {
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
        // D s_7_0: read-var datasizeshadow#1090:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1090;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: call V_read(s_7_4, s_7_2)
        let s_7_5: Bits = V_read(state, tracer, s_7_4, s_7_2);
        // D s_7_6: write-var operand1 <= s_7_5
        fn_state.operand1 = s_7_5;
        // D s_7_7: read-var operand1:bv
        let s_7_7: Bits = fn_state.operand1;
        // D s_7_8: write-var operand2 <= s_7_7
        fn_state.operand2 = s_7_7;
        // D s_7_9: read-var datasizeshadow#1090:i64
        let s_7_9: i64 = fn_state.datasizeshadow_1090;
        // D s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_11: cast reint s_7_10 -> i64
        let s_7_11: i64 = (s_7_10 as i64);
        // D s_7_12: read-var m:i64
        let s_7_12: i64 = fn_state.m;
        // D s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_14: call V_read(s_7_13, s_7_11)
        let s_7_14: Bits = V_read(state, tracer, s_7_13, s_7_11);
        // D s_7_15: write-var operand3 <= s_7_14
        fn_state.operand3 = s_7_14;
        // N s_7_16: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u32
        let s_8_0: u32 = 0;
        // D s_8_1: read-var op:u32
        let s_8_1: u32 = fn_state.op;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b10 b9
        if s_8_3 {
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
        // D s_9_0: read-var datasizeshadow#1090:i64
        let s_9_0: i64 = fn_state.datasizeshadow_1090;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var d:i64
        let s_9_3: i64 = fn_state.d;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: call V_read(s_9_4, s_9_2)
        let s_9_5: Bits = V_read(state, tracer, s_9_4, s_9_2);
        // D s_9_6: write-var operand1 <= s_9_5
        fn_state.operand1 = s_9_5;
        // D s_9_7: read-var operand1:bv
        let s_9_7: Bits = fn_state.operand1;
        // D s_9_8: write-var operand2 <= s_9_7
        fn_state.operand2 = s_9_7;
        // D s_9_9: read-var datasizeshadow#1090:i64
        let s_9_9: i64 = fn_state.datasizeshadow_1090;
        // D s_9_10: cast zx s_9_9 -> i
        let s_9_10: i128 = (i128::try_from(s_9_9).unwrap());
        // D s_9_11: cast reint s_9_10 -> i64
        let s_9_11: i64 = (s_9_10 as i64);
        // D s_9_12: read-var m:i64
        let s_9_12: i64 = fn_state.m;
        // D s_9_13: cast zx s_9_12 -> i
        let s_9_13: i128 = (i128::try_from(s_9_12).unwrap());
        // D s_9_14: call V_read(s_9_13, s_9_11)
        let s_9_14: Bits = V_read(state, tracer, s_9_13, s_9_11);
        // D s_9_15: not s_9_14
        let s_9_15: Bits = !s_9_14;
        // D s_9_16: write-var operand3 <= s_9_15
        fn_state.operand3 = s_9_15;
        // N s_9_17: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b3
        return block_3(state, tracer, fn_state);
    }
}
