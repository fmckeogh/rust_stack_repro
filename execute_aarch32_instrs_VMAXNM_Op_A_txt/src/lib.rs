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
use FPMinNum::*;
use Elem_set::*;
use CheckAdvSIMDOrVFPEnabled::*;
use FPMaxNum::*;
use FPSCR_read::*;
use D_set::*;
use S_set::*;
use Elem_read::*;
use S_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VMAXNM_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    advsimd: bool,
    d__arg: i64,
    elements: i128,
    esize: i64,
    m: i64,
    maximum: bool,
    n: i64,
    regs: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        r: i64,
        gs_914878: Bits,
        e: i64,
        gs_325249: i64,
        ga_365729: u64,
        ga_365730: i64,
        ga_365732: i128,
        gs_914866: Bits,
        esizeshadow_7922: i64,
        ga_365737: i64,
        ga_365738: Bits,
        gs_914873: Bits,
        gs_914889: Bits,
        op2: Bits,
        ga_365739: i128,
        gs_325243: i64,
        d: i128,
        op1: Bits,
        ga_365736: u64,
        ga_365731: Bits,
        gs_914854: Bits,
        gs_914884: Bits,
        advsimd: bool,
        d__arg: i64,
        elements: i128,
        esize: i64,
        m: i64,
        maximum: bool,
        n: i64,
        regs: i128,
    }
    let fn_state = FunctionState {
        advsimd,
        d__arg,
        elements,
        esize,
        m,
        maximum,
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
        // D s_0_3: write-var esizeshadow#7922 <= s_0_2
        fn_state.esizeshadow_7922 = s_0_2;
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
        // N s_1_1: branch s_1_0 b21 b2
        if s_1_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var esizeshadow#7922:i64
        let s_2_0: i64 = fn_state.esizeshadow_7922;
        // C s_2_1: const #16s : i
        let s_2_1: i128 = 16;
        // D s_2_2: cast zx s_2_0 -> i
        let s_2_2: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_1
        let s_2_3: bool = ((s_2_2) == (s_2_1));
        // D s_2_4: not s_2_3
        let s_2_4: bool = !s_2_3;
        // N s_2_5: branch s_2_4 b8 b3
        if s_2_4 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var maximum:u8
        let s_3_0: bool = fn_state.maximum;
        // N s_3_1: branch s_3_0 b6 b4
        if s_3_0 {
            return block_6(state, tracer, fn_state);
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
        // D s_4_26: call FPMinNum(s_4_24, s_4_25, s_4_23)
        let s_4_26: Bits = FPMinNum(state, tracer, s_4_24, s_4_25, s_4_23);
        // D s_4_27: write-var gs#914854 <= s_4_26
        fn_state.gs_914854 = s_4_26;
        // N s_4_28: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#914854:bv
        let s_5_0: Bits = fn_state.gs_914854;
        // D s_5_1: cast reint s_5_0 -> u16
        let s_5_1: u16 = (s_5_0.value() as u16);
        // C s_5_2: const #0u : u16
        let s_5_2: u16 = 0;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 16u16);
        // D s_5_4: cast zx s_5_1 -> bv
        let s_5_4: Bits = Bits::new(s_5_1 as u128, 16u16);
        // C s_5_5: cast reint s_5_3 -> u128
        let s_5_5: u128 = (s_5_3.value() as u128);
        // D s_5_6: size-of s_5_3
        let s_5_6: u16 = s_5_3.length();
        // D s_5_7: cast reint s_5_4 -> u128
        let s_5_7: u128 = (s_5_4.value() as u128);
        // D s_5_8: size-of s_5_4
        let s_5_8: u16 = s_5_4.length();
        // D s_5_9: lsl s_5_5 s_5_8
        let s_5_9: u128 = s_5_5 << s_5_8;
        // D s_5_10: or s_5_9 s_5_7
        let s_5_10: u128 = ((s_5_9) | (s_5_7));
        // D s_5_11: add s_5_6 s_5_8
        let s_5_11: u16 = (s_5_6 + s_5_8);
        // D s_5_12: create-bits s_5_10 s_5_11
        let s_5_12: Bits = Bits::new(s_5_10, s_5_11);
        // D s_5_13: cast reint s_5_12 -> u32
        let s_5_13: u32 = (s_5_12.value() as u32);
        // D s_5_14: read-var d:i
        let s_5_14: i128 = fn_state.d;
        // D s_5_15: call S_set(s_5_14, s_5_13)
        let s_5_15: () = S_set(state, tracer, s_5_14, s_5_13);
        // N s_5_16: return
        return;
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
        // C s_6_3: const #0s : i
        let s_6_3: i128 = 0;
        // D s_6_4: cast zx s_6_2 -> bv
        let s_6_4: Bits = Bits::new(s_6_2 as u128, 32u16);
        // C s_6_5: const #1s : i64
        let s_6_5: i64 = 1;
        // C s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // C s_6_7: const #15s : i
        let s_6_7: i128 = 15;
        // C s_6_8: add s_6_7 s_6_6
        let s_6_8: i128 = (s_6_7 + s_6_6);
        // D s_6_9: bit-extract s_6_4 s_6_3 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_4) >> (s_6_3)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u16
        let s_6_10: u16 = (s_6_9.value() as u16);
        // D s_6_11: read-var m:i64
        let s_6_11: i64 = fn_state.m;
        // D s_6_12: cast zx s_6_11 -> i
        let s_6_12: i128 = (i128::try_from(s_6_11).unwrap());
        // D s_6_13: call S_read(s_6_12)
        let s_6_13: u32 = S_read(state, tracer, s_6_12);
        // C s_6_14: const #0s : i
        let s_6_14: i128 = 0;
        // D s_6_15: cast zx s_6_13 -> bv
        let s_6_15: Bits = Bits::new(s_6_13 as u128, 32u16);
        // C s_6_16: const #1s : i64
        let s_6_16: i64 = 1;
        // C s_6_17: cast zx s_6_16 -> i
        let s_6_17: i128 = (i128::try_from(s_6_16).unwrap());
        // C s_6_18: const #15s : i
        let s_6_18: i128 = 15;
        // C s_6_19: add s_6_18 s_6_17
        let s_6_19: i128 = (s_6_18 + s_6_17);
        // D s_6_20: bit-extract s_6_15 s_6_14 s_6_19
        let s_6_20: Bits = (Bits::new(
            ((s_6_15) >> (s_6_14)).value(),
            u16::try_from(s_6_19).unwrap(),
        ));
        // D s_6_21: cast reint s_6_20 -> u16
        let s_6_21: u16 = (s_6_20.value() as u16);
        // C s_6_22: const #() : ()
        let s_6_22: () = ();
        // S s_6_23: call FPSCR_read(s_6_22)
        let s_6_23: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_6_22);
        // D s_6_24: cast zx s_6_10 -> bv
        let s_6_24: Bits = Bits::new(s_6_10 as u128, 16u16);
        // D s_6_25: cast zx s_6_21 -> bv
        let s_6_25: Bits = Bits::new(s_6_21 as u128, 16u16);
        // D s_6_26: call FPMaxNum(s_6_24, s_6_25, s_6_23)
        let s_6_26: Bits = FPMaxNum(state, tracer, s_6_24, s_6_25, s_6_23);
        // D s_6_27: write-var gs#914866 <= s_6_26
        fn_state.gs_914866 = s_6_26;
        // N s_6_28: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#914866:bv
        let s_7_0: Bits = fn_state.gs_914866;
        // D s_7_1: cast reint s_7_0 -> u16
        let s_7_1: u16 = (s_7_0.value() as u16);
        // C s_7_2: const #0u : u16
        let s_7_2: u16 = 0;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 16u16);
        // D s_7_4: cast zx s_7_1 -> bv
        let s_7_4: Bits = Bits::new(s_7_1 as u128, 16u16);
        // C s_7_5: cast reint s_7_3 -> u128
        let s_7_5: u128 = (s_7_3.value() as u128);
        // D s_7_6: size-of s_7_3
        let s_7_6: u16 = s_7_3.length();
        // D s_7_7: cast reint s_7_4 -> u128
        let s_7_7: u128 = (s_7_4.value() as u128);
        // D s_7_8: size-of s_7_4
        let s_7_8: u16 = s_7_4.length();
        // D s_7_9: lsl s_7_5 s_7_8
        let s_7_9: u128 = s_7_5 << s_7_8;
        // D s_7_10: or s_7_9 s_7_7
        let s_7_10: u128 = ((s_7_9) | (s_7_7));
        // D s_7_11: add s_7_6 s_7_8
        let s_7_11: u16 = (s_7_6 + s_7_8);
        // D s_7_12: create-bits s_7_10 s_7_11
        let s_7_12: Bits = Bits::new(s_7_10, s_7_11);
        // D s_7_13: cast reint s_7_12 -> u32
        let s_7_13: u32 = (s_7_12.value() as u32);
        // D s_7_14: read-var d:i
        let s_7_14: i128 = fn_state.d;
        // D s_7_15: call S_set(s_7_14, s_7_13)
        let s_7_15: () = S_set(state, tracer, s_7_14, s_7_13);
        // N s_7_16: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var esizeshadow#7922:i64
        let s_8_0: i64 = fn_state.esizeshadow_7922;
        // C s_8_1: const #32s : i
        let s_8_1: i128 = 32;
        // D s_8_2: cast zx s_8_0 -> i
        let s_8_2: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_3: cmp-eq s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) == (s_8_1));
        // D s_8_4: not s_8_3
        let s_8_4: bool = !s_8_3;
        // N s_8_5: branch s_8_4 b14 b9
        if s_8_4 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var maximum:u8
        let s_9_0: bool = fn_state.maximum;
        // N s_9_1: branch s_9_0 b12 b10
        if s_9_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var n:i64
        let s_10_0: i64 = fn_state.n;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: call S_read(s_10_1)
        let s_10_2: u32 = S_read(state, tracer, s_10_1);
        // D s_10_3: read-var m:i64
        let s_10_3: i64 = fn_state.m;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: call S_read(s_10_4)
        let s_10_5: u32 = S_read(state, tracer, s_10_4);
        // C s_10_6: const #() : ()
        let s_10_6: () = ();
        // S s_10_7: call FPSCR_read(s_10_6)
        let s_10_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_10_6);
        // D s_10_8: cast zx s_10_2 -> bv
        let s_10_8: Bits = Bits::new(s_10_2 as u128, 32u16);
        // D s_10_9: cast zx s_10_5 -> bv
        let s_10_9: Bits = Bits::new(s_10_5 as u128, 32u16);
        // D s_10_10: call FPMinNum(s_10_8, s_10_9, s_10_7)
        let s_10_10: Bits = FPMinNum(state, tracer, s_10_8, s_10_9, s_10_7);
        // D s_10_11: write-var gs#914873 <= s_10_10
        fn_state.gs_914873 = s_10_10;
        // N s_10_12: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#914873:bv
        let s_11_0: Bits = fn_state.gs_914873;
        // D s_11_1: cast reint s_11_0 -> u32
        let s_11_1: u32 = (s_11_0.value() as u32);
        // D s_11_2: read-var d:i
        let s_11_2: i128 = fn_state.d;
        // D s_11_3: call S_set(s_11_2, s_11_1)
        let s_11_3: () = S_set(state, tracer, s_11_2, s_11_1);
        // N s_11_4: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var n:i64
        let s_12_0: i64 = fn_state.n;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: call S_read(s_12_1)
        let s_12_2: u32 = S_read(state, tracer, s_12_1);
        // D s_12_3: read-var m:i64
        let s_12_3: i64 = fn_state.m;
        // D s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_5: call S_read(s_12_4)
        let s_12_5: u32 = S_read(state, tracer, s_12_4);
        // C s_12_6: const #() : ()
        let s_12_6: () = ();
        // S s_12_7: call FPSCR_read(s_12_6)
        let s_12_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_12_6);
        // D s_12_8: cast zx s_12_2 -> bv
        let s_12_8: Bits = Bits::new(s_12_2 as u128, 32u16);
        // D s_12_9: cast zx s_12_5 -> bv
        let s_12_9: Bits = Bits::new(s_12_5 as u128, 32u16);
        // D s_12_10: call FPMaxNum(s_12_8, s_12_9, s_12_7)
        let s_12_10: Bits = FPMaxNum(state, tracer, s_12_8, s_12_9, s_12_7);
        // D s_12_11: write-var gs#914878 <= s_12_10
        fn_state.gs_914878 = s_12_10;
        // N s_12_12: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#914878:bv
        let s_13_0: Bits = fn_state.gs_914878;
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
        // D s_14_0: read-var esizeshadow#7922:i64
        let s_14_0: i64 = fn_state.esizeshadow_7922;
        // C s_14_1: const #64s : i
        let s_14_1: i128 = 64;
        // D s_14_2: cast zx s_14_0 -> i
        let s_14_2: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_3: cmp-eq s_14_2 s_14_1
        let s_14_3: bool = ((s_14_2) == (s_14_1));
        // D s_14_4: not s_14_3
        let s_14_4: bool = !s_14_3;
        // N s_14_5: branch s_14_4 b20 b15
        if s_14_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var maximum:u8
        let s_15_0: bool = fn_state.maximum;
        // N s_15_1: branch s_15_0 b18 b16
        if s_15_0 {
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
        // D s_16_0: read-var n:i64
        let s_16_0: i64 = fn_state.n;
        // D s_16_1: cast zx s_16_0 -> i
        let s_16_1: i128 = (i128::try_from(s_16_0).unwrap());
        // D s_16_2: call D_read(s_16_1)
        let s_16_2: u64 = D_read(state, tracer, s_16_1);
        // D s_16_3: read-var m:i64
        let s_16_3: i64 = fn_state.m;
        // D s_16_4: cast zx s_16_3 -> i
        let s_16_4: i128 = (i128::try_from(s_16_3).unwrap());
        // D s_16_5: call D_read(s_16_4)
        let s_16_5: u64 = D_read(state, tracer, s_16_4);
        // C s_16_6: const #() : ()
        let s_16_6: () = ();
        // S s_16_7: call FPSCR_read(s_16_6)
        let s_16_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_16_6);
        // D s_16_8: cast zx s_16_2 -> bv
        let s_16_8: Bits = Bits::new(s_16_2 as u128, 64u16);
        // D s_16_9: cast zx s_16_5 -> bv
        let s_16_9: Bits = Bits::new(s_16_5 as u128, 64u16);
        // D s_16_10: call FPMinNum(s_16_8, s_16_9, s_16_7)
        let s_16_10: Bits = FPMinNum(state, tracer, s_16_8, s_16_9, s_16_7);
        // D s_16_11: write-var gs#914884 <= s_16_10
        fn_state.gs_914884 = s_16_10;
        // N s_16_12: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#914884:bv
        let s_17_0: Bits = fn_state.gs_914884;
        // D s_17_1: cast reint s_17_0 -> u64
        let s_17_1: u64 = (s_17_0.value() as u64);
        // D s_17_2: read-var d:i
        let s_17_2: i128 = fn_state.d;
        // D s_17_3: call D_set(s_17_2, s_17_1)
        let s_17_3: () = D_set(state, tracer, s_17_2, s_17_1);
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
        // D s_18_2: call D_read(s_18_1)
        let s_18_2: u64 = D_read(state, tracer, s_18_1);
        // D s_18_3: read-var m:i64
        let s_18_3: i64 = fn_state.m;
        // D s_18_4: cast zx s_18_3 -> i
        let s_18_4: i128 = (i128::try_from(s_18_3).unwrap());
        // D s_18_5: call D_read(s_18_4)
        let s_18_5: u64 = D_read(state, tracer, s_18_4);
        // C s_18_6: const #() : ()
        let s_18_6: () = ();
        // S s_18_7: call FPSCR_read(s_18_6)
        let s_18_7: ProductType5c790c8ef59cc8b2 = FPSCR_read(state, tracer, s_18_6);
        // D s_18_8: cast zx s_18_2 -> bv
        let s_18_8: Bits = Bits::new(s_18_2 as u128, 64u16);
        // D s_18_9: cast zx s_18_5 -> bv
        let s_18_9: Bits = Bits::new(s_18_5 as u128, 64u16);
        // D s_18_10: call FPMaxNum(s_18_8, s_18_9, s_18_7)
        let s_18_10: Bits = FPMaxNum(state, tracer, s_18_8, s_18_9, s_18_7);
        // D s_18_11: write-var gs#914889 <= s_18_10
        fn_state.gs_914889 = s_18_10;
        // N s_18_12: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#914889:bv
        let s_19_0: Bits = fn_state.gs_914889;
        // D s_19_1: cast reint s_19_0 -> u64
        let s_19_1: u64 = (s_19_0.value() as u64);
        // D s_19_2: read-var d:i
        let s_19_2: i128 = fn_state.d;
        // D s_19_3: call D_set(s_19_2, s_19_1)
        let s_19_3: () = D_set(state, tracer, s_19_2, s_19_1);
        // N s_19_4: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0s : i64
        let s_21_0: i64 = 0;
        // C s_21_1: const #1s : i
        let s_21_1: i128 = 1;
        // D s_21_2: read-var regs:i
        let s_21_2: i128 = fn_state.regs;
        // D s_21_3: sub s_21_2 s_21_1
        let s_21_3: i128 = ((s_21_2) - (s_21_1));
        // D s_21_4: cast reint s_21_3 -> i64
        let s_21_4: i64 = (s_21_3 as i64);
        // D s_21_5: write-var gs#325243 <= s_21_4
        fn_state.gs_325243 = s_21_4;
        // D s_21_6: write-var r <= s_21_0
        fn_state.r = s_21_0;
        // N s_21_7: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var r:i64
        let s_22_0: i64 = fn_state.r;
        // D s_22_1: read-var gs#325243:i64
        let s_22_1: i64 = fn_state.gs_325243;
        // D s_22_2: cmp-gt s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) > (s_22_1));
        // N s_22_3: branch s_22_2 b32 b23
        if s_22_2 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0s : i64
        let s_23_0: i64 = 0;
        // C s_23_1: const #1s : i
        let s_23_1: i128 = 1;
        // D s_23_2: read-var elements:i
        let s_23_2: i128 = fn_state.elements;
        // D s_23_3: sub s_23_2 s_23_1
        let s_23_3: i128 = ((s_23_2) - (s_23_1));
        // D s_23_4: cast reint s_23_3 -> i64
        let s_23_4: i64 = (s_23_3 as i64);
        // D s_23_5: write-var gs#325249 <= s_23_4
        fn_state.gs_325249 = s_23_4;
        // D s_23_6: write-var e <= s_23_0
        fn_state.e = s_23_0;
        // N s_23_7: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var e:i64
        let s_24_0: i64 = fn_state.e;
        // D s_24_1: read-var gs#325249:i64
        let s_24_1: i64 = fn_state.gs_325249;
        // D s_24_2: cmp-gt s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) > (s_24_1));
        // N s_24_3: branch s_24_2 b31 b25
        if s_24_2 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var n:i64
        let s_25_0: i64 = fn_state.n;
        // D s_25_1: cast zx s_25_0 -> i
        let s_25_1: i128 = (i128::try_from(s_25_0).unwrap());
        // D s_25_2: read-var r:i64
        let s_25_2: i64 = fn_state.r;
        // D s_25_3: cast zx s_25_2 -> i
        let s_25_3: i128 = (i128::try_from(s_25_2).unwrap());
        // D s_25_4: add s_25_1 s_25_3
        let s_25_4: i128 = (s_25_1 + s_25_3);
        // D s_25_5: call D_read(s_25_4)
        let s_25_5: u64 = D_read(state, tracer, s_25_4);
        // D s_25_6: read-var esizeshadow#7922:i64
        let s_25_6: i64 = fn_state.esizeshadow_7922;
        // D s_25_7: cast zx s_25_6 -> i
        let s_25_7: i128 = (i128::try_from(s_25_6).unwrap());
        // D s_25_8: cast reint s_25_7 -> i64
        let s_25_8: i64 = (s_25_7 as i64);
        // D s_25_9: cast zx s_25_5 -> bv
        let s_25_9: Bits = Bits::new(s_25_5 as u128, 64u16);
        // D s_25_10: read-var e:i64
        let s_25_10: i64 = fn_state.e;
        // D s_25_11: cast zx s_25_10 -> i
        let s_25_11: i128 = (i128::try_from(s_25_10).unwrap());
        // D s_25_12: cast zx s_25_8 -> i
        let s_25_12: i128 = (i128::try_from(s_25_8).unwrap());
        // D s_25_13: call Elem_read(s_25_9, s_25_11, s_25_12)
        let s_25_13: Bits = Elem_read(state, tracer, s_25_9, s_25_11, s_25_12);
        // D s_25_14: write-var op1 <= s_25_13
        fn_state.op1 = s_25_13;
        // D s_25_15: read-var m:i64
        let s_25_15: i64 = fn_state.m;
        // D s_25_16: cast zx s_25_15 -> i
        let s_25_16: i128 = (i128::try_from(s_25_15).unwrap());
        // D s_25_17: read-var r:i64
        let s_25_17: i64 = fn_state.r;
        // D s_25_18: cast zx s_25_17 -> i
        let s_25_18: i128 = (i128::try_from(s_25_17).unwrap());
        // D s_25_19: add s_25_16 s_25_18
        let s_25_19: i128 = (s_25_16 + s_25_18);
        // D s_25_20: call D_read(s_25_19)
        let s_25_20: u64 = D_read(state, tracer, s_25_19);
        // D s_25_21: read-var esizeshadow#7922:i64
        let s_25_21: i64 = fn_state.esizeshadow_7922;
        // D s_25_22: cast zx s_25_21 -> i
        let s_25_22: i128 = (i128::try_from(s_25_21).unwrap());
        // D s_25_23: cast reint s_25_22 -> i64
        let s_25_23: i64 = (s_25_22 as i64);
        // D s_25_24: cast zx s_25_20 -> bv
        let s_25_24: Bits = Bits::new(s_25_20 as u128, 64u16);
        // D s_25_25: read-var e:i64
        let s_25_25: i64 = fn_state.e;
        // D s_25_26: cast zx s_25_25 -> i
        let s_25_26: i128 = (i128::try_from(s_25_25).unwrap());
        // D s_25_27: cast zx s_25_23 -> i
        let s_25_27: i128 = (i128::try_from(s_25_23).unwrap());
        // D s_25_28: call Elem_read(s_25_24, s_25_26, s_25_27)
        let s_25_28: Bits = Elem_read(state, tracer, s_25_24, s_25_26, s_25_27);
        // D s_25_29: write-var op2 <= s_25_28
        fn_state.op2 = s_25_28;
        // D s_25_30: read-var maximum:u8
        let s_25_30: bool = fn_state.maximum;
        // N s_25_31: branch s_25_30 b29 b26
        if s_25_30 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var r:i64
        let s_26_0: i64 = fn_state.r;
        // D s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // D s_26_2: read-var d:i
        let s_26_2: i128 = fn_state.d;
        // D s_26_3: add s_26_2 s_26_1
        let s_26_3: i128 = (s_26_2 + s_26_1);
        // D s_26_4: write-var ga#365739 <= s_26_3
        fn_state.ga_365739 = s_26_3;
        // D s_26_5: read-var r:i64
        let s_26_5: i64 = fn_state.r;
        // D s_26_6: cast zx s_26_5 -> i
        let s_26_6: i128 = (i128::try_from(s_26_5).unwrap());
        // D s_26_7: read-var d:i
        let s_26_7: i128 = fn_state.d;
        // D s_26_8: add s_26_7 s_26_6
        let s_26_8: i128 = (s_26_7 + s_26_6);
        // D s_26_9: call D_read(s_26_8)
        let s_26_9: u64 = D_read(state, tracer, s_26_8);
        // D s_26_10: write-var ga#365736 <= s_26_9
        fn_state.ga_365736 = s_26_9;
        // D s_26_11: read-var esizeshadow#7922:i64
        let s_26_11: i64 = fn_state.esizeshadow_7922;
        // D s_26_12: cast zx s_26_11 -> i
        let s_26_12: i128 = (i128::try_from(s_26_11).unwrap());
        // D s_26_13: cast reint s_26_12 -> i64
        let s_26_13: i64 = (s_26_12 as i64);
        // D s_26_14: write-var ga#365737 <= s_26_13
        fn_state.ga_365737 = s_26_13;
        // C s_26_15: const #() : ()
        let s_26_15: () = ();
        // S s_26_16: call StandardFPSCRValue(s_26_15)
        let s_26_16: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_26_15,
        );
        // D s_26_17: read-var op1:bv
        let s_26_17: Bits = fn_state.op1;
        // D s_26_18: read-var op2:bv
        let s_26_18: Bits = fn_state.op2;
        // D s_26_19: call FPMinNum(s_26_17, s_26_18, s_26_16)
        let s_26_19: Bits = FPMinNum(state, tracer, s_26_17, s_26_18, s_26_16);
        // D s_26_20: write-var ga#365738 <= s_26_19
        fn_state.ga_365738 = s_26_19;
        // N s_26_21: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var ga#365736:u64
        let s_27_0: u64 = fn_state.ga_365736;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 64u16);
        // D s_27_2: read-var e:i64
        let s_27_2: i64 = fn_state.e;
        // D s_27_3: cast zx s_27_2 -> i
        let s_27_3: i128 = (i128::try_from(s_27_2).unwrap());
        // D s_27_4: read-var ga#365737:i64
        let s_27_4: i64 = fn_state.ga_365737;
        // D s_27_5: cast zx s_27_4 -> i
        let s_27_5: i128 = (i128::try_from(s_27_4).unwrap());
        // D s_27_6: read-var ga#365738:bv
        let s_27_6: Bits = fn_state.ga_365738;
        // D s_27_7: call Elem_set(s_27_1, s_27_3, s_27_5, s_27_6)
        let s_27_7: Bits = Elem_set(state, tracer, s_27_1, s_27_3, s_27_5, s_27_6);
        // D s_27_8: cast reint s_27_7 -> u64
        let s_27_8: u64 = (s_27_7.value() as u64);
        // D s_27_9: read-var ga#365739:i
        let s_27_9: i128 = fn_state.ga_365739;
        // D s_27_10: call D_set(s_27_9, s_27_8)
        let s_27_10: () = D_set(state, tracer, s_27_9, s_27_8);
        // N s_27_11: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var e:i64
        let s_28_0: i64 = fn_state.e;
        // C s_28_1: const #1s : i64
        let s_28_1: i64 = 1;
        // D s_28_2: add s_28_0 s_28_1
        let s_28_2: i64 = (s_28_0 + s_28_1);
        // D s_28_3: write-var e <= s_28_2
        fn_state.e = s_28_2;
        // N s_28_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var r:i64
        let s_29_0: i64 = fn_state.r;
        // D s_29_1: cast zx s_29_0 -> i
        let s_29_1: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_2: read-var d:i
        let s_29_2: i128 = fn_state.d;
        // D s_29_3: add s_29_2 s_29_1
        let s_29_3: i128 = (s_29_2 + s_29_1);
        // D s_29_4: write-var ga#365732 <= s_29_3
        fn_state.ga_365732 = s_29_3;
        // D s_29_5: read-var r:i64
        let s_29_5: i64 = fn_state.r;
        // D s_29_6: cast zx s_29_5 -> i
        let s_29_6: i128 = (i128::try_from(s_29_5).unwrap());
        // D s_29_7: read-var d:i
        let s_29_7: i128 = fn_state.d;
        // D s_29_8: add s_29_7 s_29_6
        let s_29_8: i128 = (s_29_7 + s_29_6);
        // D s_29_9: call D_read(s_29_8)
        let s_29_9: u64 = D_read(state, tracer, s_29_8);
        // D s_29_10: write-var ga#365729 <= s_29_9
        fn_state.ga_365729 = s_29_9;
        // D s_29_11: read-var esizeshadow#7922:i64
        let s_29_11: i64 = fn_state.esizeshadow_7922;
        // D s_29_12: cast zx s_29_11 -> i
        let s_29_12: i128 = (i128::try_from(s_29_11).unwrap());
        // D s_29_13: cast reint s_29_12 -> i64
        let s_29_13: i64 = (s_29_12 as i64);
        // D s_29_14: write-var ga#365730 <= s_29_13
        fn_state.ga_365730 = s_29_13;
        // C s_29_15: const #() : ()
        let s_29_15: () = ();
        // S s_29_16: call StandardFPSCRValue(s_29_15)
        let s_29_16: ProductType5c790c8ef59cc8b2 = StandardFPSCRValue(
            state,
            tracer,
            s_29_15,
        );
        // D s_29_17: read-var op1:bv
        let s_29_17: Bits = fn_state.op1;
        // D s_29_18: read-var op2:bv
        let s_29_18: Bits = fn_state.op2;
        // D s_29_19: call FPMaxNum(s_29_17, s_29_18, s_29_16)
        let s_29_19: Bits = FPMaxNum(state, tracer, s_29_17, s_29_18, s_29_16);
        // D s_29_20: write-var ga#365731 <= s_29_19
        fn_state.ga_365731 = s_29_19;
        // N s_29_21: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var ga#365729:u64
        let s_30_0: u64 = fn_state.ga_365729;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 64u16);
        // D s_30_2: read-var e:i64
        let s_30_2: i64 = fn_state.e;
        // D s_30_3: cast zx s_30_2 -> i
        let s_30_3: i128 = (i128::try_from(s_30_2).unwrap());
        // D s_30_4: read-var ga#365730:i64
        let s_30_4: i64 = fn_state.ga_365730;
        // D s_30_5: cast zx s_30_4 -> i
        let s_30_5: i128 = (i128::try_from(s_30_4).unwrap());
        // D s_30_6: read-var ga#365731:bv
        let s_30_6: Bits = fn_state.ga_365731;
        // D s_30_7: call Elem_set(s_30_1, s_30_3, s_30_5, s_30_6)
        let s_30_7: Bits = Elem_set(state, tracer, s_30_1, s_30_3, s_30_5, s_30_6);
        // D s_30_8: cast reint s_30_7 -> u64
        let s_30_8: u64 = (s_30_7.value() as u64);
        // D s_30_9: read-var ga#365732:i
        let s_30_9: i128 = fn_state.ga_365732;
        // D s_30_10: call D_set(s_30_9, s_30_8)
        let s_30_10: () = D_set(state, tracer, s_30_9, s_30_8);
        // N s_30_11: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var r:i64
        let s_31_0: i64 = fn_state.r;
        // C s_31_1: const #1s : i64
        let s_31_1: i64 = 1;
        // D s_31_2: add s_31_0 s_31_1
        let s_31_2: i64 = (s_31_0 + s_31_1);
        // D s_31_3: write-var r <= s_31_2
        fn_state.r = s_31_2;
        // N s_31_4: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: return
        return;
    }
}
