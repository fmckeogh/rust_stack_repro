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
use SPESampleAddOpOther::*;
use ConditionHolds::*;
use common::*;
pub fn execute_aarch64_instrs_integer_conditional_select<T: Tracer>(
    state: &mut State,
    tracer: &T,
    condition: u8,
    d: i64,
    datasize: i64,
    else_inc: bool,
    else_inv: bool,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasizeshadow_1177: i64,
        result: Bits,
        taken: bool,
        condition: u8,
        d: i64,
        datasize: i64,
        else_inc: bool,
        else_inv: bool,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        condition,
        d,
        datasize,
        else_inc,
        else_inv,
        m,
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
        // D s_0_3: write-var datasizeshadow#1177 <= s_0_2
        fn_state.datasizeshadow_1177 = s_0_2;
        // C s_0_4: const #0u : u8
        let s_0_4: bool = false;
        // D s_0_5: write-var taken <= s_0_4
        fn_state.taken = s_0_4;
        // D s_0_6: read-var condition:u8
        let s_0_6: u8 = fn_state.condition;
        // D s_0_7: call ConditionHolds(s_0_6)
        let s_0_7: bool = ConditionHolds(state, tracer, s_0_6);
        // N s_0_8: branch s_0_7 b11 b1
        if s_0_7 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1177:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1177;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var m:i64
        let s_1_3: i64 = fn_state.m;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call X_read(s_1_4, s_1_2)
        let s_1_5: Bits = X_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var result <= s_1_5
        fn_state.result = s_1_5;
        // D s_1_7: read-var else_inv:u8
        let s_1_7: bool = fn_state.else_inv;
        // N s_1_8: branch s_1_7 b10 b2
        if s_1_7 {
            return block_10(state, tracer, fn_state);
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
        // D s_3_0: read-var else_inc:u8
        let s_3_0: bool = fn_state.else_inc;
        // N s_3_1: branch s_3_0 b9 b4
        if s_3_0 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
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
        // D s_6_0: read-var datasizeshadow#1177:i64
        let s_6_0: i64 = fn_state.datasizeshadow_1177;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var d:i64
        let s_6_3: i64 = fn_state.d;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: read-var result:bv
        let s_6_5: Bits = fn_state.result;
        // D s_6_6: call X_set(s_6_4, s_6_2, s_6_5)
        let s_6_6: () = X_set(state, tracer, s_6_4, s_6_2, s_6_5);
        // C s_6_7: const #22416u : u32
        let s_6_7: u32 = 22416;
        // D s_6_8: read-reg s_6_7:u8
        let s_6_8: bool = {
            let value = state.read_register::<bool>(s_6_7 as isize);
            tracer.read_register(s_6_7 as isize, value);
            value
        };
        // N s_6_9: branch s_6_8 b8 b7
        if s_6_8 {
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
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: read-var taken:u8
        let s_8_1: bool = fn_state.taken;
        // D s_8_2: call SPESampleAddOpOther(s_8_0, s_8_1)
        let s_8_2: () = SPESampleAddOpOther(state, tracer, s_8_0, s_8_1);
        // N s_8_3: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1s : i
        let s_9_0: i128 = 1;
        // D s_9_1: read-var result:bv
        let s_9_1: Bits = fn_state.result;
        // C s_9_2: cast cvt s_9_0 -> bv
        let s_9_2: Bits = Bits::new(s_9_0 as u128, 128);
        // D s_9_3: add s_9_1 s_9_2
        let s_9_3: Bits = (s_9_1 + s_9_2);
        // D s_9_4: write-var result <= s_9_3
        fn_state.result = s_9_3;
        // N s_9_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var result:bv
        let s_10_0: Bits = fn_state.result;
        // D s_10_1: not s_10_0
        let s_10_1: Bits = !s_10_0;
        // D s_10_2: write-var result <= s_10_1
        fn_state.result = s_10_1;
        // N s_10_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var datasizeshadow#1177:i64
        let s_11_0: i64 = fn_state.datasizeshadow_1177;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // D s_11_3: read-var n:i64
        let s_11_3: i64 = fn_state.n;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: call X_read(s_11_4, s_11_2)
        let s_11_5: Bits = X_read(state, tracer, s_11_4, s_11_2);
        // D s_11_6: write-var result <= s_11_5
        fn_state.result = s_11_5;
        // C s_11_7: const #1u : u8
        let s_11_7: bool = true;
        // D s_11_8: write-var taken <= s_11_7
        fn_state.taken = s_11_7;
        // N s_11_9: jump b6
        return block_6(state, tracer, fn_state);
    }
}
