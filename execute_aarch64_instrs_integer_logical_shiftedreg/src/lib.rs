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
use IsZeroBit::*;
use ShiftReg::*;
use common::*;
pub fn execute_aarch64_instrs_integer_logical_shiftedreg<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    invert: bool,
    m: i64,
    n: i64,
    op: u32,
    setflags: bool,
    shift_amount: i64,
    shift_type: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        resultshadow_1050: Bits,
        datasizeshadow_1049: i64,
        result: Bits,
        operand1: Bits,
        operand2: Bits,
        d: i64,
        datasize: i64,
        invert: bool,
        m: i64,
        n: i64,
        op: u32,
        setflags: bool,
        shift_amount: i64,
        shift_type: u32,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        invert,
        m,
        n,
        op,
        setflags,
        shift_amount,
        shift_type,
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
        // D s_0_3: write-var datasizeshadow#1049 <= s_0_2
        fn_state.datasizeshadow_1049 = s_0_2;
        // D s_0_4: read-var datasizeshadow#1049:i64
        let s_0_4: i64 = fn_state.datasizeshadow_1049;
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
        // D s_0_11: read-var datasizeshadow#1049:i64
        let s_0_11: i64 = fn_state.datasizeshadow_1049;
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: read-var m:i64
        let s_0_14: i64 = fn_state.m;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: read-var shift_amount:i64
        let s_0_16: i64 = fn_state.shift_amount;
        // D s_0_17: cast zx s_0_16 -> i
        let s_0_17: i128 = (i128::try_from(s_0_16).unwrap());
        // D s_0_18: read-var shift_type:u32
        let s_0_18: u32 = fn_state.shift_type;
        // D s_0_19: call ShiftReg(s_0_15, s_0_18, s_0_17, s_0_13)
        let s_0_19: Bits = ShiftReg(state, tracer, s_0_15, s_0_18, s_0_17, s_0_13);
        // D s_0_20: write-var operand2 <= s_0_19
        fn_state.operand2 = s_0_19;
        // D s_0_21: read-var invert:u8
        let s_0_21: bool = fn_state.invert;
        // N s_0_22: branch s_0_21 b13 b1
        if s_0_21 {
            return block_13(state, tracer, fn_state);
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
        // C s_2_0: const #0u : u32
        let s_2_0: u32 = 0;
        // D s_2_1: read-var op:u32
        let s_2_1: u32 = fn_state.op;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b8 b3
        if s_2_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var operand1:bv
        let s_3_0: Bits = fn_state.operand1;
        // D s_3_1: read-var operand2:bv
        let s_3_1: Bits = fn_state.operand2;
        // D s_3_2: and s_3_0 s_3_1
        let s_3_2: Bits = ((s_3_0) & (s_3_1));
        // D s_3_3: write-var result <= s_3_2
        fn_state.result = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var result:bv
        let s_4_0: Bits = fn_state.result;
        // D s_4_1: write-var resultshadow#1050 <= s_4_0
        fn_state.resultshadow_1050 = s_4_0;
        // D s_4_2: read-var setflags:u8
        let s_4_2: bool = fn_state.setflags;
        // N s_4_3: branch s_4_2 b7 b5
        if s_4_2 {
            return block_7(state, tracer, fn_state);
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
        // D s_6_0: read-var datasizeshadow#1049:i64
        let s_6_0: i64 = fn_state.datasizeshadow_1049;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: cast reint s_6_1 -> i64
        let s_6_2: i64 = (s_6_1 as i64);
        // D s_6_3: read-var d:i64
        let s_6_3: i64 = fn_state.d;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: read-var resultshadow#1050:bv
        let s_6_5: Bits = fn_state.resultshadow_1050;
        // D s_6_6: call X_set(s_6_4, s_6_2, s_6_5)
        let s_6_6: () = X_set(state, tracer, s_6_4, s_6_2, s_6_5);
        // N s_6_7: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1s : i
        let s_7_0: i128 = 1;
        // D s_7_1: read-var datasizeshadow#1049:i64
        let s_7_1: i64 = fn_state.datasizeshadow_1049;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: sub s_7_2 s_7_0
        let s_7_3: i128 = ((s_7_2) - (s_7_0));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var resultshadow#1050:bv
        let s_7_6: Bits = fn_state.resultshadow_1050;
        // C s_7_7: const #1u : u64
        let s_7_7: u64 = 1;
        // D s_7_8: bit-extract s_7_6 s_7_5 s_7_7
        let s_7_8: Bits = (Bits::new(
            ((s_7_6) >> (s_7_5)).value(),
            u16::try_from(s_7_7).unwrap(),
        ));
        // D s_7_9: cast reint s_7_8 -> u8
        let s_7_9: bool = ((s_7_8.value()) != 0);
        // C s_7_10: const #0s : i
        let s_7_10: i128 = 0;
        // C s_7_11: const #0u : u64
        let s_7_11: u64 = 0;
        // D s_7_12: cast zx s_7_9 -> u64
        let s_7_12: u64 = (s_7_9 as u64);
        // C s_7_13: const #1u : u64
        let s_7_13: u64 = 1;
        // D s_7_14: and s_7_12 s_7_13
        let s_7_14: u64 = ((s_7_12) & (s_7_13));
        // D s_7_15: cmp-eq s_7_14 s_7_13
        let s_7_15: bool = ((s_7_14) == (s_7_13));
        // D s_7_16: lsl s_7_12 s_7_10
        let s_7_16: u64 = s_7_12 << s_7_10;
        // D s_7_17: or s_7_11 s_7_16
        let s_7_17: u64 = ((s_7_11) | (s_7_16));
        // D s_7_18: cmpl s_7_16
        let s_7_18: u64 = !s_7_16;
        // D s_7_19: and s_7_11 s_7_18
        let s_7_19: u64 = ((s_7_11) & (s_7_18));
        // D s_7_20: select s_7_15 s_7_17 s_7_19
        let s_7_20: u64 = if s_7_15 { s_7_17 } else { s_7_19 };
        // D s_7_21: cast trunc s_7_20 -> u8
        let s_7_21: bool = ((s_7_20) != 0);
        // D s_7_22: read-var resultshadow#1050:bv
        let s_7_22: Bits = fn_state.resultshadow_1050;
        // D s_7_23: call IsZeroBit(s_7_22)
        let s_7_23: bool = IsZeroBit(state, tracer, s_7_22);
        // D s_7_24: cast zx s_7_21 -> bv
        let s_7_24: Bits = Bits::new(s_7_21 as u128, 1u16);
        // D s_7_25: cast zx s_7_23 -> bv
        let s_7_25: Bits = Bits::new(s_7_23 as u128, 1u16);
        // D s_7_26: cast reint s_7_24 -> u128
        let s_7_26: u128 = (s_7_24.value() as u128);
        // D s_7_27: size-of s_7_24
        let s_7_27: u16 = s_7_24.length();
        // D s_7_28: cast reint s_7_25 -> u128
        let s_7_28: u128 = (s_7_25.value() as u128);
        // D s_7_29: size-of s_7_25
        let s_7_29: u16 = s_7_25.length();
        // D s_7_30: lsl s_7_26 s_7_29
        let s_7_30: u128 = s_7_26 << s_7_29;
        // D s_7_31: or s_7_30 s_7_28
        let s_7_31: u128 = ((s_7_30) | (s_7_28));
        // D s_7_32: add s_7_27 s_7_29
        let s_7_32: u16 = (s_7_27 + s_7_29);
        // D s_7_33: create-bits s_7_31 s_7_32
        let s_7_33: Bits = Bits::new(s_7_31, s_7_32);
        // D s_7_34: cast reint s_7_33 -> u8
        let s_7_34: u8 = (s_7_33.value() as u8);
        // D s_7_35: cast zx s_7_34 -> bv
        let s_7_35: Bits = Bits::new(s_7_34 as u128, 2u16);
        // C s_7_36: const #0u : u8
        let s_7_36: u8 = 0;
        // C s_7_37: cast zx s_7_36 -> bv
        let s_7_37: Bits = Bits::new(s_7_36 as u128, 2u16);
        // D s_7_38: cast reint s_7_35 -> u128
        let s_7_38: u128 = (s_7_35.value() as u128);
        // D s_7_39: size-of s_7_35
        let s_7_39: u16 = s_7_35.length();
        // C s_7_40: cast reint s_7_37 -> u128
        let s_7_40: u128 = (s_7_37.value() as u128);
        // D s_7_41: size-of s_7_37
        let s_7_41: u16 = s_7_37.length();
        // D s_7_42: lsl s_7_38 s_7_41
        let s_7_42: u128 = s_7_38 << s_7_41;
        // D s_7_43: or s_7_42 s_7_40
        let s_7_43: u128 = ((s_7_42) | (s_7_40));
        // D s_7_44: add s_7_39 s_7_41
        let s_7_44: u16 = (s_7_39 + s_7_41);
        // D s_7_45: create-bits s_7_43 s_7_44
        let s_7_45: Bits = Bits::new(s_7_43, s_7_44);
        // D s_7_46: cast reint s_7_45 -> u8
        let s_7_46: u8 = (s_7_45.value() as u8);
        // C s_7_47: const #3s : i
        let s_7_47: i128 = 3;
        // D s_7_48: cast zx s_7_46 -> bv
        let s_7_48: Bits = Bits::new(s_7_46 as u128, 4u16);
        // C s_7_49: const #1s : i64
        let s_7_49: i64 = 1;
        // C s_7_50: cast zx s_7_49 -> i
        let s_7_50: i128 = (i128::try_from(s_7_49).unwrap());
        // C s_7_51: const #0s : i
        let s_7_51: i128 = 0;
        // C s_7_52: add s_7_51 s_7_50
        let s_7_52: i128 = (s_7_51 + s_7_50);
        // D s_7_53: bit-extract s_7_48 s_7_47 s_7_52
        let s_7_53: Bits = (Bits::new(
            ((s_7_48) >> (s_7_47)).value(),
            u16::try_from(s_7_52).unwrap(),
        ));
        // D s_7_54: cast reint s_7_53 -> u8
        let s_7_54: bool = ((s_7_53.value()) != 0);
        // C s_7_55: const #16984u : u32
        let s_7_55: u32 = 16984;
        // N s_7_56: write-reg s_7_55 <= s_7_54
        let s_7_56: () = {
            state.write_register::<bool>(s_7_55 as isize, s_7_54);
            tracer.write_register(s_7_55 as isize, s_7_54);
        };
        // C s_7_57: const #2s : i
        let s_7_57: i128 = 2;
        // D s_7_58: cast zx s_7_46 -> bv
        let s_7_58: Bits = Bits::new(s_7_46 as u128, 4u16);
        // C s_7_59: const #1s : i64
        let s_7_59: i64 = 1;
        // C s_7_60: cast zx s_7_59 -> i
        let s_7_60: i128 = (i128::try_from(s_7_59).unwrap());
        // C s_7_61: const #0s : i
        let s_7_61: i128 = 0;
        // C s_7_62: add s_7_61 s_7_60
        let s_7_62: i128 = (s_7_61 + s_7_60);
        // D s_7_63: bit-extract s_7_58 s_7_57 s_7_62
        let s_7_63: Bits = (Bits::new(
            ((s_7_58) >> (s_7_57)).value(),
            u16::try_from(s_7_62).unwrap(),
        ));
        // D s_7_64: cast reint s_7_63 -> u8
        let s_7_64: bool = ((s_7_63.value()) != 0);
        // C s_7_65: const #16997u : u32
        let s_7_65: u32 = 16997;
        // N s_7_66: write-reg s_7_65 <= s_7_64
        let s_7_66: () = {
            state.write_register::<bool>(s_7_65 as isize, s_7_64);
            tracer.write_register(s_7_65 as isize, s_7_64);
        };
        // C s_7_67: const #1s : i
        let s_7_67: i128 = 1;
        // D s_7_68: cast zx s_7_46 -> bv
        let s_7_68: Bits = Bits::new(s_7_46 as u128, 4u16);
        // C s_7_69: const #1s : i64
        let s_7_69: i64 = 1;
        // C s_7_70: cast zx s_7_69 -> i
        let s_7_70: i128 = (i128::try_from(s_7_69).unwrap());
        // C s_7_71: const #0s : i
        let s_7_71: i128 = 0;
        // C s_7_72: add s_7_71 s_7_70
        let s_7_72: i128 = (s_7_71 + s_7_70);
        // D s_7_73: bit-extract s_7_68 s_7_67 s_7_72
        let s_7_73: Bits = (Bits::new(
            ((s_7_68) >> (s_7_67)).value(),
            u16::try_from(s_7_72).unwrap(),
        ));
        // D s_7_74: cast reint s_7_73 -> u8
        let s_7_74: bool = ((s_7_73.value()) != 0);
        // C s_7_75: const #16971u : u32
        let s_7_75: u32 = 16971;
        // N s_7_76: write-reg s_7_75 <= s_7_74
        let s_7_76: () = {
            state.write_register::<bool>(s_7_75 as isize, s_7_74);
            tracer.write_register(s_7_75 as isize, s_7_74);
        };
        // C s_7_77: const #0s : i
        let s_7_77: i128 = 0;
        // D s_7_78: cast zx s_7_46 -> bv
        let s_7_78: Bits = Bits::new(s_7_46 as u128, 4u16);
        // C s_7_79: const #1s : i64
        let s_7_79: i64 = 1;
        // C s_7_80: cast zx s_7_79 -> i
        let s_7_80: i128 = (i128::try_from(s_7_79).unwrap());
        // C s_7_81: const #0s : i
        let s_7_81: i128 = 0;
        // C s_7_82: add s_7_81 s_7_80
        let s_7_82: i128 = (s_7_81 + s_7_80);
        // D s_7_83: bit-extract s_7_78 s_7_77 s_7_82
        let s_7_83: Bits = (Bits::new(
            ((s_7_78) >> (s_7_77)).value(),
            u16::try_from(s_7_82).unwrap(),
        ));
        // D s_7_84: cast reint s_7_83 -> u8
        let s_7_84: bool = ((s_7_83.value()) != 0);
        // C s_7_85: const #16996u : u32
        let s_7_85: u32 = 16996;
        // N s_7_86: write-reg s_7_85 <= s_7_84
        let s_7_86: () = {
            state.write_register::<bool>(s_7_85 as isize, s_7_84);
            tracer.write_register(s_7_85 as isize, s_7_84);
        };
        // N s_7_87: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #2u : u32
        let s_8_0: u32 = 2;
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
        // D s_9_0: read-var operand1:bv
        let s_9_0: Bits = fn_state.operand1;
        // D s_9_1: read-var operand2:bv
        let s_9_1: Bits = fn_state.operand2;
        // D s_9_2: or s_9_0 s_9_1
        let s_9_2: Bits = ((s_9_0) | (s_9_1));
        // D s_9_3: write-var result <= s_9_2
        fn_state.result = s_9_2;
        // N s_9_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1u : u32
        let s_10_0: u32 = 1;
        // D s_10_1: read-var op:u32
        let s_10_1: u32 = fn_state.op;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b12 b11
        if s_10_3 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var operand1:bv
        let s_11_0: Bits = fn_state.operand1;
        // D s_11_1: read-var operand2:bv
        let s_11_1: Bits = fn_state.operand2;
        // D s_11_2: xor s_11_0 s_11_1
        let s_11_2: Bits = ((s_11_0) ^ (s_11_1));
        // D s_11_3: write-var result <= s_11_2
        fn_state.result = s_11_2;
        // N s_11_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b4
        return block_4(state, tracer, fn_state);
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
        // N s_13_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
