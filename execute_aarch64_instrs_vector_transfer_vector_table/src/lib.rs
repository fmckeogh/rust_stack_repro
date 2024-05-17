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
use V_read::*;
use V_set::*;
use fmod_int::*;
use Zeros::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_transfer_vector_table<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i64,
    is_tbl: bool,
    m: i64,
    n__arg: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_173899: i64,
        indices: Bits,
        n: i128,
        table: Bits,
        result: Bits,
        i: i64,
        indexshadow_1996: i128,
        u_2157: i64,
        datasizeshadow_1995: i64,
        gs_173884: i64,
        d: i64,
        datasize: i64,
        elements: i64,
        is_tbl: bool,
        m: i64,
        n__arg: i64,
        regs: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        is_tbl,
        m,
        n__arg,
        regs,
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
        // D s_0_3: write-var datasizeshadow#1995 <= s_0_2
        fn_state.datasizeshadow_1995 = s_0_2;
        // D s_0_4: read-var n__arg:i64
        let s_0_4: i64 = fn_state.n__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var n <= s_0_5
        fn_state.n = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckFPAdvSIMDEnabled64(s_0_7)
        let s_0_8: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_7);
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1995:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1995;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var m:i64
        let s_1_3: i64 = fn_state.m;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var indices <= s_1_5
        fn_state.indices = s_1_5;
        // C s_1_7: const #128s : i
        let s_1_7: i128 = 128;
        // D s_1_8: read-var regs:i64
        let s_1_8: i64 = fn_state.regs;
        // D s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_10: mul s_1_7 s_1_9
        let s_1_10: i128 = ((s_1_7) * (s_1_9));
        // D s_1_11: cast reint s_1_10 -> i64
        let s_1_11: i64 = (s_1_10 as i64);
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call Zeros(s_1_12)
        let s_1_13: Bits = Zeros(state, tracer, s_1_12);
        // D s_1_14: write-var table <= s_1_13
        fn_state.table = s_1_13;
        // C s_1_15: const #0s : i64
        let s_1_15: i64 = 0;
        // C s_1_16: const #1s : i
        let s_1_16: i128 = 1;
        // D s_1_17: read-var regs:i64
        let s_1_17: i64 = fn_state.regs;
        // D s_1_18: cast zx s_1_17 -> i
        let s_1_18: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_19: sub s_1_18 s_1_16
        let s_1_19: i128 = ((s_1_18) - (s_1_16));
        // D s_1_20: cast reint s_1_19 -> i64
        let s_1_20: i64 = (s_1_19 as i64);
        // D s_1_21: write-var gs#173884 <= s_1_20
        fn_state.gs_173884 = s_1_20;
        // D s_1_22: write-var i <= s_1_15
        fn_state.i = s_1_15;
        // N s_1_23: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var i:i64
        let s_2_0: i64 = fn_state.i;
        // D s_2_1: read-var gs#173884:i64
        let s_2_1: i64 = fn_state.gs_173884;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b4 b3
        if s_2_2 {
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
        // C s_3_0: const #128s : i
        let s_3_0: i128 = 128;
        // D s_3_1: read-var i:i64
        let s_3_1: i64 = fn_state.i;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_0 s_3_2
        let s_3_3: i128 = ((s_3_0) * (s_3_2));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // C s_3_5: const #127s : i
        let s_3_5: i128 = 127;
        // D s_3_6: cast zx s_3_4 -> i
        let s_3_6: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_7: add s_3_6 s_3_5
        let s_3_7: i128 = (s_3_6 + s_3_5);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // C s_3_9: const #128s : i
        let s_3_9: i128 = 128;
        // D s_3_10: read-var i:i64
        let s_3_10: i64 = fn_state.i;
        // D s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_12: mul s_3_9 s_3_11
        let s_3_12: i128 = ((s_3_9) * (s_3_11));
        // D s_3_13: cast reint s_3_12 -> i64
        let s_3_13: i64 = (s_3_12 as i64);
        // C s_3_14: const #128s : i64
        let s_3_14: i64 = 128;
        // D s_3_15: read-var n:i
        let s_3_15: i128 = fn_state.n;
        // D s_3_16: call V_read(s_3_15, s_3_14)
        let s_3_16: Bits = V_read(state, tracer, s_3_15, s_3_14);
        // D s_3_17: cast zx s_3_8 -> i
        let s_3_17: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_18: cast zx s_3_13 -> i
        let s_3_18: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_19: read-var table:bv
        let s_3_19: Bits = fn_state.table;
        // D s_3_20: sub s_3_17 s_3_18
        let s_3_20: i128 = ((s_3_17) - (s_3_18));
        // C s_3_21: const #1u : u64
        let s_3_21: u64 = 1;
        // C s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 64u16);
        // D s_3_23: lsl s_3_22 s_3_20
        let s_3_23: Bits = s_3_22 << s_3_20;
        // D s_3_24: sub s_3_23 s_3_22
        let s_3_24: Bits = ((s_3_23) - (s_3_22));
        // D s_3_25: and s_3_16 s_3_24
        let s_3_25: Bits = ((s_3_16) & (s_3_24));
        // D s_3_26: lsl s_3_25 s_3_18
        let s_3_26: Bits = s_3_25 << s_3_18;
        // D s_3_27: lsl s_3_24 s_3_18
        let s_3_27: Bits = s_3_24 << s_3_18;
        // D s_3_28: cmpl s_3_27
        let s_3_28: Bits = !s_3_27;
        // D s_3_29: and s_3_19 s_3_28
        let s_3_29: Bits = ((s_3_19) & (s_3_28));
        // D s_3_30: or s_3_29 s_3_26
        let s_3_30: Bits = ((s_3_29) | (s_3_26));
        // D s_3_31: write-var table <= s_3_30
        fn_state.table = s_3_30;
        // C s_3_32: const #1s : i
        let s_3_32: i128 = 1;
        // D s_3_33: read-var n:i
        let s_3_33: i128 = fn_state.n;
        // D s_3_34: add s_3_33 s_3_32
        let s_3_34: i128 = (s_3_33 + s_3_32);
        // C s_3_35: const #32s : i
        let s_3_35: i128 = 32;
        // D s_3_36: call fmod_int(s_3_34, s_3_35)
        let s_3_36: i128 = fmod_int(state, tracer, s_3_34, s_3_35);
        // D s_3_37: write-var n <= s_3_36
        fn_state.n = s_3_36;
        // D s_3_38: read-var i:i64
        let s_3_38: i64 = fn_state.i;
        // C s_3_39: const #1s : i64
        let s_3_39: i64 = 1;
        // D s_3_40: add s_3_38 s_3_39
        let s_3_40: i64 = (s_3_38 + s_3_39);
        // D s_3_41: write-var i <= s_3_40
        fn_state.i = s_3_40;
        // N s_3_42: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var is_tbl:u8
        let s_4_0: bool = fn_state.is_tbl;
        // N s_4_1: branch s_4_0 b13 b5
        if s_4_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var datasizeshadow#1995:i64
        let s_5_0: i64 = fn_state.datasizeshadow_1995;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var d:i64
        let s_5_3: i64 = fn_state.d;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: call V_read(s_5_4, s_5_2)
        let s_5_5: Bits = V_read(state, tracer, s_5_4, s_5_2);
        // D s_5_6: write-var result <= s_5_5
        fn_state.result = s_5_5;
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0s : i64
        let s_6_0: i64 = 0;
        // C s_6_1: const #1s : i
        let s_6_1: i128 = 1;
        // D s_6_2: read-var elements:i64
        let s_6_2: i64 = fn_state.elements;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: sub s_6_3 s_6_1
        let s_6_4: i128 = ((s_6_3) - (s_6_1));
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // D s_6_6: write-var gs#173899 <= s_6_5
        fn_state.gs_173899 = s_6_5;
        // D s_6_7: write-var u#2157 <= s_6_0
        fn_state.u_2157 = s_6_0;
        // N s_6_8: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var u#2157:i64
        let s_7_0: i64 = fn_state.u_2157;
        // D s_7_1: read-var gs#173899:i64
        let s_7_1: i64 = fn_state.gs_173899;
        // D s_7_2: cmp-gt s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) > (s_7_1));
        // N s_7_3: branch s_7_2 b12 b8
        if s_7_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #8s : i64
        let s_8_0: i64 = 8;
        // D s_8_1: read-var u#2157:i64
        let s_8_1: i64 = fn_state.u_2157;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // C s_8_3: cast zx s_8_0 -> i
        let s_8_3: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_4: read-var indices:bv
        let s_8_4: Bits = fn_state.indices;
        // D s_8_5: call Elem_read(s_8_4, s_8_2, s_8_3)
        let s_8_5: Bits = Elem_read(state, tracer, s_8_4, s_8_2, s_8_3);
        // D s_8_6: cast reint s_8_5 -> u8
        let s_8_6: u8 = (s_8_5.value() as u8);
        // D s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 8u16);
        // D s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (s_8_7.value() as i128);
        // D s_8_9: write-var indexshadow#1996 <= s_8_8
        fn_state.indexshadow_1996 = s_8_8;
        // C s_8_10: const #16s : i
        let s_8_10: i128 = 16;
        // D s_8_11: read-var regs:i64
        let s_8_11: i64 = fn_state.regs;
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: mul s_8_10 s_8_12
        let s_8_13: i128 = ((s_8_10) * (s_8_12));
        // D s_8_14: cast reint s_8_13 -> i64
        let s_8_14: i64 = (s_8_13 as i64);
        // D s_8_15: cast zx s_8_14 -> i
        let s_8_15: i128 = (i128::try_from(s_8_14).unwrap());
        // D s_8_16: read-var indexshadow#1996:i
        let s_8_16: i128 = fn_state.indexshadow_1996;
        // D s_8_17: cmp-lt s_8_16 s_8_15
        let s_8_17: bool = ((s_8_16) < (s_8_15));
        // N s_8_18: branch s_8_17 b11 b9
        if s_8_17 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var u#2157:i64
        let s_10_0: i64 = fn_state.u_2157;
        // C s_10_1: const #1s : i64
        let s_10_1: i64 = 1;
        // D s_10_2: add s_10_0 s_10_1
        let s_10_2: i64 = (s_10_0 + s_10_1);
        // D s_10_3: write-var u#2157 <= s_10_2
        fn_state.u_2157 = s_10_2;
        // N s_10_4: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #8s : i64
        let s_11_0: i64 = 8;
        // C s_11_1: const #8s : i64
        let s_11_1: i64 = 8;
        // C s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: read-var table:bv
        let s_11_3: Bits = fn_state.table;
        // D s_11_4: read-var indexshadow#1996:i
        let s_11_4: i128 = fn_state.indexshadow_1996;
        // D s_11_5: call Elem_read(s_11_3, s_11_4, s_11_2)
        let s_11_5: Bits = Elem_read(state, tracer, s_11_3, s_11_4, s_11_2);
        // D s_11_6: cast reint s_11_5 -> u8
        let s_11_6: u8 = (s_11_5.value() as u8);
        // D s_11_7: read-var u#2157:i64
        let s_11_7: i64 = fn_state.u_2157;
        // D s_11_8: cast zx s_11_7 -> i
        let s_11_8: i128 = (i128::try_from(s_11_7).unwrap());
        // C s_11_9: cast zx s_11_0 -> i
        let s_11_9: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_10: cast zx s_11_6 -> bv
        let s_11_10: Bits = Bits::new(s_11_6 as u128, 8u16);
        // D s_11_11: read-var result:bv
        let s_11_11: Bits = fn_state.result;
        // D s_11_12: call Elem_set(s_11_11, s_11_8, s_11_9, s_11_10)
        let s_11_12: Bits = Elem_set(state, tracer, s_11_11, s_11_8, s_11_9, s_11_10);
        // D s_11_13: write-var result <= s_11_12
        fn_state.result = s_11_12;
        // N s_11_14: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var datasizeshadow#1995:i64
        let s_12_0: i64 = fn_state.datasizeshadow_1995;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: cast reint s_12_1 -> i64
        let s_12_2: i64 = (s_12_1 as i64);
        // D s_12_3: read-var d:i64
        let s_12_3: i64 = fn_state.d;
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: read-var result:bv
        let s_12_5: Bits = fn_state.result;
        // D s_12_6: call V_set(s_12_4, s_12_2, s_12_5)
        let s_12_6: () = V_set(state, tracer, s_12_4, s_12_2, s_12_5);
        // N s_12_7: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var datasizeshadow#1995:i64
        let s_13_0: i64 = fn_state.datasizeshadow_1995;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call Zeros(s_13_1)
        let s_13_2: Bits = Zeros(state, tracer, s_13_1);
        // D s_13_3: write-var result <= s_13_2
        fn_state.result = s_13_2;
        // N s_13_4: jump b6
        return block_6(state, tracer, fn_state);
    }
}
