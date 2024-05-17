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
pub fn execute_STNT1B_Z_P_AR_D_64_unscaled<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    esize: i64,
    g: i64,
    m: i64,
    msize: i64,
    n: i64,
    t: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        gs_249207: i64,
        offsetshadow_5263: u64,
        base: Bits,
        mbytes: i64,
        accdesc: ProductType9878976b5bcce9c9,
        srcshadow_5262: Bits,
        elements: i64,
        offset: u64,
        VLshadow_5261: i64,
        baseshadow_5264: Bits,
        src: Bits,
        mask: Bits,
        VLshadow_5260: i64,
        VL: i64,
        esize: i64,
        g: i64,
        m: i64,
        msize: i64,
        n: i64,
        t: i64,
    }
    let fn_state = FunctionState {
        VL,
        esize,
        g,
        m,
        msize,
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
        // D s_0_3: write-var VLshadow#5260 <= s_0_2
        fn_state.VLshadow_5260 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckNonStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#5260:i64
        let s_1_0: i64 = fn_state.VLshadow_5260;
        // D s_1_1: write-var VLshadow#5261 <= s_1_0
        fn_state.VLshadow_5261 = s_1_0;
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: read-var VLshadow#5261:i64
        let s_1_3: i64 = fn_state.VLshadow_5261;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#5261:i64
        let s_1_7: i64 = fn_state.VLshadow_5261;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: read-var esize:i64
        let s_1_9: i64 = fn_state.esize;
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
        // D s_1_20: write-var mask <= s_1_19
        fn_state.mask = s_1_19;
        // C s_1_21: const #8s : i
        let s_1_21: i128 = 8;
        // D s_1_22: read-var msize:i64
        let s_1_22: i64 = fn_state.msize;
        // D s_1_23: cast zx s_1_22 -> i
        let s_1_23: i128 = (i128::try_from(s_1_22).unwrap());
        // D s_1_24: div s_1_23 s_1_21
        let s_1_24: i128 = ((s_1_23) / (s_1_21));
        // D s_1_25: cast reint s_1_24 -> i64
        let s_1_25: i64 = (s_1_24 as i64);
        // D s_1_26: write-var mbytes <= s_1_25
        fn_state.mbytes = s_1_25;
        // C s_1_27: const #1u : u32
        let s_1_27: u32 = 1;
        // C s_1_28: const #1u : u8
        let s_1_28: bool = true;
        // C s_1_29: const #0u : u8
        let s_1_29: bool = false;
        // C s_1_30: const #1u : u8
        let s_1_30: bool = true;
        // S s_1_31: call CreateAccDescSVE(s_1_27, s_1_28, s_1_29, s_1_30)
        let s_1_31: ProductType9878976b5bcce9c9 = CreateAccDescSVE(
            state,
            tracer,
            s_1_27,
            s_1_28,
            s_1_29,
            s_1_30,
        );
        // D s_1_32: write-var accdesc <= s_1_31
        fn_state.accdesc = s_1_31;
        // D s_1_33: read-var esize:i64
        let s_1_33: i64 = fn_state.esize;
        // D s_1_34: cast zx s_1_33 -> i
        let s_1_34: i128 = (i128::try_from(s_1_33).unwrap());
        // D s_1_35: read-var mask:bv
        let s_1_35: Bits = fn_state.mask;
        // D s_1_36: call AnyActiveElement(s_1_35, s_1_34)
        let s_1_36: bool = AnyActiveElement(state, tracer, s_1_35, s_1_34);
        // N s_1_37: branch s_1_36 b10 b2
        if s_1_36 {
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
        // D s_3_1: write-var srcshadow#5262 <= s_3_0
        fn_state.srcshadow_5262 = s_3_0;
        // D s_3_2: read-var offset:u64
        let s_3_2: u64 = fn_state.offset;
        // D s_3_3: write-var offsetshadow#5263 <= s_3_2
        fn_state.offsetshadow_5263 = s_3_2;
        // D s_3_4: read-var base:bv
        let s_3_4: Bits = fn_state.base;
        // D s_3_5: write-var baseshadow#5264 <= s_3_4
        fn_state.baseshadow_5264 = s_3_4;
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
        // D s_3_12: write-var gs#249207 <= s_3_11
        fn_state.gs_249207 = s_3_11;
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
        // D s_4_1: read-var gs#249207:i64
        let s_4_1: i64 = fn_state.gs_249207;
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
        // D s_5_0: read-var e:i64
        let s_5_0: i64 = fn_state.e;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var esize:i64
        let s_5_2: i64 = fn_state.esize;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-var mask:bv
        let s_5_4: Bits = fn_state.mask;
        // D s_5_5: call ActivePredicateElement(s_5_4, s_5_1, s_5_3)
        let s_5_5: bool = ActivePredicateElement(state, tracer, s_5_4, s_5_1, s_5_3);
        // N s_5_6: branch s_5_5 b8 b6
        if s_5_5 {
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
        // D s_8_0: read-var esize:i64
        let s_8_0: i64 = fn_state.esize;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var e:i64
        let s_8_3: i64 = fn_state.e;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cast zx s_8_2 -> i
        let s_8_5: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_6: read-var baseshadow#5264:bv
        let s_8_6: Bits = fn_state.baseshadow_5264;
        // D s_8_7: call Elem_read(s_8_6, s_8_4, s_8_5)
        let s_8_7: Bits = Elem_read(state, tracer, s_8_6, s_8_4, s_8_5);
        // D s_8_8: cast reint s_8_7 -> u64
        let s_8_8: u64 = (s_8_7.value() as u64);
        // C s_8_9: const #64s : i
        let s_8_9: i128 = 64;
        // D s_8_10: cast zx s_8_8 -> bv
        let s_8_10: Bits = Bits::new(s_8_8 as u128, 64u16);
        // D s_8_11: bits-cast zx s_8_10 -> bv length s_8_9
        let s_8_11: Bits = s_8_10.zero_extend(s_8_9);
        // D s_8_12: cast reint s_8_11 -> u64
        let s_8_12: u64 = (s_8_11.value() as u64);
        // D s_8_13: cast zx s_8_12 -> bv
        let s_8_13: Bits = Bits::new(s_8_12 as u128, 64u16);
        // D s_8_14: read-var offsetshadow#5263:u64
        let s_8_14: u64 = fn_state.offsetshadow_5263;
        // D s_8_15: cast zx s_8_14 -> bv
        let s_8_15: Bits = Bits::new(s_8_14 as u128, 64u16);
        // D s_8_16: add s_8_13 s_8_15
        let s_8_16: Bits = (s_8_13 + s_8_15);
        // D s_8_17: cast reint s_8_16 -> u64
        let s_8_17: u64 = (s_8_16.value() as u64);
        // D s_8_18: read-var esize:i64
        let s_8_18: i64 = fn_state.esize;
        // D s_8_19: cast zx s_8_18 -> i
        let s_8_19: i128 = (i128::try_from(s_8_18).unwrap());
        // D s_8_20: cast reint s_8_19 -> i64
        let s_8_20: i64 = (s_8_19 as i64);
        // D s_8_21: read-var e:i64
        let s_8_21: i64 = fn_state.e;
        // D s_8_22: cast zx s_8_21 -> i
        let s_8_22: i128 = (i128::try_from(s_8_21).unwrap());
        // D s_8_23: cast zx s_8_20 -> i
        let s_8_23: i128 = (i128::try_from(s_8_20).unwrap());
        // D s_8_24: read-var srcshadow#5262:bv
        let s_8_24: Bits = fn_state.srcshadow_5262;
        // D s_8_25: call Elem_read(s_8_24, s_8_22, s_8_23)
        let s_8_25: Bits = Elem_read(state, tracer, s_8_24, s_8_22, s_8_23);
        // D s_8_26: cast reint s_8_25 -> u64
        let s_8_26: u64 = (s_8_25.value() as u64);
        // C s_8_27: const #0s : i
        let s_8_27: i128 = 0;
        // C s_8_28: const #8s : i
        let s_8_28: i128 = 8;
        // D s_8_29: cast zx s_8_26 -> bv
        let s_8_29: Bits = Bits::new(s_8_26 as u128, 64u16);
        // D s_8_30: bit-extract s_8_29 s_8_27 s_8_28
        let s_8_30: Bits = (Bits::new(
            ((s_8_29) >> (s_8_27)).value(),
            u16::try_from(s_8_28).unwrap(),
        ));
        // D s_8_31: cast reint s_8_30 -> u8
        let s_8_31: u8 = (s_8_30.value() as u8);
        // D s_8_32: read-var mbytes:i64
        let s_8_32: i64 = fn_state.mbytes;
        // D s_8_33: cast zx s_8_32 -> i
        let s_8_33: i128 = (i128::try_from(s_8_32).unwrap());
        // D s_8_34: cast zx s_8_31 -> bv
        let s_8_34: Bits = Bits::new(s_8_31 as u128, 8u16);
        // D s_8_35: read-var accdesc:struct
        let s_8_35: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_36: call Mem_set(s_8_17, s_8_33, s_8_35, s_8_34)
        let s_8_36: () = Mem_set(state, tracer, s_8_17, s_8_33, s_8_35, s_8_34);
        // N s_8_37: jump b7
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
        // D s_10_0: read-var VLshadow#5261:i64
        let s_10_0: i64 = fn_state.VLshadow_5261;
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
        // D s_10_14: read-var VLshadow#5261:i64
        let s_10_14: i64 = fn_state.VLshadow_5261;
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
