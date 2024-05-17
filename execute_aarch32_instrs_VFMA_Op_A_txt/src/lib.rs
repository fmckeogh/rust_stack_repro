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
use CheckAdvSIMDOrVFPEnabled::*;
use FPNeg::*;
use FPSCR_read::*;
use D_set::*;
use S_set::*;
use FPMulAdd::*;
use S_read::*;
use D_read::*;
use Elem_read::*;
use common::*;
pub fn execute_aarch32_instrs_VFMA_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    advsimd: bool,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    n: i64,
    op1_neg: bool,
    regs: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        e: i64,
        ga_352932: i128,
        op64shadow_7456: u64,
        ga_352929: u64,
        ga_352931: Bits,
        op16shadow_7454: u16,
        gs_308902: i64,
        gs_894444: Bits,
        esizeshadow_7453: i64,
        gs_894429: Bits,
        ga_352930: i64,
        gs_894413: Bits,
        gs_894422: Bits,
        gs_308908: i64,
        d: i128,
        gs_894439: Bits,
        op1: Bits,
        op32shadow_7455: u32,
        gs_894434: Bits,
        advsimd: bool,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        op1_neg: bool,
        regs: i128,
    }
    let fn_state = FunctionState {
        advsimd,
        d__arg,
        elements,
        esize,
        m,
        n,
        op1_neg,
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
        // D s_0_3: write-var esizeshadow#7453 <= s_0_2
        fn_state.esizeshadow_7453 = s_0_2;
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
        // N s_1_1: branch s_1_0 b24 b2
        if s_1_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var esizeshadow#7453:i64
        let s_2_0: i64 = fn_state.esizeshadow_7453;
        // C s_2_1: const #16s : i
        let s_2_1: i128 = 16;
        // D s_2_2: cast zx s_2_0 -> i
        let s_2_2: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_1
        let s_2_3: bool = ((s_2_2) == (s_2_1));
        // D s_2_4: not s_2_3
        let s_2_4: bool = !s_2_3;
        // N s_2_5: branch s_2_4 b9 b3
        if s_2_4 {
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
        // D s_3_0: read-var op1_neg:u8
        let s_3_0: bool = fn_state.op1_neg;
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
        // D s_4_0: read-var n:i64
        let s_4_0: i64 = fn_state.n;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: call S_read(s_4_1)
        let s_4_2: u32 = S_read(state, tracer, s_4_1);
        // C s_4_3: const #0s : i
        let s_4_3: i128 = 0;
        // D s_4_4: cast zx s_4_2 -> bv
        let s_4_4: Bits = Bits::new(s_4_2 as u128, 32u16);
        // C s_4_5: const #1s : i64
        let s_4_5: i64 = 1;
        // C s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // C s_4_7: const #15s : i
        let s_4_7: i128 = 15;
        // C s_4_8: add s_4_7 s_4_6
        let s_4_8: i128 = (s_4_7 + s_4_6);
        // D s_4_9: bit-extract s_4_4 s_4_3 s_4_8
        let s_4_9: Bits = (Bits::new(
            ((s_4_4) >> (s_4_3)).value(),
            u16::try_from(s_4_8).unwrap(),
        ));
        // D s_4_10: cast reint s_4_9 -> u16
        let s_4_10: u16 = (s_4_9.value() as u16);
        // D s_4_11: write-var op16shadow#7454 <= s_4_10
        fn_state.op16shadow_7454 = s_4_10;
        // N s_4_12: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var d:i
        let s_5_0: i128 = fn_state.d;
        // D s_5_1: call S_read(s_5_0)
        let s_5_1: u32 = S_read(state, tracer, s_5_0);
        // C s_5_2: const #0s : i
        let s_5_2: i128 = 0;
        // D s_5_3: cast zx s_5_1 -> bv
        let s_5_3: Bits = Bits::new(s_5_1 as u128, 32u16);
        // C s_5_4: const #1s : i64
        let s_5_4: i64 = 1;
        // C s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // C s_5_6: const #15s : i
        let s_5_6: i128 = 15;
        // C s_5_7: add s_5_6 s_5_5
        let s_5_7: i128 = (s_5_6 + s_5_5);
        // D s_5_8: bit-extract s_5_3 s_5_2 s_5_7
        let s_5_8: Bits = (Bits::new(
            ((s_5_3) >> (s_5_2)).value(),
            u16::try_from(s_5_7).unwrap(),
        ));
        // D s_5_9: cast reint s_5_8 -> u16
        let s_5_9: u16 = (s_5_8.value() as u16);
        // D s_5_10: read-var m:i64
        let s_5_10: i64 = fn_state.m;
        // D s_5_11: cast zx s_5_10 -> i
        let s_5_11: i128 = (i128::try_from(s_5_10).unwrap());
        // D s_5_12: call S_read(s_5_11)
        let s_5_12: u32 = S_read(state, tracer, s_5_11);
        // C s_5_13: const #0s : i
        let s_5_13: i128 = 0;
        // D s_5_14: cast zx s_5_12 -> bv
        let s_5_14: Bits = Bits::new(s_5_12 as u128, 32u16);
        // C s_5_15: const #1s : i64
        let s_5_15: i64 = 1;
        // C s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (i128::try_from(s_5_15).unwrap());
        // C s_5_17: const #15s : i
        let s_5_17: i128 = 15;
        // C s_5_18: add s_5_17 s_5_16
        let s_5_18: i128 = (s_5_17 + s_5_16);
        // D s_5_19: bit-extract s_5_14 s_5_13 s_5_18
        let s_5_19: Bits = (Bits::new(
            ((s_5_14) >> (s_5_13)).value(),
            u16::try_from(s_5_18).unwrap(),
        ));
        // D s_5_20: cast reint s_5_19 -> u16
        let s_5_20: u16 = (s_5_19.value() as u16);
        // C s_5_21: const #() : ()
        let s_5_21: () = ();
        // S s_5_22: call FPSCR_read(s_5_21)
        let s_5_22: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_5_21);
        // D s_5_23: cast zx s_5_9 -> bv
        let s_5_23: Bits = Bits::new(s_5_9 as u128, 16u16);
        // D s_5_24: read-var op16shadow#7454:u16
        let s_5_24: u16 = fn_state.op16shadow_7454;
        // D s_5_25: cast zx s_5_24 -> bv
        let s_5_25: Bits = Bits::new(s_5_24 as u128, 16u16);
        // D s_5_26: cast zx s_5_20 -> bv
        let s_5_26: Bits = Bits::new(s_5_20 as u128, 16u16);
        // D s_5_27: call FPMulAdd(s_5_23, s_5_25, s_5_26, s_5_22)
        let s_5_27: Bits = FPMulAdd(state, tracer, s_5_23, s_5_25, s_5_26, s_5_22);
        // D s_5_28: write-var gs#894422 <= s_5_27
        fn_state.gs_894422 = s_5_27;
        // N s_5_29: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#894422:bv
        let s_6_0: Bits = fn_state.gs_894422;
        // D s_6_1: cast reint s_6_0 -> u16
        let s_6_1: u16 = (s_6_0.value() as u16);
        // C s_6_2: const #0u : u16
        let s_6_2: u16 = 0;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 16u16);
        // D s_6_4: cast zx s_6_1 -> bv
        let s_6_4: Bits = Bits::new(s_6_1 as u128, 16u16);
        // C s_6_5: cast reint s_6_3 -> u128
        let s_6_5: u128 = (s_6_3.value() as u128);
        // D s_6_6: size-of s_6_3
        let s_6_6: u16 = s_6_3.length();
        // D s_6_7: cast reint s_6_4 -> u128
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
        // D s_6_13: cast reint s_6_12 -> u32
        let s_6_13: u32 = (s_6_12.value() as u32);
        // D s_6_14: read-var d:i
        let s_6_14: i128 = fn_state.d;
        // D s_6_15: call S_set(s_6_14, s_6_13)
        let s_6_15: () = S_set(state, tracer, s_6_14, s_6_13);
        // N s_6_16: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var n:i64
        let s_7_0: i64 = fn_state.n;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call S_read(s_7_1)
        let s_7_2: u32 = S_read(state, tracer, s_7_1);
        // C s_7_3: const #0s : i
        let s_7_3: i128 = 0;
        // D s_7_4: cast zx s_7_2 -> bv
        let s_7_4: Bits = Bits::new(s_7_2 as u128, 32u16);
        // C s_7_5: const #1s : i64
        let s_7_5: i64 = 1;
        // C s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // C s_7_7: const #15s : i
        let s_7_7: i128 = 15;
        // C s_7_8: add s_7_7 s_7_6
        let s_7_8: i128 = (s_7_7 + s_7_6);
        // D s_7_9: bit-extract s_7_4 s_7_3 s_7_8
        let s_7_9: Bits = (Bits::new(
            ((s_7_4) >> (s_7_3)).value(),
            u16::try_from(s_7_8).unwrap(),
        ));
        // D s_7_10: cast reint s_7_9 -> u16
        let s_7_10: u16 = (s_7_9.value() as u16);
        // D s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 16u16);
        // D s_7_12: call FPNeg(s_7_11)
        let s_7_12: Bits = FPNeg(state, tracer, s_7_11);
        // D s_7_13: write-var gs#894413 <= s_7_12
        fn_state.gs_894413 = s_7_12;
        // N s_7_14: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#894413:bv
        let s_8_0: Bits = fn_state.gs_894413;
        // D s_8_1: cast reint s_8_0 -> u16
        let s_8_1: u16 = (s_8_0.value() as u16);
        // D s_8_2: write-var op16shadow#7454 <= s_8_1
        fn_state.op16shadow_7454 = s_8_1;
        // N s_8_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esizeshadow#7453:i64
        let s_9_0: i64 = fn_state.esizeshadow_7453;
        // C s_9_1: const #32s : i
        let s_9_1: i128 = 32;
        // D s_9_2: cast zx s_9_0 -> i
        let s_9_2: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_1
        let s_9_3: bool = ((s_9_2) == (s_9_1));
        // D s_9_4: not s_9_3
        let s_9_4: bool = !s_9_3;
        // N s_9_5: branch s_9_4 b16 b10
        if s_9_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var op1_neg:u8
        let s_10_0: bool = fn_state.op1_neg;
        // N s_10_1: branch s_10_0 b14 b11
        if s_10_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var n:i64
        let s_11_0: i64 = fn_state.n;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call S_read(s_11_1)
        let s_11_2: u32 = S_read(state, tracer, s_11_1);
        // D s_11_3: write-var op32shadow#7455 <= s_11_2
        fn_state.op32shadow_7455 = s_11_2;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var d:i
        let s_12_0: i128 = fn_state.d;
        // D s_12_1: call S_read(s_12_0)
        let s_12_1: u32 = S_read(state, tracer, s_12_0);
        // D s_12_2: read-var m:i64
        let s_12_2: i64 = fn_state.m;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: call S_read(s_12_3)
        let s_12_4: u32 = S_read(state, tracer, s_12_3);
        // C s_12_5: const #() : ()
        let s_12_5: () = ();
        // S s_12_6: call FPSCR_read(s_12_5)
        let s_12_6: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_12_5);
        // D s_12_7: cast zx s_12_1 -> bv
        let s_12_7: Bits = Bits::new(s_12_1 as u128, 32u16);
        // D s_12_8: read-var op32shadow#7455:u32
        let s_12_8: u32 = fn_state.op32shadow_7455;
        // D s_12_9: cast zx s_12_8 -> bv
        let s_12_9: Bits = Bits::new(s_12_8 as u128, 32u16);
        // D s_12_10: cast zx s_12_4 -> bv
        let s_12_10: Bits = Bits::new(s_12_4 as u128, 32u16);
        // D s_12_11: call FPMulAdd(s_12_7, s_12_9, s_12_10, s_12_6)
        let s_12_11: Bits = FPMulAdd(state, tracer, s_12_7, s_12_9, s_12_10, s_12_6);
        // D s_12_12: write-var gs#894434 <= s_12_11
        fn_state.gs_894434 = s_12_11;
        // N s_12_13: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#894434:bv
        let s_13_0: Bits = fn_state.gs_894434;
        // D s_13_1: cast reint s_13_0 -> u32
        let s_13_1: u32 = (s_13_0.value() as u32);
        // D s_13_2: read-var d:i
        let s_13_2: i128 = fn_state.d;
        // D s_13_3: call S_set(s_13_2, s_13_1)
        let s_13_3: () = S_set(state, tracer, s_13_2, s_13_1);
        // N s_13_4: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var n:i64
        let s_14_0: i64 = fn_state.n;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call S_read(s_14_1)
        let s_14_2: u32 = S_read(state, tracer, s_14_1);
        // D s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 32u16);
        // D s_14_4: call FPNeg(s_14_3)
        let s_14_4: Bits = FPNeg(state, tracer, s_14_3);
        // D s_14_5: write-var gs#894429 <= s_14_4
        fn_state.gs_894429 = s_14_4;
        // N s_14_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#894429:bv
        let s_15_0: Bits = fn_state.gs_894429;
        // D s_15_1: cast reint s_15_0 -> u32
        let s_15_1: u32 = (s_15_0.value() as u32);
        // D s_15_2: write-var op32shadow#7455 <= s_15_1
        fn_state.op32shadow_7455 = s_15_1;
        // N s_15_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var esizeshadow#7453:i64
        let s_16_0: i64 = fn_state.esizeshadow_7453;
        // C s_16_1: const #64s : i
        let s_16_1: i128 = 64;
        // D s_16_2: cast zx s_16_0 -> i
        let s_16_2: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_3: cmp-eq s_16_2 s_16_1
        let s_16_3: bool = ((s_16_2) == (s_16_1));
        // D s_16_4: not s_16_3
        let s_16_4: bool = !s_16_3;
        // N s_16_5: branch s_16_4 b23 b17
        if s_16_4 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var op1_neg:u8
        let s_17_0: bool = fn_state.op1_neg;
        // N s_17_1: branch s_17_0 b21 b18
        if s_17_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var n:i64
        let s_18_0: i64 = fn_state.n;
        // D s_18_1: cast zx s_18_0 -> i
        let s_18_1: i128 = (i128::try_from(s_18_0).unwrap());
        // D s_18_2: call D_read(s_18_1)
        let s_18_2: u64 = D_read(state, tracer, s_18_1);
        // D s_18_3: write-var op64shadow#7456 <= s_18_2
        fn_state.op64shadow_7456 = s_18_2;
        // N s_18_4: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var d:i
        let s_19_0: i128 = fn_state.d;
        // D s_19_1: call D_read(s_19_0)
        let s_19_1: u64 = D_read(state, tracer, s_19_0);
        // D s_19_2: read-var m:i64
        let s_19_2: i64 = fn_state.m;
        // D s_19_3: cast zx s_19_2 -> i
        let s_19_3: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_4: call D_read(s_19_3)
        let s_19_4: u64 = D_read(state, tracer, s_19_3);
        // C s_19_5: const #() : ()
        let s_19_5: () = ();
        // S s_19_6: call FPSCR_read(s_19_5)
        let s_19_6: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_19_5);
        // D s_19_7: cast zx s_19_1 -> bv
        let s_19_7: Bits = Bits::new(s_19_1 as u128, 64u16);
        // D s_19_8: read-var op64shadow#7456:u64
        let s_19_8: u64 = fn_state.op64shadow_7456;
        // D s_19_9: cast zx s_19_8 -> bv
        let s_19_9: Bits = Bits::new(s_19_8 as u128, 64u16);
        // D s_19_10: cast zx s_19_4 -> bv
        let s_19_10: Bits = Bits::new(s_19_4 as u128, 64u16);
        // D s_19_11: call FPMulAdd(s_19_7, s_19_9, s_19_10, s_19_6)
        let s_19_11: Bits = FPMulAdd(state, tracer, s_19_7, s_19_9, s_19_10, s_19_6);
        // D s_19_12: write-var gs#894444 <= s_19_11
        fn_state.gs_894444 = s_19_11;
        // N s_19_13: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#894444:bv
        let s_20_0: Bits = fn_state.gs_894444;
        // D s_20_1: cast reint s_20_0 -> u64
        let s_20_1: u64 = (s_20_0.value() as u64);
        // D s_20_2: read-var d:i
        let s_20_2: i128 = fn_state.d;
        // D s_20_3: call D_set(s_20_2, s_20_1)
        let s_20_3: () = D_set(state, tracer, s_20_2, s_20_1);
        // N s_20_4: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var n:i64
        let s_21_0: i64 = fn_state.n;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: call D_read(s_21_1)
        let s_21_2: u64 = D_read(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 64u16);
        // D s_21_4: call FPNeg(s_21_3)
        let s_21_4: Bits = FPNeg(state, tracer, s_21_3);
        // D s_21_5: write-var gs#894439 <= s_21_4
        fn_state.gs_894439 = s_21_4;
        // N s_21_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#894439:bv
        let s_22_0: Bits = fn_state.gs_894439;
        // D s_22_1: cast reint s_22_0 -> u64
        let s_22_1: u64 = (s_22_0.value() as u64);
        // D s_22_2: write-var op64shadow#7456 <= s_22_1
        fn_state.op64shadow_7456 = s_22_1;
        // N s_22_3: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0s : i64
        let s_24_0: i64 = 0;
        // C s_24_1: const #1s : i
        let s_24_1: i128 = 1;
        // D s_24_2: read-var regs:i
        let s_24_2: i128 = fn_state.regs;
        // D s_24_3: sub s_24_2 s_24_1
        let s_24_3: i128 = ((s_24_2) - (s_24_1));
        // D s_24_4: cast reint s_24_3 -> i64
        let s_24_4: i64 = (s_24_3 as i64);
        // D s_24_5: write-var gs#308902 <= s_24_4
        fn_state.gs_308902 = s_24_4;
        // D s_24_6: write-var r <= s_24_0
        fn_state.r = s_24_0;
        // N s_24_7: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var r:i64
        let s_25_0: i64 = fn_state.r;
        // D s_25_1: read-var gs#308902:i64
        let s_25_1: i64 = fn_state.gs_308902;
        // D s_25_2: cmp-gt s_25_0 s_25_1
        let s_25_2: bool = ((s_25_0) > (s_25_1));
        // N s_25_3: branch s_25_2 b35 b26
        if s_25_2 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0s : i64
        let s_26_0: i64 = 0;
        // C s_26_1: const #1s : i
        let s_26_1: i128 = 1;
        // D s_26_2: read-var elements:i
        let s_26_2: i128 = fn_state.elements;
        // D s_26_3: sub s_26_2 s_26_1
        let s_26_3: i128 = ((s_26_2) - (s_26_1));
        // D s_26_4: cast reint s_26_3 -> i64
        let s_26_4: i64 = (s_26_3 as i64);
        // D s_26_5: write-var gs#308908 <= s_26_4
        fn_state.gs_308908 = s_26_4;
        // D s_26_6: write-var e <= s_26_0
        fn_state.e = s_26_0;
        // N s_26_7: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var e:i64
        let s_27_0: i64 = fn_state.e;
        // D s_27_1: read-var gs#308908:i64
        let s_27_1: i64 = fn_state.gs_308908;
        // D s_27_2: cmp-gt s_27_0 s_27_1
        let s_27_2: bool = ((s_27_0) > (s_27_1));
        // N s_27_3: branch s_27_2 b34 b28
        if s_27_2 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var n:i64
        let s_28_0: i64 = fn_state.n;
        // D s_28_1: cast zx s_28_0 -> i
        let s_28_1: i128 = (i128::try_from(s_28_0).unwrap());
        // D s_28_2: read-var r:i64
        let s_28_2: i64 = fn_state.r;
        // D s_28_3: cast zx s_28_2 -> i
        let s_28_3: i128 = (i128::try_from(s_28_2).unwrap());
        // D s_28_4: add s_28_1 s_28_3
        let s_28_4: i128 = (s_28_1 + s_28_3);
        // D s_28_5: call D_read(s_28_4)
        let s_28_5: u64 = D_read(state, tracer, s_28_4);
        // D s_28_6: read-var esizeshadow#7453:i64
        let s_28_6: i64 = fn_state.esizeshadow_7453;
        // D s_28_7: cast zx s_28_6 -> i
        let s_28_7: i128 = (i128::try_from(s_28_6).unwrap());
        // D s_28_8: cast reint s_28_7 -> i64
        let s_28_8: i64 = (s_28_7 as i64);
        // D s_28_9: cast zx s_28_5 -> bv
        let s_28_9: Bits = Bits::new(s_28_5 as u128, 64u16);
        // D s_28_10: read-var e:i64
        let s_28_10: i64 = fn_state.e;
        // D s_28_11: cast zx s_28_10 -> i
        let s_28_11: i128 = (i128::try_from(s_28_10).unwrap());
        // D s_28_12: cast zx s_28_8 -> i
        let s_28_12: i128 = (i128::try_from(s_28_8).unwrap());
        // D s_28_13: call Elem_read(s_28_9, s_28_11, s_28_12)
        let s_28_13: Bits = Elem_read(state, tracer, s_28_9, s_28_11, s_28_12);
        // D s_28_14: write-var op1 <= s_28_13
        fn_state.op1 = s_28_13;
        // D s_28_15: read-var op1_neg:u8
        let s_28_15: bool = fn_state.op1_neg;
        // N s_28_16: branch s_28_15 b32 b29
        if s_28_15 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var r:i64
        let s_30_0: i64 = fn_state.r;
        // D s_30_1: cast zx s_30_0 -> i
        let s_30_1: i128 = (i128::try_from(s_30_0).unwrap());
        // D s_30_2: read-var d:i
        let s_30_2: i128 = fn_state.d;
        // D s_30_3: add s_30_2 s_30_1
        let s_30_3: i128 = (s_30_2 + s_30_1);
        // D s_30_4: write-var ga#352932 <= s_30_3
        fn_state.ga_352932 = s_30_3;
        // D s_30_5: read-var r:i64
        let s_30_5: i64 = fn_state.r;
        // D s_30_6: cast zx s_30_5 -> i
        let s_30_6: i128 = (i128::try_from(s_30_5).unwrap());
        // D s_30_7: read-var d:i
        let s_30_7: i128 = fn_state.d;
        // D s_30_8: add s_30_7 s_30_6
        let s_30_8: i128 = (s_30_7 + s_30_6);
        // D s_30_9: call D_read(s_30_8)
        let s_30_9: u64 = D_read(state, tracer, s_30_8);
        // D s_30_10: write-var ga#352929 <= s_30_9
        fn_state.ga_352929 = s_30_9;
        // D s_30_11: read-var esizeshadow#7453:i64
        let s_30_11: i64 = fn_state.esizeshadow_7453;
        // D s_30_12: cast zx s_30_11 -> i
        let s_30_12: i128 = (i128::try_from(s_30_11).unwrap());
        // D s_30_13: cast reint s_30_12 -> i64
        let s_30_13: i64 = (s_30_12 as i64);
        // D s_30_14: write-var ga#352930 <= s_30_13
        fn_state.ga_352930 = s_30_13;
        // D s_30_15: read-var r:i64
        let s_30_15: i64 = fn_state.r;
        // D s_30_16: cast zx s_30_15 -> i
        let s_30_16: i128 = (i128::try_from(s_30_15).unwrap());
        // D s_30_17: read-var d:i
        let s_30_17: i128 = fn_state.d;
        // D s_30_18: add s_30_17 s_30_16
        let s_30_18: i128 = (s_30_17 + s_30_16);
        // D s_30_19: call D_read(s_30_18)
        let s_30_19: u64 = D_read(state, tracer, s_30_18);
        // D s_30_20: read-var esizeshadow#7453:i64
        let s_30_20: i64 = fn_state.esizeshadow_7453;
        // D s_30_21: cast zx s_30_20 -> i
        let s_30_21: i128 = (i128::try_from(s_30_20).unwrap());
        // D s_30_22: cast reint s_30_21 -> i64
        let s_30_22: i64 = (s_30_21 as i64);
        // D s_30_23: cast zx s_30_19 -> bv
        let s_30_23: Bits = Bits::new(s_30_19 as u128, 64u16);
        // D s_30_24: read-var e:i64
        let s_30_24: i64 = fn_state.e;
        // D s_30_25: cast zx s_30_24 -> i
        let s_30_25: i128 = (i128::try_from(s_30_24).unwrap());
        // D s_30_26: cast zx s_30_22 -> i
        let s_30_26: i128 = (i128::try_from(s_30_22).unwrap());
        // D s_30_27: call Elem_read(s_30_23, s_30_25, s_30_26)
        let s_30_27: Bits = Elem_read(state, tracer, s_30_23, s_30_25, s_30_26);
        // D s_30_28: read-var m:i64
        let s_30_28: i64 = fn_state.m;
        // D s_30_29: cast zx s_30_28 -> i
        let s_30_29: i128 = (i128::try_from(s_30_28).unwrap());
        // D s_30_30: read-var r:i64
        let s_30_30: i64 = fn_state.r;
        // D s_30_31: cast zx s_30_30 -> i
        let s_30_31: i128 = (i128::try_from(s_30_30).unwrap());
        // D s_30_32: add s_30_29 s_30_31
        let s_30_32: i128 = (s_30_29 + s_30_31);
        // D s_30_33: call D_read(s_30_32)
        let s_30_33: u64 = D_read(state, tracer, s_30_32);
        // D s_30_34: read-var esizeshadow#7453:i64
        let s_30_34: i64 = fn_state.esizeshadow_7453;
        // D s_30_35: cast zx s_30_34 -> i
        let s_30_35: i128 = (i128::try_from(s_30_34).unwrap());
        // D s_30_36: cast reint s_30_35 -> i64
        let s_30_36: i64 = (s_30_35 as i64);
        // D s_30_37: cast zx s_30_33 -> bv
        let s_30_37: Bits = Bits::new(s_30_33 as u128, 64u16);
        // D s_30_38: read-var e:i64
        let s_30_38: i64 = fn_state.e;
        // D s_30_39: cast zx s_30_38 -> i
        let s_30_39: i128 = (i128::try_from(s_30_38).unwrap());
        // D s_30_40: cast zx s_30_36 -> i
        let s_30_40: i128 = (i128::try_from(s_30_36).unwrap());
        // D s_30_41: call Elem_read(s_30_37, s_30_39, s_30_40)
        let s_30_41: Bits = Elem_read(state, tracer, s_30_37, s_30_39, s_30_40);
        // C s_30_42: const #() : ()
        let s_30_42: () = ();
        // S s_30_43: call StandardFPSCRValue(s_30_42)
        let s_30_43: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_30_42,
        );
        // D s_30_44: read-var op1:bv
        let s_30_44: Bits = fn_state.op1;
        // D s_30_45: call FPMulAdd(s_30_27, s_30_44, s_30_41, s_30_43)
        let s_30_45: Bits = FPMulAdd(state, tracer, s_30_27, s_30_44, s_30_41, s_30_43);
        // D s_30_46: write-var ga#352931 <= s_30_45
        fn_state.ga_352931 = s_30_45;
        // N s_30_47: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var ga#352929:u64
        let s_31_0: u64 = fn_state.ga_352929;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 64u16);
        // D s_31_2: read-var e:i64
        let s_31_2: i64 = fn_state.e;
        // D s_31_3: cast zx s_31_2 -> i
        let s_31_3: i128 = (i128::try_from(s_31_2).unwrap());
        // D s_31_4: read-var ga#352930:i64
        let s_31_4: i64 = fn_state.ga_352930;
        // D s_31_5: cast zx s_31_4 -> i
        let s_31_5: i128 = (i128::try_from(s_31_4).unwrap());
        // D s_31_6: read-var ga#352931:bv
        let s_31_6: Bits = fn_state.ga_352931;
        // D s_31_7: call Elem_set(s_31_1, s_31_3, s_31_5, s_31_6)
        let s_31_7: Bits = Elem_set(state, tracer, s_31_1, s_31_3, s_31_5, s_31_6);
        // D s_31_8: cast reint s_31_7 -> u64
        let s_31_8: u64 = (s_31_7.value() as u64);
        // D s_31_9: read-var ga#352932:i
        let s_31_9: i128 = fn_state.ga_352932;
        // D s_31_10: call D_set(s_31_9, s_31_8)
        let s_31_10: () = D_set(state, tracer, s_31_9, s_31_8);
        // D s_31_11: read-var e:i64
        let s_31_11: i64 = fn_state.e;
        // C s_31_12: const #1s : i64
        let s_31_12: i64 = 1;
        // D s_31_13: add s_31_11 s_31_12
        let s_31_13: i64 = (s_31_11 + s_31_12);
        // D s_31_14: write-var e <= s_31_13
        fn_state.e = s_31_13;
        // N s_31_15: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var op1:bv
        let s_32_0: Bits = fn_state.op1;
        // D s_32_1: call FPNeg(s_32_0)
        let s_32_1: Bits = FPNeg(state, tracer, s_32_0);
        // D s_32_2: write-var op1 <= s_32_1
        fn_state.op1 = s_32_1;
        // N s_32_3: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_33_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var r:i64
        let s_34_0: i64 = fn_state.r;
        // C s_34_1: const #1s : i64
        let s_34_1: i64 = 1;
        // D s_34_2: add s_34_0 s_34_1
        let s_34_2: i64 = (s_34_0 + s_34_1);
        // D s_34_3: write-var r <= s_34_2
        fn_state.r = s_34_2;
        // N s_34_4: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: return
        return;
    }
}
