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
use R_read::*;
use CheckAdvSIMDEnabled::*;
use MemU_set::*;
use R_set::*;
use AArch32_Abort::*;
use IsAligned__1::*;
use CreateAccDescASIMD::*;
use Elem_read::*;
use D_read::*;
use AlignmentFault::*;
use common::*;
pub fn execute_aarch32_instrs_VST4_m_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    alignment: i64,
    d: i64,
    d2: i128,
    d3: i128,
    d4: i128,
    ebytes: i64,
    elements: i128,
    m: i64,
    n: i64,
    register_index: bool,
    wback: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        e: i64,
        gs_321962: i64,
        address: u32,
        accdesc: ProductType9878976b5bcce9c9,
        alignment: i64,
        d: i64,
        d2: i128,
        d3: i128,
        d4: i128,
        ebytes: i64,
        elements: i128,
        m: i64,
        n: i64,
        register_index: bool,
        wback: bool,
    }
    let fn_state = FunctionState {
        alignment,
        d,
        d2,
        d3,
        d4,
        ebytes,
        elements,
        m,
        n,
        register_index,
        wback,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var n:i64
        let s_1_0: i64 = fn_state.n;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: call R_read(s_1_1)
        let s_1_2: u32 = R_read(state, tracer, s_1_1);
        // D s_1_3: write-var address <= s_1_2
        fn_state.address = s_1_2;
        // C s_1_4: const #1u : u32
        let s_1_4: u32 = 1;
        // C s_1_5: const #0u : u8
        let s_1_5: bool = false;
        // C s_1_6: const #0u : u8
        let s_1_6: bool = false;
        // S s_1_7: call CreateAccDescASIMD(s_1_4, s_1_5, s_1_6)
        let s_1_7: ProductType9878976b5bcce9c9 = CreateAccDescASIMD(
            state,
            tracer,
            s_1_4,
            s_1_5,
            s_1_6,
        );
        // D s_1_8: write-var accdesc <= s_1_7
        fn_state.accdesc = s_1_7;
        // D s_1_9: read-var address:u32
        let s_1_9: u32 = fn_state.address;
        // D s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 32u16);
        // D s_1_11: read-var alignment:i64
        let s_1_11: i64 = fn_state.alignment;
        // D s_1_12: cast zx s_1_11 -> i
        let s_1_12: i128 = (i128::try_from(s_1_11).unwrap());
        // D s_1_13: call IsAligned__1(s_1_10, s_1_12)
        let s_1_13: bool = IsAligned__1(state, tracer, s_1_10, s_1_12);
        // D s_1_14: not s_1_13
        let s_1_14: bool = !s_1_13;
        // N s_1_15: branch s_1_14 b15 b2
        if s_1_14 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_2_0: jump b3
        return block_3(state, tracer, fn_state);
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
        // D s_3_2: read-var elements:i
        let s_3_2: i128 = fn_state.elements;
        // D s_3_3: sub s_3_2 s_3_1
        let s_3_3: i128 = ((s_3_2) - (s_3_1));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: write-var gs#321962 <= s_3_4
        fn_state.gs_321962 = s_3_4;
        // D s_3_6: write-var e <= s_3_0
        fn_state.e = s_3_0;
        // N s_3_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var e:i64
        let s_4_0: i64 = fn_state.e;
        // D s_4_1: read-var gs#321962:i64
        let s_4_1: i64 = fn_state.gs_321962;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b10 b5
        if s_4_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var d:i64
        let s_5_0: i64 = fn_state.d;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call D_read(s_5_1)
        let s_5_2: u64 = D_read(state, tracer, s_5_1);
        // C s_5_3: const #8s : i
        let s_5_3: i128 = 8;
        // D s_5_4: read-var ebytes:i64
        let s_5_4: i64 = fn_state.ebytes;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: mul s_5_3 s_5_5
        let s_5_6: i128 = ((s_5_3) * (s_5_5));
        // D s_5_7: cast reint s_5_6 -> i64
        let s_5_7: i64 = (s_5_6 as i64);
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: cast reint s_5_8 -> i64
        let s_5_9: i64 = (s_5_8 as i64);
        // D s_5_10: cast zx s_5_2 -> bv
        let s_5_10: Bits = Bits::new(s_5_2 as u128, 64u16);
        // D s_5_11: read-var e:i64
        let s_5_11: i64 = fn_state.e;
        // D s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_13: cast zx s_5_9 -> i
        let s_5_13: i128 = (i128::try_from(s_5_9).unwrap());
        // D s_5_14: call Elem_read(s_5_10, s_5_12, s_5_13)
        let s_5_14: Bits = Elem_read(state, tracer, s_5_10, s_5_12, s_5_13);
        // D s_5_15: read-var ebytes:i64
        let s_5_15: i64 = fn_state.ebytes;
        // D s_5_16: cast zx s_5_15 -> i
        let s_5_16: i128 = (i128::try_from(s_5_15).unwrap());
        // D s_5_17: read-var address:u32
        let s_5_17: u32 = fn_state.address;
        // D s_5_18: call MemU_set(s_5_17, s_5_16, s_5_14)
        let s_5_18: () = MemU_set(state, tracer, s_5_17, s_5_16, s_5_14);
        // N s_5_19: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var address:u32
        let s_6_0: u32 = fn_state.address;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 32u16);
        // D s_6_2: read-var ebytes:i64
        let s_6_2: i64 = fn_state.ebytes;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: cast cvt s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 128);
        // D s_6_5: add s_6_1 s_6_4
        let s_6_5: Bits = (s_6_1 + s_6_4);
        // D s_6_6: cast reint s_6_5 -> u32
        let s_6_6: u32 = (s_6_5.value() as u32);
        // D s_6_7: read-var d2:i
        let s_6_7: i128 = fn_state.d2;
        // D s_6_8: call D_read(s_6_7)
        let s_6_8: u64 = D_read(state, tracer, s_6_7);
        // C s_6_9: const #8s : i
        let s_6_9: i128 = 8;
        // D s_6_10: read-var ebytes:i64
        let s_6_10: i64 = fn_state.ebytes;
        // D s_6_11: cast zx s_6_10 -> i
        let s_6_11: i128 = (i128::try_from(s_6_10).unwrap());
        // D s_6_12: mul s_6_9 s_6_11
        let s_6_12: i128 = ((s_6_9) * (s_6_11));
        // D s_6_13: cast reint s_6_12 -> i64
        let s_6_13: i64 = (s_6_12 as i64);
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_15: cast reint s_6_14 -> i64
        let s_6_15: i64 = (s_6_14 as i64);
        // D s_6_16: cast zx s_6_8 -> bv
        let s_6_16: Bits = Bits::new(s_6_8 as u128, 64u16);
        // D s_6_17: read-var e:i64
        let s_6_17: i64 = fn_state.e;
        // D s_6_18: cast zx s_6_17 -> i
        let s_6_18: i128 = (i128::try_from(s_6_17).unwrap());
        // D s_6_19: cast zx s_6_15 -> i
        let s_6_19: i128 = (i128::try_from(s_6_15).unwrap());
        // D s_6_20: call Elem_read(s_6_16, s_6_18, s_6_19)
        let s_6_20: Bits = Elem_read(state, tracer, s_6_16, s_6_18, s_6_19);
        // D s_6_21: read-var ebytes:i64
        let s_6_21: i64 = fn_state.ebytes;
        // D s_6_22: cast zx s_6_21 -> i
        let s_6_22: i128 = (i128::try_from(s_6_21).unwrap());
        // D s_6_23: call MemU_set(s_6_6, s_6_22, s_6_20)
        let s_6_23: () = MemU_set(state, tracer, s_6_6, s_6_22, s_6_20);
        // N s_6_24: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #2s : i
        let s_7_0: i128 = 2;
        // D s_7_1: read-var ebytes:i64
        let s_7_1: i64 = fn_state.ebytes;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: mul s_7_0 s_7_2
        let s_7_3: i128 = ((s_7_0) * (s_7_2));
        // D s_7_4: cast reint s_7_3 -> i64
        let s_7_4: i64 = (s_7_3 as i64);
        // D s_7_5: read-var address:u32
        let s_7_5: u32 = fn_state.address;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 32u16);
        // D s_7_7: cast zx s_7_4 -> i
        let s_7_7: i128 = (i128::try_from(s_7_4).unwrap());
        // D s_7_8: cast cvt s_7_7 -> bv
        let s_7_8: Bits = Bits::new(s_7_7 as u128, 128);
        // D s_7_9: add s_7_6 s_7_8
        let s_7_9: Bits = (s_7_6 + s_7_8);
        // D s_7_10: cast reint s_7_9 -> u32
        let s_7_10: u32 = (s_7_9.value() as u32);
        // D s_7_11: read-var d3:i
        let s_7_11: i128 = fn_state.d3;
        // D s_7_12: call D_read(s_7_11)
        let s_7_12: u64 = D_read(state, tracer, s_7_11);
        // C s_7_13: const #8s : i
        let s_7_13: i128 = 8;
        // D s_7_14: read-var ebytes:i64
        let s_7_14: i64 = fn_state.ebytes;
        // D s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // D s_7_16: mul s_7_13 s_7_15
        let s_7_16: i128 = ((s_7_13) * (s_7_15));
        // D s_7_17: cast reint s_7_16 -> i64
        let s_7_17: i64 = (s_7_16 as i64);
        // D s_7_18: cast zx s_7_17 -> i
        let s_7_18: i128 = (i128::try_from(s_7_17).unwrap());
        // D s_7_19: cast reint s_7_18 -> i64
        let s_7_19: i64 = (s_7_18 as i64);
        // D s_7_20: cast zx s_7_12 -> bv
        let s_7_20: Bits = Bits::new(s_7_12 as u128, 64u16);
        // D s_7_21: read-var e:i64
        let s_7_21: i64 = fn_state.e;
        // D s_7_22: cast zx s_7_21 -> i
        let s_7_22: i128 = (i128::try_from(s_7_21).unwrap());
        // D s_7_23: cast zx s_7_19 -> i
        let s_7_23: i128 = (i128::try_from(s_7_19).unwrap());
        // D s_7_24: call Elem_read(s_7_20, s_7_22, s_7_23)
        let s_7_24: Bits = Elem_read(state, tracer, s_7_20, s_7_22, s_7_23);
        // D s_7_25: read-var ebytes:i64
        let s_7_25: i64 = fn_state.ebytes;
        // D s_7_26: cast zx s_7_25 -> i
        let s_7_26: i128 = (i128::try_from(s_7_25).unwrap());
        // D s_7_27: call MemU_set(s_7_10, s_7_26, s_7_24)
        let s_7_27: () = MemU_set(state, tracer, s_7_10, s_7_26, s_7_24);
        // N s_7_28: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #3s : i
        let s_8_0: i128 = 3;
        // D s_8_1: read-var ebytes:i64
        let s_8_1: i64 = fn_state.ebytes;
        // D s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // D s_8_3: mul s_8_0 s_8_2
        let s_8_3: i128 = ((s_8_0) * (s_8_2));
        // D s_8_4: cast reint s_8_3 -> i64
        let s_8_4: i64 = (s_8_3 as i64);
        // D s_8_5: read-var address:u32
        let s_8_5: u32 = fn_state.address;
        // D s_8_6: cast zx s_8_5 -> bv
        let s_8_6: Bits = Bits::new(s_8_5 as u128, 32u16);
        // D s_8_7: cast zx s_8_4 -> i
        let s_8_7: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_8: cast cvt s_8_7 -> bv
        let s_8_8: Bits = Bits::new(s_8_7 as u128, 128);
        // D s_8_9: add s_8_6 s_8_8
        let s_8_9: Bits = (s_8_6 + s_8_8);
        // D s_8_10: cast reint s_8_9 -> u32
        let s_8_10: u32 = (s_8_9.value() as u32);
        // D s_8_11: read-var d4:i
        let s_8_11: i128 = fn_state.d4;
        // D s_8_12: call D_read(s_8_11)
        let s_8_12: u64 = D_read(state, tracer, s_8_11);
        // C s_8_13: const #8s : i
        let s_8_13: i128 = 8;
        // D s_8_14: read-var ebytes:i64
        let s_8_14: i64 = fn_state.ebytes;
        // D s_8_15: cast zx s_8_14 -> i
        let s_8_15: i128 = (i128::try_from(s_8_14).unwrap());
        // D s_8_16: mul s_8_13 s_8_15
        let s_8_16: i128 = ((s_8_13) * (s_8_15));
        // D s_8_17: cast reint s_8_16 -> i64
        let s_8_17: i64 = (s_8_16 as i64);
        // D s_8_18: cast zx s_8_17 -> i
        let s_8_18: i128 = (i128::try_from(s_8_17).unwrap());
        // D s_8_19: cast reint s_8_18 -> i64
        let s_8_19: i64 = (s_8_18 as i64);
        // D s_8_20: cast zx s_8_12 -> bv
        let s_8_20: Bits = Bits::new(s_8_12 as u128, 64u16);
        // D s_8_21: read-var e:i64
        let s_8_21: i64 = fn_state.e;
        // D s_8_22: cast zx s_8_21 -> i
        let s_8_22: i128 = (i128::try_from(s_8_21).unwrap());
        // D s_8_23: cast zx s_8_19 -> i
        let s_8_23: i128 = (i128::try_from(s_8_19).unwrap());
        // D s_8_24: call Elem_read(s_8_20, s_8_22, s_8_23)
        let s_8_24: Bits = Elem_read(state, tracer, s_8_20, s_8_22, s_8_23);
        // D s_8_25: read-var ebytes:i64
        let s_8_25: i64 = fn_state.ebytes;
        // D s_8_26: cast zx s_8_25 -> i
        let s_8_26: i128 = (i128::try_from(s_8_25).unwrap());
        // D s_8_27: call MemU_set(s_8_10, s_8_26, s_8_24)
        let s_8_27: () = MemU_set(state, tracer, s_8_10, s_8_26, s_8_24);
        // N s_8_28: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #4s : i
        let s_9_0: i128 = 4;
        // D s_9_1: read-var ebytes:i64
        let s_9_1: i64 = fn_state.ebytes;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // D s_9_3: mul s_9_0 s_9_2
        let s_9_3: i128 = ((s_9_0) * (s_9_2));
        // D s_9_4: cast reint s_9_3 -> i64
        let s_9_4: i64 = (s_9_3 as i64);
        // D s_9_5: read-var address:u32
        let s_9_5: u32 = fn_state.address;
        // D s_9_6: cast zx s_9_5 -> bv
        let s_9_6: Bits = Bits::new(s_9_5 as u128, 32u16);
        // D s_9_7: cast zx s_9_4 -> i
        let s_9_7: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_8: cast cvt s_9_7 -> bv
        let s_9_8: Bits = Bits::new(s_9_7 as u128, 128);
        // D s_9_9: add s_9_6 s_9_8
        let s_9_9: Bits = (s_9_6 + s_9_8);
        // D s_9_10: cast reint s_9_9 -> u32
        let s_9_10: u32 = (s_9_9.value() as u32);
        // D s_9_11: write-var address <= s_9_10
        fn_state.address = s_9_10;
        // D s_9_12: read-var e:i64
        let s_9_12: i64 = fn_state.e;
        // C s_9_13: const #1s : i64
        let s_9_13: i64 = 1;
        // D s_9_14: add s_9_12 s_9_13
        let s_9_14: i64 = (s_9_12 + s_9_13);
        // D s_9_15: write-var e <= s_9_14
        fn_state.e = s_9_14;
        // N s_9_16: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var wback:u8
        let s_10_0: bool = fn_state.wback;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
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
        // D s_12_0: read-var register_index:u8
        let s_12_0: bool = fn_state.register_index;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
            return block_14(state, tracer, fn_state);
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
        // D s_13_2: call R_read(s_13_1)
        let s_13_2: u32 = R_read(state, tracer, s_13_1);
        // C s_13_3: const #32s : i
        let s_13_3: i128 = 32;
        // D s_13_4: cast zx s_13_2 -> bv
        let s_13_4: Bits = Bits::new(s_13_2 as u128, 32u16);
        // C s_13_5: cast cvt s_13_3 -> bv
        let s_13_5: Bits = Bits::new(s_13_3 as u128, 128);
        // D s_13_6: add s_13_4 s_13_5
        let s_13_6: Bits = (s_13_4 + s_13_5);
        // D s_13_7: cast reint s_13_6 -> u32
        let s_13_7: u32 = (s_13_6.value() as u32);
        // D s_13_8: read-var n:i64
        let s_13_8: i64 = fn_state.n;
        // D s_13_9: cast zx s_13_8 -> i
        let s_13_9: i128 = (i128::try_from(s_13_8).unwrap());
        // D s_13_10: call R_set(s_13_9, s_13_7)
        let s_13_10: () = R_set(state, tracer, s_13_9, s_13_7);
        // N s_13_11: return
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
        // D s_14_2: call R_read(s_14_1)
        let s_14_2: u32 = R_read(state, tracer, s_14_1);
        // D s_14_3: read-var m:i64
        let s_14_3: i64 = fn_state.m;
        // D s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // D s_14_5: call R_read(s_14_4)
        let s_14_5: u32 = R_read(state, tracer, s_14_4);
        // D s_14_6: cast zx s_14_2 -> bv
        let s_14_6: Bits = Bits::new(s_14_2 as u128, 32u16);
        // D s_14_7: cast zx s_14_5 -> bv
        let s_14_7: Bits = Bits::new(s_14_5 as u128, 32u16);
        // D s_14_8: add s_14_6 s_14_7
        let s_14_8: Bits = (s_14_6 + s_14_7);
        // D s_14_9: cast reint s_14_8 -> u32
        let s_14_9: u32 = (s_14_8.value() as u32);
        // D s_14_10: read-var n:i64
        let s_14_10: i64 = fn_state.n;
        // D s_14_11: cast zx s_14_10 -> i
        let s_14_11: i128 = (i128::try_from(s_14_10).unwrap());
        // D s_14_12: call R_set(s_14_11, s_14_9)
        let s_14_12: () = R_set(state, tracer, s_14_11, s_14_9);
        // N s_14_13: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var accdesc:struct
        let s_15_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_15_1: call AlignmentFault(s_15_0)
        let s_15_1: ProductType1d757adad216cdef = AlignmentFault(state, tracer, s_15_0);
        // D s_15_2: read-var address:u32
        let s_15_2: u32 = fn_state.address;
        // D s_15_3: call AArch32_Abort(s_15_2, s_15_1)
        let s_15_3: () = AArch32_Abort(state, tracer, s_15_2, s_15_1);
        // N s_15_4: jump b3
        return block_3(state, tracer, fn_state);
    }
}
