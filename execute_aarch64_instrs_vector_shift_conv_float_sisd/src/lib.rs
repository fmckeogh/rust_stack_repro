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
use V_set::*;
use V_read::*;
use FPCR_read::*;
use Elem_set::*;
use IsMerging::*;
use FPToFixed::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_shift_conv_float_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i128,
    esize: i64,
    fracbits: i128,
    n: i64,
    rounding: u32,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        esizeshadow_1361: i64,
        ga_254873: Bits,
        fpcr: ProductType5c790c8ef59cc8b2,
        e: i64,
        datasizeshadow_1362: i64,
        gs_152073: bool,
        result: u128,
        gs_152079: i64,
        ga_254872: i64,
        d: i64,
        datasize: i64,
        elements: i128,
        esize: i64,
        fracbits: i128,
        n: i64,
        rounding: u32,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        fracbits,
        n,
        rounding,
        is_unsigned,
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
        // D s_0_3: write-var esizeshadow#1361 <= s_0_2
        fn_state.esizeshadow_1361 = s_0_2;
        // D s_0_4: read-var datasize:i64
        let s_0_4: i64 = fn_state.datasize;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var datasizeshadow#1362 <= s_0_6
        fn_state.datasizeshadow_1362 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPAdvSIMDEnabled64(s_0_8)
        let s_0_9: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1362:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1362;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var n:i64
        let s_1_3: i64 = fn_state.n;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call V_read(s_1_4, s_1_2)
        let s_1_5: Bits = V_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var operand <= s_1_5
        fn_state.operand = s_1_5;
        // C s_1_7: const #() : ()
        let s_1_7: () = ();
        // S s_1_8: call FPCR_read(s_1_7)
        let s_1_8: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_1_7);
        // D s_1_9: write-var fpcr <= s_1_8
        fn_state.fpcr = s_1_8;
        // C s_1_10: const #1s : i
        let s_1_10: i128 = 1;
        // D s_1_11: read-var elements:i
        let s_1_11: i128 = fn_state.elements;
        // D s_1_12: cmp-eq s_1_11 s_1_10
        let s_1_12: bool = ((s_1_11) == (s_1_10));
        // N s_1_13: branch s_1_12 b11 b2
        if s_1_12 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#152073 <= s_2_0
        fn_state.gs_152073 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#152073:u8
        let s_3_0: bool = fn_state.gs_152073;
        // N s_3_1: branch s_3_0 b10 b4
        if s_3_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: u8 = 0;
        // C s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 0u16);
        // C s_4_2: const #0u : u64
        let s_4_2: u64 = 0;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 64u16);
        // C s_4_4: cast reint s_4_1 -> u128
        let s_4_4: u128 = (s_4_1.value() as u128);
        // D s_4_5: size-of s_4_1
        let s_4_5: u16 = s_4_1.length();
        // C s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: lsl s_4_4 s_4_7
        let s_4_8: u128 = s_4_4 << s_4_7;
        // D s_4_9: or s_4_8 s_4_6
        let s_4_9: u128 = ((s_4_8) | (s_4_6));
        // D s_4_10: add s_4_5 s_4_7
        let s_4_10: u16 = (s_4_5 + s_4_7);
        // D s_4_11: create-bits s_4_9 s_4_10
        let s_4_11: Bits = Bits::new(s_4_9, s_4_10);
        // C s_4_12: const #0u : u64
        let s_4_12: u64 = 0;
        // C s_4_13: cast zx s_4_12 -> bv
        let s_4_13: Bits = Bits::new(s_4_12 as u128, 64u16);
        // D s_4_14: cast reint s_4_11 -> u128
        let s_4_14: u128 = (s_4_11.value() as u128);
        // D s_4_15: size-of s_4_11
        let s_4_15: u16 = s_4_11.length();
        // C s_4_16: cast reint s_4_13 -> u128
        let s_4_16: u128 = (s_4_13.value() as u128);
        // D s_4_17: size-of s_4_13
        let s_4_17: u16 = s_4_13.length();
        // D s_4_18: lsl s_4_14 s_4_17
        let s_4_18: u128 = s_4_14 << s_4_17;
        // D s_4_19: or s_4_18 s_4_16
        let s_4_19: u128 = ((s_4_18) | (s_4_16));
        // D s_4_20: add s_4_15 s_4_17
        let s_4_20: u16 = (s_4_15 + s_4_17);
        // D s_4_21: create-bits s_4_19 s_4_20
        let s_4_21: Bits = Bits::new(s_4_19, s_4_20);
        // D s_4_22: cast reint s_4_21 -> u128
        let s_4_22: u128 = (s_4_21.value() as u128);
        // D s_4_23: write-var result <= s_4_22
        fn_state.result = s_4_22;
        // N s_4_24: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // C s_5_1: const #1s : i
        let s_5_1: i128 = 1;
        // D s_5_2: read-var elements:i
        let s_5_2: i128 = fn_state.elements;
        // D s_5_3: sub s_5_2 s_5_1
        let s_5_3: i128 = ((s_5_2) - (s_5_1));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: write-var gs#152079 <= s_5_4
        fn_state.gs_152079 = s_5_4;
        // D s_5_6: write-var e <= s_5_0
        fn_state.e = s_5_0;
        // N s_5_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#152079:i64
        let s_6_1: i64 = fn_state.gs_152079;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b9 b7
        if s_6_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esizeshadow#1361:i64
        let s_7_0: i64 = fn_state.esizeshadow_1361;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: read-var e:i64
        let s_7_3: i64 = fn_state.e;
        // D s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_5: cast zx s_7_2 -> i
        let s_7_5: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_6: read-var operand:bv
        let s_7_6: Bits = fn_state.operand;
        // D s_7_7: call Elem_read(s_7_6, s_7_4, s_7_5)
        let s_7_7: Bits = Elem_read(state, tracer, s_7_6, s_7_4, s_7_5);
        // D s_7_8: read-var esizeshadow#1361:i64
        let s_7_8: i64 = fn_state.esizeshadow_1361;
        // D s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: write-var ga#254872 <= s_7_10
        fn_state.ga_254872 = s_7_10;
        // D s_7_12: read-var esizeshadow#1361:i64
        let s_7_12: i64 = fn_state.esizeshadow_1361;
        // D s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // D s_7_14: cast reint s_7_13 -> i64
        let s_7_14: i64 = (s_7_13 as i64);
        // D s_7_15: read-var fracbits:i
        let s_7_15: i128 = fn_state.fracbits;
        // D s_7_16: read-var is_unsigned:u8
        let s_7_16: bool = fn_state.is_unsigned;
        // D s_7_17: read-var fpcr:struct
        let s_7_17: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_7_18: read-var rounding:u32
        let s_7_18: u32 = fn_state.rounding;
        // D s_7_19: call FPToFixed(s_7_7, s_7_15, s_7_16, s_7_17, s_7_18, s_7_14)
        let s_7_19: Bits = FPToFixed(
            state,
            tracer,
            s_7_7,
            s_7_15,
            s_7_16,
            s_7_17,
            s_7_18,
            s_7_14,
        );
        // D s_7_20: write-var ga#254873 <= s_7_19
        fn_state.ga_254873 = s_7_19;
        // N s_7_21: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var result:u128
        let s_8_0: u128 = fn_state.result;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 128u16);
        // D s_8_2: read-var e:i64
        let s_8_2: i64 = fn_state.e;
        // D s_8_3: cast zx s_8_2 -> i
        let s_8_3: i128 = (i128::try_from(s_8_2).unwrap());
        // D s_8_4: read-var ga#254872:i64
        let s_8_4: i64 = fn_state.ga_254872;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-var ga#254873:bv
        let s_8_6: Bits = fn_state.ga_254873;
        // D s_8_7: call Elem_set(s_8_1, s_8_3, s_8_5, s_8_6)
        let s_8_7: Bits = Elem_set(state, tracer, s_8_1, s_8_3, s_8_5, s_8_6);
        // D s_8_8: cast reint s_8_7 -> u128
        let s_8_8: u128 = (s_8_7.value() as u128);
        // D s_8_9: write-var result <= s_8_8
        fn_state.result = s_8_8;
        // D s_8_10: read-var e:i64
        let s_8_10: i64 = fn_state.e;
        // C s_8_11: const #1s : i64
        let s_8_11: i64 = 1;
        // D s_8_12: add s_8_10 s_8_11
        let s_8_12: i64 = (s_8_10 + s_8_11);
        // D s_8_13: write-var e <= s_8_12
        fn_state.e = s_8_12;
        // N s_8_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #128s : i64
        let s_9_0: i64 = 128;
        // D s_9_1: read-var d:i64
        let s_9_1: i64 = fn_state.d;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: read-var result:u128
        let s_9_3: u128 = fn_state.result;
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 128u16);
        // D s_9_5: call V_set(s_9_2, s_9_0, s_9_4)
        let s_9_5: () = V_set(state, tracer, s_9_2, s_9_0, s_9_4);
        // N s_9_6: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #128s : i64
        let s_10_0: i64 = 128;
        // D s_10_1: read-var d:i64
        let s_10_1: i64 = fn_state.d;
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: call V_read(s_10_2, s_10_0)
        let s_10_3: Bits = V_read(state, tracer, s_10_2, s_10_0);
        // D s_10_4: cast reint s_10_3 -> u128
        let s_10_4: u128 = (s_10_3.value() as u128);
        // D s_10_5: write-var result <= s_10_4
        fn_state.result = s_10_4;
        // N s_10_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var fpcr:struct
        let s_11_0: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_11_1: call IsMerging(s_11_0)
        let s_11_1: bool = IsMerging(state, tracer, s_11_0);
        // D s_11_2: write-var gs#152073 <= s_11_1
        fn_state.gs_152073 = s_11_1;
        // N s_11_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
