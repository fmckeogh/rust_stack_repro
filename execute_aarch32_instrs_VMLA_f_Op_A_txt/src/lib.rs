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
use FPMul::*;
use FPAdd::*;
use CheckAdvSIMDOrVFPEnabled::*;
use FPNeg::*;
use FPSCR_read::*;
use D_set::*;
use S_set::*;
use Elem_read::*;
use S_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VMLA_f_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    add: bool,
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
        gs_898419: Bits,
        r: i64,
        product: Bits,
        gs_898407: Bits,
        addend16shadow_7502: u16,
        e: i64,
        addend32shadow_7503: u32,
        gs_312162: i64,
        ga_355432: i64,
        addend: Bits,
        esizeshadow_7501: i64,
        gs_898395: Bits,
        ga_355431: u64,
        gs_898403: Bits,
        gs_898368: Bits,
        gs_312156: i64,
        gs_898414: Bits,
        addend64shadow_7504: u64,
        gs_898398: Bits,
        gs_898411: Bits,
        d: i128,
        gs_898379: Bits,
        ga_355433: Bits,
        gs_898370: Bits,
        ga_355434: i128,
        gs_898391: Bits,
        gs_898384: Bits,
        add: bool,
        advsimd: bool,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        n: i64,
        regs: i128,
    }
    let fn_state = FunctionState {
        add,
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
        // D s_0_3: write-var esizeshadow#7501 <= s_0_2
        fn_state.esizeshadow_7501 = s_0_2;
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
        // N s_1_1: branch s_1_0 b30 b2
        if s_1_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var esizeshadow#7501:i64
        let s_2_0: i64 = fn_state.esizeshadow_7501;
        // C s_2_1: const #16s : i
        let s_2_1: i128 = 16;
        // D s_2_2: cast zx s_2_0 -> i
        let s_2_2: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_1
        let s_2_3: bool = ((s_2_2) == (s_2_1));
        // D s_2_4: not s_2_3
        let s_2_4: bool = !s_2_3;
        // N s_2_5: branch s_2_4 b11 b3
        if s_2_4 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var add:u8
        let s_3_0: bool = fn_state.add;
        // N s_3_1: branch s_3_0 b9 b4
        if s_3_0 {
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
        // D s_4_11: read-var m:i64
        let s_4_11: i64 = fn_state.m;
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_13: call S_read(s_4_12)
        let s_4_13: u32 = S_read(state, tracer, s_4_12);
        // C s_4_14: const #0s : i
        let s_4_14: i128 = 0;
        // D s_4_15: cast zx s_4_13 -> bv
        let s_4_15: Bits = Bits::new(s_4_13 as u128, 32u16);
        // C s_4_16: const #1s : i64
        let s_4_16: i64 = 1;
        // C s_4_17: cast zx s_4_16 -> i
        let s_4_17: i128 = (i128::try_from(s_4_16).unwrap());
        // C s_4_18: const #15s : i
        let s_4_18: i128 = 15;
        // C s_4_19: add s_4_18 s_4_17
        let s_4_19: i128 = (s_4_18 + s_4_17);
        // D s_4_20: bit-extract s_4_15 s_4_14 s_4_19
        let s_4_20: Bits = (Bits::new(
            ((s_4_15) >> (s_4_14)).value(),
            u16::try_from(s_4_19).unwrap(),
        ));
        // D s_4_21: cast reint s_4_20 -> u16
        let s_4_21: u16 = (s_4_20.value() as u16);
        // C s_4_22: const #() : ()
        let s_4_22: () = ();
        // S s_4_23: call FPSCR_read(s_4_22)
        let s_4_23: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_4_22);
        // D s_4_24: cast zx s_4_10 -> bv
        let s_4_24: Bits = Bits::new(s_4_10 as u128, 16u16);
        // D s_4_25: cast zx s_4_21 -> bv
        let s_4_25: Bits = Bits::new(s_4_21 as u128, 16u16);
        // D s_4_26: call FPMul(s_4_24, s_4_25, s_4_23)
        let s_4_26: Bits = FPMul(state, tracer, s_4_24, s_4_25, s_4_23);
        // D s_4_27: write-var gs#898368 <= s_4_26
        fn_state.gs_898368 = s_4_26;
        // N s_4_28: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#898368:bv
        let s_5_0: Bits = fn_state.gs_898368;
        // D s_5_1: cast reint s_5_0 -> u16
        let s_5_1: u16 = (s_5_0.value() as u16);
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 16u16);
        // D s_5_3: call FPNeg(s_5_2)
        let s_5_3: Bits = FPNeg(state, tracer, s_5_2);
        // D s_5_4: write-var gs#898370 <= s_5_3
        fn_state.gs_898370 = s_5_3;
        // N s_5_5: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#898370:bv
        let s_6_0: Bits = fn_state.gs_898370;
        // D s_6_1: cast reint s_6_0 -> u16
        let s_6_1: u16 = (s_6_0.value() as u16);
        // D s_6_2: write-var addend16shadow#7502 <= s_6_1
        fn_state.addend16shadow_7502 = s_6_1;
        // N s_6_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var d:i
        let s_7_0: i128 = fn_state.d;
        // D s_7_1: call S_read(s_7_0)
        let s_7_1: u32 = S_read(state, tracer, s_7_0);
        // C s_7_2: const #0s : i
        let s_7_2: i128 = 0;
        // D s_7_3: cast zx s_7_1 -> bv
        let s_7_3: Bits = Bits::new(s_7_1 as u128, 32u16);
        // C s_7_4: const #1s : i64
        let s_7_4: i64 = 1;
        // C s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // C s_7_6: const #15s : i
        let s_7_6: i128 = 15;
        // C s_7_7: add s_7_6 s_7_5
        let s_7_7: i128 = (s_7_6 + s_7_5);
        // D s_7_8: bit-extract s_7_3 s_7_2 s_7_7
        let s_7_8: Bits = (Bits::new(
            ((s_7_3) >> (s_7_2)).value(),
            u16::try_from(s_7_7).unwrap(),
        ));
        // D s_7_9: cast reint s_7_8 -> u16
        let s_7_9: u16 = (s_7_8.value() as u16);
        // C s_7_10: const #() : ()
        let s_7_10: () = ();
        // S s_7_11: call FPSCR_read(s_7_10)
        let s_7_11: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_7_10);
        // D s_7_12: cast zx s_7_9 -> bv
        let s_7_12: Bits = Bits::new(s_7_9 as u128, 16u16);
        // D s_7_13: read-var addend16shadow#7502:u16
        let s_7_13: u16 = fn_state.addend16shadow_7502;
        // D s_7_14: cast zx s_7_13 -> bv
        let s_7_14: Bits = Bits::new(s_7_13 as u128, 16u16);
        // D s_7_15: call FPAdd(s_7_12, s_7_14, s_7_11)
        let s_7_15: Bits = FPAdd(state, tracer, s_7_12, s_7_14, s_7_11);
        // D s_7_16: write-var gs#898384 <= s_7_15
        fn_state.gs_898384 = s_7_15;
        // N s_7_17: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#898384:bv
        let s_8_0: Bits = fn_state.gs_898384;
        // D s_8_1: cast reint s_8_0 -> u16
        let s_8_1: u16 = (s_8_0.value() as u16);
        // C s_8_2: const #0u : u16
        let s_8_2: u16 = 0;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 16u16);
        // D s_8_4: cast zx s_8_1 -> bv
        let s_8_4: Bits = Bits::new(s_8_1 as u128, 16u16);
        // C s_8_5: cast reint s_8_3 -> u128
        let s_8_5: u128 = (s_8_3.value() as u128);
        // D s_8_6: size-of s_8_3
        let s_8_6: u16 = s_8_3.length();
        // D s_8_7: cast reint s_8_4 -> u128
        let s_8_7: u128 = (s_8_4.value() as u128);
        // D s_8_8: size-of s_8_4
        let s_8_8: u16 = s_8_4.length();
        // D s_8_9: lsl s_8_5 s_8_8
        let s_8_9: u128 = s_8_5 << s_8_8;
        // D s_8_10: or s_8_9 s_8_7
        let s_8_10: u128 = ((s_8_9) | (s_8_7));
        // D s_8_11: add s_8_6 s_8_8
        let s_8_11: u16 = (s_8_6 + s_8_8);
        // D s_8_12: create-bits s_8_10 s_8_11
        let s_8_12: Bits = Bits::new(s_8_10, s_8_11);
        // D s_8_13: cast reint s_8_12 -> u32
        let s_8_13: u32 = (s_8_12.value() as u32);
        // D s_8_14: read-var d:i
        let s_8_14: i128 = fn_state.d;
        // D s_8_15: call S_set(s_8_14, s_8_13)
        let s_8_15: () = S_set(state, tracer, s_8_14, s_8_13);
        // N s_8_16: return
        return;
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
        // D s_9_2: call S_read(s_9_1)
        let s_9_2: u32 = S_read(state, tracer, s_9_1);
        // C s_9_3: const #0s : i
        let s_9_3: i128 = 0;
        // D s_9_4: cast zx s_9_2 -> bv
        let s_9_4: Bits = Bits::new(s_9_2 as u128, 32u16);
        // C s_9_5: const #1s : i64
        let s_9_5: i64 = 1;
        // C s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // C s_9_7: const #15s : i
        let s_9_7: i128 = 15;
        // C s_9_8: add s_9_7 s_9_6
        let s_9_8: i128 = (s_9_7 + s_9_6);
        // D s_9_9: bit-extract s_9_4 s_9_3 s_9_8
        let s_9_9: Bits = (Bits::new(
            ((s_9_4) >> (s_9_3)).value(),
            u16::try_from(s_9_8).unwrap(),
        ));
        // D s_9_10: cast reint s_9_9 -> u16
        let s_9_10: u16 = (s_9_9.value() as u16);
        // D s_9_11: read-var m:i64
        let s_9_11: i64 = fn_state.m;
        // D s_9_12: cast zx s_9_11 -> i
        let s_9_12: i128 = (i128::try_from(s_9_11).unwrap());
        // D s_9_13: call S_read(s_9_12)
        let s_9_13: u32 = S_read(state, tracer, s_9_12);
        // C s_9_14: const #0s : i
        let s_9_14: i128 = 0;
        // D s_9_15: cast zx s_9_13 -> bv
        let s_9_15: Bits = Bits::new(s_9_13 as u128, 32u16);
        // C s_9_16: const #1s : i64
        let s_9_16: i64 = 1;
        // C s_9_17: cast zx s_9_16 -> i
        let s_9_17: i128 = (i128::try_from(s_9_16).unwrap());
        // C s_9_18: const #15s : i
        let s_9_18: i128 = 15;
        // C s_9_19: add s_9_18 s_9_17
        let s_9_19: i128 = (s_9_18 + s_9_17);
        // D s_9_20: bit-extract s_9_15 s_9_14 s_9_19
        let s_9_20: Bits = (Bits::new(
            ((s_9_15) >> (s_9_14)).value(),
            u16::try_from(s_9_19).unwrap(),
        ));
        // D s_9_21: cast reint s_9_20 -> u16
        let s_9_21: u16 = (s_9_20.value() as u16);
        // C s_9_22: const #() : ()
        let s_9_22: () = ();
        // S s_9_23: call FPSCR_read(s_9_22)
        let s_9_23: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_9_22);
        // D s_9_24: cast zx s_9_10 -> bv
        let s_9_24: Bits = Bits::new(s_9_10 as u128, 16u16);
        // D s_9_25: cast zx s_9_21 -> bv
        let s_9_25: Bits = Bits::new(s_9_21 as u128, 16u16);
        // D s_9_26: call FPMul(s_9_24, s_9_25, s_9_23)
        let s_9_26: Bits = FPMul(state, tracer, s_9_24, s_9_25, s_9_23);
        // D s_9_27: write-var gs#898379 <= s_9_26
        fn_state.gs_898379 = s_9_26;
        // N s_9_28: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#898379:bv
        let s_10_0: Bits = fn_state.gs_898379;
        // D s_10_1: cast reint s_10_0 -> u16
        let s_10_1: u16 = (s_10_0.value() as u16);
        // D s_10_2: write-var addend16shadow#7502 <= s_10_1
        fn_state.addend16shadow_7502 = s_10_1;
        // N s_10_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#7501:i64
        let s_11_0: i64 = fn_state.esizeshadow_7501;
        // C s_11_1: const #32s : i
        let s_11_1: i128 = 32;
        // D s_11_2: cast zx s_11_0 -> i
        let s_11_2: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_3: cmp-eq s_11_2 s_11_1
        let s_11_3: bool = ((s_11_2) == (s_11_1));
        // D s_11_4: not s_11_3
        let s_11_4: bool = !s_11_3;
        // N s_11_5: branch s_11_4 b20 b12
        if s_11_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var add:u8
        let s_12_0: bool = fn_state.add;
        // N s_12_1: branch s_12_0 b18 b13
        if s_12_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var n:i64
        let s_13_0: i64 = fn_state.n;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: call S_read(s_13_1)
        let s_13_2: u32 = S_read(state, tracer, s_13_1);
        // D s_13_3: read-var m:i64
        let s_13_3: i64 = fn_state.m;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: call S_read(s_13_4)
        let s_13_5: u32 = S_read(state, tracer, s_13_4);
        // C s_13_6: const #() : ()
        let s_13_6: () = ();
        // S s_13_7: call FPSCR_read(s_13_6)
        let s_13_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_13_6);
        // D s_13_8: cast zx s_13_2 -> bv
        let s_13_8: Bits = Bits::new(s_13_2 as u128, 32u16);
        // D s_13_9: cast zx s_13_5 -> bv
        let s_13_9: Bits = Bits::new(s_13_5 as u128, 32u16);
        // D s_13_10: call FPMul(s_13_8, s_13_9, s_13_7)
        let s_13_10: Bits = FPMul(state, tracer, s_13_8, s_13_9, s_13_7);
        // D s_13_11: write-var gs#898391 <= s_13_10
        fn_state.gs_898391 = s_13_10;
        // N s_13_12: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#898391:bv
        let s_14_0: Bits = fn_state.gs_898391;
        // D s_14_1: cast reint s_14_0 -> u32
        let s_14_1: u32 = (s_14_0.value() as u32);
        // D s_14_2: cast zx s_14_1 -> bv
        let s_14_2: Bits = Bits::new(s_14_1 as u128, 32u16);
        // D s_14_3: call FPNeg(s_14_2)
        let s_14_3: Bits = FPNeg(state, tracer, s_14_2);
        // D s_14_4: write-var gs#898395 <= s_14_3
        fn_state.gs_898395 = s_14_3;
        // N s_14_5: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#898395:bv
        let s_15_0: Bits = fn_state.gs_898395;
        // D s_15_1: cast reint s_15_0 -> u32
        let s_15_1: u32 = (s_15_0.value() as u32);
        // D s_15_2: write-var addend32shadow#7503 <= s_15_1
        fn_state.addend32shadow_7503 = s_15_1;
        // N s_15_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var d:i
        let s_16_0: i128 = fn_state.d;
        // D s_16_1: call S_read(s_16_0)
        let s_16_1: u32 = S_read(state, tracer, s_16_0);
        // C s_16_2: const #() : ()
        let s_16_2: () = ();
        // S s_16_3: call FPSCR_read(s_16_2)
        let s_16_3: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_16_2);
        // D s_16_4: cast zx s_16_1 -> bv
        let s_16_4: Bits = Bits::new(s_16_1 as u128, 32u16);
        // D s_16_5: read-var addend32shadow#7503:u32
        let s_16_5: u32 = fn_state.addend32shadow_7503;
        // D s_16_6: cast zx s_16_5 -> bv
        let s_16_6: Bits = Bits::new(s_16_5 as u128, 32u16);
        // D s_16_7: call FPAdd(s_16_4, s_16_6, s_16_3)
        let s_16_7: Bits = FPAdd(state, tracer, s_16_4, s_16_6, s_16_3);
        // D s_16_8: write-var gs#898403 <= s_16_7
        fn_state.gs_898403 = s_16_7;
        // N s_16_9: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#898403:bv
        let s_17_0: Bits = fn_state.gs_898403;
        // D s_17_1: cast reint s_17_0 -> u32
        let s_17_1: u32 = (s_17_0.value() as u32);
        // D s_17_2: read-var d:i
        let s_17_2: i128 = fn_state.d;
        // D s_17_3: call S_set(s_17_2, s_17_1)
        let s_17_3: () = S_set(state, tracer, s_17_2, s_17_1);
        // N s_17_4: return
        return;
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
        // D s_18_2: call S_read(s_18_1)
        let s_18_2: u32 = S_read(state, tracer, s_18_1);
        // D s_18_3: read-var m:i64
        let s_18_3: i64 = fn_state.m;
        // D s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_5: call S_read(s_18_4)
        let s_18_5: u32 = S_read(state, tracer, s_18_4);
        // C s_18_6: const #() : ()
        let s_18_6: () = ();
        // S s_18_7: call FPSCR_read(s_18_6)
        let s_18_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_18_6);
        // D s_18_8: cast zx s_18_2 -> bv
        let s_18_8: Bits = Bits::new(s_18_2 as u128, 32u16);
        // D s_18_9: cast zx s_18_5 -> bv
        let s_18_9: Bits = Bits::new(s_18_5 as u128, 32u16);
        // D s_18_10: call FPMul(s_18_8, s_18_9, s_18_7)
        let s_18_10: Bits = FPMul(state, tracer, s_18_8, s_18_9, s_18_7);
        // D s_18_11: write-var gs#898398 <= s_18_10
        fn_state.gs_898398 = s_18_10;
        // N s_18_12: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#898398:bv
        let s_19_0: Bits = fn_state.gs_898398;
        // D s_19_1: cast reint s_19_0 -> u32
        let s_19_1: u32 = (s_19_0.value() as u32);
        // D s_19_2: write-var addend32shadow#7503 <= s_19_1
        fn_state.addend32shadow_7503 = s_19_1;
        // N s_19_3: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var esizeshadow#7501:i64
        let s_20_0: i64 = fn_state.esizeshadow_7501;
        // C s_20_1: const #64s : i
        let s_20_1: i128 = 64;
        // D s_20_2: cast zx s_20_0 -> i
        let s_20_2: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_3: cmp-eq s_20_2 s_20_1
        let s_20_3: bool = ((s_20_2) == (s_20_1));
        // D s_20_4: not s_20_3
        let s_20_4: bool = !s_20_3;
        // N s_20_5: branch s_20_4 b29 b21
        if s_20_4 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var add:u8
        let s_21_0: bool = fn_state.add;
        // N s_21_1: branch s_21_0 b27 b22
        if s_21_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var n:i64
        let s_22_0: i64 = fn_state.n;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: call D_read(s_22_1)
        let s_22_2: u64 = D_read(state, tracer, s_22_1);
        // D s_22_3: read-var m:i64
        let s_22_3: i64 = fn_state.m;
        // D s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // D s_22_5: call D_read(s_22_4)
        let s_22_5: u64 = D_read(state, tracer, s_22_4);
        // C s_22_6: const #() : ()
        let s_22_6: () = ();
        // S s_22_7: call FPSCR_read(s_22_6)
        let s_22_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_22_6);
        // D s_22_8: cast zx s_22_2 -> bv
        let s_22_8: Bits = Bits::new(s_22_2 as u128, 64u16);
        // D s_22_9: cast zx s_22_5 -> bv
        let s_22_9: Bits = Bits::new(s_22_5 as u128, 64u16);
        // D s_22_10: call FPMul(s_22_8, s_22_9, s_22_7)
        let s_22_10: Bits = FPMul(state, tracer, s_22_8, s_22_9, s_22_7);
        // D s_22_11: write-var gs#898407 <= s_22_10
        fn_state.gs_898407 = s_22_10;
        // N s_22_12: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#898407:bv
        let s_23_0: Bits = fn_state.gs_898407;
        // D s_23_1: cast reint s_23_0 -> u64
        let s_23_1: u64 = (s_23_0.value() as u64);
        // D s_23_2: cast zx s_23_1 -> bv
        let s_23_2: Bits = Bits::new(s_23_1 as u128, 64u16);
        // D s_23_3: call FPNeg(s_23_2)
        let s_23_3: Bits = FPNeg(state, tracer, s_23_2);
        // D s_23_4: write-var gs#898411 <= s_23_3
        fn_state.gs_898411 = s_23_3;
        // N s_23_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#898411:bv
        let s_24_0: Bits = fn_state.gs_898411;
        // D s_24_1: cast reint s_24_0 -> u64
        let s_24_1: u64 = (s_24_0.value() as u64);
        // D s_24_2: write-var addend64shadow#7504 <= s_24_1
        fn_state.addend64shadow_7504 = s_24_1;
        // N s_24_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var d:i
        let s_25_0: i128 = fn_state.d;
        // D s_25_1: call D_read(s_25_0)
        let s_25_1: u64 = D_read(state, tracer, s_25_0);
        // C s_25_2: const #() : ()
        let s_25_2: () = ();
        // S s_25_3: call FPSCR_read(s_25_2)
        let s_25_3: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_25_2);
        // D s_25_4: cast zx s_25_1 -> bv
        let s_25_4: Bits = Bits::new(s_25_1 as u128, 64u16);
        // D s_25_5: read-var addend64shadow#7504:u64
        let s_25_5: u64 = fn_state.addend64shadow_7504;
        // D s_25_6: cast zx s_25_5 -> bv
        let s_25_6: Bits = Bits::new(s_25_5 as u128, 64u16);
        // D s_25_7: call FPAdd(s_25_4, s_25_6, s_25_3)
        let s_25_7: Bits = FPAdd(state, tracer, s_25_4, s_25_6, s_25_3);
        // D s_25_8: write-var gs#898419 <= s_25_7
        fn_state.gs_898419 = s_25_7;
        // N s_25_9: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#898419:bv
        let s_26_0: Bits = fn_state.gs_898419;
        // D s_26_1: cast reint s_26_0 -> u64
        let s_26_1: u64 = (s_26_0.value() as u64);
        // D s_26_2: read-var d:i
        let s_26_2: i128 = fn_state.d;
        // D s_26_3: call D_set(s_26_2, s_26_1)
        let s_26_3: () = D_set(state, tracer, s_26_2, s_26_1);
        // N s_26_4: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var n:i64
        let s_27_0: i64 = fn_state.n;
        // D s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // D s_27_2: call D_read(s_27_1)
        let s_27_2: u64 = D_read(state, tracer, s_27_1);
        // D s_27_3: read-var m:i64
        let s_27_3: i64 = fn_state.m;
        // D s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_5: call D_read(s_27_4)
        let s_27_5: u64 = D_read(state, tracer, s_27_4);
        // C s_27_6: const #() : ()
        let s_27_6: () = ();
        // S s_27_7: call FPSCR_read(s_27_6)
        let s_27_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_27_6);
        // D s_27_8: cast zx s_27_2 -> bv
        let s_27_8: Bits = Bits::new(s_27_2 as u128, 64u16);
        // D s_27_9: cast zx s_27_5 -> bv
        let s_27_9: Bits = Bits::new(s_27_5 as u128, 64u16);
        // D s_27_10: call FPMul(s_27_8, s_27_9, s_27_7)
        let s_27_10: Bits = FPMul(state, tracer, s_27_8, s_27_9, s_27_7);
        // D s_27_11: write-var gs#898414 <= s_27_10
        fn_state.gs_898414 = s_27_10;
        // N s_27_12: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#898414:bv
        let s_28_0: Bits = fn_state.gs_898414;
        // D s_28_1: cast reint s_28_0 -> u64
        let s_28_1: u64 = (s_28_0.value() as u64);
        // D s_28_2: write-var addend64shadow#7504 <= s_28_1
        fn_state.addend64shadow_7504 = s_28_1;
        // N s_28_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_29_0: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0s : i64
        let s_30_0: i64 = 0;
        // C s_30_1: const #1s : i
        let s_30_1: i128 = 1;
        // D s_30_2: read-var regs:i
        let s_30_2: i128 = fn_state.regs;
        // D s_30_3: sub s_30_2 s_30_1
        let s_30_3: i128 = ((s_30_2) - (s_30_1));
        // D s_30_4: cast reint s_30_3 -> i64
        let s_30_4: i64 = (s_30_3 as i64);
        // D s_30_5: write-var gs#312156 <= s_30_4
        fn_state.gs_312156 = s_30_4;
        // D s_30_6: write-var r <= s_30_0
        fn_state.r = s_30_0;
        // N s_30_7: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var r:i64
        let s_31_0: i64 = fn_state.r;
        // D s_31_1: read-var gs#312156:i64
        let s_31_1: i64 = fn_state.gs_312156;
        // D s_31_2: cmp-gt s_31_0 s_31_1
        let s_31_2: bool = ((s_31_0) > (s_31_1));
        // N s_31_3: branch s_31_2 b41 b32
        if s_31_2 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0s : i64
        let s_32_0: i64 = 0;
        // C s_32_1: const #1s : i
        let s_32_1: i128 = 1;
        // D s_32_2: read-var elements:i
        let s_32_2: i128 = fn_state.elements;
        // D s_32_3: sub s_32_2 s_32_1
        let s_32_3: i128 = ((s_32_2) - (s_32_1));
        // D s_32_4: cast reint s_32_3 -> i64
        let s_32_4: i64 = (s_32_3 as i64);
        // D s_32_5: write-var gs#312162 <= s_32_4
        fn_state.gs_312162 = s_32_4;
        // D s_32_6: write-var e <= s_32_0
        fn_state.e = s_32_0;
        // N s_32_7: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var e:i64
        let s_33_0: i64 = fn_state.e;
        // D s_33_1: read-var gs#312162:i64
        let s_33_1: i64 = fn_state.gs_312162;
        // D s_33_2: cmp-gt s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) > (s_33_1));
        // N s_33_3: branch s_33_2 b40 b34
        if s_33_2 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var n:i64
        let s_34_0: i64 = fn_state.n;
        // D s_34_1: cast zx s_34_0 -> i
        let s_34_1: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_2: read-var r:i64
        let s_34_2: i64 = fn_state.r;
        // D s_34_3: cast zx s_34_2 -> i
        let s_34_3: i128 = (i128::try_from(s_34_2).unwrap());
        // D s_34_4: add s_34_1 s_34_3
        let s_34_4: i128 = (s_34_1 + s_34_3);
        // D s_34_5: call D_read(s_34_4)
        let s_34_5: u64 = D_read(state, tracer, s_34_4);
        // D s_34_6: read-var esizeshadow#7501:i64
        let s_34_6: i64 = fn_state.esizeshadow_7501;
        // D s_34_7: cast zx s_34_6 -> i
        let s_34_7: i128 = (i128::try_from(s_34_6).unwrap());
        // D s_34_8: cast reint s_34_7 -> i64
        let s_34_8: i64 = (s_34_7 as i64);
        // D s_34_9: cast zx s_34_5 -> bv
        let s_34_9: Bits = Bits::new(s_34_5 as u128, 64u16);
        // D s_34_10: read-var e:i64
        let s_34_10: i64 = fn_state.e;
        // D s_34_11: cast zx s_34_10 -> i
        let s_34_11: i128 = (i128::try_from(s_34_10).unwrap());
        // D s_34_12: cast zx s_34_8 -> i
        let s_34_12: i128 = (i128::try_from(s_34_8).unwrap());
        // D s_34_13: call Elem_read(s_34_9, s_34_11, s_34_12)
        let s_34_13: Bits = Elem_read(state, tracer, s_34_9, s_34_11, s_34_12);
        // D s_34_14: read-var m:i64
        let s_34_14: i64 = fn_state.m;
        // D s_34_15: cast zx s_34_14 -> i
        let s_34_15: i128 = (i128::try_from(s_34_14).unwrap());
        // D s_34_16: read-var r:i64
        let s_34_16: i64 = fn_state.r;
        // D s_34_17: cast zx s_34_16 -> i
        let s_34_17: i128 = (i128::try_from(s_34_16).unwrap());
        // D s_34_18: add s_34_15 s_34_17
        let s_34_18: i128 = (s_34_15 + s_34_17);
        // D s_34_19: call D_read(s_34_18)
        let s_34_19: u64 = D_read(state, tracer, s_34_18);
        // D s_34_20: read-var esizeshadow#7501:i64
        let s_34_20: i64 = fn_state.esizeshadow_7501;
        // D s_34_21: cast zx s_34_20 -> i
        let s_34_21: i128 = (i128::try_from(s_34_20).unwrap());
        // D s_34_22: cast reint s_34_21 -> i64
        let s_34_22: i64 = (s_34_21 as i64);
        // D s_34_23: cast zx s_34_19 -> bv
        let s_34_23: Bits = Bits::new(s_34_19 as u128, 64u16);
        // D s_34_24: read-var e:i64
        let s_34_24: i64 = fn_state.e;
        // D s_34_25: cast zx s_34_24 -> i
        let s_34_25: i128 = (i128::try_from(s_34_24).unwrap());
        // D s_34_26: cast zx s_34_22 -> i
        let s_34_26: i128 = (i128::try_from(s_34_22).unwrap());
        // D s_34_27: call Elem_read(s_34_23, s_34_25, s_34_26)
        let s_34_27: Bits = Elem_read(state, tracer, s_34_23, s_34_25, s_34_26);
        // C s_34_28: const #() : ()
        let s_34_28: () = ();
        // S s_34_29: call StandardFPSCRValue(s_34_28)
        let s_34_29: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_34_28,
        );
        // D s_34_30: call FPMul(s_34_13, s_34_27, s_34_29)
        let s_34_30: Bits = FPMul(state, tracer, s_34_13, s_34_27, s_34_29);
        // D s_34_31: write-var product <= s_34_30
        fn_state.product = s_34_30;
        // N s_34_32: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var add:u8
        let s_35_0: bool = fn_state.add;
        // N s_35_1: branch s_35_0 b39 b36
        if s_35_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var product:bv
        let s_36_0: Bits = fn_state.product;
        // D s_36_1: call FPNeg(s_36_0)
        let s_36_1: Bits = FPNeg(state, tracer, s_36_0);
        // D s_36_2: write-var addend <= s_36_1
        fn_state.addend = s_36_1;
        // N s_36_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var r:i64
        let s_37_0: i64 = fn_state.r;
        // D s_37_1: cast zx s_37_0 -> i
        let s_37_1: i128 = (i128::try_from(s_37_0).unwrap());
        // D s_37_2: read-var d:i
        let s_37_2: i128 = fn_state.d;
        // D s_37_3: add s_37_2 s_37_1
        let s_37_3: i128 = (s_37_2 + s_37_1);
        // D s_37_4: write-var ga#355434 <= s_37_3
        fn_state.ga_355434 = s_37_3;
        // D s_37_5: read-var r:i64
        let s_37_5: i64 = fn_state.r;
        // D s_37_6: cast zx s_37_5 -> i
        let s_37_6: i128 = (i128::try_from(s_37_5).unwrap());
        // D s_37_7: read-var d:i
        let s_37_7: i128 = fn_state.d;
        // D s_37_8: add s_37_7 s_37_6
        let s_37_8: i128 = (s_37_7 + s_37_6);
        // D s_37_9: call D_read(s_37_8)
        let s_37_9: u64 = D_read(state, tracer, s_37_8);
        // D s_37_10: write-var ga#355431 <= s_37_9
        fn_state.ga_355431 = s_37_9;
        // D s_37_11: read-var esizeshadow#7501:i64
        let s_37_11: i64 = fn_state.esizeshadow_7501;
        // D s_37_12: cast zx s_37_11 -> i
        let s_37_12: i128 = (i128::try_from(s_37_11).unwrap());
        // D s_37_13: cast reint s_37_12 -> i64
        let s_37_13: i64 = (s_37_12 as i64);
        // D s_37_14: write-var ga#355432 <= s_37_13
        fn_state.ga_355432 = s_37_13;
        // D s_37_15: read-var r:i64
        let s_37_15: i64 = fn_state.r;
        // D s_37_16: cast zx s_37_15 -> i
        let s_37_16: i128 = (i128::try_from(s_37_15).unwrap());
        // D s_37_17: read-var d:i
        let s_37_17: i128 = fn_state.d;
        // D s_37_18: add s_37_17 s_37_16
        let s_37_18: i128 = (s_37_17 + s_37_16);
        // D s_37_19: call D_read(s_37_18)
        let s_37_19: u64 = D_read(state, tracer, s_37_18);
        // D s_37_20: read-var esizeshadow#7501:i64
        let s_37_20: i64 = fn_state.esizeshadow_7501;
        // D s_37_21: cast zx s_37_20 -> i
        let s_37_21: i128 = (i128::try_from(s_37_20).unwrap());
        // D s_37_22: cast reint s_37_21 -> i64
        let s_37_22: i64 = (s_37_21 as i64);
        // D s_37_23: cast zx s_37_19 -> bv
        let s_37_23: Bits = Bits::new(s_37_19 as u128, 64u16);
        // D s_37_24: read-var e:i64
        let s_37_24: i64 = fn_state.e;
        // D s_37_25: cast zx s_37_24 -> i
        let s_37_25: i128 = (i128::try_from(s_37_24).unwrap());
        // D s_37_26: cast zx s_37_22 -> i
        let s_37_26: i128 = (i128::try_from(s_37_22).unwrap());
        // D s_37_27: call Elem_read(s_37_23, s_37_25, s_37_26)
        let s_37_27: Bits = Elem_read(state, tracer, s_37_23, s_37_25, s_37_26);
        // C s_37_28: const #() : ()
        let s_37_28: () = ();
        // S s_37_29: call StandardFPSCRValue(s_37_28)
        let s_37_29: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_37_28,
        );
        // D s_37_30: read-var addend:bv
        let s_37_30: Bits = fn_state.addend;
        // D s_37_31: call FPAdd(s_37_27, s_37_30, s_37_29)
        let s_37_31: Bits = FPAdd(state, tracer, s_37_27, s_37_30, s_37_29);
        // D s_37_32: write-var ga#355433 <= s_37_31
        fn_state.ga_355433 = s_37_31;
        // N s_37_33: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var ga#355431:u64
        let s_38_0: u64 = fn_state.ga_355431;
        // D s_38_1: cast zx s_38_0 -> bv
        let s_38_1: Bits = Bits::new(s_38_0 as u128, 64u16);
        // D s_38_2: read-var e:i64
        let s_38_2: i64 = fn_state.e;
        // D s_38_3: cast zx s_38_2 -> i
        let s_38_3: i128 = (i128::try_from(s_38_2).unwrap());
        // D s_38_4: read-var ga#355432:i64
        let s_38_4: i64 = fn_state.ga_355432;
        // D s_38_5: cast zx s_38_4 -> i
        let s_38_5: i128 = (i128::try_from(s_38_4).unwrap());
        // D s_38_6: read-var ga#355433:bv
        let s_38_6: Bits = fn_state.ga_355433;
        // D s_38_7: call Elem_set(s_38_1, s_38_3, s_38_5, s_38_6)
        let s_38_7: Bits = Elem_set(state, tracer, s_38_1, s_38_3, s_38_5, s_38_6);
        // D s_38_8: cast reint s_38_7 -> u64
        let s_38_8: u64 = (s_38_7.value() as u64);
        // D s_38_9: read-var ga#355434:i
        let s_38_9: i128 = fn_state.ga_355434;
        // D s_38_10: call D_set(s_38_9, s_38_8)
        let s_38_10: () = D_set(state, tracer, s_38_9, s_38_8);
        // D s_38_11: read-var e:i64
        let s_38_11: i64 = fn_state.e;
        // C s_38_12: const #1s : i64
        let s_38_12: i64 = 1;
        // D s_38_13: add s_38_11 s_38_12
        let s_38_13: i64 = (s_38_11 + s_38_12);
        // D s_38_14: write-var e <= s_38_13
        fn_state.e = s_38_13;
        // N s_38_15: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var product:bv
        let s_39_0: Bits = fn_state.product;
        // D s_39_1: write-var addend <= s_39_0
        fn_state.addend = s_39_0;
        // N s_39_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var r:i64
        let s_40_0: i64 = fn_state.r;
        // C s_40_1: const #1s : i64
        let s_40_1: i64 = 1;
        // D s_40_2: add s_40_0 s_40_1
        let s_40_2: i64 = (s_40_0 + s_40_1);
        // D s_40_3: write-var r <= s_40_2
        fn_state.r = s_40_2;
        // N s_40_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_41_0: return
        return;
    }
}
