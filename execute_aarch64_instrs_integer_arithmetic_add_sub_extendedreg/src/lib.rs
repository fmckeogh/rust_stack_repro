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
use SP_read::*;
use SP_set::*;
use AddWithCarry::*;
use ExtendReg::*;
use common::*;
pub fn execute_aarch64_instrs_integer_arithmetic_add_sub_extendedreg<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    extend_type: u32,
    m: i64,
    n: i64,
    setflags: bool,
    shift: i64,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasizeshadow_1029: i64,
        nzcv: u8,
        carry_in: bool,
        gs_143068: bool,
        result: Bits,
        operand1: Bits,
        ga_249166: ProductTyped54bc449dd09e5bd,
        operand2: Bits,
        d: i64,
        datasize: i64,
        extend_type: u32,
        m: i64,
        n: i64,
        setflags: bool,
        shift: i64,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        extend_type,
        m,
        n,
        setflags,
        shift,
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
        // D s_0_3: write-var datasizeshadow#1029 <= s_0_2
        fn_state.datasizeshadow_1029 = s_0_2;
        // C s_0_4: const #31s : i
        let s_0_4: i128 = 31;
        // D s_0_5: read-var n:i64
        let s_0_5: i64 = fn_state.n;
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: cmp-eq s_0_6 s_0_4
        let s_0_7: bool = ((s_0_6) == (s_0_4));
        // N s_0_8: branch s_0_7 b14 b1
        if s_0_7 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1029:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1029;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call X_read(s_1_4, s_1_2)
        let s_1_5: Bits = X_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var operand1 <= s_1_5
        fn_state.operand1 = s_1_5;
        // N s_1_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var datasizeshadow#1029:i64
        let s_2_0: i64 = fn_state.datasizeshadow_1029;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: cast reint s_2_1 -> i64
        let s_2_2: i64 = (s_2_1 as i64);
        // D s_2_3: read-var m:i64
        let s_2_3: i64 = fn_state.m;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: read-var shift:i64
        let s_2_5: i64 = fn_state.shift;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: cast zx s_2_2 -> i
        let s_2_7: i128 = (i128::try_from(s_2_2).unwrap());
        // D s_2_8: read-var extend_type:u32
        let s_2_8: u32 = fn_state.extend_type;
        // D s_2_9: call ExtendReg(s_2_4, s_2_8, s_2_6, s_2_7)
        let s_2_9: Bits = ExtendReg(state, tracer, s_2_4, s_2_8, s_2_6, s_2_7);
        // D s_2_10: write-var operand2 <= s_2_9
        fn_state.operand2 = s_2_9;
        // D s_2_11: read-var sub_op:u8
        let s_2_11: bool = fn_state.sub_op;
        // N s_2_12: branch s_2_11 b13 b3
        if s_2_11 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var carry_in <= s_3_0
        fn_state.carry_in = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
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
        // D s_4_2: read-var carry_in:u8
        let s_4_2: bool = fn_state.carry_in;
        // D s_4_3: call AddWithCarry(s_4_0, s_4_1, s_4_2)
        let s_4_3: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_4_0,
            s_4_1,
            s_4_2,
        );
        // D s_4_4: write-var ga#249166 <= s_4_3
        fn_state.ga_249166 = s_4_3;
        // D s_4_5: read-var ga#249166.0:struct
        let s_4_5: Bits = fn_state.ga_249166._0;
        // D s_4_6: read-var ga#249166.1:struct
        let s_4_6: u8 = fn_state.ga_249166._1;
        // D s_4_7: write-var result <= s_4_5
        fn_state.result = s_4_5;
        // D s_4_8: write-var nzcv <= s_4_6
        fn_state.nzcv = s_4_6;
        // D s_4_9: read-var setflags:u8
        let s_4_9: bool = fn_state.setflags;
        // N s_4_10: branch s_4_9 b12 b5
        if s_4_9 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
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
        // C s_6_0: const #31s : i
        let s_6_0: i128 = 31;
        // D s_6_1: read-var d:i64
        let s_6_1: i64 = fn_state.d;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cmp-eq s_6_2 s_6_0
        let s_6_3: bool = ((s_6_2) == (s_6_0));
        // N s_6_4: branch s_6_3 b11 b7
        if s_6_3 {
            return block_11(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#143068 <= s_7_0
        fn_state.gs_143068 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#143068:u8
        let s_8_0: bool = fn_state.gs_143068;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
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
        // D s_9_0: read-var datasizeshadow#1029:i64
        let s_9_0: i64 = fn_state.datasizeshadow_1029;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // D s_9_3: read-var d:i64
        let s_9_3: i64 = fn_state.d;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: read-var result:bv
        let s_9_5: Bits = fn_state.result;
        // D s_9_6: call X_set(s_9_4, s_9_2, s_9_5)
        let s_9_6: () = X_set(state, tracer, s_9_4, s_9_2, s_9_5);
        // N s_9_7: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #64s : i
        let s_10_0: i128 = 64;
        // D s_10_1: read-var result:bv
        let s_10_1: Bits = fn_state.result;
        // D s_10_2: bits-cast zx s_10_1 -> bv length s_10_0
        let s_10_2: Bits = s_10_1.zero_extend(s_10_0);
        // D s_10_3: cast reint s_10_2 -> u64
        let s_10_3: u64 = (s_10_2.value() as u64);
        // D s_10_4: call SP_set(s_10_3)
        let s_10_4: () = SP_set(state, tracer, s_10_3);
        // N s_10_5: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var setflags:u8
        let s_11_0: bool = fn_state.setflags;
        // D s_11_1: not s_11_0
        let s_11_1: bool = !s_11_0;
        // D s_11_2: write-var gs#143068 <= s_11_1
        fn_state.gs_143068 = s_11_1;
        // N s_11_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #3s : i
        let s_12_0: i128 = 3;
        // D s_12_1: read-var nzcv:u8
        let s_12_1: u8 = fn_state.nzcv;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 4u16);
        // C s_12_3: const #1s : i64
        let s_12_3: i64 = 1;
        // C s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // C s_12_5: const #0s : i
        let s_12_5: i128 = 0;
        // C s_12_6: add s_12_5 s_12_4
        let s_12_6: i128 = (s_12_5 + s_12_4);
        // D s_12_7: bit-extract s_12_2 s_12_0 s_12_6
        let s_12_7: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_6).unwrap(),
        ));
        // D s_12_8: cast reint s_12_7 -> u8
        let s_12_8: bool = ((s_12_7.value()) != 0);
        // C s_12_9: const #16984u : u32
        let s_12_9: u32 = 16984;
        // N s_12_10: write-reg s_12_9 <= s_12_8
        let s_12_10: () = {
            state.write_register::<bool>(s_12_9 as isize, s_12_8);
            tracer.write_register(s_12_9 as isize, s_12_8);
        };
        // C s_12_11: const #2s : i
        let s_12_11: i128 = 2;
        // D s_12_12: read-var nzcv:u8
        let s_12_12: u8 = fn_state.nzcv;
        // D s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 4u16);
        // C s_12_14: const #1s : i64
        let s_12_14: i64 = 1;
        // C s_12_15: cast zx s_12_14 -> i
        let s_12_15: i128 = (i128::try_from(s_12_14).unwrap());
        // C s_12_16: const #0s : i
        let s_12_16: i128 = 0;
        // C s_12_17: add s_12_16 s_12_15
        let s_12_17: i128 = (s_12_16 + s_12_15);
        // D s_12_18: bit-extract s_12_13 s_12_11 s_12_17
        let s_12_18: Bits = (Bits::new(
            ((s_12_13) >> (s_12_11)).value(),
            u16::try_from(s_12_17).unwrap(),
        ));
        // D s_12_19: cast reint s_12_18 -> u8
        let s_12_19: bool = ((s_12_18.value()) != 0);
        // C s_12_20: const #16997u : u32
        let s_12_20: u32 = 16997;
        // N s_12_21: write-reg s_12_20 <= s_12_19
        let s_12_21: () = {
            state.write_register::<bool>(s_12_20 as isize, s_12_19);
            tracer.write_register(s_12_20 as isize, s_12_19);
        };
        // C s_12_22: const #1s : i
        let s_12_22: i128 = 1;
        // D s_12_23: read-var nzcv:u8
        let s_12_23: u8 = fn_state.nzcv;
        // D s_12_24: cast zx s_12_23 -> bv
        let s_12_24: Bits = Bits::new(s_12_23 as u128, 4u16);
        // C s_12_25: const #1s : i64
        let s_12_25: i64 = 1;
        // C s_12_26: cast zx s_12_25 -> i
        let s_12_26: i128 = (i128::try_from(s_12_25).unwrap());
        // C s_12_27: const #0s : i
        let s_12_27: i128 = 0;
        // C s_12_28: add s_12_27 s_12_26
        let s_12_28: i128 = (s_12_27 + s_12_26);
        // D s_12_29: bit-extract s_12_24 s_12_22 s_12_28
        let s_12_29: Bits = (Bits::new(
            ((s_12_24) >> (s_12_22)).value(),
            u16::try_from(s_12_28).unwrap(),
        ));
        // D s_12_30: cast reint s_12_29 -> u8
        let s_12_30: bool = ((s_12_29.value()) != 0);
        // C s_12_31: const #16971u : u32
        let s_12_31: u32 = 16971;
        // N s_12_32: write-reg s_12_31 <= s_12_30
        let s_12_32: () = {
            state.write_register::<bool>(s_12_31 as isize, s_12_30);
            tracer.write_register(s_12_31 as isize, s_12_30);
        };
        // C s_12_33: const #0s : i
        let s_12_33: i128 = 0;
        // D s_12_34: read-var nzcv:u8
        let s_12_34: u8 = fn_state.nzcv;
        // D s_12_35: cast zx s_12_34 -> bv
        let s_12_35: Bits = Bits::new(s_12_34 as u128, 4u16);
        // C s_12_36: const #1s : i64
        let s_12_36: i64 = 1;
        // C s_12_37: cast zx s_12_36 -> i
        let s_12_37: i128 = (i128::try_from(s_12_36).unwrap());
        // C s_12_38: const #0s : i
        let s_12_38: i128 = 0;
        // C s_12_39: add s_12_38 s_12_37
        let s_12_39: i128 = (s_12_38 + s_12_37);
        // D s_12_40: bit-extract s_12_35 s_12_33 s_12_39
        let s_12_40: Bits = (Bits::new(
            ((s_12_35) >> (s_12_33)).value(),
            u16::try_from(s_12_39).unwrap(),
        ));
        // D s_12_41: cast reint s_12_40 -> u8
        let s_12_41: bool = ((s_12_40.value()) != 0);
        // C s_12_42: const #16996u : u32
        let s_12_42: u32 = 16996;
        // N s_12_43: write-reg s_12_42 <= s_12_41
        let s_12_43: () = {
            state.write_register::<bool>(s_12_42 as isize, s_12_41);
            tracer.write_register(s_12_42 as isize, s_12_41);
        };
        // N s_12_44: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var operand2:bv
        let s_13_0: Bits = fn_state.operand2;
        // D s_13_1: not s_13_0
        let s_13_1: Bits = !s_13_0;
        // D s_13_2: write-var operand2 <= s_13_1
        fn_state.operand2 = s_13_1;
        // C s_13_3: const #1u : u8
        let s_13_3: bool = true;
        // D s_13_4: write-var carry_in <= s_13_3
        fn_state.carry_in = s_13_3;
        // N s_13_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call SP_read(s_14_0)
        let s_14_1: u64 = SP_read(state, tracer, s_14_0);
        // C s_14_2: const #1s : i
        let s_14_2: i128 = 1;
        // D s_14_3: read-var datasizeshadow#1029:i64
        let s_14_3: i64 = fn_state.datasizeshadow_1029;
        // D s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_5: sub s_14_4 s_14_2
        let s_14_5: i128 = ((s_14_4) - (s_14_2));
        // D s_14_6: cast reint s_14_5 -> i64
        let s_14_6: i64 = (s_14_5 as i64);
        // C s_14_7: const #0s : i
        let s_14_7: i128 = 0;
        // S s_14_8: cast zx s_14_1 -> bv
        let s_14_8: Bits = Bits::new(s_14_1 as u128, 64u16);
        // D s_14_9: cast zx s_14_6 -> i
        let s_14_9: i128 = (i128::try_from(s_14_6).unwrap());
        // C s_14_10: const #1s : i64
        let s_14_10: i64 = 1;
        // C s_14_11: cast zx s_14_10 -> i
        let s_14_11: i128 = (i128::try_from(s_14_10).unwrap());
        // D s_14_12: sub s_14_9 s_14_7
        let s_14_12: i128 = ((s_14_9) - (s_14_7));
        // D s_14_13: add s_14_12 s_14_11
        let s_14_13: i128 = (s_14_12 + s_14_11);
        // D s_14_14: bit-extract s_14_8 s_14_7 s_14_13
        let s_14_14: Bits = (Bits::new(
            ((s_14_8) >> (s_14_7)).value(),
            u16::try_from(s_14_13).unwrap(),
        ));
        // D s_14_15: write-var operand1 <= s_14_14
        fn_state.operand1 = s_14_14;
        // N s_14_16: jump b2
        return block_2(state, tracer, fn_state);
    }
}
