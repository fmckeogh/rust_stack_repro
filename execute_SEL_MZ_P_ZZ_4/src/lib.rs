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
use CheckStreamingSVEEnabled::*;
use P_read::*;
use ActivePredicateElement::*;
use CounterToPredicate::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_SEL_MZ_P_ZZ_4<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    g: i64,
    m: i64,
    n: i64,
    nreg: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        gs_288871: i64,
        gs_288886: i64,
        VLshadow_6813: i64,
        mask: Bits,
        u_9054: i64,
        gs_288877: i64,
        elements: i64,
        VLshadow_6814: i64,
        operand1: Bits,
        results: [Bits; 4usize],
        esizeshadow_6812: i64,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        g: i64,
        m: i64,
        n: i64,
        nreg: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        g,
        m,
        n,
        nreg,
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
        // D s_0_3: write-var esizeshadow#6812 <= s_0_2
        fn_state.esizeshadow_6812 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6813 <= s_0_6
        fn_state.VLshadow_6813 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckStreamingSVEEnabled(s_0_8)
        let s_0_9: () = CheckStreamingSVEEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#6813:i64
        let s_1_0: i64 = fn_state.VLshadow_6813;
        // D s_1_1: write-var VLshadow#6814 <= s_1_0
        fn_state.VLshadow_6814 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#6814:i64
        let s_1_3: i64 = fn_state.VLshadow_6814;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#6814:i64
        let s_1_7: i64 = fn_state.VLshadow_6814;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esizeshadow#6812:i64
        let s_1_9: i64 = fn_state.esizeshadow_6812;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: div s_1_8 s_1_10
        let s_1_11: i128 = ((s_1_8) / (s_1_10));
        // D s_1_12: cast reint s_1_11 -> i64
        let s_1_12: i64 = (s_1_11 as i64);
        // D s_1_13: write-var elements <= s_1_12
        fn_state.elements = s_1_12;
        // D s_1_14: cast zx s_1_6 -> i
        let s_1_14: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_15: cast reint s_1_14 -> i64
        let s_1_15: i64 = (s_1_14 as i64);
        // D s_1_16: read-var g:i64
        let s_1_16: i64 = fn_state.g;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: cast zx s_1_15 -> i
        let s_1_18: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_19: call P_read(s_1_17, s_1_18)
        let s_1_19: Bits = P_read(state, tracer, s_1_17, s_1_18);
        // C s_1_20: const #0s : i
        let s_1_20: i128 = 0;
        // C s_1_21: const #1s : i64
        let s_1_21: i64 = 1;
        // C s_1_22: cast zx s_1_21 -> i
        let s_1_22: i128 = (i128::try_from(s_1_21).unwrap());
        // C s_1_23: const #15s : i
        let s_1_23: i128 = 15;
        // C s_1_24: add s_1_23 s_1_22
        let s_1_24: i128 = (s_1_23 + s_1_22);
        // D s_1_25: bit-extract s_1_19 s_1_20 s_1_24
        let s_1_25: Bits = (Bits::new(
            ((s_1_19) >> (s_1_20)).value(),
            u16::try_from(s_1_24).unwrap(),
        ));
        // D s_1_26: cast reint s_1_25 -> u16
        let s_1_26: u16 = (s_1_25.value() as u16);
        // D s_1_27: cast zx s_1_6 -> i
        let s_1_27: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_28: read-var nreg:i64
        let s_1_28: i64 = fn_state.nreg;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: mul s_1_27 s_1_29
        let s_1_30: i128 = ((s_1_27) * (s_1_29));
        // D s_1_31: cast reint s_1_30 -> i64
        let s_1_31: i64 = (s_1_30 as i64);
        // D s_1_32: cast zx s_1_31 -> i
        let s_1_32: i128 = (i128::try_from(s_1_31).unwrap());
        // D s_1_33: cast reint s_1_32 -> i64
        let s_1_33: i64 = (s_1_32 as i64);
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: call CounterToPredicate(s_1_26, s_1_34)
        let s_1_35: Bits = CounterToPredicate(state, tracer, s_1_26, s_1_34);
        // D s_1_36: write-var mask <= s_1_35
        fn_state.mask = s_1_35;
        // C s_1_37: const #0s : i64
        let s_1_37: i64 = 0;
        // C s_1_38: const #1s : i
        let s_1_38: i128 = 1;
        // D s_1_39: read-var nreg:i64
        let s_1_39: i64 = fn_state.nreg;
        // D s_1_40: cast zx s_1_39 -> i
        let s_1_40: i128 = (i128::try_from(s_1_39).unwrap());
        // D s_1_41: sub s_1_40 s_1_38
        let s_1_41: i128 = ((s_1_40) - (s_1_38));
        // D s_1_42: cast reint s_1_41 -> i64
        let s_1_42: i64 = (s_1_41 as i64);
        // D s_1_43: write-var gs#288871 <= s_1_42
        fn_state.gs_288871 = s_1_42;
        // D s_1_44: write-var r <= s_1_37
        fn_state.r = s_1_37;
        // N s_1_45: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#288871:i64
        let s_2_1: i64 = fn_state.gs_288871;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
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
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var r:i64
        let s_3_2: i64 = fn_state.r;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: add s_3_1 s_3_3
        let s_3_4: i128 = (s_3_1 + s_3_3);
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: read-var VLshadow#6814:i64
        let s_3_6: i64 = fn_state.VLshadow_6814;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: cast zx s_3_5 -> i
        let s_3_9: i128 = (i128::try_from(s_3_5).unwrap());
        // D s_3_10: cast zx s_3_8 -> i
        let s_3_10: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_11: call Z_read(s_3_9, s_3_10)
        let s_3_11: Bits = Z_read(state, tracer, s_3_9, s_3_10);
        // D s_3_12: write-var operand1 <= s_3_11
        fn_state.operand1 = s_3_11;
        // D s_3_13: read-var m:i64
        let s_3_13: i64 = fn_state.m;
        // D s_3_14: cast zx s_3_13 -> i
        let s_3_14: i128 = (i128::try_from(s_3_13).unwrap());
        // D s_3_15: read-var r:i64
        let s_3_15: i64 = fn_state.r;
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (i128::try_from(s_3_15).unwrap());
        // D s_3_17: add s_3_14 s_3_16
        let s_3_17: i128 = (s_3_14 + s_3_16);
        // D s_3_18: cast reint s_3_17 -> i64
        let s_3_18: i64 = (s_3_17 as i64);
        // D s_3_19: read-var VLshadow#6814:i64
        let s_3_19: i64 = fn_state.VLshadow_6814;
        // D s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: cast zx s_3_18 -> i
        let s_3_22: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_23: cast zx s_3_21 -> i
        let s_3_23: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_24: call Z_read(s_3_22, s_3_23)
        let s_3_24: Bits = Z_read(state, tracer, s_3_22, s_3_23);
        // D s_3_25: write-var operand2 <= s_3_24
        fn_state.operand2 = s_3_24;
        // C s_3_26: const #0s : i64
        let s_3_26: i64 = 0;
        // C s_3_27: const #1s : i
        let s_3_27: i128 = 1;
        // D s_3_28: read-var elements:i64
        let s_3_28: i64 = fn_state.elements;
        // D s_3_29: cast zx s_3_28 -> i
        let s_3_29: i128 = (i128::try_from(s_3_28).unwrap());
        // D s_3_30: sub s_3_29 s_3_27
        let s_3_30: i128 = ((s_3_29) - (s_3_27));
        // D s_3_31: cast reint s_3_30 -> i64
        let s_3_31: i64 = (s_3_30 as i64);
        // D s_3_32: write-var gs#288877 <= s_3_31
        fn_state.gs_288877 = s_3_31;
        // D s_3_33: write-var e <= s_3_26
        fn_state.e = s_3_26;
        // N s_3_34: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#288877:i64
        let s_4_1: i64 = fn_state.gs_288877;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
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
        // D s_5_0: read-var r:i64
        let s_5_0: i64 = fn_state.r;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var elements:i64
        let s_5_2: i64 = fn_state.elements;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: mul s_5_1 s_5_3
        let s_5_4: i128 = ((s_5_1) * (s_5_3));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: read-var e:i64
        let s_5_7: i64 = fn_state.e;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: add s_5_6 s_5_8
        let s_5_9: i128 = (s_5_6 + s_5_8);
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: read-var esizeshadow#6812:i64
        let s_5_12: i64 = fn_state.esizeshadow_6812;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: read-var mask:bv
        let s_5_14: Bits = fn_state.mask;
        // D s_5_15: call ActivePredicateElement(s_5_14, s_5_11, s_5_13)
        let s_5_15: bool = ActivePredicateElement(state, tracer, s_5_14, s_5_11, s_5_13);
        // N s_5_16: branch s_5_15 b8 b6
        if s_5_15 {
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
        // D s_6_0: read-var results:[bv; 4]
        let s_6_0: [Bits; 4usize] = fn_state.results;
        // D s_6_1: cast cvt s_6_0 -> [bv; 0]
        let s_6_1: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_6_0);
        // D s_6_2: read-var r:i64
        let s_6_2: i64 = fn_state.r;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: read-element s_6_1[s_6_3]
        let s_6_4: Bits = s_6_1[(s_6_3) as usize];
        // D s_6_5: read-var esizeshadow#6812:i64
        let s_6_5: i64 = fn_state.esizeshadow_6812;
        // D s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_7: cast reint s_6_6 -> i64
        let s_6_7: i64 = (s_6_6 as i64);
        // D s_6_8: read-var esizeshadow#6812:i64
        let s_6_8: i64 = fn_state.esizeshadow_6812;
        // D s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_10: cast reint s_6_9 -> i64
        let s_6_10: i64 = (s_6_9 as i64);
        // D s_6_11: read-var e:i64
        let s_6_11: i64 = fn_state.e;
        // D s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_13: cast zx s_6_10 -> i
        let s_6_13: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_14: read-var operand2:bv
        let s_6_14: Bits = fn_state.operand2;
        // D s_6_15: call Elem_read(s_6_14, s_6_12, s_6_13)
        let s_6_15: Bits = Elem_read(state, tracer, s_6_14, s_6_12, s_6_13);
        // D s_6_16: read-var e:i64
        let s_6_16: i64 = fn_state.e;
        // D s_6_17: cast zx s_6_16 -> i
        let s_6_17: i128 = (i128::try_from(s_6_16).unwrap());
        // D s_6_18: cast zx s_6_7 -> i
        let s_6_18: i128 = (i128::try_from(s_6_7).unwrap());
        // D s_6_19: call Elem_set(s_6_4, s_6_17, s_6_18, s_6_15)
        let s_6_19: Bits = Elem_set(state, tracer, s_6_4, s_6_17, s_6_18, s_6_15);
        // D s_6_20: read-var results:[bv; 4]
        let s_6_20: [Bits; 4usize] = fn_state.results;
        // D s_6_21: cast cvt s_6_20 -> [bv; 0]
        let s_6_21: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_6_20);
        // D s_6_22: read-var r:i64
        let s_6_22: i64 = fn_state.r;
        // D s_6_23: cast zx s_6_22 -> i
        let s_6_23: i128 = (i128::try_from(s_6_22).unwrap());
        // D s_6_24: mutate-element s_6_21[s_6_23] <= s_6_19
        let s_6_24: alloc::vec::Vec<Bits> = {
            let mut local = s_6_21.clone();
            local[(s_6_23) as usize] = s_6_19;
            local
        };
        // D s_6_25: cast cvt s_6_24 -> [bv; 4]
        let s_6_25: [Bits; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_24);
            buf
        };
        // D s_6_26: write-var results <= s_6_25
        fn_state.results = s_6_25;
        // N s_6_27: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var e:i64
        let s_7_0: i64 = fn_state.e;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var e <= s_7_2
        fn_state.e = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var results:[bv; 4]
        let s_8_0: [Bits; 4usize] = fn_state.results;
        // D s_8_1: cast cvt s_8_0 -> [bv; 0]
        let s_8_1: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_8_0);
        // D s_8_2: read-var r:i64
        let s_8_2: i64 = fn_state.r;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: read-element s_8_1[s_8_3]
        let s_8_4: Bits = s_8_1[(s_8_3) as usize];
        // D s_8_5: read-var esizeshadow#6812:i64
        let s_8_5: i64 = fn_state.esizeshadow_6812;
        // D s_8_6: cast zx s_8_5 -> i
        let s_8_6: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_7: cast reint s_8_6 -> i64
        let s_8_7: i64 = (s_8_6 as i64);
        // D s_8_8: read-var esizeshadow#6812:i64
        let s_8_8: i64 = fn_state.esizeshadow_6812;
        // D s_8_9: cast zx s_8_8 -> i
        let s_8_9: i128 = (i128::try_from(s_8_8).unwrap());
        // D s_8_10: cast reint s_8_9 -> i64
        let s_8_10: i64 = (s_8_9 as i64);
        // D s_8_11: read-var e:i64
        let s_8_11: i64 = fn_state.e;
        // D s_8_12: cast zx s_8_11 -> i
        let s_8_12: i128 = (i128::try_from(s_8_11).unwrap());
        // D s_8_13: cast zx s_8_10 -> i
        let s_8_13: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_14: read-var operand1:bv
        let s_8_14: Bits = fn_state.operand1;
        // D s_8_15: call Elem_read(s_8_14, s_8_12, s_8_13)
        let s_8_15: Bits = Elem_read(state, tracer, s_8_14, s_8_12, s_8_13);
        // D s_8_16: read-var e:i64
        let s_8_16: i64 = fn_state.e;
        // D s_8_17: cast zx s_8_16 -> i
        let s_8_17: i128 = (i128::try_from(s_8_16).unwrap());
        // D s_8_18: cast zx s_8_7 -> i
        let s_8_18: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_19: call Elem_set(s_8_4, s_8_17, s_8_18, s_8_15)
        let s_8_19: Bits = Elem_set(state, tracer, s_8_4, s_8_17, s_8_18, s_8_15);
        // D s_8_20: read-var results:[bv; 4]
        let s_8_20: [Bits; 4usize] = fn_state.results;
        // D s_8_21: cast cvt s_8_20 -> [bv; 0]
        let s_8_21: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_8_20);
        // D s_8_22: read-var r:i64
        let s_8_22: i64 = fn_state.r;
        // D s_8_23: cast zx s_8_22 -> i
        let s_8_23: i128 = (i128::try_from(s_8_22).unwrap());
        // D s_8_24: mutate-element s_8_21[s_8_23] <= s_8_19
        let s_8_24: alloc::vec::Vec<Bits> = {
            let mut local = s_8_21.clone();
            local[(s_8_23) as usize] = s_8_19;
            local
        };
        // D s_8_25: cast cvt s_8_24 -> [bv; 4]
        let s_8_25: [Bits; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_8_24);
            buf
        };
        // D s_8_26: write-var results <= s_8_25
        fn_state.results = s_8_25;
        // N s_8_27: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var r:i64
        let s_9_0: i64 = fn_state.r;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var r <= s_9_2
        fn_state.r = s_9_2;
        // N s_9_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0s : i64
        let s_10_0: i64 = 0;
        // C s_10_1: const #1s : i
        let s_10_1: i128 = 1;
        // D s_10_2: read-var nreg:i64
        let s_10_2: i64 = fn_state.nreg;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: sub s_10_3 s_10_1
        let s_10_4: i128 = ((s_10_3) - (s_10_1));
        // D s_10_5: cast reint s_10_4 -> i64
        let s_10_5: i64 = (s_10_4 as i64);
        // D s_10_6: write-var gs#288886 <= s_10_5
        fn_state.gs_288886 = s_10_5;
        // D s_10_7: write-var u#9054 <= s_10_0
        fn_state.u_9054 = s_10_0;
        // N s_10_8: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var u#9054:i64
        let s_11_0: i64 = fn_state.u_9054;
        // D s_11_1: read-var gs#288886:i64
        let s_11_1: i64 = fn_state.gs_288886;
        // D s_11_2: cmp-gt s_11_0 s_11_1
        let s_11_2: bool = ((s_11_0) > (s_11_1));
        // N s_11_3: branch s_11_2 b13 b12
        if s_11_2 {
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
        // D s_12_0: read-var d:i64
        let s_12_0: i64 = fn_state.d;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var u#9054:i64
        let s_12_2: i64 = fn_state.u_9054;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: add s_12_1 s_12_3
        let s_12_4: i128 = (s_12_1 + s_12_3);
        // D s_12_5: cast reint s_12_4 -> i64
        let s_12_5: i64 = (s_12_4 as i64);
        // D s_12_6: read-var VLshadow#6814:i64
        let s_12_6: i64 = fn_state.VLshadow_6814;
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_8: cast reint s_12_7 -> i64
        let s_12_8: i64 = (s_12_7 as i64);
        // D s_12_9: read-var results:[bv; 4]
        let s_12_9: [Bits; 4usize] = fn_state.results;
        // D s_12_10: cast cvt s_12_9 -> [bv; 0]
        let s_12_10: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_12_9);
        // D s_12_11: read-var u#9054:i64
        let s_12_11: i64 = fn_state.u_9054;
        // D s_12_12: cast zx s_12_11 -> i
        let s_12_12: i128 = (i128::try_from(s_12_11).unwrap());
        // D s_12_13: read-element s_12_10[s_12_12]
        let s_12_13: Bits = s_12_10[(s_12_12) as usize];
        // D s_12_14: cast zx s_12_5 -> i
        let s_12_14: i128 = (i128::try_from(s_12_5).unwrap());
        // D s_12_15: cast zx s_12_8 -> i
        let s_12_15: i128 = (i128::try_from(s_12_8).unwrap());
        // D s_12_16: call Z_set(s_12_14, s_12_15, s_12_13)
        let s_12_16: () = Z_set(state, tracer, s_12_14, s_12_15, s_12_13);
        // D s_12_17: read-var u#9054:i64
        let s_12_17: i64 = fn_state.u_9054;
        // C s_12_18: const #1s : i64
        let s_12_18: i64 = 1;
        // D s_12_19: add s_12_17 s_12_18
        let s_12_19: i64 = (s_12_17 + s_12_18);
        // D s_12_20: write-var u#9054 <= s_12_19
        fn_state.u_9054 = s_12_19;
        // N s_12_21: jump b11
        return block_11(state, tracer, fn_state);
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
