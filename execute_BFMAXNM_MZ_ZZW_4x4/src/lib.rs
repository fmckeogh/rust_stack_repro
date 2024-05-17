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
use Z_set::*;
use CheckStreamingSVEEnabled::*;
use Elem_read::*;
use FPCR_read::*;
use Z_read::*;
use BFMaxNum::*;
use Elem_set::*;
use common::*;
pub fn execute_BFMAXNM_MZ_ZZW_4x4<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    m: i64,
    nreg: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        gs_877046: Bits,
        VLshadow_7090: i64,
        gs_294821: i64,
        gs_294815: i64,
        elements: i64,
        VLshadow_7091: i64,
        gs_294833: i64,
        operand1: Bits,
        results: [Bits; 4usize],
        ga_342558: Bits,
        u_9431: i64,
        operand2: Bits,
        VL: i64,
        dn: i64,
        m: i64,
        nreg: i64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        m,
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
        // D s_0_3: write-var VLshadow#7090 <= s_0_2
        fn_state.VLshadow_7090 = s_0_2;
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
        // D s_1_0: read-var VLshadow#7090:i64
        let s_1_0: i64 = fn_state.VLshadow_7090;
        // D s_1_1: write-var VLshadow#7091 <= s_1_0
        fn_state.VLshadow_7091 = s_1_0;
        // C s_1_2: const #16s : i
        let s_1_2: i128 = 16;
        // D s_1_3: read-var VLshadow#7091:i64
        let s_1_3: i64 = fn_state.VLshadow_7091;
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
        // D s_1_14: write-var gs#294815 <= s_1_13
        fn_state.gs_294815 = s_1_13;
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
        // D s_2_1: read-var gs#294815:i64
        let s_2_1: i64 = fn_state.gs_294815;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b8 b3
        if s_2_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var dn:i64
        let s_3_0: i64 = fn_state.dn;
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
        // D s_3_6: read-var VLshadow#7091:i64
        let s_3_6: i64 = fn_state.VLshadow_7091;
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
        // D s_3_19: read-var VLshadow#7091:i64
        let s_3_19: i64 = fn_state.VLshadow_7091;
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
        // D s_3_32: write-var gs#294821 <= s_3_31
        fn_state.gs_294821 = s_3_31;
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
        // D s_4_1: read-var gs#294821:i64
        let s_4_1: i64 = fn_state.gs_294821;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b7 b5
        if s_4_2 {
            return block_7(state, tracer, fn_state);
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
        // D s_5_14: read-var results:[bv; 4]
        let s_5_14: [Bits; 4usize] = fn_state.results;
        // D s_5_15: cast cvt s_5_14 -> [bv; 0]
        let s_5_15: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_5_14);
        // D s_5_16: read-var r:i64
        let s_5_16: i64 = fn_state.r;
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_18: read-element s_5_15[s_5_17]
        let s_5_18: Bits = s_5_15[(s_5_17) as usize];
        // D s_5_19: write-var ga#342558 <= s_5_18
        fn_state.ga_342558 = s_5_18;
        // C s_5_20: const #() : ()
        let s_5_20: () = ();
        // S s_5_21: call FPCR_read(s_5_20)
        let s_5_21: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_5_20);
        // D s_5_22: cast zx s_5_6 -> bv
        let s_5_22: Bits = Bits::new(s_5_6 as u128, 16u16);
        // D s_5_23: cast zx s_5_13 -> bv
        let s_5_23: Bits = Bits::new(s_5_13 as u128, 16u16);
        // D s_5_24: call BFMaxNum(s_5_22, s_5_23, s_5_21)
        let s_5_24: Bits = BFMaxNum(state, tracer, s_5_22, s_5_23, s_5_21);
        // D s_5_25: write-var gs#877046 <= s_5_24
        fn_state.gs_877046 = s_5_24;
        // N s_5_26: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#877046:bv
        let s_6_0: Bits = fn_state.gs_877046;
        // D s_6_1: cast reint s_6_0 -> u16
        let s_6_1: u16 = (s_6_0.value() as u16);
        // D s_6_2: read-var e:i64
        let s_6_2: i64 = fn_state.e;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // C s_6_4: const #16s : i64
        let s_6_4: i64 = 16;
        // C s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: cast zx s_6_1 -> bv
        let s_6_6: Bits = Bits::new(s_6_1 as u128, 16u16);
        // D s_6_7: read-var ga#342558:bv
        let s_6_7: Bits = fn_state.ga_342558;
        // D s_6_8: call Elem_set(s_6_7, s_6_3, s_6_5, s_6_6)
        let s_6_8: Bits = Elem_set(state, tracer, s_6_7, s_6_3, s_6_5, s_6_6);
        // D s_6_9: read-var results:[bv; 4]
        let s_6_9: [Bits; 4usize] = fn_state.results;
        // D s_6_10: cast cvt s_6_9 -> [bv; 0]
        let s_6_10: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_6_9);
        // D s_6_11: read-var r:i64
        let s_6_11: i64 = fn_state.r;
        // D s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_13: mutate-element s_6_10[s_6_12] <= s_6_8
        let s_6_13: alloc::vec::Vec<Bits> = {
            let mut local = s_6_10.clone();
            local[(s_6_12) as usize] = s_6_8;
            local
        };
        // D s_6_14: cast cvt s_6_13 -> [bv; 4]
        let s_6_14: [Bits; 4usize] = {
            let mut buf = [Default::default(); 4usize];
            buf.copy_from_slice(&s_6_13);
            buf
        };
        // D s_6_15: write-var results <= s_6_14
        fn_state.results = s_6_14;
        // D s_6_16: read-var e:i64
        let s_6_16: i64 = fn_state.e;
        // C s_6_17: const #1s : i64
        let s_6_17: i64 = 1;
        // D s_6_18: add s_6_16 s_6_17
        let s_6_18: i64 = (s_6_16 + s_6_17);
        // D s_6_19: write-var e <= s_6_18
        fn_state.e = s_6_18;
        // N s_6_20: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var r:i64
        let s_7_0: i64 = fn_state.r;
        // C s_7_1: const #1s : i64
        let s_7_1: i64 = 1;
        // D s_7_2: add s_7_0 s_7_1
        let s_7_2: i64 = (s_7_0 + s_7_1);
        // D s_7_3: write-var r <= s_7_2
        fn_state.r = s_7_2;
        // N s_7_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0s : i64
        let s_8_0: i64 = 0;
        // C s_8_1: const #1s : i
        let s_8_1: i128 = 1;
        // D s_8_2: read-var nreg:i64
        let s_8_2: i64 = fn_state.nreg;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: sub s_8_3 s_8_1
        let s_8_4: i128 = ((s_8_3) - (s_8_1));
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: write-var gs#294833 <= s_8_5
        fn_state.gs_294833 = s_8_5;
        // D s_8_7: write-var u#9431 <= s_8_0
        fn_state.u_9431 = s_8_0;
        // N s_8_8: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var u#9431:i64
        let s_9_0: i64 = fn_state.u_9431;
        // D s_9_1: read-var gs#294833:i64
        let s_9_1: i64 = fn_state.gs_294833;
        // D s_9_2: cmp-gt s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) > (s_9_1));
        // N s_9_3: branch s_9_2 b11 b10
        if s_9_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var dn:i64
        let s_10_0: i64 = fn_state.dn;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var u#9431:i64
        let s_10_2: i64 = fn_state.u_9431;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: add s_10_1 s_10_3
        let s_10_4: i128 = (s_10_1 + s_10_3);
        // D s_10_5: cast reint s_10_4 -> i64
        let s_10_5: i64 = (s_10_4 as i64);
        // D s_10_6: read-var VLshadow#7091:i64
        let s_10_6: i64 = fn_state.VLshadow_7091;
        // D s_10_7: cast zx s_10_6 -> i
        let s_10_7: i128 = (i128::try_from(s_10_6).unwrap());
        // D s_10_8: cast reint s_10_7 -> i64
        let s_10_8: i64 = (s_10_7 as i64);
        // D s_10_9: read-var results:[bv; 4]
        let s_10_9: [Bits; 4usize] = fn_state.results;
        // D s_10_10: cast cvt s_10_9 -> [bv; 0]
        let s_10_10: alloc::vec::Vec<Bits> = alloc::vec::Vec::from(s_10_9);
        // D s_10_11: read-var u#9431:i64
        let s_10_11: i64 = fn_state.u_9431;
        // D s_10_12: cast zx s_10_11 -> i
        let s_10_12: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_13: read-element s_10_10[s_10_12]
        let s_10_13: Bits = s_10_10[(s_10_12) as usize];
        // D s_10_14: cast zx s_10_5 -> i
        let s_10_14: i128 = (i128::try_from(s_10_5).unwrap());
        // D s_10_15: cast zx s_10_8 -> i
        let s_10_15: i128 = (i128::try_from(s_10_8).unwrap());
        // D s_10_16: call Z_set(s_10_14, s_10_15, s_10_13)
        let s_10_16: () = Z_set(state, tracer, s_10_14, s_10_15, s_10_13);
        // D s_10_17: read-var u#9431:i64
        let s_10_17: i64 = fn_state.u_9431;
        // C s_10_18: const #1s : i64
        let s_10_18: i64 = 1;
        // D s_10_19: add s_10_17 s_10_18
        let s_10_19: i64 = (s_10_17 + s_10_18);
        // D s_10_20: write-var u#9431 <= s_10_19
        fn_state.u_9431 = s_10_19;
        // N s_10_21: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
}
