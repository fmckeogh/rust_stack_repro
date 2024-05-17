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
use execute_aarch64_instrs_integer_arithmetic_add_sub_immediate::*;
use place_slice::*;
use common::*;
pub fn decode_sub_addsub_imm_aarch64_instrs_integer_arithmetic_add_sub_immediate<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    imm12: u16,
    sh: bool,
    S: bool,
    op: bool,
    sf: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_249159: i64,
        setflags: bool,
        imm: Bits,
        datasize: i64,
        n: i64,
        d: i64,
        sub_op: bool,
        Rd: u8,
        Rn: u8,
        imm12: u16,
        sh: bool,
        S: bool,
        op: bool,
        sf: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        imm12,
        sh,
        S,
        op,
        sf,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var d <= s_0_3
        fn_state.d = s_0_3;
        // D s_0_5: read-var Rn:u8
        let s_0_5: u8 = fn_state.Rn;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var n <= s_0_8
        fn_state.n = s_0_8;
        // D s_0_10: read-var sf:u8
        let s_0_10: bool = fn_state.sf;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // C s_0_12: const #1u : u8
        let s_0_12: bool = true;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b6 b1
        if s_0_14 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: write-var ga#249159 <= s_1_0
        fn_state.ga_249159 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#249159:i64
        let s_2_0: i64 = fn_state.ga_249159;
        // D s_2_1: write-var datasize <= s_2_0
        fn_state.datasize = s_2_0;
        // D s_2_2: read-var op:u8
        let s_2_2: bool = fn_state.op;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #1u : u8
        let s_2_4: bool = true;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // D s_2_7: write-var sub_op <= s_2_6
        fn_state.sub_op = s_2_6;
        // D s_2_8: read-var S:u8
        let s_2_8: bool = fn_state.S;
        // D s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // C s_2_10: const #1u : u8
        let s_2_10: bool = true;
        // C s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 1u16);
        // D s_2_12: cmp-eq s_2_9 s_2_11
        let s_2_12: bool = ((s_2_9) == (s_2_11));
        // D s_2_13: write-var setflags <= s_2_12
        fn_state.setflags = s_2_12;
        // D s_2_14: read-var sh:u8
        let s_2_14: bool = fn_state.sh;
        // D s_2_15: cast zx s_2_14 -> bv
        let s_2_15: Bits = Bits::new(s_2_14 as u128, 1u16);
        // C s_2_16: const #0u : u8
        let s_2_16: bool = false;
        // C s_2_17: cast zx s_2_16 -> bv
        let s_2_17: Bits = Bits::new(s_2_16 as u128, 1u16);
        // D s_2_18: cmp-eq s_2_15 s_2_17
        let s_2_18: bool = ((s_2_15) == (s_2_17));
        // D s_2_19: not s_2_18
        let s_2_19: bool = !s_2_18;
        // N s_2_20: branch s_2_19 b5 b3
        if s_2_19 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var imm12:u12
        let s_3_0: u16 = fn_state.imm12;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 12u16);
        // D s_3_2: read-var datasize:i64
        let s_3_2: i64 = fn_state.datasize;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: bits-cast zx s_3_1 -> bv length s_3_3
        let s_3_4: Bits = s_3_1.zero_extend(s_3_3);
        // D s_3_5: write-var imm <= s_3_4
        fn_state.imm = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var imm:bv
        let s_4_0: Bits = fn_state.imm;
        // D s_4_1: read-var datasize:i64
        let s_4_1: i64 = fn_state.datasize;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // D s_4_4: read-var d:i64
        let s_4_4: i64 = fn_state.d;
        // D s_4_5: read-var n:i64
        let s_4_5: i64 = fn_state.n;
        // D s_4_6: read-var setflags:u8
        let s_4_6: bool = fn_state.setflags;
        // D s_4_7: read-var sub_op:u8
        let s_4_7: bool = fn_state.sub_op;
        // D s_4_8: call execute_aarch64_instrs_integer_arithmetic_add_sub_immediate(s_4_4, s_4_3, s_4_0, s_4_5, s_4_6, s_4_7)
        let s_4_8: () = execute_aarch64_instrs_integer_arithmetic_add_sub_immediate(
            state,
            tracer,
            s_4_4,
            s_4_3,
            s_4_0,
            s_4_5,
            s_4_6,
            s_4_7,
        );
        // N s_4_9: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i
        let s_5_0: i128 = 0;
        // C s_5_1: const #12s : i
        let s_5_1: i128 = 12;
        // C s_5_2: const #12s : i
        let s_5_2: i128 = 12;
        // D s_5_3: read-var datasize:i64
        let s_5_3: i64 = fn_state.datasize;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: read-var imm12:u12
        let s_5_5: u16 = fn_state.imm12;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 12u16);
        // D s_5_7: call place_slice(s_5_4, s_5_6, s_5_0, s_5_1, s_5_2)
        let s_5_7: Bits = place_slice(state, tracer, s_5_4, s_5_6, s_5_0, s_5_1, s_5_2);
        // D s_5_8: write-var imm <= s_5_7
        fn_state.imm = s_5_7;
        // N s_5_9: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // D s_6_1: write-var ga#249159 <= s_6_0
        fn_state.ga_249159 = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
