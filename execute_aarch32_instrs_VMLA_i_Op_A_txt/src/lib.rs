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
use Q_read::*;
use Elem_set::*;
use CheckAdvSIMDEnabled::*;
use Q_set::*;
use D_set::*;
use Din_read::*;
use asl_Int::*;
use Elem_read::*;
use D_read::*;
use Qin_read::*;
use common::*;
pub fn execute_aarch32_instrs_VMLA_i_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
    d__arg: i64,
    elements: i128,
    esize: i64,
    long_destination: bool,
    m: i64,
    n: i64,
    regs: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_312320: i64,
        r: i64,
        product: i128,
        e: i64,
        addend: i128,
        gs_312326: i64,
        d: i128,
        esizeshadow_7517: i64,
        add: bool,
        d__arg: i64,
        elements: i128,
        esize: i64,
        long_destination: bool,
        m: i64,
        n: i64,
        regs: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        add,
        d__arg,
        elements,
        esize,
        long_destination,
        m,
        n,
        regs,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#7517 <= s_0_2
        fn_state.esizeshadow_7517 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckAdvSIMDEnabled(s_0_7)
        let s_0_8: () = CheckAdvSIMDEnabled(state, tracer, s_0_7);
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#312320 <= s_1_5
        fn_state.gs_312320 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#312320:i64
        let s_2_1: i64 = fn_state.gs_312320;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b13 b3
        if s_2_2 {
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
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var elements:i
        let s_3_2: i128 = fn_state.elements;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#312326 <= s_3_4
        fn_state.gs_312326 = s_3_4;
        // D s_3_6: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#312326:i64
        let s_4_1: i64 = fn_state.gs_312326;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b12 b5
        if s_4_2 {
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
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var r:i64
        let s_5_2: i64 = fn_state.r;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: i128 = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: call Din_read(s_5_6)
        let s_5_7: u64 = Din_read(state, tracer, s_5_6);
        // D s_5_8: read-var esizeshadow#7517:i64
        let s_5_8: i64 = fn_state.esizeshadow_7517;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: cast zx s_5_7 -> bv
        let s_5_11: Bits = Bits::new(s_5_7 as u128, 64u16);
        // D s_5_12: read-var e:i64
        let s_5_12: i64 = fn_state.e;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: cast zx s_5_10 -> i
        let s_5_14: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_15: call Elem_read(s_5_11, s_5_13, s_5_14)
        let s_5_15: Bits = Elem_read(state, tracer, s_5_11, s_5_13, s_5_14);
        // D s_5_16: read-var is_unsigned:u8
        let s_5_16: bool = fn_state.is_unsigned;
        // D s_5_17: call asl_Int(s_5_15, s_5_16)
        let s_5_17: i128 = asl_Int(state, tracer, s_5_15, s_5_16);
        // D s_5_18: read-var m:i64
        let s_5_18: i64 = fn_state.m;
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: read-var r:i64
        let s_5_20: i64 = fn_state.r;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: add s_5_19 s_5_21
        let s_5_22: i128 = (s_5_19 + s_5_21);
        // D s_5_23: cast reint s_5_22 -> i64
        let s_5_23: i64 = (s_5_22 as i64);
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: call Din_read(s_5_24)
        let s_5_25: u64 = Din_read(state, tracer, s_5_24);
        // D s_5_26: read-var esizeshadow#7517:i64
        let s_5_26: i64 = fn_state.esizeshadow_7517;
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_28: cast reint s_5_27 -> i64
        let s_5_28: i64 = (s_5_27 as i64);
        // D s_5_29: cast zx s_5_25 -> bv
        let s_5_29: Bits = Bits::new(s_5_25 as u128, 64u16);
        // D s_5_30: read-var e:i64
        let s_5_30: i64 = fn_state.e;
        // D s_5_31: cast zx s_5_30 -> i
        let s_5_31: i128 = (i128::try_from(s_5_30).unwrap());
        // D s_5_32: cast zx s_5_28 -> i
        let s_5_32: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_33: call Elem_read(s_5_29, s_5_31, s_5_32)
        let s_5_33: Bits = Elem_read(state, tracer, s_5_29, s_5_31, s_5_32);
        // D s_5_34: read-var is_unsigned:u8
        let s_5_34: bool = fn_state.is_unsigned;
        // D s_5_35: call asl_Int(s_5_33, s_5_34)
        let s_5_35: i128 = asl_Int(state, tracer, s_5_33, s_5_34);
        // D s_5_36: mul s_5_17 s_5_35
        let s_5_36: i128 = ((s_5_17) * (s_5_35));
        // D s_5_37: write-var product <= s_5_36
        fn_state.product = s_5_36;
        // D s_5_38: read-var add:u8
        let s_5_38: bool = fn_state.add;
        // N s_5_39: branch s_5_38 b11 b6
        if s_5_38 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var product:i
        let s_6_0: i128 = fn_state.product;
        // D s_6_1: neg s_6_0
        let s_6_1: i128 = -s_6_0;
        // D s_6_2: write-var addend <= s_6_1
        fn_state.addend = s_6_1;
        // N s_6_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var long_destination:u8
        let s_7_0: bool = fn_state.long_destination;
        // N s_7_1: branch s_7_0 b10 b8
        if s_7_0 {
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
        // D s_8_0: read-var r:i64
        let s_8_0: i64 = fn_state.r;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var d:i
        let s_8_2: i128 = fn_state.d;
        // D s_8_3: add s_8_2 s_8_1
        let s_8_3: i128 = (s_8_2 + s_8_1);
        // D s_8_4: read-var r:i64
        let s_8_4: i64 = fn_state.r;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-var d:i
        let s_8_6: i128 = fn_state.d;
        // D s_8_7: add s_8_6 s_8_5
        let s_8_7: i128 = (s_8_6 + s_8_5);
        // D s_8_8: call D_read(s_8_7)
        let s_8_8: u64 = D_read(state, tracer, s_8_7);
        // D s_8_9: read-var esizeshadow#7517:i64
        let s_8_9: i64 = fn_state.esizeshadow_7517;
        // D s_8_10: cast zx s_8_9 -> i
        let s_8_10: i128 = (i128::try_from(s_8_9).unwrap());
        // D s_8_11: cast reint s_8_10 -> i64
        let s_8_11: i64 = (s_8_10 as i64);
        // D s_8_12: read-var r:i64
        let s_8_12: i64 = fn_state.r;
        // D s_8_13: cast zx s_8_12 -> i
        let s_8_13: i128 = (i128::try_from(s_8_12).unwrap());
        // D s_8_14: read-var d:i
        let s_8_14: i128 = fn_state.d;
        // D s_8_15: add s_8_14 s_8_13
        let s_8_15: i128 = (s_8_14 + s_8_13);
        // D s_8_16: call Din_read(s_8_15)
        let s_8_16: u64 = Din_read(state, tracer, s_8_15);
        // D s_8_17: read-var esizeshadow#7517:i64
        let s_8_17: i64 = fn_state.esizeshadow_7517;
        // D s_8_18: cast zx s_8_17 -> i
        let s_8_18: i128 = (i128::try_from(s_8_17).unwrap());
        // D s_8_19: cast reint s_8_18 -> i64
        let s_8_19: i64 = (s_8_18 as i64);
        // D s_8_20: cast zx s_8_16 -> bv
        let s_8_20: Bits = Bits::new(s_8_16 as u128, 64u16);
        // D s_8_21: read-var e:i64
        let s_8_21: i64 = fn_state.e;
        // D s_8_22: cast zx s_8_21 -> i
        let s_8_22: i128 = (i128::try_from(s_8_21).unwrap());
        // D s_8_23: cast zx s_8_19 -> i
        let s_8_23: i128 = (i128::try_from(s_8_19).unwrap());
        // D s_8_24: call Elem_read(s_8_20, s_8_22, s_8_23)
        let s_8_24: Bits = Elem_read(state, tracer, s_8_20, s_8_22, s_8_23);
        // D s_8_25: read-var addend:i
        let s_8_25: i128 = fn_state.addend;
        // D s_8_26: cast cvt s_8_25 -> bv
        let s_8_26: Bits = Bits::new(s_8_25 as u128, 128);
        // D s_8_27: add s_8_24 s_8_26
        let s_8_27: Bits = (s_8_24 + s_8_26);
        // D s_8_28: cast zx s_8_8 -> bv
        let s_8_28: Bits = Bits::new(s_8_8 as u128, 64u16);
        // D s_8_29: read-var e:i64
        let s_8_29: i64 = fn_state.e;
        // D s_8_30: cast zx s_8_29 -> i
        let s_8_30: i128 = (i128::try_from(s_8_29).unwrap());
        // D s_8_31: cast zx s_8_11 -> i
        let s_8_31: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_32: call Elem_set(s_8_28, s_8_30, s_8_31, s_8_27)
        let s_8_32: Bits = Elem_set(state, tracer, s_8_28, s_8_30, s_8_31, s_8_27);
        // D s_8_33: cast reint s_8_32 -> u64
        let s_8_33: u64 = (s_8_32.value() as u64);
        // D s_8_34: call D_set(s_8_3, s_8_33)
        let s_8_34: () = D_set(state, tracer, s_8_3, s_8_33);
        // N s_8_35: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var e:i64
        let s_9_0: i64 = fn_state.e;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var e <= s_9_2
        fn_state.e = s_9_2;
        // N s_9_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #1s : i
        let s_10_0: i128 = 1;
        // D s_10_1: read-var d:i
        let s_10_1: i128 = fn_state.d;
        // D s_10_2: lsr s_10_1 s_10_0
        let s_10_2: i128 = s_10_1 >> s_10_0;
        // C s_10_3: const #1s : i
        let s_10_3: i128 = 1;
        // D s_10_4: read-var d:i
        let s_10_4: i128 = fn_state.d;
        // D s_10_5: lsr s_10_4 s_10_3
        let s_10_5: i128 = s_10_4 >> s_10_3;
        // D s_10_6: call Q_read(s_10_5)
        let s_10_6: u128 = Q_read(state, tracer, s_10_5);
        // C s_10_7: const #2s : i
        let s_10_7: i128 = 2;
        // D s_10_8: read-var esizeshadow#7517:i64
        let s_10_8: i64 = fn_state.esizeshadow_7517;
        // D s_10_9: cast zx s_10_8 -> i
        let s_10_9: i128 = (i128::try_from(s_10_8).unwrap());
        // D s_10_10: mul s_10_7 s_10_9
        let s_10_10: i128 = ((s_10_7) * (s_10_9));
        // D s_10_11: cast reint s_10_10 -> i64
        let s_10_11: i64 = (s_10_10 as i64);
        // D s_10_12: cast zx s_10_11 -> i
        let s_10_12: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_13: cast reint s_10_12 -> i64
        let s_10_13: i64 = (s_10_12 as i64);
        // C s_10_14: const #1s : i
        let s_10_14: i128 = 1;
        // D s_10_15: read-var d:i
        let s_10_15: i128 = fn_state.d;
        // D s_10_16: lsr s_10_15 s_10_14
        let s_10_16: i128 = s_10_15 >> s_10_14;
        // D s_10_17: call Qin_read(s_10_16)
        let s_10_17: u128 = Qin_read(state, tracer, s_10_16);
        // C s_10_18: const #2s : i
        let s_10_18: i128 = 2;
        // D s_10_19: read-var esizeshadow#7517:i64
        let s_10_19: i64 = fn_state.esizeshadow_7517;
        // D s_10_20: cast zx s_10_19 -> i
        let s_10_20: i128 = (i128::try_from(s_10_19).unwrap());
        // D s_10_21: mul s_10_18 s_10_20
        let s_10_21: i128 = ((s_10_18) * (s_10_20));
        // D s_10_22: cast reint s_10_21 -> i64
        let s_10_22: i64 = (s_10_21 as i64);
        // D s_10_23: cast zx s_10_22 -> i
        let s_10_23: i128 = (i128::try_from(s_10_22).unwrap());
        // D s_10_24: cast reint s_10_23 -> i64
        let s_10_24: i64 = (s_10_23 as i64);
        // D s_10_25: cast zx s_10_17 -> bv
        let s_10_25: Bits = Bits::new(s_10_17 as u128, 128u16);
        // D s_10_26: read-var e:i64
        let s_10_26: i64 = fn_state.e;
        // D s_10_27: cast zx s_10_26 -> i
        let s_10_27: i128 = (i128::try_from(s_10_26).unwrap());
        // D s_10_28: cast zx s_10_24 -> i
        let s_10_28: i128 = (i128::try_from(s_10_24).unwrap());
        // D s_10_29: call Elem_read(s_10_25, s_10_27, s_10_28)
        let s_10_29: Bits = Elem_read(state, tracer, s_10_25, s_10_27, s_10_28);
        // D s_10_30: read-var addend:i
        let s_10_30: i128 = fn_state.addend;
        // D s_10_31: cast cvt s_10_30 -> bv
        let s_10_31: Bits = Bits::new(s_10_30 as u128, 128);
        // D s_10_32: add s_10_29 s_10_31
        let s_10_32: Bits = (s_10_29 + s_10_31);
        // D s_10_33: cast zx s_10_6 -> bv
        let s_10_33: Bits = Bits::new(s_10_6 as u128, 128u16);
        // D s_10_34: read-var e:i64
        let s_10_34: i64 = fn_state.e;
        // D s_10_35: cast zx s_10_34 -> i
        let s_10_35: i128 = (i128::try_from(s_10_34).unwrap());
        // D s_10_36: cast zx s_10_13 -> i
        let s_10_36: i128 = (i128::try_from(s_10_13).unwrap());
        // D s_10_37: call Elem_set(s_10_33, s_10_35, s_10_36, s_10_32)
        let s_10_37: Bits = Elem_set(state, tracer, s_10_33, s_10_35, s_10_36, s_10_32);
        // D s_10_38: cast reint s_10_37 -> u128
        let s_10_38: u128 = (s_10_37.value() as u128);
        // D s_10_39: call Q_set(s_10_2, s_10_38)
        let s_10_39: () = Q_set(state, tracer, s_10_2, s_10_38);
        // N s_10_40: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var product:i
        let s_11_0: i128 = fn_state.product;
        // D s_11_1: write-var addend <= s_11_0
        fn_state.addend = s_11_0;
        // N s_11_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var r:i64
        let s_12_0: i64 = fn_state.r;
        // C s_12_1: const #1s : i64
        let s_12_1: i64 = 1;
        // D s_12_2: add s_12_0 s_12_1
        let s_12_2: i64 = (s_12_0 + s_12_1);
        // D s_12_3: write-var r <= s_12_2
        fn_state.r = s_12_2;
        // N s_12_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: return
        return;
    }
}
