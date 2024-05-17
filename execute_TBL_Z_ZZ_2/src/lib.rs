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
use Elem_set::*;
use Elem_read::*;
use CheckSVEEnabled::*;
use Z_read::*;
use Zeros::*;
use Z_set::*;
use common::*;
pub fn execute_TBL_Z_ZZ_2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    double_table: bool,
    esize: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_195183: i64,
        e: i64,
        indexes: Bits,
        ga_283642: i64,
        table_elems: i64,
        esizeshadow_2898: i64,
        VLshadow_2899: i64,
        ga_283656: i64,
        idx: i128,
        table: Bits,
        VLshadow_2900: i64,
        elements: i64,
        ga_283657: Bits,
        result: Bits,
        table_size: i64,
        VL: i64,
        d: i64,
        double_table: bool,
        esize: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        double_table,
        esize,
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
        // D s_0_0: read-var esize:i64
        let s_0_0: i64 = fn_state.esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var esizeshadow#2898 <= s_0_2
        fn_state.esizeshadow_2898 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#2899 <= s_0_6
        fn_state.VLshadow_2899 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSVEEnabled(s_0_8)
        let s_0_9: () = CheckSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#2899:i64
        let s_1_0: i64 = fn_state.VLshadow_2899;
        // D s_1_1: write-var VLshadow#2900 <= s_1_0
        fn_state.VLshadow_2900 = s_1_0;
        // D s_1_2: read-var VLshadow#2900:i64
        let s_1_2: i64 = fn_state.VLshadow_2900;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#2898:i64
        let s_1_4: i64 = fn_state.esizeshadow_2898;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: write-var elements <= s_1_7
        fn_state.elements = s_1_7;
        // D s_1_9: read-var VLshadow#2900:i64
        let s_1_9: i64 = fn_state.VLshadow_2900;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: read-var m:i64
        let s_1_12: i64 = fn_state.m;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast zx s_1_11 -> i
        let s_1_14: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_15: call Z_read(s_1_13, s_1_14)
        let s_1_15: Bits = Z_read(state, tracer, s_1_13, s_1_14);
        // D s_1_16: write-var indexes <= s_1_15
        fn_state.indexes = s_1_15;
        // D s_1_17: read-var double_table:u8
        let s_1_17: bool = fn_state.double_table;
        // N s_1_18: branch s_1_17 b13 b2
        if s_1_17 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var VLshadow#2900:i64
        let s_2_0: i64 = fn_state.VLshadow_2900;
        // D s_2_1: write-var ga#283642 <= s_2_0
        fn_state.ga_283642 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#283642:i64
        let s_3_0: i64 = fn_state.ga_283642;
        // D s_3_1: write-var table_size <= s_3_0
        fn_state.table_size = s_3_0;
        // D s_3_2: read-var table_size:i64
        let s_3_2: i64 = fn_state.table_size;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-var esizeshadow#2898:i64
        let s_3_4: i64 = fn_state.esizeshadow_2898;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: div s_3_3 s_3_5
        let s_3_6: i128 = ((s_3_3) / (s_3_5));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: write-var table_elems <= s_3_7
        fn_state.table_elems = s_3_7;
        // D s_3_9: read-var double_table:u8
        let s_3_9: bool = fn_state.double_table;
        // N s_3_10: branch s_3_9 b12 b4
        if s_3_9 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var table_size:i64
        let s_4_0: i64 = fn_state.table_size;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var n:i64
        let s_4_3: i64 = fn_state.n;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: cast zx s_4_2 -> i
        let s_4_5: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_6: call Z_read(s_4_4, s_4_5)
        let s_4_6: Bits = Z_read(state, tracer, s_4_4, s_4_5);
        // D s_4_7: write-var table <= s_4_6
        fn_state.table = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var elements:i64
        let s_5_2: i64 = fn_state.elements;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: sub s_5_3 s_5_1
        let s_5_4: i128 = ((s_5_3) - (s_5_1));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: write-var gs#195183 <= s_5_5
        fn_state.gs_195183 = s_5_5;
        // D s_5_7: write-var e <= s_5_0
        fn_state.e = s_5_0;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#195183:i64
        let s_6_1: i64 = fn_state.gs_195183;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b11 b7
        if s_6_2 {
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
        // D s_7_0: read-var esizeshadow#2898:i64
        let s_7_0: i64 = fn_state.esizeshadow_2898;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var e:i64
        let s_7_3: i64 = fn_state.e;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast zx s_7_2 -> i
        let s_7_5: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_6: read-var indexes:bv
        let s_7_6: Bits = fn_state.indexes;
        // D s_7_7: call Elem_read(s_7_6, s_7_4, s_7_5)
        let s_7_7: Bits = Elem_read(state, tracer, s_7_6, s_7_4, s_7_5);
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (s_7_7.value() as i128);
        // D s_7_9: write-var idx <= s_7_8
        fn_state.idx = s_7_8;
        // D s_7_10: read-var esizeshadow#2898:i64
        let s_7_10: i64 = fn_state.esizeshadow_2898;
        // D s_7_11: cast zx s_7_10 -> i
        let s_7_11: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_12: cast reint s_7_11 -> i64
        let s_7_12: i64 = (s_7_11 as i64);
        // D s_7_13: write-var ga#283656 <= s_7_12
        fn_state.ga_283656 = s_7_12;
        // D s_7_14: read-var table_elems:i64
        let s_7_14: i64 = fn_state.table_elems;
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: read-var idx:i
        let s_7_16: i128 = fn_state.idx;
        // D s_7_17: cmp-lt s_7_16 s_7_15
        let s_7_17: bool = ((s_7_16) < (s_7_15));
        // N s_7_18: branch s_7_17 b10 b8
        if s_7_17 {
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
        // D s_8_0: read-var esizeshadow#2898:i64
        let s_8_0: i64 = fn_state.esizeshadow_2898;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call Zeros(s_8_1)
        let s_8_2: Bits = Zeros(state, tracer, s_8_1);
        // D s_8_3: write-var ga#283657 <= s_8_2
        fn_state.ga_283657 = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var ga#283656:i64
        let s_9_2: i64 = fn_state.ga_283656;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var result:bv
        let s_9_4: Bits = fn_state.result;
        // D s_9_5: read-var ga#283657:bv
        let s_9_5: Bits = fn_state.ga_283657;
        // D s_9_6: call Elem_set(s_9_4, s_9_1, s_9_3, s_9_5)
        let s_9_6: Bits = Elem_set(state, tracer, s_9_4, s_9_1, s_9_3, s_9_5);
        // D s_9_7: write-var result <= s_9_6
        fn_state.result = s_9_6;
        // D s_9_8: read-var e:i64
        let s_9_8: i64 = fn_state.e;
        // C s_9_9: const #1s : i64
        let s_9_9: i64 = 1;
        // D s_9_10: add s_9_8 s_9_9
        let s_9_10: i64 = (s_9_8 + s_9_9);
        // D s_9_11: write-var e <= s_9_10
        fn_state.e = s_9_10;
        // N s_9_12: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esizeshadow#2898:i64
        let s_10_0: i64 = fn_state.esizeshadow_2898;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: read-var table:bv
        let s_10_4: Bits = fn_state.table;
        // D s_10_5: read-var idx:i
        let s_10_5: i128 = fn_state.idx;
        // D s_10_6: call Elem_read(s_10_4, s_10_5, s_10_3)
        let s_10_6: Bits = Elem_read(state, tracer, s_10_4, s_10_5, s_10_3);
        // D s_10_7: write-var ga#283657 <= s_10_6
        fn_state.ga_283657 = s_10_6;
        // N s_10_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var VLshadow#2900:i64
        let s_11_0: i64 = fn_state.VLshadow_2900;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // D s_11_3: read-var d:i64
        let s_11_3: i64 = fn_state.d;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: cast zx s_11_2 -> i
        let s_11_5: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_6: read-var result:bv
        let s_11_6: Bits = fn_state.result;
        // D s_11_7: call Z_set(s_11_4, s_11_5, s_11_6)
        let s_11_7: () = Z_set(state, tracer, s_11_4, s_11_5, s_11_6);
        // N s_11_8: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1s : i
        let s_12_0: i128 = 1;
        // D s_12_1: read-var n:i64
        let s_12_1: i64 = fn_state.n;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: add s_12_2 s_12_0
        let s_12_3: i128 = (s_12_2 + s_12_0);
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // C s_12_5: const #32s : i
        let s_12_5: i128 = 32;
        // D s_12_6: cast zx s_12_4 -> i
        let s_12_6: i128 = (i128::try_from(s_12_4).unwrap());
        // D s_12_7: mod s_12_6 s_12_5
        let s_12_7: i128 = ((s_12_6) % (s_12_5));
        // D s_12_8: cast reint s_12_7 -> i64
        let s_12_8: i64 = (s_12_7 as i64);
        // D s_12_9: read-var VLshadow#2900:i64
        let s_12_9: i64 = fn_state.VLshadow_2900;
        // D s_12_10: cast zx s_12_9 -> i
        let s_12_10: i128 = (i128::try_from(s_12_9).unwrap());
        // D s_12_11: cast reint s_12_10 -> i64
        let s_12_11: i64 = (s_12_10 as i64);
        // D s_12_12: cast zx s_12_8 -> i
        let s_12_12: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_13: cast zx s_12_11 -> i
        let s_12_13: i128 = (i128::try_from(s_12_11).unwrap());
        // D s_12_14: call Z_read(s_12_12, s_12_13)
        let s_12_14: Bits = Z_read(state, tracer, s_12_12, s_12_13);
        // D s_12_15: read-var VLshadow#2900:i64
        let s_12_15: i64 = fn_state.VLshadow_2900;
        // D s_12_16: cast zx s_12_15 -> i
        let s_12_16: i128 = (i128::try_from(s_12_15).unwrap());
        // D s_12_17: cast reint s_12_16 -> i64
        let s_12_17: i64 = (s_12_16 as i64);
        // D s_12_18: read-var n:i64
        let s_12_18: i64 = fn_state.n;
        // D s_12_19: cast zx s_12_18 -> i
        let s_12_19: i128 = (i128::try_from(s_12_18).unwrap());
        // D s_12_20: cast zx s_12_17 -> i
        let s_12_20: i128 = (i128::try_from(s_12_17).unwrap());
        // D s_12_21: call Z_read(s_12_19, s_12_20)
        let s_12_21: Bits = Z_read(state, tracer, s_12_19, s_12_20);
        // D s_12_22: cast reint s_12_14 -> u128
        let s_12_22: u128 = (s_12_14.value() as u128);
        // D s_12_23: size-of s_12_14
        let s_12_23: u16 = s_12_14.length();
        // D s_12_24: cast reint s_12_21 -> u128
        let s_12_24: u128 = (s_12_21.value() as u128);
        // D s_12_25: size-of s_12_21
        let s_12_25: u16 = s_12_21.length();
        // D s_12_26: lsl s_12_22 s_12_25
        let s_12_26: u128 = s_12_22 << s_12_25;
        // D s_12_27: or s_12_26 s_12_24
        let s_12_27: u128 = ((s_12_26) | (s_12_24));
        // D s_12_28: add s_12_23 s_12_25
        let s_12_28: u16 = (s_12_23 + s_12_25);
        // D s_12_29: create-bits s_12_27 s_12_28
        let s_12_29: Bits = Bits::new(s_12_27, s_12_28);
        // C s_12_30: const #1s : i
        let s_12_30: i128 = 1;
        // D s_12_31: read-var table_size:i64
        let s_12_31: i64 = fn_state.table_size;
        // D s_12_32: cast zx s_12_31 -> i
        let s_12_32: i128 = (i128::try_from(s_12_31).unwrap());
        // D s_12_33: sub s_12_32 s_12_30
        let s_12_33: i128 = ((s_12_32) - (s_12_30));
        // D s_12_34: cast reint s_12_33 -> i64
        let s_12_34: i64 = (s_12_33 as i64);
        // C s_12_35: const #0s : i
        let s_12_35: i128 = 0;
        // D s_12_36: cast zx s_12_34 -> i
        let s_12_36: i128 = (i128::try_from(s_12_34).unwrap());
        // C s_12_37: const #1s : i64
        let s_12_37: i64 = 1;
        // C s_12_38: cast zx s_12_37 -> i
        let s_12_38: i128 = (i128::try_from(s_12_37).unwrap());
        // D s_12_39: sub s_12_36 s_12_35
        let s_12_39: i128 = ((s_12_36) - (s_12_35));
        // D s_12_40: add s_12_39 s_12_38
        let s_12_40: i128 = (s_12_39 + s_12_38);
        // D s_12_41: bit-extract s_12_29 s_12_35 s_12_40
        let s_12_41: Bits = (Bits::new(
            ((s_12_29) >> (s_12_35)).value(),
            u16::try_from(s_12_40).unwrap(),
        ));
        // D s_12_42: write-var table <= s_12_41
        fn_state.table = s_12_41;
        // N s_12_43: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #2s : i
        let s_13_0: i128 = 2;
        // D s_13_1: read-var VLshadow#2900:i64
        let s_13_1: i64 = fn_state.VLshadow_2900;
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // D s_13_3: mul s_13_2 s_13_0
        let s_13_3: i128 = ((s_13_2) * (s_13_0));
        // D s_13_4: cast reint s_13_3 -> i64
        let s_13_4: i64 = (s_13_3 as i64);
        // D s_13_5: write-var ga#283642 <= s_13_4
        fn_state.ga_283642 = s_13_4;
        // N s_13_6: jump b3
        return block_3(state, tracer, fn_state);
    }
}
