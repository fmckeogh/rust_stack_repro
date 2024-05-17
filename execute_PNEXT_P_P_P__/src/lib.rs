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
use CheckSVEEnabled::*;
use P_read::*;
use ActivePredicateElement::*;
use LastActiveElement::*;
use PredTest::*;
use Zeros::*;
use P_set::*;
use common::*;
pub fn execute_PNEXT_P_P_P__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    dn: i64,
    esize: i64,
    v: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        nextshadow_3063: i128,
        psize: i64,
        elements: i64,
        gs_199048: bool,
        result: Bits,
        PL: i64,
        mask: Bits,
        next: i128,
        VL: i64,
        dn: i64,
        esize: i64,
        v: i64,
    }
    let fn_state = FunctionState {
        VL,
        dn,
        esize,
        v,
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
        // D s_1_11: write-var elements <= s_1_10
        fn_state.elements = s_1_10;
        // D s_1_12: read-var PL:i64
        let s_1_12: i64 = fn_state.PL;
        // D s_1_13: cast zx s_1_12 -> i
        let s_1_13: i128 = (i128::try_from(s_1_12).unwrap());
        // D s_1_14: cast reint s_1_13 -> i64
        let s_1_14: i64 = (s_1_13 as i64);
        // D s_1_15: read-var v:i64
        let s_1_15: i64 = fn_state.v;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast zx s_1_14 -> i
        let s_1_17: i128 = (i128::try_from(s_1_14).unwrap());
        // D s_1_18: call P_read(s_1_16, s_1_17)
        let s_1_18: Bits = P_read(state, tracer, s_1_16, s_1_17);
        // D s_1_19: write-var mask <= s_1_18
        fn_state.mask = s_1_18;
        // D s_1_20: read-var PL:i64
        let s_1_20: i64 = fn_state.PL;
        // D s_1_21: cast zx s_1_20 -> i
        let s_1_21: i128 = (i128::try_from(s_1_20).unwrap());
        // D s_1_22: cast reint s_1_21 -> i64
        let s_1_22: i64 = (s_1_21 as i64);
        // D s_1_23: read-var dn:i64
        let s_1_23: i64 = fn_state.dn;
        // D s_1_24: cast zx s_1_23 -> i
        let s_1_24: i128 = (i128::try_from(s_1_23).unwrap());
        // D s_1_25: cast zx s_1_22 -> i
        let s_1_25: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_26: call P_read(s_1_24, s_1_25)
        let s_1_26: Bits = P_read(state, tracer, s_1_24, s_1_25);
        // C s_1_27: const #8s : i
        let s_1_27: i128 = 8;
        // D s_1_28: read-var esize:i64
        let s_1_28: i64 = fn_state.esize;
        // D s_1_29: cast zx s_1_28 -> i
        let s_1_29: i128 = (i128::try_from(s_1_28).unwrap());
        // D s_1_30: div s_1_29 s_1_27
        let s_1_30: i128 = ((s_1_29) / (s_1_27));
        // D s_1_31: cast reint s_1_30 -> i64
        let s_1_31: i64 = (s_1_30 as i64);
        // D s_1_32: write-var psize <= s_1_31
        fn_state.psize = s_1_31;
        // D s_1_33: read-var esize:i64
        let s_1_33: i64 = fn_state.esize;
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: call LastActiveElement(s_1_26, s_1_34)
        let s_1_35: i128 = LastActiveElement(state, tracer, s_1_26, s_1_34);
        // C s_1_36: const #1s : i
        let s_1_36: i128 = 1;
        // D s_1_37: add s_1_35 s_1_36
        let s_1_37: i128 = (s_1_35 + s_1_36);
        // D s_1_38: write-var next <= s_1_37
        fn_state.next = s_1_37;
        // N s_1_39: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var elements:i64
        let s_2_0: i64 = fn_state.elements;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: read-var next:i
        let s_2_2: i128 = fn_state.next;
        // D s_2_3: cmp-lt s_2_2 s_2_1
        let s_2_3: bool = ((s_2_2) < (s_2_1));
        // N s_2_4: branch s_2_3 b10 b3
        if s_2_3 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#199048 <= s_3_0
        fn_state.gs_199048 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#199048:u8
        let s_4_0: bool = fn_state.gs_199048;
        // D s_4_1: not s_4_0
        let s_4_1: bool = !s_4_0;
        // N s_4_2: branch s_4_1 b6 b5
        if s_4_1 {
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
        // C s_5_0: const #1s : i
        let s_5_0: i128 = 1;
        // D s_5_1: read-var next:i
        let s_5_1: i128 = fn_state.next;
        // D s_5_2: add s_5_1 s_5_0
        let s_5_2: i128 = (s_5_1 + s_5_0);
        // D s_5_3: write-var next <= s_5_2
        fn_state.next = s_5_2;
        // N s_5_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var next:i
        let s_6_0: i128 = fn_state.next;
        // D s_6_1: write-var nextshadow#3063 <= s_6_0
        fn_state.nextshadow_3063 = s_6_0;
        // D s_6_2: read-var PL:i64
        let s_6_2: i64 = fn_state.PL;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: call Zeros(s_6_3)
        let s_6_4: Bits = Zeros(state, tracer, s_6_3);
        // D s_6_5: write-var result <= s_6_4
        fn_state.result = s_6_4;
        // D s_6_6: read-var elements:i64
        let s_6_6: i64 = fn_state.elements;
        // D s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_8: read-var nextshadow#3063:i
        let s_6_8: i128 = fn_state.nextshadow_3063;
        // D s_6_9: cmp-lt s_6_8 s_6_7
        let s_6_9: bool = ((s_6_8) < (s_6_7));
        // N s_6_10: branch s_6_9 b9 b7
        if s_6_9 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esize:i64
        let s_8_0: i64 = fn_state.esize;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: read-var mask:bv
        let s_8_2: Bits = fn_state.mask;
        // D s_8_3: read-var result:bv
        let s_8_3: Bits = fn_state.result;
        // D s_8_4: call PredTest(s_8_2, s_8_3, s_8_1)
        let s_8_4: u8 = PredTest(state, tracer, s_8_2, s_8_3, s_8_1);
        // C s_8_5: const #3s : i
        let s_8_5: i128 = 3;
        // D s_8_6: cast zx s_8_4 -> bv
        let s_8_6: Bits = Bits::new(s_8_4 as u128, 4u16);
        // C s_8_7: const #1s : i64
        let s_8_7: i64 = 1;
        // C s_8_8: cast zx s_8_7 -> i
        let s_8_8: i128 = (i128::try_from(s_8_7).unwrap());
        // C s_8_9: const #0s : i
        let s_8_9: i128 = 0;
        // C s_8_10: add s_8_9 s_8_8
        let s_8_10: i128 = (s_8_9 + s_8_8);
        // D s_8_11: bit-extract s_8_6 s_8_5 s_8_10
        let s_8_11: Bits = (Bits::new(
            ((s_8_6) >> (s_8_5)).value(),
            u16::try_from(s_8_10).unwrap(),
        ));
        // D s_8_12: cast reint s_8_11 -> u8
        let s_8_12: bool = ((s_8_11.value()) != 0);
        // C s_8_13: const #16984u : u32
        let s_8_13: u32 = 16984;
        // N s_8_14: write-reg s_8_13 <= s_8_12
        let s_8_14: () = {
            state.write_register::<bool>(s_8_13 as isize, s_8_12);
            tracer.write_register(s_8_13 as isize, s_8_12);
        };
        // C s_8_15: const #2s : i
        let s_8_15: i128 = 2;
        // D s_8_16: cast zx s_8_4 -> bv
        let s_8_16: Bits = Bits::new(s_8_4 as u128, 4u16);
        // C s_8_17: const #1s : i64
        let s_8_17: i64 = 1;
        // C s_8_18: cast zx s_8_17 -> i
        let s_8_18: i128 = (i128::try_from(s_8_17).unwrap());
        // C s_8_19: const #0s : i
        let s_8_19: i128 = 0;
        // C s_8_20: add s_8_19 s_8_18
        let s_8_20: i128 = (s_8_19 + s_8_18);
        // D s_8_21: bit-extract s_8_16 s_8_15 s_8_20
        let s_8_21: Bits = (Bits::new(
            ((s_8_16) >> (s_8_15)).value(),
            u16::try_from(s_8_20).unwrap(),
        ));
        // D s_8_22: cast reint s_8_21 -> u8
        let s_8_22: bool = ((s_8_21.value()) != 0);
        // C s_8_23: const #16997u : u32
        let s_8_23: u32 = 16997;
        // N s_8_24: write-reg s_8_23 <= s_8_22
        let s_8_24: () = {
            state.write_register::<bool>(s_8_23 as isize, s_8_22);
            tracer.write_register(s_8_23 as isize, s_8_22);
        };
        // C s_8_25: const #1s : i
        let s_8_25: i128 = 1;
        // D s_8_26: cast zx s_8_4 -> bv
        let s_8_26: Bits = Bits::new(s_8_4 as u128, 4u16);
        // C s_8_27: const #1s : i64
        let s_8_27: i64 = 1;
        // C s_8_28: cast zx s_8_27 -> i
        let s_8_28: i128 = (i128::try_from(s_8_27).unwrap());
        // C s_8_29: const #0s : i
        let s_8_29: i128 = 0;
        // C s_8_30: add s_8_29 s_8_28
        let s_8_30: i128 = (s_8_29 + s_8_28);
        // D s_8_31: bit-extract s_8_26 s_8_25 s_8_30
        let s_8_31: Bits = (Bits::new(
            ((s_8_26) >> (s_8_25)).value(),
            u16::try_from(s_8_30).unwrap(),
        ));
        // D s_8_32: cast reint s_8_31 -> u8
        let s_8_32: bool = ((s_8_31.value()) != 0);
        // C s_8_33: const #16971u : u32
        let s_8_33: u32 = 16971;
        // N s_8_34: write-reg s_8_33 <= s_8_32
        let s_8_34: () = {
            state.write_register::<bool>(s_8_33 as isize, s_8_32);
            tracer.write_register(s_8_33 as isize, s_8_32);
        };
        // C s_8_35: const #0s : i
        let s_8_35: i128 = 0;
        // D s_8_36: cast zx s_8_4 -> bv
        let s_8_36: Bits = Bits::new(s_8_4 as u128, 4u16);
        // C s_8_37: const #1s : i64
        let s_8_37: i64 = 1;
        // C s_8_38: cast zx s_8_37 -> i
        let s_8_38: i128 = (i128::try_from(s_8_37).unwrap());
        // C s_8_39: const #0s : i
        let s_8_39: i128 = 0;
        // C s_8_40: add s_8_39 s_8_38
        let s_8_40: i128 = (s_8_39 + s_8_38);
        // D s_8_41: bit-extract s_8_36 s_8_35 s_8_40
        let s_8_41: Bits = (Bits::new(
            ((s_8_36) >> (s_8_35)).value(),
            u16::try_from(s_8_40).unwrap(),
        ));
        // D s_8_42: cast reint s_8_41 -> u8
        let s_8_42: bool = ((s_8_41.value()) != 0);
        // C s_8_43: const #16996u : u32
        let s_8_43: u32 = 16996;
        // N s_8_44: write-reg s_8_43 <= s_8_42
        let s_8_44: () = {
            state.write_register::<bool>(s_8_43 as isize, s_8_42);
            tracer.write_register(s_8_43 as isize, s_8_42);
        };
        // D s_8_45: read-var PL:i64
        let s_8_45: i64 = fn_state.PL;
        // D s_8_46: cast zx s_8_45 -> i
        let s_8_46: i128 = (i128::try_from(s_8_45).unwrap());
        // D s_8_47: cast reint s_8_46 -> i64
        let s_8_47: i64 = (s_8_46 as i64);
        // D s_8_48: read-var dn:i64
        let s_8_48: i64 = fn_state.dn;
        // D s_8_49: cast zx s_8_48 -> i
        let s_8_49: i128 = (i128::try_from(s_8_48).unwrap());
        // D s_8_50: cast zx s_8_47 -> i
        let s_8_50: i128 = (i128::try_from(s_8_47).unwrap());
        // D s_8_51: read-var result:bv
        let s_8_51: Bits = fn_state.result;
        // D s_8_52: call P_set(s_8_49, s_8_50, s_8_51)
        let s_8_52: () = P_set(state, tracer, s_8_49, s_8_50, s_8_51);
        // N s_8_53: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var psize:i64
        let s_9_0: i64 = fn_state.psize;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: cast reint s_9_1 -> i64
        let s_9_2: i64 = (s_9_1 as i64);
        // C s_9_3: const #1u : u8
        let s_9_3: bool = true;
        // C s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 1u16);
        // D s_9_5: read-var psize:i64
        let s_9_5: i64 = fn_state.psize;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: bits-cast zx s_9_4 -> bv length s_9_6
        let s_9_7: Bits = s_9_4.zero_extend(s_9_6);
        // D s_9_8: cast zx s_9_2 -> i
        let s_9_8: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_9: read-var result:bv
        let s_9_9: Bits = fn_state.result;
        // D s_9_10: read-var nextshadow#3063:i
        let s_9_10: i128 = fn_state.nextshadow_3063;
        // D s_9_11: call Elem_set(s_9_9, s_9_10, s_9_8, s_9_7)
        let s_9_11: Bits = Elem_set(state, tracer, s_9_9, s_9_10, s_9_8, s_9_7);
        // D s_9_12: write-var result <= s_9_11
        fn_state.result = s_9_11;
        // N s_9_13: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var esize:i64
        let s_10_0: i64 = fn_state.esize;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: read-var mask:bv
        let s_10_2: Bits = fn_state.mask;
        // D s_10_3: read-var next:i
        let s_10_3: i128 = fn_state.next;
        // D s_10_4: call ActivePredicateElement(s_10_2, s_10_3, s_10_1)
        let s_10_4: bool = ActivePredicateElement(state, tracer, s_10_2, s_10_3, s_10_1);
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // D s_10_6: write-var gs#199048 <= s_10_5
        fn_state.gs_199048 = s_10_5;
        // N s_10_7: jump b4
        return block_4(state, tracer, fn_state);
    }
}
