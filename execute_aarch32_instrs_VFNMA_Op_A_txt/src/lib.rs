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
use CheckVFPEnabled::*;
use FPNeg::*;
use FPSCR_read::*;
use D_set::*;
use Zeros::*;
use FPMulAdd::*;
use S_read::*;
use S_set::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VFNMA_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    esize: i64,
    m: i64,
    n: i64,
    op1_neg: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        op64shadow_7471: u64,
        gs_894703: Bits,
        gs_894696: Bits,
        gs_894690: Bits,
        gs_894719: Bits,
        gs_894714: Bits,
        ga_353061: u16,
        gs_894711: Bits,
        gs_894733: Bits,
        gs_894725: Bits,
        op16shadow_7469: u16,
        op32shadow_7470: u32,
        gs_894728: Bits,
        d: i64,
        esize: i64,
        m: i64,
        n: i64,
        op1_neg: bool,
    }
    let fn_state = FunctionState {
        d,
        esize,
        m,
        n,
        op1_neg,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // S s_0_1: call CheckVFPEnabled(s_0_0)
        let s_0_1: () = CheckVFPEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var esize:i64
        let s_1_0: i64 = fn_state.esize;
        // C s_1_1: const #16s : i
        let s_1_1: i128 = 16;
        // D s_1_2: cast zx s_1_0 -> i
        let s_1_2: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_3: cmp-eq s_1_2 s_1_1
        let s_1_3: bool = ((s_1_2) == (s_1_1));
        // D s_1_4: not s_1_3
        let s_1_4: bool = !s_1_3;
        // N s_1_5: branch s_1_4 b9 b2
        if s_1_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var op1_neg:u8
        let s_2_0: bool = fn_state.op1_neg;
        // N s_2_1: branch s_2_0 b7 b3
        if s_2_0 {
            return block_7(state, tracer, fn_state);
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
        // D s_3_11: write-var op16shadow#7469 <= s_3_10
        fn_state.op16shadow_7469 = s_3_10;
        // N s_3_12: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #16s : i
        let s_4_0: i128 = 16;
        // S s_4_1: call Zeros(s_4_0)
        let s_4_1: Bits = Zeros(state, tracer, s_4_0);
        // S s_4_2: cast reint s_4_1 -> u16
        let s_4_2: u16 = (s_4_1.value() as u16);
        // D s_4_3: write-var ga#353061 <= s_4_2
        fn_state.ga_353061 = s_4_2;
        // D s_4_4: read-var d:i64
        let s_4_4: i64 = fn_state.d;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: call S_read(s_4_5)
        let s_4_6: u32 = S_read(state, tracer, s_4_5);
        // C s_4_7: const #0s : i
        let s_4_7: i128 = 0;
        // D s_4_8: cast zx s_4_6 -> bv
        let s_4_8: Bits = Bits::new(s_4_6 as u128, 32u16);
        // C s_4_9: const #1s : i64
        let s_4_9: i64 = 1;
        // C s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // C s_4_11: const #15s : i
        let s_4_11: i128 = 15;
        // C s_4_12: add s_4_11 s_4_10
        let s_4_12: i128 = (s_4_11 + s_4_10);
        // D s_4_13: bit-extract s_4_8 s_4_7 s_4_12
        let s_4_13: Bits = (Bits::new(
            ((s_4_8) >> (s_4_7)).value(),
            u16::try_from(s_4_12).unwrap(),
        ));
        // D s_4_14: cast reint s_4_13 -> u16
        let s_4_14: u16 = (s_4_13.value() as u16);
        // D s_4_15: cast zx s_4_14 -> bv
        let s_4_15: Bits = Bits::new(s_4_14 as u128, 16u16);
        // D s_4_16: call FPNeg(s_4_15)
        let s_4_16: Bits = FPNeg(state, tracer, s_4_15);
        // D s_4_17: write-var gs#894696 <= s_4_16
        fn_state.gs_894696 = s_4_16;
        // N s_4_18: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#894696:bv
        let s_5_0: Bits = fn_state.gs_894696;
        // D s_5_1: cast reint s_5_0 -> u16
        let s_5_1: u16 = (s_5_0.value() as u16);
        // D s_5_2: read-var m:i64
        let s_5_2: i64 = fn_state.m;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: call S_read(s_5_3)
        let s_5_4: u32 = S_read(state, tracer, s_5_3);
        // C s_5_5: const #0s : i
        let s_5_5: i128 = 0;
        // D s_5_6: cast zx s_5_4 -> bv
        let s_5_6: Bits = Bits::new(s_5_4 as u128, 32u16);
        // C s_5_7: const #1s : i64
        let s_5_7: i64 = 1;
        // C s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // C s_5_9: const #15s : i
        let s_5_9: i128 = 15;
        // C s_5_10: add s_5_9 s_5_8
        let s_5_10: i128 = (s_5_9 + s_5_8);
        // D s_5_11: bit-extract s_5_6 s_5_5 s_5_10
        let s_5_11: Bits = (Bits::new(
            ((s_5_6) >> (s_5_5)).value(),
            u16::try_from(s_5_10).unwrap(),
        ));
        // D s_5_12: cast reint s_5_11 -> u16
        let s_5_12: u16 = (s_5_11.value() as u16);
        // C s_5_13: const #() : ()
        let s_5_13: () = ();
        // S s_5_14: call FPSCR_read(s_5_13)
        let s_5_14: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_5_13);
        // D s_5_15: cast zx s_5_1 -> bv
        let s_5_15: Bits = Bits::new(s_5_1 as u128, 16u16);
        // D s_5_16: read-var op16shadow#7469:u16
        let s_5_16: u16 = fn_state.op16shadow_7469;
        // D s_5_17: cast zx s_5_16 -> bv
        let s_5_17: Bits = Bits::new(s_5_16 as u128, 16u16);
        // D s_5_18: cast zx s_5_12 -> bv
        let s_5_18: Bits = Bits::new(s_5_12 as u128, 16u16);
        // D s_5_19: call FPMulAdd(s_5_15, s_5_17, s_5_18, s_5_14)
        let s_5_19: Bits = FPMulAdd(state, tracer, s_5_15, s_5_17, s_5_18, s_5_14);
        // D s_5_20: write-var gs#894703 <= s_5_19
        fn_state.gs_894703 = s_5_19;
        // N s_5_21: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#894703:bv
        let s_6_0: Bits = fn_state.gs_894703;
        // D s_6_1: cast reint s_6_0 -> u16
        let s_6_1: u16 = (s_6_0.value() as u16);
        // D s_6_2: read-var ga#353061:u16
        let s_6_2: u16 = fn_state.ga_353061;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 16u16);
        // D s_6_4: cast zx s_6_1 -> bv
        let s_6_4: Bits = Bits::new(s_6_1 as u128, 16u16);
        // D s_6_5: cast reint s_6_3 -> u128
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
        // D s_6_14: read-var d:i64
        let s_6_14: i64 = fn_state.d;
        // D s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (i128::try_from(s_6_14).unwrap());
        // D s_6_16: call S_set(s_6_15, s_6_13)
        let s_6_16: () = S_set(state, tracer, s_6_15, s_6_13);
        // N s_6_17: return
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
        // D s_7_13: write-var gs#894690 <= s_7_12
        fn_state.gs_894690 = s_7_12;
        // N s_7_14: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#894690:bv
        let s_8_0: Bits = fn_state.gs_894690;
        // D s_8_1: cast reint s_8_0 -> u16
        let s_8_1: u16 = (s_8_0.value() as u16);
        // D s_8_2: write-var op16shadow#7469 <= s_8_1
        fn_state.op16shadow_7469 = s_8_1;
        // N s_8_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esize:i64
        let s_9_0: i64 = fn_state.esize;
        // C s_9_1: const #32s : i
        let s_9_1: i128 = 32;
        // D s_9_2: cast zx s_9_0 -> i
        let s_9_2: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_3: cmp-eq s_9_2 s_9_1
        let s_9_3: bool = ((s_9_2) == (s_9_1));
        // D s_9_4: not s_9_3
        let s_9_4: bool = !s_9_3;
        // N s_9_5: branch s_9_4 b17 b10
        if s_9_4 {
            return block_17(state, tracer, fn_state);
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
        // N s_10_1: branch s_10_0 b15 b11
        if s_10_0 {
            return block_15(state, tracer, fn_state);
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
        // D s_11_3: write-var op32shadow#7470 <= s_11_2
        fn_state.op32shadow_7470 = s_11_2;
        // N s_11_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var d:i64
        let s_12_0: i64 = fn_state.d;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: call S_read(s_12_1)
        let s_12_2: u32 = S_read(state, tracer, s_12_1);
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 32u16);
        // D s_12_4: call FPNeg(s_12_3)
        let s_12_4: Bits = FPNeg(state, tracer, s_12_3);
        // D s_12_5: write-var gs#894714 <= s_12_4
        fn_state.gs_894714 = s_12_4;
        // N s_12_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#894714:bv
        let s_13_0: Bits = fn_state.gs_894714;
        // D s_13_1: cast reint s_13_0 -> u32
        let s_13_1: u32 = (s_13_0.value() as u32);
        // D s_13_2: read-var m:i64
        let s_13_2: i64 = fn_state.m;
        // D s_13_3: cast zx s_13_2 -> i
        let s_13_3: i128 = (i128::try_from(s_13_2).unwrap());
        // D s_13_4: call S_read(s_13_3)
        let s_13_4: u32 = S_read(state, tracer, s_13_3);
        // C s_13_5: const #() : ()
        let s_13_5: () = ();
        // S s_13_6: call FPSCR_read(s_13_5)
        let s_13_6: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_13_5);
        // D s_13_7: cast zx s_13_1 -> bv
        let s_13_7: Bits = Bits::new(s_13_1 as u128, 32u16);
        // D s_13_8: read-var op32shadow#7470:u32
        let s_13_8: u32 = fn_state.op32shadow_7470;
        // D s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 32u16);
        // D s_13_10: cast zx s_13_4 -> bv
        let s_13_10: Bits = Bits::new(s_13_4 as u128, 32u16);
        // D s_13_11: call FPMulAdd(s_13_7, s_13_9, s_13_10, s_13_6)
        let s_13_11: Bits = FPMulAdd(state, tracer, s_13_7, s_13_9, s_13_10, s_13_6);
        // D s_13_12: write-var gs#894719 <= s_13_11
        fn_state.gs_894719 = s_13_11;
        // N s_13_13: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#894719:bv
        let s_14_0: Bits = fn_state.gs_894719;
        // D s_14_1: cast reint s_14_0 -> u32
        let s_14_1: u32 = (s_14_0.value() as u32);
        // D s_14_2: read-var d:i64
        let s_14_2: i64 = fn_state.d;
        // D s_14_3: cast zx s_14_2 -> i
        let s_14_3: i128 = (i128::try_from(s_14_2).unwrap());
        // D s_14_4: call S_set(s_14_3, s_14_1)
        let s_14_4: () = S_set(state, tracer, s_14_3, s_14_1);
        // N s_14_5: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var n:i64
        let s_15_0: i64 = fn_state.n;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call S_read(s_15_1)
        let s_15_2: u32 = S_read(state, tracer, s_15_1);
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 32u16);
        // D s_15_4: call FPNeg(s_15_3)
        let s_15_4: Bits = FPNeg(state, tracer, s_15_3);
        // D s_15_5: write-var gs#894711 <= s_15_4
        fn_state.gs_894711 = s_15_4;
        // N s_15_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#894711:bv
        let s_16_0: Bits = fn_state.gs_894711;
        // D s_16_1: cast reint s_16_0 -> u32
        let s_16_1: u32 = (s_16_0.value() as u32);
        // D s_16_2: write-var op32shadow#7470 <= s_16_1
        fn_state.op32shadow_7470 = s_16_1;
        // N s_16_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var esize:i64
        let s_17_0: i64 = fn_state.esize;
        // C s_17_1: const #64s : i
        let s_17_1: i128 = 64;
        // D s_17_2: cast zx s_17_0 -> i
        let s_17_2: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_3: cmp-eq s_17_2 s_17_1
        let s_17_3: bool = ((s_17_2) == (s_17_1));
        // D s_17_4: not s_17_3
        let s_17_4: bool = !s_17_3;
        // N s_17_5: branch s_17_4 b25 b18
        if s_17_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var op1_neg:u8
        let s_18_0: bool = fn_state.op1_neg;
        // N s_18_1: branch s_18_0 b23 b19
        if s_18_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var n:i64
        let s_19_0: i64 = fn_state.n;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: call D_read(s_19_1)
        let s_19_2: u64 = D_read(state, tracer, s_19_1);
        // D s_19_3: write-var op64shadow#7471 <= s_19_2
        fn_state.op64shadow_7471 = s_19_2;
        // N s_19_4: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var d:i64
        let s_20_0: i64 = fn_state.d;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: call D_read(s_20_1)
        let s_20_2: u64 = D_read(state, tracer, s_20_1);
        // D s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 64u16);
        // D s_20_4: call FPNeg(s_20_3)
        let s_20_4: Bits = FPNeg(state, tracer, s_20_3);
        // D s_20_5: write-var gs#894728 <= s_20_4
        fn_state.gs_894728 = s_20_4;
        // N s_20_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#894728:bv
        let s_21_0: Bits = fn_state.gs_894728;
        // D s_21_1: cast reint s_21_0 -> u64
        let s_21_1: u64 = (s_21_0.value() as u64);
        // D s_21_2: read-var m:i64
        let s_21_2: i64 = fn_state.m;
        // D s_21_3: cast zx s_21_2 -> i
        let s_21_3: i128 = (i128::try_from(s_21_2).unwrap());
        // D s_21_4: call D_read(s_21_3)
        let s_21_4: u64 = D_read(state, tracer, s_21_3);
        // C s_21_5: const #() : ()
        let s_21_5: () = ();
        // S s_21_6: call FPSCR_read(s_21_5)
        let s_21_6: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_21_5);
        // D s_21_7: cast zx s_21_1 -> bv
        let s_21_7: Bits = Bits::new(s_21_1 as u128, 64u16);
        // D s_21_8: read-var op64shadow#7471:u64
        let s_21_8: u64 = fn_state.op64shadow_7471;
        // D s_21_9: cast zx s_21_8 -> bv
        let s_21_9: Bits = Bits::new(s_21_8 as u128, 64u16);
        // D s_21_10: cast zx s_21_4 -> bv
        let s_21_10: Bits = Bits::new(s_21_4 as u128, 64u16);
        // D s_21_11: call FPMulAdd(s_21_7, s_21_9, s_21_10, s_21_6)
        let s_21_11: Bits = FPMulAdd(state, tracer, s_21_7, s_21_9, s_21_10, s_21_6);
        // D s_21_12: write-var gs#894733 <= s_21_11
        fn_state.gs_894733 = s_21_11;
        // N s_21_13: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#894733:bv
        let s_22_0: Bits = fn_state.gs_894733;
        // D s_22_1: cast reint s_22_0 -> u64
        let s_22_1: u64 = (s_22_0.value() as u64);
        // D s_22_2: read-var d:i64
        let s_22_2: i64 = fn_state.d;
        // D s_22_3: cast zx s_22_2 -> i
        let s_22_3: i128 = (i128::try_from(s_22_2).unwrap());
        // D s_22_4: call D_set(s_22_3, s_22_1)
        let s_22_4: () = D_set(state, tracer, s_22_3, s_22_1);
        // N s_22_5: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var n:i64
        let s_23_0: i64 = fn_state.n;
        // D s_23_1: cast zx s_23_0 -> i
        let s_23_1: i128 = (i128::try_from(s_23_0).unwrap());
        // D s_23_2: call D_read(s_23_1)
        let s_23_2: u64 = D_read(state, tracer, s_23_1);
        // D s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 64u16);
        // D s_23_4: call FPNeg(s_23_3)
        let s_23_4: Bits = FPNeg(state, tracer, s_23_3);
        // D s_23_5: write-var gs#894725 <= s_23_4
        fn_state.gs_894725 = s_23_4;
        // N s_23_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#894725:bv
        let s_24_0: Bits = fn_state.gs_894725;
        // D s_24_1: cast reint s_24_0 -> u64
        let s_24_1: u64 = (s_24_0.value() as u64);
        // D s_24_2: write-var op64shadow#7471 <= s_24_1
        fn_state.op64shadow_7471 = s_24_1;
        // N s_24_3: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: return
        return;
    }
}
