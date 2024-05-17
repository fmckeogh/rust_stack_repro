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
use Elem_set::*;
use V_read::*;
use FPCR_read::*;
use V_set::*;
use IsMerging::*;
use Vpart_set::*;
use Elem_read::*;
use FPConvert::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_arithmetic_unary_float_xtn_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    elements: i64,
    esize: i64,
    n: i64,
    part: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        datasizeshadow_1351: i64,
        fpcr: ProductType5c790c8ef59cc8b2,
        e: i64,
        gs_151839: i64,
        ga_254737: i64,
        gs_151833: bool,
        gs_693047: Bits,
        merge: bool,
        result: u128,
        d: i64,
        datasize: i64,
        elements: i64,
        esize: i64,
        n: i64,
        part: i64,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        n,
        part,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#1351 <= s_0_2
        fn_state.datasizeshadow_1351 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPAdvSIMDEnabled64(s_0_4)
        let s_0_5: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var datasizeshadow#1351:i64
        let s_1_1: i64 = fn_state.datasizeshadow_1351;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: mul s_1_0 s_1_2
        let s_1_3: i128 = ((s_1_0) * (s_1_2));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var n:i64
        let s_1_7: i64 = fn_state.n;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call V_read(s_1_8, s_1_6)
        let s_1_9: Bits = V_read(state, tracer, s_1_8, s_1_6);
        // D s_1_10: write-var operand <= s_1_9
        fn_state.operand = s_1_9;
        // C s_1_11: const #() : ()
        let s_1_11: () = ();
        // S s_1_12: call FPCR_read(s_1_11)
        let s_1_12: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_1_11);
        // D s_1_13: write-var fpcr <= s_1_12
        fn_state.fpcr = s_1_12;
        // C s_1_14: const #1s : i
        let s_1_14: i128 = 1;
        // D s_1_15: read-var elements:i64
        let s_1_15: i64 = fn_state.elements;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cmp-eq s_1_16 s_1_14
        let s_1_17: bool = ((s_1_16) == (s_1_14));
        // N s_1_18: branch s_1_17 b13 b2
        if s_1_17 {
            return block_13(state, tracer, fn_state);
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
        // D s_2_1: write-var gs#151833 <= s_2_0
        fn_state.gs_151833 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#151833:u8
        let s_3_0: bool = fn_state.gs_151833;
        // D s_3_1: write-var merge <= s_3_0
        fn_state.merge = s_3_0;
        // D s_3_2: read-var merge:u8
        let s_3_2: bool = fn_state.merge;
        // N s_3_3: branch s_3_2 b12 b4
        if s_3_2 {
            return block_12(state, tracer, fn_state);
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
        // D s_5_2: read-var elements:i64
        let s_5_2: i64 = fn_state.elements;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: sub s_5_3 s_5_1
        let s_5_4: i128 = ((s_5_3) - (s_5_1));
        // D s_5_5: cast reint s_5_4 -> i64
        let s_5_5: i64 = (s_5_4 as i64);
        // D s_5_6: write-var gs#151839 <= s_5_5
        fn_state.gs_151839 = s_5_5;
        // D s_5_7: write-var e <= s_5_0
        fn_state.e = s_5_0;
        // N s_5_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var e:i64
        let s_6_0: i64 = fn_state.e;
        // D s_6_1: read-var gs#151839:i64
        let s_6_1: i64 = fn_state.gs_151839;
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
        // D s_7_0: read-var esize:i64
        let s_7_0: i64 = fn_state.esize;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: cast reint s_7_1 -> i64
        let s_7_2: i64 = (s_7_1 as i64);
        // D s_7_3: write-var ga#254737 <= s_7_2
        fn_state.ga_254737 = s_7_2;
        // C s_7_4: const #2s : i
        let s_7_4: i128 = 2;
        // D s_7_5: read-var esize:i64
        let s_7_5: i64 = fn_state.esize;
        // D s_7_6: cast zx s_7_5 -> i
        let s_7_6: i128 = (i128::try_from(s_7_5).unwrap());
        // D s_7_7: mul s_7_4 s_7_6
        let s_7_7: i128 = ((s_7_4) * (s_7_6));
        // D s_7_8: cast reint s_7_7 -> i64
        let s_7_8: i64 = (s_7_7 as i64);
        // D s_7_9: cast zx s_7_8 -> i
        let s_7_9: i128 = (i128::try_from(s_7_8).unwrap());
        // D s_7_10: cast reint s_7_9 -> i64
        let s_7_10: i64 = (s_7_9 as i64);
        // D s_7_11: read-var e:i64
        let s_7_11: i64 = fn_state.e;
        // D s_7_12: cast zx s_7_11 -> i
        let s_7_12: i128 = (i128::try_from(s_7_11).unwrap());
        // D s_7_13: cast zx s_7_10 -> i
        let s_7_13: i128 = (i128::try_from(s_7_10).unwrap());
        // D s_7_14: read-var operand:bv
        let s_7_14: Bits = fn_state.operand;
        // D s_7_15: call Elem_read(s_7_14, s_7_12, s_7_13)
        let s_7_15: Bits = Elem_read(state, tracer, s_7_14, s_7_12, s_7_13);
        // D s_7_16: cast reint s_7_15 -> u64
        let s_7_16: u64 = (s_7_15.value() as u64);
        // D s_7_17: read-var esize:i64
        let s_7_17: i64 = fn_state.esize;
        // D s_7_18: cast zx s_7_17 -> i
        let s_7_18: i128 = (i128::try_from(s_7_17).unwrap());
        // D s_7_19: cast reint s_7_18 -> i64
        let s_7_19: i64 = (s_7_18 as i64);
        // D s_7_20: cast zx s_7_16 -> bv
        let s_7_20: Bits = Bits::new(s_7_16 as u128, 64u16);
        // D s_7_21: read-var fpcr:struct
        let s_7_21: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // C s_7_22: const #5u : u32
        let s_7_22: u32 = 5;
        // D s_7_23: call FPConvert(s_7_20, s_7_21, s_7_22, s_7_19)
        let s_7_23: Bits = FPConvert(state, tracer, s_7_20, s_7_21, s_7_22, s_7_19);
        // D s_7_24: write-var gs#693047 <= s_7_23
        fn_state.gs_693047 = s_7_23;
        // N s_7_25: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#693047:bv
        let s_8_0: Bits = fn_state.gs_693047;
        // D s_8_1: cast reint s_8_0 -> u32
        let s_8_1: u32 = (s_8_0.value() as u32);
        // D s_8_2: read-var result:u128
        let s_8_2: u128 = fn_state.result;
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 128u16);
        // D s_8_4: read-var e:i64
        let s_8_4: i64 = fn_state.e;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: read-var ga#254737:i64
        let s_8_6: i64 = fn_state.ga_254737;
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: cast zx s_8_1 -> bv
        let s_8_8: Bits = Bits::new(s_8_1 as u128, 32u16);
        // D s_8_9: call Elem_set(s_8_3, s_8_5, s_8_7, s_8_8)
        let s_8_9: Bits = Elem_set(state, tracer, s_8_3, s_8_5, s_8_7, s_8_8);
        // D s_8_10: cast reint s_8_9 -> u128
        let s_8_10: u128 = (s_8_9.value() as u128);
        // D s_8_11: write-var result <= s_8_10
        fn_state.result = s_8_10;
        // D s_8_12: read-var e:i64
        let s_8_12: i64 = fn_state.e;
        // C s_8_13: const #1s : i64
        let s_8_13: i64 = 1;
        // D s_8_14: add s_8_12 s_8_13
        let s_8_14: i64 = (s_8_12 + s_8_13);
        // D s_8_15: write-var e <= s_8_14
        fn_state.e = s_8_14;
        // N s_8_16: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var merge:u8
        let s_9_0: bool = fn_state.merge;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var datasizeshadow#1351:i64
        let s_10_0: i64 = fn_state.datasizeshadow_1351;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var datasizeshadow#1351:i64
        let s_10_3: i64 = fn_state.datasizeshadow_1351;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast reint s_10_4 -> i64
        let s_10_5: i64 = (s_10_4 as i64);
        // C s_10_6: const #0s : i
        let s_10_6: i128 = 0;
        // D s_10_7: read-var result:u128
        let s_10_7: u128 = fn_state.result;
        // D s_10_8: cast zx s_10_7 -> bv
        let s_10_8: Bits = Bits::new(s_10_7 as u128, 128u16);
        // D s_10_9: cast zx s_10_5 -> i
        let s_10_9: i128 = (i128::try_from(s_10_5).unwrap());
        // D s_10_10: call Elem_read(s_10_8, s_10_6, s_10_9)
        let s_10_10: Bits = Elem_read(state, tracer, s_10_8, s_10_6, s_10_9);
        // D s_10_11: read-var d:i64
        let s_10_11: i64 = fn_state.d;
        // D s_10_12: cast zx s_10_11 -> i
        let s_10_12: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_13: read-var part:i64
        let s_10_13: i64 = fn_state.part;
        // D s_10_14: cast zx s_10_13 -> i
        let s_10_14: i128 = (i128::try_from(s_10_13).unwrap());
        // D s_10_15: cast zx s_10_2 -> i
        let s_10_15: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_16: call Vpart_set(s_10_12, s_10_14, s_10_15, s_10_10)
        let s_10_16: () = Vpart_set(state, tracer, s_10_12, s_10_14, s_10_15, s_10_10);
        // N s_10_17: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #128s : i64
        let s_11_0: i64 = 128;
        // D s_11_1: read-var d:i64
        let s_11_1: i64 = fn_state.d;
        // D s_11_2: cast zx s_11_1 -> i
        let s_11_2: i128 = (i128::try_from(s_11_1).unwrap());
        // D s_11_3: read-var result:u128
        let s_11_3: u128 = fn_state.result;
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 128u16);
        // D s_11_5: call V_set(s_11_2, s_11_0, s_11_4)
        let s_11_5: () = V_set(state, tracer, s_11_2, s_11_0, s_11_4);
        // N s_11_6: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #128s : i64
        let s_12_0: i64 = 128;
        // D s_12_1: read-var d:i64
        let s_12_1: i64 = fn_state.d;
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: call V_read(s_12_2, s_12_0)
        let s_12_3: Bits = V_read(state, tracer, s_12_2, s_12_0);
        // D s_12_4: cast reint s_12_3 -> u128
        let s_12_4: u128 = (s_12_3.value() as u128);
        // D s_12_5: write-var result <= s_12_4
        fn_state.result = s_12_4;
        // N s_12_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var fpcr:struct
        let s_13_0: ProductType5c790c8ef59cc8b2 = fn_state.fpcr;
        // D s_13_1: call IsMerging(s_13_0)
        let s_13_1: bool = IsMerging(state, tracer, s_13_0);
        // D s_13_2: write-var gs#151833 <= s_13_1
        fn_state.gs_151833 = s_13_1;
        // N s_13_3: jump b3
        return block_3(state, tracer, fn_state);
    }
}
