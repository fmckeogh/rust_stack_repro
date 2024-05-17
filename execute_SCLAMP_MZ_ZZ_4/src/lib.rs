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
use integer_subrange::*;
use Z_set::*;
use common::*;
pub fn execute_SCLAMP_MZ_ZZ_4<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
    nreg: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        operand3: Bits,
        gs_286541: i64,
        VLshadow_6722: i64,
        esizeshadow_6720: i64,
        u_8920: i64,
        gs_286522: i64,
        elements: i64,
        gs_286516: i64,
        operand1: Bits,
        results: [Bits; 4usize],
        VLshadow_6721: i64,
        operand2: Bits,
        VL: i64,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
        nreg: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        esize,
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
        // D s_0_3: write-var esizeshadow#6720 <= s_0_2
        fn_state.esizeshadow_6720 = s_0_2;
        // D s_0_4: read-var VL:i64
        let s_0_4: i64 = fn_state.VL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var VLshadow#6721 <= s_0_6
        fn_state.VLshadow_6721 = s_0_6;
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
        // D s_1_0: read-var VLshadow#6721:i64
        let s_1_0: i64 = fn_state.VLshadow_6721;
        // D s_1_1: write-var VLshadow#6722 <= s_1_0
        fn_state.VLshadow_6722 = s_1_0;
        // D s_1_2: read-var VLshadow#6722:i64
        let s_1_2: i64 = fn_state.VLshadow_6722;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: read-var esizeshadow#6720:i64
        let s_1_4: i64 = fn_state.esizeshadow_6720;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: div s_1_3 s_1_5
        let s_1_6: i128 = ((s_1_3) / (s_1_5));
        // D s_1_7: cast reint s_1_6 -> i64
        let s_1_7: i64 = (s_1_6 as i64);
        // D s_1_8: write-var elements <= s_1_7
        fn_state.elements = s_1_7;
        // C s_1_9: const #0s : i64
        let s_1_9: i64 = 0;
        // C s_1_10: const #1s : i
        let s_1_10: i128 = 1;
        // D s_1_11: read-var nreg:i64
        let s_1_11: i64 = fn_state.nreg;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: sub s_1_12 s_1_10
        let s_1_13: i128 = ((s_1_12) - (s_1_10));
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: write-var gs#286516 <= s_1_14
        fn_state.gs_286516 = s_1_14;
        // D s_1_16: write-var r <= s_1_9
        fn_state.r = s_1_9;
        // N s_1_17: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#286516:i64
        let s_2_1: i64 = fn_state.gs_286516;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var VLshadow#6722:i64
        let s_3_0: i64 = fn_state.VLshadow_6722;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: cast reint s_3_1 -> i64
        let s_3_2: i64 = (s_3_1 as i64);
        // D s_3_3: read-var n:i64
        let s_3_3: i64 = fn_state.n;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: cast zx s_3_2 -> i
        let s_3_5: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_6: call Z_read(s_3_4, s_3_5)
        let s_3_6: Bits = Z_read(state, tracer, s_3_4, s_3_5);
        // D s_3_7: write-var operand1 <= s_3_6
        fn_state.operand1 = s_3_6;
        // D s_3_8: read-var VLshadow#6722:i64
        let s_3_8: i64 = fn_state.VLshadow_6722;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // D s_3_11: read-var m:i64
        let s_3_11: i64 = fn_state.m;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: cast zx s_3_10 -> i
        let s_3_13: i128 = (i128::try_from(s_3_10).unwrap());
        // D s_3_14: call Z_read(s_3_12, s_3_13)
        let s_3_14: Bits = Z_read(state, tracer, s_3_12, s_3_13);
        // D s_3_15: write-var operand2 <= s_3_14
        fn_state.operand2 = s_3_14;
        // D s_3_16: read-var d:i64
        let s_3_16: i64 = fn_state.d;
        // D s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // D s_3_18: read-var r:i64
        let s_3_18: i64 = fn_state.r;
        // D s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // D s_3_20: add s_3_17 s_3_19
        let s_3_20: i128 = (s_3_17 + s_3_19);
        // D s_3_21: cast reint s_3_20 -> i64
        let s_3_21: i64 = (s_3_20 as i64);
        // D s_3_22: read-var VLshadow#6722:i64
        let s_3_22: i64 = fn_state.VLshadow_6722;
        // D s_3_23: cast zx s_3_22 -> i
        let s_3_23: i128 = (i128::try_from(s_3_22).unwrap());
        // D s_3_24: cast reint s_3_23 -> i64
        let s_3_24: i64 = (s_3_23 as i64);
        // D s_3_25: cast zx s_3_21 -> i
        let s_3_25: i128 = (i128::try_from(s_3_21).unwrap());
        // D s_3_26: cast zx s_3_24 -> i
        let s_3_26: i128 = (i128::try_from(s_3_24).unwrap());
        // D s_3_27: call Z_read(s_3_25, s_3_26)
        let s_3_27: Bits = Z_read(state, tracer, s_3_25, s_3_26);
        // D s_3_28: write-var operand3 <= s_3_27
        fn_state.operand3 = s_3_27;
        // C s_3_29: const #0s : i64
        let s_3_29: i64 = 0;
        // C s_3_30: const #1s : i
        let s_3_30: i128 = 1;
        // D s_3_31: read-var elements:i64
        let s_3_31: i64 = fn_state.elements;
        // D s_3_32: cast zx s_3_31 -> i
        let s_3_32: i128 = (i128::try_from(s_3_31).unwrap());
        // D s_3_33: sub s_3_32 s_3_30
        let s_3_33: i128 = ((s_3_32) - (s_3_30));
        // D s_3_34: cast reint s_3_33 -> i64
        let s_3_34: i64 = (s_3_33 as i64);
        // D s_3_35: write-var gs#286522 <= s_3_34
        fn_state.gs_286522 = s_3_34;
        // D s_3_36: write-var e <= s_3_29
        fn_state.e = s_3_29;
        // N s_3_37: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#286522:i64
        let s_4_1: i64 = fn_state.gs_286522;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#6720:i64
        let s_5_0: i64 = fn_state.esizeshadow_6720;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: cast reint s_5_1 -> i64
        let s_5_2: i64 = (s_5_1 as i64);
        // D s_5_3: read-var e:i64
        let s_5_3: i64 = fn_state.e;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_5: cast zx s_5_2 -> i
        let s_5_5: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_6: read-var operand1:bv
        let s_5_6: Bits = fn_state.operand1;
        // D s_5_7: call Elem_read(s_5_6, s_5_4, s_5_5)
        let s_5_7: Bits = Elem_read(state, tracer, s_5_6, s_5_4, s_5_5);
        // D s_5_8: cast sx s_5_7 -> i
        let s_5_8: i128 = {
            let sign_bit = s_5_7.length() - 1;
            let mut result = s_5_7.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // D s_5_10: read-var esizeshadow#6720:i64
        let s_5_10: i64 = fn_state.esizeshadow_6720;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: cast reint s_5_11 -> i64
        let s_5_12: i64 = (s_5_11 as i64);
        // D s_5_13: read-var e:i64
        let s_5_13: i64 = fn_state.e;
        // D s_5_14: cast zx s_5_13 -> i
        let s_5_14: i128 = (i128::try_from(s_5_13).unwrap());
        // D s_5_15: cast zx s_5_12 -> i
        let s_5_15: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_16: read-var operand2:bv
        let s_5_16: Bits = fn_state.operand2;
        // D s_5_17: call Elem_read(s_5_16, s_5_14, s_5_15)
        let s_5_17: Bits = Elem_read(state, tracer, s_5_16, s_5_14, s_5_15);
        // D s_5_18: cast sx s_5_17 -> i
        let s_5_18: i128 = {
            let sign_bit = s_5_17.length() - 1;
            let mut result = s_5_17.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_19: cast reint s_5_18 -> i64
        let s_5_19: i64 = (s_5_18 as i64);
        // D s_5_20: read-var esizeshadow#6720:i64
        let s_5_20: i64 = fn_state.esizeshadow_6720;
        // D s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // D s_5_22: cast reint s_5_21 -> i64
        let s_5_22: i64 = (s_5_21 as i64);
        // D s_5_23: read-var e:i64
        let s_5_23: i64 = fn_state.e;
        // D s_5_24: cast zx s_5_23 -> i
        let s_5_24: i128 = (i128::try_from(s_5_23).unwrap());
        // D s_5_25: cast zx s_5_22 -> i
        let s_5_25: i128 = (i128::try_from(s_5_22).unwrap());
        // D s_5_26: read-var operand3:bv
        let s_5_26: Bits = fn_state.operand3;
        // D s_5_27: call Elem_read(s_5_26, s_5_24, s_5_25)
        let s_5_27: Bits = Elem_read(state, tracer, s_5_26, s_5_24, s_5_25);
        // D s_5_28: cast sx s_5_27 -> i
        let s_5_28: i128 = {
            let sign_bit = s_5_27.length() - 1;
            let mut result = s_5_27.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_5_29: cast reint s_5_28 -> i64
        let s_5_29: i64 = (s_5_28 as i64);
        // D s_5_30: cast zx s_5_9 -> i
        let s_5_30: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_31: cast zx s_5_29 -> i
        let s_5_31: i128 = (i128::try_from(s_5_29).unwrap());
        // D s_5_32: cmp-gt s_5_30 s_5_31
        let s_5_32: bool = ((s_5_30) > (s_5_31));
        // D s_5_33: select s_5_32 s_5_30 s_5_31
        let s_5_33: i128 = if s_5_32 { s_5_30 } else { s_5_31 };
        // D s_5_34: cast reint s_5_33 -> i64
        let s_5_34: i64 = (s_5_33 as i64);
        // D s_5_35: cast zx s_5_34 -> i
        let s_5_35: i128 = (i128::try_from(s_5_34).unwrap());
        // D s_5_36: cast zx s_5_19 -> i
        let s_5_36: i128 = (i128::try_from(s_5_19).unwrap());
        // D s_5_37: cmp-lt s_5_35 s_5_36
        let s_5_37: bool = ((s_5_35) < (s_5_36));
        // D s_5_38: select s_5_37 s_5_35 s_5_36
        let s_5_38: i128 = if s_5_37 { s_5_35 } else { s_5_36 };
        // D s_5_39: cast reint s_5_38 -> i64
        let s_5_39: i64 = (s_5_38 as i64);
        // D s_5_40: read-var results:[bv; 4]
        let s_5_40: [Bits; 4usize] = fn_state.results;
        // D s_5_41: cast cvt s_5_40 -> [bv; 0]
        let s_5_41: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_5_40);
        // D s_5_42: read-var r:i64
        let s_5_42: i64 = fn_state.r;
        // D s_5_43: cast zx s_5_42 -> i
        let s_5_43: i128 = (i128::try_from(s_5_42).unwrap());
        // D s_5_44: read-element s_5_41[s_5_43]
        let s_5_44: Bits = s_5_41[(s_5_43) as usize];
        // D s_5_45: read-var esizeshadow#6720:i64
        let s_5_45: i64 = fn_state.esizeshadow_6720;
        // D s_5_46: cast zx s_5_45 -> i
        let s_5_46: i128 = (i128::try_from(s_5_45).unwrap());
        // D s_5_47: cast reint s_5_46 -> i64
        let s_5_47: i64 = (s_5_46 as i64);
        // C s_5_48: const #1s : i
        let s_5_48: i128 = 1;
        // D s_5_49: read-var esizeshadow#6720:i64
        let s_5_49: i64 = fn_state.esizeshadow_6720;
        // D s_5_50: cast zx s_5_49 -> i
        let s_5_50: i128 = (i128::try_from(s_5_49).unwrap());
        // D s_5_51: sub s_5_50 s_5_48
        let s_5_51: i128 = ((s_5_50) - (s_5_48));
        // D s_5_52: cast reint s_5_51 -> i64
        let s_5_52: i64 = (s_5_51 as i64);
        // C s_5_53: const #0s : i
        let s_5_53: i128 = 0;
        // D s_5_54: cast zx s_5_39 -> i
        let s_5_54: i128 = (i128::try_from(s_5_39).unwrap());
        // D s_5_55: cast zx s_5_52 -> i
        let s_5_55: i128 = (i128::try_from(s_5_52).unwrap());
        // D s_5_56: call integer_subrange(s_5_54, s_5_55, s_5_53)
        let s_5_56: Bits = integer_subrange(state, tracer, s_5_54, s_5_55, s_5_53);
        // D s_5_57: read-var e:i64
        let s_5_57: i64 = fn_state.e;
        // D s_5_58: cast zx s_5_57 -> i
        let s_5_58: i128 = (i128::try_from(s_5_57).unwrap());
        // D s_5_59: cast zx s_5_47 -> i
        let s_5_59: i128 = (i128::try_from(s_5_47).unwrap());
        // D s_5_60: call Elem_set(s_5_44, s_5_58, s_5_59, s_5_56)
        let s_5_60: Bits = Elem_set(state, tracer, s_5_44, s_5_58, s_5_59, s_5_56);
        // D s_5_61: read-var results:[bv; 4]
        let s_5_61: [Bits; 4usize] = fn_state.results;
        // D s_5_62: cast cvt s_5_61 -> [bv; 0]
        let s_5_62: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_5_61);
        // D s_5_63: read-var r:i64
        let s_5_63: i64 = fn_state.r;
        // D s_5_64: cast zx s_5_63 -> i
        let s_5_64: i128 = (i128::try_from(s_5_63).unwrap());
        // D s_5_65: mutate-element s_5_62[s_5_64] <= s_5_60
        let s_5_65: alloc::vec::Vec<Bits> = {
            let mut local = s_5_62.clone();
            local[(s_5_64) as usize] = s_5_60;
            local
        };
        // D s_5_66: cast cvt s_5_65 -> [bv; 4]
        let s_5_66: [Bits; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_5_65);
            buf
        };
        // D s_5_67: write-var results <= s_5_66
        fn_state.results = s_5_66;
        // D s_5_68: read-var e:i64
        let s_5_68: i64 = fn_state.e;
        // C s_5_69: const #1s : i64
        let s_5_69: i64 = 1;
        // D s_5_70: add s_5_68 s_5_69
        let s_5_70: i64 = (s_5_68 + s_5_69);
        // D s_5_71: write-var e <= s_5_70
        fn_state.e = s_5_70;
        // N s_5_72: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var r:i64
        let s_6_0: i64 = fn_state.r;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var r <= s_6_2
        fn_state.r = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0s : i64
        let s_7_0: i64 = 0;
        // C s_7_1: const #1s : i
        let s_7_1: i128 = 1;
        // D s_7_2: read-var nreg:i64
        let s_7_2: i64 = fn_state.nreg;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: sub s_7_3 s_7_1
        let s_7_4: i128 = ((s_7_3) - (s_7_1));
        // D s_7_5: cast reint s_7_4 -> i64
        let s_7_5: i64 = (s_7_4 as i64);
        // D s_7_6: write-var gs#286541 <= s_7_5
        fn_state.gs_286541 = s_7_5;
        // D s_7_7: write-var u#8920 <= s_7_0
        fn_state.u_8920 = s_7_0;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var u#8920:i64
        let s_8_0: i64 = fn_state.u_8920;
        // D s_8_1: read-var gs#286541:i64
        let s_8_1: i64 = fn_state.gs_286541;
        // D s_8_2: cmp-gt s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) > (s_8_1));
        // N s_8_3: branch s_8_2 b10 b9
        if s_8_2 {
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
        // D s_9_0: read-var d:i64
        let s_9_0: i64 = fn_state.d;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: read-var u#8920:i64
        let s_9_2: i64 = fn_state.u_8920;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: add s_9_1 s_9_3
        let s_9_4: i128 = (s_9_1 + s_9_3);
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // D s_9_6: read-var VLshadow#6722:i64
        let s_9_6: i64 = fn_state.VLshadow_6722;
        // D s_9_7: cast zx s_9_6 -> i
        let s_9_7: i128 = (i128::try_from(s_9_6).unwrap());
        // D s_9_8: cast reint s_9_7 -> i64
        let s_9_8: i64 = (s_9_7 as i64);
        // D s_9_9: read-var results:[bv; 4]
        let s_9_9: [Bits; 4usize] = fn_state.results;
        // D s_9_10: cast cvt s_9_9 -> [bv; 0]
        let s_9_10: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_9_9);
        // D s_9_11: read-var u#8920:i64
        let s_9_11: i64 = fn_state.u_8920;
        // D s_9_12: cast zx s_9_11 -> i
        let s_9_12: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_13: read-element s_9_10[s_9_12]
        let s_9_13: Bits = s_9_10[(s_9_12) as usize];
        // D s_9_14: cast zx s_9_5 -> i
        let s_9_14: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_15: cast zx s_9_8 -> i
        let s_9_15: i128 = (i128::try_from(s_9_8).unwrap());
        // D s_9_16: call Z_set(s_9_14, s_9_15, s_9_13)
        let s_9_16: () = Z_set(state, tracer, s_9_14, s_9_15, s_9_13);
        // D s_9_17: read-var u#8920:i64
        let s_9_17: i64 = fn_state.u_8920;
        // C s_9_18: const #1s : i64
        let s_9_18: i64 = 1;
        // D s_9_19: add s_9_17 s_9_18
        let s_9_19: i64 = (s_9_17 + s_9_18);
        // D s_9_20: write-var u#8920 <= s_9_19
        fn_state.u_8920 = s_9_19;
        // N s_9_21: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: return
        return;
    }
}
