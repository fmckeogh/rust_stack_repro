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
use Q_read::*;
use Elem_set::*;
use CheckAdvSIMDEnabled::*;
use Q_set::*;
use D_set::*;
use Din_read::*;
use Elem_read::*;
use FPConvert__1::*;
use D_read::*;
use Qin_read::*;
use common::*;
pub fn execute_aarch32_instrs_VCVT_hs_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d__arg: i64,
    elements: i64,
    half_to_single: bool,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_352312: u64,
        e: i64,
        gs_893266: Bits,
        gs_893251: Bits,
        d: i128,
        ga_352301: u128,
        ga_352304: i128,
        gs_307940: i64,
        d__arg: i64,
        elements: i64,
        half_to_single: bool,
        m: i64,
    }
    let fn_state = FunctionState {
        d__arg,
        elements,
        half_to_single,
        m,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var d__arg:i64
        let s_0_0: i64 = fn_state.d__arg;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: write-var d <= s_0_1
        fn_state.d = s_0_1;
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call CheckAdvSIMDEnabled(s_0_3)
        let s_0_4: () = CheckAdvSIMDEnabled(state, tracer, s_0_3);
        // N s_0_5: jump b1
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
        // D s_1_2: read-var elements:i64
        let s_1_2: i64 = fn_state.elements;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: sub s_1_3 s_1_1
        let s_1_4: i128 = ((s_1_3) - (s_1_1));
        // D s_1_5: cast reint s_1_4 -> i64
        let s_1_5: i64 = (s_1_4 as i64);
        // D s_1_6: write-var gs#307940 <= s_1_5
        fn_state.gs_307940 = s_1_5;
        // D s_1_7: write-var e <= s_1_0
        fn_state.e = s_1_0;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var e:i64
        let s_2_0: i64 = fn_state.e;
        // D s_2_1: read-var gs#307940:i64
        let s_2_1: i64 = fn_state.gs_307940;
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
        // D s_3_0: read-var half_to_single:u8
        let s_3_0: bool = fn_state.half_to_single;
        // N s_3_1: branch s_3_0 b7 b4
        if s_3_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var d:i
        let s_4_0: i128 = fn_state.d;
        // D s_4_1: call D_read(s_4_0)
        let s_4_1: u64 = D_read(state, tracer, s_4_0);
        // D s_4_2: write-var ga#352312 <= s_4_1
        fn_state.ga_352312 = s_4_1;
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // D s_4_4: read-var m:i64
        let s_4_4: i64 = fn_state.m;
        // D s_4_5: lsr s_4_4 s_4_3
        let s_4_5: i64 = s_4_4 >> s_4_3;
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: call Qin_read(s_4_6)
        let s_4_7: u128 = Qin_read(state, tracer, s_4_6);
        // C s_4_8: const #32s : i64
        let s_4_8: i64 = 32;
        // D s_4_9: cast zx s_4_7 -> bv
        let s_4_9: Bits = Bits::new(s_4_7 as u128, 128u16);
        // D s_4_10: read-var e:i64
        let s_4_10: i64 = fn_state.e;
        // D s_4_11: cast zx s_4_10 -> i
        let s_4_11: i128 = (i128::try_from(s_4_10).unwrap());
        // C s_4_12: cast zx s_4_8 -> i
        let s_4_12: i128 = (i128::try_from(s_4_8).unwrap());
        // D s_4_13: call Elem_read(s_4_9, s_4_11, s_4_12)
        let s_4_13: Bits = Elem_read(state, tracer, s_4_9, s_4_11, s_4_12);
        // D s_4_14: cast reint s_4_13 -> u32
        let s_4_14: u32 = (s_4_13.value() as u32);
        // C s_4_15: const #() : ()
        let s_4_15: () = ();
        // S s_4_16: call StandardFPSCRValue(s_4_15)
        let s_4_16: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_4_15,
        );
        // C s_4_17: const #16s : i64
        let s_4_17: i64 = 16;
        // D s_4_18: cast zx s_4_14 -> bv
        let s_4_18: Bits = Bits::new(s_4_14 as u128, 32u16);
        // D s_4_19: call FPConvert__1(s_4_18, s_4_16, s_4_17)
        let s_4_19: Bits = FPConvert__1(state, tracer, s_4_18, s_4_16, s_4_17);
        // D s_4_20: write-var gs#893251 <= s_4_19
        fn_state.gs_893251 = s_4_19;
        // N s_4_21: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#893251:bv
        let s_5_0: Bits = fn_state.gs_893251;
        // D s_5_1: cast reint s_5_0 -> u16
        let s_5_1: u16 = (s_5_0.value() as u16);
        // D s_5_2: read-var ga#352312:u64
        let s_5_2: u64 = fn_state.ga_352312;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 64u16);
        // D s_5_4: read-var e:i64
        let s_5_4: i64 = fn_state.e;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // C s_5_6: const #16s : i64
        let s_5_6: i64 = 16;
        // C s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // D s_5_8: cast zx s_5_1 -> bv
        let s_5_8: Bits = Bits::new(s_5_1 as u128, 16u16);
        // D s_5_9: call Elem_set(s_5_3, s_5_5, s_5_7, s_5_8)
        let s_5_9: Bits = Elem_set(state, tracer, s_5_3, s_5_5, s_5_7, s_5_8);
        // D s_5_10: cast reint s_5_9 -> u64
        let s_5_10: u64 = (s_5_9.value() as u64);
        // D s_5_11: read-var d:i
        let s_5_11: i128 = fn_state.d;
        // D s_5_12: call D_set(s_5_11, s_5_10)
        let s_5_12: () = D_set(state, tracer, s_5_11, s_5_10);
        // N s_5_13: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // C s_6_1: const #1s : i64
        let s_6_1: i64 = 1;
        // D s_6_2: add s_6_0 s_6_1
        let s_6_2: i64 = (s_6_0 + s_6_1);
        // D s_6_3: write-var e <= s_6_2
        fn_state.e = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #1s : i
        let s_7_0: i128 = 1;
        // D s_7_1: read-var d:i
        let s_7_1: i128 = fn_state.d;
        // D s_7_2: lsr s_7_1 s_7_0
        let s_7_2: i128 = s_7_1 >> s_7_0;
        // D s_7_3: write-var ga#352304 <= s_7_2
        fn_state.ga_352304 = s_7_2;
        // C s_7_4: const #1s : i
        let s_7_4: i128 = 1;
        // D s_7_5: read-var d:i
        let s_7_5: i128 = fn_state.d;
        // D s_7_6: lsr s_7_5 s_7_4
        let s_7_6: i128 = s_7_5 >> s_7_4;
        // D s_7_7: call Q_read(s_7_6)
        let s_7_7: u128 = Q_read(state, tracer, s_7_6);
        // D s_7_8: write-var ga#352301 <= s_7_7
        fn_state.ga_352301 = s_7_7;
        // D s_7_9: read-var m:i64
        let s_7_9: i64 = fn_state.m;
        // D s_7_10: cast zx s_7_9 -> i
        let s_7_10: i128 = (i128::try_from(s_7_9).unwrap());
        // D s_7_11: call Din_read(s_7_10)
        let s_7_11: u64 = Din_read(state, tracer, s_7_10);
        // C s_7_12: const #16s : i64
        let s_7_12: i64 = 16;
        // D s_7_13: cast zx s_7_11 -> bv
        let s_7_13: Bits = Bits::new(s_7_11 as u128, 64u16);
        // D s_7_14: read-var e:i64
        let s_7_14: i64 = fn_state.e;
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // C s_7_16: cast zx s_7_12 -> i
        let s_7_16: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_17: call Elem_read(s_7_13, s_7_15, s_7_16)
        let s_7_17: Bits = Elem_read(state, tracer, s_7_13, s_7_15, s_7_16);
        // D s_7_18: cast reint s_7_17 -> u16
        let s_7_18: u16 = (s_7_17.value() as u16);
        // C s_7_19: const #() : ()
        let s_7_19: () = ();
        // S s_7_20: call StandardFPSCRValue(s_7_19)
        let s_7_20: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_7_19,
        );
        // C s_7_21: const #32s : i64
        let s_7_21: i64 = 32;
        // D s_7_22: cast zx s_7_18 -> bv
        let s_7_22: Bits = Bits::new(s_7_18 as u128, 16u16);
        // D s_7_23: call FPConvert__1(s_7_22, s_7_20, s_7_21)
        let s_7_23: Bits = FPConvert__1(state, tracer, s_7_22, s_7_20, s_7_21);
        // D s_7_24: write-var gs#893266 <= s_7_23
        fn_state.gs_893266 = s_7_23;
        // N s_7_25: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#893266:bv
        let s_8_0: Bits = fn_state.gs_893266;
        // D s_8_1: cast reint s_8_0 -> u32
        let s_8_1: u32 = (s_8_0.value() as u32);
        // D s_8_2: read-var ga#352301:u128
        let s_8_2: u128 = fn_state.ga_352301;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 128u16);
        // D s_8_4: read-var e:i64
        let s_8_4: i64 = fn_state.e;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // C s_8_6: const #32s : i64
        let s_8_6: i64 = 32;
        // C s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: cast zx s_8_1 -> bv
        let s_8_8: Bits = Bits::new(s_8_1 as u128, 32u16);
        // D s_8_9: call Elem_set(s_8_3, s_8_5, s_8_7, s_8_8)
        let s_8_9: Bits = Elem_set(state, tracer, s_8_3, s_8_5, s_8_7, s_8_8);
        // D s_8_10: cast reint s_8_9 -> u128
        let s_8_10: u128 = (s_8_9.value() as u128);
        // D s_8_11: read-var ga#352304:i
        let s_8_11: i128 = fn_state.ga_352304;
        // D s_8_12: call Q_set(s_8_11, s_8_10)
        let s_8_12: () = Q_set(state, tracer, s_8_11, s_8_10);
        // N s_8_13: jump b6
        return block_6(state, tracer, fn_state);
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
