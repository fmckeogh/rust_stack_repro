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
use Elem_set::*;
use FPAdd::*;
use CheckAdvSIMDOrVFPEnabled::*;
use D_set::*;
use FPSCR_read::*;
use S_set::*;
use Elem_read::*;
use S_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VADD_f_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    advsimd: bool,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    regs: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        ga_350705: Bits,
        e: i64,
        esizeshadow_7349: i64,
        gs_306106: i64,
        gs_890515: Bits,
        gs_890502: Bits,
        ga_350704: i64,
        gs_890509: Bits,
        ga_350703: u64,
        d: i128,
        ga_350706: i128,
        gs_306112: i64,
        advsimd: bool,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        regs: i128,
    }
    let fn_state = FunctionState {
        advsimd,
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
        // D s_0_3: write-var esizeshadow#7349 <= s_0_2
        fn_state.esizeshadow_7349 = s_0_2;
        // D s_0_4: read-var d__arg:i64
        let s_0_4: i64 = fn_state.d__arg;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: write-var d <= s_0_5
        fn_state.d = s_0_5;
        // C s_0_7: const #1u : u8
        let s_0_7: bool = true;
        // D s_0_8: read-var advsimd:u8
        let s_0_8: bool = fn_state.advsimd;
        // D s_0_9: call CheckAdvSIMDOrVFPEnabled(s_0_7, s_0_8)
        let s_0_9: () = CheckAdvSIMDOrVFPEnabled(state, tracer, s_0_7, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var advsimd:u8
        let s_1_0: bool = fn_state.advsimd;
        // N s_1_1: branch s_1_0 b12 b2
        if s_1_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var esizeshadow#7349:i64
        let s_2_0: i64 = fn_state.esizeshadow_7349;
        // C s_2_1: const #16s : i
        let s_2_1: i128 = 16;
        // D s_2_2: cast zx s_2_0 -> i
        let s_2_2: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_1
        let s_2_3: bool = ((s_2_2) == (s_2_1));
        // D s_2_4: not s_2_3
        let s_2_4: bool = !s_2_3;
        // N s_2_5: branch s_2_4 b5 b3
        if s_2_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var n:i64
        let s_3_0: i64 = fn_state.n;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call S_read(s_3_1)
        let s_3_2: u32 = S_read(state, tracer, s_3_1);
        // C s_3_3: const #0s : i
        let s_3_3: i128 = 0;
        // D s_3_4: cast zx s_3_2 -> bv
        let s_3_4: Bits = Bits::new(s_3_2 as u128, 32u16);
        // C s_3_5: const #1s : i64
        let s_3_5: i64 = 1;
        // C s_3_6: cast zx s_3_5 -> i
        let s_3_6: i128 = (i128::try_from(s_3_5).unwrap());
        // C s_3_7: const #15s : i
        let s_3_7: i128 = 15;
        // C s_3_8: add s_3_7 s_3_6
        let s_3_8: i128 = (s_3_7 + s_3_6);
        // D s_3_9: bit-extract s_3_4 s_3_3 s_3_8
        let s_3_9: Bits = (Bits::new(
            ((s_3_4) >> (s_3_3)).value(),
            u16::try_from(s_3_8).unwrap(),
        ));
        // D s_3_10: cast reint s_3_9 -> u16
        let s_3_10: u16 = (s_3_9.value() as u16);
        // D s_3_11: read-var m:i64
        let s_3_11: i64 = fn_state.m;
        // D s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // D s_3_13: call S_read(s_3_12)
        let s_3_13: u32 = S_read(state, tracer, s_3_12);
        // C s_3_14: const #0s : i
        let s_3_14: i128 = 0;
        // D s_3_15: cast zx s_3_13 -> bv
        let s_3_15: Bits = Bits::new(s_3_13 as u128, 32u16);
        // C s_3_16: const #1s : i64
        let s_3_16: i64 = 1;
        // C s_3_17: cast zx s_3_16 -> i
        let s_3_17: i128 = (i128::try_from(s_3_16).unwrap());
        // C s_3_18: const #15s : i
        let s_3_18: i128 = 15;
        // C s_3_19: add s_3_18 s_3_17
        let s_3_19: i128 = (s_3_18 + s_3_17);
        // D s_3_20: bit-extract s_3_15 s_3_14 s_3_19
        let s_3_20: Bits = (Bits::new(
            ((s_3_15) >> (s_3_14)).value(),
            u16::try_from(s_3_19).unwrap(),
        ));
        // D s_3_21: cast reint s_3_20 -> u16
        let s_3_21: u16 = (s_3_20.value() as u16);
        // C s_3_22: const #() : ()
        let s_3_22: () = ();
        // S s_3_23: call FPSCR_read(s_3_22)
        let s_3_23: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_3_22);
        // D s_3_24: cast zx s_3_10 -> bv
        let s_3_24: Bits = Bits::new(s_3_10 as u128, 16u16);
        // D s_3_25: cast zx s_3_21 -> bv
        let s_3_25: Bits = Bits::new(s_3_21 as u128, 16u16);
        // D s_3_26: call FPAdd(s_3_24, s_3_25, s_3_23)
        let s_3_26: Bits = FPAdd(state, tracer, s_3_24, s_3_25, s_3_23);
        // D s_3_27: write-var gs#890502 <= s_3_26
        fn_state.gs_890502 = s_3_26;
        // N s_3_28: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#890502:bv
        let s_4_0: Bits = fn_state.gs_890502;
        // D s_4_1: cast reint s_4_0 -> u16
        let s_4_1: u16 = (s_4_0.value() as u16);
        // C s_4_2: const #0u : u16
        let s_4_2: u16 = 0;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 16u16);
        // D s_4_4: cast zx s_4_1 -> bv
        let s_4_4: Bits = Bits::new(s_4_1 as u128, 16u16);
        // C s_4_5: cast reint s_4_3 -> u128
        let s_4_5: u128 = (s_4_3.value() as u128);
        // D s_4_6: size-of s_4_3
        let s_4_6: u16 = s_4_3.length();
        // D s_4_7: cast reint s_4_4 -> u128
        let s_4_7: u128 = (s_4_4.value() as u128);
        // D s_4_8: size-of s_4_4
        let s_4_8: u16 = s_4_4.length();
        // D s_4_9: lsl s_4_5 s_4_8
        let s_4_9: u128 = s_4_5 << s_4_8;
        // D s_4_10: or s_4_9 s_4_7
        let s_4_10: u128 = ((s_4_9) | (s_4_7));
        // D s_4_11: add s_4_6 s_4_8
        let s_4_11: u16 = (s_4_6 + s_4_8);
        // D s_4_12: create-bits s_4_10 s_4_11
        let s_4_12: Bits = Bits::new(s_4_10, s_4_11);
        // D s_4_13: cast reint s_4_12 -> u32
        let s_4_13: u32 = (s_4_12.value() as u32);
        // D s_4_14: read-var d:i
        let s_4_14: i128 = fn_state.d;
        // D s_4_15: call S_set(s_4_14, s_4_13)
        let s_4_15: () = S_set(state, tracer, s_4_14, s_4_13);
        // N s_4_16: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var esizeshadow#7349:i64
        let s_5_0: i64 = fn_state.esizeshadow_7349;
        // C s_5_1: const #32s : i
        let s_5_1: i128 = 32;
        // D s_5_2: cast zx s_5_0 -> i
        let s_5_2: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_3: cmp-eq s_5_2 s_5_1
        let s_5_3: bool = ((s_5_2) == (s_5_1));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
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
        // D s_6_0: read-var n:i64
        let s_6_0: i64 = fn_state.n;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call S_read(s_6_1)
        let s_6_2: u32 = S_read(state, tracer, s_6_1);
        // D s_6_3: read-var m:i64
        let s_6_3: i64 = fn_state.m;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: call S_read(s_6_4)
        let s_6_5: u32 = S_read(state, tracer, s_6_4);
        // C s_6_6: const #() : ()
        let s_6_6: () = ();
        // S s_6_7: call FPSCR_read(s_6_6)
        let s_6_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_6_6);
        // D s_6_8: cast zx s_6_2 -> bv
        let s_6_8: Bits = Bits::new(s_6_2 as u128, 32u16);
        // D s_6_9: cast zx s_6_5 -> bv
        let s_6_9: Bits = Bits::new(s_6_5 as u128, 32u16);
        // D s_6_10: call FPAdd(s_6_8, s_6_9, s_6_7)
        let s_6_10: Bits = FPAdd(state, tracer, s_6_8, s_6_9, s_6_7);
        // D s_6_11: write-var gs#890509 <= s_6_10
        fn_state.gs_890509 = s_6_10;
        // N s_6_12: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#890509:bv
        let s_7_0: Bits = fn_state.gs_890509;
        // D s_7_1: cast reint s_7_0 -> u32
        let s_7_1: u32 = (s_7_0.value() as u32);
        // D s_7_2: read-var d:i
        let s_7_2: i128 = fn_state.d;
        // D s_7_3: call S_set(s_7_2, s_7_1)
        let s_7_3: () = S_set(state, tracer, s_7_2, s_7_1);
        // N s_7_4: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esizeshadow#7349:i64
        let s_8_0: i64 = fn_state.esizeshadow_7349;
        // C s_8_1: const #64s : i
        let s_8_1: i128 = 64;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b11 b9
        if s_8_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var n:i64
        let s_9_0: i64 = fn_state.n;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call D_read(s_9_1)
        let s_9_2: u64 = D_read(state, tracer, s_9_1);
        // D s_9_3: read-var m:i64
        let s_9_3: i64 = fn_state.m;
        // D s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_5: call D_read(s_9_4)
        let s_9_5: u64 = D_read(state, tracer, s_9_4);
        // C s_9_6: const #() : ()
        let s_9_6: () = ();
        // S s_9_7: call FPSCR_read(s_9_6)
        let s_9_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_9_6);
        // D s_9_8: cast zx s_9_2 -> bv
        let s_9_8: Bits = Bits::new(s_9_2 as u128, 64u16);
        // D s_9_9: cast zx s_9_5 -> bv
        let s_9_9: Bits = Bits::new(s_9_5 as u128, 64u16);
        // D s_9_10: call FPAdd(s_9_8, s_9_9, s_9_7)
        let s_9_10: Bits = FPAdd(state, tracer, s_9_8, s_9_9, s_9_7);
        // D s_9_11: write-var gs#890515 <= s_9_10
        fn_state.gs_890515 = s_9_10;
        // N s_9_12: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#890515:bv
        let s_10_0: Bits = fn_state.gs_890515;
        // D s_10_1: cast reint s_10_0 -> u64
        let s_10_1: u64 = (s_10_0.value() as u64);
        // D s_10_2: read-var d:i
        let s_10_2: i128 = fn_state.d;
        // D s_10_3: call D_set(s_10_2, s_10_1)
        let s_10_3: () = D_set(state, tracer, s_10_2, s_10_1);
        // N s_10_4: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0s : i64
        let s_12_0: i64 = 0;
        // C s_12_1: const #1s : i
        let s_12_1: i128 = 1;
        // D s_12_2: read-var regs:i
        let s_12_2: i128 = fn_state.regs;
        // D s_12_3: sub s_12_2 s_12_1
        let s_12_3: i128 = ((s_12_2) - (s_12_1));
        // D s_12_4: cast reint s_12_3 -> i64
        let s_12_4: i64 = (s_12_3 as i64);
        // D s_12_5: write-var gs#306106 <= s_12_4
        fn_state.gs_306106 = s_12_4;
        // D s_12_6: write-var r <= s_12_0
        fn_state.r = s_12_0;
        // N s_12_7: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var r:i64
        let s_13_0: i64 = fn_state.r;
        // D s_13_1: read-var gs#306106:i64
        let s_13_1: i64 = fn_state.gs_306106;
        // D s_13_2: cmp-gt s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) > (s_13_1));
        // N s_13_3: branch s_13_2 b19 b14
        if s_13_2 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0s : i64
        let s_14_0: i64 = 0;
        // C s_14_1: const #1s : i
        let s_14_1: i128 = 1;
        // D s_14_2: read-var elements:i
        let s_14_2: i128 = fn_state.elements;
        // D s_14_3: sub s_14_2 s_14_1
        let s_14_3: i128 = ((s_14_2) - (s_14_1));
        // D s_14_4: cast reint s_14_3 -> i64
        let s_14_4: i64 = (s_14_3 as i64);
        // D s_14_5: write-var gs#306112 <= s_14_4
        fn_state.gs_306112 = s_14_4;
        // D s_14_6: write-var e <= s_14_0
        fn_state.e = s_14_0;
        // N s_14_7: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var e:i64
        let s_15_0: i64 = fn_state.e;
        // D s_15_1: read-var gs#306112:i64
        let s_15_1: i64 = fn_state.gs_306112;
        // D s_15_2: cmp-gt s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) > (s_15_1));
        // N s_15_3: branch s_15_2 b18 b16
        if s_15_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var r:i64
        let s_16_0: i64 = fn_state.r;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: read-var d:i
        let s_16_2: i128 = fn_state.d;
        // D s_16_3: add s_16_2 s_16_1
        let s_16_3: i128 = (s_16_2 + s_16_1);
        // D s_16_4: write-var ga#350706 <= s_16_3
        fn_state.ga_350706 = s_16_3;
        // D s_16_5: read-var r:i64
        let s_16_5: i64 = fn_state.r;
        // D s_16_6: cast zx s_16_5 -> i
        let s_16_6: i128 = (i128::try_from(s_16_5).unwrap());
        // D s_16_7: read-var d:i
        let s_16_7: i128 = fn_state.d;
        // D s_16_8: add s_16_7 s_16_6
        let s_16_8: i128 = (s_16_7 + s_16_6);
        // D s_16_9: call D_read(s_16_8)
        let s_16_9: u64 = D_read(state, tracer, s_16_8);
        // D s_16_10: write-var ga#350703 <= s_16_9
        fn_state.ga_350703 = s_16_9;
        // D s_16_11: read-var esizeshadow#7349:i64
        let s_16_11: i64 = fn_state.esizeshadow_7349;
        // D s_16_12: cast zx s_16_11 -> i
        let s_16_12: i128 = (i128::try_from(s_16_11).unwrap());
        // D s_16_13: cast reint s_16_12 -> i64
        let s_16_13: i64 = (s_16_12 as i64);
        // D s_16_14: write-var ga#350704 <= s_16_13
        fn_state.ga_350704 = s_16_13;
        // D s_16_15: read-var n:i64
        let s_16_15: i64 = fn_state.n;
        // D s_16_16: cast zx s_16_15 -> i
        let s_16_16: i128 = (i128::try_from(s_16_15).unwrap());
        // D s_16_17: read-var r:i64
        let s_16_17: i64 = fn_state.r;
        // D s_16_18: cast zx s_16_17 -> i
        let s_16_18: i128 = (i128::try_from(s_16_17).unwrap());
        // D s_16_19: add s_16_16 s_16_18
        let s_16_19: i128 = (s_16_16 + s_16_18);
        // D s_16_20: call D_read(s_16_19)
        let s_16_20: u64 = D_read(state, tracer, s_16_19);
        // D s_16_21: read-var esizeshadow#7349:i64
        let s_16_21: i64 = fn_state.esizeshadow_7349;
        // D s_16_22: cast zx s_16_21 -> i
        let s_16_22: i128 = (i128::try_from(s_16_21).unwrap());
        // D s_16_23: cast reint s_16_22 -> i64
        let s_16_23: i64 = (s_16_22 as i64);
        // D s_16_24: cast zx s_16_20 -> bv
        let s_16_24: Bits = Bits::new(s_16_20 as u128, 64u16);
        // D s_16_25: read-var e:i64
        let s_16_25: i64 = fn_state.e;
        // D s_16_26: cast zx s_16_25 -> i
        let s_16_26: i128 = (i128::try_from(s_16_25).unwrap());
        // D s_16_27: cast zx s_16_23 -> i
        let s_16_27: i128 = (i128::try_from(s_16_23).unwrap());
        // D s_16_28: call Elem_read(s_16_24, s_16_26, s_16_27)
        let s_16_28: Bits = Elem_read(state, tracer, s_16_24, s_16_26, s_16_27);
        // D s_16_29: read-var m:i64
        let s_16_29: i64 = fn_state.m;
        // D s_16_30: cast zx s_16_29 -> i
        let s_16_30: i128 = (i128::try_from(s_16_29).unwrap());
        // D s_16_31: read-var r:i64
        let s_16_31: i64 = fn_state.r;
        // D s_16_32: cast zx s_16_31 -> i
        let s_16_32: i128 = (i128::try_from(s_16_31).unwrap());
        // D s_16_33: add s_16_30 s_16_32
        let s_16_33: i128 = (s_16_30 + s_16_32);
        // D s_16_34: call D_read(s_16_33)
        let s_16_34: u64 = D_read(state, tracer, s_16_33);
        // D s_16_35: read-var esizeshadow#7349:i64
        let s_16_35: i64 = fn_state.esizeshadow_7349;
        // D s_16_36: cast zx s_16_35 -> i
        let s_16_36: i128 = (i128::try_from(s_16_35).unwrap());
        // D s_16_37: cast reint s_16_36 -> i64
        let s_16_37: i64 = (s_16_36 as i64);
        // D s_16_38: cast zx s_16_34 -> bv
        let s_16_38: Bits = Bits::new(s_16_34 as u128, 64u16);
        // D s_16_39: read-var e:i64
        let s_16_39: i64 = fn_state.e;
        // D s_16_40: cast zx s_16_39 -> i
        let s_16_40: i128 = (i128::try_from(s_16_39).unwrap());
        // D s_16_41: cast zx s_16_37 -> i
        let s_16_41: i128 = (i128::try_from(s_16_37).unwrap());
        // D s_16_42: call Elem_read(s_16_38, s_16_40, s_16_41)
        let s_16_42: Bits = Elem_read(state, tracer, s_16_38, s_16_40, s_16_41);
        // C s_16_43: const #() : ()
        let s_16_43: () = ();
        // S s_16_44: call StandardFPSCRValue(s_16_43)
        let s_16_44: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_16_43,
        );
        // D s_16_45: call FPAdd(s_16_28, s_16_42, s_16_44)
        let s_16_45: Bits = FPAdd(state, tracer, s_16_28, s_16_42, s_16_44);
        // D s_16_46: write-var ga#350705 <= s_16_45
        fn_state.ga_350705 = s_16_45;
        // N s_16_47: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var ga#350703:u64
        let s_17_0: u64 = fn_state.ga_350703;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 64u16);
        // D s_17_2: read-var e:i64
        let s_17_2: i64 = fn_state.e;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: read-var ga#350704:i64
        let s_17_4: i64 = fn_state.ga_350704;
        // D s_17_5: cast zx s_17_4 -> i
        let s_17_5: i128 = (i128::try_from(s_17_4).unwrap());
        // D s_17_6: read-var ga#350705:bv
        let s_17_6: Bits = fn_state.ga_350705;
        // D s_17_7: call Elem_set(s_17_1, s_17_3, s_17_5, s_17_6)
        let s_17_7: Bits = Elem_set(state, tracer, s_17_1, s_17_3, s_17_5, s_17_6);
        // D s_17_8: cast reint s_17_7 -> u64
        let s_17_8: u64 = (s_17_7.value() as u64);
        // D s_17_9: read-var ga#350706:i
        let s_17_9: i128 = fn_state.ga_350706;
        // D s_17_10: call D_set(s_17_9, s_17_8)
        let s_17_10: () = D_set(state, tracer, s_17_9, s_17_8);
        // D s_17_11: read-var e:i64
        let s_17_11: i64 = fn_state.e;
        // C s_17_12: const #1s : i64
        let s_17_12: i64 = 1;
        // D s_17_13: add s_17_11 s_17_12
        let s_17_13: i64 = (s_17_11 + s_17_12);
        // D s_17_14: write-var e <= s_17_13
        fn_state.e = s_17_13;
        // N s_17_15: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var r:i64
        let s_18_0: i64 = fn_state.r;
        // C s_18_1: const #1s : i64
        let s_18_1: i64 = 1;
        // D s_18_2: add s_18_0 s_18_1
        let s_18_2: i64 = (s_18_0 + s_18_1);
        // D s_18_3: write-var r <= s_18_2
        fn_state.r = s_18_2;
        // N s_18_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_19_0: return
        return;
    }
}
