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
use CheckStreamingSVEEnabled::*;
use Elem_read::*;
use Elem_set::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_UUNPK_MZ_Z_4<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    n: i64,
    nreg: i64,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        r: i64,
        e: i64,
        ga_337778: Bits,
        gs_286741: i64,
        ga_337780: Bits,
        u_8935: i64,
        ga_337781: i64,
        VLshadow_6730: i64,
        VLshadow_6731: i64,
        gs_286765: i64,
        gs_286753: i64,
        hsize: i64,
        elements: i64,
        element: Bits,
        i: i64,
        esizeshadow_6729: i64,
        ga_337779: i64,
        results: [Bits; 4usize],
        VL: i64,
        d: i64,
        esize: i64,
        n: i64,
        nreg: i64,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
        n,
        nreg,
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
        // D s_0_3: write-var esizeshadow#6729 <= s_0_2
        fn_state.esizeshadow_6729 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6730 <= s_0_6
        fn_state.VLshadow_6730 = s_0_6;
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
        // D s_1_0: read-var VLshadow#6730:i64
        let s_1_0: i64 = fn_state.VLshadow_6730;
        // D s_1_1: write-var VLshadow#6731 <= s_1_0
        fn_state.VLshadow_6731 = s_1_0;
        // D s_1_2: read-var VLshadow#6731:i64
        let s_1_2: i64 = fn_state.VLshadow_6731;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#6729:i64
        let s_1_4: i64 = fn_state.esizeshadow_6729;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: write-var elements <= s_1_7
        fn_state.elements = s_1_7;
        // C s_1_9: const #2s : i
        let s_1_9: i128 = 2;
        // D s_1_10: read-var esizeshadow#6729:i64
        let s_1_10: i64 = fn_state.esizeshadow_6729;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: div s_1_11 s_1_9
        let s_1_12: i128 = ((s_1_11) / (s_1_9));
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var hsize <= s_1_13
        fn_state.hsize = s_1_13;
        // C s_1_15: const #2s : i
        let s_1_15: i128 = 2;
        // D s_1_16: read-var nreg:i64
        let s_1_16: i64 = fn_state.nreg;
        // D s_1_17: cast zx s_1_16 -> i
        let s_1_17: i128 = (i128::try_from(s_1_16).unwrap());
        // D s_1_18: div s_1_17 s_1_15
        let s_1_18: i128 = ((s_1_17) / (s_1_15));
        // D s_1_19: cast reint s_1_18 -> i64
        let s_1_19: i64 = (s_1_18 as i64);
        // C s_1_20: const #0s : i64
        let s_1_20: i64 = 0;
        // C s_1_21: const #1s : i
        let s_1_21: i128 = 1;
        // D s_1_22: cast zx s_1_19 -> i
        let s_1_22: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_23: sub s_1_22 s_1_21
        let s_1_23: i128 = ((s_1_22) - (s_1_21));
        // D s_1_24: cast reint s_1_23 -> i64
        let s_1_24: i64 = (s_1_23 as i64);
        // D s_1_25: write-var gs#286741 <= s_1_24
        fn_state.gs_286741 = s_1_24;
        // D s_1_26: write-var r <= s_1_20
        fn_state.r = s_1_20;
        // N s_1_27: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#286741:i64
        let s_2_1: i64 = fn_state.gs_286741;
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
        // D s_3_6: read-var VLshadow#6731:i64
        let s_3_6: i64 = fn_state.VLshadow_6731;
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
        // D s_3_12: write-var operand <= s_3_11
        fn_state.operand = s_3_11;
        // C s_3_13: const #0s : i64
        let s_3_13: i64 = 0;
        // D s_3_14: write-var i <= s_3_13
        fn_state.i = s_3_13;
        // N s_3_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #1s : i64
        let s_4_1: i64 = 1;
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
        // D s_5_6: write-var gs#286753 <= s_5_5
        fn_state.gs_286753 = s_5_5;
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
        // D s_6_1: read-var gs#286753:i64
        let s_6_1: i64 = fn_state.gs_286753;
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
        // D s_7_0: read-var i:i64
        let s_7_0: i64 = fn_state.i;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: read-var elements:i64
        let s_7_2: i64 = fn_state.elements;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: mul s_7_1 s_7_3
        let s_7_4: i128 = ((s_7_1) * (s_7_3));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: read-var e:i64
        let s_7_7: i64 = fn_state.e;
        // D s_7_8: cast zx s_7_7 -> i
        let s_7_8: i128 = (i128::try_from(s_7_7).unwrap());
        // D s_7_9: add s_7_6 s_7_8
        let s_7_9: i128 = (s_7_6 + s_7_8);
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: read-var hsize:i64
        let s_7_11: i64 = fn_state.hsize;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: cast reint s_7_12 -> i64
        let s_7_13: i64 = (s_7_12 as i64);
        // D s_7_14: cast zx s_7_10 -> i
        let s_7_14: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_15: cast zx s_7_13 -> i
        let s_7_15: i128 = (i128::try_from(s_7_13).unwrap());
        // D s_7_16: read-var operand:bv
        let s_7_16: Bits = fn_state.operand;
        // D s_7_17: call Elem_read(s_7_16, s_7_14, s_7_15)
        let s_7_17: Bits = Elem_read(state, tracer, s_7_16, s_7_14, s_7_15);
        // D s_7_18: write-var element <= s_7_17
        fn_state.element = s_7_17;
        // C s_7_19: const #2s : i
        let s_7_19: i128 = 2;
        // D s_7_20: read-var r:i64
        let s_7_20: i64 = fn_state.r;
        // D s_7_21: cast zx s_7_20 -> i
        let s_7_21: i128 = (i128::try_from(s_7_20).unwrap());
        // D s_7_22: mul s_7_19 s_7_21
        let s_7_22: i128 = ((s_7_19) * (s_7_21));
        // D s_7_23: cast reint s_7_22 -> i64
        let s_7_23: i64 = (s_7_22 as i64);
        // D s_7_24: cast zx s_7_23 -> i
        let s_7_24: i128 = (i128::try_from(s_7_23).unwrap());
        // D s_7_25: read-var i:i64
        let s_7_25: i64 = fn_state.i;
        // D s_7_26: cast zx s_7_25 -> i
        let s_7_26: i128 = (i128::try_from(s_7_25).unwrap());
        // D s_7_27: add s_7_24 s_7_26
        let s_7_27: i128 = (s_7_24 + s_7_26);
        // D s_7_28: cast reint s_7_27 -> i64
        let s_7_28: i64 = (s_7_27 as i64);
        // D s_7_29: write-var ga#337781 <= s_7_28
        fn_state.ga_337781 = s_7_28;
        // C s_7_30: const #2s : i
        let s_7_30: i128 = 2;
        // D s_7_31: read-var r:i64
        let s_7_31: i64 = fn_state.r;
        // D s_7_32: cast zx s_7_31 -> i
        let s_7_32: i128 = (i128::try_from(s_7_31).unwrap());
        // D s_7_33: mul s_7_30 s_7_32
        let s_7_33: i128 = ((s_7_30) * (s_7_32));
        // D s_7_34: cast reint s_7_33 -> i64
        let s_7_34: i64 = (s_7_33 as i64);
        // D s_7_35: cast zx s_7_34 -> i
        let s_7_35: i128 = (i128::try_from(s_7_34).unwrap());
        // D s_7_36: read-var i:i64
        let s_7_36: i64 = fn_state.i;
        // D s_7_37: cast zx s_7_36 -> i
        let s_7_37: i128 = (i128::try_from(s_7_36).unwrap());
        // D s_7_38: add s_7_35 s_7_37
        let s_7_38: i128 = (s_7_35 + s_7_37);
        // D s_7_39: cast reint s_7_38 -> i64
        let s_7_39: i64 = (s_7_38 as i64);
        // D s_7_40: read-var results:[bv; 4]
        let s_7_40: [Bits; 4usize] = fn_state.results;
        // D s_7_41: cast cvt s_7_40 -> [bv; 0]
        let s_7_41: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_7_40);
        // D s_7_42: cast zx s_7_39 -> i
        let s_7_42: i128 = (i128::try_from(s_7_39).unwrap());
        // D s_7_43: read-element s_7_41[s_7_42]
        let s_7_43: Bits = s_7_41[(s_7_42) as usize];
        // D s_7_44: write-var ga#337778 <= s_7_43
        fn_state.ga_337778 = s_7_43;
        // D s_7_45: read-var esizeshadow#6729:i64
        let s_7_45: i64 = fn_state.esizeshadow_6729;
        // D s_7_46: cast zx s_7_45 -> i
        let s_7_46: i128 = (i128::try_from(s_7_45).unwrap());
        // D s_7_47: cast reint s_7_46 -> i64
        let s_7_47: i64 = (s_7_46 as i64);
        // D s_7_48: write-var ga#337779 <= s_7_47
        fn_state.ga_337779 = s_7_47;
        // D s_7_49: read-var is_unsigned:u8
        let s_7_49: bool = fn_state.is_unsigned;
        // N s_7_50: branch s_7_49 b10 b8
        if s_7_49 {
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
        // D s_8_0: read-var esizeshadow#6729:i64
        let s_8_0: i64 = fn_state.esizeshadow_6729;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var element:bv
        let s_8_2: Bits = fn_state.element;
        // D s_8_3: bits-cast sx s_8_2 -> bv length s_8_1
        let s_8_3: Bits = s_8_2.sign_extend(s_8_1);
        // D s_8_4: write-var ga#337780 <= s_8_3
        fn_state.ga_337780 = s_8_3;
        // N s_8_5: jump b9
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
        // D s_9_2: read-var ga#337779:i64
        let s_9_2: i64 = fn_state.ga_337779;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: read-var ga#337778:bv
        let s_9_4: Bits = fn_state.ga_337778;
        // D s_9_5: read-var ga#337780:bv
        let s_9_5: Bits = fn_state.ga_337780;
        // D s_9_6: call Elem_set(s_9_4, s_9_1, s_9_3, s_9_5)
        let s_9_6: Bits = Elem_set(state, tracer, s_9_4, s_9_1, s_9_3, s_9_5);
        // D s_9_7: read-var results:[bv; 4]
        let s_9_7: [Bits; 4usize] = fn_state.results;
        // D s_9_8: cast cvt s_9_7 -> [bv; 0]
        let s_9_8: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_9_7);
        // D s_9_9: read-var ga#337781:i64
        let s_9_9: i64 = fn_state.ga_337781;
        // D s_9_10: cast zx s_9_9 -> i
        let s_9_10: i128 = (i128::try_from(s_9_9).unwrap());
        // D s_9_11: mutate-element s_9_8[s_9_10] <= s_9_6
        let s_9_11: alloc::vec::Vec<Bits> = {
            let mut local = s_9_8.clone();
            local[(s_9_10) as usize] = s_9_6;
            local
        };
        // D s_9_12: cast cvt s_9_11 -> [bv; 4]
        let s_9_12: [Bits; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_9_11);
            buf
        };
        // D s_9_13: write-var results <= s_9_12
        fn_state.results = s_9_12;
        // D s_9_14: read-var e:i64
        let s_9_14: i64 = fn_state.e;
        // C s_9_15: const #1s : i64
        let s_9_15: i64 = 1;
        // D s_9_16: add s_9_14 s_9_15
        let s_9_16: i64 = (s_9_14 + s_9_15);
        // D s_9_17: write-var e <= s_9_16
        fn_state.e = s_9_16;
        // N s_9_18: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esizeshadow#6729:i64
        let s_10_0: i64 = fn_state.esizeshadow_6729;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var element:bv
        let s_10_2: Bits = fn_state.element;
        // D s_10_3: bits-cast zx s_10_2 -> bv length s_10_1
        let s_10_3: Bits = s_10_2.zero_extend(s_10_1);
        // D s_10_4: write-var ga#337780 <= s_10_3
        fn_state.ga_337780 = s_10_3;
        // N s_10_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var i:i64
        let s_11_0: i64 = fn_state.i;
        // C s_11_1: const #1s : i64
        let s_11_1: i64 = 1;
        // D s_11_2: add s_11_0 s_11_1
        let s_11_2: i64 = (s_11_0 + s_11_1);
        // D s_11_3: write-var i <= s_11_2
        fn_state.i = s_11_2;
        // N s_11_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // C s_13_0: const #0s : i64
        let s_13_0: i64 = 0;
        // C s_13_1: const #1s : i
        let s_13_1: i128 = 1;
        // D s_13_2: read-var nreg:i64
        let s_13_2: i64 = fn_state.nreg;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: sub s_13_3 s_13_1
        let s_13_4: i128 = ((s_13_3) - (s_13_1));
        // D s_13_5: cast reint s_13_4 -> i64
        let s_13_5: i64 = (s_13_4 as i64);
        // D s_13_6: write-var gs#286765 <= s_13_5
        fn_state.gs_286765 = s_13_5;
        // D s_13_7: write-var u#8935 <= s_13_0
        fn_state.u_8935 = s_13_0;
        // N s_13_8: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var u#8935:i64
        let s_14_0: i64 = fn_state.u_8935;
        // D s_14_1: read-var gs#286765:i64
        let s_14_1: i64 = fn_state.gs_286765;
        // D s_14_2: cmp-gt s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) > (s_14_1));
        // N s_14_3: branch s_14_2 b16 b15
        if s_14_2 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var d:i64
        let s_15_0: i64 = fn_state.d;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: read-var u#8935:i64
        let s_15_2: i64 = fn_state.u_8935;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: add s_15_1 s_15_3
        let s_15_4: i128 = (s_15_1 + s_15_3);
        // D s_15_5: cast reint s_15_4 -> i64
        let s_15_5: i64 = (s_15_4 as i64);
        // D s_15_6: read-var VLshadow#6731:i64
        let s_15_6: i64 = fn_state.VLshadow_6731;
        // D s_15_7: cast zx s_15_6 -> i
        let s_15_7: i128 = (i128::try_from(s_15_6).unwrap());
        // D s_15_8: cast reint s_15_7 -> i64
        let s_15_8: i64 = (s_15_7 as i64);
        // D s_15_9: read-var results:[bv; 4]
        let s_15_9: [Bits; 4usize] = fn_state.results;
        // D s_15_10: cast cvt s_15_9 -> [bv; 0]
        let s_15_10: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_15_9);
        // D s_15_11: read-var u#8935:i64
        let s_15_11: i64 = fn_state.u_8935;
        // D s_15_12: cast zx s_15_11 -> i
        let s_15_12: i128 = (i128::try_from(s_15_11).unwrap());
        // D s_15_13: read-element s_15_10[s_15_12]
        let s_15_13: Bits = s_15_10[(s_15_12) as usize];
        // D s_15_14: cast zx s_15_5 -> i
        let s_15_14: i128 = (i128::try_from(s_15_5).unwrap());
        // D s_15_15: cast zx s_15_8 -> i
        let s_15_15: i128 = (i128::try_from(s_15_8).unwrap());
        // D s_15_16: call Z_set(s_15_14, s_15_15, s_15_13)
        let s_15_16: () = Z_set(state, tracer, s_15_14, s_15_15, s_15_13);
        // D s_15_17: read-var u#8935:i64
        let s_15_17: i64 = fn_state.u_8935;
        // C s_15_18: const #1s : i64
        let s_15_18: i64 = 1;
        // D s_15_19: add s_15_17 s_15_18
        let s_15_19: i64 = (s_15_17 + s_15_18);
        // D s_15_20: write-var u#8935 <= s_15_19
        fn_state.u_8935 = s_15_19;
        // N s_15_21: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
}
