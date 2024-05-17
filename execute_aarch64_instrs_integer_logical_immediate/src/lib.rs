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
use SP_set::*;
use common::*;
pub fn execute_aarch64_instrs_integer_logical_immediate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    imm: Bits,
    n: i64,
    op: u32,
    setflags: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        datasizeshadow_1038: i64,
        gs_143350: bool,
        result: Bits,
        operand1: Bits,
        resultshadow_1039: Bits,
        operand2: Bits,
        d: i64,
        datasize: i64,
        imm: Bits,
        n: i64,
        op: u32,
        setflags: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        imm,
        n,
        op,
        setflags,
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
        // D s_0_3: write-var datasizeshadow#1038 <= s_0_2
        fn_state.datasizeshadow_1038 = s_0_2;
        // D s_0_4: read-var datasizeshadow#1038:i64
        let s_0_4: i64 = fn_state.datasizeshadow_1038;
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
        // D s_0_11: read-var imm:bv
        let s_0_11: Bits = fn_state.imm;
        // D s_0_12: write-var operand2 <= s_0_11
        fn_state.operand2 = s_0_11;
        // C s_0_13: const #0u : u32
        let s_0_13: u32 = 0;
        // D s_0_14: read-var op:u32
        let s_0_14: u32 = fn_state.op;
        // D s_0_15: cmp-eq s_0_13 s_0_14
        let s_0_15: bool = ((s_0_13) == (s_0_14));
        // D s_0_16: not s_0_15
        let s_0_16: bool = !s_0_15;
        // N s_0_17: branch s_0_16 b11 b1
        if s_0_16 {
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
        // D s_1_0: read-var operand1:bv
        let s_1_0: Bits = fn_state.operand1;
        // D s_1_1: read-var operand2:bv
        let s_1_1: Bits = fn_state.operand2;
        // D s_1_2: and s_1_0 s_1_1
        let s_1_2: Bits = ((s_1_0) & (s_1_1));
        // D s_1_3: write-var result <= s_1_2
        fn_state.result = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var result:bv
        let s_2_0: Bits = fn_state.result;
        // D s_2_1: write-var resultshadow#1039 <= s_2_0
        fn_state.resultshadow_1039 = s_2_0;
        // D s_2_2: read-var setflags:u8
        let s_2_2: bool = fn_state.setflags;
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
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
        // C s_4_0: const #31s : i
        let s_4_0: i128 = 31;
        // D s_4_1: read-var d:i64
        let s_4_1: i64 = fn_state.d;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // N s_4_4: branch s_4_3 b9 b5
        if s_4_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#143350 <= s_5_0
        fn_state.gs_143350 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#143350:u8
        let s_6_0: bool = fn_state.gs_143350;
        // N s_6_1: branch s_6_0 b8 b7
        if s_6_0 {
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
        // D s_7_0: read-var datasizeshadow#1038:i64
        let s_7_0: i64 = fn_state.datasizeshadow_1038;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var d:i64
        let s_7_3: i64 = fn_state.d;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: read-var resultshadow#1039:bv
        let s_7_5: Bits = fn_state.resultshadow_1039;
        // D s_7_6: call X_set(s_7_4, s_7_2, s_7_5)
        let s_7_6: () = X_set(state, tracer, s_7_4, s_7_2, s_7_5);
        // N s_7_7: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i
        let s_8_0: i128 = 64;
        // D s_8_1: read-var resultshadow#1039:bv
        let s_8_1: Bits = fn_state.resultshadow_1039;
        // D s_8_2: bits-cast zx s_8_1 -> bv length s_8_0
        let s_8_2: Bits = s_8_1.zero_extend(s_8_0);
        // D s_8_3: cast reint s_8_2 -> u64
        let s_8_3: u64 = (s_8_2.value() as u64);
        // D s_8_4: call SP_set(s_8_3)
        let s_8_4: () = SP_set(state, tracer, s_8_3);
        // N s_8_5: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var setflags:u8
        let s_9_0: bool = fn_state.setflags;
        // D s_9_1: not s_9_0
        let s_9_1: bool = !s_9_0;
        // D s_9_2: write-var gs#143350 <= s_9_1
        fn_state.gs_143350 = s_9_1;
        // N s_9_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1s : i
        let s_10_0: i128 = 1;
        // D s_10_1: read-var datasizeshadow#1038:i64
        let s_10_1: i64 = fn_state.datasizeshadow_1038;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: sub s_10_2 s_10_0
        let s_10_3: i128 = ((s_10_2) - (s_10_0));
        // D s_10_4: cast reint s_10_3 -> i64
        let s_10_4: i64 = (s_10_3 as i64);
        // D s_10_5: cast zx s_10_4 -> i
        let s_10_5: i128 = (i128::try_from(s_10_4).unwrap());
        // D s_10_6: read-var resultshadow#1039:bv
        let s_10_6: Bits = fn_state.resultshadow_1039;
        // C s_10_7: const #1u : u64
        let s_10_7: u64 = 1;
        // D s_10_8: bit-extract s_10_6 s_10_5 s_10_7
        let s_10_8: Bits = (Bits::new(
            ((s_10_6) >> (s_10_5)).value(),
            u16::try_from(s_10_7).unwrap(),
        ));
        // D s_10_9: cast reint s_10_8 -> u8
        let s_10_9: bool = ((s_10_8.value()) != 0);
        // C s_10_10: const #0s : i
        let s_10_10: i128 = 0;
        // C s_10_11: const #0u : u64
        let s_10_11: u64 = 0;
        // D s_10_12: cast zx s_10_9 -> u64
        let s_10_12: u64 = (s_10_9 as u64);
        // C s_10_13: const #1u : u64
        let s_10_13: u64 = 1;
        // D s_10_14: and s_10_12 s_10_13
        let s_10_14: u64 = ((s_10_12) & (s_10_13));
        // D s_10_15: cmp-eq s_10_14 s_10_13
        let s_10_15: bool = ((s_10_14) == (s_10_13));
        // D s_10_16: lsl s_10_12 s_10_10
        let s_10_16: u64 = s_10_12 << s_10_10;
        // D s_10_17: or s_10_11 s_10_16
        let s_10_17: u64 = ((s_10_11) | (s_10_16));
        // D s_10_18: cmpl s_10_16
        let s_10_18: u64 = !s_10_16;
        // D s_10_19: and s_10_11 s_10_18
        let s_10_19: u64 = ((s_10_11) & (s_10_18));
        // D s_10_20: select s_10_15 s_10_17 s_10_19
        let s_10_20: u64 = if s_10_15 { s_10_17 } else { s_10_19 };
        // D s_10_21: cast trunc s_10_20 -> u8
        let s_10_21: bool = ((s_10_20) != 0);
        // D s_10_22: read-var resultshadow#1039:bv
        let s_10_22: Bits = fn_state.resultshadow_1039;
        // D s_10_23: call IsZeroBit(s_10_22)
        let s_10_23: bool = IsZeroBit(state, tracer, s_10_22);
        // D s_10_24: cast zx s_10_21 -> bv
        let s_10_24: Bits = Bits::new(s_10_21 as u128, 1u16);
        // D s_10_25: cast zx s_10_23 -> bv
        let s_10_25: Bits = Bits::new(s_10_23 as u128, 1u16);
        // D s_10_26: cast reint s_10_24 -> u128
        let s_10_26: u128 = (s_10_24.value() as u128);
        // D s_10_27: size-of s_10_24
        let s_10_27: u16 = s_10_24.length();
        // D s_10_28: cast reint s_10_25 -> u128
        let s_10_28: u128 = (s_10_25.value() as u128);
        // D s_10_29: size-of s_10_25
        let s_10_29: u16 = s_10_25.length();
        // D s_10_30: lsl s_10_26 s_10_29
        let s_10_30: u128 = s_10_26 << s_10_29;
        // D s_10_31: or s_10_30 s_10_28
        let s_10_31: u128 = ((s_10_30) | (s_10_28));
        // D s_10_32: add s_10_27 s_10_29
        let s_10_32: u16 = (s_10_27 + s_10_29);
        // D s_10_33: create-bits s_10_31 s_10_32
        let s_10_33: Bits = Bits::new(s_10_31, s_10_32);
        // D s_10_34: cast reint s_10_33 -> u8
        let s_10_34: u8 = (s_10_33.value() as u8);
        // D s_10_35: cast zx s_10_34 -> bv
        let s_10_35: Bits = Bits::new(s_10_34 as u128, 2u16);
        // C s_10_36: const #0u : u8
        let s_10_36: u8 = 0;
        // C s_10_37: cast zx s_10_36 -> bv
        let s_10_37: Bits = Bits::new(s_10_36 as u128, 2u16);
        // D s_10_38: cast reint s_10_35 -> u128
        let s_10_38: u128 = (s_10_35.value() as u128);
        // D s_10_39: size-of s_10_35
        let s_10_39: u16 = s_10_35.length();
        // C s_10_40: cast reint s_10_37 -> u128
        let s_10_40: u128 = (s_10_37.value() as u128);
        // D s_10_41: size-of s_10_37
        let s_10_41: u16 = s_10_37.length();
        // D s_10_42: lsl s_10_38 s_10_41
        let s_10_42: u128 = s_10_38 << s_10_41;
        // D s_10_43: or s_10_42 s_10_40
        let s_10_43: u128 = ((s_10_42) | (s_10_40));
        // D s_10_44: add s_10_39 s_10_41
        let s_10_44: u16 = (s_10_39 + s_10_41);
        // D s_10_45: create-bits s_10_43 s_10_44
        let s_10_45: Bits = Bits::new(s_10_43, s_10_44);
        // D s_10_46: cast reint s_10_45 -> u8
        let s_10_46: u8 = (s_10_45.value() as u8);
        // C s_10_47: const #3s : i
        let s_10_47: i128 = 3;
        // D s_10_48: cast zx s_10_46 -> bv
        let s_10_48: Bits = Bits::new(s_10_46 as u128, 4u16);
        // C s_10_49: const #1s : i64
        let s_10_49: i64 = 1;
        // C s_10_50: cast zx s_10_49 -> i
        let s_10_50: i128 = (i128::try_from(s_10_49).unwrap());
        // C s_10_51: const #0s : i
        let s_10_51: i128 = 0;
        // C s_10_52: add s_10_51 s_10_50
        let s_10_52: i128 = (s_10_51 + s_10_50);
        // D s_10_53: bit-extract s_10_48 s_10_47 s_10_52
        let s_10_53: Bits = (Bits::new(
            ((s_10_48) >> (s_10_47)).value(),
            u16::try_from(s_10_52).unwrap(),
        ));
        // D s_10_54: cast reint s_10_53 -> u8
        let s_10_54: bool = ((s_10_53.value()) != 0);
        // C s_10_55: const #16984u : u32
        let s_10_55: u32 = 16984;
        // N s_10_56: write-reg s_10_55 <= s_10_54
        let s_10_56: () = {
            state.write_register::<bool>(s_10_55 as isize, s_10_54);
            tracer.write_register(s_10_55 as isize, s_10_54);
        };
        // C s_10_57: const #2s : i
        let s_10_57: i128 = 2;
        // D s_10_58: cast zx s_10_46 -> bv
        let s_10_58: Bits = Bits::new(s_10_46 as u128, 4u16);
        // C s_10_59: const #1s : i64
        let s_10_59: i64 = 1;
        // C s_10_60: cast zx s_10_59 -> i
        let s_10_60: i128 = (i128::try_from(s_10_59).unwrap());
        // C s_10_61: const #0s : i
        let s_10_61: i128 = 0;
        // C s_10_62: add s_10_61 s_10_60
        let s_10_62: i128 = (s_10_61 + s_10_60);
        // D s_10_63: bit-extract s_10_58 s_10_57 s_10_62
        let s_10_63: Bits = (Bits::new(
            ((s_10_58) >> (s_10_57)).value(),
            u16::try_from(s_10_62).unwrap(),
        ));
        // D s_10_64: cast reint s_10_63 -> u8
        let s_10_64: bool = ((s_10_63.value()) != 0);
        // C s_10_65: const #16997u : u32
        let s_10_65: u32 = 16997;
        // N s_10_66: write-reg s_10_65 <= s_10_64
        let s_10_66: () = {
            state.write_register::<bool>(s_10_65 as isize, s_10_64);
            tracer.write_register(s_10_65 as isize, s_10_64);
        };
        // C s_10_67: const #1s : i
        let s_10_67: i128 = 1;
        // D s_10_68: cast zx s_10_46 -> bv
        let s_10_68: Bits = Bits::new(s_10_46 as u128, 4u16);
        // C s_10_69: const #1s : i64
        let s_10_69: i64 = 1;
        // C s_10_70: cast zx s_10_69 -> i
        let s_10_70: i128 = (i128::try_from(s_10_69).unwrap());
        // C s_10_71: const #0s : i
        let s_10_71: i128 = 0;
        // C s_10_72: add s_10_71 s_10_70
        let s_10_72: i128 = (s_10_71 + s_10_70);
        // D s_10_73: bit-extract s_10_68 s_10_67 s_10_72
        let s_10_73: Bits = (Bits::new(
            ((s_10_68) >> (s_10_67)).value(),
            u16::try_from(s_10_72).unwrap(),
        ));
        // D s_10_74: cast reint s_10_73 -> u8
        let s_10_74: bool = ((s_10_73.value()) != 0);
        // C s_10_75: const #16971u : u32
        let s_10_75: u32 = 16971;
        // N s_10_76: write-reg s_10_75 <= s_10_74
        let s_10_76: () = {
            state.write_register::<bool>(s_10_75 as isize, s_10_74);
            tracer.write_register(s_10_75 as isize, s_10_74);
        };
        // C s_10_77: const #0s : i
        let s_10_77: i128 = 0;
        // D s_10_78: cast zx s_10_46 -> bv
        let s_10_78: Bits = Bits::new(s_10_46 as u128, 4u16);
        // C s_10_79: const #1s : i64
        let s_10_79: i64 = 1;
        // C s_10_80: cast zx s_10_79 -> i
        let s_10_80: i128 = (i128::try_from(s_10_79).unwrap());
        // C s_10_81: const #0s : i
        let s_10_81: i128 = 0;
        // C s_10_82: add s_10_81 s_10_80
        let s_10_82: i128 = (s_10_81 + s_10_80);
        // D s_10_83: bit-extract s_10_78 s_10_77 s_10_82
        let s_10_83: Bits = (Bits::new(
            ((s_10_78) >> (s_10_77)).value(),
            u16::try_from(s_10_82).unwrap(),
        ));
        // D s_10_84: cast reint s_10_83 -> u8
        let s_10_84: bool = ((s_10_83.value()) != 0);
        // C s_10_85: const #16996u : u32
        let s_10_85: u32 = 16996;
        // N s_10_86: write-reg s_10_85 <= s_10_84
        let s_10_86: () = {
            state.write_register::<bool>(s_10_85 as isize, s_10_84);
            tracer.write_register(s_10_85 as isize, s_10_84);
        };
        // N s_10_87: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #2u : u32
        let s_11_0: u32 = 2;
        // D s_11_1: read-var op:u32
        let s_11_1: u32 = fn_state.op;
        // D s_11_2: cmp-eq s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) == (s_11_1));
        // D s_11_3: not s_11_2
        let s_11_3: bool = !s_11_2;
        // N s_11_4: branch s_11_3 b13 b12
        if s_11_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var operand1:bv
        let s_12_0: Bits = fn_state.operand1;
        // D s_12_1: read-var operand2:bv
        let s_12_1: Bits = fn_state.operand2;
        // D s_12_2: or s_12_0 s_12_1
        let s_12_2: Bits = ((s_12_0) | (s_12_1));
        // D s_12_3: write-var result <= s_12_2
        fn_state.result = s_12_2;
        // N s_12_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #1u : u32
        let s_13_0: u32 = 1;
        // D s_13_1: read-var op:u32
        let s_13_1: u32 = fn_state.op;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b15 b14
        if s_13_3 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var operand1:bv
        let s_14_0: Bits = fn_state.operand1;
        // D s_14_1: read-var operand2:bv
        let s_14_1: Bits = fn_state.operand2;
        // D s_14_2: xor s_14_0 s_14_1
        let s_14_2: Bits = ((s_14_0) ^ (s_14_1));
        // D s_14_3: write-var result <= s_14_2
        fn_state.result = s_14_2;
        // N s_14_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
