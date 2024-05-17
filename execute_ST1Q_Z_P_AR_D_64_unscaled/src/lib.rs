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
use AnyActiveElement::*;
use ActivePredicateElement::*;
use P_read::*;
use X_read::*;
use Elem_read::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use Mem_set::*;
use common::*;
pub fn execute_ST1Q_Z_P_AR_D_64_unscaled<T: Tracer>(
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
        baseshadow_5288: Bits,
        e: i64,
        base: Bits,
        VLshadow_5285: i64,
        accdesc: ProductType9878976b5bcce9c9,
        offsetshadow_5287: u64,
        elements: i64,
        offset: u64,
        gs_249503: i64,
        srcshadow_5286: Bits,
        src: Bits,
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
        // D s_0_3: write-var VLshadow#5285 <= s_0_2
        fn_state.VLshadow_5285 = s_0_2;
        // C s_0_4: const #8s : i
        let s_0_4: i128 = 8;
        // D s_0_5: read-var VLshadow#5285:i64
        let s_0_5: i64 = fn_state.VLshadow_5285;
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
        // D s_1_1: read-var VLshadow#5285:i64
        let s_1_1: i64 = fn_state.VLshadow_5285;
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
        // C s_1_14: const #1u : u32
        let s_1_14: u32 = 1;
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
        // N s_1_23: branch s_1_22 b10 b2
        if s_1_22 {
            return block_10(state, tracer, fn_state);
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
        // D s_3_0: read-var src:bv
        let s_3_0: Bits = fn_state.src;
        // D s_3_1: write-var srcshadow#5286 <= s_3_0
        fn_state.srcshadow_5286 = s_3_0;
        // D s_3_2: read-var offset:u64
        let s_3_2: u64 = fn_state.offset;
        // D s_3_3: write-var offsetshadow#5287 <= s_3_2
        fn_state.offsetshadow_5287 = s_3_2;
        // D s_3_4: read-var base:bv
        let s_3_4: Bits = fn_state.base;
        // D s_3_5: write-var baseshadow#5288 <= s_3_4
        fn_state.baseshadow_5288 = s_3_4;
        // C s_3_6: const #0s : i64
        let s_3_6: i64 = 0;
        // C s_3_7: const #1s : i
        let s_3_7: i128 = 1;
        // D s_3_8: read-var elements:i64
        let s_3_8: i64 = fn_state.elements;
        // D s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_10: sub s_3_9 s_3_7
        let s_3_10: i128 = ((s_3_9) - (s_3_7));
        // D s_3_11: cast reint s_3_10 -> i64
        let s_3_11: i64 = (s_3_10 as i64);
        // D s_3_12: write-var gs#249503 <= s_3_11
        fn_state.gs_249503 = s_3_11;
        // D s_3_13: write-var e <= s_3_6
        fn_state.e = s_3_6;
        // N s_3_14: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#249503:i64
        let s_4_1: i64 = fn_state.gs_249503;
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
        // D s_8_8: read-var baseshadow#5288:bv
        let s_8_8: Bits = fn_state.baseshadow_5288;
        // D s_8_9: call Elem_read(s_8_8, s_8_6, s_8_7)
        let s_8_9: Bits = Elem_read(state, tracer, s_8_8, s_8_6, s_8_7);
        // D s_8_10: cast reint s_8_9 -> u64
        let s_8_10: u64 = (s_8_9.value() as u64);
        // D s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 64u16);
        // D s_8_12: read-var offsetshadow#5287:u64
        let s_8_12: u64 = fn_state.offsetshadow_5287;
        // D s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 64u16);
        // D s_8_14: add s_8_11 s_8_13
        let s_8_14: Bits = (s_8_11 + s_8_13);
        // D s_8_15: cast reint s_8_14 -> u64
        let s_8_15: u64 = (s_8_14.value() as u64);
        // C s_8_16: const #128s : i64
        let s_8_16: i64 = 128;
        // D s_8_17: read-var e:i64
        let s_8_17: i64 = fn_state.e;
        // D s_8_18: cast zx s_8_17 -> i
        let s_8_18: i128 = (i128::try_from(s_8_17).unwrap());
        // C s_8_19: cast zx s_8_16 -> i
        let s_8_19: i128 = (i128::try_from(s_8_16).unwrap());
        // D s_8_20: read-var srcshadow#5286:bv
        let s_8_20: Bits = fn_state.srcshadow_5286;
        // D s_8_21: call Elem_read(s_8_20, s_8_18, s_8_19)
        let s_8_21: Bits = Elem_read(state, tracer, s_8_20, s_8_18, s_8_19);
        // D s_8_22: cast reint s_8_21 -> u128
        let s_8_22: u128 = (s_8_21.value() as u128);
        // C s_8_23: const #16s : i
        let s_8_23: i128 = 16;
        // D s_8_24: cast zx s_8_22 -> bv
        let s_8_24: Bits = Bits::new(s_8_22 as u128, 128u16);
        // D s_8_25: read-var accdesc:struct
        let s_8_25: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_26: call Mem_set(s_8_15, s_8_23, s_8_25, s_8_24)
        let s_8_26: () = Mem_set(state, tracer, s_8_15, s_8_23, s_8_25, s_8_24);
        // N s_8_27: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#5285:i64
        let s_10_0: i64 = fn_state.VLshadow_5285;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var n:i64
        let s_10_3: i64 = fn_state.n;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: call Z_read(s_10_4, s_10_5)
        let s_10_6: Bits = Z_read(state, tracer, s_10_4, s_10_5);
        // D s_10_7: write-var base <= s_10_6
        fn_state.base = s_10_6;
        // C s_10_8: const #64s : i64
        let s_10_8: i64 = 64;
        // D s_10_9: read-var m:i64
        let s_10_9: i64 = fn_state.m;
        // D s_10_10: cast zx s_10_9 -> i
        let s_10_10: i128 = (i128::try_from(s_10_9).unwrap());
        // D s_10_11: call X_read(s_10_10, s_10_8)
        let s_10_11: Bits = X_read(state, tracer, s_10_10, s_10_8);
        // D s_10_12: cast reint s_10_11 -> u64
        let s_10_12: u64 = (s_10_11.value() as u64);
        // D s_10_13: write-var offset <= s_10_12
        fn_state.offset = s_10_12;
        // D s_10_14: read-var VLshadow#5285:i64
        let s_10_14: i64 = fn_state.VLshadow_5285;
        // D s_10_15: cast zx s_10_14 -> i
        let s_10_15: i128 = (i128::try_from(s_10_14).unwrap());
        // D s_10_16: cast reint s_10_15 -> i64
        let s_10_16: i64 = (s_10_15 as i64);
        // D s_10_17: read-var t:i64
        let s_10_17: i64 = fn_state.t;
        // D s_10_18: cast zx s_10_17 -> i
        let s_10_18: i128 = (i128::try_from(s_10_17).unwrap());
        // D s_10_19: cast zx s_10_16 -> i
        let s_10_19: i128 = (i128::try_from(s_10_16).unwrap());
        // D s_10_20: call Z_read(s_10_18, s_10_19)
        let s_10_20: Bits = Z_read(state, tracer, s_10_18, s_10_19);
        // D s_10_21: write-var src <= s_10_20
        fn_state.src = s_10_20;
        // N s_10_22: jump b3
        return block_3(state, tracer, fn_state);
    }
}
