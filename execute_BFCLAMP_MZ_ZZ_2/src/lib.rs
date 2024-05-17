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
use FPCR_read::*;
use BFMaxNum::*;
use BFMinNum::*;
use Elem_read::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_BFCLAMP_MZ_ZZ_2<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    m: i64,
    n: i64,
    nreg: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        gs_295147: i64,
        e: i64,
        operand3: Bits,
        gs_877468: Bits,
        gs_877465: Bits,
        element2: u16,
        VLshadow_7102: i64,
        VLshadow_7103: i64,
        elements: i64,
        u_9459: i64,
        ga_342727: Bits,
        operand1: Bits,
        results: [Bits; 4usize],
        gs_295153: i64,
        gs_295166: i64,
        operand2: Bits,
        VL: i64,
        d: i64,
        m: i64,
        n: i64,
        nreg: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
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
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#7102 <= s_0_2
        fn_state.VLshadow_7102 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#7102:i64
        let s_1_0: i64 = fn_state.VLshadow_7102;
        // D s_1_1: write-var VLshadow#7103 <= s_1_0
        fn_state.VLshadow_7103 = s_1_0;
        // C s_1_2: const #16s : i
        let s_1_2: i128 = 16;
        // D s_1_3: read-var VLshadow#7103:i64
        let s_1_3: i64 = fn_state.VLshadow_7103;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: write-var elements <= s_1_6
        fn_state.elements = s_1_6;
        // C s_1_8: const #0s : i64
        let s_1_8: i64 = 0;
        // C s_1_9: const #1s : i
        let s_1_9: i128 = 1;
        // D s_1_10: read-var nreg:i64
        let s_1_10: i64 = fn_state.nreg;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: sub s_1_11 s_1_9
        let s_1_12: i128 = ((s_1_11) - (s_1_9));
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: write-var gs#295147 <= s_1_13
        fn_state.gs_295147 = s_1_13;
        // D s_1_15: write-var r <= s_1_8
        fn_state.r = s_1_8;
        // N s_1_16: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#295147:i64
        let s_2_1: i64 = fn_state.gs_295147;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b9 b3
        if s_2_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var VLshadow#7103:i64
        let s_3_0: i64 = fn_state.VLshadow_7103;
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
        // D s_3_8: read-var VLshadow#7103:i64
        let s_3_8: i64 = fn_state.VLshadow_7103;
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
        // D s_3_22: read-var VLshadow#7103:i64
        let s_3_22: i64 = fn_state.VLshadow_7103;
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
        // D s_3_35: write-var gs#295153 <= s_3_34
        fn_state.gs_295153 = s_3_34;
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
        // D s_4_1: read-var gs#295153:i64
        let s_4_1: i64 = fn_state.gs_295153;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b8 b5
        if s_4_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16s : i64
        let s_5_0: i64 = 16;
        // D s_5_1: read-var e:i64
        let s_5_1: i64 = fn_state.e;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // C s_5_3: cast zx s_5_0 -> i
        let s_5_3: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_4: read-var operand1:bv
        let s_5_4: Bits = fn_state.operand1;
        // D s_5_5: call Elem_read(s_5_4, s_5_2, s_5_3)
        let s_5_5: Bits = Elem_read(state, tracer, s_5_4, s_5_2, s_5_3);
        // D s_5_6: cast reint s_5_5 -> u16
        let s_5_6: u16 = (s_5_5.value() as u16);
        // C s_5_7: const #16s : i64
        let s_5_7: i64 = 16;
        // D s_5_8: read-var e:i64
        let s_5_8: i64 = fn_state.e;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // C s_5_10: cast zx s_5_7 -> i
        let s_5_10: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_11: read-var operand2:bv
        let s_5_11: Bits = fn_state.operand2;
        // D s_5_12: call Elem_read(s_5_11, s_5_9, s_5_10)
        let s_5_12: Bits = Elem_read(state, tracer, s_5_11, s_5_9, s_5_10);
        // D s_5_13: cast reint s_5_12 -> u16
        let s_5_13: u16 = (s_5_12.value() as u16);
        // D s_5_14: write-var element2 <= s_5_13
        fn_state.element2 = s_5_13;
        // C s_5_15: const #16s : i64
        let s_5_15: i64 = 16;
        // D s_5_16: read-var e:i64
        let s_5_16: i64 = fn_state.e;
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (i128::try_from(s_5_16).unwrap());
        // C s_5_18: cast zx s_5_15 -> i
        let s_5_18: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_19: read-var operand3:bv
        let s_5_19: Bits = fn_state.operand3;
        // D s_5_20: call Elem_read(s_5_19, s_5_17, s_5_18)
        let s_5_20: Bits = Elem_read(state, tracer, s_5_19, s_5_17, s_5_18);
        // D s_5_21: cast reint s_5_20 -> u16
        let s_5_21: u16 = (s_5_20.value() as u16);
        // D s_5_22: read-var results:[bv; 4]
        let s_5_22: [Bits; 4usize] = fn_state.results;
        // D s_5_23: cast cvt s_5_22 -> [bv; 0]
        let s_5_23: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_5_22);
        // D s_5_24: read-var r:i64
        let s_5_24: i64 = fn_state.r;
        // D s_5_25: cast zx s_5_24 -> i
        let s_5_25: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_26: read-element s_5_23[s_5_25]
        let s_5_26: Bits = s_5_23[(s_5_25) as usize];
        // D s_5_27: write-var ga#342727 <= s_5_26
        fn_state.ga_342727 = s_5_26;
        // C s_5_28: const #() : ()
        let s_5_28: () = ();
        // S s_5_29: call FPCR_read(s_5_28)
        let s_5_29: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_5_28);
        // D s_5_30: cast zx s_5_6 -> bv
        let s_5_30: Bits = Bits::new(s_5_6 as u128, 16u16);
        // D s_5_31: cast zx s_5_21 -> bv
        let s_5_31: Bits = Bits::new(s_5_21 as u128, 16u16);
        // D s_5_32: call BFMaxNum(s_5_30, s_5_31, s_5_29)
        let s_5_32: Bits = BFMaxNum(state, tracer, s_5_30, s_5_31, s_5_29);
        // D s_5_33: write-var gs#877465 <= s_5_32
        fn_state.gs_877465 = s_5_32;
        // N s_5_34: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#877465:bv
        let s_6_0: Bits = fn_state.gs_877465;
        // D s_6_1: cast reint s_6_0 -> u16
        let s_6_1: u16 = (s_6_0.value() as u16);
        // C s_6_2: const #() : ()
        let s_6_2: () = ();
        // S s_6_3: call FPCR_read(s_6_2)
        let s_6_3: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_6_2);
        // D s_6_4: cast zx s_6_1 -> bv
        let s_6_4: Bits = Bits::new(s_6_1 as u128, 16u16);
        // D s_6_5: read-var element2:u16
        let s_6_5: u16 = fn_state.element2;
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 16u16);
        // D s_6_7: call BFMinNum(s_6_4, s_6_6, s_6_3)
        let s_6_7: Bits = BFMinNum(state, tracer, s_6_4, s_6_6, s_6_3);
        // D s_6_8: write-var gs#877468 <= s_6_7
        fn_state.gs_877468 = s_6_7;
        // N s_6_9: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#877468:bv
        let s_7_0: Bits = fn_state.gs_877468;
        // D s_7_1: cast reint s_7_0 -> u16
        let s_7_1: u16 = (s_7_0.value() as u16);
        // D s_7_2: read-var e:i64
        let s_7_2: i64 = fn_state.e;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // C s_7_4: const #16s : i64
        let s_7_4: i64 = 16;
        // C s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: cast zx s_7_1 -> bv
        let s_7_6: Bits = Bits::new(s_7_1 as u128, 16u16);
        // D s_7_7: read-var ga#342727:bv
        let s_7_7: Bits = fn_state.ga_342727;
        // D s_7_8: call Elem_set(s_7_7, s_7_3, s_7_5, s_7_6)
        let s_7_8: Bits = Elem_set(state, tracer, s_7_7, s_7_3, s_7_5, s_7_6);
        // D s_7_9: read-var results:[bv; 4]
        let s_7_9: [Bits; 4usize] = fn_state.results;
        // D s_7_10: cast cvt s_7_9 -> [bv; 0]
        let s_7_10: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_7_9);
        // D s_7_11: read-var r:i64
        let s_7_11: i64 = fn_state.r;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: mutate-element s_7_10[s_7_12] <= s_7_8
        let s_7_13: alloc::vec::Vec<Bits> = {
            let mut local = s_7_10.clone();
            local[(s_7_12) as usize] = s_7_8;
            local
        };
        // D s_7_14: cast cvt s_7_13 -> [bv; 4]
        let s_7_14: [Bits; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_7_13);
            buf
        };
        // D s_7_15: write-var results <= s_7_14
        fn_state.results = s_7_14;
        // D s_7_16: read-var e:i64
        let s_7_16: i64 = fn_state.e;
        // C s_7_17: const #1s : i64
        let s_7_17: i64 = 1;
        // D s_7_18: add s_7_16 s_7_17
        let s_7_18: i64 = (s_7_16 + s_7_17);
        // D s_7_19: write-var e <= s_7_18
        fn_state.e = s_7_18;
        // N s_7_20: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var r:i64
        let s_8_0: i64 = fn_state.r;
        // C s_8_1: const #1s : i64
        let s_8_1: i64 = 1;
        // D s_8_2: add s_8_0 s_8_1
        let s_8_2: i64 = (s_8_0 + s_8_1);
        // D s_8_3: write-var r <= s_8_2
        fn_state.r = s_8_2;
        // N s_8_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0s : i64
        let s_9_0: i64 = 0;
        // C s_9_1: const #1s : i
        let s_9_1: i128 = 1;
        // D s_9_2: read-var nreg:i64
        let s_9_2: i64 = fn_state.nreg;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_4: sub s_9_3 s_9_1
        let s_9_4: i128 = ((s_9_3) - (s_9_1));
        // D s_9_5: cast reint s_9_4 -> i64
        let s_9_5: i64 = (s_9_4 as i64);
        // D s_9_6: write-var gs#295166 <= s_9_5
        fn_state.gs_295166 = s_9_5;
        // D s_9_7: write-var u#9459 <= s_9_0
        fn_state.u_9459 = s_9_0;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var u#9459:i64
        let s_10_0: i64 = fn_state.u_9459;
        // D s_10_1: read-var gs#295166:i64
        let s_10_1: i64 = fn_state.gs_295166;
        // D s_10_2: cmp-gt s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) > (s_10_1));
        // N s_10_3: branch s_10_2 b12 b11
        if s_10_2 {
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
        // D s_11_0: read-var d:i64
        let s_11_0: i64 = fn_state.d;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: read-var u#9459:i64
        let s_11_2: i64 = fn_state.u_9459;
        // D s_11_3: cast zx s_11_2 -> i
        let s_11_3: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_4: add s_11_1 s_11_3
        let s_11_4: i128 = (s_11_1 + s_11_3);
        // D s_11_5: cast reint s_11_4 -> i64
        let s_11_5: i64 = (s_11_4 as i64);
        // D s_11_6: read-var VLshadow#7103:i64
        let s_11_6: i64 = fn_state.VLshadow_7103;
        // D s_11_7: cast zx s_11_6 -> i
        let s_11_7: i128 = (i128::try_from(s_11_6).unwrap());
        // D s_11_8: cast reint s_11_7 -> i64
        let s_11_8: i64 = (s_11_7 as i64);
        // D s_11_9: read-var results:[bv; 4]
        let s_11_9: [Bits; 4usize] = fn_state.results;
        // D s_11_10: cast cvt s_11_9 -> [bv; 0]
        let s_11_10: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_11_9);
        // D s_11_11: read-var u#9459:i64
        let s_11_11: i64 = fn_state.u_9459;
        // D s_11_12: cast zx s_11_11 -> i
        let s_11_12: i128 = (i128::try_from(s_11_11).unwrap());
        // D s_11_13: read-element s_11_10[s_11_12]
        let s_11_13: Bits = s_11_10[(s_11_12) as usize];
        // D s_11_14: cast zx s_11_5 -> i
        let s_11_14: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_15: cast zx s_11_8 -> i
        let s_11_15: i128 = (i128::try_from(s_11_8).unwrap());
        // D s_11_16: call Z_set(s_11_14, s_11_15, s_11_13)
        let s_11_16: () = Z_set(state, tracer, s_11_14, s_11_15, s_11_13);
        // D s_11_17: read-var u#9459:i64
        let s_11_17: i64 = fn_state.u_9459;
        // C s_11_18: const #1s : i64
        let s_11_18: i64 = 1;
        // D s_11_19: add s_11_17 s_11_18
        let s_11_19: i64 = (s_11_17 + s_11_18);
        // D s_11_20: write-var u#9459 <= s_11_19
        fn_state.u_9459 = s_11_19;
        // N s_11_21: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
}
