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
use D_set::*;
use Elem_set::*;
use Zeros::*;
use CheckAdvSIMDEnabled::*;
use Elem_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VTBL_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    is_vtbl: bool,
    length: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        table2: u64,
        table1: u64,
        table: u64,
        d: i128,
        table3: u64,
        i: i64,
        indexshadow_7876: i128,
        d__arg: i64,
        is_vtbl: bool,
        length: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d__arg,
        is_vtbl,
        length,
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
        // D s_0_0: read-var d__arg:i64
        let s_0_0: i64 = fn_state.d__arg;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: write-var d <= s_0_1
        fn_state.d = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call CheckAdvSIMDEnabled(s_0_3)
        let s_0_4: () = CheckAdvSIMDEnabled(state, tracer, s_0_3);
        // N s_0_5: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #4s : i
        let s_1_0: i128 = 4;
        // D s_1_1: read-var length:i64
        let s_1_1: i64 = fn_state.length;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: cmp-eq s_1_2 s_1_0
        let s_1_3: bool = ((s_1_2) == (s_1_0));
        // N s_1_4: branch s_1_3 b19 b2
        if s_1_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i
        let s_2_0: i128 = 64;
        // S s_2_1: call Zeros(s_2_0)
        let s_2_1: Bits = Zeros(state, tracer, s_2_0);
        // S s_2_2: cast reint s_2_1 -> u64
        let s_2_2: u64 = (s_2_1.value() as u64);
        // D s_2_3: write-var table3 <= s_2_2
        fn_state.table3 = s_2_2;
        // N s_2_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #3s : i
        let s_3_0: i128 = 3;
        // D s_3_1: read-var length:i64
        let s_3_1: i64 = fn_state.length;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-ge s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) >= (s_3_0));
        // N s_3_4: branch s_3_3 b18 b4
        if s_3_3 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #64s : i
        let s_4_0: i128 = 64;
        // S s_4_1: call Zeros(s_4_0)
        let s_4_1: Bits = Zeros(state, tracer, s_4_0);
        // S s_4_2: cast reint s_4_1 -> u64
        let s_4_2: u64 = (s_4_1.value() as u64);
        // D s_4_3: write-var table2 <= s_4_2
        fn_state.table2 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2s : i
        let s_5_0: i128 = 2;
        // D s_5_1: read-var length:i64
        let s_5_1: i64 = fn_state.length;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cmp-ge s_5_2 s_5_0
        let s_5_3: bool = ((s_5_2) >= (s_5_0));
        // N s_5_4: branch s_5_3 b17 b6
        if s_5_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i
        let s_6_0: i128 = 64;
        // S s_6_1: call Zeros(s_6_0)
        let s_6_1: Bits = Zeros(state, tracer, s_6_0);
        // S s_6_2: cast reint s_6_1 -> u64
        let s_6_2: u64 = (s_6_1.value() as u64);
        // D s_6_3: write-var table1 <= s_6_2
        fn_state.table1 = s_6_2;
        // N s_6_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var table3:u64
        let s_7_0: u64 = fn_state.table3;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 64u16);
        // D s_7_2: read-var table2:u64
        let s_7_2: u64 = fn_state.table2;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 64u16);
        // D s_7_4: cast reint s_7_1 -> u128
        let s_7_4: u128 = (s_7_1.value() as u128);
        // D s_7_5: size-of s_7_1
        let s_7_5: u16 = s_7_1.length();
        // D s_7_6: cast reint s_7_3 -> u128
        let s_7_6: u128 = (s_7_3.value() as u128);
        // D s_7_7: size-of s_7_3
        let s_7_7: u16 = s_7_3.length();
        // D s_7_8: lsl s_7_4 s_7_7
        let s_7_8: u128 = s_7_4 << s_7_7;
        // D s_7_9: or s_7_8 s_7_6
        let s_7_9: u128 = ((s_7_8) | (s_7_6));
        // D s_7_10: add s_7_5 s_7_7
        let s_7_10: u16 = (s_7_5 + s_7_7);
        // D s_7_11: create-bits s_7_9 s_7_10
        let s_7_11: Bits = Bits::new(s_7_9, s_7_10);
        // D s_7_12: cast reint s_7_11 -> u128
        let s_7_12: u128 = (s_7_11.value() as u128);
        // D s_7_13: cast zx s_7_12 -> bv
        let s_7_13: Bits = Bits::new(s_7_12 as u128, 128u16);
        // D s_7_14: read-var table1:u64
        let s_7_14: u64 = fn_state.table1;
        // D s_7_15: cast zx s_7_14 -> bv
        let s_7_15: Bits = Bits::new(s_7_14 as u128, 64u16);
        // D s_7_16: cast reint s_7_13 -> u128
        let s_7_16: u128 = (s_7_13.value() as u128);
        // D s_7_17: size-of s_7_13
        let s_7_17: u16 = s_7_13.length();
        // D s_7_18: cast reint s_7_15 -> u128
        let s_7_18: u128 = (s_7_15.value() as u128);
        // D s_7_19: size-of s_7_15
        let s_7_19: u16 = s_7_15.length();
        // D s_7_20: lsl s_7_16 s_7_19
        let s_7_20: u128 = s_7_16 << s_7_19;
        // D s_7_21: or s_7_20 s_7_18
        let s_7_21: u128 = ((s_7_20) | (s_7_18));
        // D s_7_22: add s_7_17 s_7_19
        let s_7_22: u16 = (s_7_17 + s_7_19);
        // D s_7_23: create-bits s_7_21 s_7_22
        let s_7_23: Bits = Bits::new(s_7_21, s_7_22);
        // D s_7_24: cast reint s_7_23 -> u192
        let s_7_24: u64 = (s_7_23.value() as u64);
        // D s_7_25: read-var n:i64
        let s_7_25: i64 = fn_state.n;
        // D s_7_26: cast zx s_7_25 -> i
        let s_7_26: i128 = (i128::try_from(s_7_25).unwrap());
        // D s_7_27: call D_read(s_7_26)
        let s_7_27: u64 = D_read(state, tracer, s_7_26);
        // D s_7_28: cast zx s_7_24 -> bv
        let s_7_28: Bits = Bits::new(s_7_24 as u128, 192u16);
        // D s_7_29: cast zx s_7_27 -> bv
        let s_7_29: Bits = Bits::new(s_7_27 as u128, 64u16);
        // D s_7_30: cast reint s_7_28 -> u128
        let s_7_30: u128 = (s_7_28.value() as u128);
        // D s_7_31: size-of s_7_28
        let s_7_31: u16 = s_7_28.length();
        // D s_7_32: cast reint s_7_29 -> u128
        let s_7_32: u128 = (s_7_29.value() as u128);
        // D s_7_33: size-of s_7_29
        let s_7_33: u16 = s_7_29.length();
        // D s_7_34: lsl s_7_30 s_7_33
        let s_7_34: u128 = s_7_30 << s_7_33;
        // D s_7_35: or s_7_34 s_7_32
        let s_7_35: u128 = ((s_7_34) | (s_7_32));
        // D s_7_36: add s_7_31 s_7_33
        let s_7_36: u16 = (s_7_31 + s_7_33);
        // D s_7_37: create-bits s_7_35 s_7_36
        let s_7_37: Bits = Bits::new(s_7_35, s_7_36);
        // D s_7_38: cast reint s_7_37 -> u256
        let s_7_38: u64 = (s_7_37.value() as u64);
        // D s_7_39: write-var table <= s_7_38
        fn_state.table = s_7_38;
        // C s_7_40: const #0s : i64
        let s_7_40: i64 = 0;
        // D s_7_41: write-var i <= s_7_40
        fn_state.i = s_7_40;
        // N s_7_42: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var i:i64
        let s_8_0: i64 = fn_state.i;
        // C s_8_1: const #7s : i64
        let s_8_1: i64 = 7;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b16 b9
        if s_8_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var m:i64
        let s_9_0: i64 = fn_state.m;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call D_read(s_9_1)
        let s_9_2: u64 = D_read(state, tracer, s_9_1);
        // C s_9_3: const #8s : i64
        let s_9_3: i64 = 8;
        // D s_9_4: cast zx s_9_2 -> bv
        let s_9_4: Bits = Bits::new(s_9_2 as u128, 64u16);
        // D s_9_5: read-var i:i64
        let s_9_5: i64 = fn_state.i;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // C s_9_7: cast zx s_9_3 -> i
        let s_9_7: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_8: call Elem_read(s_9_4, s_9_6, s_9_7)
        let s_9_8: Bits = Elem_read(state, tracer, s_9_4, s_9_6, s_9_7);
        // D s_9_9: cast reint s_9_8 -> u8
        let s_9_9: u8 = (s_9_8.value() as u8);
        // D s_9_10: cast zx s_9_9 -> bv
        let s_9_10: Bits = Bits::new(s_9_9 as u128, 8u16);
        // D s_9_11: cast zx s_9_10 -> i
        let s_9_11: i128 = (s_9_10.value() as i128);
        // D s_9_12: write-var indexshadow#7876 <= s_9_11
        fn_state.indexshadow_7876 = s_9_11;
        // C s_9_13: const #8s : i
        let s_9_13: i128 = 8;
        // D s_9_14: read-var length:i64
        let s_9_14: i64 = fn_state.length;
        // D s_9_15: cast zx s_9_14 -> i
        let s_9_15: i128 = (i128::try_from(s_9_14).unwrap());
        // D s_9_16: mul s_9_13 s_9_15
        let s_9_16: i128 = ((s_9_13) * (s_9_15));
        // D s_9_17: cast reint s_9_16 -> i64
        let s_9_17: i64 = (s_9_16 as i64);
        // D s_9_18: cast zx s_9_17 -> i
        let s_9_18: i128 = (i128::try_from(s_9_17).unwrap());
        // D s_9_19: read-var indexshadow#7876:i
        let s_9_19: i128 = fn_state.indexshadow_7876;
        // D s_9_20: cmp-lt s_9_19 s_9_18
        let s_9_20: bool = ((s_9_19) < (s_9_18));
        // N s_9_21: branch s_9_20 b15 b10
        if s_9_20 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var is_vtbl:u8
        let s_10_0: bool = fn_state.is_vtbl;
        // N s_10_1: branch s_10_0 b14 b11
        if s_10_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var i:i64
        let s_13_0: i64 = fn_state.i;
        // C s_13_1: const #1s : i64
        let s_13_1: i64 = 1;
        // D s_13_2: add s_13_0 s_13_1
        let s_13_2: i64 = (s_13_0 + s_13_1);
        // D s_13_3: write-var i <= s_13_2
        fn_state.i = s_13_2;
        // N s_13_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var d:i
        let s_14_0: i128 = fn_state.d;
        // D s_14_1: call D_read(s_14_0)
        let s_14_1: u64 = D_read(state, tracer, s_14_0);
        // C s_14_2: const #8s : i64
        let s_14_2: i64 = 8;
        // C s_14_3: const #8s : i
        let s_14_3: i128 = 8;
        // S s_14_4: call Zeros(s_14_3)
        let s_14_4: Bits = Zeros(state, tracer, s_14_3);
        // S s_14_5: cast reint s_14_4 -> u8
        let s_14_5: u8 = (s_14_4.value() as u8);
        // D s_14_6: cast zx s_14_1 -> bv
        let s_14_6: Bits = Bits::new(s_14_1 as u128, 64u16);
        // D s_14_7: read-var i:i64
        let s_14_7: i64 = fn_state.i;
        // D s_14_8: cast zx s_14_7 -> i
        let s_14_8: i128 = (i128::try_from(s_14_7).unwrap());
        // C s_14_9: cast zx s_14_2 -> i
        let s_14_9: i128 = (i128::try_from(s_14_2).unwrap());
        // S s_14_10: cast zx s_14_5 -> bv
        let s_14_10: Bits = Bits::new(s_14_5 as u128, 8u16);
        // D s_14_11: call Elem_set(s_14_6, s_14_8, s_14_9, s_14_10)
        let s_14_11: Bits = Elem_set(state, tracer, s_14_6, s_14_8, s_14_9, s_14_10);
        // D s_14_12: cast reint s_14_11 -> u64
        let s_14_12: u64 = (s_14_11.value() as u64);
        // D s_14_13: read-var d:i
        let s_14_13: i128 = fn_state.d;
        // D s_14_14: call D_set(s_14_13, s_14_12)
        let s_14_14: () = D_set(state, tracer, s_14_13, s_14_12);
        // N s_14_15: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var d:i
        let s_15_0: i128 = fn_state.d;
        // D s_15_1: call D_read(s_15_0)
        let s_15_1: u64 = D_read(state, tracer, s_15_0);
        // C s_15_2: const #8s : i64
        let s_15_2: i64 = 8;
        // C s_15_3: const #8s : i64
        let s_15_3: i64 = 8;
        // D s_15_4: read-var table:u256
        let s_15_4: u64 = fn_state.table;
        // D s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 256u16);
        // C s_15_6: cast zx s_15_3 -> i
        let s_15_6: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_7: read-var indexshadow#7876:i
        let s_15_7: i128 = fn_state.indexshadow_7876;
        // D s_15_8: call Elem_read(s_15_5, s_15_7, s_15_6)
        let s_15_8: Bits = Elem_read(state, tracer, s_15_5, s_15_7, s_15_6);
        // D s_15_9: cast reint s_15_8 -> u8
        let s_15_9: u8 = (s_15_8.value() as u8);
        // D s_15_10: cast zx s_15_1 -> bv
        let s_15_10: Bits = Bits::new(s_15_1 as u128, 64u16);
        // D s_15_11: read-var i:i64
        let s_15_11: i64 = fn_state.i;
        // D s_15_12: cast zx s_15_11 -> i
        let s_15_12: i128 = (i128::try_from(s_15_11).unwrap());
        // C s_15_13: cast zx s_15_2 -> i
        let s_15_13: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_14: cast zx s_15_9 -> bv
        let s_15_14: Bits = Bits::new(s_15_9 as u128, 8u16);
        // D s_15_15: call Elem_set(s_15_10, s_15_12, s_15_13, s_15_14)
        let s_15_15: Bits = Elem_set(state, tracer, s_15_10, s_15_12, s_15_13, s_15_14);
        // D s_15_16: cast reint s_15_15 -> u64
        let s_15_16: u64 = (s_15_15.value() as u64);
        // D s_15_17: read-var d:i
        let s_15_17: i128 = fn_state.d;
        // D s_15_18: call D_set(s_15_17, s_15_16)
        let s_15_18: () = D_set(state, tracer, s_15_17, s_15_16);
        // N s_15_19: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #1s : i
        let s_17_0: i128 = 1;
        // D s_17_1: read-var n:i64
        let s_17_1: i64 = fn_state.n;
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // D s_17_3: add s_17_2 s_17_0
        let s_17_3: i128 = (s_17_2 + s_17_0);
        // D s_17_4: cast reint s_17_3 -> i64
        let s_17_4: i64 = (s_17_3 as i64);
        // D s_17_5: cast zx s_17_4 -> i
        let s_17_5: i128 = (i128::try_from(s_17_4).unwrap());
        // D s_17_6: call D_read(s_17_5)
        let s_17_6: u64 = D_read(state, tracer, s_17_5);
        // D s_17_7: write-var table1 <= s_17_6
        fn_state.table1 = s_17_6;
        // N s_17_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #2s : i
        let s_18_0: i128 = 2;
        // D s_18_1: read-var n:i64
        let s_18_1: i64 = fn_state.n;
        // D s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // D s_18_3: add s_18_2 s_18_0
        let s_18_3: i128 = (s_18_2 + s_18_0);
        // D s_18_4: cast reint s_18_3 -> i64
        let s_18_4: i64 = (s_18_3 as i64);
        // D s_18_5: cast zx s_18_4 -> i
        let s_18_5: i128 = (i128::try_from(s_18_4).unwrap());
        // D s_18_6: call D_read(s_18_5)
        let s_18_6: u64 = D_read(state, tracer, s_18_5);
        // D s_18_7: write-var table2 <= s_18_6
        fn_state.table2 = s_18_6;
        // N s_18_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #3s : i
        let s_19_0: i128 = 3;
        // D s_19_1: read-var n:i64
        let s_19_1: i64 = fn_state.n;
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (i128::try_from(s_19_1).unwrap());
        // D s_19_3: add s_19_2 s_19_0
        let s_19_3: i128 = (s_19_2 + s_19_0);
        // D s_19_4: cast reint s_19_3 -> i64
        let s_19_4: i64 = (s_19_3 as i64);
        // D s_19_5: cast zx s_19_4 -> i
        let s_19_5: i128 = (i128::try_from(s_19_4).unwrap());
        // D s_19_6: call D_read(s_19_5)
        let s_19_6: u64 = D_read(state, tracer, s_19_5);
        // D s_19_7: write-var table3 <= s_19_6
        fn_state.table3 = s_19_6;
        // N s_19_8: jump b3
        return block_3(state, tracer, fn_state);
    }
}
