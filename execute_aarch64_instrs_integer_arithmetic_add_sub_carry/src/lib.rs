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
use AddWithCarry::*;
use common::*;
pub fn execute_aarch64_instrs_integer_arithmetic_add_sub_carry<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    m: i64,
    n: i64,
    setflags: bool,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: Bits,
        nzcv: u8,
        operand1: Bits,
        datasizeshadow_1020: i64,
        operand2: Bits,
        ga_248996: ProductTyped54bc449dd09e5bd,
        d: i64,
        datasize: i64,
        m: i64,
        n: i64,
        setflags: bool,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        m,
        n,
        setflags,
        sub_op,
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
        // D s_0_3: write-var datasizeshadow#1020 <= s_0_2
        fn_state.datasizeshadow_1020 = s_0_2;
        // D s_0_4: read-var datasizeshadow#1020:i64
        let s_0_4: i64 = fn_state.datasizeshadow_1020;
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
        // D s_0_10: write-var operand1 <= s_0_9
        fn_state.operand1 = s_0_9;
        // D s_0_11: read-var datasizeshadow#1020:i64
        let s_0_11: i64 = fn_state.datasizeshadow_1020;
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: read-var m:i64
        let s_0_14: i64 = fn_state.m;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: call X_read(s_0_15, s_0_13)
        let s_0_16: Bits = X_read(state, tracer, s_0_15, s_0_13);
        // D s_0_17: write-var operand2 <= s_0_16
        fn_state.operand2 = s_0_16;
        // D s_0_18: read-var sub_op:u8
        let s_0_18: bool = fn_state.sub_op;
        // N s_0_19: branch s_0_18 b6 b1
        if s_0_18 {
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
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #16971u : u32
        let s_2_0: u32 = 16971;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: bool = {
            let value = state.read_register::<bool>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: read-var operand1:bv
        let s_2_2: Bits = fn_state.operand1;
        // D s_2_3: read-var operand2:bv
        let s_2_3: Bits = fn_state.operand2;
        // D s_2_4: call AddWithCarry(s_2_2, s_2_3, s_2_1)
        let s_2_4: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_2_2,
            s_2_3,
            s_2_1,
        );
        // D s_2_5: write-var ga#248996 <= s_2_4
        fn_state.ga_248996 = s_2_4;
        // D s_2_6: read-var ga#248996.0:struct
        let s_2_6: Bits = fn_state.ga_248996._0;
        // D s_2_7: read-var ga#248996.1:struct
        let s_2_7: u8 = fn_state.ga_248996._1;
        // D s_2_8: write-var result <= s_2_6
        fn_state.result = s_2_6;
        // D s_2_9: write-var nzcv <= s_2_7
        fn_state.nzcv = s_2_7;
        // D s_2_10: read-var setflags:u8
        let s_2_10: bool = fn_state.setflags;
        // N s_2_11: branch s_2_10 b5 b3
        if s_2_10 {
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
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasizeshadow#1020:i64
        let s_4_0: i64 = fn_state.datasizeshadow_1020;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: read-var result:bv
        let s_4_5: Bits = fn_state.result;
        // D s_4_6: call X_set(s_4_4, s_4_2, s_4_5)
        let s_4_6: () = X_set(state, tracer, s_4_4, s_4_2, s_4_5);
        // N s_4_7: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #3s : i
        let s_5_0: i128 = 3;
        // D s_5_1: read-var nzcv:u8
        let s_5_1: u8 = fn_state.nzcv;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 4u16);
        // C s_5_3: const #1s : i64
        let s_5_3: i64 = 1;
        // C s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: const #0s : i
        let s_5_5: i128 = 0;
        // C s_5_6: add s_5_5 s_5_4
        let s_5_6: i128 = (s_5_5 + s_5_4);
        // D s_5_7: bit-extract s_5_2 s_5_0 s_5_6
        let s_5_7: Bits = (Bits::new(
            ((s_5_2) >> (s_5_0)).value(),
            u16::try_from(s_5_6).unwrap(),
        ));
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: bool = ((s_5_7.value()) != 0);
        // C s_5_9: const #16984u : u32
        let s_5_9: u32 = 16984;
        // N s_5_10: write-reg s_5_9 <= s_5_8
        let s_5_10: () = {
            state.write_register::<bool>(s_5_9 as isize, s_5_8);
            tracer.write_register(s_5_9 as isize, s_5_8);
        };
        // C s_5_11: const #2s : i
        let s_5_11: i128 = 2;
        // D s_5_12: read-var nzcv:u8
        let s_5_12: u8 = fn_state.nzcv;
        // D s_5_13: cast zx s_5_12 -> bv
        let s_5_13: Bits = Bits::new(s_5_12 as u128, 4u16);
        // C s_5_14: const #1s : i64
        let s_5_14: i64 = 1;
        // C s_5_15: cast zx s_5_14 -> i
        let s_5_15: i128 = (i128::try_from(s_5_14).unwrap());
        // C s_5_16: const #0s : i
        let s_5_16: i128 = 0;
        // C s_5_17: add s_5_16 s_5_15
        let s_5_17: i128 = (s_5_16 + s_5_15);
        // D s_5_18: bit-extract s_5_13 s_5_11 s_5_17
        let s_5_18: Bits = (Bits::new(
            ((s_5_13) >> (s_5_11)).value(),
            u16::try_from(s_5_17).unwrap(),
        ));
        // D s_5_19: cast reint s_5_18 -> u8
        let s_5_19: bool = ((s_5_18.value()) != 0);
        // C s_5_20: const #16997u : u32
        let s_5_20: u32 = 16997;
        // N s_5_21: write-reg s_5_20 <= s_5_19
        let s_5_21: () = {
            state.write_register::<bool>(s_5_20 as isize, s_5_19);
            tracer.write_register(s_5_20 as isize, s_5_19);
        };
        // C s_5_22: const #1s : i
        let s_5_22: i128 = 1;
        // D s_5_23: read-var nzcv:u8
        let s_5_23: u8 = fn_state.nzcv;
        // D s_5_24: cast zx s_5_23 -> bv
        let s_5_24: Bits = Bits::new(s_5_23 as u128, 4u16);
        // C s_5_25: const #1s : i64
        let s_5_25: i64 = 1;
        // C s_5_26: cast zx s_5_25 -> i
        let s_5_26: i128 = (i128::try_from(s_5_25).unwrap());
        // C s_5_27: const #0s : i
        let s_5_27: i128 = 0;
        // C s_5_28: add s_5_27 s_5_26
        let s_5_28: i128 = (s_5_27 + s_5_26);
        // D s_5_29: bit-extract s_5_24 s_5_22 s_5_28
        let s_5_29: Bits = (Bits::new(
            ((s_5_24) >> (s_5_22)).value(),
            u16::try_from(s_5_28).unwrap(),
        ));
        // D s_5_30: cast reint s_5_29 -> u8
        let s_5_30: bool = ((s_5_29.value()) != 0);
        // C s_5_31: const #16971u : u32
        let s_5_31: u32 = 16971;
        // N s_5_32: write-reg s_5_31 <= s_5_30
        let s_5_32: () = {
            state.write_register::<bool>(s_5_31 as isize, s_5_30);
            tracer.write_register(s_5_31 as isize, s_5_30);
        };
        // C s_5_33: const #0s : i
        let s_5_33: i128 = 0;
        // D s_5_34: read-var nzcv:u8
        let s_5_34: u8 = fn_state.nzcv;
        // D s_5_35: cast zx s_5_34 -> bv
        let s_5_35: Bits = Bits::new(s_5_34 as u128, 4u16);
        // C s_5_36: const #1s : i64
        let s_5_36: i64 = 1;
        // C s_5_37: cast zx s_5_36 -> i
        let s_5_37: i128 = (i128::try_from(s_5_36).unwrap());
        // C s_5_38: const #0s : i
        let s_5_38: i128 = 0;
        // C s_5_39: add s_5_38 s_5_37
        let s_5_39: i128 = (s_5_38 + s_5_37);
        // D s_5_40: bit-extract s_5_35 s_5_33 s_5_39
        let s_5_40: Bits = (Bits::new(
            ((s_5_35) >> (s_5_33)).value(),
            u16::try_from(s_5_39).unwrap(),
        ));
        // D s_5_41: cast reint s_5_40 -> u8
        let s_5_41: bool = ((s_5_40.value()) != 0);
        // C s_5_42: const #16996u : u32
        let s_5_42: u32 = 16996;
        // N s_5_43: write-reg s_5_42 <= s_5_41
        let s_5_43: () = {
            state.write_register::<bool>(s_5_42 as isize, s_5_41);
            tracer.write_register(s_5_42 as isize, s_5_41);
        };
        // N s_5_44: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var operand2:bv
        let s_6_0: Bits = fn_state.operand2;
        // D s_6_1: not s_6_0
        let s_6_1: Bits = !s_6_0;
        // D s_6_2: write-var operand2 <= s_6_1
        fn_state.operand2 = s_6_1;
        // N s_6_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
