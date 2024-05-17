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
use PredTest::*;
use Elem_set::*;
use CheckSVEEnabled::*;
use P_set::*;
use P_read::*;
use ActivePredicateElement::*;
use common::*;
pub fn execute_PFIRST_P_P_P__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    g: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        psize: i64,
        gs_198988: bool,
        firstshadow_3061: i128,
        first: i128,
        result: Bits,
        PL: i64,
        mask: Bits,
        gs_198984: i64,
        VL: i64,
        dn: i64,
        esize: i64,
        g: i64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        g,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckSVEEnabled(s_0_0)
        let s_0_1: () = CheckSVEEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VL:i64
        let s_1_0: i64 = fn_state.VL;
        // C s_1_1: const #8s : i
        let s_1_1: i128 = 8;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: div s_1_2 s_1_1
        let s_1_3: i128 = ((s_1_2) / (s_1_1));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var PL <= s_1_4
        fn_state.PL = s_1_4;
        // D s_1_6: cast zx s_1_0 -> i
        let s_1_6: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_7: read-var esize:i64
        let s_1_7: i64 = fn_state.esize;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: div s_1_6 s_1_8
        let s_1_9: i128 = ((s_1_6) / (s_1_8));
        // D s_1_10: cast reint s_1_9 -> i64
        let s_1_10: i64 = (s_1_9 as i64);
        // D s_1_11: read-var PL:i64
        let s_1_11: i64 = fn_state.PL;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: cast reint s_1_12 -> i64
        let s_1_13: i64 = (s_1_12 as i64);
        // D s_1_14: read-var g:i64
        let s_1_14: i64 = fn_state.g;
        // D s_1_15: cast zx s_1_14 -> i
        let s_1_15: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_16: cast zx s_1_13 -> i
        let s_1_16: i128 = (i128::try_from(s_1_13).unwrap());
        // D s_1_17: call P_read(s_1_15, s_1_16)
        let s_1_17: Bits = P_read(state, tracer, s_1_15, s_1_16);
        // D s_1_18: write-var mask <= s_1_17
        fn_state.mask = s_1_17;
        // D s_1_19: read-var PL:i64
        let s_1_19: i64 = fn_state.PL;
        // D s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // D s_1_21: cast reint s_1_20 -> i64
        let s_1_21: i64 = (s_1_20 as i64);
        // D s_1_22: read-var dn:i64
        let s_1_22: i64 = fn_state.dn;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: cast zx s_1_21 -> i
        let s_1_24: i128 = (i128::try_from(s_1_21).unwrap());
        // D s_1_25: call P_read(s_1_23, s_1_24)
        let s_1_25: Bits = P_read(state, tracer, s_1_23, s_1_24);
        // D s_1_26: write-var result <= s_1_25
        fn_state.result = s_1_25;
        // C s_1_27: const #-1s : i
        let s_1_27: i128 = -1;
        // D s_1_28: write-var first <= s_1_27
        fn_state.first = s_1_27;
        // C s_1_29: const #8s : i
        let s_1_29: i128 = 8;
        // D s_1_30: read-var esize:i64
        let s_1_30: i64 = fn_state.esize;
        // D s_1_31: cast zx s_1_30 -> i
        let s_1_31: i128 = (i128::try_from(s_1_30).unwrap());
        // D s_1_32: div s_1_31 s_1_29
        let s_1_32: i128 = ((s_1_31) / (s_1_29));
        // D s_1_33: cast reint s_1_32 -> i64
        let s_1_33: i64 = (s_1_32 as i64);
        // D s_1_34: write-var psize <= s_1_33
        fn_state.psize = s_1_33;
        // C s_1_35: const #0s : i64
        let s_1_35: i64 = 0;
        // C s_1_36: const #1s : i
        let s_1_36: i128 = 1;
        // D s_1_37: cast zx s_1_10 -> i
        let s_1_37: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_38: sub s_1_37 s_1_36
        let s_1_38: i128 = ((s_1_37) - (s_1_36));
        // D s_1_39: cast reint s_1_38 -> i64
        let s_1_39: i64 = (s_1_38 as i64);
        // D s_1_40: write-var gs#198984 <= s_1_39
        fn_state.gs_198984 = s_1_39;
        // D s_1_41: write-var e <= s_1_35
        fn_state.e = s_1_35;
        // N s_1_42: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#198984:i64
        let s_2_1: i64 = fn_state.gs_198984;
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
        // D s_3_0: read-var e:i64
        let s_3_0: i64 = fn_state.e;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var esize:i64
        let s_3_2: i64 = fn_state.esize;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-var mask:bv
        let s_3_4: Bits = fn_state.mask;
        // D s_3_5: call ActivePredicateElement(s_3_4, s_3_1, s_3_3)
        let s_3_5: bool = ActivePredicateElement(state, tracer, s_3_4, s_3_1, s_3_3);
        // N s_3_6: branch s_3_5 b9 b4
        if s_3_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#198988 <= s_4_0
        fn_state.gs_198988 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#198988:u8
        let s_5_0: bool = fn_state.gs_198988;
        // N s_5_1: branch s_5_0 b8 b6
        if s_5_0 {
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
        // N s_6_0: jump b7
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
        // N s_7_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var e:i64
        let s_8_0: i64 = fn_state.e;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: write-var first <= s_8_1
        fn_state.first = s_8_1;
        // N s_8_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #-1s : i
        let s_9_0: i128 = -1;
        // D s_9_1: read-var first:i
        let s_9_1: i128 = fn_state.first;
        // D s_9_2: cmp-eq s_9_1 s_9_0
        let s_9_2: bool = ((s_9_1) == (s_9_0));
        // D s_9_3: write-var gs#198988 <= s_9_2
        fn_state.gs_198988 = s_9_2;
        // N s_9_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var first:i
        let s_10_0: i128 = fn_state.first;
        // D s_10_1: write-var firstshadow#3061 <= s_10_0
        fn_state.firstshadow_3061 = s_10_0;
        // C s_10_2: const #0s : i
        let s_10_2: i128 = 0;
        // D s_10_3: read-var firstshadow#3061:i
        let s_10_3: i128 = fn_state.firstshadow_3061;
        // D s_10_4: cmp-ge s_10_3 s_10_2
        let s_10_4: bool = ((s_10_3) >= (s_10_2));
        // N s_10_5: branch s_10_4 b13 b11
        if s_10_4 {
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
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var esize:i64
        let s_12_0: i64 = fn_state.esize;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: read-var mask:bv
        let s_12_2: Bits = fn_state.mask;
        // D s_12_3: read-var result:bv
        let s_12_3: Bits = fn_state.result;
        // D s_12_4: call PredTest(s_12_2, s_12_3, s_12_1)
        let s_12_4: u8 = PredTest(state, tracer, s_12_2, s_12_3, s_12_1);
        // C s_12_5: const #3s : i
        let s_12_5: i128 = 3;
        // D s_12_6: cast zx s_12_4 -> bv
        let s_12_6: Bits = Bits::new(s_12_4 as u128, 4u16);
        // C s_12_7: const #1s : i64
        let s_12_7: i64 = 1;
        // C s_12_8: cast zx s_12_7 -> i
        let s_12_8: i128 = (i128::try_from(s_12_7).unwrap());
        // C s_12_9: const #0s : i
        let s_12_9: i128 = 0;
        // C s_12_10: add s_12_9 s_12_8
        let s_12_10: i128 = (s_12_9 + s_12_8);
        // D s_12_11: bit-extract s_12_6 s_12_5 s_12_10
        let s_12_11: Bits = (Bits::new(
            ((s_12_6) >> (s_12_5)).value(),
            u16::try_from(s_12_10).unwrap(),
        ));
        // D s_12_12: cast reint s_12_11 -> u8
        let s_12_12: bool = ((s_12_11.value()) != 0);
        // C s_12_13: const #16984u : u32
        let s_12_13: u32 = 16984;
        // N s_12_14: write-reg s_12_13 <= s_12_12
        let s_12_14: () = {
            state.write_register::<bool>(s_12_13 as isize, s_12_12);
            tracer.write_register(s_12_13 as isize, s_12_12);
        };
        // C s_12_15: const #2s : i
        let s_12_15: i128 = 2;
        // D s_12_16: cast zx s_12_4 -> bv
        let s_12_16: Bits = Bits::new(s_12_4 as u128, 4u16);
        // C s_12_17: const #1s : i64
        let s_12_17: i64 = 1;
        // C s_12_18: cast zx s_12_17 -> i
        let s_12_18: i128 = (i128::try_from(s_12_17).unwrap());
        // C s_12_19: const #0s : i
        let s_12_19: i128 = 0;
        // C s_12_20: add s_12_19 s_12_18
        let s_12_20: i128 = (s_12_19 + s_12_18);
        // D s_12_21: bit-extract s_12_16 s_12_15 s_12_20
        let s_12_21: Bits = (Bits::new(
            ((s_12_16) >> (s_12_15)).value(),
            u16::try_from(s_12_20).unwrap(),
        ));
        // D s_12_22: cast reint s_12_21 -> u8
        let s_12_22: bool = ((s_12_21.value()) != 0);
        // C s_12_23: const #16997u : u32
        let s_12_23: u32 = 16997;
        // N s_12_24: write-reg s_12_23 <= s_12_22
        let s_12_24: () = {
            state.write_register::<bool>(s_12_23 as isize, s_12_22);
            tracer.write_register(s_12_23 as isize, s_12_22);
        };
        // C s_12_25: const #1s : i
        let s_12_25: i128 = 1;
        // D s_12_26: cast zx s_12_4 -> bv
        let s_12_26: Bits = Bits::new(s_12_4 as u128, 4u16);
        // C s_12_27: const #1s : i64
        let s_12_27: i64 = 1;
        // C s_12_28: cast zx s_12_27 -> i
        let s_12_28: i128 = (i128::try_from(s_12_27).unwrap());
        // C s_12_29: const #0s : i
        let s_12_29: i128 = 0;
        // C s_12_30: add s_12_29 s_12_28
        let s_12_30: i128 = (s_12_29 + s_12_28);
        // D s_12_31: bit-extract s_12_26 s_12_25 s_12_30
        let s_12_31: Bits = (Bits::new(
            ((s_12_26) >> (s_12_25)).value(),
            u16::try_from(s_12_30).unwrap(),
        ));
        // D s_12_32: cast reint s_12_31 -> u8
        let s_12_32: bool = ((s_12_31.value()) != 0);
        // C s_12_33: const #16971u : u32
        let s_12_33: u32 = 16971;
        // N s_12_34: write-reg s_12_33 <= s_12_32
        let s_12_34: () = {
            state.write_register::<bool>(s_12_33 as isize, s_12_32);
            tracer.write_register(s_12_33 as isize, s_12_32);
        };
        // C s_12_35: const #0s : i
        let s_12_35: i128 = 0;
        // D s_12_36: cast zx s_12_4 -> bv
        let s_12_36: Bits = Bits::new(s_12_4 as u128, 4u16);
        // C s_12_37: const #1s : i64
        let s_12_37: i64 = 1;
        // C s_12_38: cast zx s_12_37 -> i
        let s_12_38: i128 = (i128::try_from(s_12_37).unwrap());
        // C s_12_39: const #0s : i
        let s_12_39: i128 = 0;
        // C s_12_40: add s_12_39 s_12_38
        let s_12_40: i128 = (s_12_39 + s_12_38);
        // D s_12_41: bit-extract s_12_36 s_12_35 s_12_40
        let s_12_41: Bits = (Bits::new(
            ((s_12_36) >> (s_12_35)).value(),
            u16::try_from(s_12_40).unwrap(),
        ));
        // D s_12_42: cast reint s_12_41 -> u8
        let s_12_42: bool = ((s_12_41.value()) != 0);
        // C s_12_43: const #16996u : u32
        let s_12_43: u32 = 16996;
        // N s_12_44: write-reg s_12_43 <= s_12_42
        let s_12_44: () = {
            state.write_register::<bool>(s_12_43 as isize, s_12_42);
            tracer.write_register(s_12_43 as isize, s_12_42);
        };
        // D s_12_45: read-var PL:i64
        let s_12_45: i64 = fn_state.PL;
        // D s_12_46: cast zx s_12_45 -> i
        let s_12_46: i128 = (i128::try_from(s_12_45).unwrap());
        // D s_12_47: cast reint s_12_46 -> i64
        let s_12_47: i64 = (s_12_46 as i64);
        // D s_12_48: read-var dn:i64
        let s_12_48: i64 = fn_state.dn;
        // D s_12_49: cast zx s_12_48 -> i
        let s_12_49: i128 = (i128::try_from(s_12_48).unwrap());
        // D s_12_50: cast zx s_12_47 -> i
        let s_12_50: i128 = (i128::try_from(s_12_47).unwrap());
        // D s_12_51: read-var result:bv
        let s_12_51: Bits = fn_state.result;
        // D s_12_52: call P_set(s_12_49, s_12_50, s_12_51)
        let s_12_52: () = P_set(state, tracer, s_12_49, s_12_50, s_12_51);
        // N s_12_53: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var psize:i64
        let s_13_0: i64 = fn_state.psize;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: cast reint s_13_1 -> i64
        let s_13_2: i64 = (s_13_1 as i64);
        // C s_13_3: const #1u : u8
        let s_13_3: bool = true;
        // C s_13_4: cast zx s_13_3 -> bv
        let s_13_4: Bits = Bits::new(s_13_3 as u128, 1u16);
        // D s_13_5: read-var psize:i64
        let s_13_5: i64 = fn_state.psize;
        // D s_13_6: cast zx s_13_5 -> i
        let s_13_6: i128 = (i128::try_from(s_13_5).unwrap());
        // D s_13_7: bits-cast zx s_13_4 -> bv length s_13_6
        let s_13_7: Bits = s_13_4.zero_extend(s_13_6);
        // D s_13_8: cast reint s_13_7 -> u8
        let s_13_8: bool = ((s_13_7.value()) != 0);
        // D s_13_9: cast zx s_13_2 -> i
        let s_13_9: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_10: cast zx s_13_8 -> bv
        let s_13_10: Bits = Bits::new(s_13_8 as u128, 1u16);
        // D s_13_11: read-var result:bv
        let s_13_11: Bits = fn_state.result;
        // D s_13_12: read-var firstshadow#3061:i
        let s_13_12: i128 = fn_state.firstshadow_3061;
        // D s_13_13: call Elem_set(s_13_11, s_13_12, s_13_9, s_13_10)
        let s_13_13: Bits = Elem_set(state, tracer, s_13_11, s_13_12, s_13_9, s_13_10);
        // D s_13_14: write-var result <= s_13_13
        fn_state.result = s_13_13;
        // N s_13_15: jump b12
        return block_12(state, tracer, fn_state);
    }
}
