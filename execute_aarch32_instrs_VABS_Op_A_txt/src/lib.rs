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
use FPAbs::*;
use Elem_set::*;
use u__id::*;
use integer_subrange::*;
use CheckAdvSIMDOrVFPEnabled::*;
use D_set::*;
use S_set::*;
use Elem_read::*;
use S_read::*;
use D_read::*;
use common::*;
pub fn execute_aarch32_instrs_VABS_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    advsimd: bool,
    d__arg: i64,
    elements: i128,
    esize: i64,
    floating_point: bool,
    m: i64,
    regs: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_350509: i128,
        r: i64,
        e: i64,
        gs_890161: Bits,
        gs_890155: Bits,
        gs_305877: bool,
        ga_350508: Bits,
        gs_305875: bool,
        gs_305860: i64,
        esizeshadow_7337: i64,
        ga_350506: u64,
        d: i128,
        gs_890165: Bits,
        ga_350507: i64,
        gs_305866: i64,
        advsimd: bool,
        d__arg: i64,
        elements: i128,
        esize: i64,
        floating_point: bool,
        m: i64,
        regs: i128,
    }
    let fn_state = FunctionState {
        advsimd,
        d__arg,
        elements,
        esize,
        floating_point,
        m,
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
        // D s_0_3: write-var esizeshadow#7337 <= s_0_2
        fn_state.esizeshadow_7337 = s_0_2;
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
        // D s_2_0: read-var esizeshadow#7337:i64
        let s_2_0: i64 = fn_state.esizeshadow_7337;
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
        // D s_3_0: read-var m:i64
        let s_3_0: i64 = fn_state.m;
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
        // D s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 16u16);
        // D s_3_12: call FPAbs(s_3_11)
        let s_3_12: Bits = FPAbs(state, tracer, s_3_11);
        // D s_3_13: write-var gs#890155 <= s_3_12
        fn_state.gs_890155 = s_3_12;
        // N s_3_14: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#890155:bv
        let s_4_0: Bits = fn_state.gs_890155;
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
        // D s_5_0: read-var esizeshadow#7337:i64
        let s_5_0: i64 = fn_state.esizeshadow_7337;
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
        // D s_6_0: read-var m:i64
        let s_6_0: i64 = fn_state.m;
        // D s_6_1: cast zx s_6_0 -> i
        let s_6_1: i128 = (i128::try_from(s_6_0).unwrap());
        // D s_6_2: call S_read(s_6_1)
        let s_6_2: u32 = S_read(state, tracer, s_6_1);
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 32u16);
        // D s_6_4: call FPAbs(s_6_3)
        let s_6_4: Bits = FPAbs(state, tracer, s_6_3);
        // D s_6_5: write-var gs#890161 <= s_6_4
        fn_state.gs_890161 = s_6_4;
        // N s_6_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#890161:bv
        let s_7_0: Bits = fn_state.gs_890161;
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
        // D s_8_0: read-var esizeshadow#7337:i64
        let s_8_0: i64 = fn_state.esizeshadow_7337;
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
        // D s_9_0: read-var m:i64
        let s_9_0: i64 = fn_state.m;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call D_read(s_9_1)
        let s_9_2: u64 = D_read(state, tracer, s_9_1);
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 64u16);
        // D s_9_4: call FPAbs(s_9_3)
        let s_9_4: Bits = FPAbs(state, tracer, s_9_3);
        // D s_9_5: write-var gs#890165 <= s_9_4
        fn_state.gs_890165 = s_9_4;
        // N s_9_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#890165:bv
        let s_10_0: Bits = fn_state.gs_890165;
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
        // D s_12_5: write-var gs#305860 <= s_12_4
        fn_state.gs_305860 = s_12_4;
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
        // D s_13_1: read-var gs#305860:i64
        let s_13_1: i64 = fn_state.gs_305860;
        // D s_13_2: cmp-gt s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) > (s_13_1));
        // N s_13_3: branch s_13_2 b28 b14
        if s_13_2 {
            return block_28(state, tracer, fn_state);
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
        // D s_14_5: write-var gs#305866 <= s_14_4
        fn_state.gs_305866 = s_14_4;
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
        // D s_15_1: read-var gs#305866:i64
        let s_15_1: i64 = fn_state.gs_305866;
        // D s_15_2: cmp-gt s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) > (s_15_1));
        // N s_15_3: branch s_15_2 b27 b16
        if s_15_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var floating_point:u8
        let s_16_0: bool = fn_state.floating_point;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var m:i64
        let s_17_0: i64 = fn_state.m;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: read-var r:i64
        let s_17_2: i64 = fn_state.r;
        // D s_17_3: cast zx s_17_2 -> i
        let s_17_3: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_4: add s_17_1 s_17_3
        let s_17_4: i128 = (s_17_1 + s_17_3);
        // D s_17_5: call D_read(s_17_4)
        let s_17_5: u64 = D_read(state, tracer, s_17_4);
        // D s_17_6: read-var esizeshadow#7337:i64
        let s_17_6: i64 = fn_state.esizeshadow_7337;
        // D s_17_7: cast zx s_17_6 -> i
        let s_17_7: i128 = (i128::try_from(s_17_6).unwrap());
        // D s_17_8: cast reint s_17_7 -> i64
        let s_17_8: i64 = (s_17_7 as i64);
        // D s_17_9: cast zx s_17_5 -> bv
        let s_17_9: Bits = Bits::new(s_17_5 as u128, 64u16);
        // D s_17_10: read-var e:i64
        let s_17_10: i64 = fn_state.e;
        // D s_17_11: cast zx s_17_10 -> i
        let s_17_11: i128 = (i128::try_from(s_17_10).unwrap());
        // D s_17_12: cast zx s_17_8 -> i
        let s_17_12: i128 = (i128::try_from(s_17_8).unwrap());
        // D s_17_13: call Elem_read(s_17_9, s_17_11, s_17_12)
        let s_17_13: Bits = Elem_read(state, tracer, s_17_9, s_17_11, s_17_12);
        // D s_17_14: cast sx s_17_13 -> i
        let s_17_14: i128 = {
            let sign_bit = s_17_13.length() - 1;
            let mut result = s_17_13.value() as i128;
            if ((result >> sign_bit) & 1) == 1 {
                let cleared_bit = result & !(1 << sign_bit);
                result = cleared_bit - (1 << sign_bit);
            }
            result
        };
        // D s_17_15: cast reint s_17_14 -> i64
        let s_17_15: i64 = (s_17_14 as i64);
        // D s_17_16: cast zx s_17_15 -> i
        let s_17_16: i128 = (i128::try_from(s_17_15).unwrap());
        // D s_17_17: abs s_17_16
        let s_17_17: i128 = (s_17_16).abs();
        // D s_17_18: read-var r:i64
        let s_17_18: i64 = fn_state.r;
        // D s_17_19: cast zx s_17_18 -> i
        let s_17_19: i128 = (i128::try_from(s_17_18).unwrap());
        // D s_17_20: read-var d:i
        let s_17_20: i128 = fn_state.d;
        // D s_17_21: add s_17_20 s_17_19
        let s_17_21: i128 = (s_17_20 + s_17_19);
        // D s_17_22: read-var r:i64
        let s_17_22: i64 = fn_state.r;
        // D s_17_23: cast zx s_17_22 -> i
        let s_17_23: i128 = (i128::try_from(s_17_22).unwrap());
        // D s_17_24: read-var d:i
        let s_17_24: i128 = fn_state.d;
        // D s_17_25: add s_17_24 s_17_23
        let s_17_25: i128 = (s_17_24 + s_17_23);
        // D s_17_26: call D_read(s_17_25)
        let s_17_26: u64 = D_read(state, tracer, s_17_25);
        // D s_17_27: read-var esizeshadow#7337:i64
        let s_17_27: i64 = fn_state.esizeshadow_7337;
        // D s_17_28: cast zx s_17_27 -> i
        let s_17_28: i128 = (i128::try_from(s_17_27).unwrap());
        // D s_17_29: cast reint s_17_28 -> i64
        let s_17_29: i64 = (s_17_28 as i64);
        // C s_17_30: const #1s : i
        let s_17_30: i128 = 1;
        // D s_17_31: read-var esizeshadow#7337:i64
        let s_17_31: i64 = fn_state.esizeshadow_7337;
        // D s_17_32: cast zx s_17_31 -> i
        let s_17_32: i128 = (i128::try_from(s_17_31).unwrap());
        // D s_17_33: sub s_17_32 s_17_30
        let s_17_33: i128 = ((s_17_32) - (s_17_30));
        // D s_17_34: cast reint s_17_33 -> i64
        let s_17_34: i64 = (s_17_33 as i64);
        // C s_17_35: const #0s : i
        let s_17_35: i128 = 0;
        // D s_17_36: cast zx s_17_34 -> i
        let s_17_36: i128 = (i128::try_from(s_17_34).unwrap());
        // D s_17_37: call integer_subrange(s_17_17, s_17_36, s_17_35)
        let s_17_37: Bits = integer_subrange(state, tracer, s_17_17, s_17_36, s_17_35);
        // D s_17_38: cast zx s_17_26 -> bv
        let s_17_38: Bits = Bits::new(s_17_26 as u128, 64u16);
        // D s_17_39: read-var e:i64
        let s_17_39: i64 = fn_state.e;
        // D s_17_40: cast zx s_17_39 -> i
        let s_17_40: i128 = (i128::try_from(s_17_39).unwrap());
        // D s_17_41: cast zx s_17_29 -> i
        let s_17_41: i128 = (i128::try_from(s_17_29).unwrap());
        // D s_17_42: call Elem_set(s_17_38, s_17_40, s_17_41, s_17_37)
        let s_17_42: Bits = Elem_set(state, tracer, s_17_38, s_17_40, s_17_41, s_17_37);
        // D s_17_43: cast reint s_17_42 -> u64
        let s_17_43: u64 = (s_17_42.value() as u64);
        // D s_17_44: call D_set(s_17_21, s_17_43)
        let s_17_44: () = D_set(state, tracer, s_17_21, s_17_43);
        // N s_17_45: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var e:i64
        let s_18_0: i64 = fn_state.e;
        // C s_18_1: const #1s : i64
        let s_18_1: i64 = 1;
        // D s_18_2: add s_18_0 s_18_1
        let s_18_2: i64 = (s_18_0 + s_18_1);
        // D s_18_3: write-var e <= s_18_2
        fn_state.e = s_18_2;
        // N s_18_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var esizeshadow#7337:i64
        let s_19_0: i64 = fn_state.esizeshadow_7337;
        // D s_19_1: cast zx s_19_0 -> i
        let s_19_1: i128 = (i128::try_from(s_19_0).unwrap());
        // D s_19_2: call __id(s_19_1)
        let s_19_2: i128 = u__id(state, tracer, s_19_1);
        // D s_19_3: cast reint s_19_2 -> i64
        let s_19_3: i64 = (s_19_2 as i64);
        // C s_19_4: const #16s : i
        let s_19_4: i128 = 16;
        // D s_19_5: cast zx s_19_3 -> i
        let s_19_5: i128 = (i128::try_from(s_19_3).unwrap());
        // D s_19_6: cmp-eq s_19_5 s_19_4
        let s_19_6: bool = ((s_19_5) == (s_19_4));
        // N s_19_7: branch s_19_6 b26 b20
        if s_19_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var esizeshadow#7337:i64
        let s_20_0: i64 = fn_state.esizeshadow_7337;
        // D s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // D s_20_2: call __id(s_20_1)
        let s_20_2: i128 = u__id(state, tracer, s_20_1);
        // D s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: const #32s : i
        let s_20_4: i128 = 32;
        // D s_20_5: cast zx s_20_3 -> i
        let s_20_5: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_6: cmp-eq s_20_5 s_20_4
        let s_20_6: bool = ((s_20_5) == (s_20_4));
        // D s_20_7: write-var gs#305875 <= s_20_6
        fn_state.gs_305875 = s_20_6;
        // N s_20_8: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#305875:u8
        let s_21_0: bool = fn_state.gs_305875;
        // N s_21_1: branch s_21_0 b25 b22
        if s_21_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var esizeshadow#7337:i64
        let s_22_0: i64 = fn_state.esizeshadow_7337;
        // D s_22_1: cast zx s_22_0 -> i
        let s_22_1: i128 = (i128::try_from(s_22_0).unwrap());
        // D s_22_2: call __id(s_22_1)
        let s_22_2: i128 = u__id(state, tracer, s_22_1);
        // D s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: const #64s : i
        let s_22_4: i128 = 64;
        // D s_22_5: cast zx s_22_3 -> i
        let s_22_5: i128 = (i128::try_from(s_22_3).unwrap());
        // D s_22_6: cmp-eq s_22_5 s_22_4
        let s_22_6: bool = ((s_22_5) == (s_22_4));
        // D s_22_7: write-var gs#305877 <= s_22_6
        fn_state.gs_305877 = s_22_6;
        // N s_22_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#305877:u8
        let s_23_0: bool = fn_state.gs_305877;
        // N s_23_1: assert s_23_0
        let s_23_1: () = assert!(s_23_0);
        // D s_23_2: read-var r:i64
        let s_23_2: i64 = fn_state.r;
        // D s_23_3: cast zx s_23_2 -> i
        let s_23_3: i128 = (i128::try_from(s_23_2).unwrap());
        // D s_23_4: read-var d:i
        let s_23_4: i128 = fn_state.d;
        // D s_23_5: add s_23_4 s_23_3
        let s_23_5: i128 = (s_23_4 + s_23_3);
        // D s_23_6: write-var ga#350509 <= s_23_5
        fn_state.ga_350509 = s_23_5;
        // D s_23_7: read-var r:i64
        let s_23_7: i64 = fn_state.r;
        // D s_23_8: cast zx s_23_7 -> i
        let s_23_8: i128 = (i128::try_from(s_23_7).unwrap());
        // D s_23_9: read-var d:i
        let s_23_9: i128 = fn_state.d;
        // D s_23_10: add s_23_9 s_23_8
        let s_23_10: i128 = (s_23_9 + s_23_8);
        // D s_23_11: call D_read(s_23_10)
        let s_23_11: u64 = D_read(state, tracer, s_23_10);
        // D s_23_12: write-var ga#350506 <= s_23_11
        fn_state.ga_350506 = s_23_11;
        // D s_23_13: read-var esizeshadow#7337:i64
        let s_23_13: i64 = fn_state.esizeshadow_7337;
        // D s_23_14: cast zx s_23_13 -> i
        let s_23_14: i128 = (i128::try_from(s_23_13).unwrap());
        // D s_23_15: cast reint s_23_14 -> i64
        let s_23_15: i64 = (s_23_14 as i64);
        // D s_23_16: write-var ga#350507 <= s_23_15
        fn_state.ga_350507 = s_23_15;
        // D s_23_17: read-var m:i64
        let s_23_17: i64 = fn_state.m;
        // D s_23_18: cast zx s_23_17 -> i
        let s_23_18: i128 = (i128::try_from(s_23_17).unwrap());
        // D s_23_19: read-var r:i64
        let s_23_19: i64 = fn_state.r;
        // D s_23_20: cast zx s_23_19 -> i
        let s_23_20: i128 = (i128::try_from(s_23_19).unwrap());
        // D s_23_21: add s_23_18 s_23_20
        let s_23_21: i128 = (s_23_18 + s_23_20);
        // D s_23_22: call D_read(s_23_21)
        let s_23_22: u64 = D_read(state, tracer, s_23_21);
        // D s_23_23: read-var esizeshadow#7337:i64
        let s_23_23: i64 = fn_state.esizeshadow_7337;
        // D s_23_24: cast zx s_23_23 -> i
        let s_23_24: i128 = (i128::try_from(s_23_23).unwrap());
        // D s_23_25: cast reint s_23_24 -> i64
        let s_23_25: i64 = (s_23_24 as i64);
        // D s_23_26: cast zx s_23_22 -> bv
        let s_23_26: Bits = Bits::new(s_23_22 as u128, 64u16);
        // D s_23_27: read-var e:i64
        let s_23_27: i64 = fn_state.e;
        // D s_23_28: cast zx s_23_27 -> i
        let s_23_28: i128 = (i128::try_from(s_23_27).unwrap());
        // D s_23_29: cast zx s_23_25 -> i
        let s_23_29: i128 = (i128::try_from(s_23_25).unwrap());
        // D s_23_30: call Elem_read(s_23_26, s_23_28, s_23_29)
        let s_23_30: Bits = Elem_read(state, tracer, s_23_26, s_23_28, s_23_29);
        // D s_23_31: call FPAbs(s_23_30)
        let s_23_31: Bits = FPAbs(state, tracer, s_23_30);
        // D s_23_32: write-var ga#350508 <= s_23_31
        fn_state.ga_350508 = s_23_31;
        // N s_23_33: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var ga#350506:u64
        let s_24_0: u64 = fn_state.ga_350506;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 64u16);
        // D s_24_2: read-var e:i64
        let s_24_2: i64 = fn_state.e;
        // D s_24_3: cast zx s_24_2 -> i
        let s_24_3: i128 = (i128::try_from(s_24_2).unwrap());
        // D s_24_4: read-var ga#350507:i64
        let s_24_4: i64 = fn_state.ga_350507;
        // D s_24_5: cast zx s_24_4 -> i
        let s_24_5: i128 = (i128::try_from(s_24_4).unwrap());
        // D s_24_6: read-var ga#350508:bv
        let s_24_6: Bits = fn_state.ga_350508;
        // D s_24_7: call Elem_set(s_24_1, s_24_3, s_24_5, s_24_6)
        let s_24_7: Bits = Elem_set(state, tracer, s_24_1, s_24_3, s_24_5, s_24_6);
        // D s_24_8: cast reint s_24_7 -> u64
        let s_24_8: u64 = (s_24_7.value() as u64);
        // D s_24_9: read-var ga#350509:i
        let s_24_9: i128 = fn_state.ga_350509;
        // D s_24_10: call D_set(s_24_9, s_24_8)
        let s_24_10: () = D_set(state, tracer, s_24_9, s_24_8);
        // N s_24_11: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var gs#305877 <= s_25_0
        fn_state.gs_305877 = s_25_0;
        // N s_25_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#305875 <= s_26_0
        fn_state.gs_305875 = s_26_0;
        // N s_26_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var r:i64
        let s_27_0: i64 = fn_state.r;
        // C s_27_1: const #1s : i64
        let s_27_1: i64 = 1;
        // D s_27_2: add s_27_0 s_27_1
        let s_27_2: i64 = (s_27_0 + s_27_1);
        // D s_27_3: write-var r <= s_27_2
        fn_state.r = s_27_2;
        // N s_27_4: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: return
        return;
    }
}
