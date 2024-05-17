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
use CreateAccDescSVE::*;
use Elem_set::*;
use Mem_read::*;
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use X_read::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_LD1Q_Z_P_AR_D_64_unscaled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    g: i64,
    m: i64,
    n: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        gs_249446: i64,
        base: Bits,
        accdesc: ProductType9878976b5bcce9c9,
        offsetshadow_5282: u64,
        baseshadow_5283: Bits,
        elements: i64,
        offset: u64,
        VLshadow_5281: i64,
        result: Bits,
        gs_812669: Bits,
        PL: i64,
        mask: Bits,
        VL: i64,
        g: i64,
        m: i64,
        n: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        VL,
        g,
        m,
        n,
        t,
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
        // D s_0_3: write-var VLshadow#5281 <= s_0_2
        fn_state.VLshadow_5281 = s_0_2;
        // C s_0_4: const #8s : i
        let s_0_4: i128 = 8;
        // D s_0_5: read-var VLshadow#5281:i64
        let s_0_5: i64 = fn_state.VLshadow_5281;
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: div s_0_6 s_0_4
        let s_0_7: i128 = ((s_0_6) / (s_0_4));
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var PL <= s_0_8
        fn_state.PL = s_0_8;
        // C s_0_10: const #() : ()
        let s_0_10: () = ();
        // S s_0_11: call CheckNonStreamingSVEEnabled(s_0_10)
        let s_0_11: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_10);
        // N s_0_12: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #128s : i
        let s_1_0: i128 = 128;
        // D s_1_1: read-var VLshadow#5281:i64
        let s_1_1: i64 = fn_state.VLshadow_5281;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: div s_1_2 s_1_0
        let s_1_3: i128 = ((s_1_2) / (s_1_0));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: write-var elements <= s_1_4
        fn_state.elements = s_1_4;
        // D s_1_6: read-var PL:i64
        let s_1_6: i64 = fn_state.PL;
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: read-var g:i64
        let s_1_9: i64 = fn_state.g;
        // D s_1_10: cast zx s_1_9 -> i
        let s_1_10: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_11: cast zx s_1_8 -> i
        let s_1_11: i128 = (i128::try_from(s_1_8).unwrap());
        // D s_1_12: call P_read(s_1_10, s_1_11)
        let s_1_12: Bits = P_read(state, tracer, s_1_10, s_1_11);
        // D s_1_13: write-var mask <= s_1_12
        fn_state.mask = s_1_12;
        // C s_1_14: const #0u : u32
        let s_1_14: u32 = 0;
        // C s_1_15: const #0u : u8
        let s_1_15: bool = false;
        // C s_1_16: const #0u : u8
        let s_1_16: bool = false;
        // C s_1_17: const #1u : u8
        let s_1_17: bool = true;
        // S s_1_18: call CreateAccDescSVE(s_1_14, s_1_15, s_1_16, s_1_17)
        let s_1_18: ProductType9878976b5bcce9c9 = CreateAccDescSVE(
            state,
            tracer,
            s_1_14,
            s_1_15,
            s_1_16,
            s_1_17,
        );
        // D s_1_19: write-var accdesc <= s_1_18
        fn_state.accdesc = s_1_18;
        // C s_1_20: const #128s : i
        let s_1_20: i128 = 128;
        // D s_1_21: read-var mask:bv
        let s_1_21: Bits = fn_state.mask;
        // D s_1_22: call AnyActiveElement(s_1_21, s_1_20)
        let s_1_22: bool = AnyActiveElement(state, tracer, s_1_21, s_1_20);
        // N s_1_23: branch s_1_22 b11 b2
        if s_1_22 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var offset:u64
        let s_3_0: u64 = fn_state.offset;
        // D s_3_1: write-var offsetshadow#5282 <= s_3_0
        fn_state.offsetshadow_5282 = s_3_0;
        // D s_3_2: read-var base:bv
        let s_3_2: Bits = fn_state.base;
        // D s_3_3: write-var baseshadow#5283 <= s_3_2
        fn_state.baseshadow_5283 = s_3_2;
        // C s_3_4: const #0s : i64
        let s_3_4: i64 = 0;
        // C s_3_5: const #1s : i
        let s_3_5: i128 = 1;
        // D s_3_6: read-var elements:i64
        let s_3_6: i64 = fn_state.elements;
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (i128::try_from(s_3_6).unwrap());
        // D s_3_8: sub s_3_7 s_3_5
        let s_3_8: i128 = ((s_3_7) - (s_3_5));
        // D s_3_9: cast reint s_3_8 -> i64
        let s_3_9: i64 = (s_3_8 as i64);
        // D s_3_10: write-var gs#249446 <= s_3_9
        fn_state.gs_249446 = s_3_9;
        // D s_3_11: write-var e <= s_3_4
        fn_state.e = s_3_4;
        // N s_3_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#249446:i64
        let s_4_1: i64 = fn_state.gs_249446;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b10 b5
        if s_4_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #128s : i
        let s_5_0: i128 = 128;
        // D s_5_1: read-var e:i64
        let s_5_1: i64 = fn_state.e;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: read-var mask:bv
        let s_5_3: Bits = fn_state.mask;
        // D s_5_4: call ActivePredicateElement(s_5_3, s_5_2, s_5_0)
        let s_5_4: bool = ActivePredicateElement(state, tracer, s_5_3, s_5_2, s_5_0);
        // N s_5_5: branch s_5_4 b8 b6
        if s_5_4 {
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
        // C s_6_0: const #128s : i64
        let s_6_0: i64 = 128;
        // C s_6_1: const #0u : u8
        let s_6_1: u8 = 0;
        // C s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 0u16);
        // C s_6_3: const #0u : u64
        let s_6_3: u64 = 0;
        // C s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 64u16);
        // C s_6_5: cast reint s_6_2 -> u128
        let s_6_5: u128 = (s_6_2.value() as u128);
        // D s_6_6: size-of s_6_2
        let s_6_6: u16 = s_6_2.length();
        // C s_6_7: cast reint s_6_4 -> u128
        let s_6_7: u128 = (s_6_4.value() as u128);
        // D s_6_8: size-of s_6_4
        let s_6_8: u16 = s_6_4.length();
        // D s_6_9: lsl s_6_5 s_6_8
        let s_6_9: u128 = s_6_5 << s_6_8;
        // D s_6_10: or s_6_9 s_6_7
        let s_6_10: u128 = ((s_6_9) | (s_6_7));
        // D s_6_11: add s_6_6 s_6_8
        let s_6_11: u16 = (s_6_6 + s_6_8);
        // D s_6_12: create-bits s_6_10 s_6_11
        let s_6_12: Bits = Bits::new(s_6_10, s_6_11);
        // C s_6_13: const #0u : u64
        let s_6_13: u64 = 0;
        // C s_6_14: cast zx s_6_13 -> bv
        let s_6_14: Bits = Bits::new(s_6_13 as u128, 64u16);
        // D s_6_15: cast reint s_6_12 -> u128
        let s_6_15: u128 = (s_6_12.value() as u128);
        // D s_6_16: size-of s_6_12
        let s_6_16: u16 = s_6_12.length();
        // C s_6_17: cast reint s_6_14 -> u128
        let s_6_17: u128 = (s_6_14.value() as u128);
        // D s_6_18: size-of s_6_14
        let s_6_18: u16 = s_6_14.length();
        // D s_6_19: lsl s_6_15 s_6_18
        let s_6_19: u128 = s_6_15 << s_6_18;
        // D s_6_20: or s_6_19 s_6_17
        let s_6_20: u128 = ((s_6_19) | (s_6_17));
        // D s_6_21: add s_6_16 s_6_18
        let s_6_21: u16 = (s_6_16 + s_6_18);
        // D s_6_22: create-bits s_6_20 s_6_21
        let s_6_22: Bits = Bits::new(s_6_20, s_6_21);
        // D s_6_23: read-var e:i64
        let s_6_23: i64 = fn_state.e;
        // D s_6_24: cast zx s_6_23 -> i
        let s_6_24: i128 = (i128::try_from(s_6_23).unwrap());
        // C s_6_25: cast zx s_6_0 -> i
        let s_6_25: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_26: read-var result:bv
        let s_6_26: Bits = fn_state.result;
        // D s_6_27: call Elem_set(s_6_26, s_6_24, s_6_25, s_6_22)
        let s_6_27: Bits = Elem_set(state, tracer, s_6_26, s_6_24, s_6_25, s_6_22);
        // D s_6_28: write-var result <= s_6_27
        fn_state.result = s_6_27;
        // N s_6_29: jump b7
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
        // C s_8_0: const #2s : i
        let s_8_0: i128 = 2;
        // D s_8_1: read-var e:i64
        let s_8_1: i64 = fn_state.e;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: mul s_8_0 s_8_2
        let s_8_3: i128 = ((s_8_0) * (s_8_2));
        // D s_8_4: cast reint s_8_3 -> i64
        let s_8_4: i64 = (s_8_3 as i64);
        // C s_8_5: const #64s : i64
        let s_8_5: i64 = 64;
        // D s_8_6: cast zx s_8_4 -> i
        let s_8_6: i128 = (i128::try_from(s_8_4).unwrap());
        // C s_8_7: cast zx s_8_5 -> i
        let s_8_7: i128 = (i128::try_from(s_8_5).unwrap());
        // D s_8_8: read-var baseshadow#5283:bv
        let s_8_8: Bits = fn_state.baseshadow_5283;
        // D s_8_9: call Elem_read(s_8_8, s_8_6, s_8_7)
        let s_8_9: Bits = Elem_read(state, tracer, s_8_8, s_8_6, s_8_7);
        // D s_8_10: cast reint s_8_9 -> u64
        let s_8_10: u64 = (s_8_9.value() as u64);
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 64u16);
        // D s_8_12: read-var offsetshadow#5282:u64
        let s_8_12: u64 = fn_state.offsetshadow_5282;
        // D s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 64u16);
        // D s_8_14: add s_8_11 s_8_13
        let s_8_14: Bits = (s_8_11 + s_8_13);
        // D s_8_15: cast reint s_8_14 -> u64
        let s_8_15: u64 = (s_8_14.value() as u64);
        // C s_8_16: const #16s : i
        let s_8_16: i128 = 16;
        // D s_8_17: read-var accdesc:struct
        let s_8_17: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_18: call Mem_read(s_8_15, s_8_16, s_8_17)
        let s_8_18: Bits = Mem_read(state, tracer, s_8_15, s_8_16, s_8_17);
        // D s_8_19: write-var gs#812669 <= s_8_18
        fn_state.gs_812669 = s_8_18;
        // N s_8_20: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#812669:bv
        let s_9_0: Bits = fn_state.gs_812669;
        // D s_9_1: cast reint s_9_0 -> u128
        let s_9_1: u128 = (s_9_0.value() as u128);
        // D s_9_2: read-var e:i64
        let s_9_2: i64 = fn_state.e;
        // D s_9_3: cast zx s_9_2 -> i
        let s_9_3: i128 = (i128::try_from(s_9_2).unwrap());
        // C s_9_4: const #128s : i64
        let s_9_4: i64 = 128;
        // C s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: cast zx s_9_1 -> bv
        let s_9_6: Bits = Bits::new(s_9_1 as u128, 128u16);
        // D s_9_7: read-var result:bv
        let s_9_7: Bits = fn_state.result;
        // D s_9_8: call Elem_set(s_9_7, s_9_3, s_9_5, s_9_6)
        let s_9_8: Bits = Elem_set(state, tracer, s_9_7, s_9_3, s_9_5, s_9_6);
        // D s_9_9: write-var result <= s_9_8
        fn_state.result = s_9_8;
        // N s_9_10: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#5281:i64
        let s_10_0: i64 = fn_state.VLshadow_5281;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var t:i64
        let s_10_3: i64 = fn_state.t;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: read-var result:bv
        let s_10_6: Bits = fn_state.result;
        // D s_10_7: call Z_set(s_10_4, s_10_5, s_10_6)
        let s_10_7: () = Z_set(state, tracer, s_10_4, s_10_5, s_10_6);
        // N s_10_8: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var VLshadow#5281:i64
        let s_11_0: i64 = fn_state.VLshadow_5281;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: cast reint s_11_1 -> i64
        let s_11_2: i64 = (s_11_1 as i64);
        // D s_11_3: read-var n:i64
        let s_11_3: i64 = fn_state.n;
        // D s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // D s_11_5: cast zx s_11_2 -> i
        let s_11_5: i128 = (i128::try_from(s_11_2).unwrap());
        // D s_11_6: call Z_read(s_11_4, s_11_5)
        let s_11_6: Bits = Z_read(state, tracer, s_11_4, s_11_5);
        // D s_11_7: write-var base <= s_11_6
        fn_state.base = s_11_6;
        // C s_11_8: const #64s : i64
        let s_11_8: i64 = 64;
        // D s_11_9: read-var m:i64
        let s_11_9: i64 = fn_state.m;
        // D s_11_10: cast zx s_11_9 -> i
        let s_11_10: i128 = (i128::try_from(s_11_9).unwrap());
        // D s_11_11: call X_read(s_11_10, s_11_8)
        let s_11_11: Bits = X_read(state, tracer, s_11_10, s_11_8);
        // D s_11_12: cast reint s_11_11 -> u64
        let s_11_12: u64 = (s_11_11.value() as u64);
        // D s_11_13: write-var offset <= s_11_12
        fn_state.offset = s_11_12;
        // N s_11_14: jump b3
        return block_3(state, tracer, fn_state);
    }
}
