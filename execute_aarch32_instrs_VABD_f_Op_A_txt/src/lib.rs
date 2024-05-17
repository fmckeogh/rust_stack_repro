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
use StandardFPSCRValue::*;
use FPAbs::*;
use Elem_set::*;
use CheckAdvSIMDEnabled::*;
use FPSub::*;
use D_set::*;
use Elem_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VABD_f_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i64,
    esize: i64,
    m: i64,
    n: i64,
    regs: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        ga_350336: Bits,
        e: i64,
        gs_305637: i64,
        ga_350337: i128,
        gs_305643: i64,
        d: i128,
        ga_350334: u64,
        ga_350335: i64,
        esizeshadow_7331: i64,
        ga_350333: Bits,
        d__arg: i64,
        elements: i64,
        esize: i64,
        m: i64,
        n: i64,
        regs: i64,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        esize,
        m,
        n,
        regs,
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
        // D s_0_3: write-var esizeshadow#7331 <= s_0_2
        fn_state.esizeshadow_7331 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call CheckAdvSIMDEnabled(s_0_7)
        let s_0_8: () = CheckAdvSIMDEnabled(state, tracer, s_0_7);
        // N s_0_9: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: read-var regs:i64
        let s_1_2: i64 = fn_state.regs;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#305637 <= s_1_5
        fn_state.gs_305637 = s_1_5;
        // D s_1_7: write-var r <= s_1_0
        fn_state.r = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var r:i64
        let s_2_0: i64 = fn_state.r;
        // D s_2_1: read-var gs#305637:i64
        let s_2_1: i64 = fn_state.gs_305637;
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
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: read-var elements:i64
        let s_3_2: i64 = fn_state.elements;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: sub s_3_3 s_3_1
        let s_3_4: i128 = ((s_3_3) - (s_3_1));
        // D s_3_5: cast reint s_3_4 -> i64
        let s_3_5: i64 = (s_3_4 as i64);
        // D s_3_6: write-var gs#305643 <= s_3_5
        fn_state.gs_305643 = s_3_5;
        // D s_3_7: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#305643:i64
        let s_4_1: i64 = fn_state.gs_305643;
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
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: read-var r:i64
        let s_5_2: i64 = fn_state.r;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: add s_5_1 s_5_3
        let s_5_4: i128 = (s_5_1 + s_5_3);
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: cast zx s_5_5 -> i
        let s_5_6: i128 = (i128::try_from(s_5_5).unwrap());
        // D s_5_7: call D_read(s_5_6)
        let s_5_7: u64 = D_read(state, tracer, s_5_6);
        // D s_5_8: read-var esizeshadow#7331:i64
        let s_5_8: i64 = fn_state.esizeshadow_7331;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: cast reint s_5_9 -> i64
        let s_5_10: i64 = (s_5_9 as i64);
        // D s_5_11: cast zx s_5_7 -> bv
        let s_5_11: Bits = Bits::new(s_5_7 as u128, 64u16);
        // D s_5_12: read-var e:i64
        let s_5_12: i64 = fn_state.e;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // D s_5_14: cast zx s_5_10 -> i
        let s_5_14: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_15: call Elem_read(s_5_11, s_5_13, s_5_14)
        let s_5_15: Bits = Elem_read(state, tracer, s_5_11, s_5_13, s_5_14);
        // D s_5_16: read-var m:i64
        let s_5_16: i64 = fn_state.m;
        // D s_5_17: cast zx s_5_16 -> i
        let s_5_17: i128 = (i128::try_from(s_5_16).unwrap());
        // D s_5_18: read-var r:i64
        let s_5_18: i64 = fn_state.r;
        // D s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // D s_5_20: add s_5_17 s_5_19
        let s_5_20: i128 = (s_5_17 + s_5_19);
        // D s_5_21: cast reint s_5_20 -> i64
        let s_5_21: i64 = (s_5_20 as i64);
        // D s_5_22: cast zx s_5_21 -> i
        let s_5_22: i128 = (i128::try_from(s_5_21).unwrap());
        // D s_5_23: call D_read(s_5_22)
        let s_5_23: u64 = D_read(state, tracer, s_5_22);
        // D s_5_24: read-var esizeshadow#7331:i64
        let s_5_24: i64 = fn_state.esizeshadow_7331;
        // D s_5_25: cast zx s_5_24 -> i
        let s_5_25: i128 = (i128::try_from(s_5_24).unwrap());
        // D s_5_26: cast reint s_5_25 -> i64
        let s_5_26: i64 = (s_5_25 as i64);
        // D s_5_27: cast zx s_5_23 -> bv
        let s_5_27: Bits = Bits::new(s_5_23 as u128, 64u16);
        // D s_5_28: read-var e:i64
        let s_5_28: i64 = fn_state.e;
        // D s_5_29: cast zx s_5_28 -> i
        let s_5_29: i128 = (i128::try_from(s_5_28).unwrap());
        // D s_5_30: cast zx s_5_26 -> i
        let s_5_30: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_31: call Elem_read(s_5_27, s_5_29, s_5_30)
        let s_5_31: Bits = Elem_read(state, tracer, s_5_27, s_5_29, s_5_30);
        // D s_5_32: read-var r:i64
        let s_5_32: i64 = fn_state.r;
        // D s_5_33: cast zx s_5_32 -> i
        let s_5_33: i128 = (i128::try_from(s_5_32).unwrap());
        // D s_5_34: read-var d:i
        let s_5_34: i128 = fn_state.d;
        // D s_5_35: add s_5_34 s_5_33
        let s_5_35: i128 = (s_5_34 + s_5_33);
        // D s_5_36: write-var ga#350337 <= s_5_35
        fn_state.ga_350337 = s_5_35;
        // D s_5_37: read-var r:i64
        let s_5_37: i64 = fn_state.r;
        // D s_5_38: cast zx s_5_37 -> i
        let s_5_38: i128 = (i128::try_from(s_5_37).unwrap());
        // D s_5_39: read-var d:i
        let s_5_39: i128 = fn_state.d;
        // D s_5_40: add s_5_39 s_5_38
        let s_5_40: i128 = (s_5_39 + s_5_38);
        // D s_5_41: call D_read(s_5_40)
        let s_5_41: u64 = D_read(state, tracer, s_5_40);
        // D s_5_42: write-var ga#350334 <= s_5_41
        fn_state.ga_350334 = s_5_41;
        // D s_5_43: read-var esizeshadow#7331:i64
        let s_5_43: i64 = fn_state.esizeshadow_7331;
        // D s_5_44: cast zx s_5_43 -> i
        let s_5_44: i128 = (i128::try_from(s_5_43).unwrap());
        // D s_5_45: cast reint s_5_44 -> i64
        let s_5_45: i64 = (s_5_44 as i64);
        // D s_5_46: write-var ga#350335 <= s_5_45
        fn_state.ga_350335 = s_5_45;
        // C s_5_47: const #() : ()
        let s_5_47: () = ();
        // S s_5_48: call StandardFPSCRValue(s_5_47)
        let s_5_48: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_5_47,
        );
        // D s_5_49: call FPSub(s_5_15, s_5_31, s_5_48)
        let s_5_49: Bits = FPSub(state, tracer, s_5_15, s_5_31, s_5_48);
        // D s_5_50: write-var ga#350333 <= s_5_49
        fn_state.ga_350333 = s_5_49;
        // N s_5_51: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#350333:bv
        let s_6_0: Bits = fn_state.ga_350333;
        // D s_6_1: call FPAbs(s_6_0)
        let s_6_1: Bits = FPAbs(state, tracer, s_6_0);
        // D s_6_2: write-var ga#350336 <= s_6_1
        fn_state.ga_350336 = s_6_1;
        // N s_6_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#350334:u64
        let s_7_0: u64 = fn_state.ga_350334;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 64u16);
        // D s_7_2: read-var e:i64
        let s_7_2: i64 = fn_state.e;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var ga#350335:i64
        let s_7_4: i64 = fn_state.ga_350335;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_6: read-var ga#350336:bv
        let s_7_6: Bits = fn_state.ga_350336;
        // D s_7_7: call Elem_set(s_7_1, s_7_3, s_7_5, s_7_6)
        let s_7_7: Bits = Elem_set(state, tracer, s_7_1, s_7_3, s_7_5, s_7_6);
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7.value() as u64);
        // D s_7_9: read-var ga#350337:i
        let s_7_9: i128 = fn_state.ga_350337;
        // D s_7_10: call D_set(s_7_9, s_7_8)
        let s_7_10: () = D_set(state, tracer, s_7_9, s_7_8);
        // D s_7_11: read-var e:i64
        let s_7_11: i64 = fn_state.e;
        // C s_7_12: const #1s : i64
        let s_7_12: i64 = 1;
        // D s_7_13: add s_7_11 s_7_12
        let s_7_13: i64 = (s_7_11 + s_7_12);
        // D s_7_14: write-var e <= s_7_13
        fn_state.e = s_7_13;
        // N s_7_15: jump b4
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
        // N s_9_0: return
        return;
    }
}
