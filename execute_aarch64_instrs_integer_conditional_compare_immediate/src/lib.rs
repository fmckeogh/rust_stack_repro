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
use SPESampleAddOpOther::*;
use ConditionHolds::*;
use AddWithCarry::*;
use common::*;
pub fn execute_aarch64_instrs_integer_conditional_compare_immediate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    condition: u8,
    datasize: i64,
    flags__arg: u8,
    imm: Bits,
    n: i64,
    sub_op: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        carry_in: bool,
        taken: bool,
        operand1: Bits,
        flags: u8,
        ga_251038: ProductTyped54bc449dd09e5bd,
        operand2: Bits,
        datasizeshadow_1115: i64,
        condition: u8,
        datasize: i64,
        flags__arg: u8,
        imm: Bits,
        n: i64,
        sub_op: bool,
    }
    let fn_state = FunctionState {
        condition,
        datasize,
        flags__arg,
        imm,
        n,
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
        // D s_0_3: write-var datasizeshadow#1115 <= s_0_2
        fn_state.datasizeshadow_1115 = s_0_2;
        // D s_0_4: read-var flags__arg:u8
        let s_0_4: u8 = fn_state.flags__arg;
        // D s_0_5: write-var flags <= s_0_4
        fn_state.flags = s_0_4;
        // C s_0_6: const #0u : u8
        let s_0_6: bool = false;
        // D s_0_7: write-var taken <= s_0_6
        fn_state.taken = s_0_6;
        // D s_0_8: read-var condition:u8
        let s_0_8: u8 = fn_state.condition;
        // D s_0_9: call ConditionHolds(s_0_8)
        let s_0_9: bool = ConditionHolds(state, tracer, s_0_8);
        // N s_0_10: branch s_0_9 b5 b1
        if s_0_9 {
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
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #3s : i
        let s_2_0: i128 = 3;
        // D s_2_1: read-var flags:u8
        let s_2_1: u8 = fn_state.flags;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 4u16);
        // C s_2_3: const #1s : i64
        let s_2_3: i64 = 1;
        // C s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // C s_2_5: const #0s : i
        let s_2_5: i128 = 0;
        // C s_2_6: add s_2_5 s_2_4
        let s_2_6: i128 = (s_2_5 + s_2_4);
        // D s_2_7: bit-extract s_2_2 s_2_0 s_2_6
        let s_2_7: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_6).unwrap(),
        ));
        // D s_2_8: cast reint s_2_7 -> u8
        let s_2_8: bool = ((s_2_7.value()) != 0);
        // C s_2_9: const #16984u : u32
        let s_2_9: u32 = 16984;
        // N s_2_10: write-reg s_2_9 <= s_2_8
        let s_2_10: () = {
            state.write_register::<bool>(s_2_9 as isize, s_2_8);
            tracer.write_register(s_2_9 as isize, s_2_8);
        };
        // C s_2_11: const #2s : i
        let s_2_11: i128 = 2;
        // D s_2_12: read-var flags:u8
        let s_2_12: u8 = fn_state.flags;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 4u16);
        // C s_2_14: const #1s : i64
        let s_2_14: i64 = 1;
        // C s_2_15: cast zx s_2_14 -> i
        let s_2_15: i128 = (i128::try_from(s_2_14).unwrap());
        // C s_2_16: const #0s : i
        let s_2_16: i128 = 0;
        // C s_2_17: add s_2_16 s_2_15
        let s_2_17: i128 = (s_2_16 + s_2_15);
        // D s_2_18: bit-extract s_2_13 s_2_11 s_2_17
        let s_2_18: Bits = (Bits::new(
            ((s_2_13) >> (s_2_11)).value(),
            u16::try_from(s_2_17).unwrap(),
        ));
        // D s_2_19: cast reint s_2_18 -> u8
        let s_2_19: bool = ((s_2_18.value()) != 0);
        // C s_2_20: const #16997u : u32
        let s_2_20: u32 = 16997;
        // N s_2_21: write-reg s_2_20 <= s_2_19
        let s_2_21: () = {
            state.write_register::<bool>(s_2_20 as isize, s_2_19);
            tracer.write_register(s_2_20 as isize, s_2_19);
        };
        // C s_2_22: const #1s : i
        let s_2_22: i128 = 1;
        // D s_2_23: read-var flags:u8
        let s_2_23: u8 = fn_state.flags;
        // D s_2_24: cast zx s_2_23 -> bv
        let s_2_24: Bits = Bits::new(s_2_23 as u128, 4u16);
        // C s_2_25: const #1s : i64
        let s_2_25: i64 = 1;
        // C s_2_26: cast zx s_2_25 -> i
        let s_2_26: i128 = (i128::try_from(s_2_25).unwrap());
        // C s_2_27: const #0s : i
        let s_2_27: i128 = 0;
        // C s_2_28: add s_2_27 s_2_26
        let s_2_28: i128 = (s_2_27 + s_2_26);
        // D s_2_29: bit-extract s_2_24 s_2_22 s_2_28
        let s_2_29: Bits = (Bits::new(
            ((s_2_24) >> (s_2_22)).value(),
            u16::try_from(s_2_28).unwrap(),
        ));
        // D s_2_30: cast reint s_2_29 -> u8
        let s_2_30: bool = ((s_2_29.value()) != 0);
        // C s_2_31: const #16971u : u32
        let s_2_31: u32 = 16971;
        // N s_2_32: write-reg s_2_31 <= s_2_30
        let s_2_32: () = {
            state.write_register::<bool>(s_2_31 as isize, s_2_30);
            tracer.write_register(s_2_31 as isize, s_2_30);
        };
        // C s_2_33: const #0s : i
        let s_2_33: i128 = 0;
        // D s_2_34: read-var flags:u8
        let s_2_34: u8 = fn_state.flags;
        // D s_2_35: cast zx s_2_34 -> bv
        let s_2_35: Bits = Bits::new(s_2_34 as u128, 4u16);
        // C s_2_36: const #1s : i64
        let s_2_36: i64 = 1;
        // C s_2_37: cast zx s_2_36 -> i
        let s_2_37: i128 = (i128::try_from(s_2_36).unwrap());
        // C s_2_38: const #0s : i
        let s_2_38: i128 = 0;
        // C s_2_39: add s_2_38 s_2_37
        let s_2_39: i128 = (s_2_38 + s_2_37);
        // D s_2_40: bit-extract s_2_35 s_2_33 s_2_39
        let s_2_40: Bits = (Bits::new(
            ((s_2_35) >> (s_2_33)).value(),
            u16::try_from(s_2_39).unwrap(),
        ));
        // D s_2_41: cast reint s_2_40 -> u8
        let s_2_41: bool = ((s_2_40.value()) != 0);
        // C s_2_42: const #16996u : u32
        let s_2_42: u32 = 16996;
        // N s_2_43: write-reg s_2_42 <= s_2_41
        let s_2_43: () = {
            state.write_register::<bool>(s_2_42 as isize, s_2_41);
            tracer.write_register(s_2_42 as isize, s_2_41);
        };
        // C s_2_44: const #22416u : u32
        let s_2_44: u32 = 22416;
        // D s_2_45: read-reg s_2_44:u8
        let s_2_45: bool = {
            let value = state.read_register::<bool>(s_2_44 as isize);
            tracer.read_register(s_2_44 as isize, value);
            value
        };
        // N s_2_46: branch s_2_45 b4 b3
        if s_2_45 {
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
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // D s_4_1: read-var taken:u8
        let s_4_1: bool = fn_state.taken;
        // D s_4_2: call SPESampleAddOpOther(s_4_0, s_4_1)
        let s_4_2: () = SPESampleAddOpOther(state, tracer, s_4_0, s_4_1);
        // N s_4_3: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var datasizeshadow#1115:i64
        let s_5_0: i64 = fn_state.datasizeshadow_1115;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var n:i64
        let s_5_3: i64 = fn_state.n;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: call X_read(s_5_4, s_5_2)
        let s_5_5: Bits = X_read(state, tracer, s_5_4, s_5_2);
        // D s_5_6: write-var operand1 <= s_5_5
        fn_state.operand1 = s_5_5;
        // D s_5_7: read-var imm:bv
        let s_5_7: Bits = fn_state.imm;
        // D s_5_8: write-var operand2 <= s_5_7
        fn_state.operand2 = s_5_7;
        // C s_5_9: const #0u : u8
        let s_5_9: bool = false;
        // D s_5_10: write-var carry_in <= s_5_9
        fn_state.carry_in = s_5_9;
        // D s_5_11: read-var sub_op:u8
        let s_5_11: bool = fn_state.sub_op;
        // N s_5_12: branch s_5_11 b8 b6
        if s_5_11 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
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
        // D s_7_2: read-var carry_in:u8
        let s_7_2: bool = fn_state.carry_in;
        // D s_7_3: call AddWithCarry(s_7_0, s_7_1, s_7_2)
        let s_7_3: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_7_0,
            s_7_1,
            s_7_2,
        );
        // D s_7_4: write-var ga#251038 <= s_7_3
        fn_state.ga_251038 = s_7_3;
        // D s_7_5: read-var ga#251038.1:struct
        let s_7_5: u8 = fn_state.ga_251038._1;
        // D s_7_6: write-var flags <= s_7_5
        fn_state.flags = s_7_5;
        // C s_7_7: const #1u : u8
        let s_7_7: bool = true;
        // D s_7_8: write-var taken <= s_7_7
        fn_state.taken = s_7_7;
        // N s_7_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var operand2:bv
        let s_8_0: Bits = fn_state.operand2;
        // D s_8_1: not s_8_0
        let s_8_1: Bits = !s_8_0;
        // D s_8_2: write-var operand2 <= s_8_1
        fn_state.operand2 = s_8_1;
        // C s_8_3: const #1u : u8
        let s_8_3: bool = true;
        // D s_8_4: write-var carry_in <= s_8_3
        fn_state.carry_in = s_8_3;
        // N s_8_5: jump b7
        return block_7(state, tracer, fn_state);
    }
}
