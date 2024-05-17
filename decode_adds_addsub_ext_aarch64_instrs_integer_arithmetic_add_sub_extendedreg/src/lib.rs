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
use execute_aarch64_instrs_integer_arithmetic_add_sub_extendedreg::*;
use DecodeRegExtend::*;
use common::*;
pub fn decode_adds_addsub_ext_aarch64_instrs_integer_arithmetic_add_sub_extendedreg<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    imm3: u8,
    option_name: u8,
    Rm: u8,
    S: bool,
    op: bool,
    sf: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        m: i64,
        setflags: bool,
        ga_249189: i64,
        datasize: i64,
        shift: i64,
        extend_type: u32,
        n: i64,
        d: i64,
        sub_op: bool,
        Rd: u8,
        Rn: u8,
        imm3: u8,
        option_name: u8,
        Rm: u8,
        S: bool,
        op: bool,
        sf: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        imm3,
        option_name,
        Rm,
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
        // D s_0_10: read-var Rm:u8
        let s_0_10: u8 = fn_state.Rm;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: write-var m <= s_0_13
        fn_state.m = s_0_13;
        // D s_0_15: read-var sf:u8
        let s_0_15: bool = fn_state.sf;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 1u16);
        // C s_0_17: const #1u : u8
        let s_0_17: bool = true;
        // C s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_19: cmp-eq s_0_16 s_0_18
        let s_0_19: bool = ((s_0_16) == (s_0_18));
        // N s_0_20: branch s_0_19 b5 b1
        if s_0_19 {
            return block_5(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#249189 <= s_1_0
        fn_state.ga_249189 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#249189:i64
        let s_2_0: i64 = fn_state.ga_249189;
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
        // D s_2_14: read-var option_name:u8
        let s_2_14: u8 = fn_state.option_name;
        // D s_2_15: call DecodeRegExtend(s_2_14)
        let s_2_15: u32 = DecodeRegExtend(state, tracer, s_2_14);
        // D s_2_16: write-var extend_type <= s_2_15
        fn_state.extend_type = s_2_15;
        // D s_2_17: read-var imm3:u8
        let s_2_17: u8 = fn_state.imm3;
        // D s_2_18: cast zx s_2_17 -> bv
        let s_2_18: Bits = Bits::new(s_2_17 as u128, 3u16);
        // D s_2_19: cast zx s_2_18 -> i
        let s_2_19: i128 = (s_2_18.value() as i128);
        // D s_2_20: cast reint s_2_19 -> i64
        let s_2_20: i64 = (s_2_19 as i64);
        // D s_2_21: write-var shift <= s_2_20
        fn_state.shift = s_2_20;
        // C s_2_22: const #4s : i
        let s_2_22: i128 = 4;
        // D s_2_23: read-var shift:i64
        let s_2_23: i64 = fn_state.shift;
        // D s_2_24: cast zx s_2_23 -> i
        let s_2_24: i128 = (i128::try_from(s_2_23).unwrap());
        // D s_2_25: cmp-gt s_2_24 s_2_22
        let s_2_25: bool = ((s_2_24) > (s_2_22));
        // N s_2_26: branch s_2_25 b4 b3
        if s_2_25 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var datasize:i64
        let s_3_0: i64 = fn_state.datasize;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var d:i64
        let s_3_3: i64 = fn_state.d;
        // D s_3_4: read-var extend_type:u32
        let s_3_4: u32 = fn_state.extend_type;
        // D s_3_5: read-var m:i64
        let s_3_5: i64 = fn_state.m;
        // D s_3_6: read-var n:i64
        let s_3_6: i64 = fn_state.n;
        // D s_3_7: read-var setflags:u8
        let s_3_7: bool = fn_state.setflags;
        // D s_3_8: read-var shift:i64
        let s_3_8: i64 = fn_state.shift;
        // D s_3_9: read-var sub_op:u8
        let s_3_9: bool = fn_state.sub_op;
        // D s_3_10: call execute_aarch64_instrs_integer_arithmetic_add_sub_extendedreg(s_3_3, s_3_2, s_3_4, s_3_5, s_3_6, s_3_7, s_3_8, s_3_9)
        let s_3_10: () = execute_aarch64_instrs_integer_arithmetic_add_sub_extendedreg(
            state,
            tracer,
            s_3_3,
            s_3_2,
            s_3_4,
            s_3_5,
            s_3_6,
            s_3_7,
            s_3_8,
            s_3_9,
        );
        // N s_3_11: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: panic
        panic!("{:?}", ());
        // N s_4_1: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: write-var ga#249189 <= s_5_0
        fn_state.ga_249189 = s_5_0;
        // N s_5_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
