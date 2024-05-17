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
use CheckFPEnabled64::*;
use Elem_set::*;
use V_read::*;
use FPCR_read::*;
use V_set::*;
use IsMerging::*;
use FPMinNum::*;
use FPMaxNum::*;
use FPMin::*;
use FPMax::*;
use common::*;
pub fn execute_aarch64_instrs_float_arithmetic_max_min<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
    operation: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand2: Bits,
        ga_255045: Bits,
        fpcr: ProductType5c790c8ef59cc8b2,
        ga_255044: i64,
        ga_255038: i64,
        ga_255041: Bits,
        esizeshadow_1372: i64,
        ga_255043: Bits,
        ga_255040: i64,
        ga_255042: i64,
        result: u128,
        operand1: Bits,
        ga_255039: Bits,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
        operation: u32,
    }
    let fn_state = FunctionState {
        d,
        esize,
        m,
        n,
        operation,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#1372 <= s_0_2
        fn_state.esizeshadow_1372 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPEnabled64(s_0_4)
        let s_0_5: () = CheckFPEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var esizeshadow#1372:i64
        let s_1_0: i64 = fn_state.esizeshadow_1372;
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
        // D s_1_7: read-var esizeshadow#1372:i64
        let s_1_7: i64 = fn_state.esizeshadow_1372;
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
        // C s_1_14: const #() : ()
        let s_1_14: () = ();
        // S s_1_15: call FPCR_read(s_1_14)
        let s_1_15: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_1_14);
        // D s_1_16: write-var fpcr <= s_1_15
        fn_state.fpcr = s_1_15;
        // D s_1_17: read-var fpcr:struct
        let s_1_17: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_1_18: call IsMerging(s_1_17)
        let s_1_18: bool = IsMerging(state, tracer, s_1_17);
        // N s_1_19: branch s_1_18 b17 b2
        if s_1_18 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: u8 = 0;
        // C s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 0u16);
        // C s_2_2: const #0u : u64
        let s_2_2: u64 = 0;
        // C s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 64u16);
        // C s_2_4: cast reint s_2_1 -> u128
        let s_2_4: u128 = (s_2_1.value() as u128);
        // D s_2_5: size-of s_2_1
        let s_2_5: u16 = s_2_1.length();
        // C s_2_6: cast reint s_2_3 -> u128
        let s_2_6: u128 = (s_2_3.value() as u128);
        // D s_2_7: size-of s_2_3
        let s_2_7: u16 = s_2_3.length();
        // D s_2_8: lsl s_2_4 s_2_7
        let s_2_8: u128 = s_2_4 << s_2_7;
        // D s_2_9: or s_2_8 s_2_6
        let s_2_9: u128 = ((s_2_8) | (s_2_6));
        // D s_2_10: add s_2_5 s_2_7
        let s_2_10: u16 = (s_2_5 + s_2_7);
        // D s_2_11: create-bits s_2_9 s_2_10
        let s_2_11: Bits = Bits::new(s_2_9, s_2_10);
        // C s_2_12: const #0u : u64
        let s_2_12: u64 = 0;
        // C s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 64u16);
        // D s_2_14: cast reint s_2_11 -> u128
        let s_2_14: u128 = (s_2_11.value() as u128);
        // D s_2_15: size-of s_2_11
        let s_2_15: u16 = s_2_11.length();
        // C s_2_16: cast reint s_2_13 -> u128
        let s_2_16: u128 = (s_2_13.value() as u128);
        // D s_2_17: size-of s_2_13
        let s_2_17: u16 = s_2_13.length();
        // D s_2_18: lsl s_2_14 s_2_17
        let s_2_18: u128 = s_2_14 << s_2_17;
        // D s_2_19: or s_2_18 s_2_16
        let s_2_19: u128 = ((s_2_18) | (s_2_16));
        // D s_2_20: add s_2_15 s_2_17
        let s_2_20: u16 = (s_2_15 + s_2_17);
        // D s_2_21: create-bits s_2_19 s_2_20
        let s_2_21: Bits = Bits::new(s_2_19, s_2_20);
        // D s_2_22: cast reint s_2_21 -> u128
        let s_2_22: u128 = (s_2_21.value() as u128);
        // D s_2_23: write-var result <= s_2_22
        fn_state.result = s_2_22;
        // N s_2_24: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u32
        let s_3_0: u32 = 0;
        // D s_3_1: read-var operation:u32
        let s_3_1: u32 = fn_state.operation;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b7 b4
        if s_3_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var esizeshadow#1372:i64
        let s_4_0: i64 = fn_state.esizeshadow_1372;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: write-var ga#255038 <= s_4_2
        fn_state.ga_255038 = s_4_2;
        // D s_4_4: read-var operand1:bv
        let s_4_4: Bits = fn_state.operand1;
        // D s_4_5: read-var operand2:bv
        let s_4_5: Bits = fn_state.operand2;
        // D s_4_6: read-var fpcr:struct
        let s_4_6: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_4_7: call FPMax(s_4_4, s_4_5, s_4_6)
        let s_4_7: Bits = FPMax(state, tracer, s_4_4, s_4_5, s_4_6);
        // D s_4_8: write-var ga#255039 <= s_4_7
        fn_state.ga_255039 = s_4_7;
        // N s_4_9: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i
        let s_5_0: i128 = 0;
        // D s_5_1: read-var result:u128
        let s_5_1: u128 = fn_state.result;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 128u16);
        // D s_5_3: read-var ga#255038:i64
        let s_5_3: i64 = fn_state.ga_255038;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: read-var ga#255039:bv
        let s_5_5: Bits = fn_state.ga_255039;
        // D s_5_6: call Elem_set(s_5_2, s_5_0, s_5_4, s_5_5)
        let s_5_6: Bits = Elem_set(state, tracer, s_5_2, s_5_0, s_5_4, s_5_5);
        // D s_5_7: cast reint s_5_6 -> u128
        let s_5_7: u128 = (s_5_6.value() as u128);
        // D s_5_8: write-var result <= s_5_7
        fn_state.result = s_5_7;
        // N s_5_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #128s : i64
        let s_6_0: i64 = 128;
        // D s_6_1: read-var d:i64
        let s_6_1: i64 = fn_state.d;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: read-var result:u128
        let s_6_3: u128 = fn_state.result;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 128u16);
        // D s_6_5: call V_set(s_6_2, s_6_0, s_6_4)
        let s_6_5: () = V_set(state, tracer, s_6_2, s_6_0, s_6_4);
        // N s_6_6: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1u : u32
        let s_7_0: u32 = 1;
        // D s_7_1: read-var operation:u32
        let s_7_1: u32 = fn_state.operation;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b10 b8
        if s_7_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esizeshadow#1372:i64
        let s_8_0: i64 = fn_state.esizeshadow_1372;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: write-var ga#255040 <= s_8_2
        fn_state.ga_255040 = s_8_2;
        // D s_8_4: read-var operand1:bv
        let s_8_4: Bits = fn_state.operand1;
        // D s_8_5: read-var operand2:bv
        let s_8_5: Bits = fn_state.operand2;
        // D s_8_6: read-var fpcr:struct
        let s_8_6: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_8_7: call FPMin(s_8_4, s_8_5, s_8_6)
        let s_8_7: Bits = FPMin(state, tracer, s_8_4, s_8_5, s_8_6);
        // D s_8_8: write-var ga#255041 <= s_8_7
        fn_state.ga_255041 = s_8_7;
        // N s_8_9: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0s : i
        let s_9_0: i128 = 0;
        // D s_9_1: read-var result:u128
        let s_9_1: u128 = fn_state.result;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 128u16);
        // D s_9_3: read-var ga#255040:i64
        let s_9_3: i64 = fn_state.ga_255040;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: read-var ga#255041:bv
        let s_9_5: Bits = fn_state.ga_255041;
        // D s_9_6: call Elem_set(s_9_2, s_9_0, s_9_4, s_9_5)
        let s_9_6: Bits = Elem_set(state, tracer, s_9_2, s_9_0, s_9_4, s_9_5);
        // D s_9_7: cast reint s_9_6 -> u128
        let s_9_7: u128 = (s_9_6.value() as u128);
        // D s_9_8: write-var result <= s_9_7
        fn_state.result = s_9_7;
        // N s_9_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #2u : u32
        let s_10_0: u32 = 2;
        // D s_10_1: read-var operation:u32
        let s_10_1: u32 = fn_state.operation;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: not s_10_2
        let s_10_3: bool = !s_10_2;
        // N s_10_4: branch s_10_3 b13 b11
        if s_10_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#1372:i64
        let s_11_0: i64 = fn_state.esizeshadow_1372;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // D s_11_3: write-var ga#255042 <= s_11_2
        fn_state.ga_255042 = s_11_2;
        // D s_11_4: read-var operand1:bv
        let s_11_4: Bits = fn_state.operand1;
        // D s_11_5: read-var operand2:bv
        let s_11_5: Bits = fn_state.operand2;
        // D s_11_6: read-var fpcr:struct
        let s_11_6: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_11_7: call FPMaxNum(s_11_4, s_11_5, s_11_6)
        let s_11_7: Bits = FPMaxNum(state, tracer, s_11_4, s_11_5, s_11_6);
        // D s_11_8: write-var ga#255043 <= s_11_7
        fn_state.ga_255043 = s_11_7;
        // N s_11_9: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var result:u128
        let s_12_1: u128 = fn_state.result;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 128u16);
        // D s_12_3: read-var ga#255042:i64
        let s_12_3: i64 = fn_state.ga_255042;
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: read-var ga#255043:bv
        let s_12_5: Bits = fn_state.ga_255043;
        // D s_12_6: call Elem_set(s_12_2, s_12_0, s_12_4, s_12_5)
        let s_12_6: Bits = Elem_set(state, tracer, s_12_2, s_12_0, s_12_4, s_12_5);
        // D s_12_7: cast reint s_12_6 -> u128
        let s_12_7: u128 = (s_12_6.value() as u128);
        // D s_12_8: write-var result <= s_12_7
        fn_state.result = s_12_7;
        // N s_12_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #3u : u32
        let s_13_0: u32 = 3;
        // D s_13_1: read-var operation:u32
        let s_13_1: u32 = fn_state.operation;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b16 b14
        if s_13_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var esizeshadow#1372:i64
        let s_14_0: i64 = fn_state.esizeshadow_1372;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: cast reint s_14_1 -> i64
        let s_14_2: i64 = (s_14_1 as i64);
        // D s_14_3: write-var ga#255044 <= s_14_2
        fn_state.ga_255044 = s_14_2;
        // D s_14_4: read-var operand1:bv
        let s_14_4: Bits = fn_state.operand1;
        // D s_14_5: read-var operand2:bv
        let s_14_5: Bits = fn_state.operand2;
        // D s_14_6: read-var fpcr:struct
        let s_14_6: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_14_7: call FPMinNum(s_14_4, s_14_5, s_14_6)
        let s_14_7: Bits = FPMinNum(state, tracer, s_14_4, s_14_5, s_14_6);
        // D s_14_8: write-var ga#255045 <= s_14_7
        fn_state.ga_255045 = s_14_7;
        // N s_14_9: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0s : i
        let s_15_0: i128 = 0;
        // D s_15_1: read-var result:u128
        let s_15_1: u128 = fn_state.result;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 128u16);
        // D s_15_3: read-var ga#255044:i64
        let s_15_3: i64 = fn_state.ga_255044;
        // D s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_5: read-var ga#255045:bv
        let s_15_5: Bits = fn_state.ga_255045;
        // D s_15_6: call Elem_set(s_15_2, s_15_0, s_15_4, s_15_5)
        let s_15_6: Bits = Elem_set(state, tracer, s_15_2, s_15_0, s_15_4, s_15_5);
        // D s_15_7: cast reint s_15_6 -> u128
        let s_15_7: u128 = (s_15_6.value() as u128);
        // D s_15_8: write-var result <= s_15_7
        fn_state.result = s_15_7;
        // N s_15_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #128s : i64
        let s_17_0: i64 = 128;
        // D s_17_1: read-var n:i64
        let s_17_1: i64 = fn_state.n;
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // D s_17_3: call V_read(s_17_2, s_17_0)
        let s_17_3: Bits = V_read(state, tracer, s_17_2, s_17_0);
        // D s_17_4: cast reint s_17_3 -> u128
        let s_17_4: u128 = (s_17_3.value() as u128);
        // D s_17_5: write-var result <= s_17_4
        fn_state.result = s_17_4;
        // N s_17_6: jump b3
        return block_3(state, tracer, fn_state);
    }
}
