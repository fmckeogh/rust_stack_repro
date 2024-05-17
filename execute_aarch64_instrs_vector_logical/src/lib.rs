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
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_logical<T: Tracer>(
    state: &mut State,
    tracer: &T,
    datasize: i64,
    imm: Bits,
    operation: u32,
    rd: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasizeshadow_1083: i64,
        result: Bits,
        datasize: i64,
        imm: Bits,
        operation: u32,
        rd: i64,
    }
    let fn_state = FunctionState {
        datasize,
        imm,
        operation,
        rd,
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
        // D s_0_3: write-var datasizeshadow#1083 <= s_0_2
        fn_state.datasizeshadow_1083 = s_0_2;
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
        // C s_1_0: const #0u : u32
        let s_1_0: u32 = 0;
        // D s_1_1: read-var operation:u32
        let s_1_1: u32 = fn_state.operation;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // D s_1_3: not s_1_2
        let s_1_3: bool = !s_1_2;
        // N s_1_4: branch s_1_3 b4 b2
        if s_1_3 {
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
        // D s_2_0: read-var imm:bv
        let s_2_0: Bits = fn_state.imm;
        // D s_2_1: write-var result <= s_2_0
        fn_state.result = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var datasizeshadow#1083:i64
        let s_3_0: i64 = fn_state.datasizeshadow_1083;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var rd:i64
        let s_3_3: i64 = fn_state.rd;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-var result:bv
        let s_3_5: Bits = fn_state.result;
        // D s_3_6: call V_set(s_3_4, s_3_2, s_3_5)
        let s_3_6: () = V_set(state, tracer, s_3_4, s_3_2, s_3_5);
        // N s_3_7: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u32
        let s_4_0: u32 = 1;
        // D s_4_1: read-var operation:u32
        let s_4_1: u32 = fn_state.operation;
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
        // D s_5_0: read-var imm:bv
        let s_5_0: Bits = fn_state.imm;
        // D s_5_1: not s_5_0
        let s_5_1: Bits = !s_5_0;
        // D s_5_2: write-var result <= s_5_1
        fn_state.result = s_5_1;
        // N s_5_3: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2u : u32
        let s_6_0: u32 = 2;
        // D s_6_1: read-var operation:u32
        let s_6_1: u32 = fn_state.operation;
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
        // D s_7_0: read-var datasizeshadow#1083:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1083;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var rd:i64
        let s_7_3: i64 = fn_state.rd;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: call V_read(s_7_4, s_7_2)
        let s_7_5: Bits = V_read(state, tracer, s_7_4, s_7_2);
        // D s_7_6: read-var imm:bv
        let s_7_6: Bits = fn_state.imm;
        // D s_7_7: or s_7_5 s_7_6
        let s_7_7: Bits = ((s_7_5) | (s_7_6));
        // D s_7_8: write-var result <= s_7_7
        fn_state.result = s_7_7;
        // N s_7_9: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #3u : u32
        let s_8_0: u32 = 3;
        // D s_8_1: read-var operation:u32
        let s_8_1: u32 = fn_state.operation;
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
        // D s_9_0: read-var datasizeshadow#1083:i64
        let s_9_0: i64 = fn_state.datasizeshadow_1083;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var rd:i64
        let s_9_3: i64 = fn_state.rd;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: call V_read(s_9_4, s_9_2)
        let s_9_5: Bits = V_read(state, tracer, s_9_4, s_9_2);
        // D s_9_6: read-var imm:bv
        let s_9_6: Bits = fn_state.imm;
        // D s_9_7: not s_9_6
        let s_9_7: Bits = !s_9_6;
        // D s_9_8: and s_9_5 s_9_7
        let s_9_8: Bits = ((s_9_5) & (s_9_7));
        // D s_9_9: write-var result <= s_9_8
        fn_state.result = s_9_8;
        // N s_9_10: jump b3
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
