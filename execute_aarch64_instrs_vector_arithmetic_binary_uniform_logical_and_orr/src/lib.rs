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
use V_read::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_binary_uniform_logical_and_orr<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    invert: bool,
    m: i64,
    n: i64,
    op: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand1: Bits,
        datasizeshadow_1048: i64,
        result: Bits,
        operand2: Bits,
        d: i64,
        datasize: i64,
        invert: bool,
        m: i64,
        n: i64,
        op: u32,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        invert,
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
        // D s_0_3: write-var datasizeshadow#1048 <= s_0_2
        fn_state.datasizeshadow_1048 = s_0_2;
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
        // D s_1_0: read-var datasizeshadow#1048:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1048;
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
        // D s_1_6: write-var operand1 <= s_1_5
        fn_state.operand1 = s_1_5;
        // D s_1_7: read-var datasizeshadow#1048:i64
        let s_1_7: i64 = fn_state.datasizeshadow_1048;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var m:i64
        let s_1_10: i64 = fn_state.m;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: call V_read(s_1_11, s_1_9)
        let s_1_12: Bits = V_read(state, tracer, s_1_11, s_1_9);
        // D s_1_13: write-var operand2 <= s_1_12
        fn_state.operand2 = s_1_12;
        // D s_1_14: read-var invert:u8
        let s_1_14: bool = fn_state.invert;
        // N s_1_15: branch s_1_14 b9 b2
        if s_1_14 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u32
        let s_3_0: u32 = 0;
        // D s_3_1: read-var op:u32
        let s_3_1: u32 = fn_state.op;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b6 b4
        if s_3_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var operand1:bv
        let s_4_0: Bits = fn_state.operand1;
        // D s_4_1: read-var operand2:bv
        let s_4_1: Bits = fn_state.operand2;
        // D s_4_2: and s_4_0 s_4_1
        let s_4_2: Bits = ((s_4_0) & (s_4_1));
        // D s_4_3: write-var result <= s_4_2
        fn_state.result = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var datasizeshadow#1048:i64
        let s_5_0: i64 = fn_state.datasizeshadow_1048;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var d:i64
        let s_5_3: i64 = fn_state.d;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: read-var result:bv
        let s_5_5: Bits = fn_state.result;
        // D s_5_6: call V_set(s_5_4, s_5_2, s_5_5)
        let s_5_6: () = V_set(state, tracer, s_5_4, s_5_2, s_5_5);
        // N s_5_7: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2u : u32
        let s_6_0: u32 = 2;
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
        // D s_7_0: read-var operand1:bv
        let s_7_0: Bits = fn_state.operand1;
        // D s_7_1: read-var operand2:bv
        let s_7_1: Bits = fn_state.operand2;
        // D s_7_2: or s_7_0 s_7_1
        let s_7_2: Bits = ((s_7_0) | (s_7_1));
        // D s_7_3: write-var result <= s_7_2
        fn_state.result = s_7_2;
        // N s_7_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var operand2:bv
        let s_9_0: Bits = fn_state.operand2;
        // D s_9_1: not s_9_0
        let s_9_1: Bits = !s_9_0;
        // D s_9_2: write-var operand2 <= s_9_1
        fn_state.operand2 = s_9_1;
        // N s_9_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
